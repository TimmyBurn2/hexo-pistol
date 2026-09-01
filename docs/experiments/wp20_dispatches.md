# The WP-2.0 dispatches — transcribed to a tracked path

**Why this file exists.** These are the operator's governing texts for the
Stage-2 opener: they set the scope, the requirements, the obligations and the
STOP protocol, and every later document quotes them. They arrived as session
prompts, which is not a path a successor can read. The Stage-3 arc paid for this
exact defect — a red team found its dispatch *"quoted 16 times and located
nowhere"*, D-469's class in its exact shape — and repaired it by transcription
(`docs/experiments/stage3_overnight_dispatch.md`). A round-2 red team found this
arc had made the same mistake again; this file is the repair.

**THERE ARE FOUR, THE THIRD'S MUTANT LIST DIFFERS FROM THE FIRST'S, AND THE
FOURTH SUPERSEDES THE SECOND.** The WP-2.0 dispatch and the WP-2.0b dispatch
arrived together; the third — the M-design-by-quotation round — arrived later
and is transcribed at the same standard; the fourth is **WP-2.0b v2**, which
arrived after WP-2.0's closure and says in its own first line that it
*"supersedes the earlier WP-2.0b dispatch text"*. **The second is kept rather
than replaced in place**, because `docs/experiments/wp20b_design.md` revision 1
was written against it and a reader who cannot see the text a superseded
revision was governed by cannot tell an amendment from a drift. **The fourth's
obligations are a superset of the second's, and the difference is enumerated in
its own section below** rather than left for a reader to diff. **A document quoting "the dispatch" must say which**, because the third
qualifies the seed mutant with *"where the pipeline samples"* and the first does
not, **and the third also drops the first's *"census direction collapsed -> its
test dies"* and adds a cold-label mutant of its own**. A design quoted the
qualified form and attributed it to the first, which is the defect this file
exists to make impossible — and a file that exists to let a successor tell the
lists apart has to name every way they differ, not the one that caught somebody.

**Provenance.** Transcribed by the session that received each, from its own
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

## WP-2.0b — census position identity on the wire, gated (v1, SUPERSEDED)

*Superseded by the v2 re-issue transcribed at the end of this file. It is kept
because `docs/experiments/wp20b_design.md` revision 1 names it as its governing
text, and a superseded governing text still has to be readable.*


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

## WP-2.0 finish — the M-design-by-quotation round

*Received by the session of 2026-08-31 that authored WP-2.0-M revision 3 and
WP-2.0-S. Transcribed here for the same reason the two above are: it is quoted by
the documents of that round and would otherwise be locatable nowhere. **Its
mutant list differs from the WP-2.0 dispatch's above** — it qualifies the seed
mutant with "where the pipeline samples" — and a document that quotes one must
say which.*

```
# [GROUNDWORK] WP-2.0 finish (fresh session): M-design by quotation -> impl -> pilot -> closure

Fresh session; it authored nothing it will now review or cite. Full
remaining arc in one session where the gates allow: §0 state -> §1
WP-2.0-M design -> §2 capture-half state -> §3 impl (row (g), branch B)
-> §4 pilot -> §5 closure. Overnight capable, delegation granted
in-dispatch (D-382). Long jobs detached, polled, liveness via ps.
D-401 never read. Standing hazards binding (no concurrent cargo, /home
not /tmp, /usr/bin/grep for records, CARGO_TARGET_DIR never exported
around ci.sh).

Read first: CLAUDE.md, docs/process.md, D-521..D-545 and D-53n, the
session summary (docs/experiments/session_2026-08-31_summary.md), the
shape matrix rev 2 + its red-team report + the selection (row (g),
branch B, strongest surviving attack), BOTH WP-2.0-M design revisions
and all review reports (revision 1 contains the true sentence), the
WP-2.0 dispatcher + amendments, tools/design_citation_check.py and
D-543.

## §0 First actions

1. Append the paste block D-lines at the next free numbers.
2. Confirm dev green at 22fba16 or a descendant, cited from gate lines.

## §1 WP-2.0-M design, by quotation (the granted round)

Discipline, all four mandatory:
- LIFT, don't rewrite: every mechanism that has survived a review enters
  this design as a QUOTATION of the surviving text (revision 1's true
  sentence by name, the matrix row (g) text, the memo's verified
  findings), cited to its source revision SHA. New prose is only
  connective tissue and NEW claims, each citing code at file:line or an
  artifact by digest (D-543).
- `tools/design_citation_check.py` runs green BEFORE the review is
  dispatched; a review finding the checker could have caught is recorded
  as author debt in the summary.
- PASSED-SECTION FREEZE (new law, paste block): any section a reviewer
  has passed in any prior revision is frozen; if this design must edit
  one, the edit is listed in the document header with grounds. An
  unlisted edit to a frozen section is a finding by itself.
- The design states mechanisms, invariants and tests only (D-483); the
  cold-label agreement criterion (D-53n) appears as an obligation with
  its registered check named, not re-argued.

REVIEW-design: fresh subagent, strongest model. Outcome rules fixed:
PASS -> §2. One FAIL -> one fix round under the same discipline ->
scoped re-check. A second FAIL -> STOP, the package returns to the
architect; no further rounds exist in this session.

## §2 Capture-half state

Verify, from the tracked record, whether the capture half's design has
a surviving PASS. If yes: quote it and proceed. If it is pending: author
it under §1's identical discipline and gates (its own granted round,
same outcome rules). If it FAILED twice previously and no grant covers
it: STOP, back to the architect — do not infer a grant.

## §3 Implementation (row (g), branch B)

- Per the selected row's own text: a labelling mode over a written
  report, zero engine seams, replay.rs's walk as the re-ask loop,
  pistol-core as referee. No engine diff of any kind (D-53n's license);
  if any step appears to need one, that is a premise failure: STOP.
- Obligations from the standing WP-2.0 dispatcher unchanged: schema
  with loader tests, ledgers append-only, determinism re-run receipt,
  mutants (schema field dropped, seed ignored where the pipeline
  samples, ledger overwrite, and — new for (g) — a re-ask served warm
  must die at the cold-label test).
- REVIEW-impl: fresh, strongest, one fix round, standing caps.

## §4 Pilot (registered before it runs)

Small registered book_v2 range (ledger updated). Criteria registered
first, including: the D-53n cold-label agreement check (sampled
positions re-labelled in fresh processes, byte-equal or STOP);
throughput MEASURED (games/hour, labels/hour); replay_check green over
pilot games; zero forfeits; the determinism re-run receipt on a
sub-range. The pilot carries no census and is not corpus (D-53n).

## §5 Closure

- Corpus-size plan for training stated as ESTIMATED with the arithmetic
  shown from the pilot's measured throughput.
- ROADMAP: production label runs unblocked pending WP-2.0b; next
  package is WP-2.0b (census identity), then production corpus, then
  the Stage-2 eval design package consuming the corpus plan and the
  Research-A findings.
- D-534 restated in one line: the 725 ms movetime overshoot blocks any
  play-config arming of the solver; no SPRT discharges it.
- ADR lines; artifacts + manifests exported with digests (D-469);
  CI all gates at closure HEAD cited from gate lines; tree clean; no
  worktrees; no processes; summary in the standing format, ONE LINE FOR
  THE MORNING first, plain language before technical.

## Laws (pointers)

Receipts; mutant-dies; CLAIM-HOME; D-424; D-374; D-477; D-479; D-483;
D-469; D-543 + citation checker; passed-section freeze; SPRT the only
strength voice (none is made here); standing caps everywhere — the
overnight loop grant is long expired.

## STOP protocol

STOP on: §1/§2 second design failure; a premise failure in §3; cold-
label mismatch; determinism failure; CI red after one fix round;
failure outside the diff; any cap exhausted. On STOP: tree clean or WIP
on `wp20-finish-stopped`, never dev; no detached processes, receipt;
exports complete; summary naming the decision owed, plain language
first, written for the operator's first read.

## Paste block (append at next free D-numbers)

D-54x: Architect grant per the D-489 precedent: WP-2.0-M's design
receives ONE fresh-session round despite its spent cap, on the recorded
ground that all five design failures were authorship-class (claims the
code doesn't make, or true claims destroyed by rewrites) and none were
judgement-class; the round runs under quotation-lift and the citation
checker; outcome rules fixed in the dispatch; a second failure returns
the package to the architect.

D-54y: Standing law, passed-section freeze: a section a reviewer has
passed is frozen across subsequent revisions; any edit to a frozen
section must be listed in the revision header with grounds, and an
unlisted edit to a frozen section is a review finding by itself.
Motivating instance: WP-2.0-M revision 2 deleting revision 1's true
sentence and building on its negation. Complements D-543: the citation
checker guards claims entering a document; the freeze guards claims
already verified in it.

## DONE

- §1 design PASSED under the discipline (or a clean STOP), checker
  green before review, freeze header honoured.
- §2 capture half verified or designed under the same gates.
- §3 pipeline landed on row (g)/branch B with zero engine seams,
  mutants dead as listed.
- §4 pilot receipts: cold-label agreement green, throughput MEASURED,
  replay_check green, zero forfeits, determinism receipt.
- §5 closure complete: corpus plan ESTIMATED with arithmetic, ROADMAP
  updated, D-534 restated, CI green at HEAD, tree clean, summary on
  disk.
```

---

## WP-2.0b v2 — the re-issue that supersedes the second dispatch above

*Received by the dispatching session of 2026-09-01, after WP-2.0's closure
(`a56449b`) and after `docs/experiments/wp20b_design.md` revision 1 had been
written against the v1 text. Transcribed from that session's own prompt.*

**WHAT v2 ADDS TO v1, enumerated because the whole reason this file exists is
that a successor must be able to tell two dispatch texts apart.** Six
differences, and none of them is a scope cut:

1. **Reading list.** v1 named *"D-535..D-538 and D-53n"*; v2 names *"decisions
   tail (D-521 on, through the WP-2.0 closure lines)"* and adds **D-512** and
   **D-537** individually. **CORRECTED in revision 3 of the design's review round**:
   an earlier form of this limb claimed v2 *"adds 'the D-527 cold-seat discipline'
   as a named read"*, and it does not — v1's own read-first list already carries the
   identical phrase, D-527 named by number. The difference in this limb is the
   range and the two added numbers, and nothing else.
2. **Scope 1 gains a fixture.** v1 asked only that the disjointness be countable;
   v2 requires *"a fixture pinning a known transposition pair to the ruled
   count"* as part of the design's own statement, not only as a test.
3. **Scope 2 gains two wire constraints.** *"Both directions preserved as
   separate fields (D-512)"*, and *"the D-551 parser lesson applies: multi-word
   values named as such in the schema, never assumed one-word."* Neither appears
   in v1.
4. **The mutant list is longer and is call-site-bound.** v1 listed three mutants
   and no law; v2 requires them *"per D-55y run green BEFORE REVIEW-impl,
   call-removed mutants included"* — that is D-553's law under the architect's
   placeholder — lists **four**, and specifies where two of them must die: the
   token-check mutant *"dies at the call site"*, and a *"transposition ruling
   inverted"* mutant is added against the scope-1 fixture.
5. **Closure names the ROADMAP successor differently.** v1: *"next is the Stage-2
   eval design package"*. v2: *"next is the production corpus package (label runs
   at the closure's planned size, census on from game one, D-537's clock
   starts), then the Stage-2 eval design package."*
6. **THE OPTION FIELD WAS EDITED AND BOTH OPTIONS WERE KEPT — this limb said
   "unchanged" and was wrong.** v1 scope 1 reads *"(the full-turn 128-bit key per
   D-8, or the canonical move-list prefix)"*; v2 reads *"(full-turn 128-bit key per
   D-8, or canonical move-list prefix **per D-6**)"*. **A citation was added to the
   second option and both options were left standing**, and scope 3 was rewritten
   from *"Whatever dependency route gets census output through pistol-cli"* to
   *"The dependency route through pistol-cli:"* — a compression of the clause the
   design's F1 says dissolves, retaining the instruction F1 says needs no obeying.
   **This is the strongest evidence in the record that the architect looked at the
   option field**, and the earlier form of this limb — which called the premise
   *"unchanged"* — buried it in the file that exists to surface differences. It is
   corrected here rather than in place. **What follows from it is `wp20b_design.md`
   §10.0's**, which no longer argues from silence: F2's constraint belongs to a
   landed document that named WP-2.0b first, and adding *"per D-6"* moves the second
   option FURTHER from `key_full`, not closer, since a bare play sequence folds
   neither transpositions nor symmetries.

```
# [ROUTINE+] WP-2.0b: census position identity on the wire, gated (v2)

Supersedes the earlier WP-2.0b dispatch text. One session, small diff
touching pistol-search/cli/engine, carrying full engine-diff
obligations: byte-identity when off, determinism, REVIEW-impl, and the
call-site mutant law (D-55y). Runs after WP-2.0's pilot closure;
production label runs wait on this package (D-53n).

Read first: CLAUDE.md, docs/process.md, decisions tail (D-521 on,
through the WP-2.0 closure lines), the WP-2.0 premise memo (P2's
verified findings are this package's scope statement), census.rs and
the D-527 cold-seat discipline, D-512 (both-directions counters), the
workspace-shape test pinning pistol-cli's dependencies, D-537 (the
disjoint-positions minimum rule the identity must serve). D-401 never
read.

## Game rules (verbatim, binding)

[CLAUDE.md's own pinned six-rule section, quoted verbatim in the prompt]

## Scope

1. TriggerObservation gains position identity. Design chooses the form
   (full-turn 128-bit key per D-8, or canonical move-list prefix per
   D-6) against ONE criterion: D-537's "win-proving firings on DISJOINT
   positions" must be countable mechanically, transposition-aware; the
   design states which transpositions count as identical and why, with
   a fixture pinning a known transposition pair to the ruled count.
2. Census rows on the wire behind a token NO committed config sets.
   Line-protocol addition documented in the protocol's one home; field
   order pinned by a report test (D-88 precedent). Both directions
   preserved as separate fields (D-512); the D-551 parser lesson
   applies: multi-word values named as such in the schema, never
   assumed one-word.
3. The dependency route through pistol-cli: the workspace-shape test is
   UPDATED deliberately, not deleted; the design quotes it and states
   the new shape it pins.
4. Cold-table discipline (D-527) intact; identity read at firing time
   from state the search already holds — quoted site proving no extra
   hashing on the non-census path.

Out of scope: detector logic, ranking, label work, committed-config
changes, strength claims.

## Obligations

- Gate off = byte-identical: two-binary diff over the standing position
  set, output digest equal to pre-change.
- Determinism all seats; census state newgame-cleared and seated if it
  exists.
- Tests: identity on every census row (schema test); disjointness
  fixture counts exactly as ruled; token absent = zero census bytes,
  pinned by a loud test.
- Mutants, per D-55y run green BEFORE REVIEW-impl, call-removed mutants
  included: identity column dropped -> schema test dies; token check
  call removed -> zero-bytes test dies at the call site; warm-table
  read introduced -> D-527 seat dies; transposition ruling inverted ->
  fixture dies.
- REVIEW-impl: fresh, strongest, one fix round; standing caps; a second
  failure = STOP and split.
- Bench guard: registered nps spot-check ON-token vs OFF at 50 000
  nodes, direction per convention — a logging path that taxes the
  engine contaminates every future measurement made with it.

## Closure

D-line recording the identity form and transposition ruling; protocol
doc updated in its one home; artifacts exported with digests (D-469);
CI all gates at closure HEAD; tree clean; summary in the standing
format, ONE LINE FOR THE MORNING first. ROADMAP: production label runs
UNBLOCKED — next is the production corpus package (label runs at the
closure's planned size, census on from game one, D-537's clock starts),
then the Stage-2 eval design package.

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

**The `D-53a`, `D-53n` and `D-55y` labels in these dispatches are not decision
keys.** They are the architect's placeholders. `D-53a` and `D-53n` are for paste
blocks that were registered at the next free numbers: `D-53a` became **D-535 …
D-538**, and `D-53n` became **D-539** and **D-540**. `D-55y` is different in kind
— it points BACKWARD at a law that already exists, and it resolves to **D-553**,
the call-site mutant law, which is the only standing law matching v2's own gloss
*"call-removed mutants included"* and *"mutation receipts run green BEFORE
REVIEW-impl is dispatched"*.
