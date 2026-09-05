# REVIEW — `docs/experiments/wp22_cap_prereg.md` revision 3

**What I reviewed.** Revision 3 of `docs/experiments/wp22_cap_prereg.md` as it
stood in the working tree while `git log -1` read `71fa6f1` (`dev`), together
with the two instruments it names beside it: `tools/texel/draw_census_samples.py`
and `crates/pistol-core/examples/fixture_key_full.rs`.

**Does it match HEAD?** No. All three files are **UNTRACKED**
(`git status --porcelain -uall` → `?? docs/experiments/wp22_cap_prereg.md`,
`?? tools/texel/draw_census_samples.py`,
`?? crates/pistol-core/examples/fixture_key_full.rs`). `71fa6f1` contains none of
them and neither does any commit. This is the same condition round 1 raised as
F13, unchanged.

**VERDICT: FAIL.**

The fix round landed most of what was asked — sixteen of the twenty-one findings
are fully addressed and the document is a great deal better than revision 2. It
fails on seven, of which three are new and measured rather than argued:

- **M1** — §6's sizing table mixes two different 20-position samples. The 16384
  row is reproduced exactly by the **superseded** join-based dry run over
  `rows[-20:]`; no 16384 census over the new dry-run slice exists.
- **M2** — §3's load-bearing new claim, *"the two are the same census"*, is
  **false, and I measured it**: 229 vs 236 firings and 13 of 20 entries differing,
  with the instrument confirmed deterministic by a byte-identical repeat run.
- **M9** — §4's *"the arms differ in the cap and in nothing else"* is false at the
  new ladder too (mean total nodes 41 888 / 44 519 / 49 344 per position, tail up
  to 1.64x the registered budget), and §1's `nodes/(2·cap)` scaling law — the
  reason the ladder's top rung is where it is — is contradicted by the document's
  own dry run (predicted 3.1 / 1.5 firings, measured 4.40 / 3.05).

Plus **M3** (the registered fallback, "the incumbent cap", has no referent
anywhere in the tree), **M5** (§2 reinstates the exact sentence F1 called false),
**M6** (F13 carried, untouched) and **M7** (the sizing's own data cannot carry it).

---

## Re-derivation ledger

Per `docs/process.md` *Re-derivation*, every load-bearing number below was
produced by a command I chose. Scope stated beside each. Binaries under
`target/release/examples/` were **run** (built by the author at 15:26–15:27); no
`cargo` was invoked, in the live tree or anywhere.

| # | claim under test | my command / scope | result |
|---|---|---|---|
| R1 | manifest identity | `sha256sum artifacts/arc3r_sweep_deduped_manifest.txt` | `00f61780cc…caf35968`, equal to the committed index at `arc3_ledger.md` §5 — **holds** |
| R2 | 89 805 body rows | `/usr/bin/grep -vc '^#'` on the manifest | `89805` — **holds** |
| R3 | instrument revision `0f58533` "unchanged at `71fa6f1`" | `git diff --stat 0f58533 71fa6f1 -- …/trigger_census.rs`, plus `git diff HEAD --` | both empty — **holds** |
| R4 | §2's "the seat every prior CENSUS in this tree was taken at" | `--nodes` over **every** `trigger_census` output in `artifacts/` | 17 files at 50 000, **6 at 400 000** — **false**, see M5 |
| R5 | §10.1 | `grep -o '--nodes [0-9]*' artifacts/wp20b_cap_out_*.txt` | 6 files, all `--nodes 400000` — **reproduces** |
| R6 | §10.2 | entry/`att_proved` counts on `stage3b_census_corpus_r{0,1}_v1.txt` | 24 entries, 361 firings, **1** proof; and 370 / **4**; both `--nodes 50000 --cap 2048` — **reproduces** |
| R7 | §10.3 | `sed -n '194,199p' crates/pistol-search/src/pvs.rs` | line 196 carries *"Every stop check and every report reads THIS"*, 197–198 the sum — **citation accurate** |
| R8 | §9's honest run | `fixture_key_full dry2.txt \| paste - dry2.txt.expected_key_full \| awk …` | **0 of 20 rows disagree** — reproduces |
| R9 | §9's shift | recomputed[0..18] against expected[1..19] | **19 of 19 rows disagree** — reproduces |
| R10 | §9's 500-row check | same pipeline over the 500-row calibration fixture | **0 of 500 rows disagree** — reproduces |
| R11 | §6's 16384 row | tally of `artifacts/wp22_cap_dryrun/dryrun_census.txt` (the **superseded** sample) | firings/position **3.05**, att-proved distinct keys **3** of 20 = **0.15** — §6's two printed figures, exactly. See M1 |
| R12 | §6's 2048 / 8192 rows | tally of the scratchpad's `dry2_c2048.txt` / `dry2_c8192.txt` (the **new** slice) | 11.45 / 4.40 firings, 4 / 1 distinct keys; `dry2_c8192.txt` carries `WALL=74` = 3.70 s/position. **No `dry2` run at 16384 exists** |
| R13 | "the two are the same census" | built the move-order fixture for the SAME 20 rows (join verified on all 20 by `key_pos`, `key_full` and `key_seq`), censused at `--cap 2048`, compared | 229 → **236** firings, 206 → **210** distinct keys, 6 → **8** def-proved keys, **13 of 20 entries differ**. See M2 |
| R14 | is that difference noise? | re-ran `dry2.txt --cap 2048` and `cmp`'d against the stored output | **byte-identical** — the difference in R13 is the instrument, not run variance |
| R15 | do the arms differ only in the cap? | mean/max `search_nodes + solver_nodes` per entry, per arm | mean **41 888 / 44 519 / 49 344**; max **53 783 / 65 783 / 82 212**; 14–15 of 20 entries overrun 50 000 at **every** arm. See M9 |
| R16 | does the cap bind at 16384? | `max(att_visits)`, `max(def_visits)` per arm | exactly 2048 / 8192 / 16384 — **the cap binds**; F3's fatal form is gone |
| R17 | "win-proving positions / position" | entries carrying an `att_proved true` row, per arm | **2 / 1 / 2** of 20 — the yields 0.20/0.05/0.15 are distinct **keys**, not positions. See M7 |
| R18 | Q2, the empty position | `fixture_key_full` and `trigger_census` on a one-line `start moves` fixture | prints `-` (= the manifest's `key_full`); censuses cleanly, 50 176 nodes, 0 firings, exit 0 — **nothing breaks** |
| R19 | where the empty row lands | rank of the `key_full == "-"` row in the registered hash order | **19 866** → the census slice, never the governed run |
| R20 | "one per distinct position" | `Counter` over the manifest's `key_full` column | **387 duplicated values**; the calibration slice holds **4 rows sharing one `key_full`** → 500 rows, **497 distinct positions**. See M14 |
| R21 | "the incumbent cap" | `/usr/bin/grep -rn 'incumbent cap' docs/`; `grep -rn per_call_node_cap configs/` | defined **nowhere**; configs carry 16384 (×18), 2048 (×2), 512 (×1); every prior census ran `--cap 2048`. See M3 |
| R22 | D-563's sizing quotation | `grep -o "each arm is sized on its OWN rate[^.]*\." docs/decisions.md` | verbatim — **holds** |
| R23 | slice extrapolation | turn-count profile of the dry-run slice vs the calibration slice | mean 17.95 vs 17.15 — comparable; s/position extrapolates on that axis |

---

## Half A — did the fix round land?

| # | round-1 finding | status | evidence |
|---|---|---|---|
| F1 | D-563's figures are at `--nodes 400000`, seat is 50 000 | **PARTIALLY ADDRESSED** | §2's second paragraph does the correction fully and in the right words. Its **first** paragraph reinstates the sentence F1 called false ("the seat every prior CENSUS in this tree was taken at") — R4 shows six prior `trigger_census` outputs at 400 000. See **M5** |
| F2 | the tree's 50 000-node censuses contradict the registered expectation | **ADDRESSED** | §4's expectation paragraph is replaced with the measured 1/361 and 4/370, and "no arm is expected to return zero" is registered. R6 reproduces both counts |
| F3 | 131072 is not a cap at this seat | **ADDRESSED as to the fatal form; the replacement rule is unsound** | The rung is dropped and the budget bound is stated. At 16384 the cap genuinely binds (R16: `att_visits` tops out at exactly 16384). But the bound does not deliver §4's stated consequence, and §1's scaling law is contradicted by the dry run — see **M9** |
| F4 | there is no sizing | **PARTIALLY ADDRESSED** | §6 is a real derivation with each arm's own rate, which is what D-563 asked for. Its inputs are a mixed sample (**M1**) and its statistics do not carry (**M7**) |
| F5 | the all-zero branch's conclusion is not identified | **ADDRESSED** | §5 registers the rule-of-three `3/F` bound, licenses nothing below it, and names a larger SAMPLE rather than a larger ladder as the remedy — F5's minimal fix, near verbatim |
| F6 | the tie band is not total and not transitive | **PARTIALLY ADDRESSED** | The rule is now "smallest cap within 10 % of the maximum ratio over selectable arms" — total in arity, no pairwise ordering. The precision floor is registered at `w < 10`. Two gaps remain: "within 10 %" is directionally ambiguous, and the floor's consequence has no referent — see **M4**, **M3** |
| F7 | the corpus digest is deferred to run time / circular | **ADDRESSED** | §3 cites `arc3_ledger.md` §5 as the committed index and §11 voids on disagreement with the **ledger**, not with the receipt. R1 confirms the digest. (The literal digest is not printed; pointing at a committed index discharges the finding) |
| F8a | the draw is unverified | **PARTIALLY ADDRESSED** | The schema assertions (§3, and `load()` in the draw) pin the `# columns:` line, 8 columns and 89 805 rows — which closes "wrong column index" and "the manifest gained a column". **Not closed:** wrong sort key and wrong slice, and F8a's asked-for differently-written reproduction of the sample digests is absent |
| F8b | the cap reaching the solver is unverified | **PARTIALLY ADDRESSED — and this is the half the task asked about** | §9's new check ("`--gate on` must produce a positive firing count on the dry-run fixture") closes only the *gate armed nothing* half, which is the D-563 failure. It does **not** close F8b's stated defect: *"if `--cap` were ignored, clamped, or the three arms otherwise identical, all three would return the same `w` and `s`, §5 would call it a tie"*. A positive firing count is invariant under a cap that does nothing. F8b's own remedy — per arm `max(att_visits) <= cap`, and total firings falling as the cap rises — is registered nowhere, though R16 shows both would pass today |
| F9 | §5 states the count 28, which §1 forbids | **ADDRESSED** | §5 uses `N` throughout and adds the N-independence clause. (§1 still names 28 in the sentence forbidding it — **M16**, a nit) |
| F10 | the dry-run slice sits inside the census slice | **ADDRESSED** | §3's table gives three disjoint slices 0–19 / 20–519 / 520–end, and `slice_of()` implements exactly that. R19–R20 confirm no tie straddles the 519/520 boundary |
| F11 | the join is unnecessary and its stated reason is false | **ADDRESSED IN MECHANISM, NEW DEFECT IN THE REASON** | The join, the out-of-tree dependency and the join defect class are all gone. But F11 told the author the true statement — *"replaying `key_seq` searches a symmetric twin"* — and revision 3 wrote something stronger and false instead. See **M2** |
| F12 | D-563 truncated at the constraining clause | **PARTIALLY ADDRESSED** | The truncated quotation is gone (§6 quotes a different D-563 sentence, verbatim per R22). The clause's substance — the calibration's records must be excluded from the corpus by construction — is still not discharged in text, which F12's minimal fix also asked for |
| F13 | the document is untracked; instrument revisions name a document revision that exists nowhere | **NOT ADDRESSED** | The document, `tools/texel/draw_census_samples.py` and `crates/pistol-core/examples/fixture_key_full.rs` are all untracked at `71fa6f1`, and §8's table identifies two of the three instruments as *"this document's revision 3"* — the exact unfalsifiable identifier F13 named. See **M6** |
| F14 | "one octave" is three; the 11 040 fact does not support the rung | **NO LONGER APPLIES** | The 131072 rung is gone |
| F15 | §1 forbids constraining the census sample while §3 constrains it | **ADDRESSED** | The "may not choose the census sample" clause is deleted; §3 reserves the census slice openly |
| F16 | nothing pins the census's own budget | **ADDRESSED** | §1 carries the sentence F16 asked for, near verbatim |
| F17 | the `depth_turns` breakdown is the label's depth | **ADDRESSED** | §7 breaks down by the `key_seq` word count and says explicitly why `depth_turns` was wrong |
| F18 | the VOID list mixes re-run with repair and omits the gate signature | **ADDRESSED** | §11 splits VOID-AND-RE-RUN / VOID-AND-REPAIR / FAIL and adds both signatures F8b named |
| F19 | the `--gate off` arm is unmotivated | **ADDRESSED** | Dropped, with the reason stated |
| F20 | §7's cost is 8x wrong | **PARTIALLY ADDRESSED** | The cost is restated at the registered seat and is right where it is measured (500 × 3.70 s = 30.8 min for the 8192 arm, from `WALL=74`). Two of the three per-position seconds have no artifact I can find, and the 4.40 belongs to the superseded sample — **M1**, **M11** |
| F21 | the draw asserts no schema | **ADDRESSED** | `load()` raises named `SystemExit`s on the columns line, the column count and the row count |
| Q1 | is the sweep corpus as trigger-dense as `bench_positions_v1`? | **ANSWERED** | Measured in the dry run: 11.45 firings/position at cap 2048 on sweep positions against 15.0 on the bench fixture. Denser than the pessimistic case; the projections do not collapse |
| Q2 | does the empty position's `canonical_form` serialize as `-`? | **ANSWERED, but not in the document** | R18: `fixture_key_full` prints `-` (`EMPTY_FIELD`), matching the manifest, and `trigger_census` searches `start moves` cleanly. R19: the row is at rank 19 866, i.e. the census slice, so it cannot reach the governed run. The document says none of this; §B's census will meet it |
| Q3 | 2 369 `key_disagreements` | **NOT ADDRESSED**, and it has a concrete consequence | R20: 387 duplicated `key_full` values; the calibration slice draws 4 rows sharing one. See **M14** |

---

## Half B — attacking revision 3 on its own terms

### M1 — MAJOR. §6's sizing table mixes two different 20-position samples, and the 16384 row is from the superseded join-based draw.

**What is wrong.** §6 states its rates are *"measured at the registered seat on
sweep-corpus positions, **in §8's dry run**"* — and §8's dry run is §3's slice,
rows 0–19 of the hash order, drawn by `key_seq`. Two of the three rows are.
The third is not.

- `artifacts/wp22_cap_dryrun/dryrun_census.txt` — argv `--cap 16384`, over
  `dryrun_fixture.txt`, which the **superseded** `draw.py` in the same directory
  built from `rows[-20:]` **through the join to
  `/home/tom/pistol-runs/…/corpus.txt`** — tallies to **3.05 firings/position**
  and **3 att-proved distinct keys of 20 = 0.15** (R11). Those are §6's two
  printed 16384 figures, digit for digit.
- The new slice's censuses (`dry2_c2048.txt`, `dry2_c8192.txt` over `dry2.txt`)
  give 11.45 / 4.40 firings and 4 / 1 keys (R12) — §6's other two rows.
- **There is no 16384 census over `dry2.txt`.** The 16384 run is timestamped
  15:30:22; `dry2.txt` did not exist until 15:33:13.

**Why it matters.** Three things break. (a) §6's provenance sentence is false for
one row in three. (b) The sizing's governing input — *"the worst measured
position-level yield"* — is chosen by comparing arms that were not run on the
same positions, and §4's own principle ("the arms differ in the cap and in
nothing else") is violated in the very measurement used to size the run. (c) The
finding §6 leans hardest on, **"THE DRY RUN'S OWN YIELDS ARE NOT MONOTONE IN THE
CAP — 4, 1, 3"**, is not a comparison at all: 4 and 1 are one population, 3 is
another. The precision floor is right for other reasons (M7), but the reason §6
gives for it does not hold.

**Minimal fix.** Run `--cap 16384` over `dry2.txt` and replace the row, or delete
the 16384 row and say the arm is unmeasured. Either way, drop the
non-monotonicity paragraph or re-derive it from one sample.

### M2 — MAJOR. "The two are the same census" is false. I measured the difference on the document's own dry-run slice.

**What is wrong.** §3 justifies dropping the join with: *"The position searched is
the canonical image rather than the corpus's own move order; the census counts
distinct `canonical_key`, which is invariant under exactly that symmetry, **so the
two are the same census**."*

The premise is sound and the conclusion does not follow from it.
`canonical_sequence` minimises the **turn list**; `canonical_form` minimises the
**sorted stone set** (`crates/pistol-core/src/symmetry.rs`). Replaying the
canonical sequence therefore yields `g·S` for some symmetry `g`, and
`canonical_key(g·S) == canonical_key(S)` — so **the keys are invariant**, and I
confirmed it end to end (R8, R10: 0 of 20 and 0 of 500 disagreements). But the
SEARCH is not equivariant: `symmetry.rs`'s own doc on `canonical_sequence` says
so in as many words — *"D-7's final tie-break is lexicographic by `(q, r)` and is
therefore not symmetry-invariant, so two mirrored openings usually do not produce
mirrored games"*. A different move order at every node is a different tree, a
different firing set, and a different wall clock.

**Measured, on the same 20 rows** (R13; the join verified row by row on `key_pos`,
`key_full` and `key_seq` before use), `--nodes 50000 --cap 2048`:

| | canonical image (`key_seq`) | as played (`moves`) |
|---|---|---|
| firings | 229 | **236** |
| distinct `key` | 206 | **210** |
| win-direction distinct keys | 4 | 4 |
| loss-direction distinct keys | 6 | **8** |
| entries with identical `(firings, search_nodes, solver_nodes)` | — | **7 of 20** |

17 of the 20 rows have a `key_seq` that differs from the played `moves`, and 13
of 20 entries came out different. **This is not run variance**: R14 re-ran the
canonical arm and got a byte-identical file, as hard rule 4 requires.

**Why it matters.** The false sentence is doing real work. It is the whole
justification for the §3 rewrite, and it licenses a silence: nothing in this
document requires **§B's census** to draw the same way. If the census builds its
fixture from the corpus `moves` column, the cap was calibrated on a measurably
different instrument — and §1 already argues, correctly, that a cap does not
transport across seats. The magnitude here is ~3 % on firings and 33 % on the
loss-direction count at n = 20; the point is not the size, it is that the
document asserts zero.

**Minimal fix.** Two sentences. Replace the false clause with F11's true one —
*"replaying `key_seq` searches a symmetry twin of the labelled position, which is
the same position and not the same search; the keys are invariant, the tree is
not"* — and register the binding consequence: *"§B's census must build its
fixture from `key_seq` by this same draw; a census built from the `moves` column
re-opens this calibration."*

### M3 — MAJOR. The registered fallback, "the incumbent cap", has no referent anywhere in the tree.

**What is wrong.** §5: *"If the selected arm's `w` is below 10, the selection is
reported as UNDERPOWERED and **the incumbent cap stands**."* R21: the phrase
occurs nowhere else in `docs/` except round 1's own quotation of it. The
candidates are not close together — `configs/` carries `per_call_node_cap =
16384` in eighteen files, `2048` in two (`bench_wp18c_solver_{on,off}.toml`) and
`512` in one (`gate_staged_solver_v0.toml`), while **every prior census in the
tree ran `--cap 2048`**. 16384 and 2048 are 8x apart and both have a claim to the
word.

**Why it matters.** This is the branch that fires when the run is thin, and the
run may well be thin (M7). A pre-registration whose fallback is ambiguous by 8x
leaves exactly the after-the-numbers choice §5 exists to forbid: whichever cap
the outcome favours can be called the incumbent.

**Minimal fix.** Name it: *"the incumbent cap is 2048, the cap every census in
this tree has been taken at (`artifacts/stage3{b,c}_census_*`), and an
UNDERPOWERED result leaves it in force."*

### M4 — MAJOR. The selection rule is total in arity but not in meaning, and its floor is perverse on a reachable input.

**(a) "Within 10 %" has two readings.** `ratio >= 0.9 * max` and `ratio >=
max / 1.1` differ (0.900 vs 0.909 of the maximum), and with three arms the
difference can change which cap is selected. A registered rule must pick one.

**(b) The floor vetoes the run on the selected arm alone.** The floor tests
`w` of the arm the ratio rule picked, and its consequence discards the whole
ladder. Given the ladder's near-equal seconds (1775 / 1850 / 2200 s at n = 500),
`w(2048) = 9` and `w(16384) = 11` give ratios 0.00507 and 0.00500 — within any
reading of the band — so the rule selects 2048, finds `w = 9 < 10`, reports
UNDERPOWERED, and throws away an arm that met the floor. The document's stated
purpose is to find the cheapest cap per proof; discarding a qualifying arm
because a **different** arm was thin is not that.

**(c) One more undefined edge.** §5 excludes `w = 0` arms and handles all-zero,
so arity is covered — I attacked that and it holds. But `s(c)` is never defined
as a measurement: whose wall clock, whether the fixture build is inside it,
measured once or replicated. Hard rule 4 makes the bestmove deterministic; it
does not make a 30-minute wall clock so, and the 10 % band sits on a ratio whose
denominator has no registered method. (Raised as a QUESTION below, not asserted
as a defect, because `w`'s noise dominates.)

**Minimal fix.** Write the band as an inequality with a direction; apply the floor
to **every** arm before the ratio rule (*"an arm with `w < 10` is not selectable;
if no arm is selectable, report UNDERPOWERED"*), which makes (b) disappear; and
state how `s(c)` is taken.

### M5 — MAJOR. §2 reinstates, in capitals, the sentence F1 called false.

**What is wrong.** §2: *"**`--nodes 50000` is the seat every prior CENSUS in this
tree was taken at**"*. R4, over **every** `trigger_census` output in `artifacts/`
rather than the two families the document names: 17 at `--nodes 50000` and
**6 at `--nodes 400000`** — `wp20b_cap_out_{corpus,trigger-rich}_{on,off}_*.txt`.
Those six are `trigger_census` runs, they are censuses, they are prior, and the
document's very next paragraph concedes they are at 400 000. §2 therefore
contradicts itself across two paragraphs.

**Why it matters.** F1 named this sentence, quoted it, and gave the replacement
wording. Half the fix was applied (the D-563 correction) and the false half was
kept and emphasised. It is also the only justification offered for the registered
seat, so a reader checking the seat checks a false sentence first.

**Minimal fix.** F1's own wording: *"`--nodes 50000` is the seat the stage-3b and
stage-3c censuses were taken at, and NOT the seat D-563's cap figures were taken
at."*

### M6 — MAJOR (carried). F13 is untouched: the document and two of its three instruments are untracked, and are identified by a document revision that exists nowhere.

`git status --porcelain -uall` shows all three untracked at `71fa6f1`. §8's table
gives `fixture_key_full.rs` and `draw_census_samples.py` the revision *"this
document's revision 3"* — a document that exists in no commit and no stash, so
"changed since revision 3" is unfalsifiable and `docs/process.md`'s *Instrument
governing revision* cannot bind. CLAUDE.md's requirement that a review be
dispatched against a named revision is likewise unsatisfiable; this report can
only name a working-tree state and a timestamp. `trigger_census.rs`'s revision is
the one that is properly named, and it verifies (R3).

**Minimal fix.** `git stash create` (or commit) the three files and put that SHA
in §8's table in place of "this document's revision 3".

### M7 — MAJOR. The sizing's own data cannot carry it, in three separate ways.

**(a) The column is mislabelled and the unit is not a position.** §6's header
reads *"win-proving positions / position"*. R17: the entries that actually
produced an `att_proved true` row are **2 / 1 / 2** of 20, not 4 / 1 / 3. The
figures 0.20 / 0.05 / 0.15 are **distinct in-tree `key`s per fixture position**,
which is the right unit for `w` — but it means the proofs are **clustered**: four
keys from two positions at cap 2048. The sizing's implicit model ("500 positions
× a per-position yield") is a Bernoulli-per-position model that the data
contradicts, and clustering inflates `w`'s variance well above the binomial the
sizing assumes.

**(b) The intervals swallow the derivation.** Clopper-Pearson 95 % on the three
counts:

| arm | count | point | 95 % CI | `n` for E[w] = 20 at the CI lower bound |
|---|---|---|---|---|
| 2048 | 4/20 | 0.20 | [0.057, 0.437] | 349 |
| **8192** | **1/20** | **0.05** | **[0.0013, 0.249]** | **15 809** |
| 16384 | 3/20 | 0.15 | [0.032, 0.379] | 624 |

The sizing takes the 8192 arm's 0.05 as *"the worst measured"* and derives
`20 / 0.05 = 400`, then registers 500. At that arm's 95 % lower bound the same
target needs **15 809 positions — 32x the registered `n`**, and E[w] at n = 500
is 0.65. A single observation is not a rate.

**(c) The three arms are not separated at all.** Fisher exact, two-sided:
4/20 vs 1/20 → **p = 0.34**; 4/20 vs 3/20 → **p = 1.00**. §6's admission that the
yields are "not monotone" understates it — nothing in the dry run distinguishes
any arm from any other, so "the LEAST productive arm" names an arm the data does
not identify. (This is also why M1's population mixing matters less than it
might: the ordering was never real.)

**Why it matters.** F4 asked for a sizing and §6 delivers the right *shape* — each
arm on its own rate, D-563's instruction honoured. But the registered `n` rests on
a single event, and the document presents 500 as clearing a floor it may miss by a
factor of thirty. The precision floor in §5 is the right instinct; it is a floor
on the OUTPUT, and what is missing is honesty about the INPUT.

**Minimal fix.** Rename the column *"win-proving distinct keys / position"*.
State the interval beside each point estimate and register the consequence
plainly: *"n = 500 clears E[w] = 20 on the point estimates; the 8192 arm's rate is
one event and its 95 % interval admits an E[w] near 1, which is what §5's
UNDERPOWERED branch exists to report."*

### M8 — MINOR. §6's stated mechanism is contradicted by §6's own numbers.

*"a firing at a larger cap is likelier to prove, which partly cancels there being
fewer of them."* Per-firing win-proof rates from the same tallies: **4/229 =
0.0175** at 2048, **1/88 = 0.0114** at 8192, 3/61 = 0.0492 at 16384 (different
sample, per M1). The 8192 arm is *lower* than 2048, so the monotone mechanism
fails on the one within-sample comparison available. Nothing in the sizing rests
on the mechanism, which is why this is MINOR. **Fix:** delete the clause, or state
the three per-firing rates and let them speak.

### M9 — MAJOR. The new ladder's arms are not equal-compute either, and §1's scaling law — which sets the ladder's top — is contradicted by the document's own dry run.

**What is wrong.** §4 keeps revision 2's claim, *"Every arm runs the same 500
positions, so the arms differ in the cap and in nothing else."* R15, over the
three dry-run arms:

| cap | mean total nodes / position | max total | max ÷ registered 50 000 | entries over budget |
|---|---|---|---|---|
| 2048 | 41 888 | 53 783 | 1.08x | 14 of 20 |
| 8192 | 44 519 | 65 783 | 1.32x | 15 of 20 |
| 16384 | 49 344 | 82 212 | **1.64x** | 15 of 20 |

Because solver nodes are absorbed **after** a call returns (`pvs.rs:196-198`, and
the doc comment the document itself quotes), the overshoot grows with the cap.
The arms differ in effective compute per position by 18 % on the mean and 1.5x on
the tail. This is F3's mechanism, not eliminated — attenuated.

**And the bound that chose the ladder's top does not predict what happens.** §1
and §4 rest on *"Firings per position scale with `nodes / (2·cap)`"*, giving
headroom 12.2 / 3.1 / 1.5. Measured: **11.45 / 4.40 / 3.05**. The formula is close
at 2048 and wrong by **42 %** at 8192 and **103 %** at 16384 — because most calls
return well before the cap. So "headroom stays above 1" is not the property it is
being used as, and §1's *"the cap that wins at one budget is not the cap that wins
at another"*, which the whole seat-validity framing rests on, is asserted from a
formula the document's own data contradicts.

**What survives.** The cap does genuinely bind at 16384 (R16: `att_visits` and
`def_visits` top out at exactly the cap in every arm), so the top rung measures a
cap and F3's fatal reading is gone. The answer to "is 1.5 still too close" is:
not for the cap's meaning, but yes for §4's equal-compute claim.

**Minimal fix.** Delete "and in nothing else"; register each arm's **mean and max
total nodes per position** in §7 (F3's minimal fix asked for exactly this and it
was not carried over); and restate the scaling sentence as a bound —
*"firings per position are at most `nodes / (2·cap)` and measured well below it"*.

### M10 — MINOR. §9's "at all three caps" is a category error.

*"honest run: 0 of 20 rows disagree, **at all three caps**"*. The criterion is
`fixture_key_full <fixture> | paste - <fixture>.expected_key_full | awk …`;
`fixture_key_full` reads `std::env::args().nth(1)` and nothing else — there is no
cap, no search, and no census in the pipeline. The claim is either vacuous or a
sign the criterion was misread; and per M1 no 16384 census over the dry-run slice
exists in any case. **Fix:** delete the clause.

### M11 — MINOR. §9 records results that live only in an ephemeral scratchpad, while `artifacts/wp22_cap_dryrun/` still documents the superseded run.

All three of §9's recorded numbers reproduce (R8–R10) — but they reproduce from
`/tmp/claude-1000/…/scratchpad/`, and the only receipt in `artifacts/` is
`wp22_cap_dryrun/RECEIPT.md`, whose criterion, defect class ("the join silently
addressing the wrong record"), input (`rows[-20:]`) and instrument (`draw.py`
with the join) are all superseded by revision 3. A reader following the document
to its receipt finds the previous experiment. The standing note that review
material dies with its scratchpad applies directly. **Fix:** export `dry2.txt`,
its three census outputs, the calibration fixture and both recomputed files to
`artifacts/wp22_cap_dryrun_rev3/` with a `sha256sum` receipt, and repoint §9.

### M12 — MINOR. The non-vacuity mutation is of a defect the new pipeline cannot produce.

§9 demonstrates the criterion is not vacuous by shifting the **expected list** by
one row (19 of 19 disagree — R9, reproduced). But in
`draw_census_samples.py:main`, the fixture line and the expected line are appended
inside one loop from one `row` object; a row misalignment between the two files
is structurally impossible. The defect the criterion still genuinely covers is a
**wrong column index** — `KEY_SEQ, KEY_FULL = 2, 4`. **Fix:** mutate that instead
(e.g. `KEY_SEQ = 3`, the `key_pos` column) and record what it produces; keep the
shift if wanted, but stop presenting it as the demonstration.

### M13 — MINOR. §5's collision caveat points at the wrong section, carries no threshold, and has no consequence for the selection.

*"§6 measures the cross-root collision rate…"* — §6 measures no such thing; §7
owns the collision column. And *"a collision rate materially above the
calibration's own is a finding against the projection"*: "materially" is
unquantified, the comparison is against a census this document does not govern,
and "a finding against the projection" changes no output of §5. Round 1's S5 gave
the fix with a number — *"arms differing by more than 2x on that column are
reported as not comparable on the ratio"* — and it was softened rather than
adopted. **Fix:** take S5's wording, or delete the caveat and say the linearity
assumption is untested.

### M14 — MINOR. Q3 is unaddressed, and it has a measurable consequence for "500 positions".

§3: *"89 805 body rows, **one per distinct position** under three-key agreement"*.
R20: **387 `key_full` values appear more than once**, which is the manifest's own
`# derived key_disagreements 2369` seen from one column. Concretely, the
calibration slice draws **four rows sharing the `key_full`
`-5,1:p2 0,0:p1 2,0:p2`** — the same three-stone position, four times — so the
governed run searches 500 rows over **497 distinct positions**. Two further
consequences: `w` (distinct keys) collapses duplicates while `s` pays for them,
biasing the ratio very slightly downward; and the registered order
`sha256(key_full)` is **not injective**, so the slice boundaries depend on
Python's stable sort over the manifest's row order. That is reproducible given the
sha-pinned manifest (and I verified the 519/520 boundary is not a tie), but the
tie-break is unregistered. **Fix:** one sentence in §3 stating the row/position
distinction with the number, and one naming the stable-sort tie-break.

### M15 — MINOR. The draw is a `tools/` instrument with no test, and two of its failure paths are not named errors.

`docs/process.md`, *tools/ review coverage rule*: *"any tools/ script that
produces a recorded number carries at least one test driving the shipped
script."* `tools/texel/draw_census_samples.py` produces every sample this
registration uses and has no test. Separately, `load()` raises named
`SystemExit`s for the three schema failures — good, that is F21 — but
`open(MANIFEST)` uses a repo-root-relative path and the default locale encoding,
so running it from anywhere else, or under a non-UTF-8 locale against a manifest
whose header carries em-dashes, raises a bare `FileNotFoundError` /
`UnicodeDecodeError` rather than the named refusal hard rule 3 asks for and the
same function otherwise gives. **Fix:** a test over a 3-row synthetic manifest
covering all three refusals plus the slice boundaries; `encoding="utf-8"`; and a
named refusal when the manifest is not found.

### M16 — MINOR (nit). §1 states the number it forbids the document to state.

*"It may not state the census count or read anything about the floor of 28."*
§5 does the right thing and uses `N` throughout. §1 names 28 in the prohibition
itself. **Fix:** *"…or read anything about the census's registered minimum."*

### M17 — nit. §6's 477 is 480.

*"1 win-proving position in 24 … 0.042 … needs 477"*. `20 / (1/24) = 480`; 477
comes from the rounded 0.042. 500 clears both, so nothing turns on it.

---

## QUESTIONS — raised, not asserted

**Q-A. Can a canonical sequence ever fail to replay to the same stones, by rule 4?**
`transform_sequence` re-canonicalises each pair through `canonical_pair`, which
**reorders the two stones of a turn by `(q, r)`**. Rule 4 makes intra-turn order
semantically live: if the reordered first stone completes a six, the second is not
played and the replayed board differs from the labelled one. §9's criterion would
catch it, and it does not occur on any of the 520 rows drawn (R8, R10). I could
not establish whether it occurs anywhere in the 89 285-row census slice without a
full pass, which this document does not govern. Raised for §B's registration,
where §11's "a fixture line refused by the rules" would fire — or, worse, would
not.

**Q-B. How is `s(c)` measured?** §5 defines it as *"the arm's wall-clock
seconds"* and §8 says the run is detached and shares the box with nothing being
timed — which is the right precaution but not a method. Whether the fixture build
is inside it, whether it is taken once, and what box load is tolerable are
unregistered, and the selection rule divides by it. `w`'s own noise almost
certainly dominates, which is why this is a question and not a finding.

**Q-C. Does the `key`/`key_pos` column in §7 have anything left to measure?**
In all three dry-run arms distinct `key` equals distinct `key_pos` exactly
(206/206, 79/79, 54/54), i.e. the symmetry fold's in-tree yield is zero on this
population — the same result D-560 reports at the root population. §7 registers
the column anyway. Harmless; possibly already answered.

---

## What I attacked and it survived — said explicitly

**S1. The key-invariance half of the `key_seq` argument is exactly right.**
`canonical_key` is a minimum over `Symmetry::ALL` of the sorted stone list, so it
is invariant under any symmetry of the replayed board, and I confirmed the
consequence end to end rather than by argument: 0 of 20 and **0 of 500**
disagreements between `pistol-core`'s recomputed `canonical_form` and the
manifest's `key_full` (R8, R10), on a pipeline where the expected values never
pass through the fixture builder. The half that fails is the leap from invariant
keys to an identical search (M2).

**S2. The empty position breaks nothing.** Attacked as instructed. The draw emits
`start moves`; `fixture_key_full` renders the empty canonical form as `-`
(`EMPTY_FIELD`), which is exactly how the manifest spells it; and
`trigger_census` searches it cleanly (entry 0, 50 176 search nodes, 0 firings,
exit 0) — R18. Its hash rank is 19 866, so it lands in the census slice and never
reaches the governed run (R19). Q2 is answered in the affirmative and Q2's
proposed exclusion of the degenerate rows is unnecessary.

**S3. The three slices really are disjoint, and reproducibly so.** `slice_of()`
implements §3's table exactly; the 519/520 boundary is not a hash tie; the
manifest is sha-pinned to a committed index (R1). F10 is properly fixed.

**S4. §10's three reproductions all reproduce.** R5 (6 files, all
`--nodes 400000`), R6 (361/1 and 370/4 at `--nodes 50000 --cap 2048`), R7
(`pvs.rs:196-198`, and the quoted sentence is at 196, verbatim). Every one was
checked with my own command over my own scope.

**S5. The instrument revision that is properly named verifies.** `0f58533`,
"unchanged at `71fa6f1`" — empty `git diff` both against `71fa6f1` and against
the working tree (R3).

**S6. D-563's sizing quotation is verbatim** (R22), and §6 honours its
instruction — each arm sized on its own rate — which is the substance F4 asked
for. The execution fails (M1, M7); the shape is right.

**S7. The argmax algebra and the N-independence clause.** `argmin_c N·s/w =
argmax_c w/s` for every positive `N`, so §5 can state the objective without
reading the floor. Correct, and F9 is properly discharged.

**S8. `w`'s unit matches the floor's unit.** The floor is *"28 win-proving firings
on disjoint positions"*; `w` counts distinct census `key`s, which is D-570's
identity for a position. They agree, and the win-direction-only reading with
`def_proved` reported beside it and never summed is what D-522 established and
D-535 preserved. This survived round 1 and survives again.

**S9. The dry-run slice is of the same kind as the calibration slice.** Attacked
as a possible extrapolation defect: mean turn count 17.95 against 17.15 (R23), so
the per-position seconds carry across the slices on the axis most likely to move
them. `docs/process.md`'s dry-run rule — same kind, differing only in identity —
is satisfied for the two arms that were actually run on it.

**S10. The cap genuinely binds at the ladder's top.** `max(att_visits)` and
`max(def_visits)` are exactly 2048, 8192 and 16384 in the three arms (R16), so
each arm measures the cap it names and F3's fatal reading does not return. What
does not survive is the equal-compute claim built on top of it (M9).

**S11. The census instrument is deterministic.** A repeat run of the 2048 arm is
byte-identical (R14), which is what let me attribute M2's difference to the
instrument rather than to variance. Hard rule 4 holds here.
