# WP-1.5b — SPRT pre-registration: staged threat-first generation vs the committed radius policy

**Revision 1. DRAFT. THIS DOCUMENT GOVERNS NOTHING YET.**

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

**The pair floor: 100 pairs.** No H1 action fires below it. This is not caution
for its own sake — D-190 records the smallest possible crossing at this
alternative as **ten pairs**, and a config change resting on twenty games is not
one this project makes. D-190's own Run 1 crossed at 37 pairs, the action was
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
| `turn_cap` | 40, as D-190's runs. A capped game has no winner and is excluded from the decided denominator, so a cap that BOUND would silently shrink the sample; the cap count is reported |
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

From `crates/pistol-arena/src/report.rs`, above the `# timing` marker and
therefore worker-invariant and byte-comparable:

```
verdict <token>            h1 | h0 | inconclusive | inconclusive_degenerate | invalid_forfeit
llr_pair <x>   bounds h0 <a> h1 <b>
nelo_pair <x> ci95 +/- <y>
pentanomial <a>/<b>/<c>/<d>/<e>
n <games>   distinct_n <games>   forfeits <k>
first_player_wins <k> of <n> decided
engine a label … config … config_sha256 …
engine b label … config … config_sha256 …
```

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
| `verdict h0` | The staged policy is NOT accepted. **This is a planning finding, never a threshold move.** See §6 |
| `inconclusive` at the game cap | Reported as inconclusive. No action. The sample is reported with its LLR and its distance from both bounds |
| `inconclusive_degenerate` | The two seats played identical games — which for two DIFFERENT configs would mean the configs are not different, i.e. a document or digest error. Investigate the instrument, not the engine |
| `invalid_forfeit`, or `forfeits > 0` at any verdict | **The run is not a measurement.** It is reported rather than discarded, and re-run only after the forfeit's cause is found. D-158 keeps the result and the manner of it separate; a forfeit is a decided result, so a rate computed over forfeits prints on the very line the run exists to produce |
| `arena_report_aborted` | No verdict exists. The games are a diagnostic and explicitly not a sample (report.rs's own header) |

**The turn cap and the book are part of the claim.** A verdict is about
`random_openings_v1` at 50 000 nodes with a 40-turn cap, and quoting it without
them would be a claim nobody measured (D-190's own discipline).

---

## 6. The honest expectation, and what a negative result means

**This is the change the whole stage exists for.** `docs/ROADMAP.md` Stage 1 is
built around threat-first generation superseding the radius policy, and
`configs/gate_v0.toml`'s committed measurement table — radius 2 reaching depth 3
in "> 100 s" — is the floor it exists to move.

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
| The governed run | ESTIMATED from the above and `openings_take`: at 4 workers, **OPERATOR-CONFIRM** (§9.3) decides it. For scale, `configs/arena_wp13_r2_vs_r3.toml`'s 2000-opening shape is recorded by D-292 at **5.44 core-hours, about 82 minutes wall** | the operator's |
| The dry run (§8) | ~3 min | to be MEASURED at execution |
| Operator attention | one launch, one report read; no mid-run decision exists — every branch is in §5 | — |

---

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

**Criterion 1 — DEFECT CLASS: SEAT/LABEL ATTRIBUTION INVERSION.** The report
attributes one seat's games, score or compute to the other seat's label, so a
positive result is read for the wrong engine. This is the class that would make
the governed run's verdict wrong in the most damaging way, silently.

*The criterion*: the report's `timing_engine a time_ms` and `timing_engine b
time_ms`, mapped through the `engine a|b label … config …` lines, must place the
`gate_v0` seat BELOW the `instrument_v0` seat in total time.

*The referent, computed outside the arena*: at the same fixed node budget on one
bench position, **MEASURED** at the base revision, release —
`configs/instrument_v0.toml` **396 ms** (nps 126 644) and `configs/gate_v0.toml`
**240 ms** (nps 209 036), both at `nodes 50176`. Radius 1's nodes are cheaper
because there are fewer candidates to order, so it spends less time for the same
node count. Nothing in that measurement passes through the arena's seat
bookkeeping.

*Why the defect could falsify it*: under attribution inversion the arena's
label→time mapping inverts relative to the direct measurement, and the criterion
fails. Note what was REJECTED as a criterion and why: running the config twice
with the seats swapped and requiring the sign to flip is **invariant** under a
consistent inversion — it maps `(seat, label)` the same wrong way in both runs —
so it would have passed while the defect was present. That is the vacuity
CLAUDE.md forbids, and it was the first criterion this document reached for.

**Criterion 2 — DEFECT CLASS: DOCUMENT BINDING.** The arena plays a config other
than the one its report names, so the run measures an engine nobody registered.

*The criterion*: each `engine <slot> … config <path> config_sha256 <hex>` line's
digest equals `sha256sum <path>` taken separately.

*The referent*: `sha256sum`, which does not share the arena's reading of the file.

*Why the defect could falsify it*: a report naming a document it did not digest
disagrees with the external digest.

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

**9.5 The run's revision.** The commit the games are played at, recorded before the
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
