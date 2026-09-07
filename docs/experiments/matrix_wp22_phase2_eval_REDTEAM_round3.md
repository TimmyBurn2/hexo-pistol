# THIRD DECISION-RED-TEAM — `matrix_wp22_phase2_eval.md` revision 4

## 1. Header

**Named revision**: `c7a3ee1aa9910388107c05e2ec21099b42070674` (`dev`).
**Matches HEAD**: YES — `git rev-parse HEAD` returned
`c7a3ee1aa9910388107c05e2ec21099b42070674` at the start of this review and again
at the end. `git status --porcelain` was empty at both points: the working tree
did not move under me and I wrote exactly one file, this one. The concurrent
agent reviewing `hex_threat_enum_v1.md` had committed its round-4 revision
(`9ab757f`) before I began, so every quotation of that memo below is of the
committed revision 4 at this SHA.

**What this document is.** A confirmation by BEHAVIOUR under D-691. For every
finding of `matrix_wp22_phase2_eval_REDTEAM.md` (4 BLOCKING / 11 MAJOR / 10
MINOR at `0641504`) and `matrix_wp22_phase2_eval_REDTEAM_round2.md` (1 BLOCKING /
7 MAJOR / 5 MINOR at `564dc9e`, plus 10 of 10 round-1 MINORs still landing) I
re-ran the ATTACK against revision 4. I did not check whether a prescribed
sentence appeared. Where a prior report named a script, I wrote my own from the
receipted data rather than copying its arithmetic. Revision 4's new text is
unreviewed and is attacked on its own account in §3.

**Instruments, all read-only. Live tree unmodified, no `cargo`, no worktree, no
`CARGO_TARGET_DIR`.**

| instrument | what it did |
|---|---|
| `sha256sum -c` in all **eight** `artifacts/wp22_phase2a/*/` directories, then `sha256sum` of each receipt file | verified every `RECEIPT_*.sha256` file by file and took each receipt's own digest |
| `…/rt3/derive.py` (mine — its own parser for the census record grammar) | re-derived **all 16** registered-criterion ratios AND **all 16** nested-test ratios from `census_r4`, and cross-checked all 144 shared purity cells against `census_r3` |
| `…/rt3/sens2.py` (mine) | recomputed the nested test under worst-of-3, mean-of-3 and best-of-3 null summaries, and per replicate |
| an inline `python3` paired re-derivation over `seed_pilot/pilot_L11.txt`'s 24 rows | §3's wins, paired mean, sd, `t`, `mean/sd`, half-range/mean, sd/mean, SEM/mean |
| an inline `python3` nps model written from §1.8's stated arithmetic | every registered floor and ceiling in the field, and the `c` at which each fires |
| an inline `python3` closed form | §1.6's six-row expected/worst-case/openings table and the two cross-checks |
| `sed -n` at named lines of `crates/pistol-arena/src/{validate,error,config,schedule}.rs`, `crates/pistol-eval/src/{eval,handcrafted}.rs`, `crates/pistol-core/src/window.rs`, `docs/decisions.md`, `docs/audit/repo_audit_2026-09.md`, `docs/research/eval_families_2026-09.md`, `docs/book_v3_ledger.md`, `docs/experiments/hex_threat_enum_v1{,_CONFIRM}.md` | every code claim and every quotation read at the line |
| `/usr/bin/grep`, `git grep`, `git ls-files`, `git worktree list`, output `LC_ALL=C sort`ed where recorded (D-265) | residue checks, section pointers, instrument tracking, digest anchoring |

**Receipt verification.** All eight directories verify `sha256sum -c` clean, file
by file. **Seven receipt-file digests are the ones the matrix prints. One is
not**, and one directory the matrix depends on is printed nowhere:

| directory | receipt digest ON DISK | what the matrix prints |
|---|---|---|
| `census/` | `aab7f4f6a8d450fac5609dce4635a21ea86ec658e4c42c11c9d89b98835e14a8` | same ✓ |
| `census_r3/` | `9bb8fc6f2e0500bcf10f48e493dd9e3615897d935b73b63d8a8f47bf179e9c39` | same ✓ |
| **`census_r4/`** | **`aa88c298ac422a87df6505afcc33fcb1e2cb156783e81ec525b13dc4681bbdd1`** | **nothing — no row in §10, no mention anywhere** |
| `clockfix_confirm/` | `7678648c5dcf2418a5a0eb320f0faac3835c21ae792723742e737fac2345bb66` | same ✓ |
| **`n6_mutants/`** | **`36b4c999ba780800e40369cff20641db4982b05d3a3497bf84a88e4c40607d2e`** | **`d7e33861129a51bcd492a271dec468cc54056f2e384dd0076e785124102016d7` — matches nothing on disk** |
| `nps_seat/` | `211b6e1002f32ee89baace6ed4ff510302b2aad7817094e109dcac121d51ca9e` | same ✓ |
| `seed_pilot/` | `f3518ad221413a042a616d3c6348458afc089fba6f30382734c802540592c7bd` | same ✓ |
| `sprt_power/` | `d82fce37add53453423e264a6220e2b47636302c7f2980c4a60f6c600005709b` | same ✓ |

`git worktree list` shows only the main tree, as §10 claims.

**A record-keeping note, not a finding.** The matrix's banner calls revision 4
"ROUND 3 OF FOUR"; my dispatch calls this review round 3. Both counts agree here.
Under D-709 one round remains after this one.

---

## 2. Confirmation table

Verdicts: **CLOSED** = I ran the attack and the defect is gone. **CLOSED BUT
MOVED** = the named defect is gone and one of the same shape is at the new text.
**STILL LANDS** = the attack reproduces.

### Round 2's BLOCKING

| # | my attack | output | verdict |
|---|---|---|---|
| **NEW-1** | Re-derived §2.5's BOTH tables from the receipted census myself — all 16 criterion ratios from `census_r4` cross-checked against `census_r3`, and all 16 nested ratios from `census_r4`'s `JOIN`/`JOINNULL` rows — then asked whether the new "UNDECIDED" is what the evidence says. | My 16 criterion ratios: L7 1.014/1.014/0.793/0.686; L9 1.108/0.760/0.673/0.533; L11 1.125/0.651/0.637/0.510; L13 0.834/0.483/0.472/0.378 — **MET at 4, FAILS at 12**, and identical to `census_r3` on all 144 shared purity cells (0 differences). My 16 nested ratios reproduce the memo's ten displayed cells exactly and give **ADDS at 13 of 16**. So the row IS restored, the tables ARE faithfully transcribed, and revision 3's false conclusion is gone. **But the verdict "UNDECIDED" rests on ONE number, 0.93, and that number is the max of three null replicates — a summarisation rule that appears in neither §6.5's registration nor the matrix. Mean-of-3 gives 1.11 and two of the three replicates give 1.17 and 1.30.** | **CLOSED on the false conclusion; the replacement is a NEW BLOCKING — R3-B1.** |

### Round 2's MAJOR

| # | my attack | output | verdict |
|---|---|---|---|
| **NEW-2** (5 code-unit cells) | Recomputed all 16 code-unit ratios myself and compared against §2.5's list. | §2.5 prints all five — `L=7` T4/T3 (1.140), `L=9` T4 (1.62), `L=11` T4 (2.269), `L=13` T4 (1.636) — names `L=13` T4 as the one that flips the verdict between units, and states why none is evidence. Faithful. | **CLOSED.** |
| **NEW-3** (currency table misnames its own rows) | Did not read the table. Recomputed each row's density in the currency the table assigns it, then grepped for EVERY site that prints a density figure. | R-A1-L11 ✓ (`45 271/62 370 = 0.7259 → 0.73`), R-A2 ✓ (`3.197`), R-A3 ✓ (`16 613 729/38 983 = 426.2` and `1.16` positions). The source mislabel is correctly identified at `eval_families_2026-09.md:126-127`. **But three further sites still print the pre-correction figures under the pre-correction label**: line 574 (R-A1-L8F: *"Against A1-L11's 1.2 and A3's 1.2"*), line 817 (§7, R-A3: *"1.2 observations per real parameter"*), line 819 (§7, R-A1-L11: *"killed: 1.2 observations per parameter"* — the FOLDED figure quoted for the UNFOLDED row). §7 is the table round 2's M-6 already found the previous fix had missed. | **STILL LANDS at three sites — R3-M3.** |
| **NEW-4** (`book_v3`'s standing claim) | Read the ledger's *"Standing claims"* section and D-704 whole, then asked whether D-704's argument covers the row §6 ranks FIRST. | The ledger's claim is quoted correctly. D-704 says *"R7 is answered by the **selected family's** acceptance SPRT"*. R-H-EXT is not a corpus-learned family — §6 concedes it is *"arguably not a Phase-2 row… a package of hand terms is Stage-0 work"* — so an R-H-EXT acceptance run spends `book_v3` **without answering R7**. The argument that equalises book cost fails at exactly the row the equalisation promotes to rank 1. | **STILL LANDS — R3-M7.** |
| **NEW-5** (§6's axis does not answer its question) | Read §6's new paragraph and asked whether the disclosure is load-bearing or decorative. | §6 says it in terms: *"with one funded run, 'which row is run first' is 'which row is run'… an order of readiness, not of expected value"*, and puts the question it cannot answer first in the HANDUP. This is the strongest new paragraph in revision 4. | **CLOSED.** |
| **NEW-6** (§12 claims two fixes that did not happen) | Compared §1.2 and §1.3 against `nps_instrument.txt` and `nps_play.txt` digit by digit, then read §12's rows. | `nps_instrument.txt` holds `529255 (526479, 531293)`, `564304`, `492073`, `453012 / 73388 nodes / 162 ms` — §1.2 is the receipt's, cell for cell. `nps_play.txt` holds `171851/171625/172499` and `482274/481548/482127`; §1.3 prints `172 627/172 665/172 211` and `478 718/480 449/482 105`, none of which is in the receipt — and §1.3 AND §12 both now say so and compute nothing from them. | **CLOSED.** |
| **NEW-7 / M-6** (§7 retains the axis §6 disavows) | Read §7's heading and its R-H-EXT cell. | §7's column is now *"**BUILD cost** (§6's axis; book cost is the field's common one)"* and its R-H-EXT cell carries NEW-8's finding verbatim. | **CLOSED at §7 — MOVED to §6, see R3-M4.** |
| **NEW-8** (R-H-EXT's build cost priced at nil) | Grepped every site that prices R-H-EXT's build cost, rather than reading the one §13 names. | Fixed at line 643 (§4) and line 813 (§7). **Not fixed at either §6 site**: line 741 *"**R-H-EXT.** Nothing to build: no fit, no artifact, no digest discipline, no shape check, no seed, no quantization, no new dependency"*; line 790 *"its book cost is the field's common cost and **its build cost is nil**"*. §6 is the section whose entire ranking axis IS build cost. §13's NEW-8 row reports the fix as landed. | **STILL LANDS at §6 — R3-M4.** |
| **NEW-13** (R-H-EXT's abort threshold cannot fire) | Re-implemented the nps model from §1.8's arithmetic and solved for the `c` at which each registered floor fires. | Every floor and ceiling reproduces exactly: L11 `[296 959, 549 733]`, L8 `[365 760, 621 921]`, L7 `[396 371, 650 390]`, R-H-EXT `[365 760, 529 255]`. `529 255 × 0.95 = 502 792` ✓, and it fires at `c > 1.1177` — an 11.8 % per-touch increase, which 3–6 added integer terms can plainly cause. **The floor can fire.** | **CLOSED — but the fix created a new false claim, R3-M5.** |
| **NEW-9** (§1.8's reason for 44.70 % is false at the code) | Read the code, not the sentence. | `crates/pistol-eval/src/handcrafted.rs:302` — `HandcraftedV0` **does** override `fn delta`, and its body reads `self.windows.get(…)` and `self.contribution(…)` and calls neither `apply` nor `undo`. `crates/pistol-eval/src/eval.rs:87`'s default IS the apply/undo roundtrip. `repo_audit_2026-09.md` A-02 gives `31.77 + 6.64 + 6.29 = 44.70` with `13.45% ThreatState::touch` next. Revision 4's replacement reason is correct at the code, and its parenthetical says the number was right and the reason was wrong. | **CLOSED.** |
| **NEW-10, NEW-11, NEW-12** | Grepped for the 1.77x cross-reference; read §1.6's column header; recomputed all four of §3's loosenesses from the receipt. | 1.77x is now attributed to `hex_threat_enum_v1.md` §6.5 and the WITHDRAWN criterion ✓. The header reads *"pairs the run is capped at"* ✓. §3 names the winning side, marks `p` one-sided (and gives the two-sided 0.0078), marks 5.25 an effect size, and marks the sizes→architectures step as a step ✓. My paired re-derivation is identical to the digit: `8/8, 23472.2, 5167.6, t=12.85`; `8/8, 36878.5, 7020.7, t=14.86`; `8/8, 13406.3, 3145.8, t=12.05`; `mean/sd = 5.25`. | **CLOSED (all three).** |

### Round 1's ten MINORs — §13 closes all ten in one row. Checked independently.

| # | my attack | output | verdict |
|---|---|---|---|
| **m-1** | `/usr/bin/grep -n '^#\{2,3\} '` for the section list; grepped every `§11`. | The pointer is fixed — line 34 now reads *"§8 is the failed-precedent check"* — and `§11` survives only inside §13's own history row. **But the numbering hole is untouched**: the sections run §1…§10, then §12, §13. There is no §11. | **CLOSED on the pointer; the hole STILL LANDS — R3-m4.** |
| **m-2** | Grepped every `§0.x` / `§6.x` / `§7.x` / `§A[0-9]` reference and checked each names its document. | All named. Line 842's formerly-bare reference now reads *"`hex_threat_enum_v1.md` §6.5's purity criterion"*. | **CLOSED.** |
| **m-3** | Recomputed all nine spread statistics from `pilot_L11.txt` myself. | half-range/mean **2.790 / 3.457 / 3.458 %**; sd/mean **2.00 / 2.34 / 2.25 %**; SEM/mean **0.71 / 0.83 / 0.80 %** — every figure §3 now prints, and the arithmetic (`half-range / mean`) is named on the page. | **CLOSED.** |
| **m-4** | `/usr/bin/grep -l 'mode = "play"' configs/*.toml`, then checked whether §1.7 prices the third seat. | Three files carry `mode = "play"`, and §1.3 now says *"**THREE committed configs**"* and names all three with the reason the third is excluded ✓. **Residue**: §1.7's play-seat wall still combines a per-answer figure MEASURED on `play_v0` (479 ms) with an answers-per-game figure from `sealbot_anchor_v7_protocol.md`, whose runs `sealbot_anchor_v3_prereg.md:40` puts on `configs/play_staged_v0.toml` — the other seat, 2.8x away in throughput and depth. | **CLOSED on the count; the cross-seat estimate STILL LANDS — R3-m6.** |
| **m-5** | Read §1.7 whole and re-did both divisions. | Reconciled: *"The book is sized for the worst case and the run pays the mean"*, with `book_v3_registration.md` §R1's `mean_pairs 3102.1` cited and `3102.1 × 2.046 s = 1.763 h` ✓ against `8500 × 2.046 = 4.831 h` ✓. | **CLOSED.** |
| **m-6** | Compared §1.7's per-answer term with `nps_play.txt`. | `11 502 ms / 24 = 479.25` and `10 978 / 24 = 457.4`; §1.7 now uses 479 ms and says *"not the 500 configured"*. Its pair arithmetic checks: `41.6 answers × 0.479 s = 19.93 s`; `500 × 19.93 = 2.77 h`; `1000 × 19.93 = 5.54 h` ✓. | **CLOSED.** |
| **m-7** | Recomputed the two disputed rows of the arithmetic pass. | Now `7.118x` and `34.38 h` at the corrected 60 500, with the correction disclosed in the header ✓. `60500/8500 = 7.1176`; `60500 × 2.046 = 34.38 h` ✓. | **CLOSED.** |
| **m-8** | Compared the `L=7` rungs — both the enum partitions and the JOIN partitions — in `census_r4`. | §2.5's criterion paragraph now carries *"(Two of those four are one partition… the sixteen cells are fifteen distinct partitions.)"* ✓. **But the defect recurs one paragraph down**: at `L=7` all four rungs share the SAME join partition — `JOIN count x T4/T3/T2/T1` are each 36 classes with `ω² = 0.004167` — so the nested test's "16 cells" are at most **13 distinct joins**, and the identical partition is scored *"adds nothing"* at T4/T3 and *"adds"* at T2/T1 purely because the null was matched to a different class count. §2.5's *"13 of 16"* carries no such qualification. | **CLOSED at the first site; RECURS at the second — R3-m2.** |
| **m-9** | `git ls-files \| grep -E 'measure_nps\|measure_play\|confirm_wedge'`. | Returns nothing. §8's row now says so in bold — *"THREE OF THEM ARE UNTRACKED… which matters most for `measure_nps.sh` because every registered nps floor is recomputed from its output"* — and withdraws the false claim. The `docs/process.md` coverage violation itself is unchanged: `measure_nps.sh` still produces 529 255, from which every floor derives, with no test driving the shipped script. | **The false claim is CLOSED; the violation is DISCLOSED, not fixed — R3-m10.** |
| **m-10** | Grepped for D-653's flip clause and for the *"quoted whole"* claim. | The flip clause is now named at line 848, and the *"quoted whole"* claim is withdrawn there. **But the same cell then re-asserts it**: *"The claim that both are quoted whole is withdrawn. **D-653 and D-705 — are quoted whole rather than glossed**"* — the residue of the original sentence, contradicting the withdrawal one clause earlier. And D-653's flip is still not connected at §6, where *"a second acceptance book moves R-A2 up"* fires it. | **CLOSED BUT MOVED — the remedy left a self-contradicting cell, R3-m3.** |

### Round 1's BLOCKING and MAJOR — spot-checked, not re-run in full

Round 2 confirmed these at their own claim and I re-ran the ones revision 4
touched. **B-1** ✓ (D-705's asymmetry clause is quoted verbatim at line 1476 of
`docs/decisions.md` and §5 reports §1.6 as the ADR's premise measured). **B-2** ✓
(my own paired re-derivation above). **B-3** ✓ (`config.rs:56` *"each opening is
exactly one pair"*, `schedule.rs:37` `let total = openings.taken.len() * 2;`;
every power cell reproduces from `sprt_power_two_arm.txt` and `screen_power.txt`).
**M-1, M-2, M-3, M-9, M-10, M-11** ✓ as recorded above and in §4. **M-4** — see
R3-M3. **M-5** ✓ (§5 separates the bench quantity from the arm; §6 ranks on the
first). **M-6, M-7** — the deleted claims survive only as quotations of their own
deletion; but see R3-M4 and R3-M7.

**Score.** Round 2's BLOCKING: closed on its stated claim, replaced by a new one.
Round 2's seven MAJOR: **four CLOSED, three STILL LAND** (NEW-3, NEW-4, NEW-8).
Round 2's five MINOR: all five CLOSED. Round 1's ten MINOR: **six CLOSED, one
closed-but-moved, three still land in part** (m-1's hole, m-4's cross-seat
estimate, m-8's recurrence, m-9's underlying violation, m-10's contradiction).

---

## 3. NEW findings against revision 4

### R3-B1 (BLOCKING). The one number that makes the row UNDECIDED and puts it at rank 4 is the WORST of three null replicates, printed as if it were one null — and the conclusion reverses under any other summary of the same three.

§2.5's whole convergence argument, and with it the word UNDECIDED and the rank,
turns on one cell:

> The one cell that is comfortably affordable **and** passes the registered
> criterion is **`L = 7` T4**… **The nested test says the enum adds nothing
> there — 0.93, below its own null.**

and

> **No cell is simultaneously affordable and supported by both measurements.**

**Reproducer** (from `artifacts/wp22_phase2a/census_r4/census_L7.txt`, receipt
verified):

```
L=7 T4 increment over COUNT-ONLY      +0.000642
null replicates r0 / r1 / r2          +0.000547  +0.000494  +0.000690   (max/min = 1.40x)
ratio vs WORST-of-3   0.93  -> "adds nothing"   <-- what the matrix prints
ratio vs MEAN-of-3    1.11  -> "adds"
ratio per replicate   1.17   1.30   0.93        (two of three say "adds")
```

Four things follow and each is independently damaging.

1. **The matrix does not disclose the summary at all.** It says *"below its own
   null"* — singular. The enum memo's §7.4b does say *"worst of three
   replicates"*; the matrix drops it. A reader of the matrix cannot know that
   0.93 is a maximum over three draws, that the spread between those draws is
   **1.40x — the widest of any of the sixteen cells** — or that two of the three
   point the other way.
2. **The summarisation rule is not registered.** §6.5 states the nested question
   as *"does `ω²(count × class) − ω²(count)` exceed `ω²(count × permuted class) −
   ω²(count)`"* — one referent, no extremum. "Worst of three" is a choice made
   in §7.4b, after the statistic was built, and it is the choice that decides the
   cell.
3. **Under mean-of-3 the matrix's headline is false.** I recomputed all sixteen
   cells: mean-of-3 gives **ADDS at 16 of 16**. `L = 7` T4 is then affordable
   (1 720 observations per nominal parameter — I checked: `5 031 327 / 2 925 =
   1 720.1`), MET on the registered criterion (1.014), and supported by the
   nested test (1.11). *"No cell is simultaneously affordable and supported by
   both"* is then simply untrue, the row is not UNDECIDED, and rank 4 loses its
   stated ground.
4. **The matrix uses a statistic its own source forbids using this way.** §6.5:
   *"So it is NOT registered as a criterion, it does not rescue anything, and
   §7.4 reports it as a finding beside the registered verdict rather than in
   place of it."* §6 rank 4: *"It ranks below R-A1-L8F **because** §2.5's nested
   test says the enum adds nothing over stone counts at exactly this cell."*
   That is the non-criterion used as a demotion criterion. §6.5 names the defect
   class in its own words — *"A statistic introduced once a registered one has
   returned an unwelcome answer is exactly the post-hoc move this project's
   process exists to forbid"* — and this is that move mirrored: the registered
   criterion returned a WELCOME answer at `L = 7` T4 and an unregistered one is
   used to take it back.

Computing the mean-of-3 takes seconds from the receipted census. Not computing
it, at the number the row's whole standing rests on, is D-291's own class.

**What would close it.** Print the three replicates and both summaries, state
which is registered, and let the row's verdict follow from the registered one —
or state that the nested test cannot rank the row at all, which is what §6.5
says.

### R3-B2 (BLOCKING). Every nested number comes from `census_r4`, which no tracked document anchors by digest — and §10 asserts it is that anchor.

§10's closing sentence:

> `artifacts/` is gitignored (hard rule 8), so an artifact directory is evidence
> only while a tracked document carries its receipt's own digest. **This table is
> that anchor.**

**Reproducer**:

```
$ sha256sum artifacts/wp22_phase2a/census_r4/RECEIPT_census_r4.sha256
aa88c298ac422a87df6505afcc33fcb1e2cb156783e81ec525b13dc4681bbdd1
$ git grep -l 'aa88c298' -- '*.md' | wc -l
0
$ /usr/bin/grep -c 'JOIN' artifacts/wp22_phase2a/census_r3/census_L7.txt   # the ONLY anchored census
0
$ /usr/bin/grep -c 'JOIN' artifacts/wp22_phase2a/census_r4/census_L7.txt
20
```

`census_r3` — the directory §2 and §10 anchor, digest `9bb8fc6f…` — contains
**zero** `JOIN` or `JOINNULL` rows. The nested test is computable only from
`census_r4`. The enum memo anchors `census_r3` at its line 803-805 and anchors
`census_r4` nowhere. §10's table has no `census_r4` row.

So the number that produced revision 4's BLOCKING fix, the word UNDECIDED, and
rank 4 rests on an artifact directory that, by the matrix's own stated rule, is
not evidence. This is MAJOR-2's defect class — a receipt that anchors nothing —
in its most consequential possible location.

### R3-B3 (BLOCKING). The field is STILL incomplete, and the missing row dominates rank 3 on both of the matrix's own axes — while rank 3's cell claims a superlative that row refutes.

Round 1's MAJOR-11 found the field missing `R-A1-L8F`: `eval_families` §0.2
measured a length-8 folded census, the matrix quoted it, and priced no row
against it. **The identical argument applies at `L = 7`, with more force, and
revision 4 does not make it.**

`eval_families_2026-09.md:53` (§0.2, the same table `R-A1-L8F` is priced from):

| L | fold | cells | median obs/cell | cells < 10 obs | covering 90 % | ceiling | % of ceiling |
|---|---|---|---|---|---|---|---|
| 7 | **yes** | **1 029** | **1 282** | **0 (0.0 %)** | **190** | 1 133 | 90.8 % |
| 8 | **yes** | 2 920 | 273 | 24 (0.8 %) | 293 | 3 320 | 88.0 % |

A free folded table at `L = 7` beats the row ranked 3 on **every column the
matrix prints for it**, and on both axes §6 ranks on:

| | R-A1-L7F (absent) | R-A1-L8F (rank 3) |
|---|---|---|
| parameters | **1 029** | 2 920 |
| median observations per cell | **1 282** | 273 |
| cells under ten observations | **0** | 24 |
| touches per stone (`3L`) | **21 (1.17x)** | 24 (1.33x) |
| nps floor (my model, §1.8's arithmetic) | **396 371 (0.75x)** | 365 760 (0.69x) |
| its kill | below §0.1's covering minimum — the SAME unrun theorem | below §0.1's covering minimum |

**The matrix knows the numbers.** Its own R-A3 cell (line 601) prints them:
*"The factor — **1 029 folded L7 cells, none under ten observations**"*. It
prices `L = 7` as a length elsewhere — rank 4 is `R-A4-CLASS` **at `L = 7`**.
`L = 7` is odd, so the enum memo §3.1's even-length refusal does not touch it;
`eval_families` §0.2 measures it directly. Nothing in the field excludes it. It
is simply not priced as a row.

**And the omission produces a false superlative.** Line 574, R-A1-L8F's own
density cell:

> …this is **the densest free table the corpus supports**.

That is false against the matrix's own line 601. It is a claim the document's
adjacent row refutes — the precise defect class §8's table credits this matrix
with not repeating (*"2 M-3 / 3 MAJOR-1 … a uniqueness claim the document's own
adjacent column refuted"*).

`L = 9` folded (8 374 cells, median 56, 12.9 % under ten) is a fourth measured
length in the same §0.2 table, also unpriced. The free-folded-table family has
four measured lengths and the field prices two.

**This reorders the matrix.** A row that is denser than rank 3, cheaper in
traffic than rank 3, carries a higher floor than rank 3, and shares rank 3's only
kill, ranks at or above rank 3.

### R3-M1 (MAJOR). §10's `n6_mutants` digest matches nothing on disk — MAJOR-2's defect class, recurring.

```
$ sha256sum artifacts/wp22_phase2a/n6_mutants/RECEIPT_n6_mutants.sha256
36b4c999ba780800e40369cff20641db4982b05d3a3497bf84a88e4c40607d2e
$ /usr/bin/grep -n 'n6_mutants' docs/experiments/matrix_wp22_phase2_eval.md
878:… `artifacts/wp22_phase2a/n6_mutants/`, `d7e33861129a51bcd492a271dec468cc54056f2e384dd0076e785124102016d7`
```

The directory gained `referent_mutants.txt` and `run_referent_mutants.sh` (the
enum memo's round-4 referent mutants) after the digest was taken, so the receipt
file changed and §10 was not updated. Round 1's MAJOR-2 was exactly this — a
digest printed after the directory had moved — and round 2 confirmed it closed
at `sprt_power/`. It has reappeared one row away, in the same table, in the same
revision that added the round-4 work.

The row's prose is stale too: *"four mutants of `report.py` … all four DEAD"*, of
a directory that now holds **eight** (four `report.py` mutants and four referent
mutants, all DEAD).

### R3-M2 (MAJOR). §6's rank-4-versus-rank-5 separation is a cross-currency comparison, and §4 promises in terms that the ranking contains none.

§4, immediately after the currency table:

> …a comparison across the column is not a comparison. Each cell below names its
> own currency; **the ranking in §6 does not rest on any cross-currency
> comparison**, and where two rows are separated it is stated on what.

§6, rank 4, separating R-A4-CLASS from R-A3:

> …and **above R-A3 because its density is three orders of magnitude better**.

**Reproducer**:

```
A4 at L=7 T4 : 1 720   scored-cell observations per nominal parameter (§4's currency for A4)
A3           :   426   window observations per parameter  (§4's currency for A3)
             :  1.16   POSITIONS per parameter            (the other figure A3's cell prints)

1720 / 426  =    4.0x   -> under one order of magnitude
1720 / 1.16 = 1 483x    -> three orders of magnitude
```

*"Three orders of magnitude"* is true only if A4's scored-cell currency is set
against A3's positions currency — the exact comparison §4 says is not a
comparison. In the nearest COMMON currency the gap is smaller still: at `L = 7`
the window population is 8 241 249 (`census_r4`, window unit `n`), so A4-L7-T4 is
`8 241 249 / 2 925 = 2 817` window observations per nominal parameter against
A3's 426 — **6.6x, well under one order of magnitude**.

The same defect is at §2 item 2, in the sentence that kills T4 and T3 at the
covering length: *"1.07 per nominal parameter, against `eval_families` §A1's kill
of `R-A1-L11` at 1.2"* — 1.07 is scored-cell observations, 1.2 is positions, and
the two populations differ by a factor of 180 (`8 174 025 / 45 271 = 180.6`, the
matrix's own figure). The near-equality of 1.07 and 1.2 reads as a like-for-like
tie and is not one.

### R3-M3 (MAJOR). NEW-3's fix landed at two sites and three still print the corrected figures under the corrected-away label — two of them in §7, the table round 2 already caught once.

```
574: | observations per parameter (MEASURED) | median 273 … Against A1-L11's 1.2 and A3's 1.2, this is
     the densest free table the corpus supports. |
817: | R-A3-L11F+F7 | … 1.2 observations per real parameter is the density that killed A1 |
819: | R-A1-L11 | — | killed: 1.2 observations per parameter, and unfolded it learns twelve answers |
```

Revision 4 established that R-A1-L11's density is **0.73 positions per parameter**
unfolded, that A3's is **426 window observations per parameter**, and that
*"observations per parameter"* was the mislabel it corrected at the source. Line
819 then quotes the FOLDED figure (1.16 → "1.2") for the UNFOLDED row under the
label just deleted. Line 817 gives A3 the same. Line 574 uses both, in the
sentence that carries R3-B3's false superlative, and compares them against a
MEDIAN at a third unit.

D-424 part 3 names this exactly: *"the second site is what gets missed"*. Round 2
found the previous fix had missed §7. Revision 4 missed §7 again.

### R3-M4 (MAJOR). NEW-8's fix did not land in §6, the section whose entire axis is build cost — and §13 reports it as landed.

Both §6 sites are unchanged:

```
741: 1. **R-H-EXT.** Nothing to build: no fit, no artifact, no digest discipline, no
     shape check, no seed, no quantization, no new dependency…
790: its book cost is the field's common cost and **its build cost is nil**.
```

against §4's line 643 and §7's line 813, which both now say *"3 to 6 integers
with **no procedure and no metric** chooses their VALUES… It is a real cost and
it is not nil."*

§6 is where the ranking happens and build cost is its declared axis. A finding
about build cost that is fixed in the cost cells and left standing in the ranking
section is fixed in the two places that do not decide anything.

§13's NEW-8 row reads *"corrected: 3 to 6 integers with no procedure and no
metric for their values"*. That is a claim of a fix that did not happen — the
same class round 2 raised as NEW-6 against §12.

### R3-M5 (MAJOR). NEW-13's remedy made *"the highest floor in the field"* false, at two sites, and made the floor column heterogeneous while §5 and §6 read it as homogeneous.

The registered floors after revision 4:

| row | floor | is it the bracket's bottom? |
|---|---|---|
| R-A4-CLASS `L=11`, R-A5, R-A2, R-A3 | 296 959 (0.56x) | yes |
| R-A1-L8F | 365 760 (0.69x) | yes |
| R-A4-CLASS `L=7` | 396 371 (0.75x) | yes |
| **R-H-EXT** | **502 792 (0.95x)** | **no — its bracket bottom is 365 760 (0.69x)** |

§5's table (line 669) and §6 rank 4 (line 761) both call **396 371** *"the highest
floor in the field"*. §5's own table prints **502 792** seven rows below it.

Two problems, not one:

1. **The superlative is simply false**, and refuted by the same table.
2. **The column no longer measures one thing.** Five rows register their
   bracket's lower end; R-H-EXT registers a decision point 0.26x above its
   bracket's lower end, because NEW-13's fix deliberately decoupled the two. §6
   then reads a high floor as *"the least throughput risk in the field"* — but
   for R-H-EXT a high floor is a STRICTER abort, not a safer row. This is a fifth
   currency, introduced by a remedy, in a document that has a currency table.

### R3-M6 (MAJOR). The `t`-merge numbers are wrong in three linked ways, and §4 and §7 say the repair price is one *"nobody has quoted"* while quoting it.

`hex_threat_enum_v1.md` §5.5 at HEAD prints **two** figures with their
populations named:

> differs within **27 of the 335 classes** those patterns present, covering
> **5 348 of the 57 996 patterns**. **Without the rule-4 exclusion it is 32
> classes of 357 and 6 099 of 59 049**, and both numbers are printed here…

1. **§2.5 pairs them across populations.** It states *"**27 of the 357 classes,
   5 348 of 59 049 patterns**"* — 27 (with-exclusion) against 357
   (without-exclusion), 5 348 (with-exclusion) against 59 049
   (without-exclusion). Neither source supports that pairing.
2. **§4's R-A4 cell misrepresents the memo it cites.** It says the confirmation
   measured 32 / 6 099 *"**not the 27 / 5 348 it published**"*. Revision 4 of the
   memo publishes BOTH, in the same sentence, with the reason. §2.5 and §4 then
   give different counts for the same defect, ten lines apart, unreconciled.
3. **§4's repair price contradicts the memo's own repair table, unflagged.** §4:
   *"adding single-axis `t` as a seventh component removes the merge entirely for
   36 extra classes (`k` 357 → **393**, parameters **+33 %**)"* — the
   confirmation's figure (`_CONFIRM.md:229-230`). The memo's §5.5 table at HEAD:
   `T4 | 357 | **384** | 7 647 059 | **9 511 040**` — **+24.4 %**. Both are
   internally consistent (`C(395,3) = 10 193 765` gives +33 %; `C(386,3) =
   9 511 040` gives +24 %), and they cannot both be the repair's size. The matrix
   quotes one and cites the other as its authority.
4. **And the price is quoted in the sentence that says it is not.** §4:
   *"repairable at a price nobody has quoted"*; §7: *"repairable at a price
   nobody has quoted (**T2: `k` 47 → 58**)"* — which is the memo's own table row
   `T2 | 47 | 58 | 18 424 | 34 220`, i.e. the quoted price, in the parenthesis
   that denies it exists.

### R3-M7 (MAJOR). NEW-4's answer does not cover the row it promotes to rank 1, and book cost is what §6 had to equalise to reach its axis at all.

The chain §6 depends on:

> **Book cost is now EQUAL across every live row** — one `book_v3` arm — so it
> cannot rank them, and what remains is build cost, on which the rows genuinely
> differ.

resting on §1.5's:

> What makes that survivable is that R7's question — *does this corpus move Elo
> at all* — is answered by whichever family's acceptance SPRT runs (D-704), so
> the claim and the rows are the same run.

D-704 at `docs/decisions.md:1474` says *"R7 is answered by the **selected
family's** acceptance SPRT"*, and defines the question as *"does this corpus move
Elo at all"*. **R-H-EXT reads no corpus.** Its terms are hand-set integers from
`threat_calculus_v1.md`. An R-H-EXT acceptance SPRT answers nothing about the
corpus, so it spends the 8 000 pairs `book_v3` exists for (`book_v3_ledger.md`:
*"The R7 acceptance SPRT | **THE REASON THIS BOOK EXISTS**"*) and leaves R7's
standing claim undischarged.

§6 itself concedes the premise: *"R-H-EXT is arguably not a Phase-2 row. Phase
2's field is the QUIET-STRUCTURE LEARNED family, and a package of hand terms is
Stage-0 work nobody did."* The matrix therefore holds simultaneously that
R-H-EXT is probably not a learned family and that book cost is equal BECAUSE
every row's acceptance run is the learned family's R7 run. The equalisation
fails at exactly the row it promotes, and the equalisation is what licenses the
build-cost axis that promotes it.

This is round 1's QUESTION 4 and round 2's NEW-4, and §13's answer (*"rests on
D-704's argument"*) does not reach it.

### R3-M8 (MAJOR). `matrix_wp22_phase2_eval_ARITHMETIC.md` is headed "Governing revision: matrix revision 4" and checks no revision-4 number.

```
$ /usr/bin/grep -n '502 792\|396 371\|1 720\|nested\|0\.73\|13 of 16' \
    docs/experiments/matrix_wp22_phase2_eval_ARITHMETIC.md
(no output)
```

Its three check tables are "Revision 1's mismatch", "The 28 that reproduced" and
"Revision 3's checks, after the red team". The revision-3 table still verifies
*"R-H-EXT floor / ceiling | 365 760.2 / 529 255.0"* — a floor revision 4 replaced
with 502 792. The file's only revision-4 edit is the two SPSA rows round 2
caught. So the document that exists to show the author checked the arithmetic
governs a revision whose new arithmetic it does not touch: the nested ratios, the
0.95x floor, 396 371, 1 720, and 0.73 are all unchecked by it.

### MINOR

**R3-m1.** §2.5: *"the enum **ADDS at 13 of 16 cells**, by 1.26x to 1.76x at `L ≥
9`"*. I computed all sixteen: the count of 13 is right, but the range is over the
memo's ten displayed rows, not sixteen. At `L ≥ 9` the adding ratios run
**1.11x** (`L=13` T1) to 1.76x. A count stated over sixteen and a range stated
over ten, in one sentence, with no marker.

**R3-m2.** m-8's recurrence — see the confirmation table. At `L = 7` the four
JOIN partitions are one partition (36 classes, `ω² = 0.004167` at every rung), so
the nested test's sixteen cells are thirteen distinct joins, and the SAME
partition scores 0.93 *"adds nothing"* and 1.01 *"adds"* depending only on which
null it was matched against.

**R3-m3.** §8's m-10 cell contradicts itself in adjacent clauses: *"The claim
that both are quoted whole is withdrawn. D-653 and D-705 — are quoted whole
rather than glossed"*. The remedy was inserted into the middle of the old
sentence and the old sentence's tail was left.

**R3-m4.** The section numbering hole survives: §1…§10, then §12, §13. No §11.

**R3-m5.** §1.6: *"the second gives **5 233**, which is fishtest's `T =
1046535/Δ²` **exactly**"*. `1046535/100 = 10 465.35`. The identity holds only
after halving (fishtest's `T` is in GAMES, 5 233 is PAIRS) — and the author's own
`_ARITHMETIC.md` carries the `/2` explicitly. The matrix drops it and keeps the
word "exactly", in a sentence whose whole purpose is an external cross-check.

**R3-m6.** §1.7's play-seat wall mixes seats — 479 ms measured on `play_v0`,
10.4 answers per game from `sealbot_anchor_v7_protocol.md` whose runs
`sealbot_anchor_v3_prereg.md:40` puts on `play_staged_v0` — in a document whose
§1.3 exists to say those two seats differ by 2.8x in throughput and depth. Game
length depends on the seat.

**R3-m7.** §1.6 item 2 calls 8 % *"the veto's failure at truth = 0"*; §5 says
*"what is its power to FIRE? §1.6 has half of it"* and that the question *"is
unasked here and is owed by whoever runs the arm"*. Under D-705's own description
of the veto — *"it can only refuse a row whose accuracy gain is paid for entirely
in speed"* — the same receipt line (`cap 1000 pairs, truth 0: h0 18344 (0.9172)`)
is a **91.7 % refusal rate on a row that paid nothing in speed**. The document
answers the question in §1.6 and disclaims it in §5, and the label encodes the
reading without arguing for it. This is round 1's QUESTION 1, unresolved.

**R3-m8.** The currency table still omits the unit §6 rank 2 ranks on. It lists
one window unit — *"one `(axis, start)` window… 16 613 729 of them at `L = 11`"* —
while R-A5's cell and §6 rank 2 both use the CELL-CENTRED window population
(**12 980 519** over 11 909 codes, `census_r4` window `raw folded`). Round 2's
NEW-3 named this omission; it is unchanged.

**R3-m9.** §10's mutant row describes four mutants of a directory that holds
eight (see R3-M1).

**R3-m10.** m-9's underlying `docs/process.md` coverage violation is disclosed
and unfixed: `measure_nps.sh`, `measure_play.sh` and `confirm_wedge.sh` are
untracked with no test driving the shipped script, and `measure_nps.sh` still
produces the single figure every registered nps floor in §4 and §5 is computed
from.

---

## 4. What I attacked and it SURVIVED

I want to be precise, because a great deal of revision 4 is right and several of
its corrections are better than the findings that prompted them.

**Every receipt verifies, and the census replicates across rounds.** All eight
directories pass `sha256sum -c` file by file. I re-derived all 144 purity cells
that `census_r3` and `census_r4` share and found **0 differences** — three
independent census runs (round 2, round 3, round 4) agreeing cell for cell on
every quantity they share is a strong result and the matrix is entitled to lean
on it.

**§2's and §2.5's transcriptions are exact.** All sixteen registered-criterion
ratios reproduce to the printed digit from my own parser. So do all ten nested
cells the memo displays. `L = 13` sees 232 T4 classes against 231 and 108 075
codes against 108 074 ✓. T2 at `L = 11`: 18 424 nominal, 1 533 observed codes,
median 41, 322 at `≤ 4`, 620 below `< 20` ✓. Every `ω²` is lower at 13 than at 11
✓. The `L = 7` T4 affordability figure (1 720) and its 856 codes at median 174 ✓.

**§3 is exact and its paired re-analysis is sound.** My independent parse of
`pilot_L11.txt` gives `8/8` on all three comparisons, paired means 23 472.2 /
36 878.5 / 13 406.3, sd 5 167.6 / 7 020.7 / 3 145.8, `t` 12.85 / 14.86 / 12.05,
`mean/sd` 5.25 for the widest — identical to the digit. The half-range, sd and
SEM statistics are all correct, and revision 4's own caveat that the sizes→
architectures step is a step (not a measurement) is the right call.

**§1.2's throughputs are the receipt's, and the replication is honest.** Every
cell of §1.2 is in `nps_instrument.txt`; 531 548 is correctly reported as an
unreceipted replicate 0.433 % away.

**Every code claim I checked is true at the line.** `validate.rs:45-51`'s refusal
and `error.rs:164-172`'s message ✓. `config.rs:56` *"each opening is exactly one
pair"* and `schedule.rs:37` ✓ — the openings-equal-pairs premise round 1
questioned is real. `handcrafted.rs:302`'s `delta` override, read-only, calling
neither `apply` nor `undo` ✓. `window.rs:9`'s `WINDOWS_PER_CELL = 18` ✓. A-02's
`31.77 + 6.64 + 6.29 = 44.70` ✓.

**Every power figure is the receipt's and my closed form agrees.** My independent
computation of `2·ln(19)/t1²` and `ln(19)²/t1²` gives 3 554.3 / 5 232.7 at Δ=10
and reproduces all six rows of §1.6's table including the `ceil_to_500` column.
Every `h1`/`h0` cell is in `sprt_power_two_arm.txt` or `screen_power.txt`.

**Every nps floor and ceiling reproduces exactly** from §1.8's stated arithmetic,
at all four traffic ratios. The `c ∈ [1.0, 2.0]` ground for R-H-EXT (*"adding
hand terms cannot"* be cheaper per touch) is correct reasoning, and the 0.95x
floor genuinely can fire (at `c > 1.118`).

**§6.4's twenty clips reproduce §2's summary faithfully** — five give `k = 16`
and 816, the coarsest gives `k = 9` and 165, the shipped clip is the
second-finest — and the qualified *"no SHIPPED cell of it is 816"* is true.

**§1.4 is right and is the most useful finding in the document.** The arena
refuses a movetime budget by name, so D-705's time-matched arm is a capability
nothing in this repository has; it is the same for every row; and §5's
observation that a constant column cannot rank a field is correct and correctly
scoped.

**§6's NEW-5 paragraph is the best new text in revision 4.** Saying in terms that
an order of readiness is not an order of expected value, and that the axis cannot
answer the question the architect actually has, is exactly what a matrix that
selects nothing should say.

**And the field IS complete against `eval_families` §7's list.** I checked all
seven of its rows — A4, A5, A2, A3, A1-L11, C-SPSA, D-W1 — and all seven are
present with their kills. R3-B3 is not against that claim; it is against the
broader completeness the field needs, which is the same ground round 1's
MAJOR-11 stood on.

---

## 5. VERDICT

# FAIL

**3 BLOCKING, 8 MAJOR, 10 MINOR.** Three of round 2's seven MAJOR still land
(NEW-3, NEW-4, NEW-8), three of round 1's MINORs still land in part, and revision
4 introduces three BLOCKING-class defects of its own — two of them created by its
own remedies.

The shape is the one D-630 and D-631 named and §8's last row quotes: *"a fix
round discharges the finding's SENTENCE and re-creates its PROPERTY one step to
the left."* Revision 4 does it four times over:

- NEW-1's remedy discharged a false conclusion and replaced it with a verdict
  resting on an undisclosed one-sided summary of three noisy draws (R3-B1), drawn
  from an artifact nothing anchors (R3-B2).
- NEW-13's remedy fixed a floor that could not fire and made *"the highest floor
  in the field"* false at two sites (R3-M5).
- NEW-3's remedy corrected two cells and left three (R3-M3), two of them in §7 —
  the table round 2 already caught the previous fix missing.
- NEW-8's remedy fixed the two cost cells and left both ranking sites (R3-M4),
  and §13 reports it as done.

**The shortest route, and it is deletion and disclosure rather than argument.**
Nine of the eleven BLOCKING/MAJOR findings close by deleting a sentence or
printing a number the session already has:

1. Print the three null replicates and both summaries at `L = 7` T4; say which
   is registered; let the verdict follow the registered criterion (R3-B1).
2. Add a `census_r4` row to §10 with digest `aa88c298…` (R3-B2).
3. Correct §10's `n6_mutants` digest to `36b4c999…` and its "four mutants" to
   eight (R3-M1).
4. Price `R-A1-L7F`, or state on the page why a length the field already uses at
   rank 4 is not a free-table row — and delete *"the densest free table the
   corpus supports"* either way (R3-B3).
5. Delete *"three orders of magnitude"* and state the separation in one currency
   (R3-M2).
6. Fix lines 574, 817, 819 to the currencies revision 4 already established
   (R3-M3).
7. Delete *"Nothing to build"* and *"its build cost is nil"* from §6, and
   correct §13's NEW-8 row (R3-M4).
8. Delete *"the highest floor in the field"* from both sites, and say in §5 that
   one row's floor is not its bracket's bottom (R3-M5).
9. Reconcile the `t`-merge figures to one population, flag the 384-versus-393
   disagreement, and delete *"a price nobody has quoted"* (R3-M6).

Only R3-M7 needs an argument rather than a deletion, and it may not have one:
if `book_v3` cannot fund a row that does not answer R7, book cost is not equal,
§6's axis is not the only axis, and rank 1 moves. **That is the architect's
ruling and it should be the first line of the HANDUP**, not a paragraph inside
§1.5.

---

## 6. Would I rank the field differently, and on what?

**Yes — and revision 4's answer to the two prior red teams is not adequate,
because both of them were attacking the same thing and revision 4 answered the
smaller half of it.**

Round 1 and round 2 both said: rank R-A5-TOPK first and R-H-EXT second, on the
ground that R-H-EXT's apparent cheapness was purchased by pricing its book access
at nothing. Revision 4 concedes the build-cost half (NEW-8) and answers the book
half with D-704's argument (NEW-4). **The book half is the half that matters, and
the answer does not reach it** (R3-M7): D-704 equalises book cost across rows
whose acceptance run answers R7, and R-H-EXT's does not. Meanwhile the conceded
half was not actually conceded where it counts — §6 still says *"Nothing to
build"* and *"build cost is nil"* (R3-M4).

**My order, and what each separation rests on:**

1. **R-A5-TOPK.** A closed-form fit, no training seed, the pilot's own shape
   with its resolution measured (`t = 12` to 15 over eight paired splits, my own
   re-derivation), and the densest head in the field at either summand. Its open
   question — 811 or 143 parameters — is a question about the evaluator, not
   about whether the row can be built, and §6 is right that both counts are
   affordable.
2. **R-A1-L7F — the row that is not in the field.** 1 029 parameters, median
   1 282 observations, **zero** cells under ten, `3L = 21` (1.17x, the cheapest
   codebook traffic there is), floor 396 371 (0.75x). It beats the current rank 3
   on every column the current rank 3 is ranked on, and shares its only kill —
   which is an enumeration `eval_families` §8 marks *"not run"*. If the architect
   will not admit it, the matrix owes a stated reason, because the argument that
   admitted `R-A1-L8F` admits this with more force.
3. **R-A1-L8F.** As ranked, minus the false superlative.
4. **R-A4-CLASS at `L = 7` T4 — and I would rank it here or higher, not on the
   matrix's ground.** The matrix puts it fourth *because the nested test says the
   enum adds nothing at this cell*. That number is 0.93 only under an
   unregistered worst-of-three rule; under the mean it is 1.11 and under two of
   the three replicates it is 1.17 and 1.30 (R3-B1). On the REGISTERED criterion
   — the one that was pre-registered, could fail, and was run — this cell is MET
   at 1.014, and it is the only row in the field carrying such a criterion at
   all. The honest statement is not that the criterion *"disagrees with itself"*;
   it is that **the registered criterion passes here and an unregistered
   statistic built afterwards is being used to demote the row, which §6.5 forbids
   in terms.**
5. **R-A3-L11F+F7**, then **R-A2-L11F**, as ranked. I have no quarrel with the
   bottom of the field; §4's attacks on both are the strongest cells in the
   document.
6. **R-H-EXT: not ranked until the operator rules.** Not because its build cost
   is high — it is genuinely the cheapest thing to build in the field — but
   because *what it costs in books is unknown*, and the matrix's own §6 says it
   may not be a Phase-2 row at all. A row whose book access is unresolved and
   whose scope is disputed is not a rank; it is a question. Putting it at rank 1
   makes an unrun cheap experiment look like a considered rival to a learned
   family, which is §4's own strongest attack on it, recorded and then not acted
   on.

**The single change that would most improve this matrix** is not another
correction round. It is to state, at the top of the HANDUP, the two rulings the
field cannot be ordered without: **may `book_v3` fund a row that does not answer
R7**, and **is R-H-EXT in Phase 2's scope at all**. Both are named inside the
document, three sections apart, and neither is where the architect will look.

---

**Instruments and scratch**: `/tmp/claude-1000/…/scratchpad/rt3/{derive,sens2}.py`
plus inline `python3` passes, all read-only against
`artifacts/wp22_phase2a/`. Live tree unmodified apart from this file. No
`cargo`, no worktree, no commit. HEAD at the end of this review:
`c7a3ee1aa9910388107c05e2ec21099b42070674`, `git status --porcelain` empty apart
from this untracked report.
