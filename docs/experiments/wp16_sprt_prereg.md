# WP-1.6 — SPRT pre-registration: threat-only quiescence (`defensive_only`) vs the committed staged policy

**Revision 2. PASSED review (D-400). GOVERNS Step 6's run as of `731150a`,
provided no further amendment lands before launch.**

**Revision 1 (`43b5d78`) FAILED its first fresh-context review** — one
BLOCKING defect and one MAJOR finding, both independently verified by the
reviewer against the live tree (config hashes, the `identity_lines` handshake
claim, a fresh from-scratch re-execution of the entire dry run byte-for-byte,
and the arena's own `Verdict` enum against §5's table — all of it held).
**BLOCKING**: §7A.2's agreement criterion let an `h1` verdict satisfy its
AGREEMENT clause unconditionally ("regardless of the bucket count") while its
own DISAGREEMENT clause was separately defined to fire on the identical
case — a contradiction with no stated precedence, exactly the
after-the-numbers ambiguity CLAUDE.md's registered-consequence rule forbids.
Fixed: the criterion is now a clean partition on the bucket count ALONE,
independent of the verdict; the "tactical correctness needn't mean shallower"
reasoning moved to how a DISAGREEMENT is investigated, not to whether the
criterion fires. **MAJOR**: §2's pair-floor paragraph asserted the 100-pair
floor stays conservative at `elo1 = 15` directionally, without computing
anything — fixed with an actual few-seconds derivation off `sprt.rs`'s own
LLR formula, solving for the `t_hat` D-190's own empirical elo1=25 crossings
imply and re-solving at `elo1 = 15` under the same decisiveness (~16.6 pairs
uncapped, ~10.0 capped — both ESTIMATES, both an order of magnitude under the
floor). This is an amendment, not a fresh draft: it reopens revision 1's
FAILED review exactly as CLAUDE.md's rule requires. §7A.1, §8, and every ADR
citation the reviewer independently re-verified are untouched.

**Provenance, stated so a reviewer does not have to reconstruct it from the ADR
log.** `docs/wp16_quiescence_design.md` revision 5 (`96c856c`) passed its fifth
fresh-context review (`wp16_design_REVIEW_rev5.md`) and closed the design gate
(`docs/decisions.md` **D-394**). Phase 2's IMPL landed the mechanism
(`de05f9c`) and its own rule-5 registered bench — `<= 2.0x` node-ratio pass,
`> 3.0x` ttd-ratio ABORT (**D-388**) — fired the ABORT outright even at the
narrowest tested setting: `q_depth_turns = 1`, both triggers live, node-ratio
2.57x, **ttd-ratio 4.52x** (**D-395**). The architect ruled (**D-396**) that
D-388's bracket stands exactly as fired and is retired as this WP's arbiter;
the governing question becomes whether the extension's STRENGTH gain outweighs
its MEASURED cost, a question only an SPRT can answer, and gated the wider,
costlier of the two triggers behind a new `q_triggers` config enum so the
FIRST SPRT tests the cheapest coherent hypothesis: `defensive_only` — trigger
(b) alone, never trigger (c). That delta landed (`8fe5abd`) and passed a
scoped fresh-context REVIEW-impl (**D-397**). A second, narrower rule-5-style
bench at `defensive_only`/`q_depth_turns = 1` measured node-ratio 2.48x,
**ttd-ratio 2.80x** (**D-398**) — cheaper than the full-trigger figure, exactly
as D-396's own rationale for gating the wider arm first predicts, but recorded
as MEASURED ACCOUNTING and explicitly **not** read against D-388's retired
bracket.

**This document is what D-396 calls Step 5**: the pre-registration for the
governed run Step 6 launches. It is a fresh draft, not an amendment — WP-1.6
has never had a prereg before this one — but it deliberately reuses
`docs/experiments/wp15b_sprt_prereg.md`'s revision-6 structure and its
Criterion 1' instrument verbatim, per the dispatch's own instruction, rather
than re-deriving a pattern this project already fought five review rounds to
get right.

**The run is this session's, per D-396's own delegation record**: "operator
authorization for this dispatch's slot-filling and launch actions (Steps 3-6,
including the governed SPRT run) is GRANTED and delegated to this session."
Unlike WP-1.5b's document, `elo1`, `openings_take` and `alpha`/`beta` are
therefore not **OPERATOR-CONFIRM** slots here — the dispatch fixes them
directly (§9.1, §9.3) — and this document's own §9 states plainly which slots
that delegation covers and which two (the run's binary digest and revision)
can only be filled at Step 6's own launch, by the same "rebuild means
re-record" discipline every SPRT document in this project uses.

---

## 1. What is being judged, and what is not

**The claim.** Threat-only, zone-bounded quiescence at the search horizon,
gated to trigger (b) only (`docs/wp16_quiescence_design.md` §3.2,
`q_triggers = "defensive_only"`, `q_depth_turns = 1`) is stronger than the
committed plain staged policy (`configs/instrument_staged_v0.toml`,
`q_depth_turns = 0` — the extension compiled in but never granted) at a fixed
node budget.

**What this run cannot judge, said first because rule 6 is about what a number
means.** SPRT judges STRENGTH, not soundness. Soundness is
`docs/wp16_quiescence_design.md`'s own gate — five fresh-context design
reviews, closed at D-394 — and Phase 2's correctness judges (`cargo test
--workspace`, `clippy -D clippy::all`, `tools/determinism.sh`,
`tools/perft_check.sh`, `tools/search_oracle_check.sh`, D-395's own header). A
green SPRT over an unsound extension is a measurement of nothing, and the
order matters here exactly as it did in WP-1.5b: §9.4 makes the design's gate
being green at the run's revision an explicit slot rather than an assumption.

**One axis separates the two seats, not several.** Unlike WP-1.5b (three axes:
selection, visibility, search value), this matchup differs on exactly one:
whether the horizon may extend by one further TURN under trigger (b) alone.
Candidate generation (staged tiers, the quiet cut), the quiet radius, and
everything else `configs/instrument_staged_v0.toml` and
`configs/instrument_staged_q_defensive_only_v0.toml` share is held fixed —
**CONFIRMED by diff, this session**: the two documents differ in exactly ONE
line, `q_depth_turns` (`0` vs `1`); `q_triggers` already reads
`"defensive_only"` on the committed baseline too (D-397's own committed
configs), so this run's ENTIRE claim is carried by that single key going from
`0` (the extension compiled in but never granted) to `1` (one turn of
extension granted under trigger (b)). **A verdict here is therefore a verdict
on trigger (b) plus its ply-1/ply-2 realisation and THE COMPLETION STONE, not
on the wider trigger (c) — D-396's own scoping, restated so a reader does not
read more into an `h1` than the matchup supports.**

---

## 2. The hypothesis and the verdict unit

- **H0**: `elo0 = 0` — the quiescence-extended seat is no stronger.
- **H1**: `elo1 = 15.0` — **FIXED by D-396's dispatch, not an OPERATOR-CONFIRM
  slot** (contrast WP-1.5b §9.1). Lower than WP-1.5b's `25.0`: that run judged
  a structural generation change with a MEASURED 2.09x-2.17x branching
  reduction as its prior; this run's own prior is D-395/D-398's cost
  accounting alone, which predicts no comparable branching effect — quiescence
  buys tactical accuracy at the horizon, not a narrower tree, and a smaller
  alternative is the honest target for that kind of gain (§6 expands this).
- `alpha = 0.05`, `beta = 0.05` — the values every prior SPRT run in this
  project used, kept for the same reason WP-1.5b kept them: two documents
  should not appear to disagree about a convention.
- **Verdict unit: the PAIR.** Both games of an opening are played, colours
  reversed, and the pentanomial pair outcome is the sample — `sprt.rs`'s own
  unit, unchanged from WP-1.5b.

**The pair floor: 100 pairs**, the same standing convention D-190 established
and every SPRT document in this project since has used (D-386's own
confirmatory run cleared it at 51 pairs). **Not re-derived from scratch for
`elo1 = 15`, but the directional claim IS computed rather than asserted** —
found in seconds from `crates/pistol-arena/src/sprt.rs`'s own LLR formula
rather than left as a hand-wave (CLAUDE.md's own "an estimate that could have
been measured in seconds is a finding" instruction), since a first draft of
this document did leave it as a directional assertion and that is a real gap
a reviewer named. D-190's "ten pairs" (uncapped) and "six pairs" (capped) at
`elo1 = 25` are EMPIRICAL crossings, not closed-form minimums — the LLR's
denominator `t1 * t_hat - t1^2 / 2` is unbounded as the sample variance
shrinks, so no first-principles minimum exists independent of an assumed
pentanomial shape. **What is computable**: solving `n * (t1(25) * t_hat -
t1(25)^2/2) = h1` for the `t_hat` D-190's own two empirical crossings imply
(`h1 = ln(0.95/0.05) = 2.944…`, this run's own bound too — alpha/beta
unchanged), then re-solving for `n` at `t1(15)` under the SAME implied
`t_hat` (the same DECISIVENESS of sample, not the same effect size): the
uncapped shape crosses at **~16.6 pairs** and the capped shape at **~10.0
pairs**. Both ESTIMATES, not measured — they assume this run's own sample
would be exactly as decisive as D-190's, which nothing here confirms — but
both are a full order of magnitude under the 100-pair floor, so the floor's
own protection (§5's confirmatory-run row fires below it regardless of where
the true minimum sits) holds with room, not merely by directional assertion.

---

## 3. The instrument

| | |
|---|---|
| arena | `target/release/arena`, built `--release --locked` at the run's revision |
| engine A | `configs/instrument_staged_q_defensive_only_v0.toml` — the quiescence-extended seat, label `staged_q` |
| engine B | `configs/instrument_staged_v0.toml` — the committed plain staged policy, unchanged, label `staged` |
| binaries | both seats run `target/release/pistol`, bound by `binary_sha256` (D-283 as qualified by D-294); recorded at Step 6's launch (§9.2) |
| book | `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` — the PRIMARY SPRT book, 2000 openings, sha-pinned |
| `openings_take` | **500 — FIXED by D-396's dispatch**, not an OPERATOR-CONFIRM slot. Bound at `>= 100` by §2's floor; comfortably clears it |
| `openings_skip` | 0 |
| budget | `kind = "nodes"`, `value = 50000` — the registered snapshot budget every SPRT document in this project uses, so this run's numbers are comparable with the record. **NOT the same convention D-398 measured under**: D-398's own cost accounting used a FIXED-DEPTH methodology (`go depth_turns 3`, reading nodes/time to reach it), the inverse of the arena's FIXED-NODES-per-move convention this run uses. §7A.2's own agreement criterion is the instrument that reconnects the two conventions for THIS run's own matchup, at THIS budget |
| `turn_cap` | 40, as every prior run in this project |
| `n_workers` | 4 — WP-1.3's red-team clearance, unchanged |
| `hang_timeout_ms` | 120000 — liveness only, never an adjudication (D-159). **Freshly calibrated for THIS seat, §9.5** |

**Engine identity closes over the config and the weights** (D-198, D-199,
D-283 as qualified by D-294): the arena re-digests each seat's document before
every spawn and refuses a mismatch by name.

**The document that IS the experiment** is
`configs/arena_wp16_defensive_only_vs_staged.toml`, authored and committed at
Step 6's own launch (mirroring D-147/D-161: the report's `experiment_sha256`
is what a replication compares, not this file's path) — not authored here,
since it needs the launch-time binary digest §9.2 records.

---

## 4. What the run reports, and which lines are read

**Identical to WP-1.5b §4** — the same arena, the same `conclusion.rs`/
`report.rs`, unchanged by this WP. Reproduced here rather than referenced, per
this project's own "a pre-registration states what it needs on its own face"
practice:

```
arena_report <schema>
arena_version <v>
experiment_sha256 <hex>
openings_file <path>
openings_body_sha256 <hex>
openings_take <k> of <total>
openings_skip <k>
opening_turns <t>
game_cap <n>
budget <kind> <value>
turn_cap <t>
sprt elo0 <a> elo1 <b> alpha <c> beta <d>
bounds h0 <a> h1 <b>
engine <slot> label <l> binary <p> binary_sha256 <hex> config <p> config_sha256 <hex> weights_sha256 <hex>
engine_id <slot> <line>
game <i> opening <o> p1 <label> p2 <label> result <token> end <normal|forfeit> forfeit_by <label|none> reason <token|none> turns <t> dup_of <j|none> nodes_a <n> nodes_b <n> depth_a <d> depth_b <d> llr_game <x|none> llr_pair <x|none>
moves <i> <turn> <turn> …
refusal <i> <verbatim>
pair <i> opening <o> bucket p<k> score_a <x>
counts n <n> distinct_n <d> wins_a <w> capped <c> losses_a <l> forfeits <f> decided <k>
pentanomial p0 <a> p1 <b> p2 <c> p3 <d> p4 <e>
capped_fraction <x>
first_player_wins <k> of <n> decided_non_forfeit forfeits <f>[ conditional]
llr_game last <x|none>
llr_pair last <x|none>
nelo_pair <x> ci95 <y>
verdict <token>
verdict_unit pair
verdict_if_clean <token|none> pairs_dropped <k>
```

and below the `# timing` marker, machine-dependent and excluded from every
comparison:

```
timing n_workers <w> wall_ms <t> discarded_in_flight <k> hang_timeout_ms <t>
timing config_sha256 <hex>
timing_engine <slot> time_ms <t> searches <s>
```

**Per-side compute is a reporting requirement** (rule 6), taken from the two
`timing_engine` lines and reported alongside the verdict, not as an input to
it.

---

## 5. Outcome handling, written before game one

Fixed here so that no reading is chosen after the numbers — the same table
shape WP-1.5b registered and D-386 executed exactly as written.

| Outcome | Action |
|---|---|
| `verdict h1` **at or above 100 pairs** | The `defensive_only` extension is accepted as stronger AT THIS BUDGET AND BOOK. Per D-396's own FLIP clause, the WP closes: the committed instrument config moves to `staged_q(defensive_only, q_depth_turns=1)` in a separate commit after the run, subject to the closure gate in §6 below — never during a live sample. `defensive_and_offensive` is recorded as licensed-not-scheduled future work, exactly as D-396 states |
| `verdict h1` **below 100 pairs** | The action is WITHHELD. A confirmatory run on the DISJOINT book decides it: the same document with `openings_file` changed to `crates/pistol-cli/tests/fixtures/openings_v1.txt` (1591 openings) and `openings_take` re-stated, D-190's own instrument, exactly as D-385/D-386 executed for WP-1.5b. Both intervals are then reported together and both are read as optimistic (D-190's own coverage measurement) |
| `verdict inconclusive_degenerate` **with `distinct_n == n`** | Read the direction off the pentanomial, per WP-1.5b's own corrected reading (`Sample::is_degenerate` fires on a `p0` sweep exactly as on a `p4` sweep). Let `k` be the one occupied pentanomial slot. `k = 3` or `k = 4` → the H1 row above, subject to the same 100-pair floor. `k = 2` → the `h0` row — the modal outcome for two closely matched deterministic engines, and also what an all-capped run produces; report `capped_fraction` beside `k` since the two causes are different findings about the same number. `k = 0` or `k = 1` → the `h0` row. In every case `llr_pair last none`, so no crossing is quoted |
| `verdict inconclusive_degenerate` **with `distinct_n == n/2`** | The two seats played identical games — a document or digest error for two seats that differ in `q_depth_turns`/`q_triggers` by construction. Investigate the instrument, not the engine |
| `verdict h0` | The extension is NOT accepted. **This is a planning finding, never a threshold move.** Per D-396's FLIP clause: the WHOLE WP returns to the architect, with BOTH narrowing (a cheaper move-set width than `defensive_only` already is) and shelving (WP-1.6 parked, unimplemented in the committed instrument) explicitly on the table — this document does not pre-select between them. See §6 |
| `verdict inconclusive_at_game_cap` | Reported as inconclusive. No action. The sample is reported with its LLR and its distance from both bounds |
| `invalid_forfeit`, or `forfeits > 0` at any verdict | **The run is not a measurement.** Reported rather than discarded; re-run only after the forfeit's cause is found (D-158) |
| A PRE-GAME refusal — digest mismatch, an openings-digest mismatch, a config refusal, or an `--out` path that exists | **exit 2 and NO REPORT AT ALL.** Re-record the digest (§9.2) and re-launch; nothing has been measured |
| `arena_report_aborted` | No verdict exists. The games are a diagnostic and explicitly not a sample |
| **Criterion 1' fails on the GOVERNED report** (§7A, Doubt 1) | **The run is not a measurement, and it is not an `h0` either.** The verdict is not read. See §7A.1 for the consequence |
| **A robustness FAILURE under Criterion 1' clause (b)** | Per D-384's own flip clause: warm-replay (licensed, not built) is needed before any verdict from that run is read. This is a hard stop, per D-396's own restatement that "the warm-replay flip live" applies unchanged to this run |

**The turn cap and the book are part of the claim.** A verdict is about
`random_openings_v1` at 50 000 nodes with a 40-turn cap, and quoting it
without them would be a claim nobody measured.

---

## 6. The honest expectation, and what a negative result means

**This is a narrower, cheaper-to-be-wrong-about claim than WP-1.5b's.**
`docs/wp16_quiescence_design.md`'s own non-goal list defers the wider trigger
(c) and the whole of Stage 3's forcing search to later work; this run judges
only whether the narrowest defensible slice — one gated defensive trigger,
one turn of extension — already buys enough tactical accuracy to be worth
D-398's measured 2.80x ttd cost.

**The design's own headline claim is qualitative, not a branching-reduction
number.** Unlike WP-1.5b, there is no MEASURED reduction to quote as a prior
here — quiescence is not a narrowing device, it is a horizon-correctness
device. The one piece of direct, MEASURED evidence this document can offer
came out of this session's own dry run (§8.4): on bench position 1
(`src 00070cdd8fb87f42`), the wider `defensive_and_offensive` seat found a
`-mate 5` at `depth_turns 2` using 716 of its 50 000-node budget, where the
plain staged seat used the full budget to reach `depth_turns 3` and reported
`cp 180` — the seat WITHOUT quiescence did not see the mate at all at this
budget. That is a single anecdote, on the WIDER trigger, off the dry run's
matchup, and is recorded as illustrative context only (§8.4 states this
again) — but it is the shape of gain the design predicts: tactical
accuracy the static horizon eval misses, not depth.

**If the run is not SPRT-positive, that is a PLANNING finding and never a
licence to move a threshold, a budget or a book.** D-395's own finding already
prices in that the mechanism is sound and the width is costly; an `h0` here
would say the narrowest, cheapest width still does not buy enough to clear
`+15` normalised Elo against a 2.80x compute tax — informative about WHERE the
trade-off sits, not a verdict on whether threat-only quiescence as a class is
worth pursuing. Recording that in advance is what stops it being said
afterwards as an excuse. What it would NOT license: re-reading `elo1`,
changing the budget, changing the book, or running until a favourable stop.

---

## 7. Costs

Stated on the document's own face, DECLARED beside MEASURED wherever a
measurement exists.

| Item | DECLARED | MEASURED |
|---|---|---|
| One search at the registered budget, `defensive_only` seat | — | D-398: summed over 22 comparable bench positions at `depth_turns = 3`, node-ratio **2.48x**, ttd-ratio **2.80x** against the plain staged seat |
| The calibration probe (§9.5) | ~2 min | **MEASURED, this session, two independent 24-position sweeps at `go nodes 50000`**: worst single search 291 ms (run 1) / 289 ms (run 2). `hang_timeout_ms = 120000` leaves a **~412x** margin over the larger figure — R-9.5d's own `~24x` convention clears with wide room, discharged NO-CHANGE |
| The dry run (§8) | ~3 min | **MEASURED: 14.254 s of arena wall time** at 4 workers over 8 games, plus the release build (already current), the two external referent searches and the Criterion 1' replay |
| Criterion 1' on the governed report | — | 4 searches per pair replayed at the registered budget — minutes at `openings_take = 500` |
| The governed run | ESTIMATED, scaled from D-398's own node-ratio against D-292's radius-policy anchor (5.44 core-hours / ~82 min wall at 2000 openings) — at `openings_take = 500` and roughly 2.5x the per-search cost of a plain-staged-only matchup, **ESTIMATE ~2-3 core-hours, ~35-50 min wall at 4 workers** | the run itself, Step 6 |
| Operator/session attention | one launch, one report read, one Criterion 1' run; every branch is in §5 | — |

---

## 7A. The doubts, their instruments, their agreement criteria and their consequences

### 7A.1 DOUBT 1 — the arena between the engines and the verdict

**THE STAGE UNDER DOUBT**: everything between the two engine processes and the
printed verdict — the arena's seat bookkeeping, its pairing, its referee and
its scoring. Identical stage to WP-1.5b's Doubt 1: this WP changes which
engine sits in which seat, not the arena code that scores them.

**THE INSTRUMENT: `tools/wp15b_attribution_check.py`, unmodified, at its
current revision** — engine-agnostic by construction (it reads labels and
moves out of the report the arena wrote, not out of anything specific to
staged-vs-radius). **Verified this session, not merely asserted**: run against
this WP's own dry-run report (§8.4), it PASSED — `1a: 16 turns replayed, 10
discriminating, 8 of 8 games directly attributed`, `1a robustness: no vacuous
pairs`, `1b: 5 decided non-forfeit games`, `1c: 8 games and 4 pairs`, `PASS —
0 failure(s)`, exit 0. A change to this script reopens this document's review
exactly as an amendment would (CLAUDE.md's instrument rule).

**CRITERION 1', quoted verbatim from D-384 (WP-1.5b's own registration,
reused rather than re-derived)**: "A report is a measurement iff (a) zero
confirmed inversions under links 1b/1c applied to all games, and (b) the
verdict is invariant under adversarial reassignment of every link-1a-vacuous
pair, recomputed from the report's own pentanomial and LLR machinery."

**THE REGISTERED CONSEQUENCE**: identical to WP-1.5b's — a non-zero exit means
THE RUN IS NOT A MEASUREMENT, the verdict is not read, the committed config
does not move, and per D-384's flip clause a robustness failure or a confirmed
inversion coinciding with a vacuous opening stops the WP for warm-replay to be
built before any verdict is read.

### 7A.2 DOUBT 2 — whether the extension changes what the search completes

This is WP-1.6's analogue of WP-1.5b's D-245 doubt, rebuilt for a mechanism
that predicts the OPPOSITE direction: WP-1.5b's staged generator narrows the
tree and should reach a DEEPER completed iteration at fixed nodes; this WP's
quiescence extension spends nodes AT the horizon rather than narrowing the
tree above it, so the registered prediction is **the same or a SHALLOWER**
completed `depth_turns` at the fixed 50 000-node budget, not a deeper one.

**THE INSTRUMENT: `tools/baseline_snapshot.sh --config`, now available at the
closed revision D-387 records** (`9282dd0`) — WP-1.5b's own Doubt 2 instrument
could not measure the staged seat at all at its authoring revision; this WP
inherits a working `--config` flag and needs no further instrument work.

**HOW IT DOES NOT SHARE THE STAGE**: identical to WP-1.5b's own reasoning —
the snapshot drives ONE engine through the line protocol and reads its own
output. No seat, no pairing, no referee, no scoring.

**THE AGREEMENT CRITERION, registered before either instrument runs, two-sided
and tie-aware — over the 24 bench positions, at `go nodes 50000` (the arena's
own budget), count the `defensive_only` seat's completed `depth_turns` against
the plain staged seat's in three buckets: strictly greater, equal, strictly
less.**

**THE PARTITION IS ON THE BUCKET COUNT ALONE, independent of the SPRT
verdict** — CLAUDE.md's own registered-consequence rule forbids a criterion
that leaves an after-the-numbers choice standing, and a verdict-conditioned
branch here would do exactly that (a first draft of this section made this
mistake and a fresh-context review caught it — recorded rather than silently
fixed, since an amendment reopens the review it corrects):

- **AGREEMENT** is the `defensive_only` seat STRICTLY GREATER on AT MOST 6 of
  the 24 positions — mostly same-or-shallower, consistent with the
  registered prediction that the extension spends nodes at the horizon rather
  than narrowing the tree above it.
- **DISAGREEMENT** is the `defensive_only` seat STRICTLY GREATER on MORE THAN
  6 of the 24 positions — the search is seeing DEEPER rather than more
  accurately, contrary to the mechanism the design claims.

6/24 = 25% is chosen as the complement of the ~75% "strictly-less-or-equal"
threshold a first draft of this criterion used and is JUDGED, not measured,
stated so a reviewer can attack it: D-398's own measured 2.48x node-ratio at
fixed depth predicts the inverse relationship holds at fixed nodes on MOST
positions, and 24 × 0.25 = 6 leaves margin below the ~92% (22/24) comparable
fraction D-398 itself measured for positions where the extra ply-1/ply-2
search still completes within budget.

**THE REGISTERED CONSEQUENCE**: on DISAGREEMENT, the work package does not
land on the SPRT alone, REGARDLESS OF THE VERDICT — the committed config does
not move even on an `h1` verdict, the disagreement is reported with both
numbers and all three buckets, and the next step is investigation, not a
re-run and not a re-reading of either threshold. **An `h1` verdict that also
falls in the DISAGREEMENT bucket is not read as confirming quiescence's
tactical-accuracy claim** — a real strength gain riding on a mechanism that is
making the search see deeper rather than more accurately is not the claim
§1 registers, and needs the same investigation a disagreeing `h0` would. This
is where the "tactical correctness at whatever depth is reached, not depth
itself" reasoning belongs — in how a DISAGREEMENT is investigated, not in
whether the criterion fires — so the criterion itself stays a clean partition
on one observable and a reader cannot pick a reading after seeing the
numbers.

**WHAT THIS INSTRUMENT STILL CANNOT SEE**: it is blind to the arena's score
path, as the SPRT is — precisely why Doubt 1 has its own instrument, and
precisely why running only one of the two would be one instrument reported
twice (CLAUDE.md's own "two instruments blind to the same stage" rule).

**TO BE RUN AT STEP 6**, alongside the governed SPRT run, not as part of this
document's own dry run (§8's dry run answers ATTRIBUTION syntax and the
Criterion 1'/2/3 chain over a DIFFERENT matchup by construction — see §8.1).

---

## 8. The dry run

CLAUDE.md: "A pre-registration's literal commands are exercised before its
review passes, on an input of the SAME KIND as the registered workload — the
same sort of artefact, differing only in identity — and never on the
registered workload itself."

**IT HAS NOW BEEN RUN.**

### 8.1 The input

`configs/arena_wp16_dryrun.toml` — an arena config, the same kind of artefact
as the registered one, differing in identity on every axis that matters:

- **engines**: `configs/instrument_staged_q_defensive_and_offensive_v0.toml`
  (label `staged_q_both`) against `configs/instrument_staged_v0.toml` (label
  `staged`) — the WIDER trigger arm, never the registered `defensive_only`
  candidate;
- **book**: `openings_v1.txt`, not the primary `random_openings_v1.txt`;
- **size**: `openings_take = 4`, eight games.

Deliberately the SAME as the governed run: the budget, the turn cap, the
worker count and the hang timeout, so the commands are exercised against the
shapes they will meet.

It is a real instance of the kind, not a synthetic stand-in: two genuinely
different engines (D-397's own unit tests already establish
`defensive_and_offensive` and `defensive_only` diverge behaviourally), played
through the real arena, over a real sha-pinned book.

### 8.2 The literal commands

`<scratch>` is any directory outside the repository (rule 8 — match logs are
artifacts). The `--out` path must not exist.

```
cargo build --release --locked --bin arena --bin pistol
sha256sum target/release/pistol
tools/config_check.sh configs/arena_wp16_dryrun.toml
target/release/arena --config configs/arena_wp16_dryrun.toml --out <scratch>/wp16_dryrun_report.txt
sha256sum configs/instrument_staged_q_defensive_and_offensive_v0.toml configs/instrument_staged_v0.toml
printf 'position start moves <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/instrument_staged_q_defensive_and_offensive_v0.toml
printf 'position start moves <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/instrument_staged_v0.toml
python3 tools/wp15b_attribution_check.py <scratch>/wp16_dryrun_report.txt target/release/pistol
```

`<P>` is BENCH POSITION 1 of `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`
(`src 00070cdd8fb87f42`, 15 stones) — the same position D-376/D-395/D-398 all
cite:

```
0,0 -1,1/1,0 0,1/0,2 -1,0/1,-1 0,-1/1,-2 0,-2/0,3 -1,-1/1,1 -1,2/-1,3
```

**`tools/wp15b_attribution_check.py` IS AN INSTRUMENT, NAMED HERE WITH ITS
REVISION** — the commit this document lands at. A change to it reopens this
document's review exactly as an amendment to the document does.

### 8.3 What the output must show, and the defect class each criterion excludes

#### Criterion 1' — DEFECT CLASS: SEAT/LABEL ATTRIBUTION INVERSION

Reused verbatim from WP-1.5b (§7A.1 above). **RESULT: PASS**, `0 failure(s)`,
exit 0 — see §8.4.

#### Criterion 2 — DEFECT CLASS: THE ENGINE LOADED A DIFFERENT DOCUMENT THAN THE REPORT NAMES

**WP-1.5b's own version of this criterion does not transfer to this WP, and
running the dry run is what found that out — exactly what the dry-run rule
exists for.** WP-1.5b's Criterion 2 reads the engine's own handshake line,
`engine_id <slot> candidate_policy …`, and requires it to differ between the
seats in the way the two documents differ. **MEASURED, this session**:
`crates/pistol-cli/src/bin/pistol.rs`'s `identity_lines` function (the
handshake's source) emits `candidate_policy staged quiet_radius <n>
quiet_top_k <k>` under `CandidatePolicy::Staged` — and, by the function's own
doc comment, "`tier_t_own_count`, `tier_t_opponent_count` and `widen_schedule`
do not ride on this line — U2-M item 2 names only `quiet_radius` and
`quiet_top_k`," a limitation that predates this WP and that WP-1.6's own
`q_depth_turns`/`q_triggers` fields were never added to either. **The dry
run's own report confirms this concretely**: both engines' `engine_id …
candidate_policy` lines read byte-identical —
`candidate_policy staged quiet_radius 2 quiet_top_k 16` — despite the two
configs genuinely differing in `q_depth_turns` (`1` vs `0`). **As WP-1.5b's
own Criterion 2 is written, it is VACUOUS for every matchup this WP can
register**: it would pass identically whether the engine actually loaded its
own named config or its sibling's, which is exactly the "a property the named
defect class PRESERVES" failure CLAUDE.md's criterion rule names.

**THIS CRITERION IS THEREFORE DEMOTED, not silently ported** — the same
treatment WP-1.5b gave its own Criterion 3 when an instrument could not
support a pass/fail reading. **What stands in for it**: Criterion 1's Link 1a
(§7A.1) already replays each engine's first free search as a genuinely
separate process, keyed to the report's own named config path, and D-397's
own unit tests establish that `defensive_only`, `defensive_and_offensive` and
plain staged are behaviourally distinct engines on real positions — so a
document swap between any two of this WP's seats would, on most openings,
also fail Link 1a's non-vacuity or its move-match check. **This is weaker than
a direct check and the gap is named rather than hidden**: a swap that happened
to produce the identical first move on a given opening's two free turns would
pass Link 1a undetected, where a genuine Criterion 2 would still catch it via
the handshake. **WHAT WOULD CLOSE THIS GAP, recorded rather than acted on**:
extending `identity_lines` to also emit `q_depth_turns`/`q_triggers` under
`Staged` — a small change to a PINNED handshake (`pistol.rs`'s own comment:
"its handshake is pinned byte-for-byte against the pre-WP-1.4 revision"),
which is a protocol amendment and not something this Step-5 authoring session
takes unilaterally. Licensed-not-scheduled, alongside `defensive_and_offensive`
itself (D-396).

#### Criterion 3 — THE EXTERNAL DEPTH REFERENT, demoted to what it can support

Context only, never a pass/fail gate on attribution — WP-1.5b's own framing,
kept. **MEASURED, this session, at bench position 1**: the `staged_q_both`
seat (`defensive_and_offensive`) returned `depth_turns 2`, `nodes 716`, `score
-mate 5` — an early stop on a FOUND MATE, using under 1.5% of the 50 000-node
budget. The plain `staged` seat returned `depth_turns 3`, `nodes 50176`
(the full budget), `score cp 180`. **This is the SAME degenerate,
mate-early-stop category D-395 and D-398 both name and exclude from any
depth-bucket tally** — a candidate finding a genuine forced win the baseline's
static horizon eval cannot see, terminating early with FEWER total nodes, not
a tree-narrowing effect. It is recorded here as illustrative context for §6's
honest expectation, not as a Criterion-3-style bucket count: with one position
in the sample and that position degenerate by the same rule §7A.2 already
names, no aggregate bucket comparison is meaningful from the dry run alone.
§7A.2's own bucket criterion is registered for the GOVERNED run's 24-position
sweep at Step 6, not for this single dry-run anecdote.

### 8.4 What the dry run recorded

Full report at `<scratch>/wp16_dryrun_report.txt`
(sha256 `d617ae7d5ad1cc9827a871c62df5ce54f4818dff7402d494765f3b93d2e38ae9`, not
committed — rule 8), produced by
`target/release/arena --config configs/arena_wp16_dryrun.toml`:

```
arena_report 4
experiment_sha256 62e2608c4d0fb9c128d98d00d24e362b9162c7cc2e5d2da7f99896875aab237f
openings_file crates/pistol-cli/tests/fixtures/openings_v1.txt
openings_take 4 of 1591
budget nodes 50000
turn_cap 40
sprt elo0 0.000000000 elo1 15.000000000 alpha 0.050000000 beta 0.050000000
engine a label staged_q_both config configs/instrument_staged_q_defensive_and_offensive_v0.toml config_sha256 bf2bb20cfc703bd65d6f96713c64707fdc1bafc840ca7f4ba97ddc567ea81785
engine b label staged config configs/instrument_staged_v0.toml config_sha256 986957cdcc72f67e44fcbc0a8c0b3ac3783fc954e89038fe1d794f09d806b825
binary_sha256 (both seats) b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673
counts n 8 distinct_n 8 wins_a 2 capped 3 losses_a 3 forfeits 0 decided 5
pentanomial p0 0 p1 1 p2 3 p3 0 p4 0
capped_fraction 0.375000000
verdict inconclusive_at_game_cap
```

**`verdict inconclusive_at_game_cap` at 4 pairs is EXPECTED and is not a
finding** — the dry run cannot and is not meant to cross an SPRT boundary at
these bounds; what it exercises is the commands and the criteria, per §8's own
opening quote.

**Criterion 1' (§8.3), run against this report**:

```
attribution_check: 1a: 16 turns replayed, 10 of them discriminating, 8 of 8 games directly attributed by replay
attribution_check: 1a robustness: no vacuous pairs — clause (b) holds trivially
attribution_check: 1b: 5 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 8 games and 4 pairs rebuilt off the score_a path
attribution_check: PASS — 0 failure(s)
```

exit 0. `tools/wp15b_attribution_check.py`, unmodified, correctly attributes
this WP's engines — it reads labels out of the report, not out of anything
staged-vs-radius-specific, so this confirms the instrument transfers before
Step 6 relies on it.

### 8.5 What the dry run is and is not

It is a real instance of the kind, and it found a real, load-bearing gap in a
reused criterion (§8.3's Criterion 2) that a synthetic stand-in would not have
surfaced — the whole justification for CLAUDE.md's "only a real instance of
the kind exercises ATTRIBUTION" rule. It is not a measurement of either
engine's strength, and nothing in §8.4 may be quoted as one.

---

## 9. FILL-IN slots

Per D-396's delegation record, most of what WP-1.5b left as OPERATOR-CONFIRM
is fixed directly by the dispatch that authorises this document. Two remain
genuinely deferred — not to an operator, but to Step 6's own launch, because
they can only be known then.

**9.1 `elo1`.** FIXED at `15.0` (§2), by D-396's dispatch. Not re-opened here.

**9.2 `binary_sha256`.** Recorded from `sha256sum target/release/pistol`
after `cargo build --release --locked --bin pistol` at Step 6's own launch
revision, for BOTH seats. Rebuild means re-record.

**9.3 `openings_take`.** FIXED at `500` (§3), by D-396's dispatch. Clears the
100-pair floor (§2) with wide margin, matching WP-1.5b's own governed run
shape so the two are comparable in scale.

**9.4 The design gate's state.** `docs/wp16_quiescence_design.md`'s own review
sequence is GREEN as of D-394, at a revision this document's own provenance
paragraph names; Step 6 confirms `git log` shows no further amendment to that
document between D-394 and the run's own launch revision before treating this
slot as still discharged.

**9.5 The calibration probe.** Discharged, this session, per §7 —
`hang_timeout_ms = 120000` leaves a ~412x margin over the MEASURED worst
single search (291 ms) of the `defensive_only` seat at the registered
50 000-node budget, over two independent 24-position sweeps. NO-CHANGE, per
R-9.5d's own convention.

**9.6 The run's revision.** The commit Step 6's games are played at, recorded
before the first game, so the report's `experiment_sha256` has something to
be compared with.

**9.7 `configs/arena_wp16_defensive_only_vs_staged.toml`.** Authored and
committed at Step 6's launch, once §9.2/§9.6 are known — "the document that IS
the experiment" (§3).

---

## 10. What flips this document

An amendment to any section reopens its review, however small the diff. It
binds the instruments too: `tools/wp15b_attribution_check.py` and
`tools/baseline_snapshot.sh` are named here with their revisions, and editing
either reopens this review exactly as an amendment would.

The claim itself flips on the run (§5). The DOCUMENT flips if
`docs/wp16_quiescence_design.md`'s own gate is not green at the run's revision
(§9.4), if the committed staged policy or its `q_depth_turns`/`q_triggers`
defaults move before the run (which would change what engine B is), if the
arena's verdict vocabulary changes, or if `identity_lines` gains the
`q_depth_turns`/`q_triggers` fields §8.3's Criterion 2 finding names as the
fix — at which point Criterion 2 is re-registered as a real gate rather than
staying demoted, and that re-registration itself reopens this review.

---

## 11. REVIEW STATE

**REVISION 2 (`731150a`) PASSES.** Revision 1 (`43b5d78`) FAILED its first
fresh-context review (one BLOCKING, one MAJOR — see the provenance paragraph
at the top). Revision 2 fixed both and reopened the review; a second
fresh-context reviewer, scoped to the fix, found zero findings against it —
PASS, recorded verbatim at `docs/decisions.md` **D-400**. **This document now
GOVERNS Step 6's run**, at revision `731150a`, provided no further amendment
lands before launch (an amendment reopens this review exactly as revision 1's
did). §9.2/§9.6/§9.7's remaining slots are filled at Step 6's own launch.
