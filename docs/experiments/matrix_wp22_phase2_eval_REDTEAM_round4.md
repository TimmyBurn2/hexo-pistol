# FOURTH DECISION-RED-TEAM — `matrix_wp22_phase2_eval.md` revision 6

## 1. Header

**Named revision**: `695b6708dbb71dce1dd06ef6ded4b494eaf78efd` (`dev`).

**Matches HEAD**: **NO, and it did at the start.** `git rev-parse HEAD` returned
`695b6708…` when I began and `1d3cc94ab7ac4296282a3952e61c51d647733eab` when I
finished. The one intervening commit is
`1d3cc94 docs(handup): the field is ranked with its contested order, and the two
architect questions are put`, which **adds `docs/experiments/wp22_phase2a_HANDUP.md`
and touches nothing else** (`git diff --stat 695b670 HEAD`: 1 file, 162
insertions). The subject file is byte-identical at both SHAs, so every line
number, quotation and reproducer below holds at HEAD. `git status --porcelain`
was empty at both points apart from this report.

I read the HANDUP. It is written contingently — its own §5 says *"the matrix's
own gate is at its fourth round as this document is written, and if that round
FAILs this HANDUP is superseded by a STOP"* — so it is not a review-outstanding
breach. It is not the subject of this review, but where it carries a matrix
defect forward I say so, because deleting the matrix's sentence and leaving the
HANDUP's is the defect class this package has failed on three times.

**What this document is.** A confirmation by BEHAVIOUR under D-691. For every
finding of the three prior reports I ran the ATTACK against revision 6 — never a
check for a prescribed sentence. Where a prior report named a script I wrote my
own from the receipted data. Revision 6's new text is attacked on its own account
in §3. Per `docs/process.md`'s re-derivation clause I chose my own scope: I
re-derived §2.5's tables from `census_r5` (which no prior reviewer has seen), ran
the shipped `tools/hex_enum` code against the property its own test pins, and
attacked `tools/receipt_digest_check.py` with documents built to fail it.

**Instruments, all read-only. Live tree unmodified apart from this file. No
`cargo`, no worktree, no `CARGO_TARGET_DIR`, no commit.**

| instrument | what it did |
|---|---|
| `sha256sum -c` in all **nine** `artifacts/wp22_phase2a/*/` directories, then `sha256sum` of each receipt file | verified every `RECEIPT_*.sha256` file by file and took each receipt's own digest |
| `…/rt4/derive.py` (mine — its own parser for the census purity grammar) | re-derived all 16 registered-criterion ratios, all 16 code-unit ratios and all 16 nested ratios from `census_r5`; compared 144 shared purity rows against `census_r3`; recomputed `census_r4` for the before/after |
| `python3` against the shipped `census.Length` at `L = 7, 9, 11, 13` | ran the pinning test's own computation and compared its population against the census's |
| an inline paired re-derivation over `seed_pilot/pilot_L11.txt` | §3's wins, paired means, sd, `t`, `mean/sd`, half-range/mean, sd/mean |
| an inline nps model written from §1.8's stated arithmetic | every floor and ceiling in §4 and §5 |
| an inline sensitivity regressing the null increment on the null's observed cell count | how far R4-B1's residual moves the ratios |
| `tools/receipt_digest_check.py` on the matrix, on `STOP_E`, and on four documents I wrote to fail it | its real coverage against its stated coverage |
| `/usr/bin/grep`, `git grep`, `git diff`, `git show`, `sed -n` at named lines, output `LC_ALL=C sort`ed where recorded (D-265) | every quotation, residue sweep, and the revision-5→6 diff |

**Receipt verification — all nine directories verify `sha256sum -c` clean, file
by file. Every digest the matrix prints matches its receipt on disk.** Round 3's
two mismatches are gone.

| directory | receipt digest ON DISK | anchored where |
|---|---|---|
| `census/` | `aab7f4f6…` | matrix §10 ✓ |
| `census_r3/` | `9bb8fc6f…` | matrix §10 ✓ |
| `census_r4/` | `aa88c298…` | matrix §10 ✓ (round 3's R3-B2 closed) |
| **`census_r5/`** | **`7948e47a…`** | **`wp22_phase2a_STOP_E.md:54` and the HANDUP — NOT the matrix.** The matrix never names it, while §10 calls itself *"that anchor"* and attributes §2.5's nested test to `census_r4`. See R4-M6. |
| `clockfix_confirm/` | `7678648c…` | matrix §10 ✓ |
| `n6_mutants/` | `36b4c999…` | matrix §10 ✓ (round 3's R3-M1 closed) |
| `nps_seat/` | `211b6e10…` | matrix §10 ✓ |
| `seed_pilot/` | `f3518ad2…` | matrix §10 ✓ |
| `sprt_power/` | `d82fce37…` | matrix §10 ✓ |

`git worktree list` shows only the main tree, as §10 claims.

---

## 2. Confirmation table

**CLOSED** = I ran the attack and the defect is gone. **CLOSED BUT MOVED** = the
named defect is gone and one of the same shape stands at the new text.
**STILL LANDS** = the attack reproduces.

### Round 1 (`0641504`) — 4 BLOCKING

| # | my attack | output | verdict |
|---|---|---|---|
| **B-1** | Read D-705 whole at `docs/decisions.md:1476`; read §5's paragraph. | The asymmetry clause is quoted verbatim and §5 reports §1.6 as the ADR's own premise measured. | **CLOSED** |
| **B-2** | Re-derived §3 from `pilot_L11.txt` with my own parser. | 8/8 on all three comparisons; paired means 23 472.2 / 36 878.5 / 13 406.3; sd 5 167.6 / 7 020.7 / 3 145.8; `t` 12.85 / 14.86 / 12.05; `mean/sd` 5.25. Identical to the printed digit. | **CLOSED** |
| **B-3** | Read D-653; re-checked the openings-equal-pairs premise at the code. | `config.rs:56` *"each opening is exactly one pair"*; `schedule.rs:37` `let total = openings.taken.len() * 2;`. Every power cell is in `sprt_power_two_arm.txt` / `screen_power.txt`. | **CLOSED** |
| **B-4** | Compared §2/§2.5/§6 against the enum memo at HEAD. | Re-pointed, and now at `wp22_phase2a_STOP_E.md`. But see **R4-B2**: the source it now points at STOPS the very sections the matrix ranks on. | **CLOSED BUT MOVED → R4-B2** |

### Round 1 — 11 MAJOR

| # | my attack | output | verdict |
|---|---|---|---|
| **M-1** | Compared §1.2 and §1.3 against `nps_instrument.txt` / `nps_play.txt` digit by digit. | `nps_instrument.txt` holds `529255 (526479, 531293)`, `564304`, `492073`, `453012 / 73388 nodes / 162 ms` — §1.2 is the receipt's, cell for cell. §1.3's six digits are still not in `nps_play.txt` and §1.3 and §12 both say so and compute nothing from them. | **CLOSED** |
| **M-2** | `sha256sum artifacts/wp22_phase2a/sprt_power/RECEIPT_sprt_power.sha256` vs §1.6. | `d82fce37…` both. | **CLOSED** |
| **M-3** | Re-read `repo_audit_2026-09.md` A-02 and re-solved every floor from §1.8's arithmetic. | `31.77 + 6.64 + 6.29 = 44.70` ✓. All floors reproduce: `[296 959, 549 733]`, `[365 760, 621 921]`, `[396 371, 650 390]`, `[339 538, 595 840]`, `[365 760, 529 255]`, `0.95 × 529 255 = 502 792`. | **CLOSED** |
| **M-4** | Recomputed each row's density in the currency §4 assigns it, then swept every site printing one. | R-A1-L11 `45 271/62 370 = 0.7259` ✓, A2 `3.197` ✓, A3 `426.2` and `1.1613` ✓, A4 `1.0689` ✓. **But §7's two cells still print `1.2 observations per parameter`, and the currency table now omits `R-A1-L7F` entirely.** | **STILL LANDS → R4-M3, R4-m6** |
| **M-5** | Read §5's separation and checked what §6 ranks on. | §5 separates BENCH quantities from the time-matched ARM; §6 ranks on build cost. | **CLOSED** |
| **M-6, M-7** | Grepped every site pricing R-H-EXT's cost and the screening-book claim. | *"the only row that can use the screening book"* is gone ✓; §4's and §7's cells price the screen. **§6 line 852 still reads *"its build cost is nil"*.** | **STILL LANDS → R4-M4** |
| **M-8** | Grepped for the numpy claim. | Deleted; the parenthetical explains why. | **CLOSED** |
| **M-9** | Checked the per-length margins. | §2.5 prints 1.014 / 1.108 / 1.125 / 0.834 — my own derivation from `census_r5` agrees to the digit. The generalised "4 %" is gone. | **CLOSED** |
| **M-10** | Read §2 item 4 and §2.5 for referent, unit and what a floor licenses. | Named in both. | **CLOSED** |
| **M-11** | Checked the field against `eval_families` §0.2's four measured folded lengths. | L7F and L8F are priced. **L = 9 folded (8 374 cells, median 56, 1 079 under ten) is still unpriced**, and `L = 9` is the length rank 2 headlines. | **STILL LANDS → R4-B4** |

### Round 1 — 10 MINOR

| # | my attack | output | verdict |
|---|---|---|---|
| **m-1** | `/usr/bin/grep -n '^#\{2,3\} '`. | Sections run §1…§10, then **§12, §13, §14. There is no §11.** The §8 pointer is right. | **STILL LANDS → R4-m2** |
| **m-2** | Grepped every `§6.5` / `§0.x` / `§A[0-9]`. | All name their document. | **CLOSED** |
| **m-3** | Recomputed all nine spread statistics from the receipt. | half-range/mean 2.790 / 3.457 / 3.458 %; sd/mean 2.00 / 2.34 / 2.25 %; SEM/mean 0.71 / 0.83 / 0.80 % — all on the page with their arithmetic. | **CLOSED** |
| **m-4** | `/usr/bin/grep -l 'mode = "play"' configs/*.toml`; then read §1.7. | Three configs named with the reason the third is excluded ✓. **§1.7 still multiplies 479 ms (measured on `play_v0`) by 10.4 answers/game from `sealbot_anchor_v7_protocol.md`, whose runs `sealbot_anchor_v3_prereg.md:40` puts on `configs/play_staged_v0.toml`.** | **STILL LANDS → R4-m5** |
| **m-5** | Re-did both divisions. | `3102.1 × 2.046 s = 1.763 h` against `8500 × 2.046 = 4.831 h`, reconciled on the page. | **CLOSED** |
| **m-6** | Compared §1.7's per-answer term with `nps_play.txt`. | 479 / 457 ms, *"not the 500 configured"*. | **CLOSED** |
| **m-7** | Re-did the two arithmetic-file rows. | `7.118x`, `34.38 h` at 60 500. | **CLOSED** |
| **m-8** | Compared the `L = 7` rungs in `census_r5`. | **Worse than round 3 found.** The fifteen-distinct qualifier revision 5 carried was **deleted** in revision 6, and at `L = 7` all four JOIN partitions are one partition (36 classes, `ω² = 0.004167`, increment `+0.000642`, ratios 2.67 / 1.71 — byte-identical at T4, T3, T2 and T1). *"ADDS AT ALL SIXTEEN CELLS"* is 13 distinct joins with one measurement counted four times. | **STILL LANDS, REGRESSED → R4-m1** |
| **m-9** | `git ls-files \| grep -E 'measure_nps\|measure_play\|confirm_wedge'`. | Returns nothing; §8 discloses it in bold. The `docs/process.md` coverage violation is unchanged. | **DISCLOSED, NOT FIXED** |
| **m-10** | Read line 911. | *"The claim that both are quoted whole is withdrawn. **D-653 and D-705 — are quoted whole rather than glossed**"* — the self-contradicting cell round 3 named, verbatim and unchanged. | **STILL LANDS → R4-m3** |

### Round 2 (`564dc9e`) — 13 findings

| # | my attack | output | verdict |
|---|---|---|---|
| **NEW-1** (B) | Re-derived §2.5's criterion table and nested table from `census_r5` with my own parser. | Criterion 1.014 / 1.108 / 1.125 / 0.834 and T2 0.793 / 0.673 / 0.637 — exact. The row is restored and priced. **But its rank now rests on a stopped package (R4-B2) and on a null still unmatched on the scored population (R4-B1).** | **CLOSED on the false conclusion; replaced → R4-B1, R4-B2** |
| **NEW-2** | Recomputed all 16 code-unit ratios. | 1.1401 / 1.6196 / 2.2691 / 1.6360 at T4 — §2.5 prints all five and names the flip. | **CLOSED** |
| **NEW-3** | See M-4. | Two cells fixed; §7's two still land; `R-A1-L7F` is absent from the currency table. | **STILL LANDS → R4-M3, R4-m6** |
| **NEW-4** | Read the ledger's standing claim and D-704 at `decisions.md:1474`. | The claim is quoted. D-704 says *"the **selected family's** acceptance SPRT"* and R-H-EXT reads no corpus. §14 sends the question to the architect **and §6 still ranks on the equalisation it concedes may not hold.** | **STILL LANDS → R4-M9** |
| **NEW-5** | Read §6's disclosure paragraph. | Intact and load-bearing: *"an order of readiness, not of expected value"*. Still the best paragraph in the document. | **CLOSED** |
| **NEW-6** | Compared §12's two rows against §1.2 / §1.3. | Both accurate. | **CLOSED** |
| **NEW-7** | Read §7's heading and its R-H-EXT cell. | *"BUILD cost"*, with the finding carried. | **CLOSED** |
| **NEW-8** | Grepped every site pricing R-H-EXT's build cost. | §4 ✓, §7 ✓, §6 rank 1 ✓ — **§6 line 852 not fixed.** | **STILL LANDS → R4-M4** |
| **NEW-9** | Read `handcrafted.rs:302`. | `HandcraftedV0` overrides `delta`; body reads `self.windows.get(…)` / `self.contribution(…)` and calls neither `apply` nor `undo`. The replacement reason is true at the code. | **CLOSED** |
| **NEW-10** | Grepped for the 1.77x. | Attributed to `hex_threat_enum_v1.md` §6.5 and the withdrawn criterion. | **CLOSED** |
| **NEW-11** | Read §1.6's header. | *"pairs the run is capped at"*. | **CLOSED** |
| **NEW-12** | Re-derived all four loosenesses. | Winning side named, `p` marked one-sided with the two-sided beside it, 5.25 marked an effect size, the sizes→architectures step marked. | **CLOSED** |
| **NEW-13** | Solved for the `c` at which R-H-EXT's floor fires. | `502 792` fires at `c > 1.1177`. It can fire. **But *"the highest floor of any codebook row"* at line 588 is a tie with `R-A4-CLASS` at `L = 7` (both 396 371), asserted as unique.** | **CLOSED; residue → R4-m7** |

### Round 3 (`c7a3ee1`) — 3 BLOCKING, 8 MAJOR, 10 MINOR

| # | my attack | output | verdict |
|---|---|---|---|
| **R3-B1** (max-of-3) | Counted the replicates in `census_r5` and recomputed both summaries at all sixteen cells. | 12 replicates. Under **both** summaries every one of the sixteen exceeds 1: mean 1.48–3.50, max 1.36–3.17. The choice no longer decides anything. **The summary rule is still unregistered** — `STOP_E` lists *"a registered summary rule"* as owed — but it is no longer load-bearing. | **CLOSED** |
| **R3-B2** (`census_r4` anchored nowhere) | `sha256sum` each receipt; `git grep` each digest. | `aa88c298…` is now in matrix §10 ✓ and `hex_threat_enum_v1_ROUND4.md` ✓. **`census_r5`, which produces every nested number §2.5 prints, is anchored in `STOP_E` and the HANDUP but is named nowhere in the matrix, whose §10 calls itself *"that anchor"* and credits `census_r4` with §2.5's nested test.** | **CLOSED at `census_r4`; the property recurs at `census_r5` → R4-M6** |
| **R3-B3** (field incomplete) | Checked all four measured folded lengths in `eval_families` §0.2 against the field. | `R-A1-L7F` added and priced correctly (1 029 / median 1 282 / 0 under ten / 190 cover 90 % / 90.8 % of ceiling — all exact against §0.2). **`L = 9` folded is still unpriced and the banner declares the field complete.** | **CLOSED at `L = 7`; STILL LANDS at `L = 9` → R4-B4** |
| **R3-M1** (`n6_mutants` digest) | `sha256sum` vs §10. | `36b4c999…` both ✓. **§10 still says *"four mutants … all four DEAD"* of a directory whose `mutants.txt` and `referent_mutants.txt` list eight, all DEAD.** | **CLOSED on the digest; the prose STILL LANDS → R4-m8** |
| **R3-M2** (three orders of magnitude) | Recomputed the comparison in every shared currency. | *"three orders of magnitude"* is deleted; 13.3x is right — `15.4773 / 1.1613` — **at `L = 7`. At `L = 9`, the cell the same paragraph headlines, it is `0.2800 / 1.1613 = 0.24x`: A4 is 4.1x WORSE than A3.** | **CLOSED as stated; a worse one created → R4-M7** |
| **R3-M3** (three sites print `1.2`) | Grepped all three. | Line 574's *"Against A1-L11's 1.2 and A3's 1.2"* is gone ✓. **Lines 880 and 882 — both in §7 — still print *"1.2 observations per real parameter"* and *"killed: 1.2 observations per parameter"*.** §14 says *"all sites swept"*. Third consecutive round in which §7 is the table the fix misses. | **STILL LANDS → R4-M3** |
| **R3-M4** (§6's two build-cost sites) | Grepped both. | Rank 1's *"Nothing to build"* is fixed ✓. **Line 852 still reads *"its book cost is the field's common cost and its build cost is nil"*.** §14 says *"§6's build-cost sentence"* was swept. | **STILL LANDS → R4-M4** |
| **R3-M5** (*"highest floor in the field"*) | Listed every registered floor and compared. | *"in the field"* is gone; §5's A4 row is rewritten. **Line 588's *"the highest floor of any codebook row"* is a tie, not a maximum**, and **§5 still does not say that R-H-EXT's floor is not its bracket's bottom** while §6 reads a high floor as *"the least throughput risk"*. | **CLOSED in part; STILL LANDS → R4-m7** |
| **R3-M6** (`t`-merge, four parts) | Read `hex_threat_enum_v1.md:416-418` and `:449`, then every matrix site. | §4 ✓ (both populations with their denominators), 384-vs-393 explained ✓, *"a price nobody has quoted"* deleted ✓. **§2.5 line 429 still reads *"27 of the 357 classes, 5 348 of 59 049 patterns"* — 27 and 5 348 are the rule-4-EXCLUDED figures, 357 and 59 049 the NOT-excluded ones. §14 claims both populations are printed with their denominators.** | **STILL LANDS at §2.5 → R4-M5** |
| **R3-M7** (book equalisation) | Re-read §1.5, §6 and D-704. | §14 puts the question to the architect. **§6 still ranks R-H-EXT first on an axis licensed by an equalisation the same document says may not hold for that row.** Round 3's *"it may not have one"* is unanswered. | **STILL LANDS → R4-M9** |
| **R3-M8** (ARITHMETIC file) | Read the file's header and its three tables. | It now says which revisions it checks. **Its header says *"Governing revision: matrix revision 5"* against a matrix headed revision 6, and it deflects revision 4/5's numbers to *"the matrix's own §12 and §13"* — the round-1 and round-2 tables, which check none of them. Revision 6's new numbers (40.4, 15.5, 339 538, the corrected nested table) are checked nowhere.** | **STILL LANDS in part → R4-m9** |
| **R3-m1** (13 of 16 / 1.26–1.76) | Recomputed the range over all sixteen and both summaries. | *"1.36x to 3.50x"* is exact: min-of-max 1.36 (`L = 13` T4), max-of-mean 3.50 (`L = 13` T1). | **CLOSED** |
| **R3-m2** (the `L = 7` joins are one partition) | See m-8. | Regressed — the qualifier was deleted. | **STILL LANDS → R4-m1** |
| **R3-m3** (self-contradicting cell) | Read line 911. | Verbatim unchanged. | **STILL LANDS → R4-m3** |
| **R3-m4** (no §11) | Section list. | Unchanged. | **STILL LANDS → R4-m2** |
| **R3-m5** (fishtest *"exactly"*) | `1046535/100`. | = 10 465.35. Line 184 still says *"the second gives 5 233, which is fishtest's `T = 1046535/Δ²` exactly"* with no `/2`, **while `_ARITHMETIC.md` carries `1046535/100/2 = 5 233` explicitly.** | **STILL LANDS → R4-m4** |
| **R3-m6** (cross-seat wall) | See m-4. | Unchanged. | **STILL LANDS → R4-m5** |
| **R3-m7** (veto's alpha) | Read §1.6 item 2 and §5. | Unchanged: answered in §1.6, disclaimed in §5. | **STILL LANDS** |
| **R3-m8** (currency table omits the cell-centred unit) | Read the table. | Unchanged — **and worse: `R-A1-L7F`, added since, appears in no row of it.** | **STILL LANDS, REGRESSED → R4-m6** |
| **R3-m9** (four mutants of eight) | See R3-M1. | Unchanged. | **STILL LANDS → R4-m8** |
| **R3-m10** (coverage violation) | See m-9. | Disclosed, unfixed. | **DISCLOSED, NOT FIXED** |

**Score.** Round 1: BLOCKING 3 CLOSED / 1 moved; MAJOR 8 CLOSED / 3 still land;
MINOR 6 CLOSED / 4 still land (one regressed). Round 2: 9 CLOSED / 3 still land /
1 closed-with-residue. Round 3: BLOCKING 1 CLOSED / 2 recur; MAJOR 2 CLOSED / 6
still land in whole or part; MINOR 2 CLOSED / 8 still land.

---

## 3. NEW findings against revision 6

### R4-B1 (BLOCKING). The corrected null is STILL not matched on the property that drives its score. *"Matching at 1.00x everywhere"* is true of the CODE SPACE and false of the SCORED POPULATION the statistic is computed on — at all sixteen cells, by 1.17x to 1.83x — and the test that pins it enumerates the wrong population.

This is the sentence the whole reversal rests on. §2.5(b):

> That is a correctness defect, it was fixed at the code
> (`permuted_within_counts`, **matching at 1.00x everywhere**)

`wp22_phase2a_STOP_E.md`, same claim tagged **MEASURED**:

> **MEASURED**: the ratio of null-join cells to real-join cells goes from 3.96 /
> 3.08 / 3.98 / 4.03 to **1.00 at every rung and length**.

and the enum memo's STOPPED banner repeats it.

**Reproducer 1 — the property holds where the test looks.** `Length.__init__`
builds the null over `range(self.size)`, and `test_census.py:249-257` checks
`len(matched) == len(real)` over the same full code space:

```
$ python3 -c "...from census import Length; ... for c in range(unit.size)"
L7  T4: CODE-SPACE real=49    matched null=49    ratio=1.000
L9  T4: CODE-SPACE real=290   matched null=290   ratio=1.000
L11 T4: CODE-SPACE real=1323  matched null=1323  ratio=1.000
L13 T4: CODE-SPACE real=3964  matched null=3964  ratio=1.000
```

**Reproducer 2 — the property fails where the score is computed.** `Moments.terms()`
takes `classes = len(self.n)`, and `self.n` counts only cells a corpus window
actually hit. From `artifacts/wp22_phase2a/census_r5/` (receipt verified), the
`classes` column of the purity block:

```
              JOIN cells   JOINNULL cells (mean of 12)   ratio
L=7  T4            36                42.0               1.167
L=9  T4           211               267.9               1.270
L=9  T2           117               145.3               1.242
L=11 T4           812              1145.2               1.410
L=11 T2           233               327.8               1.407
L=13 T4          1468              2692.8               1.834
L=13 T2           377               599.2               1.590
```

**All sixteen cells are above 1.00x, none is at 1.00x, and the excess grows with
`L` by a factor of 1.57 across the table.** `permuted_within_counts` preserves the
class multiset inside each count stratum over the code space, so the *code-space*
join is exact — but the corpus does not observe every code, and permuting
redistributes the unobserved ones, so the *observed* join is finer. The docstring's
own claim — *"the join's cell count and its per-cell code multiplicities are
exactly the real join's"* — is the code-space statement, and the census does not
compute on the code space.

**Why it is BLOCKING and not a nit.** `STOP_E` names four rounds of one defect —
*"a referent that was not matched on the property driving its score"* — and lists
as the successor package's **first owed item**: *"A referent matched on every
property that drives the score, not on one of them."* That obligation is asserted
discharged, in three documents, and is measurably undischarged. This is the fifth
instance of the one defect, created by the fix for the fourth. And §8's table
offers, as this matrix's defence against round-3 minor 1 (*"a false invariant
asserted in a governing document, pinned by a test that could not see the
counterexample"*), the sentence *"Stage E's self-checks are **enumerations over
the whole pattern space**"* — which is precisely the mechanism that produced it.

**What survives, stated because it matters.** I ran a sensitivity: within each
length, regress the mean null increment on the null's observed cell count across
the four rungs and extrapolate to the join's own cell count (**ESTIMATED**, a
within-run regression, not a measurement of an observed-matched null).

```
L=9  T4: reported 1.88 -> ~2.55     L=11 T4: reported 1.59 -> ~2.30
L=9  T2: reported 3.18 -> ~4.46     L=11 T2: reported 3.25 -> ~5.55
                                    L=13 T4: reported 1.48 -> ~2.64
```

The residual is **conservative**: correcting it raises every ratio. So *"the enum
ADDS"* is not reversed and is probably understated. What is wrong is the stated
property of the instrument, in the one sentence that licenses the reversal, and
the claim that the four-round defect is closed.

**What would close it.** Delete *"matching at 1.00x everywhere"* and *"1.00 at
every rung and length"*; print the observed-population ratios; state that the
match is over the code space and that the scored-population residual is 1.17x to
1.83x with the direction of its bias; and move the null-matching item back onto
the successor package's ledger where `STOP_E` put it.

### R4-B2 (BLOCKING). Rank 2 is ranked on two measurements whose own package was STOPPED AND SPLIT as unsettled one commit earlier, and on a criterion side §2.5 itself says licenses nothing.

§6 rank 2's ground, in terms:

> **It is the only row in the field carrying a criterion that was registered
> before its run, could fail, was run, and did not fire.**

Three things stand against that sentence, and the matrix supplies all three.

1. **§2.5(a) disclaims the not-firing.** *"Its FAIL side is evidence; **its PASS
   side is not** — the statistic is class-count monotone under position-clustered
   labels."* A criterion that fires is evidence; a criterion that does not fire
   is, by the document's own paragraph, not. §4's cell headline —
   *"kill condition — REGISTERED, RUN, AND NOT FIRED AT T4"* — is the same
   non-evidence promoted to a heading.
2. **The criterion is SUPERSEDED by the source the matrix cites.** `hex_threat_enum_v1.md`
   at HEAD, its own banner: *"**WHAT IS SUPERSEDED**: **§6.5's criterion and
   §7.4b / §7.4c**"*. §6.5 is where the criterion is defined. §2.5 presents it as
   *"The REGISTERED criterion"* with no mark.
3. **`STOP_E` sends both measurements back as an unsettled package**: *"§6.5 the
   criterion and §7.4 onward — **GOES BACK AS ITS OWN PACKAGE.** Four rounds could
   not settle what referent this question needs"*, owing three things of which
   R4-B1 shows the first is undischarged and the second (*"a registered summary
   rule"*) is still not registered.

So the row moved from rank 4 to rank 2 on (i) a criterion whose defining section
its own source marks superseded and whose PASS side this document disclaims, and
(ii) a post-hoc nested test from a stopped package whose null R4-B1 measures is
still unmatched. **The matrix ranks on the output of a package it cites as
stopped for not having settled how to produce that output.**

**And one thing the matrix does not notice cuts the OTHER way**, which is why
this is a finding about the argument rather than about the row. The blanket
withdrawal of the PASS side is justified by *"T4 carries 231 classes against the
referent's 52 at `L = 11`"* — true there, and true at `L = 9` (78 against 38).
**At `L = 7` it reverses**: `census_r5` gives `enum T4` **16 classes** against
`COUNT-ONLY`'s **23**. The enum wins 1.014 with a *coarser* partition, where the
monotonicity objection has no purchase at all. The document discounts its own
strongest cell by an argument that does not apply there, and ranks on the cell
where the argument does apply.

**What would close it.** Say which of the two measurements carries the rank; mark
the criterion superseded where it is quoted; state that the PASS side is evidence
at `L = 7` (coarser than its referent) and not at `L = 9`/`L = 11`; and either
rank on that narrowed basis or say the row cannot be ranked above an unmeasured
row on it.

### R4-B3 (BLOCKING). The correction that promoted `R-A4-CLASS` was not applied to the row it also demotes. `R-A1-L7F` is ranked 4 on a reading of `L = 7` that revision 6's own §2.5 reverses — at three sites, one of them citing a section its package marks superseded.

The three sites, unchanged from revision 5:

```
593 (§4, R-A1-L7F): `hex_threat_enum_v1.md` §7.4 measures that at `L = 7` a quotient
    knowing only stone counts explains as much as the threat tuple does, which is
    evidence … that seven cells is where structure stops being visible on this corpus.
810 (§6, rank 4):   because §2.5 measures that at `L = 7` a quotient knowing only
    stone counts explains as much as the threat tuple does
878 (§7, R-A1-L7F): §2.5 measures that at `L = 7` stone counts explain as much as
    threat structure
```

**What §2.5 measures at `L = 7` T4, in revision 6:** the registered criterion is
**MET at 1.014**, and the corrected nested test says the enum **ADDS at 1.71
worst / 2.67 mean** — the *largest* max-ratio at T4 of any length in the table.
The matrix's own rank 2 calls that same 1.4 % *"real and small"* and keeps the row
LIVE on it. **The identical evidence is a promotion three ranks up and a demotion
three ranks down, in one document.**

And site 593 cites `§7.4` of a memo whose banner marks §7.4b/§7.4c superseded and
whose corrected table lives in `STOP_E` — the very supersession §2.5 is built on.

This is the shape D-630/D-631 named and §8's last row quotes: the fix discharged
the finding's sentence at the row that was measured and re-created its property
one row to the left. The HANDUP has already carried it forward verbatim in its
rank-4 attack column.

**What would close it.** Delete all three sentences, or restate them as what
`census_r5` says: at `L = 7` the enum beats stone counting by 1.4 % on the
registered criterion and adds 1.71–2.67x conditional on them — a small positive,
not an absence.

### R4-B4 (BLOCKING). The field is still incomplete, the missing row is the fourth measured length in the same table that supplied the other three, and the banner declares the field complete.

The banner: *"**TWO ROWS WERE MISSING FROM THE FIELD AND BOTH WERE ON THIS PAGE
BEFORE THEY WERE PRICED**"* — past tense, and §14 closes R3-B3.

`eval_families_2026-09.md:53` (§0.2), the same table `R-A1-L7F` and `R-A1-L8F` are
priced from:

| L | fold | cells | median obs/cell | cells < 10 | cover 90 % | ceiling | % of ceiling |
|---|---|---|---|---|---|---|---|
| 7 | yes | 1 029 | 1 282 | 0 | 190 | 1 133 | 90.8 % — **priced** |
| 8 | yes | 2 920 | 273 | 24 | 293 | 3 320 | 88.0 % — **priced** |
| **9** | **yes** | **8 374** | **56** | **1 079 (12.9 %)** | **434** | **9 962** | **84.1 % — NOT priced** |
| 11 | yes | 38 983 | 7 | 22 142 | 810 | 88 937 | 43.8 % — priced (killed) |

```
$ /usr/bin/grep -n 'L9F\|8 374\|8374\|length 9' docs/experiments/matrix_wp22_phase2_eval.md
(no output)
```

Priced by §1.8's own arithmetic and the corpus's own counts: **8 374 parameters,
median 56 observations, `3L = 27` (1.50x), floor 339 538 (0.64x), 5.41 positions
per parameter, 1 267 window observations per parameter.** That is denser than
`R-A3` (1.16 positions), a higher floor than `R-A5`/`R-A2`/`R-A3` (296 959), and
the closest of the three sub-covering free tables to `§0.1`'s minimum. It would
land at or above rank 5.

Round 3 named it in terms — *"`L = 9` folded … is a fourth measured length in the
same §0.2 table, also unpriced. The free-folded-table family has four measured
lengths and the field prices two."* Revision 6 prices three. **And `L = 9` is not
an unused length in this document: it is the headline live length of the row
ranked 2.** The argument that admitted `R-A1-L8F` (round 1) and `R-A1-L7F`
(round 3) admits this one with the same force, for the third time.

**What would close it.** Price it, or state on the page why a length the field
already uses at rank 2 is not a free-table row — and delete the banner's claim
that the field's two missing rows have been found.

### R4-M1 (MAJOR). `R-A4-CLASS` registers TWO different nodes/sec floors, and §4's is computed at the traffic of a length the same cell rules out.

D-705: *"each row pre-registers a nodes/sec floor for the bench bracket"* and
*"a row without one is not testing what decides the outcome"*.

```
§4  (line 556): a codebook at 1.83x the traffic … gives nps ∈ [296 959, 549 733]
                = [0.56x, 1.04x]. FLOOR REGISTERED AT 296 959 nps (0.56x)
§4  (line 555): at the live lengths, 3L = 21 (L = 7, 1.17x) and 27 (L = 9, 1.50x)
§5  (line 699): 396 371 (0.75x) at L = 7, 339 538 (0.64x) at L = 9; the L = 11
                rungs are out on density
```

1.83x is `3L = 33`, i.e. `L = 11` — the rung §5 and §4's own kill cell say is out
on density. **The row therefore registers 296 959 in §4 and 396 371 / 339 538 in
§5, and §4's is the one carrying the words *"FLOOR REGISTERED"*.** Revision 6
rewrote §5's cell for the new live lengths (`git diff 90908cf 695b670`) and left
§4's untouched.

The same cell's density row inherits it: *"T4 **1.07 per nominal** … Population
8 174 025 scored cells"* — every figure is `L = 11`'s. The live cells' figures,
which the document has (`census_r5`), are **1 720.1** at `L = 7` (5 031 327 scored
cells) and **40.43** at `L = 9` (6 537 654). **The row's own §4 cell prices it
entirely at the rungs it rules out.**

### R4-M2 (MAJOR). Two rows claim *"the cheapest codebook traffic in the field"*, eighteen lines apart, and the second is refuted by the first.

```
587 (R-A1-L7F): `3L` = **21**, **1.17x** — the cheapest codebook traffic in the field
605 (R-A1-L8F): `3L` = **24**, **1.33x** — the cheapest codebook traffic in the field
```

24 is not cheaper than 21. This is the exact class §8's table credits this matrix
with not repeating — *"2 M-3 / 3 MAJOR-1 … a uniqueness claim the document's own
adjacent column refuted"* — and it is the same row whose *"densest free table"*
superlative round 3 had already corrected. The fix corrected one superlative in
the cell and left the other.

A third site compounds it: line 555 calls A4's 21 and 27 *"the two cheapest
codebook figures in the field **after** `R-A1-L7F`'s 21"* — 21 is not after 21.

### R4-M3 (MAJOR). R3-M3's third and fourth sites are still in §7, and §14 reports all sites swept. Third consecutive round at the same table.

```
880: | R-A3-L11F+F7 | … 1.2 observations per real parameter is the density that killed A1 |
882: | R-A1-L11 | — | killed: 1.2 observations per parameter, and unfolded it learns twelve answers |
```

Revision 4 established that A3's density is **426 window observations** or **1.16
positions** per parameter, that A1-L11's killing density is **0.73 positions per
parameter unfolded**, and that *"observations per parameter"* is the mislabel it
corrected at its source. Line 882 prints the FOLDED figure for the UNFOLDED row
under the deleted label; line 880 does the same for A3 and calls it *"the density
that killed A1"*, which is 0.73.

§14: *"all sites swept: the currency table's two rows, §6's build-cost sentence,
and the two *"highest floor"* claims."* §7 is not among them. Round 2 found §7
missed once; round 3 found it missed twice; this is the third.

### R4-M4 (MAJOR). *"Its build cost is nil"* still stands in §6 — the section whose entire axis is build cost — and §14 reports it fixed.

```
852: The attack prices R-H-EXT as though the screen were compulsory; it is not —
     … at which point its book cost is the field's common cost and **its build
     cost is nil**.
```

against §4 line 673, §6 rank 1 and §7 line 875, all of which now say the opposite
in bold. §6 rank 1: *"Its build cost is **NOT** nil and §4 says so"*. The two are
lines **774** and **852** — seventy-eight lines apart in the same section.

§14: *"all sites swept: … §6's build-cost sentence"* — singular, and there were
two. This is R3-M4 landing for the second consecutive round, at the site round 3
printed by line number, in a document that quotes D-424's *"the second site is
what gets missed"*.

### R4-M5 (MAJOR). §2.5 still pairs the `t`-merge counts across two populations, and §14 says both are printed with their denominators.

```
429 (§2.5): the tuple merges patterns whose exact `DEF-T` … differs — **27 of the
            357 classes, 5 348 of 59 049 patterns**
```

`hex_threat_enum_v1.md:416-418`: *"differs within **27 of the 335 classes** those
patterns present, covering **5 348 of the 57 996 patterns**. **Without the rule-4
exclusion it is 32 classes of 357 and 6 099 of 59 049**."*

27 and 5 348 are the excluded reading; 357 and 59 049 are the not-excluded one.
Neither source supports the pairing. §4 (line 561) and §7 (line 876) are correct;
§2.5 — the section a reader reaches first — is not. §14's row: *"both populations
printed with their denominators"*.

### R4-M6 (MAJOR). §10 attributes §2.5's nested test to `census_r4`, names a replicate count no receipt holds, and omits the directory that actually produced the table — while calling itself *"that anchor"*.

```
942: | the same instrument at 3 null replicates, then at 24 | §2.5's NESTED test,
     and the referent's own spread | `artifacts/wp22_phase2a/census_r4/`, `aa88c298…` |
951: `artifacts/` is gitignored …, so an artifact directory is evidence only while
     a tracked document carries its receipt's own digest. **This table is that anchor.**
```

Three defects in one row.

1. **`census_r4` did not produce §2.5's nested table.** From `census_r4`, `L = 7`
   T4 gives ratio mean 1.11 / max 0.93 — the numbers revision 6 exists to
   *replace*. §2.5's 2.67 / 1.71 are `census_r5`'s.
2. **No receipt has 24 null replicates.** `census_r4` holds 3 (`grep -c 'window\tJOINNULL count x T4 r'` → 3); `census_r5` holds 12. *"then at 24"* names a run
   nothing on disk records.
3. **`census_r5` appears nowhere in the matrix.** It is anchored in `STOP_E:53-54`
   and in the HANDUP, so the evidence survives — but the matrix's own anchor table
   does not carry the directory every decisive number in §2.5 comes from, which is
   R3-B2's property recurring one directory later, in the row the remedy for
   R3-B2 was written into.

### R4-M7 (MAJOR). §6 rank 2's *"13.3x in the one currency both share"* is the `L = 7` figure, and at `L = 9` — the cell the same paragraph headlines — the comparison reverses by 4.1x.

```
790: It beats R-A3 by **13.3x** in the one currency both share — positions per
     parameter, 15.5 against 1.16.
```

Two lines above: *"At `L = 9` T4 the row is affordable (40.4 observations per
nominal parameter), MET … and adding …"*, and the row's own heading is
*"at `L = 9` or `L = 7`"*.

```
positions per parameter:  A4 at L=7 T4  45 271 /   2 925 = 15.477   -> 13.33x A3
                          A4 at L=9 T4  45 271 / 161 700 =  0.280   ->  0.24x A3
                          A3            45 271 /  38 983 =  1.161
```

**At the row's headline live cell, A4 is 4.1x WORSE than the row it is said to
beat by 13.3x.** A reader takes 13.3x as a property of the row; it is a property
of one of its two live cells, chosen without saying so, and the other reverses it.

The parenthetical *"(The red team's own 6.6x was cross-currency too.)"* in §14 is
also wrong: round 3's 6.6x set `8 241 249 / 2 925 = 2 817` against A3's `426`,
both *"window observations per parameter"*. They are different window
populations, which is a fair objection — but it is not the objection §14 makes,
and *"the ONE currency both share"* is false either way, since two are computable.

### R4-M8 (MAJOR). `tools/receipt_digest_check.py`'s stated coverage is broader than its behaviour, in three demonstrated ways — including the exact defect it was written for.

§14: *"verifies **every digest a document prints** against the receipt on disk and
flags a document that names **a receipt directory** without printing its digest"*.
The docstring: *"Every receipt digest a document prints, checked against the
receipt on disk."*

I wrote four documents and ran the shipped script on each.

```
bad1: names artifacts/wp22_phase2a/census_r5/ and prints a zero digest
      -> FAIL, exit 1                                        (correct)

bad2: prints d7e33861129a51bcd492a271dec468cc54056f2e384dd0076e785124102016d7
      -- the exact stale digest round 3 caught -- with no directory path beside it
      -> "1 document(s), 0 check(s), 0 failure(s)", exit 0    NOT CAUGHT

bad3: names artifacts/wp22_phase2a/census_r9/  (does not exist)
      -> "1 check(s), 0 failure(s)", exit 0                   NOT CAUGHT

bad4: names artifacts/book_v9_disjoint/ and prints a bogus digest
      -> "0 check(s), 0 failure(s)", exit 0                   NOT CAUGHT
```

`bad2` fails because `hits = printed & set(known)` only ever inspects digests that
already match a receipt — a digest matching nothing is invisible unless the
directory is *also* named. `bad3` fails because a non-existent directory hits
`continue`. `bad4` fails because `named` is filtered by `"wp22" in m or "research"
in m`, hard-coding one package, and the summary line reports nothing skipped.

The tool is useful and its output line is honest about the count; **the claims
made for it in §14 and §10 are not, and its VOID/exit-2 discipline does not cover
the silent-zero-checks case.** Running it on the matrix gives *"16 check(s), 0
failure(s)"* while `census_r5` — R4-M6 — goes unremarked, because presence is all
it can see.

### R4-M9 (MAJOR). R3-M7 unanswered: §6 still ranks on a book-cost equalisation the same document says may not hold for the row it promotes to rank 1.

The chain is unchanged. §6: *"**Book cost is now EQUAL across every live row** …
so it cannot rank them, and what remains is build cost"*, resting on §1.5's
D-704 argument that R7 *"is answered by whichever family's acceptance SPRT runs"*.
D-704 at `decisions.md:1474` says *"the **selected family's** acceptance SPRT"*,
and §6 itself concedes *"R-H-EXT is arguably not a Phase-2 row … a package of hand
terms is Stage-0 work nobody did."*

§14 sends the question to the architect, which is right, **and then keeps the rank
that the unanswered question licenses.** An axis whose enabling premise is
conceded to be open at exactly one row cannot rank that row first; it can only
report that the row is unrankable until the premise is settled — which is what all
three prior red teams said. This is round 1's QUESTION 4, round 2's NEW-4, round
3's R3-M7 and now round 4, unchanged in substance across four revisions.

### R4-M10 (MAJOR). §2.5's blanket withdrawal of the PASS side is false at `L = 7`, where the enum beats its referent with a COARSER partition.

Stated in R4-B2 point 3 and repeated here because it is a finding in its own
right and it favours the row. `census_r5`, window unit, `classes` column:

```
L=7 : enum T4 = 16 classes   COUNT-ONLY = 23 classes   ratio 1.0136  (enum COARSER)
L=9 : enum T4 = 78           COUNT-ONLY = 38           ratio 1.1077  (enum finer)
L=11: enum T4 = 231          COUNT-ONLY = 52           ratio 1.1246  (enum finer)
```

§2.5(a): *"its PASS side is not [evidence] — the statistic is class-count monotone
under position-clustered labels, and T4 carries 231 classes against the referent's
52 at `L = 11`."* The justification is `L = 11`'s and is applied to all four
passes. At `L = 7` the monotonicity runs the other way and the pass is stronger
evidence, not weaker. The document withdraws its own best cell on an argument that
reverses there, and then ranks the row on the cell where the argument bites.

### MINOR

**R4-m1.** §2.5's *"THE ENUM ADDS AT ALL SIXTEEN CELLS"* counts one measurement
four times, and revision 6 **deleted** the qualifier revision 5 carried. At
`L = 7` all four rungs share one join partition — `JOIN count x T4/T3/T2/T1` are
each 36 classes, `ω² = 0.004167`, increment `+0.000642`, ratios 2.67 / 1.71,
byte-identical. Sixteen cells are **13 distinct joins**. Revision 5's *"(Two of
those four are one partition … the sixteen cells are fifteen distinct
partitions.)"* was removed in the same edit that made the sixteen-cell claim the
headline (`git diff 90908cf 695b670`).

**R4-m2.** The section numbering hole survives a third round: §1…§10, then §12,
§13, §14. There is no §11.

**R4-m3.** Line 911 still contradicts itself in adjacent clauses: *"The claim that
both are quoted whole is withdrawn. D-653 and D-705 — are quoted whole rather than
glossed"*.

**R4-m4.** Line 184 still says *"5 233, which is fishtest's `T = 1046535/Δ²`
exactly"*. `1046535/100 = 10 465.35`; the identity needs the games→pairs halving,
which the author's own `_ARITHMETIC.md` prints as `1046535/100/2 = 5 233`. The
word *"exactly"* is in a sentence whose whole purpose is an external cross-check.

**R4-m5.** §1.7's play-seat wall still multiplies 479 ms (measured on `play_v0`)
by 10.4 answers per game per side taken from `sealbot_anchor_v7_protocol.md`,
whose runs `sealbot_anchor_v3_prereg.md:40` puts on `configs/play_staged_v0.toml`
— in a document whose §1.3 exists to say the two seats differ by 2.8x in
throughput and 2.8x in depth reached. Game length depends on the seat.

**R4-m6.** §4's currency table still lists one window unit (*"one `(axis, start)`
window … 16 613 729 of them at `L = 11`"*) while `R-A5-TOPK` and §6 rank 3 use the
cell-centred population (12 980 519 over 11 909 codes) — round 2's NEW-3, round
3's R3-m8, unchanged. **And it has regressed: `R-A1-L7F`, added since, appears in
no row of the table**, and the scored-cell row still gives only `L = 11`'s
8 174 025 while `R-A4-CLASS`'s live cells are 5 031 327 and 6 537 654.

**R4-m7.** Line 588's *"the highest floor of any codebook row"* is a tie —
`R-A4-CLASS` at `L = 7` registers the same 396 371 (§5's own table) — asserted as
a maximum and glossed as *"hence the least throughput risk"*. And R3-M5's second
half is unfixed: §5 still does not say that R-H-EXT's 502 792 is a decision point
0.26x above its bracket's bottom while every other row's floor IS its bracket's
bottom, so the column is read as homogeneous by §6 when it is not.

**R4-m8.** §10's mutant row still describes *"four mutants of `report.py` … all
four DEAD"* of a directory whose two result files list eight (`M1–M4` in
`mutants.txt`, `R1–R4` in `referent_mutants.txt`, all DEAD).

**R4-m9.** `matrix_wp22_phase2_eval_ARITHMETIC.md` is headed *"Governing revision:
matrix revision 5"* against a matrix headed revision 6, and deflects revision 4's
and 5's numbers to *"the matrix's own §12 and §13"* — the round-1 and round-2
tables, which check none of them. Revision 6's new arithmetic (40.4, 15.5,
339 538, the corrected nested table, the 1.36–3.50 range) is checked nowhere.

**R4-m10.** §6's closing block is duplicated near-verbatim — *"WHAT WOULD REORDER
THIS"* + *"NOTHING IS SELECTED HERE"* at lines 831-838 and again at 862-869 — and
**the two copies have already drifted**: the first says the enumeration *"settles
whether `R-A1-L7F`'s and `R-A1-L8F`'s shared kill is real"*, the second says it
*"settles whether `R-A1-L8F`'s kill is real and could move it above `R-A5`"*.
D-423: *"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT WAITING."*

**R4-m11.** Line 671's R-H-EXT floor cell says the vacuous abort threshold is
*"the vacuous check hard rule 5's abort discipline exists to forbid and which §8
credits this document with naming elsewhere"*. `/usr/bin/grep -n 'vacuous'`
returns line 671 and line 917 only; §8's line 917 is about vacuously-passing
self-checks in Stage E, not about an abort threshold. The self-credit points at
nothing.

**R4-m12.** *"Revision 5"* names a document that never existed. `git show
90908cf:docs/experiments/matrix_wp22_phase2_eval.md | head -1` returns
*"# OPTION MATRIX — … **revision 4**."* — the commit whose message says *"revision
five"* left the header at 4, and revision 6 skips 5. §2.5, §6 and the banner all
attribute the 0.93/UNDECIDED text to *"revision 5"*; a reader tracing it finds a
file headed revision 4, which is also what round 3 reviewed.

**R4-m13.** Round 1's QUESTION 3 is unanswered across four revisions: WP-2.2
Phase 1 produced *"six tables recorded as equally principled"* (D-637), and the
field carries only the one-integer slice (`R-D-W1`) and its kill. *Deploy one of
the six* may be correctly absent under D-704; the matrix still does not say so.

---

## 4. What I attacked and it SURVIVED

A great deal of revision 6 is right, and two of its corrections are better than
the findings that prompted them.

**Every receipt verifies and every digest the matrix prints is on disk.** Nine
directories, file by file, `sha256sum -c` clean; nine receipt-file digests taken;
every one the matrix prints matches. Round 3's two mismatches are genuinely
closed.

**§2.5's tables are transcribed exactly, and I derived them from a census no prior
reviewer has seen.** All sixteen registered-criterion ratios (1.0136 / 1.1077 /
1.1246 / 0.8336 at T4; 0.7932 / 0.6732 / 0.6373 at T2), all sixteen code-unit
ratios (1.1401 / 1.6196 / 2.2691 / 1.6360), and all four displayed nested rows
(+0.000642/+0.000241/2.67/1.71; +0.001902/+0.001011/1.88/1.64;
+0.001576/+0.000485/3.25/2.51; +0.001962/+0.000599/3.27/2.48) reproduce to the
printed digit from my own parser.

**The census replicates a fourth time.** I compared all **144** purity rows that
`census_r3` and `census_r5` share — different runs, different replicate counts,
different null construction — and found **0 differences** in `ω²` or class count.
Four census runs and four independent implementations agreeing cell for cell is a
strong result and the matrix is entitled to lean on it.

**The nested conclusion survives its own defect.** *"ADDS at all sixteen cells
under both summaries, 1.36x to 3.50x"* is exact against `census_r5`, and my
sensitivity says R4-B1's residual is conservative — correcting it would raise
every ratio. The finding is against the stated property of the instrument and the
claim that a four-round obligation is discharged, not against the direction of the
result.

**R3-B1 is genuinely resolved by measurement rather than by choosing a summary.**
At twelve replicates both summaries agree at every cell. That is the right way to
close it.

**§3 is exact.** My independent parse gives means 657 864 / 634 392 / 620 985,
ranges 36 705 / 43 863 / 42 952, half-range 2.790 / 3.457 / 3.458 %, sd/mean 2.00
/ 2.34 / 2.25 %, 8/8 on all three paired comparisons, `t` 12.85 / 14.86 / 12.05,
`mean/sd` 5.25 — identical to the digit, and the caveat that sizes→architectures
is a step is the right call.

**Every nps floor and ceiling reproduces exactly** from §1.8's stated arithmetic
at all five traffic ratios, including 339 538 at `L = 9` and 502 792 at 0.95x.

**`R-A1-L7F` and `R-A1-L8F` are priced correctly against `eval_families` §0.2.**
1 029 / median 1 282 / 0 under ten / 190 cover 90 % / 90.8 % of the 1 133 ceiling
✓; 2 920 / median 273 / 24 (0.8 %) / 293 / 88.0 % of 3 320 ✓. Their shared kill is
honestly stated and its unrun enumeration is disclosed at both rows. **The field's
gap is `L = 9`, not these two.**

**Every corpus and census count I checked is right**: 8 174 025 scored cells at
`L = 11` and 180.6 per position; 1 720.11 and 40.43 observations per nominal
parameter; 108 075 codes at `L = 13` against 108 074 at `L = 11` and 232 classes
against 231; T2 at `L = 11` 1 533 observed codes, median 41, 322 at `≤ 4`, 620
below 20; `R-A5`'s 11 909 codes, median 7, 4 730 at `≤ 4`, 142 covering 90 % of
12 980 519.

**§1.2's throughputs are the receipt's** (`529 255 (526 479, 531 293)`, 564 304,
492 073, 453 012 / 73 388 nodes / 162 ms) and §1.3's six unreceipted digits are
labelled as such with nothing computed from them.

**§1.4 remains the most useful finding in the package** — the arena refuses a
movetime budget by name, the time-matched arm is a capability nothing here has, it
is constant across rows, and a constant column cannot rank a field.

**§6's readiness-versus-expected-value paragraph is still the best text in the
document**, and §14's *"the architect should read the disagreement, not the rank"*
(carried into the HANDUP) is the right instinct.

**And the HANDUP is written contingently** — *"if that round FAILs this HANDUP is
superseded by a STOP"* — so writing it before this verdict is not a breach.

---

## 5. VERDICT

# FAIL

**4 BLOCKING, 10 MAJOR, 13 MINOR.** Of round 3's twenty-one findings, **eight
MAJOR-or-above and eight MINOR still land in whole or in part**, and four of the
still-landing ones are reported as fixed in §14.

**D-709's four-round grant is spent. This is STOP AND SPLIT, with no fifth round
self-granted.**

The pattern is the one D-630/D-631 named and §8's own last row quotes — *"a fix
round discharges the finding's SENTENCE and re-creates its PROPERTY one step to
the left"* — and revision 6 does it five times:

- The nested null's correctness fix discharged *"permuted over the whole code
  space"* and re-created *"not matched on the property that drives the score"*
  over the scored population instead of the code space (**R4-B1**), pinned by a
  test that enumerates the population where it holds — which is the remedy §8
  offers against that very defect class.
- The correction that promoted `R-A4-CLASS` was applied at the row that gained and
  not at the row it also demotes (**R4-B3**).
- R3-B3's field fix priced `L = 7` and left `L = 9`, the third length in a row
  found missing from the same table (**R4-B4**).
- R3-M3's and R3-M4's fixes each missed the site round 3 printed by line number,
  and §14 reports both as swept (**R4-M3**, **R4-M4**).
- §5's floor fix for the new live lengths left §4 registering the old one, at a
  length the same cell rules out (**R4-M1**).

**The split proposal, which is round 3's shape and I endorse it.** What has failed
four times is not the whole matrix. §1 (premises, throughputs, books arithmetic,
the arena finding), §3 (the seed pilot), §4's row cells for `R-A5`, `R-A2`, `R-A3`,
`R-A1-L11`, `R-C-SPSA`, `R-D-W1`, §9 and §10's receipt discipline are finished and
have survived three or four independent re-derivations. **What has failed is one
thing: the evidence about `R-A4-CLASS` and the ranking built on it.** That should
go back with Stage E's own successor package — where `STOP_E` already sent §6.5
and §7.4 — and the rest should go up as a field with `R-A4-CLASS` marked
UNRANKED pending it.

**The shortest route, and eleven of the fourteen BLOCKING/MAJOR close by deleting
a sentence or printing a number the session already has:**

1. Delete *"matching at 1.00x everywhere"* / *"1.00 at every rung and length"*
   from the matrix, `STOP_E` and the memo banner; print the observed-population
   ratios (1.17x–1.83x) with the direction of their bias; restore the
   null-matching item to the successor package's ledger (**R4-B1**).
2. Delete the three *"stone counts explain as much"* sentences at lines 593, 810
   and 878, or restate them as `census_r5` says them (**R4-B3**).
3. Price `R-A1-L9F`, or say on the page why not, and delete the banner's
   two-missing-rows claim (**R4-B4**).
4. Say which measurement carries rank 2, mark §6.5's criterion superseded where it
   is quoted, and state that its PASS side is evidence at `L = 7` and not
   elsewhere (**R4-B2**, **R4-M10**).
5. Recompute §4's `R-A4-CLASS` floor and density cells at `L = 7` / `L = 9`
   (**R4-M1**).
6. Delete the second *"cheapest codebook traffic in the field"* (**R4-M2**).
7. Fix lines 880 and 882 to the currencies revision 4 established (**R4-M3**).
8. Delete *"its build cost is nil"* at line 852 and correct §14's row (**R4-M4**).
9. Correct §2.5's `t`-merge pairing to one population (**R4-M5**).
10. Rewrite §10's `census_r4` row and add `census_r5`; delete *"then at 24"*
    (**R4-M6**).
11. State which cell 13.3x is computed at, and print `L = 9`'s 0.24x beside it
    (**R4-M7**).
12. Narrow §14's and §10's claims for `receipt_digest_check.py` to what it does
    (**R4-M8**).

Only **R4-M9** needs an argument rather than a deletion, and after four rounds it
still does not have one.

---

## 6. Can this be handed to an architect?

**Not as it stands — and the single thing that stops it is that the two ranks the
matrix changed this revision are both built on a measurement its own package has
stopped as unsettled, and whose stated validity property is false at its own
receipt.**

Stated once, plainly: **the nested test cannot carry a rank, and two ranks rest on
it.** `R-A4-CLASS` moved from 4 to 2 because the nested figure went 0.93 → 1.71;
`R-A1-L7F` sits at 4 because the same `L = 7` cell is read the old way at three
sites. The test's null is still 1.17x–1.83x finer than the join it referees on the
population the statistic is computed over (**R4-B1**); its summary rule is still
unregistered; its defining section is marked SUPERSEDED by the memo's own banner
and sent back as an unsettled package by `wp22_phase2a_STOP_E.md`; and the
registered criterion beside it has its PASS side disclaimed by §2.5's own
paragraph. An architect reading Q1's rank order cannot tell which of those
supports rank 2, and the HANDUP's row-2 attack column already says the quiet part
— *"rewarding the row that happens to have been measured is a bias"*.

**What CAN be handed up today**, and it is most of the package: §1.4's finding
that D-705's time-matched arm is a capability nothing in this repository has;
§1.6's books arithmetic, which funds exactly one acceptance arm at Δ = 10 and is
cross-checked digit for digit against two of the project's own figures; §3's
measured seed budget; the throughput seats and the 2.8x play-seat disagreement;
the five obligations in the HANDUP's §2; and the two questions in its Q2 — may
`book_v3` fund a row that does not answer R7, and may the `book_v2` holdout be
spent on a screen. Those are settled, replicated, and useful.

**What must go back with Stage E's successor package**, because it is the same
question that failed four rounds there: the referent for *"does threat structure
carry value beyond stone counts"*, matched on every property that drives the score
— including the one the corpus, not the code space, determines — with a registered
summary rule and an argument that `ω²` over position-clustered labels is fit for
the question at all. Until that returns, `R-A4-CLASS` is a row with a real,
small, and not-yet-soundly-measured effect, and it should be handed up as
**UNRANKED** rather than second.

---

**Instruments and scratch**:
`/tmp/claude-1000/-home-tom-Projects-HeXO-AlphaBeta/38c87765-cb91-45a9-b301-2825923f8372/scratchpad/rt4/{derive.py,bad1..bad4.md}`
plus inline `python3` passes, all read-only against `artifacts/wp22_phase2a/` and
`tools/hex_enum/`. Live tree unmodified apart from this file. No `cargo`, no
worktree, no `CARGO_TARGET_DIR`, no commit. HEAD at the start of this review:
`695b6708dbb71dce1dd06ef6ded4b494eaf78efd`; at the end:
`1d3cc94ab7ac4296282a3952e61c51d647733eab` (one commit, adding the HANDUP only —
the subject file is byte-identical at both). `git status --porcelain` empty apart
from this untracked report.
