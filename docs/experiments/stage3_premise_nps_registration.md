# Stage-3 detector premise gate — the current-nps re-measurement, REGISTERED BEFORE THE RUN

**What this document governs.** ONE descriptive measurement of the INCUMBENT at
`dev` HEAD `21e05f8`: the WP-1.8c bench's two committed seats, re-run unchanged,
so the premise memo's re-derived target can be stated at TODAY's nps instead of
at WP-1.8c's. **No engine change is measured here** — nothing has been written —
so no rule-5 gain bracket applies and none is registered. What IS registered is
the reading, because the run re-reads a bracket that already has a verdict.

Registered at `21e05f8` plus the two untracked files of `git stash create`
`1831dfbe8b70bd28efc92fb5bdd2097779be311a`, **before the engine was built and
before any measurement was taken.**

Companion document: `docs/experiments/stage3_detector_premise_memo.md` §3.5,
which states the estimate this run replaces and marks it HYPOTHETICAL.

---

## 1. Why this run exists

The premise memo re-derives the factor the WP-1.8c bracket demands from that
bench's own rows, and gets ~190x (corpus band 15) / ~250.9x (band 35) / ~35.8x
(trigger-rich) **at WP-1.8c-era nps**. The dispatch asks for the factor "at
current nps". Between WP-1.8c and HEAD, D-502 and D-507 landed measured
whole-engine nps gains. A search-node speedup divides the memo's `a` and does
not touch its `c` (a df-pn visit does not go through `pistol-eval`), so the
correction moves the target **up**. The memo's §3.5 states that direction with a
hypothetical factor. **An estimate that could have been measured is a finding
(D-291)**, and this one costs about twenty minutes of machine time, so it is
measured.

## 2. What is unchanged, and the one thing that is not

| input | state at HEAD |
|---|---|
| `configs/bench_wp18c_solver_off.toml` | sha256 `10f836fd…` |
| `configs/bench_wp18c_solver_on.toml` | sha256 `8414509a…` |
| `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` | sha256 `931c50b1…`, 24 entries, 12 at `stones 15` and 12 at `stones 31`/`stones 35` |
| `crates/pistol-cli/tests/fixtures/bench_solver_positions_v1.txt` | sha256 `bf1e41b3…`, 20 entries |
| `tools/bench_block.sh` | sha256 `00533b0c…`, last changed at `fbc8e62` |

**THE SEATS ARE NOT BYTE-IDENTICAL TO WP-1.8c's, and this is stated rather than
discovered.** `git log -- configs/bench_wp18c_solver_{on,off}.toml` names
`e4bb5bf` after `e5140cc`: the safety-net cap's landing added
`safety_net_top_k` to every committed config. Both seats carry the value **0**,
which D-484 records as the gate-off value in every committed config, so the
mechanism is inert — but "inert" is a claim, and §4's criterion N is what
checks it rather than asserting it.

## 3. The literal commands

Built first, with nothing else on the machine (`ps -eo cmd | grep -c '[c]argo'`
must read 0 before the bench begins — WP-1.9b §8 hazard 5):

```bash
cargo build --release --locked -p pistol-cli
sha256sum target/release/pistol
```

Then, for each of the FOUR (seat x fixture) combinations — two seats over two
fixtures; the three BANDS are a reading of `stones` within the corpus fixture,
not separate runs:

```bash
tools/bench_block.sh \
  --engine target/release/pistol \
  --config configs/bench_wp18c_solver_{off,on}.toml \
  --fixture crates/pistol-cli/tests/fixtures/bench_{positions,solver_positions}_v1.txt \
  --grammar tail \
  --budget 'nodes 50000' \
  --reps 5 \
  --label 'stage3-premise-<seat>-<fixture>'
```

**Reps 5 on every combination, including trigger-rich.** WP-1.8c took 1 rep
there and called it diagnostic; 5 is stricter, not weaker, and is recorded here
as a deliberate departure rather than left to be noticed in the output.

## 4. The dry run, and the defect class each criterion excludes

Taken on a 2-entry slice of `bench_positions_v1.txt` written to the session
scratch directory, at `--reps 1`, on the OFF seat, BEFORE any governed leg.

- **D-1 — the block loads what it was given.** `bench_block: done: 2 entries x 1
  reps = 2 totals lines, 0 refused`, exit 0. *Excludes D-475's class*: an entry
  the engine refuses, whose `error` line goes to stdout at exit 0, leaving the
  sweep to aggregate the previous position's numbers.
- **D-2 — the record carries the field the reading needs.** Each `record` line
  states `nps` and `nodes`. *Excludes*: a reading taken from a totals line that
  does not carry the quantity the verdict is about.
- **D-3 — the ON seat reports its two counters.** One OFF record line carries
  neither `search_nodes` nor `solver_nodes`; one ON record line on the same
  entry carries both. *Excludes*: an ON seat whose gate is not actually on,
  which would make the whole run measure one seat twice.

**AMENDED BEFORE ANY GOVERNED LEG, and recorded rather than edited away.** This
section first said "six (seat x fixture) combinations" while its own brace
expansion spells two seats over two fixtures. Four is what the brace expansion
always meant and four is what runs; the miscount reached no number, because it
was found by executing §4's dry run against the registered text before the
governed legs began. Same class as WP-1.9b's one-rep amendment (`3e004e1`) —
found by dry-run discipline, corrected before the run, recorded here.

## 5. What is read, and how — fixed before the numbers exist

Per (seat, band), over the record lines:

- **nps** = sum(`nodes`) x 1000 / sum(`time`), the same aggregation the premise
  memo's instrument applies to the WP-1.8c rows, so the two are comparable by
  construction. The per-rep MEDIAN and its IQR are also reported, because
  WP-1.8c's recorded numbers are medians and the IQR gate is repo convention
  (10 %, D-215/D-362).
- **`u`** = sum(`solver_nodes`) / sum(`search_nodes`) on the ON seat.
- **ratio** = nps(ON) / nps(OFF), per band.
- **`k`** = nps(OFF, HEAD) / nps(OFF, WP-1.8c recorded), per band, against the
  recorded 250,776 (band 15), 206,975 (band 35) and 147,036 (trigger-rich).
  **Both terms of every ratio are named** (D-483).
- The target factor is then recomputed by `tools/stage3_premise_derive.py`
  from THESE rows, by the identical code path that read WP-1.8c's.

**Criterion N — node identity, and it is the check that licenses reading `k` as
a speed ratio at all.** For each of the 24 corpus entries, the OFF seat's
`nodes` at HEAD must equal the `nodes` WP-1.8c's OFF rows recorded for the same
entry. If they match, the two seats search the same tree and `k` is speed. **If
any differ, `k` is NOT a speed ratio and is not reported as one** — the run
still stands as a measurement of the incumbent at HEAD, and the discrepancy is
recorded as its own finding, with `safety_net_top_k` the first suspect.

## 6. The registered readings, quoted and not paraphrased

The bracket this run re-reads is WP-1.8c's, whose wording D-465 fixes:

> The registered bound is ≥ 0.5 in BOTH corpus bands and ≥ 0.25 trigger-rich,
> aborting below 0.5 corpus or below 0.1 trigger-rich

- **The bracket still aborts at HEAD** — the expected outcome, and the one every
  number above predicts. It is recorded as *the WP-1.8 arc's abort re-measured
  under post-WP-1.9b nps*, and the re-derived factor at HEAD replaces the memo's
  §3.5 hypothetical.
- **The bracket passes at HEAD** — which nothing in the record leads this
  session to expect. It would mean the incumbent ON seat became affordable
  without a detector, would make the premise memo's §3 arithmetic wrong in a way
  §5's criteria should have caught, and is a STOP for re-reading before anything
  is concluded from it.

**THIS RUN DOES NOT DISCHARGE D-504, and the session does not claim it does.**
D-504 discharges the WP-1.8 nps-jump limb *"inside the detector's own bracket"*,
and there is no detector. Whether a standalone re-measurement discharges the limb
instead is one of the rulings the premise memo leaves to the operator; until that
ruling, the limb stands open and this document is evidence for it, not its
receipt. D-504's Stage-2-exit limb and D-428's quiescence re-test are untouched
either way.

---

## 7. THE RESULT, read against §5 and §6 and nothing else

Taken at `21e05f8`, engine `target/release/pistol` sha256
`e0eb1b196d0c384d57aa272f29815fa619025245b8a6a40a3e1de1d76f6ff453` — the digest
WP-1.9b's closure recorded for its shipped landing `13abe40`, four
documentation-only commits earlier, so the binary measured is the one that
package shipped. Machine idle: `ps -eo cmd | grep -c '[c]argo'` read 0 before the
first leg and no build ran during any of them.

**§4's dry run: all three criteria MET**, on a 2-entry slice at `--reps 1`.
D-1 `bench_block: done: 2 entries x 1 reps = 2 totals lines, 0 refused`, exit 0.
D-2 every record line carries `nodes` and `nps`. D-3 the ON seat's entry 0 reads
`search_nodes 2486 solver_nodes 49619` where the OFF seat's same entry carries
neither field — the gate is demonstrably on.

**The four governed legs, all exit 0, all `0 refused`:**
`stage3_premise_nps_{off,on}_{positions,solver_positions}_v1.txt` under
`artifacts/`, 120 / 120 / 100 / 100 totals lines.

**Criterion N — MET on every leg.** All 24 corpus and all 20 trigger-rich OFF
entries reproduce the exact `nodes` counts WP-1.8c recorded: **0 differences of
44**. The ON seat's aggregate `search_nodes` and `solver_nodes` are identical to
WP-1.8c's too (170,815 and 2,948,225 on band 15). `safety_net_top_k`, the one
key the seats gained since WP-1.8c (`e4bb5bf`), is therefore measured inert at
its committed 0 rather than assumed so. `k` is a speed ratio.

| band | OFF nps | IQR | ON nps | IQR | ratio | bound | reading |
|---|---|---|---|---|---|---|---|
| CORPUS 15 | 439,819 | 0.51 % | 19,319 | 0.49 % | **0.0439** | ≥ 0.50 | **ABORTS** |
| CORPUS 35 | 375,684 | 0.83 % | 8,953 | 0.68 % | **0.0238** | ≥ 0.50 | **ABORTS** |
| TRIGGER-RICH | 297,184 | 0.49 % | 6,557 | 0.36 % | **0.0221** | ≥ 0.25 | **ABORTS** |

IQR gate CLEAN on all six medians, every one under 0.9 % against the 10 %
convention.

**`k`, both terms named.** OFF: 439,819/250,776 = **1.754**, 375,684/206,975 =
**1.815**, 297,184/147,036 = **2.021**. ON: 19,319/20,278 = **0.953**,
8,953/9,478 = **0.945**, 6,557/7,173 = **0.914**.

**§6's first branch fires, as expected: the bracket still aborts at HEAD, and it
aborts HARDER.** Every band's ratio is roughly half what WP-1.8c recorded, and
the reason is measured rather than argued: the search is 1.75-2.02x faster and
the gate-on seat is *slightly slower*, because the df-pn visit does not go
through `pistol-eval`. The residual per-visit cost moved the same way — 52.18 →
54.55, 114.59 → 121.26, 164.28 → 180.65.

The re-derived target at HEAD, by the same instrument and the same code path
that read WP-1.8c's rows (`artifacts/stage3_premise_nps_derivation_v1.txt`):
**379.3x (band 15) / 507.7x (band 35) / 88.3x (trigger-rich)**, against the
190.0x / 250.9x / 35.8x at 1.8c-era nps. This replaces the premise memo's §3.5
HYPOTHETICAL, whose whole point was that it should be measured.

**§6's last paragraph stands unchanged: THIS RUN DOES NOT DISCHARGE D-504**, and
the session does not claim it does. See the memo's ruling 7.
