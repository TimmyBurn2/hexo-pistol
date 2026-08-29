# WP-1.5d (B) PRE-REGISTRATION REVISION 3 — SCOPED RE-REVIEW

## Header

| | |
|---|---|
| artefact | `docs/experiments/wp15d_b_prereg.md` **revision 3** |
| named revision | `a62ffca6c6650d55c1ce6dcda9f6388582e0f6ff` |
| matches HEAD? | **YES.** `git rev-parse HEAD` → `a62ffca6c6650d55c1ce6dcda9f6388582e0f6ff`, branch `dev`. `git status --porcelain` → `M docs/decisions.md` only (the launching session's D-485/D-486 append), plus gitignored `artifacts/`. Neither touches the artefact |
| prior revisions | `0dcd0db` (rev 1) FAILED — 3 BLOCKING, 7 MAJOR, 4 MINOR (`wp15d_b_prereg_REVIEW.md`); `b75c1f6` (rev 2) FAILED — 1 BLOCKING, 3 MAJOR (`wp15d_b_prereg_REVIEW_rev2.md`) |
| reviewer freshness | fresh context. Did not author revision 1, 2 or 3; is not the reviewer of revision 1 or revision 2 |
| worktree used | `/home/tom/Projects/pistol-wt-rev3`, detached at `a62ffca`, on `/home`. **Removed at the end of this report**; `git worktree list` receipt at the foot |
| digests verified | `artifacts/wp15d_b_measurement_v1.txt` = `46aaf3fbafbc93bb4fca6816c023e6611a21a1fe739871f4b3ad945f78eefe3e` ✓ (matches the dispatch); `wp15d_b_dryrun_v1.txt` = `c318aa22…`; `wp15d_b_criterion_falsification_v1.txt` = `213ecbef…`; `wp15d_b_rev1_reviewer_probe_v1.txt` = `b192e779…` ✓ (matches `wp15d_b_worktree_export_receipt_v7.txt`); `wp15d_b_rev1_reviewer_probe_instrument_v1.rs` = `0c97df93…` ✓; engine `target/release/pistol` = `8dc2f922…` (the digest the dry-run artifact records) |

**WHAT I READ.** `CLAUDE.md` and `docs/process.md` in full; the prereg revision 3 in
full; **both** failed review reports in full; `docs/experiments/wp17_sprt_prereg.md`
and `wp16_sprt_prereg.md`; ADR lines D-190 (via citation), D-215/D-362 (via
citation), D-374, D-376, D-377, D-388, D-395, D-396, D-401, D-423, D-424, D-427,
D-465, D-469, D-472, D-478, D-479, D-482, D-483, D-484, D-485, D-486;
`crates/pistol-search/tests/wp15d_b_measurement.rs`;
`tools/wp16_warm_attribution_check.py` and `tools/wp15b_attribution_check.py` in
full at every function the criterion touches; `crates/pistol-arena/src/{config,score,
sprt,openings}.rs` and `src/bin/arena.rs`; the arena's own governed-run artifacts for
WP-1.5b, WP-1.6 and WP-1.7.

**WHAT I RAN.** Recomputation of every calibration, bench, spread and sensitivity
figure from the registered artifact; byte-level `diff` of all four quoted blocks
against their artifact lines; **an independent, self-consistent REFEREE INVERSION run
end to end through BOTH attribution checkers** (the construction the rev-2 review
built and this session recorded as un-reproduced); **digest reconstruction of all
three candidate mutations** behind the criterion-falsification receipt's row B;
**§7.7's registered timeout probe, executed as written under both readings of its
sample, replicated three times**; eight real WP-1.7 turn-39 positions timed on the
armed seat; a structural key-by-key validation of §4C's TOML block against the arena
schema; and the D-479/D-483 number sweep over every numeral in the document.

**EVIDENCE EXPORTED TO THE MAIN TREE** (D-469), before the worktree was removed:

```
978bb06c7a8ca9458c64888f7feef041a958d498bad6e18f43c0a370975e3333  artifacts/wp15d_b_rev3_criterion_reproduction_v1.txt
b59f7b495dd681dee7e6bf1541ff68f4c4cdd122c46c545247c34f1d910f9151  artifacts/wp15d_b_rev3_mutation_instrument_v1.py
450f52be5791f92f3c8c151131bb2fbe61ae70ad6b11b72b6a11625fac07b752  artifacts/wp15d_b_rev3_timeout_probe_v1.txt
7d1e3be1bfb4892519a9aa176e8d78fdbaae0aeafcebafd53bea68c9e222c3b6  artifacts/wp15d_b_rev3_number_sweep_v1.txt
```

---

# VERDICT: **FAIL**

**0 BLOCKING, 6 MAJOR, 7 MINOR.**

**One finding is SUBSTANTIVE (NEW 5). Twelve are MECHANICAL.** Per the dispatch, a
single substantive finding is a hard stop and no revision 4 is authored.

| # | severity | class | one line |
|---|---|---|---|
| NEW 1 | MAJOR | MECHANICAL | §7.2's `(100, 120]` comes from the rev-1 REVIEWER's own probe — a different instrument, grid and population — in a document whose §1 forbids any figure from anywhere else |
| NEW 2 | MAJOR | MECHANICAL | rev-2's NEW 3 closed in **one of its three** enumerated parts: `invalid_forfeit` and arena exit 1 are still unrouted, and no import-by-reference row was added |
| NEW 3 | MAJOR | MECHANICAL | §5's attribution row still reads "the two instruments DISAGREE" — revision 2's criterion. On the corrupted report they **agree** and the run is still not a measurement |
| NEW 4 | MAJOR | MECHANICAL | §4B's dry-run bullet still evaluates the dry run against revision 2's **superseded counts criterion** ("(i) 459 = 459 … the agreement criterion HOLDS") |
| **NEW 5** | **MAJOR** | **SUBSTANTIVE** | the criterion-falsification receipt's row B — restated as §4B's own table row — attributes its captured output to an input that provably did not produce it |
| NEW 6 | MAJOR | MECHANICAL | §7.7's timeout probe names its sample two incompatible ways; the literal reading selects openings inside WP-1.7's **consumed** slice |
| NEW 7 | MINOR | MECHANICAL | §4's `hang_timeout_ms` row still says the margin is confirmed against "§3's own worst single search", which §7.7 explicitly withdraws |
| NEW 8 | MINOR | MECHANICAL | §4B's second-instrument discharge points at a table row that records only ONE instrument's reaction |
| NEW 9 | MINOR | MECHANICAL | the cost paragraph's "1.5–3 hours" is 3.5–7× what its own cited source records, and "a second-instrument pass of a few seconds" is ~12 minutes |
| NEW 10 | MINOR | MECHANICAL | the dry-run block names no artifact; the falsification receipt is cited without a digest |
| NEW 11 | MINOR | MECHANICAL | "`bench_positions_v1` is 15- and 35-stone positions" — `p16` is 31 |
| NEW 12 | MINOR | MECHANICAL | `wp15b_attribution_check.py:286-290` does not contain the expression it is cited for (it is at `:284`) |
| NEW 13 | MINOR | MECHANICAL | the timeout probe registers neither the verbatim command nor replication, both of which D-376 — the precedent it cites — registered |

**Two attacks I could not reproduce are recorded as REJECTED**, with the reproducers
attempted, at the foot of the findings section.

**What is right about this revision, said first because it is the larger part.** The
agreement criterion **works**. I rebuilt the self-consistent referee inversion the
rev-2 review used, from scratch, and ran both checkers end to end: the criterion's
term (ii) moves from 0 to 459 findings on **both** instruments while the count lines
stay byte-identical. That is the BLOCKING the round existed to close, and it is
closed on its merits, verified by a reviewer who did not take the prior report's word
for it. NEW 4's remedy is likewise taken at D-376's level of specificity in every
field that determines the gate's outcome, and every calibration, bench, spread and
sensitivity figure in the document reproduces from the registered artifact.

---

# 1. CLOSURE LEDGER — part (a)

| # | rev-2 finding | status |
|---|---|---|
| **NEW 1** (BLOCKING) | the agreement criterion is invariant under the defect class it names | **CLOSED — verified by independent reproduction.** Residue → NEW 4, NEW 8 |
| **NEW 2** (MAJOR) | §7.3's "0.6 %" reproduces from nothing | **CLOSED** |
| **NEW 3** (MAJOR) | §5 routes 3 of 5 verdict tokens; `inconclusive_degenerate`, `invalid_forfeit`, arena exit 1 | **PARTIAL — one of three parts** → NEW 2 |
| **NEW 4** (MAJOR) | the slot-pass probe's input sample is unspecified | **CLOSED on the load-bearing part** (the sample is fixed, so the outcome is no longer choosable). Residue → NEW 6, NEW 13 |

## NEW 1 — CLOSED, and I did not take this on trust

**Revision 3's closing text, verbatim (`docs/experiments/wp15d_b_prereg.md:307-330`):**

> **THE AGREEMENT CRITERION.** The governed report is a measurement only if all
> three hold: **(i)** the warm pass exits `ATTRIBUTABLE (0)` — not `NO_ANSWER`,
> which is how it refuses a report it cannot vouch for; **(ii) BOTH instruments
> report ZERO `1b` move-list mismatches and ZERO `1c` rebuild mismatches** — the
> per-game FINDINGS, never the counts, which the review proved invariant; and
> **(iii)** neither instrument refuses to read the documents at all.
> …
> **THE REGISTERED CONSEQUENCE**, fixed before either runs: **any of (i), (ii) or
> (iii) failing makes the run NOT A MEASUREMENT.**

This is the FINAL rev-2 report's own remedy, word for word in substance
(`wp15d_b_prereg_REVIEW_rev2.md:412-421`), including the exclusion of `1a` that the
final report added and its draft did not have. See part (c).

**I reproduced the falsification myself.** I rebuilt the self-consistent referee
inversion from `artifacts/wp17_governed_run_v1.txt`: every decided game's `result`
flipped, `counts wins_a/losses_a` swapped, every `pair` bucket mirrored `b → 4−b`,
the pentanomial mirrored, and `llr_pair`/`verdict` recomputed with the checker's own
ported `recompute_verdict`; the replay document rebound to the new digest.

```
$ python3 artifacts/wp15d_b_rev3_mutation_instrument_v1.py
mutated pentanomial [11, 42, 214, 53, 21] verdict inconclusive_at_game_cap llr 1.719877307166857 wins_a 245 losses_a 214
mutated report sha256 39dd4f86322d05103f511621a601c5218341329f41a4394b37980cc28d25a6fc
```

```
$ python3 tools/wp16_warm_attribution_check.py mutated_report.txt mutated_replay.txt target/release/pistol
warm_attribution_check: 1b: 459 decided non-forfeit game(s) adjudicated against the move list
warm_attribution_check: 1c: 682 game(s) and 341 pair(s) rebuilt off the score_a path
warm_attribution_check: FAIL 1b game 2: 23 turns were played, so the last turn was p1's, and the report records `result p2_win`
  … 459 FAIL 1b lines …
warm_attribution_check: FAIL — 459 failure(s)
EXIT=1     FAIL-1b lines: 459

$ python3 tools/wp15b_attribution_check.py mutated_report.txt target/release/pistol
attribution_check: 1a: 1364 turns replayed, 78 of them discriminating, 78 of 682 games directly attributed by replay
attribution_check: 1b: 459 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 682 games and 341 pairs rebuilt off the score_a path
attribution_check: FAIL — 459 failure(s)
EXIT=1     FAIL-1b lines: 459     FAIL-1a lines: 0
```

The counts are byte-identical to the honest run (`1b: 459`, `1c: 682/341`), which
independently reproduces the rev-2 review's attack on revision 2's criterion; both
instruments emit 459 per-game **findings**, so revision 3's term (ii) **fires**; and
the cold `1a` line is unchanged at `1364/78/78`, which is exactly the reason revision
3 gives for excluding `1a`. **Every clause of §4B's closing table row at `:340` is
confirmed by my own run, not by the prior report's.** Receipt:
`artifacts/wp15d_b_rev3_criterion_reproduction_v1.txt`.

*Residue, carried below rather than folded in here:* the criterion's evaluation of
its own dry run was never updated (NEW 4), and the sentence that discharges
`docs/process.md`'s second-instrument test points at a row that does not show what it
says (NEW 8).

## NEW 2 — CLOSED

**Revision 3's closing text, verbatim (`:569-572`):**

> The two degenerate positions contribute
> **0.00 %** of the incumbent's Σ — both read `median_ms=0` in the artifact — so
> they cannot move a ratio at all. **Revision 2 said 0.6 % here, sourced from
> nothing**, in the very sentence correcting a provenance defect; it is the sixth
> instance in this work package and it is recorded rather than quietly replaced.

Recomputed independently from the registered artifact: `p13` and `p16` read
`median_ms=0` with all five reps `0` at all seven seats, contributing `0` ms of the
incumbent's `Σ = 4800` ms — 0.00 %. The finding is closed **and** recorded rather
than silently swapped, which is the stronger of the two remedies the review offered.

The retained "0.6 %" is a **quotation of an erroneous prior claim, explicitly
labelled unsourced**, not a figure the document consumes. That is not a provenance
defect and I do not raise it as one.

## NEW 3 — PARTIAL: one of three enumerated parts

The review enumerated three unrouted terminal states and one closing device:

> **Fix:** add §5 rows for `inconclusive_degenerate` **and** `invalid_forfeit`, plus
> one import-by-reference row for the remaining arena exits …
> Also still uncovered … **arena exit 1** (`RUN_FAILED`, `src/bin/arena.rs:61`) —
> §5 routes only exit 2 (`REFUSED`, `:63`).

**Revision 3 added one row (`:471`):**

> | `inconclusive_degenerate` | **the arena's fourth token**, which revision 2 did
> not route: the pentanomial is degenerate, so no verdict is available at any n.
> Reported as such, no config moves, and the run is NOT re-drawn on a fresh slice
> inside this WP — the book has none left (§4) |

**REPRODUCED — nothing else was added:**

```
$ /usr/bin/grep -n 'invalid_forfeit\|RUN_FAILED\|exit 1\|imported by reference' docs/experiments/wp15d_b_prereg.md
290:What each may be concluded to mean is **WP-1.6 §5's table, imported by reference
```

`invalid_forfeit` does not occur in the document at all; `RUN_FAILED` and "exit 1" do
not occur; the only import-by-reference is in §4B and is scoped to the warm
checker's **exit taxonomy**, not to the arena's verdict tokens. See NEW 2 in the
findings section, and the totality enumeration in part (d).

## NEW 4 — CLOSED on the part that carried the finding

**Revision 3's closing text, verbatim (`:615-625`):**

> **THE TIMEOUT PROBE.** Before launch, the ARMED seat
> (`configs/instrument_staged_snk_v0.toml`) is run at the registered budget
> `go nodes 50000` over **the first 50 openings of the governed slice —
> `random_openings_v1.txt` lines `1500..1549`, in file order, none skipped** —
> through the shipped `target/release/pistol`, reading the `time` field off each
> `info totals` line. **The statistic is the MAXIMUM of those 50.** If
> `hang_timeout_ms / max < 24`, the launch STOPS …

The finding as the reviewer stated it was that "whether it stops is choosable at
launch". That is closed: the sample is now fixed in the document, and the statistic
(MAXIMUM), the config, the budget, the binary and the field read are all registered.
Of D-376's five fields, three of the four that were missing are supplied.

Two residues, raised below: the sample is named two incompatible ways (NEW 6 —
reproduced, and the two readings select disjoint samples), and the **verbatim
command** and the **replication procedure** — the remaining two D-376 fields the
review's own table enumerated — are still unregistered (NEW 13).

---

# 2. PART (b) — THE D-479 / D-483 NUMBER SWEEP

I re-derived every figure from the registered artifact rather than accepting any
prior review's tick. Receipt: `artifacts/wp15d_b_rev3_number_sweep_v1.txt`.

**Provenance classes used below:** **(1)** measured, names
`artifacts/wp15d_b_measurement_v1.txt` and its seat, reproduced by me; **(1′)**
measured, from a *different* registered artifact the document names; **(1″)**
measured, from an artifact the document does **not** name; **(2)** a registered
decision (bracket, threshold, direction, parameter, count) the document fixes;
**(3)** a source-code / git / ADR referent I verified; **(✗)** unsourceable.

| line | figure | class | verified |
|---|---|---|---|
| 74 | grid `{4,8,16,32,64,128}`, seat `K=0` | 2 | matches the artifact's own `# Grid` header |
| 86, 123 | `0.75 ×` best gain | 2 | registered rule; `0.75 × 0.3640 = 0.2730` exactly |
| 92, 95 | openings `0..999`; slice `1500..1999` | 2 | `CAL/POPULATION openings=1000 skip=0 take=1000` ✓ |
| 103 | D-395's "19 of the 24 … on BOTH sides" | 3 | verbatim in `docs/decisions.md:846` ✓ |
| 148, 152, 154 | bracket `≤ 1.10`, band `(1.10, 1.25]`, abort `> 1.25` | 2 | registered |
| 159 | IQR `≤ 10 %` of own median | 2 | the `tools/bench_delta.sh` convention (D-215/D-362) |
| 166 | `7 × 1000 + 7 × 24 × 5 + 7 × 4 × 5` searches | 2 | arithmetic ✓ = 7980. **Incomplete**: omits the SENS pass's 595+672 = 1267 searches. Rev-2 called this immaterial; I concur, and note it here rather than raise it |
| 167 | **MEASURED 2662.25 s** | 1 | artifact header line 11 ✓ |
| 168-169 | **ESTIMATED 1.5–3 hours**, "from WP-1.7's own comparable run" | ✗ | **NEW 9.** WP-1.7's artifact records `wall_ms 1049514` = 17.5 min for 682 games; scaled to 1000 games ≈ 26 min |
| 169 | "one … pass of a few seconds each" | ✗ | **NEW 9.** Warm: < 1 s ✓. Cold: **~12 min** measured (it replays 1364 turns through both engines) |
| 206 | slices `0..499` / `500..999` / `1000..1499` | 3 | `openings_skip` 0 / 500 / 1000 in the three governed-run artifacts ✓ |
| 206 | "the last unconsumed slice of the 2000-opening book" | 3 | 2000 `start moves` lines ✓; `1500+500 = 2000` exactly ✓ |
| 207-211 | `openings_take 500`, `nodes 50000`, `turn_cap 40`, `n_workers 4`, `hang_timeout_ms 120000` | 2 | registered; all 21 mandatory schema keys present in §4C, none unknown |
| 224-229 | `elo0 0.0`, `elo1 15.0`, `alpha/beta 0.05`, cap 1000 games / 500 pairs, floor 100 pairs | 2 / 3 | `elo1 15` matches WP-1.6's and WP-1.7's own `sprt` lines ✓; the 100-pair floor is D-190's convention at `wp16_sprt_prereg.md:297`, `wp17_sprt_prereg.md:80` ✓ |
| 250-255 | warm replay at `a14912a`; `bfdf933..HEAD` = **181 deletions, 0 insertions** | 3 | re-derived **at `a62ffca`**: `30+57+30+33+31 = 181`, every insertion column `0` ✓ |
| 260 | "three lines in a TEST STUB engine (`FirstLegal`)" | 3 | `stub_engine.rs` is `3 25`; every other crate file is `0 n` ✓ |
| 259, 261 | `6c929da`, `a80a864` | 3 | `git log -1 --` on each file ✓ |
| 289 | exit taxonomy at `:133-136` | 3 | exact ✓ |
| 302 | count-before-adjudication at `:835-841` | 3 | exact ✓ — `judged += 1` is `:838`, the mismatch branch `:840-841` |
| 337-341 | the falsification table | 1′ / ✗ | rows A, C, E reproduce from the named receipt; **row B's input does not match the digest its own output prints (NEW 5)**; row D's numbers are the cold `1a` at `:286-290`, and the expression cited is at `:284` (NEW 12) |
| 340 | `1b: 459`, `1c: 682/341`, "both emit 459 `FAIL 1b` lines" | ✗ (as cited) | attributed to the rev-2 review's own probe, which has **no exported artifact** (`wp15d_b_worktree_export_receipt_v7.txt` records that its inputs "are not anywhere on disk"). **I re-ran it and every figure holds** — see NEW 1 above and `wp15d_b_rev3_criterion_reproduction_v1.txt` |
| 379-388 | the dry-run output block | 1″ | byte-identical to `artifacts/wp15d_b_dryrun_v1.txt:21-25` and `:29-31`, and I re-ran the warm pass and got the block byte-identical again. **The document names no artifact for it (NEW 10)** |
| 390-392 | "459 = 459 … 682 / 341" | 1″ | the numbers reproduce; **the sentence applies revision 2's criterion (NEW 4)** |
| 442 | `safety_net_top_k = 16` | 2 | `diff` of the two configs' non-comment lines → exactly `11c11 < 0 / > 16` ✓ |
| 467 | 100-pair floor row | 2 | consistent with §4A |
| 471 | "the arena's **fourth** token" | ✗ | the arena has **five** (`sprt.rs:189-193`). Folded into NEW 2 |
| 491-492 | both `SENS/TRAJECTORY` lines | 1 | `diff` against artifact lines 242-243 → **IDENTICAL** ✓ |
| 496 | 595 searches, 125 bearing (21.0 %), 29 diverged (4.87 %) | 1 | `125/595 = 21.0 %`, `29/595 = 4.874 %` ✓ |
| 498 | 672, 135 (20.1 %), 39 (5.80 %) | 1 | `135/672 = 20.1 %`, `39/672 = 5.804 %` ✓ |
| 503-505 | 4.9–5.8 %; `29/125 = 23.2 %`, `39/135 = 28.9 %` | 1 | all four ✓ |
| 506 | "500 paired openings" | 2 | `openings_take` ✓ |
| 524-525 | sha256 `46aaf3fb…`, `70cb580`, `4ec470f`, 2662.25 s, exit 0 | 1 / 3 | digest recomputed ✓; artifact header ✓ |
| 532 | the whole `CAL/SELECTED` line | 1 | `diff` against artifact line 42 → **IDENTICAL** ✓ |
| 536 | K16 `+0.2990` clears `+0.2730`; K32 `+0.2350` does not | 1 | recomputed from the seven `CAL/SEAT` lines ✓ |
| 540-543 | K=128 ≡ K=0 on mean, every histogram bucket, every counter | 1 | `2.0740 == 2.0740`, `[0,3,926,65,6,0,0,0]` identical, `capped_rows` both 0 ✓ |
| **545** | **"the pool maximum in (100, 120]"** | **✗** | **NEW 1.** From the rev-1 REVIEWER's own 150-opening probe on grid `{16,100,120,128}` and population `0..149`. Not in the registered artifact; the document names neither the probe nor its digest |
| 547-553 | `capped_rows` **818 937** at K=64, the grid maximum; `gain(64) = +0.0140`; `K_bind = 64`; threshold `+0.2730` | 1 | all ✓, and `K_bind` derived independently from `capped_rows > 0` |
| 557-559 | Σ **4800** / **4807** ms, ratio **1.0015**; IQR **0 of 168** | 1 | `4807/4800 = 1.001458` ✓; 0 violations over all 168 cells ✓ |
| 562-564 | `nodes=50176` false in **14 of the 168** cells; `p13 = 151`, `p16 = 3` | 1 | exactly 14 cells (`7 × p13`, `7 × p16`) ✓ |
| 569 | degenerate positions contribute **0.00 %** | 1 | `0` of `4800` ✓ |
| 570 | "Revision 2 said 0.6 %" | — | a labelled quotation of an unsourced prior claim, not a consumed figure. Not a defect |
| 579-582 | the twelve spread cells | 1 | every cell ✓ (`0/95, 0/71, 0/152, 0/0`; `133→133, 449→470, 2761→2748, 10596→10573`) |
| 585 | "every seat reads `depth=1` at every position" | 1 | all 28 `BENCH/SPREAD` lines ✓ |
| 599-600 | **10 596 ms**, **11.3×** | 1 | worst median across both bench sections ✓; `120000/10596 = 11.33` ✓ |
| **605** | "`bench_positions_v1` is 15- and 35-stone positions" | ✗ | **NEW 11.** 12 positions at 15 stones, 11 at 35, and `p16` at **31** |
| 606 | **491 ms**, **244×** | 1 | worst corpus median is `p23` at K=16 = 491 ✓; `120000/491 = 244.4` ✓ |
| 617-621 | `go nodes 50000`, 50 openings, `1500..1549`, MAX, `24×` | 2 | registered — but the sample is named two incompatible ways (**NEW 6**) |
| 650 | "~52 minutes" before the run began | 3 | `70cb580` 19:47:30, `4ec470f` 19:48:50; artifact written 21:24:15 minus 2662.25 s ⇒ run began 20:39:53 ⇒ 52.4 min / 51.1 min ✓ |

**Numbers I could not source: three.** `(100, 120]` at `:545` (NEW 1); "1.5–3 hours"
and "a few seconds each" at `:168-169` (NEW 9); "15- and 35-stone" at `:605`
(NEW 11). Plus one figure whose *cited* provenance is a reviewer's unexported probe
(`:340`), which I discharged by re-running it myself, and one token count that is
wrong (`:471`, folded into NEW 2).

---

# 3. PART (c) — DRAFT-REPORT CONTAMINATION

**The diff I read** (`git diff 84235f8..677bc42 -- docs/experiments/wp15d_b_prereg_REVIEW_rev2.md`):
three hunks. One reworded a sentence in "WHAT I RAN". One appended the cold
checker's end-to-end output, the byte-identical comparison table, the
`FAIL 1b`-count observation and the "general form is stronger" paragraph to NEW 1's
REPRODUCED section. The third **replaced NEW 1's "What would close it" entirely**:

| the DRAFT (`84235f8`) said | the FINAL (`677bc42`) says |
|---|---|
| "State the criterion over … the warm pass's `W coverage` / `W classification` lines **against the cold checker's `1a` line** … Agreement on a recomputed verdict token would also be defect-sensitive" | "state the criterion over the two checkers' **1b and 1c FINDINGS rather than their counts** … **my mutation left even the cold checker's `1a` line unchanged** … So `1a` is **not the discriminator** either for this defect class" |

**Revision 3 answers the FINAL, and says so.** §4B `:307-319` states the criterion
over the 1b/1c findings and adds, in the criterion box itself:

> **`1a` is NOT a term, and the reason is the review's** (rev-2 review's own
> corrected finding): a wrong winner does not change which engine moved, so `1a`
> — which asks each labelled engine what it would have played — is blind to a
> referee inversion.

That is the final's correction, not the draft's recommendation. **No place in
revision 3 quotes or answers the draft.** I checked every `1a` occurrence in the
prereg (`:310, 315, 337, 341`) and all four treat `1a` as context, never as a term.

**But two artefacts around the document are still draft-era, and one of them is
cited as revision 3's receipt.**

1. `artifacts/wp15d_b_criterion_falsification_v1.txt` carries `Head: 84235f8` — the
   draft commit — and its row D says the seat mislabel "lands in its 1a failure
   list, **which the criterion reads**". Under revision 3's criterion `1a` is not
   read. The prereg's own table row at `:341` corrects this to "reported as
   CONTEXT, not a term", so the **document** is right and the **receipt it names**
   is not. I record this rather than raise it as a separate finding, because the
   document's own text carries the correct claim.
2. The **rev-2 report's own "WHAT WOULD FLIP THIS TO PASS" list was not updated**
   with its NEW 1 section: `wp15d_b_prereg_REVIEW_rev2.md:757-762` still says to use
   "the warm pass's `W coverage` / `W classification` against the cold checker's `1a`
   line", which the same report's NEW 1 then rules out two hundred lines earlier.
   Revision 3 followed the corrected section, which is the right choice, and this is
   a defect of the rev-2 report rather than of the artefact under review. Noted so
   the architect does not read the flip-list as an unmet requirement.

**Contamination verdict: NONE in revision 3's own text.**

---

# 4. PART (d) — VERDICT-SPACE TOTALITY, AND THE CRITERION AS WRITTEN

## 4.1 Totality: every terminal state this run can reach

**Arena verdict tokens — five** (`crates/pistol-arena/src/sprt.rs:189-193`, and
`score.rs:169-175` for when each is returned):

| token | reachable because | routed? |
|---|---|---|
| `h0` | LLR crosses the lower bound | ✓ §5 |
| `h1` | LLR crosses the upper bound | ✓ §5 |
| `inconclusive_at_game_cap` | no crossing at 500 pairs | ✓ §5 |
| `inconclusive_degenerate` | `sample.is_degenerate()` — every pair in one bucket. WP-1.7 recorded 223 of 682 games capped; an all-capped sample lands every pair in bucket 2 | ✓ §5 (added this revision) |
| **`invalid_forfeit`** | `records.iter().any(GameRecord::is_forfeit)` — returned **before** every other test, so it pre-empts them all | **✗ UNROUTED** |

**Arena exit codes — three** (`src/bin/arena.rs:61-63`, whose own header reads
"0 completed cleanly, 1 abandoned or forfeited (report still written), 2 refused"):

| code | routed? |
|---|---|
| 0 | ✓ implicitly — the normal path into §5's verdict rows |
| **1 `RUN_FAILED`** | **✗ UNROUTED.** Reachable from four sites (`:194, :207, :270, :279`), and `hang_timeout_ms` exists precisely to produce one |
| 2 `REFUSED` | ✓ §5 "arena exit 2 — the run is VOID" |

**`arena_report_aborted`** — the report form a run abandoned mid-flight writes — is
**✗ UNROUTED**. `wp16_sprt_prereg.md` §5 has a row for it; `wp17_sprt_prereg.md:136`
imports it. Revision 3 does neither.

**Warm-checker exit codes — four** (`ATTRIBUTABLE 0`, `NOT_A_MEASUREMENT 1`,
`NO_ANSWER 2`, `DETERMINISM_VIOLATION 3`): §4B `:290-292` imports WP-1.6 §5's table
by reference for "what each may be concluded to mean", and WP-1.6 §5 is a decision
table (Outcome | Action) that routes 0, 1, 2, 3 **and** "a termination that is none
of 0, 1, 2 or 3". I accept all five as routed by that import. It is worth saying
that the import sits in §4B and not in §5, so §5's table is not by itself the total
map its heading implies.

**Cold-checker exit codes — three** (0; 1 on any failure; 2 via `die`): routed by the
criterion's own terms — (ii) covers a 1b/1c finding, (iii) covers a refusal, and
`:321-325` registers that a clause-(b)-only failure is expected context. ✓

**Registered gates**: calibration-selects-no-K ✓; corpus bracket ✓; the
`(1.10, 1.25]` band ✓; the corpus abort ✓; the IQR gate ✓; the 100-pair floor ✓; the
§7.7 timeout probe — routed **in place** at `:621-623` ("the launch STOPS and the
margin is reported to the architect") rather than in §5, which is acceptable since it
is a pre-game gate; its `≥ 24` boundary is well defined (the stop fires on `< 24`).

**And one routed state whose condition is now wrong**: §5 `:466` routes "the two
attribution instruments DISAGREE". Under revision 3's criterion the instruments
**agree** on a corrupted report — I measured both at 459 — and the run is still not a
measurement. See NEW 3.

**Totality verdict: three arena states unrouted, one routed under a stale
condition.**

## 4.2 The agreement criterion, term by term, under §4B's own defect class

§4B names the stage under doubt at `:283-285`: "**everything between the two engine
processes and the printed verdict — the arena's seat bookkeeping, pairing, referee
and scoring.**"

| term | can it fail? | evidence |
|---|---|---|
| **(i)** warm exits `ATTRIBUTABLE (0)` | **YES** | my referee inversion → exit **1**; the receipt's rows B and C → exit **2** |
| **(ii)** both instruments report zero `1b`/`1c` mismatches | **YES** | my referee inversion → **459** `FAIL 1b` findings on **both** instruments, while the counts stay `459` / `682`+`341` on both |
| **(iii)** neither instrument refuses to read | **YES** | rows B and C: `warm_attribution_check: CANNOT READ: …`, exit 2 |

**The registered consequence can fire**, and I fired it. The dispatch's own test —
"a criterion that CANNOT fire on corrupted data is not [fine]" — is passed.

**Two honest qualifications the architect should weigh.**

**(a) Term (ii)'s warm half is logically implied by term (i).** `link_1b` and
`link_1c` append to the same `failures` list that `main()` reads to choose between
`ATTRIBUTABLE` and `NOT_A_MEASUREMENT` (`wp16_warm_attribution_check.py:936`). So
any warm 1b/1c finding forces warm exit ≠ 0, and (i) fails first. Term (ii)'s only
independent content is the **cold** checker's findings — and I diffed the two
implementations myself:

```
$ diff <(sed -n '832,847p' tools/wp16_warm_attribution_check.py) <(sed -n '370,385p' tools/wp15b_attribution_check.py)
   … differs only in the enclosing def/comment, `report["games"]` vs a local, a hoisted
     `claimed`, and `game(s)` vs `games` in the note …
$ diff <(sed -n '850,888p' …) <(sed -n '387,431p' …)
   … differs only in error-handling wrappers and `text.split("\n")` vs `text.splitlines()` …
```

The algorithm is the same carried-over code. So the two instruments can disagree on
(ii) only by parsing the same file differently. That is not a vacuity — the
conjunction still fires, as I measured — but it is a fair statement of how much the
second instrument adds, and it is why NEW 8 below objects to the sentence that claims
more.

**(b) The one sub-class of the named defect for which no term is shown to fire is
SEAT MISLABEL.** §4B registers `1a` — by its own account the discriminator for that
defect — as "reported as context beside the criterion rather than folded into it"
(`:315-319`), and "the arena's **seat bookkeeping**" is the first item of the named
class. I did **not** build an internally consistent seat mislabel and I do not raise
this as a finding. My reading of the code is that the warm pass would catch it
anyway: `check_coverage` compares each replayed game's node counts against the
report's, so a report whose labels are swapped relative to the configs that produced
it would produce divergences, then confirmed inversions, then `failures`, then exit 1
— failing term (i). That is reasoning, not a measurement, and I mark it so.

---

# 5. NEW FINDINGS

## NEW 1 — MAJOR, **MECHANICAL**. §7.2's `(100, 120]` is a figure from the rev-1 REVIEWER's own probe — a different instrument, a different grid and a different population — inside a document whose §1 states in bold that no figure may come from anywhere else.

**Claim, verbatim (`docs/experiments/wp15d_b_prereg.md:543-545`):**

> where the cap never fires, the seat is the incumbent relabelled, and the artifact
> shows it literally — K = 128 and K = 0 agree on the mean, on every histogram bucket
> and on every counter. **The reviewer's partial re-run puts the pool maximum in
> `(100, 120]`, so the whole region above it is an inert shelf.**

**The rule it breaks, verbatim from the same document (`:64-68`):**

> **ONE RUN, ONE ARTIFACT.** The instrument emits the calibration sweep, both bench
> fixtures and the sensitivity receipt in a single pass into
> `artifacts/wp15d_b_measurement_v1.txt` … **No figure below may come from anywhere
> else**

**The attack.** The registered artifact's grid is `{4, 8, 16, 32, 64, 128}`. Nothing
in it can place a pool maximum in `(100, 120]`; the interval comes from the rev-1
prereg reviewer's own 150-opening probe on grid `{16, 100, 120, 128}` over openings
`0..149`. The document names neither that probe nor its digest, and D-483 requires
that "every number a prereg consumes is produced post-implementation by a registered
instrument and cited from that run's artifact by digest". The launching session's own
export receipt asks this review to rule on it
(`artifacts/wp15d_b_worktree_export_receipt_v7.txt:37-39`: "Whether the prereg may
cite it at all under its own one-run-one-artifact rule is a question for the rev-3
review"). **My ruling is that it may not**: §1's own registered rule, D-482 and D-483
all forbid it, and it survived revision 2's review only because the rev-2 number
sweep did not list it.

**REPRODUCED.**

```
$ /usr/bin/grep -c '^CAL/SEAT K=100 \|^CAL/SEAT K=120 ' artifacts/wp15d_b_measurement_v1.txt
0
$ /usr/bin/grep -n 'CAL/SEAT K=100\|CAL/SEAT K=120' artifacts/wp15d_b_rev1_reviewer_probe_v1.txt
60:CAL/SEAT K=100 mean_depth=2.0733 population=150 depth_hist=[…] capped_rows=1118 …
61:CAL/SEAT K=120 mean_depth=2.0733 population=150 depth_hist=[…] capped_rows=0 …
$ /usr/bin/grep -n 'CAL/POPULATION' artifacts/wp15d_b_rev1_reviewer_probe_v1.txt
56:CAL/POPULATION openings=150 skip=0 take=150
$ /usr/bin/grep -c 'wp15d_b_rev1_reviewer_probe' docs/experiments/wp15d_b_prereg.md
0
```

**Why MECHANICAL.** Nothing downstream depends on it. The selection rule evaluates
six grid points, all in the registered artifact; §2's own text says grid points above
`K_bind` "carry no information about decay" and merely "bound the pool size from
above" — and that bound *is* derivable from the registered artifact alone
(`capped_rows = 0` at K = 128). Deleting the clause is one edit, changes no
threshold, and changes nothing anyone may conclude. **It becomes SUBSTANTIVE only if
the architect wants to KEEP the figure**, since admitting a second instrument's
output would reopen §1's one-run-one-artifact registration and D-482.

## NEW 2 — MAJOR, **MECHANICAL**. Rev-2's NEW 3 was closed in one of the three parts it enumerated: `invalid_forfeit` and arena exit 1 are still unrouted, and §5 calls `inconclusive_degenerate` "the arena's fourth token" when the arena has five.

**Claim, verbatim (`:471`):**

> | `inconclusive_degenerate` | **the arena's fourth token**, which revision 2 did
> not route: …

**The attack.** `crates/pistol-arena/src/sprt.rs:189-193` emits five tokens, and
`score.rs:169-171` returns `InvalidForfeit` **before every other test**, so a single
forfeit pre-empts `h0`, `h1` and both inconclusives regardless of the sample. §4
registers `hang_timeout_ms` whose whole purpose is a liveness event. Arena exit 1
(`RUN_FAILED`) is likewise reachable from four sites and is what an abandoned or
forfeited run returns *with a report still written* — the case a launching session is
most likely to meet and least likely to have a rule for.

**REPRODUCED.**

```
$ /usr/bin/grep -n 'Verdict::[A-Z]' crates/pistol-arena/src/sprt.rs | sed -n '1,5p'
189:            Verdict::H0 => "h0",
190:            Verdict::H1 => "h1",
191:            Verdict::InconclusiveAtGameCap => "inconclusive_at_game_cap",
192:            Verdict::InconclusiveDegenerate => "inconclusive_degenerate",
193:            Verdict::InvalidForfeit => "invalid_forfeit",
$ /usr/bin/grep -c 'invalid_forfeit' docs/experiments/wp15d_b_prereg.md
0
$ /usr/bin/grep -c 'RUN_FAILED\|exit 1' docs/experiments/wp15d_b_prereg.md
0
```

**Why MECHANICAL.** Three rows, in `wp17_sprt_prereg.md:131-136`'s own style, one of
which is a single import-by-reference line. The review named the precedent and the
wording. No measurement, no semantics.

## NEW 3 — MAJOR, **MECHANICAL**. §5 routes the attribution instruments under revision 2's condition. On a corrupted report the two instruments AGREE — I measured it — and the run is still not a measurement, so §5's row cannot be reached by the failure the criterion actually registers.

**Claim, verbatim (`:466`):**

> | the two attribution instruments DISAGREE | **the run is not a measurement.** The
> verdict is not read, and the package returns to the architect with both reports
> (§4B) |

**Against §4B's own registered consequence (`:327-330`):**

> **any of (i), (ii) or (iii) failing makes the run NOT A MEASUREMENT.**

**The attack.** Revision 3 replaced an *agreement* criterion with a *conjunction of
zero-findings* criterion, and §5's row was not updated with it. On my corrupted
report the two instruments produce identical `1b`/`1c` counts and identical 459-line
failure sets — they **agree perfectly** — and the run is not a measurement. Worse,
two of the criterion's failure modes have no §5 row at all: the cold checker
reporting 1b/1c findings while the warm pass passes (fails (ii) alone), and the cold
checker refusing to read (fails (iii) alone). Neither is "Criterion 1'' fails" and
neither is "the two instruments DISAGREE". This is D-423's own named defect — the
same claim stated in two sections, and the copies have drifted.

**REPRODUCED** — see `artifacts/wp15d_b_rev3_criterion_reproduction_v1.txt` §1: both
checkers on the corrupted report emit the identical `1b: 459` / `1c: 682 / 341` lines
and identical 459-member failure sets.

**Why MECHANICAL.** Replace the row's condition with "§4B's agreement criterion fails
on any of (i), (ii), (iii)". One clause, and it points at the section that owns the
claim instead of restating it — which is what D-423 asks for anyway.

## NEW 4 — MAJOR, **MECHANICAL**. §4B's dry-run bullet still evaluates the dry run against revision 2's superseded COUNTS criterion, so the registered criterion is never shown to have been exercised on a real instance under its own labels.

**Claim, verbatim (`:369-370` and `:390-392`):**

> - *Criterion*: the warm pass reproduces the verdict WP-1.7 recorded from it, and
>   the two instruments agree on (i) and (ii).

> The warm pass exits **0** and reproduces WP-1.7's own `h0` at 341 pairs. The
> cold checker exits **non-zero on clause (b)**. **(i) 459 = 459 and (ii)
> 682 games / 341 pairs = 682 / 341: the agreement criterion HOLDS.**

**The attack.** Under revision 3's own criterion, `(i)` is "the warm pass exits
`ATTRIBUTABLE (0)`" and `(ii)` is "BOTH instruments report ZERO `1b` move-list
mismatches and ZERO `1c` rebuild mismatches". "(i) 459 = 459" does not parse under
that (i), and "(ii) 682 / 341 = 682 / 341" is precisely the count agreement the same
document says four paragraphs earlier "the review proved invariant". `git diff
b75c1f6..a62ffca -- docs/experiments/wp15d_b_prereg.md` shows this passage untouched
by the revision. So the one place the document records its criterion being exercised
on a real report of the kind — which is what `docs/process.md`'s dry-run discipline
demands and what BLOCKING 2 required — records the exercise of a **different**
criterion.

**REPRODUCED.**

```
$ git diff b75c1f6..a62ffca -- docs/experiments/wp15d_b_prereg.md | /usr/bin/grep -c '459 = 459'
0                              # the sentence is carried over unchanged
$ /usr/bin/grep -n '459 = 459' docs/experiments/wp15d_b_prereg.md
391:  cold checker exits **non-zero on clause (b)**. **(i) 459 = 459 and (ii)
```

**Why MECHANICAL.** The evidence for each of the three terms is already in the two
named artifacts — warm exit 0 (`wp15d_b_dryrun_v1.txt:18`), warm `PASS — 0
failure(s)` ⇒ zero 1b/1c mismatches (`:25`), cold `FAIL — 1 failure(s)` whose one
failure is the clause-(b) verdict ⇒ zero 1b/1c mismatches (`:31-32`), and neither
refused. I re-ran the honest warm pass and got `:19-25` back byte-identically. So the
fix is to restate the sentence over the registered terms; no re-measurement, no
change of semantics.

## NEW 5 — MAJOR, **SUBSTANTIVE**. The criterion-falsification receipt's row B — restated as §4B's own table row — attributes its captured output to an input that provably did not produce it. The digest its own refusal prints is the digest of the FULL self-consistent referee inversion, not of a single-game `result` flip.

**Claim, verbatim (`docs/experiments/wp15d_b_prereg.md:332-338`):**

> **AND IT IS SHOWN TO FAIL, WHICH IS THE WHOLE POINT** — receipt
> `artifacts/wp15d_b_criterion_falsification_v1.txt`:
> …
> | game 2's `result` flipped `p1_win`→`p2_win` | warm exits **2**, refusing: the
> replay document is bound to the report's sha256 and "the two documents are not
> about each other" | **FAILS (i)** |

**The receipt's own captured output (`artifacts/wp15d_b_criterion_falsification_v1.txt:15-21`):**

> `== B. VERDICT-FIELD CORRUPTION (game 2 result p1_win -> p2_win) -> CRITERION FAILS`
> `warm: exit 2 (NO_ANSWER). It binds the replay document to the report's digest:`
> `  warm_attribution_check: CANNOT READ: the replay document was taken from a report whose sha256 is 082`
> `  ce8b45867c20bbfaa49a5aa233c2fb4045a58c56dd2e7c7fe908ed9f2a5fb, and \`…/mutated_report.txt\` hashes to 39d`
> `  d4f86322d05103f511621a601c5218341329f41a4394b37980cc28d25a6fc`

**The attack.** `39dd4f86…` is not the digest of any single-game mutation. I built
all three candidates from the same honest report and hashed them:

**REPRODUCED.**

```
$ python3 artifacts/wp15d_b_rev3_mutation_instrument_v1.py     # + the two single-game variants
honest WP-1.7 report                           : 082ce8b45867c20bbfaa49a5aa233c2fb4045a58c56dd2e7c7fe908ed9f2a5fb
game 2's `result` flipped, nothing else        : e84c36829139e08edc8a06b2162d64e980f7f178f1fe39ae9aeb03f1bafbe149
game 2 flipped AND made self-consistent        : 2910163b817814a76da292f97b79a113eade4bef7936e526c10e016e710ff0ad
ALL 459 decided games flipped, self-consistent : 39dd4f86322d05103f511621a601c5218341329f41a4394b37980cc28d25a6fc
                                        row B's own output prints ^^^^^^^^ this one
```

The file row B was actually run on is byte-identical to my full self-consistent
referee inversion — every decided game flipped, `counts`, the pentanomial, `llr_pair`
and `verdict` all rewritten. A sha256 agreement across a 354 KB file is not
coincidence.

**Why this matters beyond the label.** The document's most careful-sounding paragraph
(`:343-352`) says:

> This session rebuilt the referee inversion independently — 459 decided games
> flipped, the replay document rebound to the new digest — and the warm pass exited
> **2** … because **flipping `result` alone leaves the `score_a`, `llr` and
> pentanomial fields inconsistent with it**. … The review's measurement stands on the
> review's own receipt; this session's does not corroborate it.

On this evidence the session **did** have a fully self-consistent inversion on disk —
row B's file — and ran it against a *stale* replay binding, while the "attempted
reproduction" it recorded as failing (row E) used a *partial* mutation. Had row B's
file been run with the replay rebound, it would have reproduced the review's 459
`FAIL 1b` lines exactly, as mine does. So the document's account of what its own
session could and could not reproduce rests on a mis-identified input.

**Why SUBSTANTIVE, not mechanical.** There is no one-clause edit. Closing it requires
(i) establishing what row B's input actually was — the `/tmp` scratch is gone,
(ii) re-running the intended row-B mutation, (iii) re-exporting the receipt under a
new digest, and (iv) re-deciding what the "failed reproduction" paragraph may claim.
That is a new measurement and a change to the admissibility of a registered receipt's
rows — the dispatch's own definition. **My own reproduction (NEW 1 of the ledger)
independently establishes the property row D asserts, so the criterion's soundness is
not in doubt; the receipt's record of which input produced which output is.**

## NEW 6 — MAJOR, **MECHANICAL**. §7.7's timeout probe names its sample two incompatible ways. The literal reading selects openings inside WP-1.7's already-consumed slice, which makes the clause's own justification false.

**Claim, verbatim (`:617-625`):**

> over **the first 50 openings of the governed slice —
> `random_openings_v1.txt` lines `1500..1549`, in file order, none skipped** —
> … The 50 openings are part of the governed slice and are played by the run itself,
> so this consumes nothing and biases nothing

**The attack.** The book carries 61 comment/blank lines before its first opening, so
opening index *i* sits at **file line i + 62**. The two readings name **disjoint**
samples, and the literal one lands in WP-1.7's consumed `1000..1499` slice — so under
it the sentence's own justification ("part of the governed slice … played by the run
itself") is false.

**REPRODUCED** (full log: `artifacts/wp15d_b_rev3_timeout_probe_v1.txt`):

```
$ F=crates/pistol-cli/tests/fixtures/random_openings_v1.txt
$ wc -l < $F ; /usr/bin/grep -c -E '^\s*#|^\s*$' $F ; /usr/bin/grep -c '^start ' $F
2061
61
2000
$ /usr/bin/grep -n '^start ' $F | sed -n '1501p' | cut -d: -f1     # opening index 1500
1562

# the probe, run as written, on each reading (armed seat, go nodes 50000):
A. openings 1500..1549  (file lines 1562..1611)  MAX = 445 ms  ->  269.7x
B. file lines 1500..1549 (== openings 1438..1487) MAX = 333 ms  ->  360.4x
```

Both clear the 24× gate by an order of magnitude, so on this data the ambiguity does
not change the outcome — but a registered gate that can stop a launch must not have
two readings, and `docs/process.md`'s dry-run discipline ("a pre-registration's
literal commands are exercised before its review passes") would have caught it: this
newly registered command was never exercised.

**Why MECHANICAL.** "openings `1500..1549`, i.e. file lines `1562..1611`" — one
clause, no re-measurement, no change to the gate.

## NEW 7 — MINOR, **MECHANICAL**. §4's `hang_timeout_ms` row still says the margin is confirmed against §3's bench, which §7.7 explicitly withdraws.

**Claim, verbatim (`:211`):** "`hang_timeout_ms` | 120000 — liveness only, never an
adjudication (D-159); **its margin is confirmed at the slot pass against §3's own
worst single search**".

**Against §7.7 (`:602-609`):** "**neither bench fixture is the governed workload**, so
neither figure confirms the timeout on the workload it actually guards … **The
registered discharge is therefore the SLOT PASS**" — a probe over the *book*, not
over §3. Two sections give two different discharges for one gate. D-423.

**REPRODUCED:** `/usr/bin/grep -n "§3's own worst single search" docs/experiments/wp15d_b_prereg.md` → `211`.

## NEW 8 — MINOR, **MECHANICAL**. §4B's discharge of `docs/process.md`'s second-instrument test points at a table row that records only ONE instrument's reaction.

**Claim, verbatim (`:354-359`):**

> **THAT IS WHY THE SECOND INSTRUMENT IS NOT BLIND TO THE SAME STAGE.** … 
> `docs/process.md`'s test — "two instruments blind to the same stage are one
> instrument reported twice" — is **answered by the third row above, where the two
> react to the same corruption through different mechanisms**.

**The attack.** The third data row of that table (`:339`, the move-list reordering)
records only the **warm** pass ("warm exits **2** on a structural invariant"); the
cold checker does not appear in it. The row where both do appear (`:340`, the referee
inversion) has them reacting through the **same** mechanism — I diffed the two `1b`
and `1c` implementations and they are the same carried-over algorithm, differing only
in wrappers and `split("\n")` vs `splitlines()`. So no row shows what the sentence
says. §4B `:141-146` already discharges the same requirement correctly and
independently (the cold checker has no warm-replay per-engine subprocess state), so
this second attempt is the duplicate D-423 warns about.

**REPRODUCED:** the two diffs are quoted in part (d) §4.2(a) above.

## NEW 9 — MINOR, **MECHANICAL**. The cost paragraph's governed-run estimate is 3.5–7× what its own named source records, and its "few seconds each" for the two checker passes is ~12 minutes for one of them.

**Claim, verbatim (`:167-169`):**

> The governed run: 500 openings × 2 seats over 4 workers at the same budget —
> **ESTIMATED 1.5–3 hours** wall from WP-1.7's own comparable run, plus one
> Criterion 1'' pass and one second-instrument pass **of a few seconds each**.

**REPRODUCED.**

```
$ /usr/bin/grep -m1 '^timing n_workers' artifacts/wp17_governed_run_v1.txt
timing n_workers 4 wall_ms 1049514 discarded_in_flight 9 hang_timeout_ms 120000
$ /usr/bin/grep -m1 '^counts ' artifacts/wp17_governed_run_v1.txt
counts n 682 …
```

1 049 514 ms = **17.5 minutes** for 682 games at 4 workers; scaled to this run's
1000-game cap, ≈ **26 minutes**. And measured on this machine, on a report of the
same kind: the warm pass returns in **< 1 s**; the cold pass — which replays 1364
turns through both engines — took **~12 minutes** (00:23:14 → 00:35:25 by file
mtime; another job shared the CPU, so read this as an order of magnitude, not a
figure). D-291 marks an estimate a finding when it could have been measured; here
the measurement is a line in the artifact the estimate itself cites.

**Why MECHANICAL.** Replace the bracket with one derived from the cited artifact, and
say which pass is seconds and which is minutes. Nothing anyone may conclude changes.

## NEW 10 — MINOR, **MECHANICAL**. Two registered artifacts are consumed without being named or digested.

`/usr/bin/grep -n 'artifacts/' docs/experiments/wp15d_b_prereg.md` returns six hits.
`artifacts/wp15d_b_dryrun_v1.txt` is **not among them**, yet §4B `:378-392` quotes
eight of its lines and reads six figures off it. `artifacts/wp15d_b_criterion_
falsification_v1.txt` is named at `:333` **without a digest**. D-483 requires a
number be "cited from that run's artifact by digest", which §7.1 does correctly for
the measurement artifact and nowhere else. Both artifacts exist and both verify —
I confirmed the dry-run block is byte-identical to `wp15d_b_dryrun_v1.txt:21-25` and
`:29-31`, and re-ran the warm pass to get those lines back byte-identically — so this
is a citation gap, not an unsourceable number.

## NEW 11 — MINOR, **MECHANICAL**. "`bench_positions_v1` is 15- and 35-stone positions" is false at one of its 24 positions.

**Claim, verbatim (`:604-606`):** "`bench_positions_v1` is 15- and 35-stone positions
and its worst single median is **491 ms (a 244× margin)**".

**REPRODUCED** from the registered artifact: `{15: 12, 35: 11, 31: 1}` — `p16` is a
**31**-stone position, and it is one of the two degenerate positions §7.3 names two
sections earlier. (The 491 ms / 244× figures are correct.)

## NEW 12 — MINOR, **MECHANICAL**. A cited line range does not contain the expression it is cited for.

**Claim, verbatim (`:341`):** "the cold `1a` … flags `answers[mover] != played[free]`
(`tools/wp15b_attribution_check.py:286-290`)".

**REPRODUCED:**

```
$ awk 'NR>=284&&NR<=290{printf "%d: %s\n",NR,$0}' tools/wp15b_attribution_check.py
284:             if answers[mover] != played[free]:
285:                 failures.append(
286:                     f"1a game {index} turn {free + 1}: the report attributes …
…
289:         if here == 0:
290:             unattributed.append(index)
```

The expression is at `:284`; `:289-290` is a different branch. Same defect in the
receipt (`wp15d_b_criterion_falsification_v1.txt:33`).

## NEW 13 — MINOR, **MECHANICAL**. The timeout probe registers neither the verbatim command nor a replication procedure — two of the five fields D-376, the precedent it cites, registered before running.

D-376 registered, before its own run: "**THE COMMAND, verbatim** … `printf 'position
start moves <M>\ngo nodes 50000\nquit\n' | target/release/pistol --config …`"; the
fixture with its sha256 and "all 24 positions … none skipped"; the budget; the config;
and "**THE PROCEDURE, fixed here before game one of the probe**: run the 24-invocation
sweep TWICE, independently". `docs/process.md`'s cheap-run clause makes replication
the registered answer to instrument doubt, and this probe takes about fifteen seconds.
§7.7 registers the sample, config, budget, binary, field, statistic, threshold and
consequence — but no command and no replication.

**Measured, so the architect can size it:** I replicated the probe three times and the
maximum moved by 3 ms — `447 / 448 / 445`, a 0.7 % spread against a 24× gate at 270×.
So the missing replication is **not load-bearing on this data**; it is a registered
omission against the document's own cited precedent.

---

## REJECTED — attacks I could not reproduce

**R1. "The registered probe's sample is too shallow to exhibit the risk the gate
guards."** The rev-2 review's NEW 4 argued the statistic is monotone in the sample and
that the governed workload plays into the stone-count range where `BENCH/SPREAD` reads
10 573 ms — so a probe over *opening* positions would discharge the gate by
construction. I tested it and it does not hold. Eight real WP-1.7 games truncated to
turn 39 — the deepest governed-shape positions the run can produce — searched on the
armed seat at `go nodes 50000`:

```
time_ms = 446, 217, 363, 200, 155, 265, 243, 675   (nodes = 50176 on all eight)
MAX = 675 ms  ->  120000/675 = 177.8x
```

against the probe's own worst opening at 445 ms / 269.7×. A real turn-39 position
costs about 1.5× the probe's worst, not 24×. The spread fixture's 10 596 ms comes from
an artificially spread 99-stone position whose first non-abortable iteration runs to
4 283 392 nodes — a shape real play does not reach. **The attack is rejected and the
probe's sample is representative within a factor of ~1.5.**

**R2. "Term (ii) cannot fire, so NEW 1 is not really closed."** Rejected: it fires,
459 findings on both instruments, reproduced above.

---

# 6. WHAT I CHECKED AND FOUND SOUND

**This is the larger part of the work.**

### The measurement and every number resting on it

Every calibration, corpus, spread and sensitivity figure in the document reproduces
from `artifacts/wp15d_b_measurement_v1.txt` and from nowhere else — I recomputed all
of them from the raw lines rather than checking the prior reviews' arithmetic
(`artifacts/wp15d_b_rev3_number_sweep_v1.txt`). The three "verbatim" quotations
(`CAL/SELECTED`, both `SENS/TRAJECTORY` lines) are byte-identical to artifact lines
42 and 242-243 under `diff`. The selection is correct at every step: threshold
`0.75 × 0.3640 = 0.2730`; qualifying points `{4, 8, 16}`; largest is **K = 16**; K32's
`+0.2350` genuinely misses. `K_bind = 64` derived independently from `capped_rows > 0`,
its gain `+0.0140` far below threshold, so §2's falsifiability branch is reachable and
did not fire — and K = 64 really is the grid's `capped_rows` maximum at 818 937, which
makes the decay argument rest on a point where the treatment is applied hardest.
K = 128 and K = 0 agree on the mean, on every histogram bucket and on both counters,
exactly as claimed.

### The criterion, verified rather than accepted

I did not take the rev-2 review's reproduction on faith, and the dispatch was right to
insist: the launching session's own export receipt records that the review's inputs
"are not anywhere on disk". I rebuilt the construction from scratch and every clause
of §4B's row at `:340` holds — identical counts, 459 findings on both instruments,
`1a` unmoved. **The criterion revision 3 registers is a criterion the named defect
falsifies, and it is now backed by a receipt in the main tree rather than by prose in
a review report.**

### The bench registration

Directions in the repository's own convention (ttd ON/OFF, larger is worse, and it is
the gate; `nps` explicitly not a gate across seats with different candidate policies,
D-374). The corpus registered as a no-regression check rather than a place a gain is
expected. The spread fixture reported-not-gated with the reason stated so the report
cannot be read as a gate, and D-95's debt correctly left where D-478 put it. The IQR
gate registered before the results with the withhold-and-re-measure consequence, and
0 of 168 cells violate it — I recomputed all 168. The like-for-like premise the ratio
actually needs — **each position's node count identical across all seven seats** —
holds at all 24 positions, which I checked cell by cell.

### The SPRT arm

All 21 mandatory arena-schema keys are present in §4C's block and no unknown key is;
`openings_skip + openings_take = 1500 + 500 = 2000` sits exactly inside the book, as
the "nearly spent" paragraph records. `configs/instrument_staged_snk_v0.toml` is
committed and differs from the incumbent on **exactly one non-comment line**,
`safety_net_top_k 0 → 16`, which is §2's selected value. The slice accounting is
correct against the three prior governed-run artifacts' own `openings_skip` lines
(0 / 500 / 1000), the calibration sample (`0..999`) and the verdict sample
(`1500..1999`) are disjoint, and `openings_skip` is fixed in the document rather than
at launch — D-427's own lesson. `elo1 = 15.0` matches WP-1.6's and WP-1.7's own
`sprt` lines, so a verdict here is comparable with theirs. Engine A is the capped
seat, stated twice with the reason, and the arena's statistic is slot A's score, so
the sign is right.

### Every revision pin, re-derived at `a62ffca`

`a14912a` is the last commit touching `crates/pistol-arena/`; `bfdf933..HEAD` over the
five pinned files is exactly **181 deletions and 0 insertions**; the only non-comment
change anywhere in the crate is `stub_engine.rs` at `3 25`; `6c929da`, `a80a864` and
`70cb580` are each the last commit touching their file. The exit-taxonomy citation
`:133-136` and the count-before-adjudication citation `:835-841` are both exact.
Criterion 1'' is quoted in full and without ellipsis.

### The dry run itself

The dry-run input is WP-1.7's own preserved governed report and replay — a real
instance of the kind, never the governed workload — exactly as `docs/process.md`
requires. Its defect class ("a checker that cannot read this arena's report at all")
is genuinely excluded by reproducing a **known** verdict, which exit status alone
could not do. And it did the thing dry runs exist for: it falsified this document's
own first criterion before a reviewer had to, and §4B records that plainly instead of
quietly swapping the criterion. I re-ran the warm pass on that input and got
`wp15d_b_dryrun_v1.txt:19-25` back **byte-identically**. NEW 4 is a finding about the
sentence that reads the result, not about the dry run.

### §9's recorded breach, and §7.2's withdrawal

§9 puts the sequencing failure on the document's own face and separates mitigation
from excuse; I re-verified the commit clock independently — `70cb580` at 19:47:30,
`4ec470f` at 19:48:50, and the artifact's own 2662.25 s subtracted from its
write time puts the run's start at 20:39:53, so "~52 minutes" is right. And §7.2 does
not soften revision 1's false claim, it **withdraws** it, names the review that
measured it false, and replaces it with an argument that verifies at every step —
which is the shape a document should have when it is wrong.

---

# 7. WHAT WOULD FLIP THIS TO PASS

**One finding is SUBSTANTIVE, so under the dispatch this package returns to the
architect and revision 4 is not self-granted.** The list below is what a
future round would have to do, in the order of the cost of doing it.

**Substantive — needs an architect ruling and a new measurement:**

1. **NEW 5** — re-establish what `artifacts/wp15d_b_criterion_falsification_v1.txt`
   row B was actually run on, re-run the mutation the row claims, re-export the
   receipt under a new digest, and re-decide what the "attempted reproduction that did
   not reach the term it aimed at" paragraph (`:343-352`) may claim. My own receipt
   (`artifacts/wp15d_b_rev3_criterion_reproduction_v1.txt`) already establishes the
   property that row D asserts, so the criterion itself needs nothing.

**Mechanical — twelve enumerable one-clause edits:**

2. **NEW 1** — delete "The reviewer's partial re-run puts the pool maximum in
   **(100, 120]**, so"; the sentence's remainder is carried by `capped_rows = 0` at
   K = 128 in the registered artifact. (Keeping the figure instead is substantive.)
3. **NEW 2** — add §5 rows for `invalid_forfeit` and arena exit 1, plus one
   import-by-reference row in `wp17_sprt_prereg.md:136`'s style; and correct "the
   arena's fourth token" to "one of the arena's five tokens".
4. **NEW 3** — restate §5's attribution row as "§4B's agreement criterion fails on any
   of (i), (ii) or (iii)".
5. **NEW 4** — restate `:369-370` and `:390-392` over the criterion's registered
   terms; the evidence for all three is already in the two named artifacts.
6. **NEW 6** — write the probe's sample as "openings `1500..1549`, i.e. file lines
   `1562..1611`".
7. **NEW 7** — delete "against §3's own worst single search" from `:211` and point at
   §7.7.
8. **NEW 8** — delete `:354-359`; `:141-146` already carries the claim correctly.
9. **NEW 9** — derive the wall estimate from `wp17_governed_run_v1.txt`'s own
   `wall_ms 1049514`, and say which checker pass is seconds and which is minutes.
10. **NEW 10** — name `artifacts/wp15d_b_dryrun_v1.txt` with its digest at `:378`, and
    add the falsification receipt's digest at `:333`.
11. **NEW 11** — "15-, 31- and 35-stone positions".
12. **NEW 12** — cite `tools/wp15b_attribution_check.py:284`.
13. **NEW 13** — register the probe's verbatim command and whether it replicates, at
    D-376's level.

**Nothing** in the calibration, the bench, the selection of K = 16, the seat
assignment, the slice, the arena config, the criterion's own three terms, or the
measurement artifact's numbers needs to change. The run stands.

---

# 8. CONCURRENT WORK IN THE LIVE TREE, NOTED SO IT IS NOT MISTAKEN FOR MINE

While this review ran, the launching session independently built and exported its own
inverted-referee reproduction (`artifacts/wp15d_b_referee_inversion_{repro,instrument,
report,replay}_v1.*`, appended to `wp15d_b_worktree_export_receipt_v7.txt` under
D-486). I read it after finishing my own. Its mutated report hashes to `c532be4f…`
and mine to `39dd4f86…` — the two constructions differ in that theirs also repairs
`first_player_wins`, which neither checker reads — and both reach the same conclusion
by the same route. **These artefacts postdate the named revision and change nothing in
the artefact under review**, but they do mean the provenance gap at `:340` (a figure
cited to a reviewer's unexported probe) now has two independent receipts in the main
tree rather than none. They do not bear on NEW 5, which is about the digest printed
inside `wp15d_b_criterion_falsification_v1.txt` row B's own captured output.

---

**Worktree receipt (D-469).** Evidence exported to the main tree before removal, four
files, digests listed in the header. Worktree removed:

```
$ git worktree remove /home/tom/Projects/pistol-wt-rev3 && git worktree list
/home/tom/Projects/HeXO-AlphaBeta  a62ffca [dev]
/home/tom/Projects/pistol-wt-repro a62ffca (detached HEAD)      <- NOT MINE; the
      launching session's concurrent D-486 worktree, left untouched. It has since
      been removed by its own session:
$ git worktree list
/home/tom/Projects/HeXO-AlphaBeta  a62ffca [dev]
```

No file in the live tree was edited other than this report and the four exported
`artifacts/wp15d_b_rev3_*` receipts.
