# SECOND DECISION-RED-TEAM — `matrix_wp22_phase2_eval.md` revision 3

## 1. Header

**Named revision**: `564dc9e4ff7682e01601dcf65ea372eb9470e739` (`dev`).
**Matches HEAD**: YES — `git rev-parse HEAD` returns
`564dc9e4ff7682e01601dcf65ea372eb9470e739` at the start and at the end of this
review. The COMMIT did not move; the working tree did, and it was not me. A
concurrent agent reviewing `hex_threat_enum_v1.md` has uncommitted modifications
to that file, to `tools/hex_enum/{census,report,test_hex_enum}.py`, and an
untracked `hex_threat_enum_v1_ROUND3.md`. **Nothing I re-derived is affected**:
every number below comes from `artifacts/wp22_phase2a/`, which is untouched and
whose seven receipts I verified with `sha256sum -c` at the end of the review as
well as at the start. The one place it matters is that quotations of
`hex_threat_enum_v1.md` in §2 of this report were read from the file as it stood
at revision 3 of that memo; a successor should re-read them at whatever that
agent commits. I wrote exactly one file: this one.

**What this document is.** A scoped confirmation under D-691: for each finding of
`matrix_wp22_phase2_eval_REDTEAM.md` (4 BLOCKING, 11 MAJOR, 10 MINOR at `0641504`)
I re-ran the ATTACK against revision 3 rather than checking whether a prescribed
sentence appeared. Every instrument below is mine; where the first report named a
script I wrote my own from the receipted data rather than copying its arithmetic.
Revision 3's new text is unreviewed and is attacked on its own account in §3.

**Instruments used, all read-only, live tree unmodified, no `cargo`, no worktree.**

| instrument | what it did |
|---|---|
| `sha256sum -c` in each of the seven `artifacts/wp22_phase2a/*/` directories | verified every `RECEIPT_*.sha256` file by file, then took the digest of each receipt file itself |
| `/tmp/…/rt2/paired2.py` (mine, written from the receipt's columns) | re-derived §3's paired table from `seed_pilot/pilot_L11.txt` — wins, paired mean, sd, `t`, Cohen's `d`, one- and two-sided sign-test `p`, half-range/sd/SE of the mean |
| a second inline `python3` pass over `census_r3/census_L{7,9,11,13}.txt` | re-derived all 16 window-unit criterion ratios AND all 16 code-unit ratios, plus observations per nominal and per observed code at every MET cell |
| an inline `python3` nps model, written from §1.8's stated arithmetic | reproduced every registered floor and ceiling, and computed the floors the field does not carry |
| `sed -n` at named lines of `docs/decisions.md`, `crates/pistol-arena/src/{config,schedule,openings,validate}.rs`, `crates/pistol-eval/src/{eval,handcrafted}.rs`, `crates/pistol-core/src/window.rs`, `docs/book_v{2,3}_ledger.md`, `docs/audit/repo_audit_2026-09.md` | every quotation and every code claim read at the line, never by search |
| `/usr/bin/grep` and `git ls-files`, output sorted where recorded (D-265) | residue checks for deleted claims, section pointers, and instrument tracking |

**All seven receipts verify `sha256sum -c` clean**, and the seven receipt-file
digests are exactly the ones §1.6 and §10 print:
`9bb8fc6f…` (census_r3), `aab7f4f6…` (census), `d7e33861…` (n6_mutants),
`f3518ad2…` (seed_pilot), `211b6e10…` (nps_seat), `d82fce37…` (sprt_power),
`7678648c…` (clockfix_confirm). MAJOR-2's dead digest is gone and the corrected
one is live.

**A record-keeping note, not a finding.** The matrix's banner calls revision 3
"ROUND 2 OF FOUR"; my dispatch calls this review round 3. D-709 states that at the
moment of the grant "the matrix's red-team gate had spent NONE, its first round
being in flight", which makes the matrix's own count the defensible one. Under
either count at least one round remains.

---

## 2. Confirmation table

Verdicts: **CLOSED** = I ran the attack and the defect is gone. **CLOSED BUT
MOVED** = the named defect is gone and a smaller one of the same shape is at the
new text. **STILL LANDS** = the attack reproduces.

### BLOCKING

| # | my attack | output | verdict |
|---|---|---|---|
| **B-1** | Read D-705 whole (`sed -n '1476p' docs/decisions.md \| fold -w 110 -s`), then grepped the matrix for the false claim, then checked §5's floor column against D-705's last clause. | D-705 contains, verbatim, *"AND THE ASYMMETRY IS DELIBERATE: two `h1` bars would double the openings a row costs, and D-568's reservation plus `book_v3`'s 8 500 cannot fund that for every row."* The matrix's new §5 text quotes it exactly and re-points §1.6 as the ADR's premise measured. `grep -n "does not say"` finds the old sentence only inside the historical quotation (line 620) and §12 (line 812). §5's table: five live rows, five floors; the four blanks are R-A4-CLASS (stopped), R-A1-L11 and R-D-W1 (killed), R-C-SPSA (killed, and it changes no per-touch work) — each with its reason in the cell. | **CLOSED** on the false claim and on the floor coverage. Three residuals below (§3 NEW-11, m-10, and the retained split arithmetic). |
| **B-2** | Wrote my own parser over the 24 per-seed rows of `seed_pilot/pilot_L11.txt` and computed paired differences from scratch; then read `tools/hex_enum/seed_pilot.py` at lines 200–225 to check the pairing at the code, not at the report. | `K=8 vs K=32: 8/8, mean 23472.2, sd 5167.6, t=12.85`; `K=8 vs K=64: 8/8, mean 36878.5, sd 7020.7, t=14.86`; `K=32 vs K=64: 8/8, mean 13406.3, sd 3145.8, t=12.05`. Identical to the matrix's table to the digit. At the code: `games = sorted({…})` is built ONCE from `walk(…)` **before** `for k in ks:`; inside, `order = list(games)` then `random.Random(1000 + seed).shuffle(order)`; `HOLDOUT` is a module constant. Split `s` is therefore the identical partition at every `K`. | **CLOSED.** The pairing claim is true at the instrument and the replacement table is exact. Four small residuals in §3 NEW-12. |
| **B-3** | Did not take "1 000 openings seat 1 000 pairs" on the page. Read the arena. | `crates/pistol-arena/src/config.rs:56` — *"each opening is exactly one pair, so the game cap is twice this and is derived rather than stated again (D-157)"*. `schedule.rs:37` `let total = openings.taken.len() * 2;` and `:136` `let opening = &openings.taken[index / 2];`. `openings.rs:211-231` refuses a repeated opening up to lattice symmetry — *"A repeated opening is a forced 1-1 pair"* — so an opening cannot be replayed. Power figures re-read from the receipt: 1 000 pairs / elo1 30 / truth 30 → `h1 18378 (0.9189)`; truth 0 → `h0 18344 (0.9172)`, so the veto's failure at truth 0 is 8.28 %; 500 pairs / elo1 40 → 0.9067; elo1 50 → 0.9467; 8 000 pairs / elo1 10 → 0.9045; 4 000 → 0.6970. Every cell of §1.6's table is the receipt's. | **CLOSED.** The false finding against D-653 is deleted and the replacement is right at the code. One residual: §1.6's column header (§3 NEW-11). |
| **B-4** | Re-derived §2.5's whole criterion table myself from the receipted `census_r3` files — all four lengths, all four rungs — rather than reading `hex_threat_enum_v1.md` §7.4. | My 16 ratios: L7 1.014 / 1.014 / 0.793 / 0.686; L9 1.108 / 0.760 / 0.673 / 0.533; L11 1.125 / 0.651 / 0.637 / 0.510; L13 0.834 / 0.483 / 0.472 / 0.378. MET at 4. Every figure the matrix prints is exact, and its transcription of §7.4 is faithful. | **CLOSED** on the re-pointing and the transcription. **But §2.5's SUMMARY of that table is false and its scope statement is incomplete — see NEW-1 and NEW-2, which are new BLOCKING/MAJOR against revision 3.** |

### MAJOR

| # | my attack | output | verdict |
|---|---|---|---|
| **M-1** | `cat nps_instrument.txt` / `cat nps_play.txt` and compared every printed figure. | `nps_instrument.txt`: `band all: nps median 529255 (min 526479, max 531293, n=5)`; `band early 564304 (559064, 568030)`; `band late 492073 (491110, 494010)`; `depth_turns 2 band all: total ms median 162 nodes 73388 nps 453012`. **Every one of §1.2's cells is now the receipt's, to the digit**, and 531 548 is reported as the replicate (0.4333 % away — I checked the division). §1.3 is different: it prints `172 627 / 172 665 / 172 211` and `478 718 / 480 449 / 482 105`; the receipt holds `171 851 / 171 625 / 172 499` and `482 274 / 481 548 / 482 127`. None of §1.3's six numbers is in the receipt. The matrix says so in bold and labels them the first execution's; the receipted ratio is 2.8055 and the receipted depths 1.21 / 3.42 are exactly what §1.3 claims reproduce. | **§1.2 CLOSED. §1.3 CLOSED BUT MOVED** — the numbers are still unreceipted but are labelled, nothing is computed from them, and the receipt is anchored by digest. **§12's row about it is false — NEW-6.** |
| **M-2** | `sha256sum artifacts/wp22_phase2a/sprt_power/RECEIPT_sprt_power.sha256`. | `d82fce37add53453423e264a6220e2b47636302c7f2980c4a60f6c600005709b` — the digest §1.6 and §10 both now print. `37a86fe1…` appears nowhere. | **CLOSED.** |
| **M-3** | Read `repo_audit_2026-09.md` row A-02 at the line; re-implemented the nps model from §1.8's own arithmetic and recomputed every bracket. | A-02 lists `31.77% HandcraftedV0::delta`, `6.64% HandcraftedV0::undo`, `6.29% HandcraftedV0::apply` — sum **44.70**, and `13.45% ThreatState::touch` is the next entry. My model at base 529 255, share 0.4470: 33 touches, `c ∈ [0.5,1.5]` → **[296 959, 549 733]**; 24 touches → **[365 760, 621 921]**; 18 touches, `c ∈ [1.0,2.0]` → **[365 760, 529 255]**. Every registered floor and ceiling in §4 reproduces exactly. The `c`-bracket ground MAJOR-3 asked for is now stated in R-H-EXT's own cell. | **CLOSED** on the share, the floors and the bracket disclosure. Residuals: NEW-9 (the stated REASON for 44.70 % is false at the code) and NEW-13 (R-H-EXT's floor cannot fire). |
| **M-4** | Checked each row's cell against the new currency table's assignment, and recomputed each density in the currency the table assigns it. | R-A2 ✓ (`45 271/14 160 = 3.197`). R-A5 ✓ but uses a fifth unit the table does not list. **R-A1-L11 ✗**: the table says "positions per parameter", the cell prints "1.4 unfolded" — `45 271/62 370 = 0.73`; **1.4 is `62 370/45 271`, the reciprocal**. **R-A3 ✗**: the table says "positions per parameter", the cell says "1.2 **at §0.2's unit**", and §0.2's unit is window observations, where A3's density is `16 613 729/38 983 = 426`. | **CLOSED BUT MOVED.** The currencies are named; the table misnames two of its own rows and omits a fifth the ranking uses. **NEW-3.** |
| **M-5** | Checked whether §6 now ranks on a bench quantity or on the arm. | §5 separates them explicitly and §6's rank 1 cites traffic (18 vs 24 or 33) — a bench quantity this session measured. §6's "WHAT WOULD REORDER THIS" now concedes the arm's discriminating power. | **CLOSED BUT MOVED.** Residual: §5's headline *"THE COLUMN THAT WAS MEANT TO SEPARATE THE ROWS IS CONSTANT"* still runs "the value is constant today" together with "the axis cannot discriminate"; §6's own reorder clause says the opposite. Minor. |
| **M-6** | Grepped every site that priced R-H-EXT's cost. | §4's books cell is fixed and now names the holdout's two claimants and D-568. §6 is fixed and equalises book cost. **§7's table is not**: its column is still headed *"cost before it can be run"* — the axis §6 says revision 2 used and that was false — and its R-H-EXT cell still reads *"none beyond writing the terms"*. | **STILL LANDS in §7.** The fix landed at two of three sites; D-424 part 3's named defect ("the second site is what gets missed"). |
| **M-7** | `grep -n "only row"`. | The claim survives only as a quotation of its own deletion, in §4 and §12. | **CLOSED.** |
| **M-8** | `grep` for the numpy clause. | Deleted, with the correction stated and the reason for deleting rather than repairing given. | **CLOSED.** |
| **M-9** | `grep -n "4 %"` and read §6's closing. | The transferable-4 % sentence is gone. §2 item 4 carries "by 1.4 % to 12.5 %" and §2.5's table carries L13's 0.834. §7.4's non-transfer clause is reproduced in §2.5. | **CLOSED.** |
| **M-10** | Computed the code-unit criterion table myself for all 16 cells; grepped the matrix for `816`, `N-4`, `N-5`. | §2 item 4 now names the referent and the unit ✓. But **the code unit is under-reported — five cells exceed the referent, not one (NEW-2)**. And `816`: line 278 still reads *"`k` is COMPUTED and no cell of it is 816"* with no qualification, while `hex_threat_enum_v1.md` §7.1 now says in its own text *"§6.4's coarsest faithful clip would give `k = 16` and 816, and is not shipped"*. `N-4` and `N-5` appear nowhere in the matrix. | **HALF CLOSED, HALF STILL LANDS.** The unit/referent half is closed; the N-4/N-5 half is untouched and the headline stands unqualified against its own source's disclosure. |
| **M-11** | Checked R-A1-L8F's every figure against `eval_families_2026-09.md` §0.2 and §A6, and looked for anything that excludes it. | §0.2 L8 folded: **2 920** cells, median **273**, **24 (0.8 %)** under ten, **293** covering 90 %, ceiling **3 320**, **88.0 %** — every figure the row prints. `(3^8+3^4)/2 − 1 = 3 320` ✓. §A6 gives `3L = 24` at L8 = 1.33x ✓. Floor 365 760 (0.691x) reproduces. Nothing excludes it: §0.2 measures L8 explicitly, the even-length refusal in the enum memo §3.1 is about the CELL-CENTRED construction and this row is window-indexed, and §0.2's own caveat about centred-window variants does not bite. Its kill is honestly stated and its attack on that kill is faithful — `eval_families` §8 still reads *"Minutes of compute, **not run** (D-291)"* at HEAD, and the enum memo §6.6 does say in terms that §7 does not close `THM-WINDOW`. | **CLOSED. The row is legitimate and correctly priced.** |

### MINOR

| # | my attack | output | verdict |
|---|---|---|---|
| **m-1** | `grep -n '^#\{2,3\} '` for the section list; read line 29. | Sections are §1–§10 and §12. Line 29 still reads *"§11 is the failed-precedent check"*. There is no §11; the check is §8; and §12 exists, so the numbering has a real hole. | **STILL LANDS**, unchanged. |
| **m-2** | Grepped every `§6.x` / `§7.x` reference. | Line 14 and line 308 now say `hex_threat_enum_v1.md §7.4` ✓; line 344 says "§6.5 of the enum memo" ✓; line 504 attributes "§6.6" to Stage E ✓; line 775 ✓. **Line 747 still reads a bare "§6.5's purity criterion"** in a document that has no §6.5. | **STILL LANDS**, reduced to one site. |
| **m-3** | Recomputed the spread statistics from the pilot receipt. | half-range/mean = 2.790 % (K=8), 3.457 % (K=32), 3.458 % (K=64); sd/mean = 2.00 / 2.34 / 2.25 %; SE of the mean of 8 = 0.71 / 0.83 / 0.80 %. The document still carries "**±3.5 %**" tagged `(MEASURED, §3)` into two row cells, with the arithmetic still off the page. | **STILL LANDS**, unchanged. |
| **m-4** | `ls configs/` and read the third file's header. | `configs/play_staged_solver_v0.toml` is committed with `mode = "play"`. §1.3 still says *"There are two committed play seats"*. Its own header calls it a MEASUREMENT seat, which is what makes the sentence defensible — and §1.7 still takes its answers-per-game figure from `sealbot_anchor_v7_protocol.md`, whose runs are on that seat. | **STILL LANDS**, unchanged and still minor. |
| **m-5** | Read §1.7 whole. | Bullet 1: 8 500 openings is **4.83 h** (I checked: `8500 × 2.046 = 17 391 s = 4.831 h`). Bullet 3 still quotes *"one acceptance SPRT at Δ = 10 is 2-3 hours at the instrument seat"*. Still unreconciled, still eight lines apart, and `book_v3_registration.md` §R1's `mean_pairs 3102.1` — the reconciliation — is still uncited here. | **STILL LANDS**, unchanged. |
| **m-6** | Compared §1.7's per-answer figure with §1.3's own receipt. | §1.7 still says "500 ms per answer **configured**"; `nps_play.txt` measures `11 502 ms / 24 = 479 ms` and `10 978 / 24 = 457 ms`. D-291's class, with the measurement inside the same package. | **STILL LANDS**, unchanged. |
| **m-7** | Recomputed the two disputed rows of the arithmetic pass. | `matrix_wp22_phase2_eval_ARITHMETIC.md` still lists `SPSA openings against book_v3 \| 7.176x \| 7.1x` and `SPSA wall \| 34.67 h \| 34.4 h` under "The 28 that reproduced". `61 000/8 500 = 7.176` and `61 000 × 2.046 = 34.67 h`; `60 500/8 500 = 7.118` and `60 500 × 2.046 = 34.38 h`. Both re-derived columns are the pre-correction values. | **STILL LANDS**, unchanged. |
| **m-8** | Compared the L7 T4 and T3 rows of `census_r3`. | Identical: `k = 25` both, 16 window classes both, ω² `0.003573` both. Fifteen distinct partitions counted as sixteen. **And it now matters more than it did**: two of the four MET cells are that one partition, which is why §2's "all four of them T4" is inexact (one is T3) — see NEW-1. | **STILL LANDS**, and it is now load-bearing. |
| **m-9** | `git ls-files \| grep -E "measure_nps\|measure_play\|confirm_wedge"`. | Returns nothing — all three exist only inside gitignored `artifacts/` directories. Line 752 still claims *"every instrument here is committed under `tools/hex_enum/`, driven by two suites through CI gate 19"*, of a §10 table that lists five instruments, three of them untracked and none of those three under `tools/hex_enum/`. **`measure_nps.sh` produces 529 255, from which revision 3 recomputed EVERY registered nps floor** — so the coverage gap is more load-bearing at revision 3 than it was at revision 2. | **STILL LANDS**, and it is worse. |
| **m-10** | Grepped the matrix for D-653's flip clause. | *"flips if `book_v3`'s extension makes a holdout redundant"* appears nowhere; the only `flips if` in the matrix is D-705's. And line 753 still claims *"the two ADRs this matrix leans on hardest — D-653 and D-705 — are quoted whole rather than glossed"* — false of D-653 (flip dropped) and of §5's own "D-705, whole:" label on a one-sentence quote. §6's *"a second acceptance book moves R-A2 up"* is still that unconnected flip. | **STILL LANDS**, unchanged. |

**Score on the confirmation: 4 BLOCKING closed as stated (B-4 with a new
successor), 8 MAJOR closed, 1 MAJOR closed-but-moved with a new finding, 1 MAJOR
half-closed, 1 MAJOR still landing; 10 of 10 MINORs still land, 8 of them
untouched.**

---

## 3. NEW findings against revision 3

### NEW-1 (BLOCKING). §2 and §2.5's central conclusion is false on the matrix's own receipted census, and the counter-example is a rung the matrix prints on its own page

§2.5 states, in bold, the sentence that disposes of `R-A4-CLASS`:

> **The only rung that beats counting stones cannot be fitted, and every rung that
> can be fitted loses to counting stones.**

and §2 item 4 states:

> **MET in only 4 of 16 cells — all four of them T4, the full tuple, and by 1.4 %
> to 12.5 %** — while every affordable rung is BELOW a quotient that counts stones
> and nothing else.

**My reproducer**, from the receipted `census_r3` files and nothing else — the
`k`/`C(k+2,3)` block, the `scored cells, total:` header line, and the
`code enum T4` observed-class count at each length:

| MET cell | nominal params `C(k+2,3)` | scored cells | obs / nominal param | observed codes | obs / observed code | traffic `3L` | criterion |
|---|---|---|---|---|---|---|---|
| **L = 7, T4** | **2 925** | 5 031 327 | **1 720.1** | 856 | **5 877.7** | **21 = 1.17x** | **1.014 MET** |
| L = 7, T3 | 2 925 | 5 031 327 | 1 720.1 | 856 | 5 877.7 | 21 = 1.17x | 1.014 MET |
| **L = 9, T4** | **161 700** | 6 537 654 | **40.4** | 21 371 | **305.9** | **27 = 1.50x** | **1.108 MET** |
| L = 11, T4 | 7 647 059 | 8 174 025 | 1.07 | 108 074 | 75.6 | 33 = 1.83x | 1.125 MET |

**Two of the four MET cells are affordable, and one of them is spectacularly so.**
`L = 7` T4 costs **2 925 nominal parameters** — the matrix's own §4 cell prints
that number, *"At `L = 7`: T4 2 925"* — against **5 031 327** scored-cell
observations. That is `1 720` observations per nominal parameter and `5 878` per
observed code, against Buro's ≥ 20 safe-fit line and `eval_families` §A1's 1.2
kill, both of which the matrix uses as its density bar. `L = 9` T4 is `40.4` per
nominal parameter, still above Buro's line. So the sentence is false at two cells
and the §2 item 4 clause is false at the same two.

**On the matrix's own nps model this row would carry the field's best floor.** At
`3L = 21`, share 0.4470, `c ∈ [0.5, 1.5]`, base 529 255:

| row | traffic | floor | ceiling |
|---|---|---|---|
| **R-A4-CLASS at L = 7 T4** | 21 (1.17x) | **396 371 (0.749x)** | 650 390 (1.229x) |
| R-A1-L8F | 24 (1.33x) | 365 760 (0.691x) | 621 921 (1.175x) |
| R-A5 / R-A2 / R-A3 | 33 (1.83x) | 296 959 (0.561x) | 549 733 (1.039x) |

**Where the over-read comes from.** The enum memo scopes its own claim and the
matrix drops the scope. `hex_threat_enum_v1.md` §7.1 is headed **"THE FULL TUPLE
AT THE COVERING LENGTH IS NOT AFFORDABLE"** and prices 1.07 there. §2.5 turns "at
the covering length" into "cannot be fitted", full stop. And §6.5's registered
criterion binds *"at the rung and length **a matrix prices the row at**"* — the
length is the matrix's own choice, which is exactly what it declines to make.

**And the exclusion is unstated.** `grep -n "covering"` over the matrix returns
six hits: one is the L11-vs-L13 argument, five are about `R-A1-L8F`. Nothing in
the matrix says shorter lengths are out of scope for `R-A4-CLASS`. If the intended
ground is `eval_families` §0.1's covering minimum, then the matrix cannot also
rank `R-A1-L8F` **third** on the explicit ground that *"the boundary that excludes
the densest affordable free table in this field is an unrun theorem"* — the same
boundary, treated as binding for one row and as an unrun estimate for another,
three sections apart.

**What it licenses, and this is why it is BLOCKING.** §6 rank 6 puts
`R-A4-CLASS` with the killed rows, §5 registers no arm and no floor for it, and §7
says it is "out of the priced field" — all on a sentence that the matrix's own
receipt contradicts. `R-A4-CLASS` at `L = 7` T4 is, on the matrix's own columns,
the only learned row in the entire field that has **passed a registered criterion
that could fail**; every other learned row's kill condition is unmeasured. The
architect reads this document and never sees that.

*(The enum memo's §7.4 makes the same unscoped claim — "IT FIRES ON EVERY
AFFORDABLE RUNG" — while its own §7.1 scopes it. That is that document's finding
and its concurrent reviewer's; it is reported here because the matrix is
responsible for what it concludes from a source, not merely for transcribing it.)*

### NEW-2 (MAJOR). §2.5 says one code-unit cell "looks like the opposite"; five do, and one of them flips the criterion's verdict between units

§2.5:

> **AND ONE CELL LOOKS LIKE THE OPPOSITE AND IS NOT**: at the CODE unit at
> `L = 11`, T4's `ω²` is 0.020985 against the stone-count quotient's 0.009248 —
> 2.27x

**My reproducer**: the same script that produced the window table, run over the
`code` rows of the `## purity` block in all four `census_r3` files.

| L | rung | enum `ω²` | count-only `ω²` | ratio | code unit | window unit |
|---|---|---|---|---|---|---|
| 7 | T4 | 0.007470 | 0.006552 | **1.140** | MET | MET |
| 7 | T3 | 0.007470 | 0.006552 | **1.140** | MET | MET |
| 9 | T4 | 0.012735 | 0.007863 | **1.620** | MET | MET |
| 11 | T4 | 0.020985 | 0.009248 | **2.269** | MET | MET |
| **13** | **T4** | **0.017104** | **0.010455** | **1.636** | **MET** | **FAILS (0.834)** |

Five cells exceed the referent at the code unit, not one. And `L = 13` T4 is a
**sign flip between the two units of the same instrument** — the cell the criterion
FAILS at the window unit is a 1.64x pass at the code unit. §2.5 prints the L = 11
cell, explains why the code unit is not the binding one, and does not say there
are four more or that one of them disagrees with the window unit's verdict.

This is `wp22_phase2a_STOP_E.md`'s own N-9 — *"the criterion binds one unit while
the instrument prints two"* — and it is MAJOR-10's first half re-created inside
MAJOR-10's remedy. The document's §8 row for the meta-pattern claims exactly this
does not happen here: *"a fix round discharges the finding's SENTENCE and re-creates
its PROPERTY one step to the left"*.

### NEW-3 (MAJOR). §4's new currency table — the remedy for MAJOR-4 — misnames two of its own rows and omits the unit §6 ranks on

**(a) `R-A1-L11`'s "1.4" is the reciprocal of the currency the table assigns it.**
The table says the row's currency is *"positions per parameter … one of the 45 271
quiet positions"*. The cell prints *"**1.4** unfolded, **1.2** folded"*.

```
45 271 / 62 370 = 0.7259   ← positions per unfolded parameter
62 370 / 45 271 = 1.3777   ← parameters per position  ("1.4")
45 271 / 38 983 = 1.1613   ← positions per folded parameter  ("1.2")
```

The two numbers in one cell are in reciprocal units, and the table asserts both are
the same one. The error flatters: the honest figure for the unfolded table is
**0.73**, not 1.4. (The kill still fires — 0.73 is worse — so no conclusion moves;
the currency table's accuracy claim is what falls.) The source, `eval_families`
§A1, carries the same slip; the matrix's new table is what asserts it is right.

**(b) `R-A3`'s cell contradicts the table.** The table assigns R-A3 *"positions per
parameter"*; the cell says *"**1.2** at **§0.2's unit**"*. §0.2's unit is window
observations — the matrix's own R-A5 cell says so (*"at §0.2's unit, 16 613 729
window observations"*). At §0.2's unit A3's density is `16 613 729 / 38 983 = 426`,
not 1.2. One of the two labels is wrong and they are eight lines apart.

**(c) The table names four currencies; the document prices on five.** The census's
cell-centred window unit — **12 980 519** observations over 11 909 codes — is not in
the table, and it is the unit §6's rank-2 sentence uses:

> **Its parameter count is 811 or 143** … *both counts are affordable against
> **12 980 519** window observations*

811 is measured at §0.2's unit (16 613 729 observations); 143 at the cell-centred
unit (12 980 519). The sentence sets one row's §0.2-unit parameter count against
the other unit's observation total. So §4's own claim —

> the ranking in §6 does not rest on any cross-currency comparison

— is falsified by §6's rank-2 justification. (Both readings are affordable —
`16 613 729/811 = 20 486` and `12 980 519/143 = 90 773` — so rank 2 survives; the
no-cross-currency claim does not.)

### NEW-4 (MAJOR). §1.5 quotes two of `book_v3_ledger.md`'s three relevant sections and omits the one that carries a standing claim on the whole book

§1.5 quotes the ledger's reuse rule verbatim and reports:

> And its consumed-ranges table reads **`nothing yet`** — the book is whole.

**My reproducer**: `sed -n '84,90p' docs/book_v3_ledger.md`. The section
immediately after the one §1.5 quotes is headed *"Standing claims on the book, not
yet consumed"*, and its single row is:

> | The R7 acceptance SPRT | **THE REASON THIS BOOK EXISTS** (D-638, D-643) |
> 8000 pairs at the registered bounds; the book covers it with 500 openings to
> spare |

The matrix mentions R7 exactly once, in `R-D-W1`'s cell — *"what answers R7
instead | the selected family's acceptance SPRT"* — and never connects it to
`book_v3`. D-704 defines R7 as *"the selected family's acceptance SPRT"*.

**Why it is load-bearing at revision 3 and was not at revision 2.** §6's whole new
equalising move is:

> **Book cost is now EQUAL across every live row** — one `book_v3` arm — so it
> cannot rank them

That equality requires `book_v3`'s single arm to be available to `R-H-EXT`. The
ledger says the arm is claimed, by name, for the selected family's acceptance —
and §6 says of `R-H-EXT`, three paragraphs later, *"R-H-EXT is arguably not a
Phase-2 row … a package of hand terms is Stage-0 work nobody did"*. This is exactly
MAJOR-6's remedy — carry the ledger's claimants table — applied to `book_v2`'s
ledger and not to `book_v3`'s, in the section that quotes `book_v3`'s ledger. It is
also the first report's QUESTION 4, which revision 3 does not answer and which the
narrowed axis turns from a question into a premise.

### NEW-5 (MAJOR). §6's new axis does not answer the question §6 says it is answering

§6 opens:

> The books fund one acceptance run at Δ = 10 (§1.6 …). So the field's real
> question is not "which row is best" but **"which row is run first"**, and the
> ranking below is by **what a row costs to BUILD before any book is spent**

Both halves are individually fine. Together they do not connect. §1.6 point 3 is
explicit that there is **one** run:

> **So the field is priced against ONE acceptance run.** Whichever row the
> architect picks, the books do not fund a second row's acceptance without a new
> book.

If exactly one acceptance run is funded, then "run first" is "run, full stop", and
the ordering question collapses into the selection question D-708 forbids this
document from answering. Build cost ranks what to do first only when doing it
first does not preclude the rest — and §1.6 measures that it does. So ranking
`R-H-EXT` first is, on the matrix's own arithmetic, a recommendation to spend the
only funded acceptance arm on a row the matrix itself says may be out of Phase 2's
scope, after which no learned row — the entire subject of this matrix — can be
accepted on the committed books.

The matrix comes within one sentence of saying this twice (*"the selection is close
to irreversible on the committed books"*; *"a second acceptance book moves R-A2
up"*) and never says it. An architect who follows §6's order and §1.6's arithmetic
reaches a state this document does not describe.

### NEW-6 (MAJOR). §12's round table claims two fixes that did not happen

**(a) M-1's row**: *"§1.2 and §1.3 now print the RECEIPTED run (**529 255**), with
the other execution reported as the replicate."* §1.3 does the opposite: it prints
the first execution's six numbers and reports the RECEIPTED run as the one it does
not print. §1.3 is scrupulous about saying so in bold; §12 describes it as if it
were §1.2. A reader auditing the fix round from §12 concludes both sections were
corrected; one was.

**(b) M-4's row**: *"§4 opens with the currency table and **names each cell's
own**."* Two cells' own currencies are misnamed (NEW-3 a and b).

Under this project's own standard — a fix-round table is the artefact a successor
audits against — a row that overstates a remedy is the D-479 class in a document
rather than in a measurement.

### NEW-7 (MAJOR). §7 retains the axis and the cell §6 disavows

§6, new text:

> Revision 2 ranked by "cost before it can be run at all" and then priced
> `R-H-EXT`'s book cost at nothing, **which was false (§4)**.

§7, unchanged:

> | row | **cost before it can be run** | failure mode |
> | R-H-EXT | **none beyond writing the terms** | … |

The disavowed column heading and the disavowed cell are both still there, in the
one-line-per-row summary an architect is most likely to read as the conclusion.
This is the same finding as MAJOR-6, at the site the fix missed, and D-424 part 3
names the mechanism: *"a claim the document makes twice is a defect waiting …
the second site is what gets missed."*

### NEW-8 (MAJOR). `R-H-EXT`'s build cost is priced at nil, and the row has no procedure and no metric for choosing its 3–6 integer values against one funded run

§6 rank 1: *"Nothing to build: no fit, no artifact, no digest discipline, no shape
check, no seed, no quantization, no new dependency."* §4: *"**~3 to 6 more
integers**"*, *"the terms are **hand-set** and SPRT-gated"*, *"seed budget:
**zero**. There is nothing to seed."*, *"observations per parameter: **not
applicable**"*.

Setting 3–6 integers by hand is choosing one point in a 3-to-6-dimensional integer
space, and the field has exactly **one** funded acceptance run (§1.6). The matrix
prices that structure as a cost for one row and at zero for another:

- **`R-A2` is ranked LAST partly for it**: *"**≥ 4 trained nets** … The books fund
  ONE acceptance run, so four nets cannot each be accepted and the row must choose
  one by an offline metric"*.
- **`R-A2` at least has a metric.** §3's new paired analysis measures that
  validation loss separates candidate sizes at `t = 12`–15 on this corpus.
- **`R-H-EXT` has none.** Its own cell says observations per parameter is not
  applicable; there is no fit, no label, no validation number of any kind.
- **And the field already prices the alternative and kills it.** `R-C-SPSA` — the
  row that exists precisely because setting eval integers costs something — is
  killed at 60 500 openings and 34.4 h. So `R-H-EXT`'s new integers are either
  guessed once, or set by the procedure this matrix kills on cost. The matrix never
  connects the two rows.

§6's rebuttal of the first red team turns on *"its build cost is nil"*. That
premise is not established.

### NEW-9 (MINOR). §1.8's stated reason for the 44.70 % correction is false at the code

> all three are the eval — `delta`'s default body IS the apply/undo roundtrip
> (`crates/pistol-eval/src/eval.rs:87-92`) and the two appear separately because
> the search's own seam calls them directly.

`crates/pistol-eval/src/handcrafted.rs:302` — **`HandcraftedV0` OVERRIDES
`delta`** with a read-only incremental body (`for window in windows_through(at)`,
`self.contribution(after) - self.contribution(before)`, *"this body has mutated
nothing"*). It never calls `apply` or `undo`. The trait default at `eval.rs:87-92`
that the matrix cites does not run at the profiled seat at all.

**The number 44.70 % is right** — all three symbols are `HandcraftedV0` methods
and `perf report --no-children` reports self time, so the sum is a clean partition
with no double count, and the override makes that *more* certainly true, not less.
What is wrong is the reason given for the correction that reset every registered
abort threshold in the document.

### NEW-10 (MINOR). §4's R-A4 kill cell cites §2.5 for a figure §2.5 does not contain

> The registered purity criterion was the row's kill, and **§2.5 measures that a
> value-free quotient of stone counts passes it at 1.77x**.

`grep -n "1\.77"` over the matrix returns that line and three unrelated hits on
`31.77 %`. §2.5 contains no 1.77x and measures no such thing: §2.5's table is the
NEW criterion (enum against stone counts). 1.77x is `hex_threat_enum_v1.md` §6.5's
figure against the OLD, random-permutation criterion. The cross-reference points
at a section that carries nothing — round-1 M-7's own class — inside the cell that
states why the row is out.

### NEW-11 (MINOR). §1.6's column header now describes only one of its six rows

The header is *"cap by the openings rule"*. B-3's remedy establishes that the
inversion of `ceil_to_500(P + 500)` is **not** how an existing book seats pairs,
and the screen rows now read `1 000 openings → 1 000 pairs` and `half the holdout →
500 pairs`, which is physical seating. The `book_v3` row's 8 000 is the registered
`P` from `book_v3_registration.md` §R1, not an inversion either. So no row of the
table is a cap "by the openings rule", and the header names the exact rule the
section's own text says was misapplied.

### NEW-12 (MINOR). Four small statistical looseness's in §3's replacement table

- The *"better on"* column reads `8 of 8 splits` for a row headed `K = 8 vs
  K = 32` without naming **which** side wins. It is inferable from the val-MSE
  column above and stated in §12, not here.
- *"at 5.25 sigma for the widest"* — 5.253 is `mean/sd` (Cohen's `d`), not a
  significance level. The `t` is 14.86. Printed beside a `p`, `5.25 sigma` reads as
  a `p ≈ 10⁻⁷` claim.
- *"sign-test `p = 0.0039`"* is the **one-sided** figure (`0.5⁸`); two-sided is
  0.0078, and the direction was not pre-registered.
- *"§3's paired analysis measures that validation loss separates **table sizes**"*
  is used in §4 to make `R-A2`'s kill — *"it does not beat **A3's** validation
  loss"* — a live test. The pilot measured three sizes of ONE architecture; A2 vs
  A3 is two architectures. The instrument's resolution (~2.1 % at `t = 12`) makes
  the step reasonable; it is a step the pilot did not take, and the word
  "measures" carries it as if it had. (Nothing in the order moves either way.)

### NEW-13 (MINOR). `R-H-EXT`'s registered nps floor cannot fire, and the same cell says why

The cell registers `c ∈ [1.0, 2.0]` — a per-touch eval cost up to **double**
`handcrafted_v0`'s — giving a floor of 365 760 (0.69x). The same cell says the
added terms are *"~3 to 6 more integers"* and that *"the exact-`t` counters are
computed by `PROTO-NODE` already … so the counters are **close to free**"*. A row
whose honest expectation is ≈ 1.00x and whose abort threshold is 0.69x has an
abort threshold nothing can trip. Hard rule 5's abort threshold is the one number
in a bench registration that must be able to fire; §8's own row for `3 m-1`
credits this document with naming checks that pass vacuously.

---

## 4. What I attacked and it SURVIVED

I want to be as specific about what is finished as about what is not, because a
great deal of revision 3 is genuinely repaired and the repairs were made the hard
way.

**The receipts are exemplary.** Seven directories, `sha256sum -c` clean file by
file, and every digest the document prints is the digest of the receipt file on
disk. MAJOR-2's dead digest is the class of error this discipline exists to catch,
and it was caught and fixed.

**§1.2 is now what MAJOR-1 asked for and more.** Every cell is the receipt's, the
replicate is reported as a replicate with its 0.43 % distance, and the `depth_turns
2` row's 162 ms is the receipt's (revision 2 printed 163). I tried to find one
figure in §1.2 that is not in `nps_instrument.txt` and there is none.

**The floor arithmetic is one formula applied consistently and it is right.** I
re-implemented it from §1.8's own words and reproduced 296 959 / 549 733,
365 760 / 621 921 and 365 760 / 529 255 to the unit. The 44.70 % share is the
audit's own three lines summed, and `ThreatState::touch` at 13.45 % is correctly
excluded.

**B-2's replacement is exactly right and it is the best repair in the document.**
I wrote my own paired analysis and got the same nine numbers, and then I checked
the linchpin at the code rather than at the report: the game list is built before
the `K` loop and the shuffle is keyed on the seed alone. Revision 3 did not just
adopt the reviewer's table — it re-derived the guard, restored `R-A2`'s kill to a
live test, and explicitly refused to let the result corroborate D-614. That last
refusal is the kind of thing a fix round usually gets wrong.

**B-3 is right at the code and not merely at the ADR.** I went to
`config.rs:56`, `schedule.rs:37,136` and `openings.rs:211-231` rather than
accepting "a paired opening is played twice". Each opening is exactly one pair,
the game total is `taken × 2`, and a repeated opening is refused by name. The
first report's QUESTION 2 is answered.

**§2's census transcription is exact.** I re-derived the `k` ladder, the code
counts, the observed-class counts at both units, the medians, the Buro-line counts,
the growth curves and both the L13-vs-L11 comparisons. *"Every rung's `ω²` is LOWER
at 13 than at 11"* holds at **both** units, 8 of 8. `108 075` against `108 074` and
`232` against `231` are right. `18 %` more traffic is `39/33 = 1.1818`.

**§2.5's own criterion table is exact.** All sixteen ratios reproduce to three
decimals from the receipted files by an independent computation. My findings
against §2.5 are about what it concludes, not about what it measured.

**`R-A1-L8F` is a legitimate row, correctly priced, and adding it was right.**
Every figure checks against `eval_families` §0.2 and §A6; the ceiling arithmetic
is right; nothing excludes it that the matrix missed; its kill is stated and its
attack on that kill is faithful to both `eval_families` §8 and the enum memo §6.6
at HEAD.

**The power figures are the receipt's, and the cross-check is a real anchor.**
`--pairs 8000 --elo1 10 --truth 10 → h1 18090 (0.9045)` is digit for digit
`book_v3_registration.md` §R1's committed figure. I checked the closed form by
hand at Δ = 5, 20 and 150 and `ceil_to_500` at every cell, including the ones
already on a 500 boundary — the class the author's own arithmetic pass caught
itself on.

**Every quotation I checked resolves at the line.** D-705 (1476), D-653 (1372),
D-644 (1354), D-704 (1474), D-709 (1484), `validate.rs:45-51`, `window.rs:9`,
`eval.rs:87-92`, `book_v2_registration.md` §4's `+500` definition, `book_v3_ledger`
lines 66–72 and 84–90, `repo_audit_2026-09.md` A-02. I found no misquotation
anywhere in revision 3 — the errors I found are inferences from correct
quotations.

**M-6/M-7/M-8/M-9 are properly deleted, not argued with.** Each is gone from the
text and survives only as a quotation of its own deletion, with the reason stated.
The numpy correction in particular deletes rather than repairs, on the right ground
(*"which interpreter is on PATH is not a property of the row"*).

**And the structural act is still right.** D-708's separation holds, nothing is
selected, no ADR of selection is written, and the strongest surviving attack is
carried in §6 rather than buried. §1.1's corpus counts are re-confirmed by the
receipted pilot log (`walked 45271 quiet rows over 3487 games, 38983 distinct
folded L11 codes`) — that section remains the best in the package.

---

## 5. VERDICT

# FAIL

**One new BLOCKING, seven new MAJOR, five new MINOR; one MAJOR from round 1 still
landing at a site the fix missed, one half-landing, and ten of ten MINORs still
landing (eight untouched).**

Revision 3 did the hard part well: every one of the four BLOCKINGs is closed at its
own claim, three of them by deletion, and the two replacements (the paired analysis
and the openings-seat-pairs correction) are right at the code and not only at the
page. What fails is what the first report predicted would fail — **not arithmetic,
but what the document concludes from arithmetic that is correct.** The single
BLOCKING is the sharpest instance: the census the matrix receipted, re-ran and
transcribed exactly says that two affordable rungs beat stone counting, and the
sentence the matrix draws from it says none does.

### The shortest route, and it is deletions and corrections

**Deletions** — nothing needs to be measured for any of these:

1. **DELETE** §2.5's *"The only rung that beats counting stones cannot be fitted,
   and every rung that can be fitted loses to counting stones"* and §2 item 4's
   *"while every affordable rung is BELOW a quotient that counts stones"*. Replace
   with the scoped truth the enum memo's §7.1 already states: **at the covering
   length** the only rung that beats stone counting is unaffordable. *(NEW-1)*
2. **DELETE** §2.5's *"AND ONE CELL LOOKS LIKE THE OPPOSITE"*, or print all five
   code-unit passes and name the `L = 13` T4 disagreement between units. *(NEW-2)*
3. **DELETE** §4's *"the ranking in §6 does not rest on any cross-currency
   comparison"*, or restate §6's rank-2 sentence in one unit. *(NEW-3c)*
4. **DELETE** §7's *"none beyond writing the terms"* and re-head its column to
   match §6's narrowed axis. *(NEW-7, MAJOR-6)*
5. **DELETE** §8's *"quoted whole rather than glossed"* clause and §5's *"D-705,
   whole:"* label, or quote D-653's flip clause and D-705's remaining sentences.
   *(m-10, B-1 residual)*
6. **DELETE** line 29's *"§11 is the failed-precedent check"* → §8. *(m-1)*
7. **DELETE** line 747's bare *"§6.5"* → `hex_threat_enum_v1.md` §6.5. *(m-2)*
8. **DELETE** §4's *"§2.5 measures that a value-free quotient of stone counts
   passes it at 1.77x"* → attribute it to the enum memo §6.5 and the OLD criterion.
   *(NEW-10)*

**Corrections** — each is arithmetic already on the page or one line of a ledger:

9. **PRICE `R-A4-CLASS` at `L = 7` T4 (and note `L = 9` T4), or state the ground
   that excludes shorter lengths and apply it to `R-A1-L8F` too.** The row's cells
   are 2 925 parameters / 856 observed codes / 5 878 observations each / traffic 21
   = 1.17x / floor 396 371 (0.749x) / criterion 1.014 MET. *(NEW-1)*
10. §4's currency table → R-A1-L11's unfolded figure is **0.73** positions per
    parameter, not 1.4; R-A3's cell says "§0.2's unit" and the table says
    "positions" — pick one; add the cell-centred window unit as the fifth currency.
    *(NEW-3)*
11. §1.5 → quote `book_v3_ledger.md:88`'s standing-claims row beside the
    consumed-ranges row, and say in §6 whether `R-H-EXT` may take R7's arm.
    *(NEW-4)*
12. §6 → say what §1.6 already implies: with one funded run, "run first" is "run
    only", so ranking by build cost is not by itself an answer to which row is run.
    *(NEW-5)*
13. §12 → M-1's row is false of §1.3, M-4's row is false of two cells. *(NEW-6)*
14. §4's `R-H-EXT` → price the choice of its 3–6 integer values, or say it is
    unpriced. *(NEW-8)*
15. §1.8 → the reason is that all three symbols are `HandcraftedV0`'s own methods
    at disjoint call sites; `HandcraftedV0` overrides `delta`
    (`handcrafted.rs:302`) and the trait default cited does not run here. *(NEW-9)*
16. §2 → qualify *"no cell of it is 816"* with the enum memo §7.1's own sentence
    that §6.4's coarsest faithful clip gives exactly 816. *(MAJOR-10, half)*
17. §1.6's column header; §1.3 → print the receipted six numbers beside the
    labelled ones; §1.7 → reconcile 4.83 h with 2–3 h and use the measured 479/457
    ms; §3 → name the winning side, call 5.25 a `d`, label the sign test one-sided;
    §8 → three of §10's five instruments are untracked. *(NEW-11, m-1…m-9)*

**Nothing here needs a run.** Every number the fix needs is in the receipts already
verified, and two of the corrections are a single division.

---

## 6. Would I rank the field differently, and on what

**Yes, and on the matrix's own instruments — but not for the first red team's
reason.**

**On `R-H-EXT` at rank 1, the author is right about the screen and wrong about the
row.** The first red team priced `R-H-EXT` as the most expensive row because its
screen consumes `book_v2`'s holdout. The author's rebuttal is sound: nothing makes
the screen compulsory, the row can take a `book_v3` arm like any other, and D-653's
`elo1 >= 30` is satisfiable without it. **That specific attack does not survive.**
But rank 1 falls to two others the matrix has not answered:

- **Its access to `book_v3` is asserted, not established** (NEW-4). The ledger
  reserves that book's 8 000 pairs for the R7 acceptance SPRT, which D-704 defines
  as *the selected family's*, and §6 itself calls `R-H-EXT` arguably not a Phase-2
  row. The equalising claim that carries the whole ranking rests on a page of the
  ledger §1.5 did not quote.
- **Its build cost is not nil** (NEW-8). Three to six hand-set integers, no fit, no
  metric, one funded run — the same structure that ranks `R-A2` last, in a row that
  has strictly less to choose by, in a field that already kills `R-C-SPSA` on the
  cost of setting eval integers.

So I reach the first red team's ordering — **`R-H-EXT` not first, and not ranked at
all until the operator rules** — by a different route: not because the screen is
compulsory, but because the row's two priced advantages (free book access, nil
build) are both unestablished on the matrix's own sources.

**The bigger re-ranking is that the field is missing its best-evidenced row.**
`R-A4-CLASS` at `L = 7` T4 is out of the priced field on a sentence its own receipt
contradicts. Put it back and, on the matrix's own columns, it is at or near the top
of the learned rows:

| column | R-A4-CLASS @ L=7 T4 | R-A5-TOPK | R-A1-L8F |
|---|---|---|---|
| parameters | **2 925** nominal, 856 observed | 811 or 143 (unit-dependent) | 2 920 |
| density | **5 878** obs / observed code; 1 720 / nominal | median 7 at the raw unit; head dense | median 273 |
| traffic | **21 (1.17x)** — cheapest in the field | 33 (1.83x) | 24 (1.33x) |
| nps floor | **396 371 (0.749x)** — highest in the field | 296 959 (0.561x) | 365 760 (0.691x) |
| seed budget | 1 fit, closed form | 1 fit, closed form | 1 fit, closed form |
| a registered criterion that COULD fail | **passed, 1.014** | none registered | none registered |

That last row is the one that should move an architect, and it is worth being
exact about what the other rows have. `R-A5-TOPK`'s kill is *"unmeasured, and this
matrix does not measure it either"*, by its own cell. `R-A1-L8F`'s kill is an unrun
theorem. `R-A3`'s is a Stockfish caveat from a corpus 200 000x larger. `R-A2`'s
kill is the one real live test in the field — validation loss against A3, which §3
now measures the instrument can resolve — but it is a test that has not been run
and whose result, by the matrix's own Rapfi citation, ranks nets wrongly for
strength. **So the only learned row in this field carrying a criterion that was
registered before a run, could have failed, was RUN, and did not fail at an
affordable rung is the one the matrix deletes from the field.** Whether `L = 7`'s two-window reach is good enough is a
real question — but it is the same question the matrix answers "unrun theorem, rank
it third" for `R-A1-L8F` at `L = 8`.

**And one thing I would not change.** §6's decision to rank an ORDER rather than
select, to carry the reviewer's disagreement in the document rather than settle it,
and to hand the holdout ruling to the operator, is right and should survive
whatever the next round does to the rest.

**The single strongest thing I could not break**: the paired re-analysis in §3,
checked at the data and at the instrument. It is what a fix round should look like.
