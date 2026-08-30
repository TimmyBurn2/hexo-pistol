# The overnight dispatch — transcribed to a tracked path

**Why this file exists.** The dispatch below is the operator's governing text
for this arc: it decides ruling 1 verbatim, supplies the decision procedure for
rulings 2-7, enumerates the option field, and sets the loop grant. It arrived as
a session prompt, which is not a path a successor can read. A red team found the
consequence and named it (`artifacts/stage3_rulings_redteam_v1.md`, MAJOR 5):
*"The dispatch is quoted 16 times and located nowhere … Every one of its
quotations is self-referential."* That is D-469's class — the arc's own closure
records four WP-1.8c review reports lost exactly this way.

**Provenance, stated because a transcription's worth is its provenance.** This
is the dispatch as delivered to the session that wrote
`docs/experiments/stage3_rulings.md`, transcribed by that session from its own
prompt. It is **verbatim** apart from this preamble; where the original used a
heading level or list marker that would collide with this file's own structure
it is preserved as written. No word of it is edited, and nothing is added to it.
A reader who doubts the transcription has no second copy to check it against —
that is the cost of the defect this file repairs, and stating it is better than
implying a fidelity nothing can verify.

---

```
# [GROUNDWORK] Overnight arc: rulings -> anchor v2 -> book_v2 -> detector

Operator ASLEEP. Fully autonomous. Every fork below has a default: take
it, record "architect default applied" as a D-line, continue. Only exits
are CLOSED (per section) or STOP (§9). Multi-day allowed. Long jobs
detached, polled, liveness-checked (ps, not waiters). D-401 never read.
Hazards from the WP-1.9 closure §9 and the detector-STOP summary are
binding: no concurrent cargo; /home not /tmp for worktrees; /usr/bin/grep
for anything recorded; CARGO_TARGET_DIR never exported around ci.sh;
tag measured worktree revisions before removal.

LOOP GRANT (operator, recorded as a D-line): every review gate in this
dispatch may take up to THREE fix rounds instead of one. Round three is
always scoped to the prior review's enumerated remedies only. A document
still failing after three = STOP and split (D-481 form). This grant is
for this dispatch only and does not amend CLAUDE.md.

Read first: CLAUDE.md, docs/process.md, decisions tail (D-494 on), the
detector-STOP closure and memo (§7 rulings list, §8), WP-1.9b closure,
the 1.9 closure §9, the sealbot merge state (tools/sealbot/, D-438), the
anchor artifacts, wp18b_probe_v{1,2}, the stopped branch
stage3-detector-stopped at dda43cf.

## Game rules (verbatim, binding)

[the six rules of CLAUDE.md's own "Game rules (pinned)" section, quoted
verbatim; not re-transcribed here because CLAUDE.md owns them and a claim
a document makes twice is a defect waiting (D-423)]

## §0 Rulings, merge, housekeeping

1. Merge `stage3-detector-stopped` (dda43cf) into dev: docs, memo,
   derivation instrument, D-508. Expected conflicts docs-only; a code
   conflict = STOP. CI green before and after, cited from gate lines.
2. Ruling 1 of 7, architect-decided, apply verbatim: the resumed
   detector designs against a PER-SEARCH SOLVER CALL BUDGET (~2 calls
   per search, RE-DERIVED at HEAD by this session from the artifacts,
   never inherited), allocated by PRECISION RANKING over trigger points
   — spend the budget where a proof is most likely; the expensive calls
   are the ones that return NoWin. Pass-rate targets are retired.
3. Rulings 2-7: apply, per ruling, the first of these that fits:
   (a) the conservative default the memo/closure itself recommends;
   (b) the option preserving all registered numbers and adding no scope;
   (c) deferral to a named licensed-not-scheduled package.
   Each recorded as its own D-line with the ground named. If one ruling
   genuinely gates the whole arc and no default fits: STOP.
4. Export/worktree hygiene per D-469 throughout; dev only; main and
   pushing remain the operator's.

## §1 Sealbot anchor v2 ([ROUTINE] care, cheap, run ALONE on the box)

- Local matchserver (tools/sealbot), engine = current dev pinned
  binaries (digests recorded; rebuild means re-record).
- Play mode, `go movetime` (deployment budget; D-22 untouched — this is
  an anchor, not an instrument run). Movetime value registered before
  game one from the play config's own registered budget; sealbot at its
  standing 0.3 s unless its config says otherwise.
- TWO seats, both registered before game one:
  seat 1: committed config (all gates off);
  seat 2: committed config + solver play-mode gate ON with its capped
  calls, ONLY IF a committed play config for it exists or can be derived
  from registered values without inventing numbers; otherwise seat 2 is
  dropped with a D-line, not improvised.
- N registered first (at least 40 per seat, both colors, standard
  opening). Nondeterminism expected and welcome: count distinct games.
- Report per seat: W/L/caps/forfeits, Wilson 95 percent over decided,
  distinct-game count, per-side compute (nodes and wall), turn cap.
  Opponent UNVERIFIED, no Elo claim, the word is "anchor" everywhere.
- One D-line per seat; artifacts content-named, MANIFEST, digests in the
  ADR. Compare to D-438's anchor in one sentence, direction only.

## §2 book_v2 (its own package, [ROUTINE+])

- NEVER overwrite `random_openings_v1.txt`: it is retired-but-readable
  and stands behind every closed SPRT verdict. The six test files
  pinning the v1 filename via the compile-time constant KEEP pinning v1.
- New fixture under a new name (v2), referenced explicitly by governed
  configs that use it; no committed config flips to v2 in this arc.
- Seeded, re-executable generation: command, seed, generator revision
  committed beside the fixture. Size registered with grounds covering
  the standing SPRT worst-case n. Loadability receipt over every entry
  via the guarded bench block (D-475). Ranges fresh by construction;
  a consumed-ranges ledger file started at empty.
- One review pass (fresh, mid-strength), up to the granted rounds.
  D-line retires v1 for governed use and registers v2.

## §3 Resumed detector round (the main course)

Premise state: the STOP's memo stands as corrected by its red-team; the
target is §0's ruling 1. Then the full shape:

- MATRIX: options ranked by precision economics, at least: (a) tightened
  calculus-class trigger; (b) pattern-level must-block/open-four
  detection; (c) bounded VCDT-only probe as pre-filter; (d) two-tier
  detector -> certifier; (e) precision-ranked budget allocator over the
  current trigger (rank all firing points, call top-budget); (f) null.
  Each row: mechanism, what provable wins it can rank OUT of budget,
  cost shape, kill condition.
- DECISION-RED-TEAM before selection (fresh, strongest). Then selection
  quoting its rows.
- RECALL GATE redefined per ruling 1: on the anchor value fixture (the
  loser-wins, the winner conversions, the M4 flip position), the
  positions holding real proofs must RANK INSIDE the call budget. Pinned
  per position.
- Design -> REVIEW-design; impl behind its gate -> REVIEW-impl (fresh,
  strongest, granted rounds). Determinism seat if the detector carries
  state; counters (ranked/budgeted/fired/proof-found) on the line
  protocol, dry-run-verified. Mutants: ranking inversion -> fixture test
  dies; budget accounting drift -> sum test dies; newgame clear -> seat
  dies. Gate off = byte-identical (two-binary diff procedure).
- BENCH, registered first: ON vs OFF at 50 000 nodes, both fixture
  sets, both bands, direction per convention, the 1.8c-inherited bound
  quoted with source. This bench discharges the 1.8 re-test clause
  (D-50x) and says so.
- Bracket pass -> prereg (standing shape, book_v2 ranges, ledger
  updated, warm-replay Criterion 1'', second instrument as registered,
  slot pass D-427, honest expectation quoting the measured call-budget
  and recall receipts) -> governed run -> closure.
- Bracket fail at the registered kill point, or SPRT h0: the roadmap
  FLIPS to Stage 2 per D-471. DELEGATION: the session records the flip
  D-line itself (the operator's sleep does not suspend a registered
  clause), opens NOTHING of Stage 2, and closes.

## §4 Closure of the arc

Whatever §3's verdict: ADR lines; ROADMAP updated; artifacts exported
with digests; one summary for the whole arc at sessions/, ONE LINE FOR
THE MORNING first, plain language before technical, the seven rulings
listed with their applied defaults, the anchor numbers, the book state,
and the detector verdict with its roadmap consequence.

## Laws (pointers)

Receipts rule; mutant-dies; CLAIM-HOME; D-424; D-374 registered numbers
never move; D-477 premises quoted; D-479 both ratio terms named; D-483
no numbers in designs; D-469 export before removal; SPRT the only
strength voice; anchors are anchors.

## §9 STOP protocol

STOP on: a code conflict in §0's merge; an unrulable ruling; determinism
exit 3; CI red after the granted rounds; failure outside the diff; a
second void governed run; any cap exhausted. On STOP: tree clean or WIP
on `overnight-stopped`, never dev; no detached processes, receipt;
exports complete; summary with the decision owed named, plain language
first — written so the operator's first read over coffee answers "what
happened, what do you need from me".

## DONE

- §0: merge landed green; seven rulings recorded as D-lines.
- §1: anchor v2 taken, two seats or a D-lined one, report + MANIFEST.
- §2: book_v2 committed with generation receipt and ledger; v1 retired
  for governed use, untouched on disk.
- §3: matrix, red-team, selection, design, impl, bench all landed as
  tracked files; recall gate green per position; bracket verdict
  recorded; on pass, SPRT verdict with n, distinct_n, pentanomial,
  llr_pair, calls ranked/budgeted/fired per side; on fail/h0, the
  D-471 flip recorded.
- §4: arc summary on disk; `tools/ci.sh` all gates at closure HEAD;
  tree clean; no worktrees; no processes.
```

---

## The one place the transcription resolves an ambiguity, and it is flagged

**The option field's letters.** §3 above enumerates **six** rows, (a) through
(f). The premise memo's §8 ruling 3 and the closure's §7 both speak of the
dispatched field as **"(a)-(e)"**, and the red team flagged the mismatch
(MAJOR 6). Both readings are on the record and this file does not adjudicate
between them: what it establishes is that **the field this arc's matrix is built
over is the six rows above**, quoted from the text that governs the arc.
`docs/experiments/stage3_rulings.md` §3 states the field as six rows and no
longer claims which of them are new.
