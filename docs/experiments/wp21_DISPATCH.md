# WP-2.1 — production corpus sweep. DISPATCH RECEIVED, STOPPED BEFORE TRANCHE ONE.

> **ONE LINE FOR THE MORNING.** WP-2.1's first sentence makes WP-2.0b's closure its
> start condition and WP-2.0b has not been designed, implemented or landed — so no
> tranche was registered and none ran; the decision owed is whether WP-2.0b runs
> next or its gate is lifted — **and, added later and logically prior to both, whether
> the labelling seat arms its solver, without which the census WP-2.1 registers cannot
> fire at all (§4.4, D-563). The CAP is a separate choice, settled by a dedicated calibration run,
> not inside a tranche.**

Received 2026-09-01. Nothing was run. No config was added, no range was reserved,
no process was started, `docs/book_v2_ledger.md` is untouched — a range reserved by
a document that was never run is still spent (the ledger's own rule), so a stopped
sweep must not leave a row.

## 1. In plain language

The dispatch is for the production sweep: take the rest of `book_v2`, label it in
tranches, and assemble the corpus Stage 2 trains on. It cannot start yet, because
the thing it is supposed to switch on does not exist. Every production label game
is meant to carry the census — the per-firing log the Stage-3 detector's re-opening
is counted from — and the switch for that, plus the position identity that makes
the count meaningful, is WP-2.0b's work. WP-2.0b is written as a dispatch and has
never been run.

Running the sweep anyway is not a cheap mistake: it spends the whole remaining book
on a corpus that answers nothing about the detector, and the book is not refillable.

## 2. The precondition, and the evidence that it is unmet

The dispatch's own start condition:

> *"Starts only after WP-2.0b's closure is on dev with CI green."*

and its machinery clause:

> *"the pipeline and the WP-2.0b census token are the only machinery"*

and its registration requirement:

> *"Census: ON via the WP-2.0b token in the pipeline's experiment config … identity
> per WP-2.0b's ruling."*

WP-2.0b's state, read four ways at `a56449b`:

| where | what it says |
|---|---|
| `docs/ROADMAP.md:433` | *"WP-2.0b — census position identity on the wire, gated. **NEXT, AND IT BLOCKS PRODUCTION.**"* |
| D-539 | the census flag *is* WP-2.0b; *"production label runs … start only after WP-2.0b lands"* |
| D-561 (WP-2.0's closure) | *"production label runs begin only after WP-2.0b lands the census identity on the wire"* |
| the tree | `TriggerObservation` (`crates/pistol-search/src/census.rs:14`) carries `columns`, `attacker`, `defender` and **no position identity**; no token gates census output; `git branch -a` holds no WP-2.0b branch and `git stash list` is empty |

The `census` that does exist in `pistol-cli` is `corpus-census`, the
symmetry-equivalence census over the human corpus. It is a different instrument and
does not discharge either of WP-2.0b's two obligations.

**So both of the registration's census clauses name things that do not exist.** A
tranche registered against them would quote a token that cannot be set and an
identity ruling that has not been made.

WP-2.0b's dispatch is already written and unrun: `docs/experiments/wp20_dispatches.md:165`.

## 3. A second finding the registration would have walked into

The dispatch registers *"the full book_v2 range partitioned into registered
tranches"*, and D-560's ceiling is that same range: 4 500 openings less the pilot's
13 is 4 487, and `4 487 x 26.7 = 119 803` — the ~119 800 figure **is** the whole
remainder. `docs/book_v2_ledger.md` records two standing claimants that have not
drawn yet:

| claimant | status |
|---|---|
| the Stage-3 detector's SPRT | SCHEDULED, one slice of the standing shape |
| the WP-1.5d ±21.5 resolution run | LICENSED, NOT SCHEDULED (D-505, D-492) |

Taking the full remainder leaves them nothing, and the ledger forbids re-reading a
consumed range for a governed verdict. **This is a sizing question the operator
owns, not one a tranche plan may settle by arithmetic** — it is carried into
D-562's flip clause so it cannot be lost.

## 4. Decision owed

1. **Run WP-2.0b next** (its dispatch exists, unrun), then WP-2.1 starts as written;
   or
2. **lift WP-2.1's census gate** — which means ruling on D-539/D-561's sequencing and
   accepting that the corpus produced carries no census, so D-537's minimum
   accumulates from a later corpus or from none; or
3. **re-size the sweep** so the two standing book claimants keep a slice, which
   changes the tranche plan's range and D-560's ceiling arithmetic.
4. **RULE ON THE LABELLING SEAT'S SOLVER GATE, AND ON ITS CAP** — added after WP-2.0b's
   design found that neither this document nor any other had noticed it (D-563).
   **This decision is logically prior to (1) and (2), not an alternative to them**: the
   census cannot fire at all on `configs/instrument_v0.toml`, the config both pilot
   seats ran, so a sweep that sets WP-2.0b's token and nothing else records zero rows
   for its whole length and D-537's clock does not start. Arming costs a MEASURED
   14.5x-36x — D-560's ~63 h ceiling becomes ~900-2 300 h — and **the cap is a second
   and separate choice that the measurement does NOT settle**: measured at both caps it
   does not move the price, and it splits D-537's own quantity — ~2x more distinct
   positions at the small cap, and the sample's only win-direction proof at the large
   one (n = 1). **So the cap is set by a dedicated CALIBRATION run at
   caps 2048 and 16384 whose records are excluded from the corpus, sized on each arm's
   own firing rate (WP-2.0b §1.1 registers the inputs, not a duration)** — not inside a tranche, because the cap also moves the search's share of its
   own budget (0.78-10.03 % against 0.27-3.72 %) and a split tranche would ship a corpus
   labelled by two instruments. The gate decision does not wait on it. See `docs/experiments/wp20b_design.md` F3 and §1.1.

(1) and (3) are independent: (3) is owed whatever is decided about (1). **(4) is owed
before (1) can deliver what it promises**, because WP-2.0b lands a token and an identity
and cannot by itself make a census fire.

## 5. The dispatch, transcribed verbatim

Transcribed because it is quoted by this document and by D-562's flip clause and
would otherwise be locatable nowhere (the `wp20_dispatches.md` precedent).

```
# [ROUTINE+] WP-2.1: production corpus sweep (full book_v2)

One session to start, tranche-resumable across sessions. No engine
diff; the pipeline and the WP-2.0b census token are the only machinery.
Starts only after WP-2.0b's closure is on dev with CI green. D-401
never read. Standing hazards binding; the sweep runs detached in
tranches, liveness via ps, never trusted via waiters.

Read first: CLAUDE.md, docs/process.md, decisions tail (D-552 on,
including D-56x), the WP-2.0 closure (docs/experiments/wp20_CLOSURE.md)
and its measured numbers (0.885 s/label, 2.14x duplication, 26.7
distinct/opening, ~119,800 ceiling), WP-2.0b's closure (token, identity
form, transposition ruling), book_v2 + ledger, D-537 (the census
minimum rule and its clock).

## Game rules (verbatim, binding)

[CLAUDE.md's own pinned six-rule section, quoted verbatim in the prompt]

## Registration (before tranche one)

One short run-registration document (this is operations, not a strength
prereg, but the same honesty rules apply — D-479/D-483, exits quoted at
a SHA, dry run on a stand-in tranche):
- Tranche plan: the full book_v2 range partitioned into registered
  tranches; ledger updated per tranche; worker count stated; wall
  estimate derived from the pilot's measured 0.885 s/label and the
  worker count, arithmetic shown.
- Census: ON via the WP-2.0b token in the pipeline's experiment config
  (committed configs untouched); both directions; identity per
  WP-2.0b's ruling. D-537's clock starts at tranche one and the closure
  reports the win-proving-firing count against the registered minimum.
- Per-tranche criteria, quoted: cold-label agreement on a registered
  sample stride; determinism re-run receipt on a registered sub-range
  of tranche one only; replay_check green, zero forfeits; loader green
  over the tranche; manifests with digests.
- Dedup: applied per D-56x's default (three-key distinct, deeper label
  wins, tie to first) at CORPUS-ASSEMBLY time, with both raw and
  deduped manifests kept — raw is the record, deduped is the training
  input.
- Failure handling: a failed tranche is VOID and re-run whole; a
  partial tranche is never assembled; two consecutive void tranches =
  STOP.

## Execution

Tranches detached, polled, receipts per tranche appended to one
run log. Resumable: a fresh session continues from the ledger, never
from memory. No other work shares the box during tranches (the pilot's
timing is the estimate's basis).

## Closure

- Corpus assembled: raw and deduped manifests, digests, counts
  (records, distinct positions, outcome coverage), all MEASURED.
- Census corpus: firing counts by direction, win-proving firings on
  disjoint positions vs D-537's registered minimum — if met, the
  closure NAMES that detector round 3's precondition is satisfied and
  the package is licensed; scheduling stays the architect's.
- Ledgers final; ADR lines; artifacts exported with digests (D-469);
  CI green at closure HEAD; tree clean; summary in the standing format,
  ONE LINE FOR THE MORNING first, including the one number Stage 2
  waits on: distinct labelled positions delivered.
- ROADMAP: next is the Stage-2 eval design package (architect round:
  matrix + DECISION-RED-TEAM over the eval family), consuming this
  corpus, D-56x's usage rulings, and the Research-A findings if
  delivered.

## STOP protocol

STOP on: two consecutive void tranches; cold-label mismatch anywhere;
determinism failure; CI red after one fix round; failure outside the
diff. On STOP: tranche state recorded in the ledger, no partial
assembly, tree clean, no processes, summary naming the decision owed,
plain language first.
```

**`D-56x` in the dispatch is D-562**, appended to `docs/decisions.md` this session:
the outcome-usage amendment to D-558(1), the dedup default for D-558(2), and the
sweep's registered size and census timing.
