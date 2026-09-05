# REVIEW — `docs/experiments/wp22_cap_prereg.md` revision 4

**What I reviewed.** Revision 4 of `docs/experiments/wp22_cap_prereg.md`, its
three named instruments (`crates/pistol-search/examples/trigger_census.rs`,
`crates/pistol-core/examples/fixture_key_full.rs`,
`tools/texel/draw_census_samples.py`), the receipt
`artifacts/wp22_cap_dryrun_v2/`, and the ADR lines the document leans on
(D-522, D-535, D-537, D-563, D-570, D-596, D-599, D-601, D-602, D-615, D-617).
Third review of this document; round 1 was on revision 2, round 2 on revision 3.

**Named revision.** `162608b` (`dev`), the revision the dispatch named. **It no
longer matches HEAD**: HEAD moved to `a0be362` while I was reading (two later
commits, `29c954d` and `a0be362`). **The document itself is unchanged** —
`git diff 162608b a0be362 -- docs/experiments/wp22_cap_prereg.md` is empty and
`git status --porcelain -uall` is clean — so every finding below is about bytes
that are still HEAD's. The document is tracked, which is F13/M6's mechanical
half properly fixed.

**VERDICT: FAIL.** Five MAJOR, eight MINOR, three QUESTIONS.

The remedies largely landed. §6's sizing table now reproduces cell for cell from
one 100-position sample on the corpus's own move order; the join is back,
verified per row on both keys, and D-615 records the measurement that put it
back; the equal-compute claim and the `nodes/(2·cap)` law are genuinely
withdrawn, not softened; the incumbent cap is named; the floor is applied to
every arm before the ratio rule and the band has a direction. Sixteen of the
seventeen round-2 findings moved.

It fails on five, of which three are measured:

- **B4** — §6 says *"it does not separate the arms"*. On the quantity §5
  actually selects on it separates them by **6x**, and **§5's rule applied to
  §6's own dry run already returns 2048**. A bootstrap over the dry run's own
  roots selects 2048 in **20 000 of 20 000** replicates at n = 500, and the
  UNDERPOWERED fallback is *also* 2048 — so every branch the sizing admits
  returns the incumbent. `docs/process.md` names this class in terms: *"neither
  catches a run whose answer is already known before it is taken — that defect
  is judged, not checked."*
- **B3** — §9's non-vacuity result, *"19 of 19 disagree"*, is a 20-row figure.
  On §9's own registered 100-position input the shift gives **96 of 99**, three
  rows still agreeing because the slice holds one `key_full` four times.
- **B1** — §1 and §4's measured numbers (41 888 / 44 519 / 49 344, 1.64x, 4.40
  and 3.05) all come from populations this document has retired: the
  canonical-image draw, the `SUPERSEDED`-marked directory, and one file the tree
  does not hold. Round-2's M1 was fixed in §6 and left standing in §1 and §4.

Plus **B2** (§8's instrument revisions name a commit at which neither instrument
exists) and **B5** (§10.2's registered search does not run, and its population
cannot be pinned to a revision at all).

None of the five is a correctness defect in the instruments. B1, B2 and B5 are
provenance and are a few lines each. B3 is one re-run. B4 is two sentences. The
document is close.

---

## Re-derivation ledger

`docs/process.md` *Re-derivation*: every number below was produced by a command
I chose, over a scope I chose. Where the document prints a command I ran mine
first and the document's afterwards. Prebuilt binaries under
`target/release/examples/` were **run**; no `cargo` was invoked anywhere. All
`grep` is `/usr/bin/grep` per D-265.

| # | claim under test | my command / scope | result |
|---|---|---|---|
| R1 | §3 manifest identity | `sha256sum artifacts/arc3r_sweep_deduped_manifest.txt` | `00f61780cc…caf35968`, equal to `arc3_ledger.md` §5 and to the receipt — **holds** |
| R2 | §3 89 805 body rows | `/usr/bin/grep -vc '^#'` on the manifest | `89805` — **holds** |
| R3 | the sixteen corpus files behind the restored join | `sha256sum` of all sixteen `/home/tom/pistol-runs/arc3r-sweep/tranche-*/corpus.txt` against `arc3_ledger.md` §5 | **16 of 16 match** — holds today; nothing in the registration requires the check (**B6**) |
| R4 | §6 row 2048 | my own tally of `artifacts/wp22_cap_dryrun_v2/d3_c2048.txt` (not `tally.py`) | firings/pos **11.63**, `w` **86**, keys/pos **0.86**, proving roots **8 of 100**, keys/root **10.75**, `WALL=341` → **3.41** s/pos — **all seven cells reproduce** |
| R5 | §6 row 8192 | same over `d3_c8192.txt` | **3.90 / 17 / 0.17 / 8 of 100 / 2.125 / 4.03** — **reproduces** |
| R6 | §6 row 16384 | same over `d3_c16384.txt` | **2.67 / 19 / 0.19 / 9 of 100 / 2.111 / 4.80** — **reproduces** |
| R7 | §6 "86 keys come from 8 roots" | per-entry distinct att-proved keys, cap 2048 | roots `{4,7,60,65,67,72,82,98}`, counts **1,3,1,11,35,1,1,33** summing to 86 — holds, and see **B13**: 79 % from two roots |
| R8 | §6 CP interval on 8/100 | exact Clopper-Pearson by binomial-tail bisection | **[0.0352, 0.1516]** against the printed [0.035, 0.151]; lower bound × 500 = **17.6 roots** — **reproduces** |
| R9 | §6 projections 430 / 85-95 | 20 000-replicate bootstrap, 500 roots resampled from the 100 observed, arms coupled on the same roots | means **430 / 85 / 95** exactly; 95 % intervals **[231,657] / [53,122] / [62,133]** — the means reproduce, the spread is unstated (**B13**) |
| R10 | §6 "far clear of §5's floor of 10" | same bootstrap, `P(w < 10)` per arm | **0 of 20 000 at every arm** — the conclusion holds, and holds for a better reason: `w >= #proving roots >= 17.6` at the CP lower bound, needing no multiplier |
| R11 | **§5 applied to §6's own dry run** | `w/s` = 86/341, 17/403, 19/480 | **0.2522 / 0.0422 / 0.0396**; all selectable, `0.9·max = 0.227`, smallest qualifying = **2048**. See **B4** |
| R12 | **which cap §5 selects at n = 500** | same bootstrap, §5 evaluated on each replicate with `s = {1705, 2015, 2400}` | **2048 in 20 000 of 20 000 (100.00 %)**; UNDERPOWERED never fires. See **B4** |
| R13 | §4 "mean total nodes 41 888 / 44 519 / 49 344" | mean `search_nodes + solver_nodes` over `d3_c*.txt` (the live 100-position sample) | **45 806 / 49 079 / 53 398**; max 54 080 / 65 999 / **82 644** = 1.65x. The printed figures are **not** this sample. See **B1** |
| R14 | where §4's figures come from | same tally over the 20-row files | `dry2_c2048.txt` → **41 888.4**; `artifacts/wp22_cap_dryrun/dryrun_census.txt` → **49 344.2**, max **82 212** = 1.644x; no `dry2_c8192.txt` exists anywhere. See **B1** |
| R15 | §1 "4.40 and 3.05 measured" | firings/position, live sample | **3.90 and 2.67**; 3.05 is the SUPERSEDED `wp22_cap_dryrun/dryrun_census.txt`, 4.40 has no artifact. See **B1** |
| R16 | §9 honest run | `draw_census_samples.py dryrun`, then `fixture_key_full \| paste \| awk` | **0 of 100 disagree** — reproduces |
| R17 | §9 500-row check | same over the calibration draw | **0 of 500 disagree** — reproduces |
| R18 | **§9 "shifted by one row: 19 of 19"** | `head -99 recomputed` against `tail -n +2 expected` on the 100-row slice | **96 of 99 disagree** — **does not reproduce**. See **B3** |
| R19 | why three rows survive the shift | `Counter` over `key_full` in the drawn slices | dry-run slice = 100 rows over **97** distinct positions (`-5,1:p2 0,0:p1 2,0:p2` ×4, entries 22-25); calibration slice = 500 rows over **498** — M14 carried, now with a consequence |
| R20 | slice stability | sha256 rank of rows 99/100 and 599/600 | neither boundary is a hash tie — **the slices are stable**; the stable-sort tie-break is still unregistered |
| R21 | the draw is the receipt's | re-ran `draw_census_samples.py dryrun` and `sha256sum` | `7799f206…` = `d3.txt` **byte-identical**, expected file likewise — the draw is deterministic |
| R22 | the instrument is deterministic and d3 is the move-order draw | per-entry firings and full row lines, `d3_c2048.txt`[0..19] vs `dry2_moveorder_c2048.txt` | **236 firings, all 236 row lines identical** — and `dry2_c2048.txt` gives 229, so `dry2_c2048` is the canonical-image arm |
| R23 | §10.1 / D-615's substitution | the two 20-row files | **229 vs 236 firings, 206 vs 210 distinct keys, 6 vs 8 loss keys** — **reproduces exactly** |
| R24 | receipt integrity | `sha256sum -c RECEIPT.sha256` in `wp22_cap_dryrun_v2/` | **9 of 9 OK** |
| R25 | **§10.2's command as printed** | ran it verbatim with `/usr/bin/grep`, and again under the harness's `ugrep` | `grep: unrecognized option '--nodes [0-9]*'` on **every one of the 25 files**, both matchers; pipeline emits nothing, exits 0. See **B5** |
| R26 | §10.2's output, repaired | same command with `-e` before the pattern, scope `artifacts/*.txt` | **15 at `--nodes 50000`, 10 at `--nodes 400000`, 25 files** — the split reproduces once the command is fixed |
| R27 | §2 "the tree holds 25 `trigger_census` outputs" | `/usr/bin/grep -rl 'trigger_census: argv' artifacts/` | **31 files: 10 at 400 000 and 21 at 50 000** — the extra six are this document's own dry-run outputs. See **B5** |
| R28 | §2 "the ten are all `wp20b_*`" / §10.3 | per-file argv, `artifacts/wp20b_*` | 6 `cap_out_*` + 4 `keypos_*` = **10, all `--nodes 400000`** — **holds**; §10.3's six reproduce |
| R29 | §5 "the cap every census in this tree has been taken at" | `--cap` on each of the fifteen 50 000-node runs | **all fifteen at `--cap 2048`** — the substance holds. The pointer glob does not (**B9**) |
| R30 | §5 "16384 … has never been a census's" | `--cap` over `artifacts/wp20b_*` and `d3_c16384.txt` | five `trigger_census` outputs at `--cap 16384` — **contradicted as written** (**B10**) |
| R31 | §4 "1 in 361 firings; `r1`: 4 in 370" | row and `att_proved true` counts on the two files | **361 / 1** and **370 / 4** — **reproduces** |
| R32 | §1 `pvs.rs:196-198` | `sed -n '190,205p' crates/pistol-search/src/pvs.rs` | 196 = *"Every stop check and every report reads THIS"*, 197 the signature, 198 `self.search_nodes + self.solver_nodes` — **citation accurate** |
| R33 | §4 "the cap binds at 16384" | max `att_visits` / `def_visits` per arm | exactly **2048 / 8192 / 16384** — **holds at every rung** |
| R34 | §8 `trigger_census.rs` `0f58533` "unchanged at HEAD" | `git diff --stat 0f58533 HEAD -- …/trigger_census.rs` | empty — **holds** |
| R35 | **§8 the other two instruments at `b876d1d`** | `git cat-file -e b876d1d:<path>` for both, then `git log --diff-filter=A` | **both ABSENT at `b876d1d`**; both first added at `42967e0`, together with the document itself. See **B2** |
| R36 | §6's M8 remedy, per-firing rates | `att_proved true` rows ÷ firings, live sample | **0.0834 / 0.0487 / 0.0749** — non-monotone, as §6 now says |
| R37 | §9's cap-independence claim | read `fixture_key_full.rs` | reads `std::env::args().nth(1)` and nothing else — no cap, no search — **true as stated** |
| R38 | M15, is the draw tested? | `git grep -ln draw_census_samples -- .` | three documents, **no test file**; `tools/texel/test_texel.py` does not drive it |
| R39 | `record_number` bounds (an `IndexError` path) | max `record_number` per `corpus_index` against each tranche's body length | `corpus_index` 1..16, **no overrun** — the path is unreachable on today's manifest |
| R40 | mechanized gates | `python3 tools/design_citation_check.py` and `tools/revision_citation_check.py` on the document | **6 citations checked, 0 unreproduced; 0 revision citations stale** — both green, and as their own output says, green speaks to none of the above |

---

## Half A — did the round-2 remedies land?

| # | round-2 finding | status | evidence |
|---|---|---|---|
| **M1** | §6's sizing table mixes two 20-position samples; the 16384 row is from the superseded draw | **ADDRESSED in §6 — RELOCATED to §1 and §4** | R4-R6: every cell of all three rows reproduces from one 100-position move-order sample, and §6 says so on its face. But R13-R15: §1's *"4.40 and 3.05 measured"* and §4's *"41 888 / 44 519 / 49 344 … 1.64x"* are the 20-row canonical-image sample, the `SUPERSEDED`-marked directory, and one file the tree does not hold. See **B1** |
| **M2** | *"the two are the same census"* is false | **ADDRESSED** | The join is restored; `draw_census_samples.py` verifies `key_full` **and** `key_pos` on every row and refuses with a named `SystemExit` (I read the code and re-ran it, R21); D-615 exists and records 229/236, 206/210, 6/8, which R23 reproduces exactly. R22 confirms `d3` uses the move-order draw and the instrument is deterministic to the row line. The binding consequence M2 asked for lands in the sibling rather than here — see **Q3** |
| **M3** | "the incumbent cap" has no referent | **ADDRESSED** | §5: *"THE INCUMBENT CAP IS 2048"*. R29: all fifteen prior 50 000-node census runs are at `--cap 2048`. Two pointer defects ride along — **B9**, **B10** |
| **M4a** | "within 10 %" has two readings | **ADDRESSED** | `ratio >= 0.9 * max_ratio`, stated as an inequality with a direction. Residue: `max_ratio`'s own population is unstated — **B8** |
| **M4b** | the floor vetoes on the selected arm alone | **ADDRESSED** | Selectability is now per arm and precedes the ratio rule; M4b's pathological input no longer misbehaves — I re-ran it and the rule is well behaved |
| **M4c** | `s(c)` has no method | **ADDRESSED** | `SECONDS` around the single invocation, fixture build excluded, one run per arm, quiet box, sequential. The receipt's `WALL=` lines match (R4-R6). One-run-per-arm is raised as **Q1**, not asserted |
| **M5** | §2 reinstates the sentence F1 called false | **PARTIAL** | The enumeration is honest and the 15/10 split reproduces over `artifacts/*.txt` (R26). But *"the tree holds 25"* is false about the tree (R27: 31, of which 21 at 50 000), and the search backing it does not run (R25). See **B5** |
| **M6** | untracked; instruments identified by a revision that exists nowhere | **PARTIAL — the substance survives** | All three files are tracked and the tree is clean. But R35: `fixture_key_full.rs` and `draw_census_samples.py` are **absent at `b876d1d`**, the revision §8 names, so `git diff b876d1d HEAD -- <path>` says "new file" and process.md's *Instrument governing revision* rule still cannot bind. See **B2** |
| **M7a** | the column is mislabelled; the proofs are clustered | **ADDRESSED** | §6 now prints proving ROOTS and keys-per-proving-root as separate columns and sizes on the root rate. R4-R7 reproduce all of it |
| **M7b** | the intervals swallow the derivation | **ADDRESSED** | The CP interval is printed and reproduces (R8), and the consequence at n = 500 is stated. R10 shows the floor conclusion is robust |
| **M7c** | the arms are not separated | **NOT ADDRESSED — carried onto a sample that contradicts it** | The 20-row basis for M7c is gone, but its conclusion was kept. R11: on the new sample `w/s` is 0.2522 / 0.0422 / 0.0396 and §5 already selects 2048. See **B4** |
| **M8** | the mechanism claim is contradicted | **ADDRESSED** | Deleted, and R36 confirms the replacement statement is true on the live sample |
| **M9** | equal-compute and the scaling law | **WITHDRAWN in words; one limb not carried** | Both are withdrawn explicitly and no predictive law is claimed — verify by reading §1 and §4; neither is softened, both are struck. The supporting numbers are stale (**B1**), and M9's *"register each arm's mean and max total nodes per position in §7"* is **not** in §7 (**B11**) |
| **M10** | "at all three caps" is a category error | **ADDRESSED** | The clause is gone and the cap-independence claim replacing it is true (R37) |
| **M11** | results live in an ephemeral scratchpad | **ADDRESSED** | `artifacts/wp22_cap_dryrun_v2/` exists, `RECEIPT.sha256` verifies 9 of 9 (R24), and `artifacts/wp22_cap_dryrun/SUPERSEDED.txt` marks the old one. Caveat: the receipt carries B3's stale figure too |
| **M12** | the mutation is of a defect the pipeline cannot produce | **NOT ADDRESSED, and now worse** | `main()` still appends fixture and expected in one loop from one `row`, so the shift is structurally impossible; and R18 shows the recorded result is wrong for the registered input. Folded into **B3** |
| **M13** | §5's collision caveat | **ADDRESSED by deletion** | `grep -c collision` on the document returns 0 |
| **M14** | duplicate `key_full`; unregistered tie-break | **NOT ADDRESSED** | R19: 100 rows over **97** positions, 500 over **498**; R20: boundaries are not ties, so the slices are stable but the tie-break is unregistered. It now has a measured consequence (**B3**) |
| **M15** | no test for the draw; unnamed failure paths | **NOT ADDRESSED** | R38: no test. And the restored join adds two new bare-exception paths — see **B7** |
| **M16** | §1 states the number it forbids | **ADDRESSED** | *"or read anything about the floor"* |
| **M17** | 477 is 480 | **NO LONGER APPLIES** | The sizing is rewritten |
| Q-A | can a canonical replay diverge by rule 4? | **NO LONGER APPLIES** | The fixture is the corpus's own move order, so no turn is re-canonicalised. An unremarked dividend of restoring the join |
| Q-B | how is `s(c)` measured? | **ANSWERED** | §5, M4c. Restated narrowly as **Q1** |
| Q-C | is `key` vs `key_pos` still worth reporting? | **UNCHANGED** | R4-R6: distinct `key` equals distinct `key_pos` in all three arms (1038, 356, 252). Harmless; **Q2** |

---

## Half B — attacking revision 4 on its own terms

### B1 — MAJOR. §1's and §4's measured numbers come from three populations this document has retired, while the live measurement sits two sections away.

**What is wrong.** §1: *"the law predicts 3.1 and 1.5 firings per position at 8192
and 16384 against **4.40 and 3.05 measured**"*. §4: *"Measured mean total nodes
per position run to **41 888 / 44 519 / 49 344**, with a tail to **1.64x** the
registered budget."* Traced (R13-R15, R22):

| printed | its actual source | live 100-position value |
|---|---|---|
| 41 888 | `dry2_c2048.txt` — the **canonical-image (`key_seq`) draw**, the one §3 and D-615 declare a different census | **45 806** |
| 44 519 | `dry2_c8192.txt` — **no such file in the tree**, in either receipt directory | **49 079** |
| 49 344, 1.64x, 3.05 | `artifacts/wp22_cap_dryrun/dryrun_census.txt` — the directory the document's own §9 marks **SUPERSEDED** | **53 398**, 1.653x, **2.67** |
| 4.40 | `dry2_c8192.txt` again | **3.90** |

**Why it matters.** This is round-2's M1 exactly — a sizing number taken from a
sample the document has replaced — moved from §6 to §1 and §4 in a revision
declared REMEDIES ONLY. Two of the six numbers come from the canonical-image
draw, which is the very substitution D-615 exists to forbid: the document
refutes the shortcut in its own header and then reports the shortcut's output as
its measurement. One number has no artifact at all, which is what D-601 was
written about. Nothing in the conclusions changes — the arms are not
equal-compute on the live sample either (18 % on the mean, 1.65x on the tail),
and the scaling law is refuted more decisively by 3.90 and 2.67 than by 4.40 and
3.05 — which is why the fix is a substitution and not an argument.

**Minimal fix.** Replace the six figures with the live ones (45 806 / 49 079 /
53 398, tail 1.65x; 3.90 and 2.67) and name their sample, as §6 does. The
20-row files stay where they belong, in §10.1, which is about the substitution
and is correct there.

### B2 — MAJOR. §8's instrument revisions, and the header's governing revision, name a commit at which none of the three files exists.

**What is wrong.** §8 gives `crates/pistol-core/examples/fixture_key_full.rs` and
`tools/texel/draw_census_samples.py` the revision `b876d1d`, and the header says
*"Governing revision of the work this registers: `b876d1d`"*. R35:

```
git cat-file -e b876d1d:tools/texel/draw_census_samples.py        -> absent
git cat-file -e b876d1d:crates/pistol-core/examples/fixture_key_full.rs -> absent
git cat-file -e b876d1d:docs/experiments/wp22_cap_prereg.md       -> absent
git log --diff-filter=A -- <each>                                 -> 42967e0
```

`b876d1d` is *"test(texel): the fit's own tests drive the shipped scripts…"*, two
commits before the one that first added all three. `git diff b876d1d HEAD --
<path>` therefore reports **new file**, not *unchanged*.

**Why it matters.** M6's mechanical half was fixed — the files are tracked — and
its substance was not. `docs/process.md` *Instrument governing revision*: *"a
change to it reopens the review exactly as an amendment to the document does."*
That rule needs a revision at which the instrument can be diffed. A reviewer
asked whether `draw_census_samples.py` has changed since the revision that
governs it cannot answer, for the same reason round 2 could not answer it about
*"this document's revision 3"*. `trigger_census.rs` shows how it is done: `0f58533`,
and R34's `git diff` is empty.

**Minimal fix.** `42967e0` in §8's two rows and in the header, with *"unchanged
at HEAD"* beside each — which is true and checkable today.

### B3 — MAJOR. §9's non-vacuity result does not reproduce on §9's own registered input, and on that input the mutation is no longer total.

**What is wrong.** §9 registers its input as *"the 100-position dry-run slice"*
and records *"the expected list shifted by one row: **19 of 19 disagree**"*.
A shift over 100 rows yields 99 comparisons, never 19. R18, over the registered
input:

```
head -99 recomputed | paste with tail -n +2 expected  ->  96 of 99 disagree
```

Three rows agree **under the shift**. R19 says why: the dry-run slice holds
`-5,1:p2 0,0:p1 2,0:p2` four times, at entries 22-25, and identical `key_full`
values sort adjacently because the order is `sha256(key_full)`. The receipt
`RECEIPT.md` carries the same stale 19-of-19 under a heading that reads *"100
positions each"*.

**Why it matters.** Three things. (a) A registered dry-run result that does not
reproduce on the registered input is the one thing a dry run exists to prevent;
`docs/process.md` requires the pre-registration to *record the dry-run input and
its output*, and these two do not match. (b) The mutation is now demonstrably
**not** total, which is a real property of this criterion on this corpus and is
worth registering rather than discovering during the governed run. (c) It is
M14's duplicate-position fact arriving with a consequence, after two rounds of
being deferred as a nit.

**Minimal fix.** Run the shift on `d3.txt`, record **96 of 99**, and add the
sentence M14 asked for: the dry-run slice is 100 rows over 97 distinct positions
and the calibration slice 500 over 498, because 387 `key_full` values recur in
the manifest; the three surviving pairs are the duplicates. Keep M12's point in
view — the shift remains a mutation of a misalignment `main()` cannot produce, so
if a second mutation is wanted, `M_KEY_FULL = 3` is the one the pipeline can
actually suffer.

### B4 — MAJOR. §6 says the dry run does not separate the arms. On the quantity §5 selects on it separates them 6x, §5 applied to the dry run already returns 2048, and no outcome the sizing admits returns anything else.

**What is wrong.** §6: *"What the dry run does NOT establish, stated plainly
(M7c): it does not separate the arms. Nothing here identifies 'the least
productive arm', and the selection between them is the governed run's to make
under §5."* That is true of the **root rate** (8, 8, 9 of 100) and false of
`w/s`, which is the only quantity §5 reads. R11, from §6's own printed cells:

| cap | `w` | `s` (100 pos) | `w/s` |
|---|---|---|---|
| **2048** | 86 | 341 | **0.2522** |
| 8192 | 17 | 403 | 0.0422 |
| 16384 | 19 | 480 | 0.0396 |

All three clear the floor, `0.9 · max = 0.227`, and the smallest cap clearing it
is **2048**. §5's rule, run on §6's own dry run, already returns the answer.

R12, projecting to the governed sample — 20 000 bootstrap replicates of 500
roots drawn from the 100 observed, the three arms coupled on the same resampled
roots, `s = {1705, 2015, 2400}` from §8's own cost line:

- **2048 selected in 20 000 of 20 000 replicates (100.00 %).**
- `P(w < 10)` is **0 of 20 000 at every arm**, so the UNDERPOWERED branch never
  fires — and its consequence is *"the incumbent cap stands"*, which §5 defines
  as **2048**.

Algebraically, 2048 loses only if `w(2048) < 0.762 · w(8192)` or
`w(2048) < 0.639 · w(16384)`. The dry run has `w(2048) = 5.06 · w(8192)`. That is
a **6.6x reversal**.

**Why it matters.** `docs/process.md`, *Cost, replication, and the second
instrument*, closes with the rule this is under: *"Neither this rule nor the
dry-run rule is mechanized, and neither catches a run whose answer is already
known before it is taken — that defect is judged, not checked."* This is that
judgement. Every branch §6's own sizing admits — selection or UNDERPOWERED —
returns 2048, so a 1.7-hour three-arm run cannot change the cap, and §6's
disclaimer is what keeps that invisible: it tells the reader the arms are
undifferentiated when the document's own numbers differentiate them by 6x.

**This is not a finding that the run should not be taken.** Confirming a 6x
margin on 5x the sample is a reasonable thing to want, and the run is cheap. The
defect is that the document asserts the opposite of what its data show, and
nowhere states what result would change the cap — which is what a
pre-registration is for.

**Minimal fix.** Two sentences. In §6, replace the disclaimer with what is true:
*"the dry run does not separate the arms on the ROOT rate (8, 8, 9 of 100); on
`w/s` it separates them by 6x and already points at 2048."* In §5 or §1,
register the discriminating condition: *"the governed run overturns 2048 only if
`w(2048) < 0.76 · w(8192)` or `< 0.64 · w(16384)`; on the dry run
`w(2048) = 5.1 · w(8192)`, so this run confirms the incumbent with a measured
margin rather than choosing among three."*

### B5 — MAJOR. §10.2's registered search does not run, its output is labelled with a revision the population cannot have, and §2's count of "the tree" is false.

**What is wrong.** Three defects in one paragraph, all in the section titled
*"The reproductions this revision rests on"*.

**(a) The command errors on every file.** R25, run verbatim:

```
/usr/bin/grep: unrecognized option '--nodes [0-9]*'      (× 25)
```

Both GNU `grep` and the harness's `ugrep` parse a pattern beginning `--` as a
long option. The pipeline emits nothing and exits 0, so *"25 files, 25 argv
lines: 10 at `--nodes 400000`, 15 at `--nodes 50000`"* cannot have come from it.
`-e` before the pattern repairs it, and then the split does reproduce (R26).

**(b) "run at `b876d1d`" is a label the population cannot carry.**
`.gitignore:19` is `/artifacts/` and `git ls-files artifacts/` is empty — hard
rule 8, artifacts are never committed. So `git grep <rev> -- 'artifacts/*.txt'`
returns nothing at any revision, and a detached worktree at `b876d1d` has an
empty `artifacts/`. D-602, which this very paragraph cites as its authority:
*"where a document reports a search at a named revision, the search is executed
AT that revision … and never with a working-tree matcher"*, because *"a working
tree is not a revision"*. For this population that is not a slip, it is
impossible; the revision label should not be there at all.

**(c) The count is false about the tree.** §2: *"The tree holds **25**
`trigger_census` outputs"*. R27, over all of `artifacts/`: **31** files carrying
`trigger_census: argv`, **10 at 400 000 and 21 at 50 000**. The six extra are
`artifacts/wp22_cap_dryrun{,_v2}/`, produced by this document's own dry run. The
count was already stale when it was written, which is exactly why D-601 withdrew
counts in favour of searches.

**Why it matters.** §2 opens *"The seat, stated exactly, because revision 3
overclaimed it twice"*, so this paragraph is where the document stakes its
precision. The seat conclusion is unaffected — the six extras are all at 50 000
and strengthen it, and the 15/10 split over `artifacts/*.txt` is right (R26,
R28) — but three of the four things the paragraph does to establish it do not
hold up.

**Minimal fix.** Print the repaired command (`-e` before the pattern); scope the
sentence to `artifacts/*.txt` rather than "the tree"; and drop the revision
label, pinning the population instead by the receipt's own `sha256sum` list,
which is the only pin a gitignored directory can have.

### B6 — MINOR. The join's sixteen inputs are unpinned by the registration.

The restored join reads `/home/tom/pistol-runs/arc3r-sweep/tranche-{1..16}/corpus.txt`.
§3 points at `arc3_ledger.md` §5 as their sha index, but §11 voids only on *"a
manifest digest disagreeing with `arc3_ledger.md` §5"*, `RECEIPT.md` records only
the manifest's digest, and `draw_census_samples.py` computes none. The positions
searched come from these sixteen files, not from the manifest. R3: all sixteen
match the ledger today, so this is a gap in the registration and not a live
defect. **Fix:** extend §11's VOID clause to the corpus digests and have the
draw or the receipt print all sixteen.

### B7 — MINOR. §3 promises named refusals; the join's own failure paths are bare exceptions, and the draw still has no test.

§3: *"named refusals, not an `IndexError`"* — true of the three schema
assertions, which `load()` raises as `SystemExit`. The join added two paths that
are not: `open(TRANCHE.format(index))` raises `FileNotFoundError` if a corpus is
missing or the run directory moves, and `bodies[index][record_number - 1]`
raises `IndexError` on an out-of-range record (R39: unreachable on today's
manifest, `corpus_index` 1..16 and no overrun — so this is hard rule 3 hygiene,
not a live bug). Carried from M15: `open()` still uses the default locale
encoding, and R38 finds no test driving the shipped script, against
`docs/process.md`'s *tools/ review coverage rule* — *"any tools/ script that
produces a recorded number carries at least one test driving the shipped
script"* — which this script plainly is. **Fix:** name the two refusals,
`encoding="utf-8"`, and a test over a synthetic 3-row manifest and a synthetic
tranche covering the three schema refusals, the join refusal and the slice
boundaries.

### B8 — MINOR. `max_ratio` has no stated population, and the two readings disagree on a reachable-by-construction input.

*"Among selectable arms, select the SMALLEST cap whose ratio satisfies
`ratio >= 0.9 * max_ratio`"* — maximum over **selectable** arms, or over all
three? On `w = (9, 9, 10)` with §8's seconds: over all arms, `max = 9/1705 =
0.00528`, `0.9·max = 0.00475`, and the one selectable arm's ratio is
`10/2400 = 0.00417` — **no arm qualifies and the rule returns nothing**. Over
selectable arms it selects 16384. §11 admits *"returns none"* as an adjudicated
outcome but §5 registers no consequence for it. R10 says the input is
unreachable under §6's own projection, which is why this is MINOR and not a
repeat of M4a. **Fix:** *"`max_ratio` is the largest `w(c)/s(c)` among
SELECTABLE arms"* — five words, and the branch closes.

### B9 — MINOR. §5's incumbent-cap pointer does not match its own count.

*"(`artifacts/stage3{,b,c}_census_*`, all fifteen)"*. That glob expands to
**17** files, four of which (`stage3_census_analysis_v1`, `stage3_census_rank_v1`,
`stage3b_census_rank_v2`, `stage3c_census_rank_v2`) are analysis outputs and not
censuses, and it **misses** the two `stage3_trigger_census_*` runs that §2 counts
among the fifteen. R29 confirms the substance — all fifteen 50 000-node runs are
at `--cap 2048` — so only the pointer is wrong. **Fix:** `artifacts/stage3*` and
the argv filter, or drop the glob.

### B10 — MINOR. "16384 … has never been a census's" is contradicted by five `trigger_census` outputs at `--cap 16384`.

R30: `wp20b_cap_out_{corpus,trigger-rich}_on_16384.txt`,
`wp20b_keypos_{corpus,trigger-rich}_16384.txt` and this document's own
`d3_c16384.txt`. §2 splits the same instrument's outputs into *"censuses"* and
*"cap measurements"* by intent, which is a fair distinction, but §5's sentence
then reads as a fact about the tree and is not one. **Fix:** *"…has never been a
census's at this seat"*, or name the four `wp20b_*` files.

### B11 — MINOR. M9's reporting limb is not carried, so §4's live claim will not be measured on the governed sample.

§4 asserts the arms are not equal-compute, and §5 normalises by seconds *because*
of it. §7 registers `w`, `s`, the ratio, firings, invocations, `att_proved` rows,
proving roots, `def_proved` keys, `key` vs `key_pos`, and `w` by turn count — and
no node totals. Round 2's M9 (and round 1's F3 before it) asked for mean and max
`search_nodes + solver_nodes` per arm; the entry lines already print both
counters, so this costs one column. **Fix:** add it to §7.

### B12 — MINOR. §6's "keys per proving position" column is headed *positions* while the text calls them *roots*.

The table header reads *"proving POSITIONS"* and *"keys per proving position"*;
the prose immediately below reads *"CLUSTERED IN A HANDFUL OF ROOT POSITIONS"*,
*"86 keys come from 8 roots"*, and §7 registers *"the count of PROVING ROOT
POSITIONS"*. They are the same quantity — a fixture entry — and the numbers are
right either way, but M7a was a finding about a mislabelled column and the label
is still two words in two places. **Fix:** one term.

### B13 — MINOR. §6's projected `w` figures are means of a very skewed distribution, printed without their spread.

R7: at cap 2048 the 86 keys are distributed over the 8 proving roots as
**35, 33, 11, 3, 1, 1, 1, 1** — 79 % from two roots, median 1. §6 multiplies the
mean 10.75 through as *"the measured clustering"*. R9 vindicates the arithmetic —
the bootstrap means at n = 500 are exactly 430 / 85 / 95 — but the 95 %
intervals are **[231, 657] / [53, 122] / [62, 133]**, so the printed figures
carry roughly ±45 % that the section does not show. Nothing rests on them: R10
shows *"far clear of §5's floor of 10"* follows from `w >= #proving roots`, which
is exact and needs no multiplier, so the CP lower bound alone gives `w >= 18`.
**Fix:** print the interval, or make the floor argument the exact one and let the
projections go.

---

## QUESTIONS — raised, not asserted

**Q1. Is one wall clock per arm enough for a 10 % band?** §5 answers M4c's method
question properly, and `docs/process.md`'s *Cost, replication* clause asks for
replication *where the run is cheap* — this run is 1.7 h. Nothing turns on it
here: R11's ratio gap is 6x, orders of magnitude outside any wall-clock noise.
Raised so a successor at a closer seat does not import *"one run per arm"* as
settled methodology.

**Q2. Has `distinct key` against `distinct key_pos` anything left to measure?**
R4-R6: the two are equal in all three arms (1038/1038, 356/356, 252/252), the
same zero symmetry-fold yield D-570 records and round 2 raised as Q-C. Harmless,
one column, possibly already answered — but it has now been measured to be zero
on three separate populations.

**Q3. Does anything in *this* document bind §B's census to this draw?** Round-2's
M2 asked for the consequence to be registered here. It is registered in the
sibling: `wp22_census_prereg.md` §3 names `draw_census_samples.py`'s census slice,
rows 600 onward, 89 205 positions. The risk is discharged, by a document this one
does not govern. Worth one clause here if the two are ever separated.

---

## What I attacked and it survived — said explicitly

**S1. §6's sizing table is exact.** I re-tallied all three census outputs with my
own parser rather than the receipt's `tally.py`, and **every one of the
twenty-one cells reproduces** — firings/position, `w`, keys/position, proving
roots, keys per proving root and seconds per position, at all three caps
(R4-R6). This is the half round 2 found broken and it is now right.

**S2. The draw is deterministic and is the receipt's.** Re-running
`draw_census_samples.py dryrun` produces `d3.txt` byte-identical to the receipt
(`7799f206…`), expected file likewise (R21). The census instrument is
deterministic to the row line: `d3_c2048.txt`'s first 20 entries are **all 236
row lines identical** to `dry2_moveorder_c2048.txt` (R22).

**S3. §9's honest run and its 500-row check both reproduce**, through my own
pipeline over my own draw: **0 of 100** and **0 of 500** (R16, R17). The
shift is the only one of the three that does not (B3).

**S4. The cap-independence claim is true, and is the right thing to say.**
`fixture_key_full` reads `std::env::args().nth(1)` and nothing else — no cap, no
search, no census in the pipeline (R37). M10 is properly discharged, and the
reason given is the correct one.

**S5. The join is genuinely verified, on both keys, with a named refusal.** I
read `main()` and re-ran it: the record must carry the manifest's `key_full`
**and** its `key_pos` or the draw raises `SystemExit`, and the comment says why
either alone is insufficient. The measurement that put it back reproduces
exactly — 229 vs 236 firings, 206 vs 210 distinct keys, 6 vs 8 loss-direction
keys (R23) — and D-615 states it in those terms. **M2 is the finding this
revision handled best**: it reversed a change made on a prior review's advice
because a measurement contradicted it, and it recorded why so a successor cannot
re-derive the shortcut.

**S6. The corpus is intact.** Manifest digest and **all sixteen** tranche digests
match `arc3_ledger.md` §5 (R1, R3). B6 is about the registration, not the data.

**S7. `trigger_census.rs` at `0f58533`, "unchanged at HEAD"** — empty `git diff`
both ways (R34). The one instrument revision that is properly named verifies,
for the second review running.

**S8. The cap binds at every rung.** Max `att_visits` and `def_visits` are
exactly 2048, 8192 and 16384 (R33), so each arm measures the cap it names.
F3's fatal reading stays gone.

**S9. §4's and §10.3's tree citations reproduce.** 1 in 361 and 4 in 370 at
`--nodes 50000 --cap 2048` (R31); six `wp20b_cap_out_*`, all at 400 000 (R28);
`pvs.rs:196-198` carries the doc line and the sum (R32).

**S10. The Clopper-Pearson arithmetic is right.** [0.0352, 0.1516] against the
printed [0.035, 0.151], and 17.6 roots at n = 500 against the printed ~18 (R8).
The interval was computed by exact binomial-tail bisection, not by a normal
approximation.

**S11. The floor conclusion is robust, and by a better argument than the one
printed.** `w >= #proving roots`, so the CP lower bound alone puts `w >= 17.6`
at every arm with no clustering multiplier; the bootstrap agrees at
`P(w < 10) = 0` in 20 000 replicates (R10). I attacked the sizing hard — the
clustering is far worse than §6 says (B13) — and the conclusion it draws still
holds.

**S12. The three slices are disjoint and reproducibly so.** `slice_of()`
implements §3's table exactly and neither the 99/100 nor the 599/600 boundary is
a hash tie (R20), so the new boundaries are as stable as revision 3's were.

**S13. M4b's pathological input is genuinely gone.** I re-ran round 2's own
counterexample against the new rule: with selectability tested per arm before the
ratio, the qualifying arm is no longer discarded. The floor fix is correct.

**S14. §6's replacement for M8's mechanism claim is true on the live sample.**
Per-firing win-proof rates 0.0834 / 0.0487 / 0.0749 — not monotone in the cap,
as §6 now says (R36).

**S15. Both mechanized citation gates pass on the document at HEAD** — 6
citations checked, 0 unreproduced; 0 revision citations stale (R40). Recorded
because D-617 is about that gate; and, as the gates' own output says, a green run
means the citations are real and nothing more.
