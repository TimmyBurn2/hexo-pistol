# WP-1.5b — SPRT pre-registration: staged threat-first generation vs the committed radius policy

**Revision 3. DRAFT. THIS DOCUMENT GOVERNS NOTHING YET.**

**Revision 2 (`9c068a0`) FAILED its review — 7 BLOCKING, 8 MAJOR, 2 MINOR, 9 REJECTED
with reproducers.** Revision 3 applies all twelve numbered fixes and repairs the
dry-run criteria first, in the order the operator set. §12 is the FIX MAP: every fix
number against the section that carries it and the evidence it rests on.

**What changed that matters most.** Revision 1's dry-run criteria passed on a mutated
arena that inverted the whole verdict. Revision 2 moved the referent and asserted the
class was widened; the second reviewer BUILT the counter-example and it was not. §8's
Criterion 1 is now a CHAIN OF THREE LINKS running from *which engine actually chose
the moves* to *the printed verdict*, each link carrying a referent that does not share
the path it checks. It has been RUN — on the honest dry run, which it passes, and on
three separately mutated arenas, one per link, each of which it fails. §8.5 is that
table.

**Revision 2 also performed a reversal it had no licence for**, reclassifying this run
EXPENSIVE against D-245's record of it as CHEAP, in a paragraph that quoted D-245's
warning about exactly that move. The reclassification is now the operator's, on the
ADR log, as **D-307**; §7A cites it rather than performing it.

It names `configs/instrument_staged_v0.toml`, which does not exist at this revision, so
it cannot govern a run and no run has been taken under it. It becomes governing when
three things hold together: the config exists, every **OPERATOR-CONFIRM** slot in §9 is
filled, and the document passes a fresh-context review AT THE REVISION THAT GOVERNS THE
RUN (CLAUDE.md: "the revision that governs a run must itself pass a fresh-context
review before the first run it governs. Reviews of superseded revisions do not
transfer — an amendment reopens the review, however small the diff").

**The run is the operator's.** This session delivers the document, its dry run and its
review state; it plays no game.

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
mechanism. This run sets `turn_cap = 40` and the dry run of §8 MEASURED
`capped_fraction 0.125` on a same-kind matchup, so six is the live number. The
floor of 100 is safe under either, but a quoted number carries its conditions and
this one had been stripped of them in a document that then sets a cap and reports
the cap count. D-190's own Run 1 crossed at 37 pairs, the action was WITHHELD, and
a confirmatory run on a disjoint book decided it. That precedent is adopted here in
advance rather than discovered again.

The floor is also what §9.3 is bound by: an `openings_take` below 100 makes the
floor unreachable and is refused there rather than discovered afterwards.

---

## 3. The instrument

| | |
|---|---|
| arena | `target/release/arena`, built `--release --locked` at the run's revision |
| engine A | `configs/instrument_staged_v0.toml` — the staged seat |
| engine B | `configs/instrument_v0.toml` — the committed radius-2 policy, unchanged |
| binaries | both seats run `target/release/pistol`, bound by `binary_sha256` (D-283 **as qualified by D-294**); **OPERATOR-CONFIRM** (§9.2) |
| book | `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` — the PRIMARY SPRT book, 2000 openings, sha-pinned |
| `openings_take` | **OPERATOR-CONFIRM** (§9.3), bound at `>= 100` by §2's floor |
| `openings_skip` | 0 |
| budget | `kind = "nodes"`, `value = 50000` — the registered snapshot budget and D-190's, so the number is comparable with the runs already on the record |
| `turn_cap` | 40, as D-190's runs. **A capped game is NOT excluded from the SPRT sample** — see below. `capped_fraction` and the cap count are reported |
| `n_workers` | 4 — cleared by WP-1.3's own red-team round, which got byte-identical verdict blocks at 1, 2, 4 and 8 workers over a full book through a real early stop |
| `hang_timeout_ms` | 120000 — liveness only, never an adjudication (D-159). Its margin is justified against the STAGED seat's worst single search, which §9.5 requires the probe to measure |

**What a capped game does to the statistic, corrected.** Revision 2 wrote that
`score::tally` excludes capped games "only from `decided_clean`". READ OFF THE
SOURCE: the whole `decided` / `decided_clean` / `wins_a` / `losses_a` block sits
inside `tally`'s non-capped branch, so a capped game increments `capped` and
NOTHING ELSE. It is excluded from `decided`, from `wins_a` and from `losses_a`
alike. The load-bearing half is unchanged and was verified: `game_sample` is built
from `(wins_a, capped, losses_a)`, so capped games enter the statistic as HALVES,
and D-157 MEASURES that adding them ACCELERATES the LLR for a fixed decisive record
(a 100-80 record scores 0.2198 with none and 1.1109 with 16000).

**Engine identity closes over the config and the weights** (D-198, D-199, D-283 as
qualified by D-294): the arena re-digests each seat's document before every spawn
and refuses a mismatch by name. D-294's qualification is cited because it is
load-bearing here and revision 2 omitted it — D-283's "a mismatched build never
becomes a process" is FALSE as written for a `binary` naming no path separator, and
this document's seats therefore name `target/release/pistol` with a separator. A
rebuild means a re-recorded digest; the digest going stale is the refusal the
binding exists to produce, not a defect in the document.

**The document that IS the experiment** is `configs/arena_wp15b_staged_vs_r2.toml`,
and the report's `experiment_sha256` is what a replication compares — not this
file's path (D-147, D-161).

---

## 4. What the run reports, and which lines are read

**Taken from `crates/pistol-arena/src/conclusion.rs` and `report.rs`, not from
memory, and ONE RECORD PER LINE.** Revision 1 misspelled six of eight lines;
revision 2 corrected the spellings and then rendered three separate records
(`experiment_sha256`, `openings_body_sha256`, `bounds`) as one, in the section
whose whole purpose is exact spelling. Every record below is its own line, in the
order the report writes them. Above the `# timing` marker, therefore
worker-invariant and byte-comparable:

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

and below it, machine-dependent and excluded from comparison:

```
# timing — machine- and schedule-dependent; excluded from every comparison
timing n_workers <w> wall_ms <t> discarded_in_flight <k> hang_timeout_ms <t>
timing config_sha256 <hex>
timing_engine <slot> time_ms <t> searches <s>
```

`verdict <token>` is one of `h1`, `h0`, `inconclusive_at_game_cap`,
`inconclusive_degenerate`, `invalid_forfeit`. **`inconclusive` is not a token.**
`bounds h0 … h1 …` is a separate line in the instrument block, not a tail of
`llr_pair`. `first_player_wins` is over decided NON-FORFEIT games with the forfeit
count adjacent (D-201), which revision 1 wrote in its pre-D-201 spelling. `refusal`
appears only for a game that drew one, and its free-text field is last so it needs
no quoting.

**The `game`, `moves` and `pair` lines are what §8's Criterion 1 reads**, which is
why they are spelled here and were not in revision 2 — the section that named
Criterion 1's referent never wrote down the record it comes from.

**Per-side compute is a reporting requirement** (rule 6) and is taken from the two
`timing_engine` lines. It is not a verdict input.

---

## 5. Outcome handling, written before game one

Fixed here so that no reading is chosen after the numbers.

| Outcome | Action |
|---|---|
| `verdict h1` **at or above 100 pairs** | The staged policy is accepted as stronger AT THIS BUDGET AND BOOK. The committed config moves to staged in a separate commit, after the run, exactly as D-194 did — never during a live sample |
| `verdict h1` **below 100 pairs** | The action is WITHHELD. A confirmatory run on a DISJOINT sample decides it: the same document with `openings_file` changed to `openings_v1.txt` and `openings_take` re-stated, which is D-190's own instrument. Both intervals are then reported together and both are read as optimistic — D-190 MEASURED coverage at a sequential stop at 0.868 against 0.978 for a run reaching the cap |
| `verdict inconclusive_degenerate` **with `distinct_n == n`** | **READ THE DIRECTION OFF THE PENTANOMIAL. Revision 2 did not, and as written it ACCEPTED the staged policy on a total defeat** — `Sample::is_degenerate` is `n == 0 \|\| var <= 0`, which a `p0` sweep satisfies exactly as a `p4` sweep does (reproduced by the reviewer: `wins_a 0 … losses_a 8`, `pentanomial p0 4`, `distinct_n == n`). A degenerate sample with no duplicate games has EXACTLY ONE occupied slot; call it `k`. **`k = 3` or `k = 4`** → the H1 row above, subject to the same 100-pair floor. **`k = 2`** → the `h0` row: every pair split 1-1, which is the modal outcome for two closely matched deterministic engines and is also what an all-capped run produces (reproduced at `turn_cap = 10`: `capped 8`, `pentanomial p2 4`). **`k = 0` or `k = 1`** → the `h0` row. In every case record that no LLR exists (`llr_pair last none`) so no crossing can be quoted, and report `capped_fraction` beside `k`, since `k = 2` from capping and `k = 2` from balanced play are different findings about the same number. The 100-pair floor is no protection on its own here and is not relied on as one: no early stop is possible on a degenerate sample, so such a run always reaches the cap and always clears the floor |
| `verdict inconclusive_degenerate` **with `distinct_n == n/2`** | The two seats really did play identical games. For two DIFFERENT configs that is a document or digest error. Investigate the instrument, not the engine. **No early stop is possible on a degenerate sample**, so either row costs the whole book |
| `verdict h0` | The staged policy is NOT accepted. **This is a planning finding, never a threshold move.** See §6 |
| `verdict inconclusive_at_game_cap` | Reported as inconclusive. No action. The sample is reported with its LLR and its distance from both bounds |
| `invalid_forfeit`, or `forfeits > 0` at any verdict | **The run is not a measurement.** It is reported rather than discarded, and re-run only after the forfeit's cause is found. D-158 keeps the result and the manner of it separate; a forfeit is a decided result, so a rate computed over forfeits prints on the very line the run exists to produce |
| A PRE-GAME refusal — `EngineBinaryDigestMismatch`, an openings-digest mismatch, a config refusal, or an `--out` path that exists | **exit 2 and NO REPORT AT ALL.** §9.2 makes the stale-digest branch live by construction: any rebuild at the run's revision changes it. The action is to re-record the digest and re-launch; nothing has been measured |
| `arena_report_aborted` | No verdict exists. The games are a diagnostic and explicitly not a sample (report.rs's own header) |
| **Criterion 1 fails on the GOVERNED report** (§7A, Doubt 1) | **The run is not a measurement, and it is not an `h0` either.** The verdict is not read at all. See §7A for the consequence and what is done next |

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
| One search at the registered budget | ~0.4 s | **388 ms** — radius 2 at `go nodes 50000`, release, on BENCH POSITION 1 of `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` (`src 00070cdd8fb87f42`, 15 stones), reaching `nodes 50176`. Revision 2 billed 396 ms and named no position; see §8.3 for why the TIME is the wrong half of that measurement to bill |
| One game, both seats, 40-turn cap | ESTIMATED 30–60 s | **~5.3 s** for the radius-2-vs-radius-1 dry-run matchup at 4 workers (42.2 s wall over 8 games), and **16.2 core-seconds** of engine time per game — (70887 + 58576) ms over 8 games. The STAGED seat's per-game cost is UNMEASURED — §9.5's probe is what measures it |
| The governed run | ESTIMATED; **OPERATOR-CONFIRM** (§9.3) decides it. For scale D-292 records `arena_wp13_r2_vs_r3.toml`'s 2000-opening shape at **5.44 core-hours, ~82 min wall** — **and that anchor does not transfer**. Its 4.9 s/game came from a probe whose games were ≤ 13 turns; the same-kind dry run below MEASURED **16.3 core-seconds per game** over games of 26–40 turns, 3.3× higher, scaling to ≈18 core-hours at the same shape. Per-game cost is matchup- and length-dependent by a factor of three, which is why §9.3 is a slot and not a number | the operator's |
| The calibration probe that fills §9.3 | ESTIMATED ~5 min | **Registered as an instrument** (§9.5), because it produces a number this document registers and CLAUDE.md's clause is not about where an artefact lives |
| The dry run (§8) | ~3 min | **MEASURED: 42.2 s of arena wall time** at 4 workers, plus the release build, the two external referent searches and the Criterion 1 replay (32 further searches at the same budget) |
| Criterion 1 on the governed report | — | 4 searches per pair replayed at the registered budget. At the dry run's per-search cost that is about 0.3 s per pair, i.e. minutes at any `openings_take` §9.3 can hold |
| Operator attention | one launch, one report read, one Criterion 1 run; **no mid-run decision exists — every branch is in §5** | — |

The "every branch is in §5" claim is made here only because §5 now carries the
degenerate-direction row, the balanced-degenerate case and the Criterion 1 row it
was missing. Revision 2 asserted it while three branches had no row.

---

## 7A. The doubts, their instruments, their agreement criteria and their consequences

**Is the run cheap or expensive?** **EXPENSIVE, on the operator's ruling D-307**,
which reverses the CHEAP judgment D-245 records for this package on the measured
per-game cost above. This document CITES that ruling; it does not perform it.
Revision 2 performed it, in a paragraph that quoted D-245's warning that "a package
that judged itself EXPENSIVE in order to escape the clause would be a different
instance entirely" — which is the breach revision 2 convicted revision 1 of, in the
other direction.

What lapses with the reclassification is the proportionality rule's REPLICATION
clause, which is written for cheap runs. **What does not lapse is the
second-instrument duty**, because D-245 registers it for this document by name and
because CLAUDE.md's clause is about a DOUBT, not about a price. D-307 says so in
the same words.

CLAUDE.md (D-277) requires that a second instrument NAME THE STAGE UNDER DOUBT and
say how it does not share it, on the ground that "two instruments blind to the same
stage are one instrument reported twice". Revision 2 named a stage and then
registered an instrument blind to it. **There are two distinct doubts here and they
need two instruments; conflating them is what produced a criterion invariant under
the very stage it named.**

### 7A.1 DOUBT 1 — the arena between the engines and the verdict

**THE STAGE UNDER DOUBT:** everything between the two engine processes and the
printed verdict — the arena's seat bookkeeping, its pairing, its referee and its
scoring. That is the stage the reviewer's Mutation B lives in, and it is the stage
an SPRT number cannot see past on its own.

**THE INSTRUMENT: `tools/wp15b_attribution_check.py`, at the revision this document
lands at**, run over the GOVERNED report and not only over the dry run. It is §8's
Criterion 1 and it is described in full there.

**HOW IT DOES NOT SHARE THE STAGE**, link by link: 1a's referent is the ENGINE
PROCESS, replayed outside the arena; 1b's is GAME RULE 3 read against the recorded
move list; 1c's is the `game`-line path, which `conclusion.rs::games` writes from
`a_is_p1` directly and which is disjoint from `GameRecord::score_a`. None of the
three is computed by the code it checks.

**THE AGREEMENT CRITERION, registered before either instrument runs:** the
instrument exits 0 over the governed report, having reported a non-zero
discriminating count on link 1a and a non-zero adjudicated count on link 1b. Those
two counts are the non-vacuity guard and the instrument refuses on them itself; the
criterion names them so that a reader does not have to take the exit code's word
for the check having had anything to bite on.

**THE REGISTERED CONSEQUENCE:** a non-zero exit means **THE RUN IS NOT A
MEASUREMENT** — not an `h0`, not an `h1`, and not a re-run. The verdict is not
read. The failing link and its games are reported, the committed config does not
move, and the next step is finding the defect the link names. Exit 2 is the
instrument saying it could not take the answer, which is a VOID and not a finding
(tools/SHELL_CHECKLIST.md item 12).

### 7A.2 DOUBT 2 — whether the staged generator changes what the search completes

This is the doubt D-245 registers the baseline snapshot against, and it is a
different question from Doubt 1. It is kept, with its criterion rebuilt.

**THE INSTRUMENT: `tools/baseline_snapshot.sh` AT THE REVISION THAT LANDS ITS
`--config` FLAG** — **OPERATOR-CONFIRM** (§9.7). Revision 2 pinned `e889b5b`, which
**cannot measure the staged seat at all**: MEASURED at this document's revision,
`tools/baseline_snapshot.sh --config configs/gate_v0.toml` answers
`baseline_snapshot: FAIL: unknown argument --config`, the script hard-coding
`CONFIG="configs/instrument_v0.toml"` with no flag to move it. The design already knows — its §9 MATRIX M4
ADOPTS adding `--config` and its amendment 4 re-takes the BEFORE run under the
amended script — so revision 2 pinned the superseded revision of an instrument its
own design had committed to changing. The slot is a revision and not a path for
exactly that reason.

**HOW IT DOES NOT SHARE THE STAGE:** the snapshot drives ONE engine through the
line protocol and reads its own output. No seat, no pairing, no referee, no
scoring. It reports the design's §12.1 registered quantity — per-position
`depth_turns` and `nodes` at 50 000 nodes, above the record's `# timing` marker and
therefore invariant.

**THE AGREEMENT CRITERION, registered before either instrument runs, two-sided and
tie-aware.** Over the 24 bench positions, count the staged config's completed
`depth_turns` against the radius config's in THREE buckets — strictly greater,
EQUAL, strictly less. Ties are reported in their own bucket and are not counted as
agreement in either direction; revision 2's `>=` put them all on the agreeing side
of a small integer, which is most of the positions.

The design's MEASURED 2.09×–2.17× branching reduction is a claim about what the
staged generator does to the tree, and at a fixed node budget it predicts a deeper
completed iteration. **The registered prediction: STRICTLY GREATER on at least 12
of the 24 positions.** That number is **JUDGED, not measured**, and its ground is
stated so a reviewer can attack it: D-190 MEASURED that the radius-3-to-radius-2
narrowing moved completed second iterations from 6 of 24 to 17 of 24, and a
2.09×–2.17× reduction that buys a deeper iteration on fewer than half the bench is
not the mechanism the design measured. It may be moved before the run and not
after.

- **AGREEMENT** is `h1` with ≥ 12 strictly greater, or a non-`h1` verdict with < 12.
- **DISAGREEMENT** is either cross: `h1` with fewer than 12 — strength without the
  mechanism the design claims — or `h0` with 12 or more — the mechanism without the
  strength. Revision 2's criterion fired on neither: it was conditioned on `h1`, so
  a false `h0` never reached it, and deeper-and-weaker SATISFIED it.

**THE REGISTERED CONSEQUENCE**, verbatim from D-245: on disagreement **the work
package does not land on the SPRT alone**. Concretely: the committed config does
not move, the disagreement is reported with both numbers and all three buckets, and
the next step is an investigation — not a re-run and not a re-reading of either
threshold.

**WHAT THIS INSTRUMENT STILL CANNOT SEE, and why that is not a defect here:** it is
blind to the arena's score path, as the SPRT is. That is precisely why Doubt 1 has
its own instrument, and it is why revision 2's single pairing failed CLAUDE.md's
test — two instruments blind to the same stage are one instrument reported twice.

---

## 8. The dry run

CLAUDE.md: "A pre-registration's literal commands are exercised before its review
passes, on an input of the SAME KIND as the registered workload — the same sort of
artefact, differing only in identity — and never on the registered workload itself.
A synthetic stand-in exercises syntax; only a real instance of the kind exercises
ATTRIBUTION."

**IT HAS NOW BEEN RUN.** Revisions 1 and 2 described a dry run over a config file
that had never existed in this repository. `configs/arena_wp15b_dryrun.toml` is
committed at this document's revision and everything in §8.4 and §8.5 was produced
by running it.

### 8.1 The input

`configs/arena_wp15b_dryrun.toml` — an arena config, the same kind of artefact as
the registered one, differing in identity on every axis that matters:

- **engines**: `configs/instrument_v0.toml` (radius 2, seat A, label `r2`) against
  `configs/gate_v0.toml` (radius 1, seat B, label `r1`) — neither is the staged
  seat under test;
- **book**: `openings_v1.txt`, not the primary `random_openings_v1.txt`;
- **size**: `openings_take = 4`, eight games.

Deliberately the SAME as the governed run: the budget, the turn cap, the worker
count and the hang timeout, so the commands meet the shapes they will meet.

It is a real instance of the kind, not a synthetic stand-in: two genuinely
different engines, played through the real arena, over a real sha-pinned book.

### 8.2 The literal commands

`<repo>` is the repository root and `<scratch>` any directory outside it. The
`--out` path must not exist; the arena refuses one that does.

```
cargo build --release --locked --bin arena --bin pistol
sha256sum target/release/pistol
tools/config_check.sh configs/arena_wp15b_dryrun.toml
target/release/arena --config configs/arena_wp15b_dryrun.toml --out <scratch>/dryrun.txt
sha256sum configs/instrument_v0.toml configs/gate_v0.toml
printf 'position start moves <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/instrument_v0.toml
printf 'position start moves <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/gate_v0.toml
tools/wp15b_attribution_check.py <scratch>/dryrun.txt target/release/pistol
```

`<P>` is BENCH POSITION 1 of `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`
(`src 00070cdd8fb87f42`, 15 stones):

```
0,0 -1,1/1,0 0,1/0,2 -1,0/1,-1 0,-1/1,-2 0,-2/0,3 -1,-1/1,1 -1,2/-1,3
```

**`tools/wp15b_attribution_check.py` IS AN INSTRUMENT AND IS NAMED HERE WITH ITS
REVISION** — the commit this document lands at. CLAUDE.md: a change to it reopens
this document's review exactly as an amendment to the document does. It carries a
test driving the shipped file,
`crates/pistol-cli/tests/wp15b_attribution_check_tests.rs`
(tools/SHELL_CHECKLIST.md item 10), which corrupts a report once per link and once
in the way that would make link 1a pass vacuously.

The last four commands are the dry run's EXTERNAL REFERENTS and are the point of
it; the first four are the syntax it also happens to exercise.

### 8.3 What the output must show, and the defect class each criterion excludes

CLAUDE.md: a criterion must be one the named defect class could FALSIFY, and "an
externally derived referent, a value computed by something that does not share the
suspect input, is the operationalisation that reliably achieves this and is what a
reviewer looks for first".

#### Criterion 1 — DEFECT CLASS: SEAT/LABEL ATTRIBUTION INVERSION

The report attributes one seat's games, score or compute to the other seat's label,
so a positive result is read for the wrong engine.

**Why revisions 1 and 2 both failed this class, and it was BUILT rather than
argued.** Mutating `record.rs::score_a`'s `a_is_p1` branches inverts the whole
verdict — `nelo_pair` −992.88 → +992.88, `wins_a 0` → `wins_a 7`, the pentanomial
mirrored — and both of revision 1's criteria passed, because both looked only at
`timing_engine` and `config_sha256`. Revision 2 moved the referent to per-game
`depth_a`/`depth_b` and asserted that widened the class to the score path. It did
not: both fields live on the same `Compute` struct indexed by engine slot, while
`score_a` is a disjoint function mapping `result` through `a_is_p1`. Revision 2
changed which FIELD it read, not which PATH; the reviewer re-measured `depth_a … depth_b`
across the two reports and got IDENTICAL. Revision 2's criterion also failed on the
HONEST instrument: it required the `gate_v0` seat at the strictly GREATER depth,
and game 6 of 8 has `depth_a 2 depth_b 2` — a tie — which this session reproduced.
A criterion that fails on the honest run and passes on the broken one is worse than
none.

**THE CRITERION IS NOW A CHAIN OF THREE LINKS**, running from which engine actually
chose the moves to the printed verdict. It uses no depth field at all, so the tie
that broke revision 2 is irrelevant to it.

**Link 1a — LABEL → ENGINE.** The two free turns after the book are each some
engine's FIRST search of that game, so a fresh process reproduces them exactly. For
each game, the engine the report NAMES as that turn's mover must return the move
the report RECORDS. *The referent*: the engine process, run outside the arena.
*Non-vacuity*: two engines that answer identically satisfy this under any
labelling, so the instrument counts the turns on which they DISAGREE and refuses if
that count is zero.

**Link 1b — MOVES → RESULT.** Game rule 3: turn 1 is one stone, every later turn is
two, and a win completes the instant a placed stone forms six — so the winner
played the LAST turn, and `result p1_win` holds exactly when the recorded turn
count is odd. Applied to decided non-forfeit games only. *The referent*: a pinned
game rule read against the recorded move list, computed by nothing in the arena.

**Link 1c — RESULT → SCORE → VERDICT.** Seat A's per-game score is rebuilt from
`game <i> … p1 <label> p2 <label> … result <token>` and required to agree with
`counts wins_a/losses_a/capped`, with every `pair … bucket p<k> score_a <x>`, and
with the pentanomial. *The referent*: the other code path —
`conclusion.rs::games` writes the labels from `a_is_p1` directly, and every number
it is checked against comes through `GameRecord::score_a`.

**What the chain still cannot see**, said rather than left for a reviewer to find:
it reads the report the arena wrote. It cannot tell whether a game was LEGAL — that
the recorded stones form six in a line — because that is adjudication and needs
pistol-core. Link 1b checks only that the seat credited is the seat that moved
last. Re-adjudicating every `moves` line through pistol-core is the stronger
instrument and it is NOT built here: it needs a harness carrying its own governing
revision, which is a work item and not a criterion repair.

**What was REJECTED as a criterion, and why**: running the config twice with the
seats swapped and requiring the sign to flip is **invariant** under a consistent
inversion — it maps `(seat, label)` the same wrong way both times. That reasoning
survived both reviews and is kept.

#### Criterion 2 — DEFECT CLASS: THE ENGINE LOADED A DIFFERENT DOCUMENT THAN THE REPORT NAMES

**Revision 1's version was vacuous and the arena already refuses its class.** It
compared the report's `config_sha256` against `sha256sum <path>` — but the arena's
digest and its spawn both read the same path string out of the same document, so
the comparison tests `digest_of`, not which file the engine opened: internal
agreement between components sharing the suspect input. And the class as revision 1
stated it is caught by name: spawning seat b with seat a's config gives
`IdentityDrift … exit 1`, on which revision 1's criterion still passed.

*The criterion*: the **engine's own handshake** — `engine_id <slot>
candidate_policy …`, `engine_id <slot> config <path>`, `engine_id <slot>
tt_bytes <n>` — must differ between the seats in the way the two documents differ.
For the dry run that is `candidate_policy radius 2` against `radius 1`; for the
governed run it is `candidate_policy staged quiet_radius <n> quiet_top_k <k>`
against `candidate_policy radius 2`.

*The referent*: the engine process, which produces those lines from the file it
actually loaded and does not share the arena's reading of the document.

#### Criterion 3 — THE EXTERNAL DEPTH REFERENT, demoted to what it can support

The two direct searches measure `depth_turns 2` for radius 2 and `depth_turns 3`
for radius 1 at the same node count, on the named bench position. That is D-190's
own measured mechanism and it is worth recording — but it is **not** a criterion on
attribution, which is what revision 2 tried to make it. It is registered as a
CONTEXT measurement: the run's `depth_a`/`depth_b` distribution must be consistent
with it in aggregate, and a wholesale contradiction (say, the radius-2 seat deeper
in every game) is a finding to investigate, not a pass/fail gate. **The TIMES are
not part of any criterion**: `report.rs` declares them non-comparable, this session
measured 388 ms / 235 ms where revision 2 recorded 396 / 240 and the reviewer 474 /
285 on the same tree — three machines, one quantity, and §7 bills the DEPTH and
NODE figures rather than the milliseconds for that reason.

### 8.4 What the dry run recorded

**Input.** `configs/arena_wp15b_dryrun.toml`,
sha256 `25527dba44f689db3002b2ba1be22868f7953c24288f9e2e912c20feb106927d`.
`tools/config_check.sh` accepted it: `validate_arena_config: … ok`, exit 0.
`target/release/pistol` sha256
`a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce`, which is the
digest the config binds — the seats spawned the build that was measured.

**The report** (written to a scratch path; a match log is an artifact and is never
committed, CLAUDE.md rule 8). Verdict block, the lines §5 reads:

```
experiment_sha256 253ccb073501daa1a8fe1ddb6e42ec138d1bef48517bb13e62f7a937fa75f9c9
openings_body_sha256 e5165ce15b51847f90167ac3e26d4bc700278cb511600ce82cda1f466925c0a9
counts n 8 distinct_n 8 wins_a 0 capped 1 losses_a 7 forfeits 0 decided 7
pentanomial p0 3 p1 1 p2 0 p3 0 p4 0
capped_fraction 0.125000000
first_player_wins 3 of 7 decided_non_forfeit forfeits 0
llr_pair last -1.665756788
nelo_pair -992.879886851 ci95 240.760577378
verdict inconclusive_at_game_cap
verdict_unit pair
```

and below the marker: `timing n_workers 4 wall_ms 42203 discarded_in_flight 0
hang_timeout_ms 120000`, `timing_engine a time_ms 70887 searches 112`,
`timing_engine b time_ms 58576 searches 115`. **Four openings cannot cross a
boundary at these bounds and this verdict is not a result about either engine.**
The 16.3 core-seconds per game §7 bills is (70887 + 58576) / 8 000.

**The external referents**, both configs at `go nodes 50000` on the bench position
above, each reaching `nodes 50176`:

```
configs/instrument_v0.toml  ->  info totals depth_turns 2 seldepth 3 nodes 50176 nps 129297 time 388
configs/gate_v0.toml        ->  info totals depth_turns 3 seldepth 4 nodes 50176 nps 213117 time 235
```

**Criterion 1**, on the honest report:

```
attribution_check: 1a: 16 turns replayed, 8 of them discriminating
attribution_check: 1b: 7 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 8 games and 4 pairs rebuilt off the score_a path
attribution_check: PASS — 0 failure(s)          exit 0
```

**Criterion 2**: `engine_id a candidate_policy radius 2` against `engine_id b
candidate_policy radius 1`, and `engine_id a tt_bytes 268435456` against
`engine_id b tt_bytes 16777216` — the two documents differing in the way the
handshake reports.

**Criterion 3**: consistent. Every one of the 8 games has the `r1` seat at a
completed depth greater than or equal to the `r2` seat's, and game 6 at a tie —
which is the tie that broke revision 2's criterion and is why depth is context here
and not a gate.

**Wall time**: 42.2 s for the arena, seconds for everything else.

### 8.5 Criterion 1 against three separately mutated arenas

Each mutation was applied in a SEPARATE GIT WORKTREE (CLAUDE.md: a mutation is a
deliberate break and never goes in the live tree), built `--release --locked`, run
over the same `configs/arena_wp15b_dryrun.toml`, and the resulting report handed to
the shipped instrument. The mutations touch only `pistol-arena`; the engine binary
digest is unchanged at
`a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce` in every case,
so link 1a's referent is the same engine throughout.

| Report | Mutation | 1a | 1b | 1c | exit |
|---|---|---|---|---|---|
| honest | none | 0 | 0 | 0 | **0 PASS** |
| Mutation B | `record.rs::score_a`, `a_is_p1` branches inverted | 0 | 0 | **10** | **1 FAIL** |
| Mutation C | `conclusion.rs::games`, p1/p2 labels swapped | **8** | 0 | **10** | **1 FAIL** |
| Mutation D | `record.rs::GameResult::token`, p1_win/p2_win swapped | 0 | **7** | **10** | **1 FAIL** |

Mutation B is the one both earlier revisions blessed. It inverts the entire verdict
— `0 W / 7 L` → `7 W / 0 L`, `pentanomial p0 3 p1 1` → `p3 1 p4 3`, `nelo_pair`
−992.879886851 → +992.879886851 — and link 1c refuses it on ten separate records.
Mutation C is the one no amount of reading the report against itself can catch: the
document stays internally consistent and only the replayed ENGINE disagrees.
Mutation D is caught by a pinned game rule and by nothing else in the chain that
shares a line of code with the arena.

Sample refusals, verbatim:

```
FAIL 1c `counts wins_a 7` against 0 rebuilt from the `game` lines
FAIL 1a game 0 turn 6: the report attributes `4,-3/4,1` to `r2`, and `r2` (configs/instrument_v0.toml) answers `4,-4/4,1`
FAIL 1b game 0: 36 turns were played, so the last turn was p2's, and the report records `result p1_win`
```

**Each link is load-bearing and none is redundant**: B is invisible to 1a and 1b, C
is invisible to 1b, and 1c alone would leave C's internal consistency unchallenged
were the labels swapped consistently everywhere.

### 8.6 What the dry run is and is not

Per CLAUDE.md the dry run is NOT a governed sample and does not consume this
pre-registration's first run. **And it constrains only the dry run's input.** A
reviewer may run anything, including the registered workload; nothing here limits
that.

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
seats. Rebuild means re-record. The `binary` key names a path WITH a separator,
which is D-294's qualification of D-283 and not a style choice.

**9.3 `openings_take`.** With the pair floor at 100 (§2) and the game cap at
`2 × openings_take`, this decides both the sensitivity and the cost. **BOUND:
`openings_take >= 100`.** Below it §2's floor is unreachable, so the H1 action
could never fire however the run came out — a design that cannot produce its own
positive outcome. D-292 records the 2000-opening shape at about 82 minutes wall at
4 workers, and §7 records why that anchor understates this matchup by about 3.3×.

**9.4 The soundness gate's state.** §8 of the design must be GREEN at the revision
this run measures, and the operator confirms it rather than this document assuming
it — because a green SPRT over an unsound generator measures nothing (§1).

**9.5 The calibration probe.** The command, its positions, its budget and its
revision, recorded before it is run — it decides §9.3 and therefore the whole cost
and sensitivity, and revision 1 billed it in §7 while registering it nowhere. It is
NOT part of the sample, as WP-1.3's probe was not. **IT MUST MEASURE THE STAGED
SEAT'S WORST SINGLE SEARCH**, not only a mean per-game cost: `hang_timeout_ms =
120000` is currently justified from a radius-2 measurement (a worst single search
of 5.1 s over the 24-position bench), and the staged seat's per-search cost is
unmeasured. A watchdog sized against the wrong seat ends runs it should not end,
and a run it ends is not a measurement.

**9.6 The run's revision.** The commit the games are played at, recorded before the
first game, so the report's `experiment_sha256` has something to be compared with.

**9.7 The baseline snapshot's revision.** The commit at which
`tools/baseline_snapshot.sh` accepts `--config` (§7A.2). At this document's
revision it does not, and the script cannot measure the staged seat at all. The
slot is a REVISION, not a path: CLAUDE.md names an instrument with its revision, and
a change to the script reopens this document's review.

---

## 10. What flips this document

An amendment to any section reopens its review, however small the diff — which is
CLAUDE.md's rule and is why the OPERATOR-CONFIRM slots are slots rather than
provisional values this session guessed and a later edit would quietly change. It
binds the INSTRUMENTS too: `tools/wp15b_attribution_check.py` and
`configs/arena_wp15b_dryrun.toml` are named here with their revisions, and editing
either reopens this review as an amendment would.

The claim itself flips on the run. The DOCUMENT flips if the design's §8 soundness
gate is not green at the run's revision (§9.4), if the committed radius policy
moves before the run (which would change what engine B is), if the arena's verdict
vocabulary changes, or if `tools/baseline_snapshot.sh` lands `--config` in a shape
the §7A.2 criterion cannot be taken under.

---

## 11. REVIEW STATE

**This document has never passed a review, and it does not claim to.** Two cycles
have run and both FAILED. Revision 3 applies every finding from the second and is
**queued for a fresh-context review at THIS revision**, which the architect
dispatches; CLAUDE.md's rule is that the revision governing a run must pass its own
review, and reviews of superseded revisions do not transfer.

| Cycle | Revision | Verdict |
|---|---|---|
| 1 | rev 1, `af9fa4a` | **FAILS** — 4 BLOCKING, 7 MAJOR |
| 2 | rev 2, `9c068a0` | **FAILS** — 7 BLOCKING, 8 MAJOR, 2 MINOR, 9 REJECTED with reproducers |
| 3 | rev 3, this revision | **OUTSTANDING** — not self-reviewed, not run |

**Nothing here has been run as a governed sample and this document governs
nothing.** The dry run of §8 is not a governed sample by CLAUDE.md's own text.

### 11.1 What survived both rounds, so a reviewer knows what not to redo

Worker-invariance of the verdict block (MEASURED byte-identical at 1 and 4
workers). Criterion 2 on the `engine_id` handshake — it survives, it simply does
not cover the score path, which is now Criterion 1's chain. All nine report-line
spellings §4 regenerated from source, and the `Verdict::token` list including that
`inconclusive` is not a token. §6's corrected `gate_v0` quote and the 1.3 s / 554 s
figures. §2's D-190 precedent and its floor arithmetic — the reviewer brute-forced
the minimum crossings independently and got **10** pairs with no capped games and
**6** with, both exact. §3's D-157 capped-game treatment (the acceleration half;
the exclusion half is corrected in §3). §7's cost anchor (16.6 core-seconds per
game reproduced against the document's 16.3; this session measures 16.2 on its own
run). The pre-game refusal branch (exit 2, named error, no report file). And the
rejection of the seat-swap criterion, which is right and survives the rewrite.

The second reviewer's own summary is the fairest description of where revision 2
stood: **"The document's engineering is much better than revision 1's. It fails on
where it aimed, not on care."**

---

## 12. THE FIX MAP

Every finding of the second review against what carries it. The order is the
operator's: the dry-run criteria first, then the rest.

| # | Finding | Where it lands | Evidence |
|---|---|---|---|
| — | **Repair the dry-run criteria first: the current ones pass on a verdict inversion** | §8.3 Criterion 1, rebuilt as a three-link chain; §8.5 | Mutation B, C and D each run through a separate worktree; the chain fails all three and passes the honest run (§8.5 table) |
| 1 | Rebuild Criterion 1 on the score path | §8.3 link 1c | Mutation B: 10 refusals, exit 1, against revision 2's pass |
| 2 | Re-state Criterion 1 so it passes on an HONEST run | §8.3 — the chain reads no depth field, so the `depth_a 2 depth_b 2` tie is irrelevant to it | Honest report: PASS, exit 0. Game 6's tie reproduced and recorded in §8.4 |
| 3 | Create `configs/arena_wp15b_dryrun.toml` and actually run §8 | The config is committed; §8.4 and §8.5 are its output | Config sha256 recorded; report, referents and all four instrument runs recorded |
| 4 | §5 row 3 must name the DIRECTION | §5, the degenerate row, rewritten to read the single occupied pentanomial slot `k` and to route `k <= 2` to the `h0` action | The reviewer's `pentanomial p0 4` reproduction is cited in the row |
| 5 | §5 needs a row for a BALANCED degenerate sample, and §7's "every branch is in §5" struck until true | §5's `k = 2` case, with `capped_fraction` required beside it; §7's claim re-made with the three added rows named | The reviewer's `turn_cap = 10` reproduction is cited in the row |
| 6 | §7A's second instrument cannot measure the staged seat at the revision it names | §7A.2 and §9.7: a REVISION slot for the `--config` landing, not `e889b5b` | MEASURED here: `--config` answers `FAIL: unknown argument`; the script hard-codes `CONFIG="configs/instrument_v0.toml"` |
| 7 | §7A's agreement criterion is invariant under the stage it names as under doubt | §7A split into TWO doubts: 7A.1 gets an instrument that DOES see the score path, 7A.2's criterion is rebuilt two-sided and tie-aware | 7A.2's three buckets and the registered 12-of-24 prediction, marked JUDGED with its D-190 ground |
| 8 | §7A's EXPENSIVE reclassification reverses D-245 without an ADR line | **D-307**, on the operator's ruling; §7A cites it and no longer performs it | D-307 keeps the second-instrument duty and lapses only the replication clause |
| 9 | §4 renders three separate records as one line, and never spells `game` or `pair` | §4, rewritten one record per line, in report order, with `game`, `moves`, `refusal` and `pair` added | The lines Criterion 1 reads are now the lines §4 spells |
| 10 | §8.3's referent names no position; the TIMES do not reproduce | §7 and §8.2/§8.4 name bench position 1 by `src`; §8.3 Criterion 3 excludes the times by name | Three machines on one quantity: 396/240, 474/285, 388/235 ms. Depth and nodes reproduce exactly |
| 11 | §3's capped-game sentence is wrong in detail | §3, corrected against `score.rs` | `tally`'s `decided`/`decided_clean`/`wins_a`/`losses_a` all sit in the non-capped branch; the D-157 acceleration half is unchanged |
| 12 | §9.3 does not bind `openings_take >= 100`; §9.5's probe need not measure worst-case per-search; §3 cites D-283 without D-294 | §9.3 carries the bound; §9.5 carries the worst-search requirement with the watchdog reasoning; §3 and §9.2 cite D-294 | — |
