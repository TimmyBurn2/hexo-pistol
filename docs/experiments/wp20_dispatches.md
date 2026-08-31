# The WP-2.0 and WP-2.0b dispatches — transcribed to a tracked path

**Why this file exists.** These are the operator's governing texts for the
Stage-2 opener: they set the scope, the requirements, the obligations and the
STOP protocol, and every later document quotes them. They arrived as session
prompts, which is not a path a successor can read. The Stage-3 arc paid for this
exact defect — a red team found its dispatch *"quoted 16 times and located
nowhere"*, D-469's class in its exact shape — and repaired it by transcription
(`docs/experiments/stage3_overnight_dispatch.md`). A round-2 red team found this
arc had made the same mistake again; this file is the repair.

**Provenance.** Transcribed by the session that received them, from its own
prompt. **Verbatim** apart from this preamble and the fenced-block wrapping. A
reader who doubts the transcription has no second copy to check it against —
that is the cost of the defect this file repairs, and stating it is better than
implying a fidelity nothing can verify.

**The game-rules block** each dispatch carries is CLAUDE.md's own pinned section,
quoted there verbatim; it is not re-transcribed here because CLAUDE.md owns it
and a claim a document makes twice is a defect waiting (D-423).

---

## WP-2.0 — the label + census pipeline

```
# [GROUNDWORK] WP-2.0: Stage-2 opener — the label + census pipeline

Full round in ONE session: §0 -> premise verification -> design ->
REVIEW-design -> impl -> REVIEW-impl -> pilot run -> closure. Overnight
capable, delegation granted in-dispatch (D-382). Long jobs detached,
polled, liveness via ps. D-401 never read. Hazards from WP-1.9 closure
§9 and the detector arc binding.

This package builds INFRASTRUCTURE: it changes no committed engine
behaviour and makes no strength claim. Its deliverable is a reproducible
pipeline producing (1) labeled positions for Stage-2 training and (2)
the solver census corpus per D-53a ruling 3.

Read first: CLAUDE.md, docs/process.md, decisions tail (D-509 on),
D-434 (teacher = pistol search-bootstrapped labels), D-483/D-479,
D-53a, the detector arc closure, book_v2 + its ledger, the census
instrument (corrected, cold-table seat, D-527), the anchor v2 report.

## Game rules (verbatim, binding)

[CLAUDE.md's own pinned six-rule section, quoted verbatim in the prompt]

## §0 First actions

1. Append the architect paste block (D-53a) at the next free number if
   the closing session has not already.
2. Confirm dev green at the detector-arc closure HEAD, cited from gate
   lines.
3. Start the trigger-coverage ledger file (D-53a ruling 2) with the two
   never-firing VALUE rows as its first entries.

## Premise verification (D-477, before design)

Quote at file:line: the play/instrument mode seams the pipeline will
drive (the engine is driven as a black box over its line protocol —
the pipeline never links engine internals); the census counters and
their cold-table discipline (D-527's corrected seat); book_v2's format
and ledger; where deep-search scores are emitted on the protocol today
and at what cost shape. Premise failure = STOP with a memo.

## Scope

A pipeline (tools/, its own crate or scripts per design) that:
1. Plays self-play games: committed config vs committed config, book_v2
   openings, instrument mode, node budgets — the GAME budget and the
   LABEL budget are separate registered values (games at the standing
   50 000; labels re-scored deeper, value chosen by the design with
   grounds and recorded, D-483: in the pilot prereg, not the design).
2. Emits one record per position: canonical move list (D-6), side to
   move, the deep-search label (score + best move + depth + nodes), and
   game outcome — a documented, versioned schema with a loader test.
3. Runs the census logging on every game per D-53a: win-proving firings
   recorded with position identity, both directions, cold-table.
4. Is deterministic end-to-end given (seed, book range, config, SHA):
   a re-run receipt proves byte-identical output on a small range.
5. Maintains ledgers: book ranges consumed (shared with the SPRT
   ledger), corpus manifest with digests, census corpus manifest.

Out of scope: any training, any codebook, any eval change, any engine
diff at all, any strength claim, detector work beyond the logging flag.

## Design decides and records

Storage format and schema version; the label policy (which positions
get deep labels — all, or a registered sampling rule); dedup policy for
transposed positions (by canonical move list, stated); the census
minimum rule per D-53a (the power-style rule fixing when detector
round 3 may re-open — the RULE lands now, before any corpus exists, so
it cannot be fitted later); throughput expectation stated as a shape,
measured in the pilot, never guessed (D-500's class).

## Development round

1. Design (mechanisms, invariants, tests only) -> REVIEW-design: fresh,
   strongest, one fix round.
2. Impl -> tests: schema roundtrip; loader rejects malformed records
   loudly; determinism re-run receipt; ledger append-only behaviour;
   census records carry position identity and direction. Mutants:
   schema field dropped -> loader test dies; seed ignored -> determinism
   receipt dies; ledger overwrite -> append test dies; census direction
   collapsed -> its test dies.
3. REVIEW-impl: fresh, strongest, one fix round.
4. PILOT: a small registered range of book_v2 (size with grounds,
   ledger updated), full pipeline end to end, detached. Receipts:
   games completed / positions labeled / census firings collected;
   throughput MEASURED (games per hour, labels per hour); the re-run
   determinism receipt on a sub-range; zero forfeits; replay_check
   over the pilot games.
5. Closure: the pilot's measured throughput extrapolated to a stated
   corpus-size plan for the training package (labeled ESTIMATED, with
   the arithmetic shown); ADR lines; artifacts + manifests exported
   with digests; ROADMAP updated (next: the Stage-2 eval design
   package, which consumes this corpus and the Research-A findings);
   summary in the standing format, ONE LINE FOR THE MORNING first.

## Laws (pointers)

Receipts; mutant-dies; CLAIM-HOME; D-424; D-374; D-477; D-479; D-483;
D-469 export before removal; SPRT the only strength voice (none is made
here); caps: one fix round per review, a document failing twice = STOP
and split (the overnight loop grant has EXPIRED — standing caps apply).

## STOP protocol

STOP on: premise failure; determinism receipt failure; CI red after one
fix round; failure outside the diff; any cap exhausted. On STOP: tree
clean or WIP on `wp20-stopped`, never dev; no detached processes,
receipt; exports complete; summary naming the decision owed, plain
language first.

## DONE

- D-53a appended; trigger-coverage ledger started.
- Premise memo quoted or clean premise-STOP.
- Pipeline landed with schema, ledgers, and determinism receipt;
  mutants dead as listed.
- Pilot run receipts: counts, throughput MEASURED, replay_check green,
  zero forfeits.
- The census-minimum rule for detector round 3 recorded BEFORE any
  corpus accumulation.
- Corpus-size plan for training stated as ESTIMATED with arithmetic.
- CI all gates at closure HEAD; tree clean; no worktrees; no
  processes; summary on disk.
```

---

## WP-2.0b — census position identity on the wire, gated

```
# [ROUTINE+] WP-2.0b: census position identity on the wire, gated

One session, small diff, but it touches pistol-search/cli/engine, so it
carries the engine-diff obligations in full: byte-identity when off,
determinism, REVIEW-impl. Runs after WP-2.0's closure; production label
runs wait on this package (D-53n).

Read first: CLAUDE.md, docs/process.md, D-535..D-538 and D-53n, the
WP-2.0 premise memo (P2's verified findings are this package's scope
statement), census.rs and the D-527 cold-seat discipline, the workspace-
shape test pinning pistol-cli's dependencies. D-401 never read.

## Game rules (verbatim, binding)

[CLAUDE.md's own pinned six-rule section, quoted verbatim in the prompt]

## Scope

1. TriggerObservation gains position identity: the design chooses the
   form (the full-turn 128-bit key per D-8, or the canonical move-list
   prefix) with the disjointness requirement of D-537 as the criterion —
   the identity must let "win-proving firings on DISJOINT positions" be
   counted mechanically, transposition-aware, and the design says which
   transpositions it treats as identical and why.
2. Census rows go on the wire behind a token NO committed config sets.
   The line-protocol addition is documented in the protocol's one home;
   field order pinned by a report test (the D-88 precedent).
3. Whatever dependency route gets census output through pistol-cli, the
   workspace-shape test is UPDATED deliberately, not deleted; the design
   quotes the test and states the new shape it pins.
4. The census remains cold-table by D-527's discipline; the identity
   column is read at firing time from state the search already holds —
   the design proves (quoted site) that no extra hashing or probe is
   added on the non-census path.

Out of scope: any detector logic, any ranking, any label work, any
committed-config change, any strength claim.

## Obligations

- Gate off = byte-identical: the two-binary diff procedure over the
  standing position set, output digest equal to the pre-change engine's.
- Determinism: all seats green; if census carries state, it is
  newgame-cleared and seated.
- Tests: identity column present on every census row (schema test);
  disjointness countable (a fixture with a known transposition pair
  counts as one or two exactly as the design ruled); token absent =
  zero census bytes on the wire (loud test, not absence-of-evidence).
- Mutants: identity column dropped -> schema test dies; token check
  removed -> the zero-bytes test dies; a warm-table read introduced ->
  the D-527 seat dies.
- REVIEW-impl: fresh, strongest, one fix round. Standing caps (the
  overnight grant is expired); a second failure = STOP and split.
- Bench guard: one registered nps spot-check ON-token vs OFF at 50 000
  nodes, direction per convention, because a logging path that taxes
  the engine would contaminate every future measurement made with it.

## Closure

D-line recording the identity form and its transposition ruling; the
protocol doc updated in its one home; artifacts exported with digests
(D-469); CI all gates at closure HEAD; tree clean; summary in the
standing format, ONE LINE FOR THE MORNING first. ROADMAP: production
label runs are unblocked; next is the Stage-2 eval design package,
consuming WP-2.0's corpus plan and the Research-A findings.

## STOP protocol

STOP on: byte-identity mismatch; determinism exit 3; CI red after one
fix round; failure outside the diff; any cap exhausted. On STOP: tree
clean or WIP on `wp20b-stopped`, never dev; no detached processes,
receipt; exports complete; summary naming the decision owed, plain
language first.
```

---

## The one question a reader will bring to these, answered here

**Does WP-2.0 require games to be PLAYED and LABELLED in one pass?** Its scope
list says *"Plays self-play games"* (1) and *"Emits one record per position"* (2)
as separate numbered requirements, and its Development round §4 calls the pilot
*"full pipeline end to end"* without saying how many passes that is. **The text
does not require one pass**, and this is recorded because the shape matrix's
row (g) has that reading as its kill condition — so a successor can check the
answer against the governing text rather than against a summary of it.

**The `D-53a` and `D-53n` labels in these dispatches are not decision keys.** They
are the architect's placeholders for paste blocks that were registered at the
next free numbers: `D-53a` became **D-535 … D-538**, and `D-53n` became **D-539**
and **D-540**.
