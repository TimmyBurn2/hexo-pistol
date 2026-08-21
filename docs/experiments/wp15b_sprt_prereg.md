# WP-1.5b — SPRT pre-registration: staged threat-first generation vs the committed radius policy

**Revision 2. DRAFT. THIS DOCUMENT GOVERNS NOTHING YET.**

**Revision 1 (`af9fa4a`) FAILED its review — 4 BLOCKING, 7 MAJOR.** The finding of
the round was built rather than argued: the reviewer mutated the arena's SCORE
attribution path, producing a completely inverted verdict (`nelo_pair` −992.88 →
+992.88, `wins_a 0` → `wins_a 7`, pentanomial mirrored), and **both of revision
1's registered dry-run criteria PASSED on it**. The class revision 1 named —
"attributes one seat's games, SCORE or compute to the other seat's label … the
most damaging way, silently" — was covered only on the compute half. §8 is
rebuilt around a referent that sits above the timing marker and discriminates the
seats deterministically.

Revision 1 also dropped a commitment already on the ADR log: **D-245 names this
document by name** and pre-commits its second instrument and its consequence.
Revision 1 provided neither. §7A is that repair.

It names `configs/instrument_staged_v0.toml`, which does not exist at this
revision, so it cannot govern a run and no run has been taken under it. It becomes
governing when three things hold together: the config exists, every
**OPERATOR-CONFIRM** slot in §9 is filled, and the document passes a fresh-context
review AT THE REVISION THAT GOVERNS THE RUN (CLAUDE.md: "the revision that governs
a run must itself pass a fresh-context review before the first run it governs.
Reviews of superseded revisions do not transfer — an amendment reopens the review,
however small the diff").

**The run is the operator's.** This session delivers the document, its dry run and
its review state; it plays no game.

---

## 1. What is being judged, and what is not

**The claim.** Staged threat-first candidate generation
(`docs/experiments/wp15b_design.md`) is stronger than the committed radius-2
full-width policy at a fixed node budget.

**What this run cannot judge, said first because rule 6 is about what a number
means.** SPRT judges STRENGTH, not soundness — a pruning change that quietly
changed the value of the tree would pass one (D-106, D-117). Soundness is the
design's §8 gate and it is CI's, not this run's. A green SPRT over an unsound
generator is a measurement of nothing, and the order matters: **§8's gate must be
green at the revision this run measures**, which §9 makes an OPERATOR-CONFIRM
slot rather than an assumption.

**Three axes separate the two seats, not one.** The design's §7.2 and §5.2 record
them and this document repeats them so no reader takes the verdict for a pure
generation experiment:

1. what the search SELECTS — the staged tiers and the quiet cut;
2. what it can SEE — MEASURED, about 29 % of the adopted Tier-T option's cells lie
   outside the radius-2 ball, so the staged seat reaches cells the incumbent never
   offered;
3. SEARCH VALUE — the overload early return and the filter's licensed shortening
   of mate distances on lost positions both change what a node reports.

A verdict here is a verdict on the three together. Attributing it to any one of
them would need an experiment this document does not describe.

---

## 2. The hypothesis and the verdict unit

- **H0**: `elo0 = 0` — the staged seat is no stronger.
- **H1**: `elo1 = ` **OPERATOR-CONFIRM** (§9.1). The arena reads these as
  normalised Elo on the PAIR unit.
- `alpha = 0.05`, `beta = 0.05` — the values every prior run in this project used
  (`configs/arena_wp13_*.toml`), kept so two documents do not appear to disagree
  about a convention.
- **Verdict unit: the PAIR.** Both games of an opening are played, colours
  reversed, and the pentanomial pair outcome is the sample. This is the unit
  D-190's runs used and the one `sprt.rs` implements.

**The pair floor: 100 pairs.** No H1 action fires below it. Its ground, stated
with the conditions revision 1 stripped off: D-190 records the smallest possible
crossing as **ten pairs** — and that number holds only at `elo1 = 25` (which §9.1
leaves OPEN, and which §9.1 itself contemplates lowering) **and only with no
capped game**. With capped pairs in play it falls to **six**, because a mass point
at a half shrinks the variance and ACCELERATES the LLR, which is D-157's measured
mechanism. This run sets `turn_cap = 40` and a same-kind dry run MEASURED
`capped_fraction 0.125`, so six is the live number. The floor of 100 is safe under
either, but a quoted number carries its conditions and this one had been stripped
of them in a document that then sets a cap and reports the cap count. D-190's own Run 1 crossed at 37 pairs, the action was
WITHHELD, and a confirmatory run on a disjoint book decided it. That precedent is
adopted here in advance rather than discovered again.

---

## 3. The instrument

| | |
|---|---|
| arena | `target/release/arena`, built `--release --locked` at the run's revision |
| engine A | `configs/instrument_staged_v0.toml` — the staged seat |
| engine B | `configs/instrument_v0.toml` — the committed radius-2 policy, unchanged |
| binaries | both seats run `target/release/pistol`, bound by `binary_sha256` (D-283); **OPERATOR-CONFIRM** (§9.2) |
| book | `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` — the PRIMARY SPRT book, 2000 openings, sha-pinned |
| `openings_take` | **OPERATOR-CONFIRM** (§9.3) |
| `openings_skip` | 0 |
| budget | `kind = "nodes"`, `value = 50000` — the registered snapshot budget and D-190's, so the number is comparable with the runs already on the record |
| `turn_cap` | 40, as D-190's runs. **A capped game is NOT excluded from the SPRT sample** — it scores 0.5 into the pentanomial, and D-157 MEASURES that adding capped games ACCELERATES the LLR for a fixed decisive record (a 100-80 record scores 0.2198 with none and 1.1109 with 16000). It is excluded only from `decided_clean`, the first-player-rate denominator. Revision 1 had this backwards. `capped_fraction` and the cap count are reported |
| `n_workers` | 4 — cleared by WP-1.3's own red-team round, which got byte-identical verdict blocks at 1, 2, 4 and 8 workers over a full book through a real early stop |
| `hang_timeout_ms` | 120000 — liveness only, never an adjudication (D-159) |

**Engine identity closes over the config and the weights** (D-198, D-199, D-283):
the arena re-digests each seat's document before every spawn and refuses a
mismatch by name. A rebuild means a re-recorded digest; the digest going stale is
the refusal the binding exists to produce, not a defect in the document.

**The document that IS the experiment** is `configs/arena_wp15b_staged_vs_r2.toml`,
and the report's `experiment_sha256` is what a replication compares — not this
file's path (D-147, D-161).

---

## 4. What the run reports, and which lines are read

**Taken from `crates/pistol-arena/src/conclusion.rs` and `sprt.rs`, not from
memory.** Revision 1 misspelled six of eight lines and keyed §5 on a verdict token
that does not exist; the spellings below are read off the source and the token
list off `Verdict::token`. Above the `# timing` marker, therefore worker-invariant
and byte-comparable:

```
verdict <token>        h1 | h0 | inconclusive_at_game_cap | inconclusive_degenerate | invalid_forfeit
verdict_unit pair
verdict_if_clean <token> pairs_dropped <k>
counts n <n> distinct_n <d> wins_a <w> capped <c> losses_a <l> forfeits <f> decided <k>
capped_fraction <x>
pentanomial p0 <a> p1 <b> p2 <c> p3 <d> p4 <e>
first_player_wins <k> of <n> decided_non_forfeit forfeits <f>[ conditional]
llr_pair last <x>
nelo_pair <x> ci95 <y>
experiment_sha256 <hex>   openings_body_sha256 <hex>   bounds h0 <a> h1 <b>
engine <slot> label … config <path> config_sha256 <hex> weights_sha256 <hex>
engine_id <slot> <line>          — the ENGINE's own handshake, one line per field
```

`inconclusive` is **not** a token. `bounds h0 … h1 …` is a separate line in the
instrument block, not a tail of `llr_pair`. `first_player_wins` is over decided
NON-FORFEIT games with the forfeit count adjacent (D-201), which revision 1 wrote
in its pre-D-201 spelling — apparently copied from a comment in
`configs/arena_wp13_fair_random.toml` rather than from the source it cited.

and below it, machine-dependent and excluded from comparison:
`timing_engine a|b time_ms <t> searches <s>`.

**Per-side compute is a reporting requirement** (rule 6) and is taken from the two
`timing_engine` lines. It is not a verdict input.

---

## 5. Outcome handling, written before game one

Fixed here so that no reading is chosen after the numbers.

| Outcome | Action |
|---|---|
| `verdict h1` **at or above 100 pairs** | The staged policy is accepted as stronger AT THIS BUDGET AND BOOK. The committed config moves to staged in a separate commit, after the run, exactly as D-194 did — never during a live sample |
| `verdict h1` **below 100 pairs** | The action is WITHHELD. A confirmatory run on a DISJOINT sample decides it: the same document with `openings_file` changed to `openings_v1.txt` and `openings_take` re-stated, which is D-190's own instrument. Both intervals are then reported together and both are read as optimistic — D-190 MEASURED coverage at a sequential stop at 0.868 against 0.978 for a run reaching the cap |
| `verdict inconclusive_degenerate` **with `distinct_n == n`** (no duplicate games) and a one-sided pentanomial | **A CLEAN SWEEP, and revision 1 had no action for it.** `Sample::is_degenerate` is `n == 0 \|\| var <= 0` — every pair scoring the SAME, which includes one seat winning every pair. Reproduced by the reviewer with two genuinely different configs and zero duplicate games. This is the strongest possible positive result and revision 1 routed it to "investigate the instrument". The action: treat it as `h1` for the purposes of §5's first two rows, subject to the same 100-pair floor, and record that no LLR exists (`llr_pair last none`) so the crossing cannot be quoted |
| `verdict inconclusive_degenerate` **with `distinct_n == n/2`** | The two seats really did play identical games. For two DIFFERENT configs that is a document or digest error. Investigate the instrument, not the engine. **No early stop is possible on a degenerate sample**, so either row costs the whole book |
| `verdict h0` | The staged policy is NOT accepted. **This is a planning finding, never a threshold move.** See §6 |
| `verdict inconclusive_at_game_cap` | Reported as inconclusive. No action. The sample is reported with its LLR and its distance from both bounds |
| `invalid_forfeit`, or `forfeits > 0` at any verdict | **The run is not a measurement.** It is reported rather than discarded, and re-run only after the forfeit's cause is found. D-158 keeps the result and the manner of it separate; a forfeit is a decided result, so a rate computed over forfeits prints on the very line the run exists to produce |
| A PRE-GAME refusal — `EngineBinaryDigestMismatch`, an openings-digest mismatch, a config refusal, or an `--out` path that exists | **exit 2 and NO REPORT AT ALL.** Revision 1 had no row and asserted in §7 that "every branch is in §5". §9.2 makes the stale-digest branch live by construction: any rebuild at the run's revision changes it. The action is to re-record the digest and re-launch; nothing has been measured |
| `arena_report_aborted` | No verdict exists. The games are a diagnostic and explicitly not a sample (report.rs's own header) |

**The turn cap and the book are part of the claim.** A verdict is about
`random_openings_v1` at 50 000 nodes with a 40-turn cap, and quoting it without
them would be a claim nobody measured (D-190's own discipline).

---

## 6. The honest expectation, and what a negative result means

**This is the change the whole stage exists for.** `docs/ROADMAP.md` Stage 1 is
built around threat-first generation superseding the radius policy, and
`configs/gate_v0.toml`'s committed measurement table is the floor it exists to
move. **Quoted correctly, which revision 1 did not do**: that table reads radius 2
at `depth_turns 3` as **9.7 s** and `> 100 s` at `depth_turns 4`. Revision 1
attributed the `> 100 s` cell to depth 3 — one column left — and the design
document inherits the same misread. The cost is also strongly position-dependent:
MEASURED, radius 2 reaches depth 3 in **1.3 s** on bench position 1, while the
design measures 554 s for a full-width REFERENCE at the same radius and depth on
another position.

**And the design's own measurements do not promise a large effect.** MEASURED from
the committed census, on the BATCHED node population that the quiet cut actually
governs, the adopted option's branching reduction is **2.09×–2.17×** at corpus
depths. That is the honest prior: a real reduction, not a transformative one, and
the depth it buys is roughly logarithmic in it.

**If the run is not SPRT-positive, that is a PLANNING finding and never a licence
to move a threshold, a budget or a book.** The design's non-goals defer
quiescence to WP-1.6 and killers/history/countermove to WP-1.7, and
`docs/ROADMAP.md` says ordering changes are "worth measuring only once the
candidate set they order is the threat-first one" — so a flat result at this
budget is consistent with the staged generator being the necessary substrate for
changes that have not landed. Recording that in advance is what stops it being
said afterwards as an excuse. What it would NOT license: re-reading `elo1`,
changing the budget, changing the book, or running until a favourable stop.

---

## 7. Costs

Stated on the document's own face (D-228, as amended by D-245 and corrected by
D-292), with MEASURED beside DECLARED where a measurement exists (D-290 as
retracted-in-part by D-292 — the reconciliation duty stands, the 5× threshold does
not).

| Item | DECLARED | MEASURED |
|---|---|---|
| One search at the registered budget | ~0.4 s | **396 ms** (radius 2, 50 000 nodes, release, one bench position) |
| One game, both seats, 40-turn cap | ESTIMATED 30–60 s | to be MEASURED by the operator's calibration probe |
| The governed run | ESTIMATED; **OPERATOR-CONFIRM** (§9.3) decides it. For scale D-292 records `arena_wp13_r2_vs_r3.toml`'s 2000-opening shape at **5.44 core-hours, ~82 min wall** — **and that anchor does not transfer**. Its 4.9 s/game came from a probe whose games were ≤ 13 turns; a same-kind reconstruction of THIS document's dry-run matchup MEASURED **16.3 core-seconds per game** over games of 26–40 turns, 3.3× higher, scaling to ≈18 core-hours at the same shape. Per-game cost is matchup- and length-dependent by a factor of three, which is why §9.3 is a slot and not a number | the operator's |
| The calibration probe that fills §9.3 | ESTIMATED ~5 min | **Registered as an instrument** (§9.6), because it produces a number this document registers and CLAUDE.md's clause is not about where an artefact lives |
| The dry run (§8) | ~3 min | to be MEASURED at execution |
| Operator attention | one launch, one report read; no mid-run decision exists — every branch is in §5 | — |

---

## 7A. The second instrument, its agreement criterion, and its consequence

**This is a commitment already on the ADR log, and revision 1 dropped it.** D-245
names this document: *"WP-1.5b's SPRT pre-registration, whose second instrument is
the baseline snapshot's completed-depth change and whose registered consequence is
that on disagreement the work package does not land on the SPRT alone."* Revision
1 provided no second instrument, no criterion, no consequence and no stage under
doubt — which under rule 10 is dropping a recorded commitment silently.

**Is the run cheap or expensive?** Stated in words, because D-245 warns that "a
package that judged itself EXPENSIVE in order to escape the clause would be a
different instance entirely". **This run is EXPENSIVE** — §7's anchor is 82
minutes wall at the 2000-opening shape and §7's own caveat measures a same-kind
matchup at 3.3× that per game. The proportionality rule's replication clause is
written for cheap runs; it does not apply, and this document does not pretend it
does. **The second-instrument duty applies regardless**, because D-245 registers
it for this document by name and because CLAUDE.md's clause is about a doubt, not
about a price.

**THE STAGE UNDER DOUBT**, which CLAUDE.md (D-277) requires be named: everything
between the two engine processes and the printed verdict — the arena's seat
bookkeeping, its pairing, its referee and its scoring. That is the stage the
reviewer's Mutation B lives in, and it is the stage an SPRT number cannot see past
on its own.

**THE SECOND INSTRUMENT: `tools/baseline_snapshot.sh` at `e889b5b`**, reporting
the design's §12.1 registered quantity — per-position `depth_turns` and `nodes` at
50 000 nodes, above the record's `# timing` marker and therefore invariant. **It
does not share the stage under doubt**: the snapshot drives one engine through the
line protocol and reads its own output. No seat, no pairing, no referee, no
scoring.

**THE AGREEMENT CRITERION, registered before either instrument runs**: the staged
seat must not be BOTH SPRT-positive and snapshot-negative. Concretely — if the
SPRT returns `h1`, the snapshot's completed `depth_turns` must be greater than or
equal to the radius config's on at least as many of the 24 bench positions as it
is less than. The two instruments answer different questions (strength against an
opponent; depth at a fixed budget) so exact agreement is not the criterion and
would be the wrong one; DIRECTIONAL disagreement is.

**THE REGISTERED CONSEQUENCE**, verbatim from D-245: on disagreement **the work
package does not land on the SPRT alone**. Concretely: the committed config does
not move, the disagreement is reported with both numbers, and the next step is an
investigation of the stage under doubt — not a re-run and not a re-reading of
either threshold.

## 8. The dry run

CLAUDE.md: "A pre-registration's literal commands are exercised before its review
passes, on an input of the SAME KIND as the registered workload — the same sort of
artefact, differing only in identity — and never on the registered workload
itself. A synthetic stand-in exercises syntax; only a real instance of the kind
exercises ATTRIBUTION."

### 8.1 The input

`configs/arena_wp15b_dryrun.toml` — an arena config, the same kind of artefact as
the registered one, differing in identity on every axis that matters:

- **engines**: `configs/instrument_v0.toml` (radius 2) against
  `configs/gate_v0.toml` (radius 1) — neither is the staged seat under test;
- **book**: `openings_v1.txt`, not the primary `random_openings_v1.txt`;
- **size**: `openings_take = 4`, eight games.

It is a real instance of the kind, not a synthetic stand-in: two genuinely
different engines, played through the real arena, over a real sha-pinned book.

### 8.2 The literal commands

```
cargo build --release --locked --bin arena --bin pistol
sha256sum target/release/pistol
tools/config_check.sh configs/arena_wp15b_dryrun.toml
target/release/arena --config configs/arena_wp15b_dryrun.toml --out <path>
sha256sum configs/instrument_v0.toml configs/gate_v0.toml
printf 'position <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/instrument_v0.toml
printf 'position <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/gate_v0.toml
```

The last three are the dry run's EXTERNAL REFERENTS and are the point of it; the
first four are the syntax it also happens to exercise.

### 8.3 What the output must show, and the defect class each criterion excludes

CLAUDE.md: a criterion must be one the named defect class could FALSIFY, and "an
externally derived referent, a value computed by something that does not share the
suspect input, is the operationalisation that reliably achieves this and is what a
reviewer looks for first".

**Criterion 1 — DEFECT CLASS: SEAT/LABEL ATTRIBUTION INVERSION**, covering the
SCORE path and not only the compute path. The report attributes one seat's games,
score or compute to the other seat's label, so a positive result is read for the
wrong engine.

**Revision 1's criterion failed this class and the failure was BUILT, not
argued.** Mutating `record.rs::score_a`'s `a_is_p1` branches inverts the whole
verdict — `nelo_pair` −992.88 → +992.88, `wins_a 0` → `wins_a 7`, the pentanomial
mirrored — and revision 1's two criteria both PASSED, because both looked only at
`timing_engine` and `config_sha256`. Revision 1 also chose a referent BELOW the
`# timing` marker, which `report.rs` itself declares "excluded from every
comparison".

*The criterion*: for every game, the report's per-game `depth_a` and `depth_b`,
mapped through the `engine <slot> label … config …` lines, must place the
`gate_v0` seat at the GREATER completed depth. These fields are above the timing
marker and are deterministic and worker-invariant.

*The referent, computed outside the arena.* **MEASURED** at the base revision,
release, one bench position at `go nodes 50000`, both reaching `nodes 50176`:

```
configs/instrument_v0.toml  ->  info totals depth_turns 2  time 396  nps 126644
configs/gate_v0.toml        ->  info totals depth_turns 3  time 240  nps 209036
```

The narrower policy completes a deeper iteration at the same node count — which is
D-190's own measured mechanism, and is why radius 2 beat radius 3 there. Nothing
in that measurement passes through the arena.

*Why the defect could falsify it*: under attribution inversion on EITHER path the
label→depth mapping inverts relative to the direct measurement. `depth_a`/`depth_b`
is written per game from the seat that produced it, so a score-path inversion that
leaves `score_a` crediting the wrong label is visible here as a game whose deeper
seat carries the shallower label.

*What was REJECTED as a criterion, and why*: running the config twice with the
seats swapped and requiring the sign to flip is **invariant** under a consistent
inversion — it maps `(seat, label)` the same wrong way both times. That reasoning
survived review and is kept. What did NOT survive is revision 1's replacement,
which covered one half of the class it named.

**Criterion 2 — DEFECT CLASS: THE ENGINE LOADED A DIFFERENT DOCUMENT THAN THE
REPORT NAMES.**

**Revision 1's version of this was vacuous and the arena already refuses its
class.** It compared the report's `config_sha256` against `sha256sum <path>` — but
the arena's digest and its spawn both read the same path string out of the same
document, so the comparison tests `digest_of`, not which file the engine opened:
internal agreement between components sharing the suspect input. And the class as
revision 1 stated it is caught by name: spawning seat b with seat a's config gives
`IdentityDrift … exit 1`, on which revision 1's criterion still passed.

*The criterion*: the **engine's own handshake** — `engine_id <slot>
candidate_policy …`, `engine_id <slot> config <path>`, `engine_id <slot>
tt_bytes <n>` — must differ between the seats in the way the two documents differ.
For the dry run that is `candidate_policy radius 2` against `radius 1`; for the
governed run it is `candidate_policy staged quiet_radius <n> quiet_top_k <k>`
against `candidate_policy radius 2`.

*The referent*: the engine process, which produces those lines from the file it
actually loaded and does not share the arena's reading of the document.

### 8.4 What the dry run records

The input (`configs/arena_wp15b_dryrun.toml` and its sha256), the full output
report, the two external timing measurements, the two config digests, the verdict
on each criterion, and the wall time. Per CLAUDE.md the dry run is NOT a governed
sample and does not consume this pre-registration's first run.

**And it constrains only the dry run's input.** A reviewer may run anything,
including the registered workload; nothing here limits that.

---

## 9. OPERATOR-CONFIRM slots

Every one must be filled before this document governs anything. None has a default
and none may be filled by this session.

**9.1 `elo1`.** The alternative, in normalised Elo on the pair unit. D-190's runs
used `elo1 = 25.0`. The design's MEASURED branching reduction (2.09×–2.17× on the
governed population) is the prior; a smaller alternative buys sensitivity at the
cost of pairs.

**9.2 `binary_sha256`.** Recorded from `sha256sum target/release/pistol` after
`cargo build --release --locked --bin pistol` at the run's revision, for BOTH
seats. Rebuild means re-record.

**9.3 `openings_take`.** With the pair floor at 100 (§2) and the game cap at
`2 × openings_take`, this decides both the sensitivity and the cost. D-292 records
the 2000-opening shape at about 82 minutes wall at 4 workers.

**9.4 The soundness gate's state.** §8 of the design must be GREEN at the revision
this run measures, and the operator confirms it rather than this document assuming
it — because a green SPRT over an unsound generator measures nothing (§1).

**9.5 The calibration probe.** The command, its positions, its budget and its
revision, recorded before it is run — it decides §9.3 and therefore the whole cost
and sensitivity, and revision 1 billed it in §7 while registering it nowhere. It
is NOT part of the sample, as WP-1.3's probe was not.

**9.6 The run's revision.** The commit the games are played at, recorded before the
first game, so the report's `experiment_sha256` has something to be compared with.

---

## 10. What flips this document

An amendment to any section reopens its review, however small the diff — which is
CLAUDE.md's rule and is why the OPERATOR-CONFIRM slots are slots rather than
provisional values this session guessed and a later edit would quietly change.

The claim itself flips on the run. The DOCUMENT flips if the design's §8 soundness
gate is not green at the run's revision (§9.4), if the committed radius policy
moves before the run (which would change what engine B is), or if the arena's
verdict vocabulary changes.

---

## 11. REVIEW STATE — this document ships as a DRAFT, queued for the operator

**Both review cycles this session could run have been spent, and both FAILED.**
The session's standing instruction is that a second failure ships the draft with
both reports attached rather than starting a third amendment cycle, so that is
what this section is. **Nothing here has been run and this document governs
nothing.**

| Cycle | Revision | Verdict |
|---|---|---|
| 1 | rev 1, `af9fa4a` | **FAILS** — 4 BLOCKING, 7 MAJOR |
| 2 | rev 2, `9c068a0` | **FAILS** — 7 BLOCKING, 8 MAJOR, 2 MINOR, 9 REJECTED with reproducers |

### 11.1 The finding that matters most, because revision 2 was built to answer it and did not

Revision 1's dry-run criteria passed on a mutated arena that inverted the whole
verdict. Revision 2 moved the criterion's referent from `timing_engine … time_ms`
(below the record's own "excluded from every comparison" marker) to per-game
`depth_a`/`depth_b` (above it) and asserted that this widened the class to the
SCORE path.

**It did not.** Both fields live on the same `Compute` struct, indexed by engine
slot; `GameRecord::score_a()` is a disjoint function mapping `result` through
`a_is_p1`. Revision 2 changed which FIELD it read, not which PATH. Re-measured
under the same mutation:

```
BASE:       0 W / 7 L for seat A   pentanomial p0 3 p1 1 p2 0 p3 0 p4 0   nelo_pair -992.879886851
MUTATION B: 7 W / 0 L for seat A   pentanomial p0 0 p1 0 p2 0 p3 1 p4 3   nelo_pair +992.879886851
diff of `depth_a … depth_b` across the two reports: IDENTICAL
```

The verdict inverts completely and **both criteria still pass** — as both of
revision 1's did. The complement was built too: mutating the *print* of
`depth_a`/`depth_b` is caught instantly and changes no verdict, so the criterion
is sensitive to exactly one thing — a defect that cannot change the answer.

This is D-305's shape, one document over: a correction applied in one place and
not propagated to the claim that rested on it.

**A criterion that WOULD work, demonstrated by the reviewer and needing no new
code**: reconstruct seat A's per-game score from the `game … p1 <label> … result
<token>` line (above the marker, off the `score_a` path) and require agreement
with `counts wins_a/losses_a`, the `pair … score_a` lines and the pentanomial.
Under Mutation B that derivation disagrees with the report. Stronger still, and
what a reviewer looks for first: re-adjudicate each recorded `moves` line through
`pistol-core` outside the arena — which needs a named harness carrying its own
governing revision.

### 11.2 What the operator must fix before this document governs a run

Verbatim from the second review, in its order of severity. Each is BLOCKING
unless marked.

1. **Rebuild Criterion 1 on the score path** (§11.1).
2. **Re-state Criterion 1 so it passes on an HONEST run.** MEASURED on the real
   dry-run matchup, game 6 of 8 has `depth_a 2 depth_b 2` — a tie — so the
   registered "GREATER" fails on a correct arena. A criterion that fails on the
   honest instrument and passes on the broken one is worse than none.
3. **Create `configs/arena_wp15b_dryrun.toml` and actually run §8.** It has never
   existed in this repository. The reviewer reconstructed and ran it; this session
   did not, and CLAUDE.md requires the literal commands exercised BEFORE the
   review passes.
4. **§5 row 3 must name the DIRECTION.** `Sample::is_degenerate` fires on
   `var <= 0`, which a `p0` sweep satisfies exactly as a `p4` sweep does — so as
   written, §5 ACCEPTS the staged policy on a total defeat. Reproduced: `wins_a 0
   … losses_a 8`, `pentanomial p0 4`, `distinct_n == n`, `verdict
   inconclusive_degenerate`. And the 100-pair floor is no protection, because no
   early stop is possible on a degenerate sample so it always clears the floor.
   `p4 == pairs` → the H1 action; `p0 == pairs` → the `h0` row.
5. **§5 needs a row for a BALANCED degenerate sample** — `distinct_n == n` with
   `p2 == pairs`, which is every pair splitting 1-1 and is the modal outcome for
   two closely matched deterministic engines. Reproduced with `turn_cap = 10`:
   `capped 8`, `pentanomial p2 4`, no registered action. And §7's "every branch is
   in §5" must be struck until it is true.
6. **§7A's second instrument cannot measure the staged seat at the revision it
   names.** `tools/baseline_snapshot.sh` at `e889b5b` hard-codes
   `CONFIG="configs/instrument_v0.toml"` and has no config flag —
   `--config configs/gate_v0.toml` gives `FAIL: unknown argument`. **And the
   design already knows**: its §9 MATRIX M4 ADOPTS adding `--config`, and its
   amendment 4 says the BEFORE run is re-taken under the amended script. So this
   document pins the SUPERSEDED revision of an instrument its own design has
   committed to changing — the tenth instance of D-305's pattern, crossing a
   document boundary this time. Re-point it at the amended script, at the
   revision that lands it.
7. **§7A's agreement criterion is invariant under the stage it names as under
   doubt.** It is conditioned on `h1`, so a score inversion that produces a false
   `h0` never fires it; when it does fire, the staged generator is deeper by
   construction (its whole claim is a 2.09×–2.17× branching reduction), so
   deeper-and-weaker SATISFIES it and a false `h1` is confirmed rather than
   caught; and `depth_turns` is a small integer, so most positions tie and ties
   sit in the `>=` bucket. The two instruments are independent of each other and
   jointly blind to the score path. **The CONSEQUENCE is correct and is D-245's
   verbatim; the criterion is what fails.**
8. **§7A's EXPENSIVE reclassification reverses D-245 without an ADR line.** D-245
   records this package as having judged its run CHEAP and APPLIED the replication
   clause, and warns in the same sentence that "a package that judged itself
   EXPENSIVE in order to escape the clause would be a different instance entirely,
   and it is the one T4 is still waiting for". §7A quotes that warning and then
   performs the move. The re-costing itself is sound — the ≈18 core-hour anchor
   reproduces — but shedding a logged duty unilaterally is the breach revision 2
   convicted revision 1 of, in the other direction. Make it an OPERATOR-CONFIRM
   slot and an ADR line amending D-245's T4 disposition, or keep the clause.
9. MAJOR — **§4 renders three separate report records as one line**
   (`experiment_sha256`, `openings_body_sha256`, `bounds`), in the section whose
   whole purpose is exact spelling and which convicts revision 1 of that same
   error; and it **never spells the `game …` line**, which is the line Criterion 1
   reads, nor the `pair … score_a` line.
10. MAJOR — **§8.3's referent names no position.** The depth and node figures
    reproduce exactly (`depth_turns 2` / `3`, `nodes 50176`); the TIMES do not —
    MEASURED 474 ms / 285 ms against this document's 396 / 240 — which is machine
    variance on a quantity `report.rs` declares non-comparable, and §7 bills the
    396 ms as a MEASURED cost.
11. MAJOR — **§3's capped-game sentence is wrong in detail**: `score::tally`
    excludes capped games from `decided`, `wins_a` and `losses_a`, not only from
    `decided_clean`. The load-bearing half — they score 0.5 into the pentanomial,
    D-157's acceleration — is correct and was verified.
12. MINOR/MAJOR — **§9.3 does not bind `openings_take >= 100`**, so §2's floor is
    unreachable if the slot is filled below it; **§9.5's probe is not required to
    measure worst-case per-search time** against `hang_timeout_ms`, which is
    justified from a radius-2 measurement for a seat whose per-search cost is
    unmeasured; and **§3 cites D-283 without D-294's qualification**.

### 11.3 What survived both rounds, so the operator knows what not to redo

Worker-invariance of the verdict block (MEASURED byte-identical at 1 and 4
workers). Criterion 2 on the `engine_id` handshake — it survives, it simply does
not cover the score path. All nine report-line spellings §4 regenerated from
source, and the `Verdict::token` list including that `inconclusive` is not a
token. §6's corrected `gate_v0` quote and the 1.3 s / 554 s figures. §2's D-190
precedent and its floor arithmetic — the reviewer brute-forced the minimum
crossings independently and got **10** pairs with no capped games and **6** with,
both exact. §3's D-157 capped-game treatment. §7's cost anchor (16.6 core-seconds
per game reproduced against the document's 16.3). The pre-game refusal branch
(exit 2, named error, no report file). And the rejection of the seat-swap
criterion, which is right and should survive any rewrite.

The second reviewer's own summary is the fairest description of where this
document stands: **"The document's engineering is much better than revision 1's.
It fails on where it aimed, not on care."**
