# REVIEW-design — `docs/experiments/hex_threat_enum_v1.md` revision 1 — **FAIL**

## Header

**Named revision under review**: `git stash create` SHA
`ecfc2d8e6e7fbdec84ff3d0cbfee42e37f344154` (a merge commit, parents `0585e25` and
`0c005e8`), branch `dev`. Read with
`git show ecfc2d8e6e7fbdec84ff3d0cbfee42e37f344154:docs/experiments/hex_threat_enum_v1.md`.

**Does it still match the live tree? THE TREE MOVED DURING THIS REVIEW, and the answer has
two halves.**

- *The document itself: YES, byte for byte.* When this review began, `git rev-parse HEAD` was
  `0585e2570b346df9b575f1225dfa93065b206859` and the memo was an uncommitted `A ` entry. When
  it finished, HEAD was `c5123c172a7fea72d1c45c5e62509265757a6b17`, reached by three new
  commits — `d73deba` (*"the phase-2a paste block lands seven rulings"*), `01d1529`, `c5123c1`
  (*"the hex threat enum is defined as a computed quotient before any of it is measured"*).
  `diff <(git show HEAD:docs/experiments/hex_threat_enum_v1.md) <(git show ecfc2d8e…:…)` is
  empty, and so is the same diff for `docs/decisions.md`. **Every finding below applies verbatim
  at `c5123c1`.**
- *The surroundings: NO, and two of the changes are load-bearing for this review.*
  (i) **D-706 landed at `d73deba`**, which is *after* the memo's own declared governing revision
  `0585e25` — see M-6, which the move confirms rather than repairs.
  (ii) **§6's named instrument now exists**: `tools/hex_enum/{hexenum,census,report,seed_pilot,
  test_hex_enum}.py` and `tools/hex_enum_tests.sh`, **untracked** in the working tree at the
  moment of writing (`git status --porcelain` → `?? tools/hex_enum/`), i.e. at no revision at
  all. `hexenum.py` is `sha256 5a4a2767…f5be`. The tree was still moving as this was written
  (`M tools/ci.sh` appeared during the final pass). This report reviews the MEMO; but because
  `process.md`'s *"Instrument governing revision"* binds the instrument to the same review, and
  because the instrument resolves three ambiguities of the memo **against the memo's own
  words**, its output is cited below as evidence — never as the subject.

**Instruments built.** One file, pure stdlib, an INDEPENDENT re-implementation of §5's tuple
written from the memo's prose before `tools/hex_enum/` existed:
`/tmp/claude-1000/hexenum/repro.py`. That path is ephemeral (MEMORY.md's *"Review reports live
in ephemeral scratchpads"*), so the whole script is inlined in **Appendix A** and every number
carries the tag (`F1`…`F8`) it prints. Runtime ≈ 4 min, single thread, no seed, no sampling —
every enumeration exhaustive.

**Cross-validation of the two implementations.** Over all 66 339 patterns at `L ∈ {7, 9, 11}`,
this reviewer's `T4` and the shipped `hexenum.tuple_of` **disagree 0 times**, and the shipped
instrument's class counts at odd `L` (25 / 98 / 357 / 357 and the ladder 121 / 47) reproduce
this reviewer's exactly. **So none of the disagreements reported below is an implementation
difference. They are all in the definition.**

---

## 1. Re-derivation ledger

Commands are mine, chosen to differ in scope from the memo's wherever the memo printed one.
All file reads pinned to `0585e25` (the memo's own governing revision) via `git show`.

| # | Claim (memo §) | My command / scope | Returned | Repro |
|---|---|---|---|---|
| 1 | 89 805 deduped positions (§1.4) | `awk '!/^#/ && NF' artifacts/arc3r_sweep_deduped_manifest.txt \| wc -l` — memo used `grep -vc "^#"`; mine also rejects blank lines | `89805` | **YES** |
| 1b | same, no duplicate rows | `awk '!/^#/' … \| LC_ALL=C sort -u \| wc -l` (scope the memo did not take) | `89805` | **YES** — the "deduped" adjective holds |
| 1c | manifest digest `00f61780…f35968` | `sha256sum artifacts/arc3r_sweep_deduped_manifest.txt` | `00f61780cc1654958696786051dbd8de7d1bbab3f5d49153b1694770caf35968` | **YES** |
| 1d | that digest is the one `book_v3_ledger.md` quotes | `/usr/bin/grep -n "00f61780" docs/book_v3_ledger.md` | line 50, `…deduped_manifest.txt` | **YES** |
| 2 | 74 672 `eval` rows; 45 271 quiet, 60.63 % (§1.4) | read `artifacts/research_2026-09/game_count.txt` (a receipted artifact, **not** the premise memo the memo cites) + `45271/74672` | `eval records: 74672`; `0.606266…` → 60.63 % | **YES** |
| 3 | 3 487 games, 25.75/game, median 23 (§1.4) | same artifact, and independently `docs/book_v3_ledger.md:51` (`control: corpus5 ^ v2: 3487 of 3487`) | `3487`, `25.75`, `23 (min 8, max 39)` | **YES**, by two disjoint sources |
| 4 | research receipt verifies 7 of 7 (§1.4) | `cd artifacts/research_2026-09 && sha256sum -c RECEIPT_research_2026-09.sha256` | 7 lines, all `OK` | **YES** |
| 4b | receipt digest `13fe5712…16dab` | `sha256sum artifacts/research_2026-09/RECEIPT_research_2026-09.sha256` | `13fe5712dafa94e2916267800f7b4162db380e27b03a16e00fd97f0981916dab` | **YES** |
| 5 | census table L7/8/9/11 folded (§1.4) | read `docs/research/eval_families_2026-09.md` §0.2 folded rows at `0585e25` | 1 029 / 2 920 / 8 374 / 38 983; medians 1 282/273/56/7; <10: 0/24/1 079/22 142; 90 %: 190/293/434/810 | **YES**, all sixteen cells |
| 6 | 13 calculus anchors `:12`,`:28`,`:37`,`:38`,`:40`,`:45`,`:64`,`:68`,`:87`,`:104`,`:128`,`:152`,`:156` (§1.1–1.2) | `git show 0585e25:docs/research/threat_calculus_v1.md` → `sed -n "${n}p"` per line, grep for the ID | 13 of 13 land on the named ID | **YES** |
| 7 | 17 verbatim quotations from the calculus and `eval_families` (§1.1–1.2, §5, §6, §8) | whitespace-flattened `in` test over both files at `0585e25` | 17 of 17 present (one, *"a 4-bit `Pattern` enum…"*, differs only by the source's `**` emphasis marks) | **YES** |
| 8 | `query.rs:81-142`; `:81` = `pub enum LiveCount {`, `:142` = `pub fn live_windows_at_count…` (§1.3) | `awk 'NR>=70&&NR<=150{print NR"\t"$0}' crates/pistol-solver/src/query.rs` | exact match at both lines | **YES** |
| 8b | file byte-identical at `d83ac01` and HEAD (§1.3) | `diff <(git show d83ac01:…/query.rs) …/query.rs` | empty | **YES** |
| 8c | `Class::Hot` is `own >= 4` (§1.3, §6.7) | `sed -n '25,46p' crates/pistol-solver/src/sets.rs` — the memo cites `query.rs`, the constant is asserted in `sets.rs:27-31` | `3 < HOT_MIN && WIN_IN_ONE_PLY == HOT_MIN + 1 && HOT_MIN + TURN_STONES == WINDOW_LEN` ⇒ HOT_MIN = 4 | **YES** |
| 9 | `AXES = ((0, 1), (1, 0), (1, -1))` at `tools/texel/features.py:14` (§3) | `/usr/bin/grep -n AXES tools/texel/features.py` | `14:AXES = ((0, 1), (1, 0), (1, -1))` | **YES** |
| 9b | `tools/texel/extract.py:24-27` "names the columns" (§6.5) | `awk 'NR>=18&&NR<=32…' tools/texel/extract.py` | `SCORE_KIND, SCORE_VALUE… = 7, 8, …` on **line 26**; line 27 is blank; the anchoring comment is line **23** | **YES with slip** — range is off by one at both ends (m-7) |
| 10 | `C(18,3) = C(16+2,3) = 816`, i.e. the 816 is Rapfi's `k=16` (§1.1) | arithmetic, `18·17·16/6`; cross-check `C(19,4) = 19·18·17·16/24 = 3876` = Rapfi's `PCODE_NB` | `816`; `3876` | **YES** — §1.1's identification is correct |
| 11 | `\|W_L\| = min(6, L−5)` for `L ≥ 6` (§5) | F1: enumerate window starts, L = 6…14 | True for all nine | **YES** (single alignment only — see B-2) |
| 12 | `(3^(L−1) + 3^⌈(L−1)/2⌉)/2` is the folded raw-code ceiling (§6.2) | F8: enumerate `{min(p, reversed p)}` at L = 7, 9, 11 | 378 / 3 321 / 29 646, formula agrees exactly | **YES** for odd L |
| 13 | §5.4 reversal invariance is a priori (§5.4, D-706) | F4: all 3^10 = 59 049 length-11 patterns | **0** mismatches | **YES** |
| 14 | §5.1 `r4` induces no refinement (§5.1) | F4: `k` with and without both `r4` components | 357 and 357 | **YES** |
| 15 | §5.2's per-side bound 37, joint 1 369 (§5.2) | F4/F5: realised joint classes at L=11 | 357 ≤ 1 369 | **YES** (bound holds; see M-2 for what the memo does not do with it) |
| 16 | §5.3's cubic at 16/40/100 (§5.3) | `C3(k)` for k = 16, 40, 100 | 816 / 11 480 / 171 700 | **YES** |
| 17 | D-706 is "the standing line" at the memo's governing revision (§ header, §1.1) | `git show 0585e25:docs/decisions.md \| grep -cE "^D-70[2-8] "`; `git log -S "D-706 [hex-enum-computed]" -- docs/decisions.md` | `0`; highest ADR at `0585e25` is **D-701**; D-706 first appears at `d73deba`, *after* `0585e25` | **NO** — see M-6 |
| 18 | §6.6: `k(L) = k(11)` for `L ≥ 11` **by derivation** | `python3 tools/hex_enum/hexenum.py 11 12 13` (the memo's own named instrument) | `357`, **`1325`**, `357` | **NO** — falsified at L=12; see B-1(a) |
| 18b | same, under this reviewer's reading of §3 | F2 | `357, 357, 357` | **YES** under reading (A) — which is B-1(b): the convention decides it |
| 19 | §6.1: "enumerate all `3^(L−1)` patterns", every L | `sum(1 for _ in hexenum.patterns(L))` for L=6…14, and F2 | odd L: matches. even L: instrument gives `3^(L−2)`, reading (A) needs `3^L`; §6.1 matches neither | **NO** — see B-2 |
| 20 | §5.5 "no threshold appears"; §6.4 "none of them hand-written" | read §6.4's rung table and `hexenum.project`/`clip` | three chosen clip points: `open` at ≥2, `min` at ≥4 (twice) | **NO** — see m-4 |
| 21 | §6.4's T1, `"clipped at ≥ 4"` | F5 (∞ clips to 4) vs `hexenum.clip` (∞ kept apart) | `k = 16`, `C = 816` vs `k = 25`, `C = 2 925` | **NO** — the text admits both; see B-3 |
| 22 | §5.4 reversal invariance at even `L` under BOTH readings of §3 | F2 (reading A) and `tuple_of(p[::-1]) == tuple_of(p)` over `hexenum.patterns(L)`, L = 6…12 | 0 mismatches in every case | **YES** — the mechanism works under either reading |
| 23 | this reviewer's tuple vs the shipped one | all 66 339 patterns at L ∈ {7,9,11}, element-wise | **0** disagreements | **YES** — no finding below is an implementation difference |

---

## 2. Findings

### BLOCKING

#### B-1 — §6.6's stabilisation length is not a measurement. Its "by derivation" half is **FALSE** at the memo's own instrument (`k(12) = 1325`, not 357); its criterion is not the relation that holds; and on the only steps where the criterion is well posed the answer is forced to 11 by §5's construction.

D-706 registers *"THM-WINDOW stabilisation length"* as one of five receipts that are *"the only
basis on which A4-CLASS is priced"*. Three separate things are wrong with it.

**(a) The derivation is false at the instrument §6 names.** §6.6: *"at `L ≥ 11` the window set
is complete and no further cell can enter any component, so `k(L) = k(11)` for all `L ≥ 11`
**by derivation**. The instrument's report at `L = 12, 13, 14` is therefore a CONFIRMATION."*
Run `tools/hex_enum/hexenum.py` (untracked, `sha256 5a4a2767…f5be`) exactly as its own
`main` invites:

```
$ python3 tools/hex_enum/hexenum.py 11 12 13
L 11  T4 k 357   codes 7647059    T3 k 121  codes 302621    T2 k 47   codes 18424   T1 k 25  codes 2925
L 12  T4 k 1325  codes 388578775  T3 k 443  codes 14587990  T2 k 119  codes 287980  T1 k 51  codes 23426
L 13  T4 k 357   codes 7647059    T3 k 121  codes 302621    T2 k 47   codes 18424   T1 k 25  codes 2925
```

`357, 1325, 357`. The derivation is correct **per cell** — no cell outside `c±5` enters any
component — but §3 makes the even-`L` class not a per-cell object, so the per-cell argument does
not reach it. §6.6 states the conclusion for *all* `L ≥ 11` and the first even length above 11
falsifies it, at a parameter count of **388 578 775**.

**(b) Which way (a) falls is decided by an ambiguity in §3, not by hex.** Under this reviewer's
reading of §3 (the pair of alignments of ONE cell, so `L+1` cells are read), both alignments at
`L = 12` see the same six windows and `k(12) = 357` — the derivation holds. Under the shipped
reading (two adjacent empty cells inside one `L`-cell window) it is 1325 — the derivation fails.
**A derivation whose truth value is set by an undecided convention is not a derivation.** See
B-2.

**(c) The criterion is not the relation that holds, and where it is well posed it is empty.**
§6.6 defines the stabilisation length as *"the smallest `L` at which `k(L+1)` refines no class
of `k(L)`"*, and §6.1 reports *"the contiguous range `L = 6 … 14` because §6.6's refinement test
needs `k(L+1)` beside `k(L)`"*.

- The instrument does not implement that test. `hexenum.refinement()` takes `step=2` and its own
  docstring anticipates the answer: *"Neither means the two partitions are not comparable at
  all, which is the interesting answer below the covering length."* Run it:

  ```
  $ python3 tools/hex_enum/hexenum.py --refine 7 9 11
  L 7 -> 9   T4  k 25  -> 98   joint 177  neither refines the other
  L 9 -> 11  T4  k 98  -> 357  joint 920  neither refines the other
  L 11 -> 13 T4  k 357 -> 357  joint 357  the longer partition is a function of the shorter
  ```

  "Neither refines the other" is the instrument reporting that §6.6's word *refines* names a
  relation that does not hold below 11. `k(L)` is not a nested sequence, so *"the smallest L at
  which…"* ranges over a family that is not a chain.
- Applied literally to the memo's own contiguous `L = 6…14` under this reviewer's reading of §3,
  the criterion **fires at L = 6** — **F3**, over all 3^12 = 531 441 length-13 neighbourhoods:

  ```
  L= 6 ->  7: max (L+1)-classes per L-class =  1  DETERMINES
  L= 7 ->  8: max (L+1)-classes per L-class = 20  splits
  L= 8 ->  9: max (L+1)-classes per L-class =  1  DETERMINES
  L= 9 -> 10: max (L+1)-classes per L-class = 36  splits
  L=10 -> 11: max (L+1)-classes per L-class =  1  DETERMINES
  L=11 -> 12: max (L+1)-classes per L-class =  1  DETERMINES
  L=12 -> 13: max (L+1)-classes per L-class =  1  DETERMINES
  ```

  It fires there for a reason with nothing to do with hex: every even rung reads more cells than
  the odd rung above it, so every even→odd step "determines" trivially and every odd→even step
  splits. **F2** shows the same as a class count — `k` under the memo's conventions at
  `L = 6…14` is `33, 25, 177, 98, 920, 357, 357, 357, 357`, which **falls** at every even→odd
  step. The memo's "L = 10" reads the same eleven cells `c±5` as its "L = 11" and separates 920
  classes against 357; each L=10 class lands in exactly one L=11 class while an L=11 class
  spreads over up to nine L=10 classes (**F3**). Lengthening 10 → 11 *destroys* information.
- Restrict to odd steps — which is what the instrument actually does — and the criterion is well
  posed and **empty**. §5 fixes the tuple's reach at `c±5`; an odd `L < 11` therefore always
  omits a window through `c`, so `L → L+2` always splits until 11 and never after. The answer is
  11, by the one-line argument §6.6 already gives as the derivation. §6.6 concedes the `L ≥ 11`
  half is *"a CONFIRMATION … not an independent measurement"*; **the other half is one too.**

*Minimal reproducer*: Appendix A `F2`/`F3`; and the two `hexenum.py` invocations above.

*Why blocking*: §1.2's premise table says of THM-WINDOW *"the owed enumeration §6 discharges"*.
A receipt that (a) is false at one length, (b) has its truth set by an undecided convention, and
(c) cannot take a value other than the one the construction forces, discharges nothing — and it
is one of the five things D-706 says the row is priced on.

#### B-2 — §3 does not determine the enum at even `L`. Three readings are open, they give different `k`, §6.1's `3^(L−1)` matches **none** of them, and the reading the shipped instrument took contradicts §4's "the index is a CELL".

§3: *"For even `L` there is no cell at the middle, so `P_L(c, a)` is taken as the unordered PAIR
of the two alignments — `c` at index `L/2 − 1` and `c` at index `L/2` — and the class is the
multiset of the two tuples."*

`c` is one cell, and *"`c` at index `L/2 − 1`"* and *"`c` at index `L/2`"* place the same cell at
two positions of an `L`-cell window — which is two different sets of board cells, `[c−(L/2−1),
c+L/2]` and `[c−L/2, c+(L/2−1)]`, union `L+1` cells. That is reading **(A)**. The sentence can
also be read as two positions *within one fixed window*, which then requires both middle cells
empty — reading **(B)**, which is what `tools/hex_enum/hexenum.py` implements (`centres(L)`
returns both indices and `patterns(L)` fixes both empty). A third, that the class is the pair
over one window with one middle cell empty, is not consistent at all. Measured:

| memo's label | (A) this reviewer: cells / strings / k | (B) the shipped instrument: cells / strings / k | §6.1 claims |
|---|---|---|---|
| L = 6 | 7 / 729 / **33** | 6 / 81 / **10** | 3^5 = 243 |
| L = 8 | 9 / 6 561 / **177** | 8 / 729 / **43** | 3^7 = 2 187 |
| L = 10 | 11 / 59 049 / **920** | 10 / 6 561 / **182** | 3^9 = 19 683 |
| L = 12 | 13 / 531 441 / **357** | 12 / 59 049 / **1 325** | 3^11 = 177 147 |

(A) is **F2**; (B) is `python3 tools/hex_enum/hexenum.py 6 8 10 12` and
`sum(1 for _ in hexenum.patterns(L))`. **§6.1's `3^(L−1)` is right at every odd `L` and wrong at
every even one, under both readings** — (A) needs `3^L`, (B) needs `3^(L−2)`.

The consequences are not cosmetic:

- **B-1(a)** turns on which reading is taken: (A) gives `k(12) = 357` and §6.6's derivation
  holds; (B) gives 1 325 and it fails.
- **(B) contradicts §4.** §4 is titled *"Why the index is a CELL and not a window"* and argues
  *"`C(k+2, 3)` is a per-CELL object and the row is only well posed per cell"*. Under (B) an
  even-`L` class is the unordered pair of the tuples of **two adjacent cells**, so `C(k+2,3)` is
  not a per-cell object at even `L` and the even rows are not comparable with the odd ones.
- **(B) silently biases the population.** It enumerates only patterns in which the cell's
  neighbour along the axis is also empty — `3^(L−2)` of `3^(L−1)`, i.e. **two thirds of the
  candidate cells cannot be described at even `L` at all**. §6.3's census walk would have to skip
  them, and the memo does not say so.
- **§5's `|W_L| = min(6, L−5)`** is the count for one alignment. Under (A) the pair's window set
  is the union — **4 at L = 8, 6 at L = 10**, not 3 and 5. §6.7 self-check 4 compares the
  enumerated count against `min(6, L−5)` and would be checking one alignment while §7 reports
  the pair.
- **§6.2's ceiling** `(3^(L−1) + 3^⌈(L−1)/2⌉)/2` is the fold count of a string about a fixed
  centre. Exact at odd `L` (ledger #12); at even `L` there is no such centre.
- **The cost argument.** §6.6 says *"a short window is cheap"* and `eval_families` §0.2 prices
  L = 8 folded at 2 920 cells / median 273 observations — a length-8 **window**, all eight cells
  encoded. Neither (A) nor (B) is that object. Any matrix row priced at "L = 8" on §0.2's
  observability is pricing something else.

*Minimal reproducer*: `python3 tools/hex_enum/hexenum.py 6 8 10 12` beside Appendix A `F2`.

#### B-3 — the T1 rung's clip is ambiguous about `∞`, and the two readings differ by exactly the imported figure: `k = 16 → C(18,3) = 816` against `k = 25 → 2 925`. Both readings destroy the LAW-SUPPORT boundary that is T1's only stated justification.

§6.4: *"**T1** | `(min_own, min_opp)` with both clipped at `≥ 4` | the cost alone, at the
resolution `LAW-SUPPORT` gives for `k ≤ 2` own turns."* §5.2 fixes
`min_X ∈ {1,…,6, ∞}`. Does `∞` clip to 4?

- **No it does not**, and then `min_X` has four buckets `{1,2,3,≥4}` per side, so `k ≤ 16`
  **before any enumeration**, and all sixteen are realised — **F5**: `k = 16`,
  `C(18,3) = **816**`.
- **Yes it is kept apart**, and then there are five buckets, `k = 25`, `C(27,3) = 2 925` — which
  is what `hexenum.clip()` implements (`return INF if least == INF else min(least, 4)`), on a
  ground the memo does not state: *"`INF` is NOT folded into the clip, because DEF-WINDOW's dead
  is a different thing from an expensive live window."*

§1.1 says of 816: *"`C(18,3) = C(16+2,3)` — the 816 is Rapfi's `k = 16` carried across the axis
count … if that number is not 816 then §7 says so and the matrix prices the row on it."* Under
the first reading it **is** 816, and not because hex agrees with Rapfi — because four buckets
squared is sixteen. The memo's registered falsifier is a coin-flip on a sentence the memo never
disambiguates, and one face of the coin is the import. (816 reaches §7's table anyway: the
shipped instrument prints `L 10 … T1 k 16 codes 816`.)

**And the clip point is wrong on its own ground under both readings.** `min_X = 6 − (X's stones
in the best open window)`, so LAW-SUPPORT (`threat_calculus_v1.md:69`, *"a forced win in k own
turns requires an open window already holding ≥ 6−2k own stones"*) reads: `k = 1 ⟺ own ≥ 4 ⟺
min ≤ 2`, and **`k = 2 ⟺ own ≥ 2 ⟺ min ≤ 4`**. The `k ≤ 2` boundary sits between `min = 4` and
`min = 5`. Clipping at `≥ 4` puts `min = 4` — two own stones, a LAW-SUPPORT `k = 2` candidate —
in the same bucket as `min = 5` and `min = 6`, which are not. `hexenum.clip`'s docstring states
the error explicitly: *"LAW-SUPPORT gives the attacker's resolution at k <= 2 own turns, which is
a cost of at most 4; beyond that the exact number stops mattering."* At most 4 is precisely why
4 must stay separate from 5. **T1 cannot express the criterion it cites.** Clipping at `≥ 5`
gives `k = 25` at L = 11 (**F5**) *and* keeps the boundary.

*Minimal reproducer*: Appendix A `F5`; and `hexenum.py 11` for the shipped `T1 k 25`.

### MAJOR

#### M-1 — dispatch question 1, answered YES: the tuple merges patterns whose exact DEF-T after a stone at `c` differs, and `t = 1` vs `t = 2` is exactly the calculus's §5 table row split (PAT-C4 against PAT-O4 / PAT-GAP).

RULE-EXACT (`:64-66`): *"only exact `t` decides truth."* I computed, for every length-11
pattern, the exact minimum hitting set (DEF-T) over own's plan family (DEF-PLAN: open window,
≥4 own, empty set of size ≤2) **after placing one own stone at `c`** — which is the question §3
says the tuple asks, *"what a stone placed at `c` buys"*. Result — **F6**: **27 of the 357
classes contain patterns whose `t` differs; 5 348 of 59 049 patterns sit in such a class.**

Two witnesses, `.` empty, `X` own, `O` opp, `c` = index 5:

```
class (min_own=3, open_own=6, r4_own=0, min_opp=6, open_opp=1, r4_opp=0)
   ......X..XX   place X at c -> .....XX..XX   plans {7,8}                  t = 1
   ......XXX..   place X at c -> .....XXXX..   plans {9,10},{4,9},{3,4}     t = 2

class (min_own=2, open_own=6, r4_own=0, min_opp=6, open_opp=1, r4_opp=0)
   ......X.XXX   place X at c -> .....XX.XXX   plans {7},{4,7}              t = 1
   ......XXX.X   place X at c -> .....XXXX.X   plans {9},{4,9},{3,4}        t = 2
```

Both members of each pair carry the identical 6-tuple, so they carry the identical class, and
since the 3-axis multiset is built from per-axis classes the merge survives to the code. The
calculus's §5 table is a table of `t`: PAT-C4 *"single plan"* `t = 1`; PAT-O4, PAT-O5 and
PAT-GAP `t = 2`. LAW-OVERLOAD makes `t` the criterion for a forced win. So the tuple assigns
one weight to a cell that buys a hittable single plan and to a cell that buys an unhittable
pair of them.

This is not a claim that the memo is dishonest — §6.5 exists precisely to price what the
quotient loses, and §8 records the family's known-wrong functional form. It is a claim that
**the witness belongs on the page before §7 runs**, because the memo's §1.3 argument is that
the tuple *"carries `min` and `open` at full resolution and lets the equivalence do the
classifying"*, and a reader takes from that that calculus-material distinctions survive. Two of
them do not, in the simplest shapes the game has.

*Minimal reproducer*: Appendix A, `F6`.

#### M-2 — dispatch question 2: the split side. `C(k+2,3) = 7 647 059` at the enumerated `k(11) = 357` — 0.006 observations per parameter, two hundred times worse than the row `eval_families` §0.3 already kills — and the memo declines to apply its own cubic to its own bound.

**F4**: `k(11) = 357`, so the row's parameter count at T4 is `C(359,3) = 7 647 059`, against the
45 271-row quiet population: **0.0059 observations per parameter**. `eval_families` §A1 kills
R-A1-L11 at **1.2**, and Buro's floor is `≥ 20`. §6.4's T3 is 302 621 parameters (0.15) and T2
is 18 424 (2.5); only T1 (816) and T2 clear Buro's `≤ 4`-match rule at all.

None of this is a measurement the memo is forbidden to anticipate. §5.2 already **derives** the
bound `37² = 1 369`; §5.3 already **derives** `C(k+2,3)` at three points. It applies the cubic
at `k = 16`, `40`, `100` — and never at its own bound, where `C(1371,3) = 428 558 605` (**F4**).
The three exemplars span 816 to 171 700; the derived bound is three orders of magnitude past
the largest of them. The only anchor a reader carries out of §5.3 is **816**, the imported
number, and the sentence that follows (*"extremely sensitive to a quantity this memo
deliberately does not choose"*) reads as caution while the exemplars do the choosing.

D-291: *"an estimate that could have been measured in seconds is a finding."* Here it is not
even an estimate — `C(1371,3)` is one line of arithmetic in a document that already contains
three of them.

**And the memo's own instrument now prints the answer**, so this is no longer a reviewer's
number: `python3 tools/hex_enum/hexenum.py 11 12` returns `L 11 T4 k 357 codes 7647059` and
`L 12 T4 k 1325 codes 388578775`. The row's headline parameter count at the memo's own covering
length is **7.6 million against 45 271 positions**, and at L = 12 it is **389 million**. §7 will
land those numbers into a matrix whose only other anchor is the 816 of §5.3.

*Minimal reproducer*: Appendix A, `F4`, `F5`.

#### M-3 — §6.5 invokes `process.md`'s "Criterion and defect class" and satisfies neither half: it registers **no criterion at all**, and its first referent is a mathematical identity that the named defect cannot falsify.

`process.md`: *"AND IT RECORDS WHAT THAT OUTPUT MUST SHOW … A criterion that is a property the
named defect class PRESERVES … passes vacuously and is not a criterion; it must be one that
defect could falsify."*

§6.5 names the defect — *"the quotient throws away a distinction that carries value"* — and
then names two referents and **no threshold**, and §7 says *"NO SELECTION"* and §6.5 says `η²`
*"gates nothing (D-614)"*. Nothing can fail. That is the first half.

The second half is sharper. **Referent 1 cannot fail as a matter of arithmetic.** The tuple is a
function of the pattern and is reversal-invariant (F4), so the folded raw code partition is a
*refinement* of the tuple partition. Between-class variance is monotone under refinement (the
law of total variance the memo's own self-check 3 uses), therefore `η²(raw) ≥ η²(tuple)`
**always**, for every corpus, for every tuple, whether or not the quotient lost anything of
value. The size of the gap is driven by the ratio of class counts (29 646 against 357), not by
what was discarded. A quantity whose sign is fixed by an identity and whose magnitude tracks
`k` is *"internal agreement between components sharing an input"* in the clause's own words.

**Referent 2 tests a different proposition**, and the memo says so honestly: *"a tuple whose
`η²` does not beat it has separated nothing the corpus can see."* "Separates nothing" is not
"throws away a distinction that carries value" — a tuple that separated only stone count would
beat a random quotient comfortably while discarding every structural distinction. So the
externally derived referent the clause asks for is present, and it answers a *different* defect
than the one §6.5 names.

*Minimal reproducer*: no compute needed — the refinement claim is the definition of `η²` plus
F4's reversal check. If it is disputed, the falsifier is a single pattern whose tuple is not a
function of its folded code; F4 enumerates that none exists.

#### M-4 — §6.4's T2 rung reproduces exactly the defect §5.1 names three pages earlier.

§5.1: *"A component that changes no class may not be used to claim the tuple is richer than it
is."* T2 is *"`(min_own, min_opp, [open_own ≥ 1], [open_opp ≥ 1])`"* and is described as
*"completion cost plus alive-or-dead"*. But §5.2 states the link itself: `open_X = 0 ⟺
min_X = ∞`. So `[open_X ≥ 1]` is a function of `min_X` and induces no refinement. Measured —
**F5**:

```
T2 as written                      k=  47
T2 with the two booleans deleted   k=  47
```

T2 *is* `(min_own, min_opp)` uncapped. The ladder therefore has four rungs of which one is
mislabelled as carrying `DEF-WINDOW`'s predicate when it carries only `LAW-SUPPORT`'s
arithmetic.

#### M-5 — T3's justification is wrong three ways, and one of them is a fact about hex the memo could have derived.

T3 is *"`open` clipped to `{0, 1, ≥2}`"*, justified as *"the live-count distinction `LiveCount`
makes, at the resolution `LAW-OVERLOAD`'s `t ≥ 3` needs"*.

1. `2 ≠ 3`. A clip at `≥2` cannot express a distinction whose threshold is three.
2. `LiveCount` (`query.rs:81-86`) distinguishes *"Exactly two own stones"* from *"Exactly three
   own stones"* **inside one window** — a stone count, which the tuple carries in `min_X`, not
   a count of windows, which is what `open_X` is and what T3 clips. The attribution names the
   wrong quantity.
3. LAW-OVERLOAD's `t ≥ 3` is unobservable to a single-axis tuple in this game. **F6**: the
   maximum exact DEF-T over **all** 3^11 = 177 147 length-11 single-axis lines is **2**. The
   calculus's own `t = 3` cases (LAW-OVERLOAD's addition floor: *"crossing fours t=3"*,
   *"same-line double t=3"*) are cross-axis or longer than the tuple's reach. §5.5 already says
   the tuple is *"single-axis by construction"*; §6.4 then justifies a rung by a whole-position
   criterion the construction excludes.

#### M-6 — the memo's declared governing revision does not contain the ADR the memo says binds it, and that ADR was written in the same uncommitted act as the memo.

Header: *"**Governing revision**: `0585e25` (`dev`), the revision every file, line and count
quoted below was read at (D-692)"*, and §1.1: *"D-706 is the standing line that binds the
definition."*

```
$ git show 0585e25:docs/decisions.md | /usr/bin/grep -cE "^D-70[2-8] "
0
$ git show 0585e25:docs/decisions.md | /usr/bin/grep -oE "^D-[0-9]+" | LC_ALL=C sort -V | tail -1
D-701
$ git log --oneline -S "D-706 [hex-enum-computed]" -- docs/decisions.md | tail -1
d73deba docs(adr): the phase-2a paste block lands seven rulings, ...
```

D-702 through D-708 were added by the *same* uncommitted change that added this memo, and they
landed at `d73deba` — **after** `0585e25`, the revision the memo's header names as *"the
revision every file, line and count quoted below was read at"*. The tree moving during this
review did not repair it; it dated it. D-706 already asserts, in
its own text, the memo's §5.4 conclusion (*"reversal invariance is by construction rather than
by measurement"*) and the memo's §6.6 receipt list (*"THM-WINDOW stabilisation length"*). So the
memo quotes as a binding premise a line written alongside it, and the "standing line" that
constrains the definition contains the definition's conclusions. D-477 makes a premise a thing
quoted at `file:line`; §1's whole discipline is that premises are external. This one is not.

*Fix*: either land the ADR at a revision the memo can name and re-derive against, or say on the
face of the memo that D-706 and this document are one act and that the memo's compliance with
it is therefore not evidence.

#### M-7 — §1.2 claims THM-WINDOW's owed enumeration is discharged. It is not: the construction cannot produce THM-WINDOW's two numbers, and the question §6.6 answers is upstream of the one that was owed.

§1.2's premise table, THM-WINDOW row: *"The owed enumeration §6 discharges."*

THM-WINDOW's numbers are window-relative context lengths: a run of `n` own stones needs
`n + 2·(6 − n)` cells to decide whether the six can still be completed on both sides — `4 + 4 =
**8**` for a four, `5 + 2 = **7**` for a five, and `1 + 10 = 11` at the extreme, which is
§0.1's covering bound. The memo's construction is CELL-centred with a reach fixed at `c±5` by
§5, and **every** component of the tuple (`min_X`, `open_X`, `r4_X`) reads a window that may
start at `c−5`. No component, and no sub-question of any component, can stabilise before 11.
So neither 7 nor 8 can appear anywhere in §7. That is a difference of question, not a defect of
the tuple — but the word *"discharges"* licenses closing THM-WINDOW on a number that answers
something else.

Worse, `eval_families` §8 states the owed question in the form that matters here: *"THM-WINDOW
asks whether a shorter window still separates structures of different value once summed over
overlaps."* §6.6 measures whether the **tuple** gains information as `L` grows. Those coincide
only if the tuple is already sufficient — and **M-1 measures that it is not**. A stabilisation
length computed from an insufficient tuple is a lower bound on the sufficient length that the
document presents as the answer.

---

### MINOR

**m-1 — §4's containment convention is vacuous at every length the memo headlines.** *"AND A
WINDOW NOT CONTAINING `c` IS NOT `c`'s BUSINESS"* is presented as a substantive choice.
Enumerated: 6-windows inside a length-`L` pattern that miss `c` number `0` for `L ≤ 11` (and
`0` under §3's pair convention through `L = 10`); the first is at `L = 12`. At `L = 11`, the
memo's own headline, the convention forbids nothing.

**m-2 — §6.7 self-check 5's `Completed` arm cannot fail.** `Class::Completed` is *"Every cell
own: the window is a completed run"* (`crates/pistol-solver/src/sets.rs:44-45`). Every window
through `c` contains `c`, which §3 fixes empty. The set of length-11 patterns satisfying
`Completed` at `c` is empty at every `L ≤ 11`, so "is a union of tuple classes" holds
trivially. A self-check that cannot fail is not a check (§5.1's own standard, applied to §6.7).

**m-3 — §6.3 imports Buro's three floors without the transferability tag its own source
carries.** `eval_families` §A1 tags *"his generator keeps only configurations seen in ≥ 75 of
~11 M positions"* **EMP** — its own scheme's word for non-transferable. §6.3 reports *"cells
under Buro's floors at all three of his published figures"* — `≤ 4`, `< 20`, `< 75` — with no
tag. These are thresholds established on a square-board engine, they enter the "observations"
receipt D-706 says the row is priced on, and the scope rule (`threat_calculus_v1.md:12-13`) is
about numbers, not about whether they are named as someone else's. `≤ 4` and `≥ 20` are ARCH in
the source (rules of thumb); `75` is not.

**m-4 — §5.5's "no threshold appears" is true of the tuple and false of what §7 will report.**
§6.4 introduces three chosen clip points (`open ≥ 2`, `min ≥ 4` twice) and calls the rungs
*"none of them hand-written"*. A clip point is hand-written; B-3 is what it costs.

**m-5 — §4's "carries no information" understates the scored-cell restriction, and §6.3's
alternative population is not the same kind of object.** On an unbounded board an additive
per-cell sum over cells taking the all-empty code diverges unless that code's weight is exactly
zero; the restriction to scored cells is a well-definedness requirement, not a convenience, and
the memo does not say so. The alternative — *"every empty cell inside rule 5's radius-8 legal
region"* — is a 2-D disc (217 cells per stone) where the scored set is a union of three
length-11 segments (DEF-STAR's 30 cells per stone). Scoring the radius-8 region makes the
evaluation depend on the size of the legal region, i.e. smuggles in a space term with no line
content. Reporting the two counts side by side is fine; calling them two populations for the
same object is not.

**m-6 — the CODE-unit `η²` in §6.5 will be ≈1 by memorisation at T4.** `C(357+2,3) = 7.6 M`
classes against an observation count that cannot plausibly exceed a few tens of millions
(45 271 positions × scored cells). No guard is registered, and §6.5 offers `η²` at the code unit
as a comparable number beside the window-unit one.

**m-7 — two citation slips.** `tools/texel/extract.py:24-27`: the column names are on lines
24-26, the anchoring comment is line 23, line 27 is blank. And §6.1 cites *"the dispatch's
headline set"* — an out-of-tree reference inside a document D-706 makes governing.

**m-8 — the tuple is equivariant under own/opp swap and the memo neither claims nor uses it.**
**F7**: 0 mismatches over all 59 049 patterns; 17 of the 357 classes are swap-fixed, giving
**187 orbits**. `eval_families` §0.4's argument for the reversal fold (*"reflection is a lattice
symmetry and the win condition is direction-agnostic"*) has an exact analogue here — a
mover-antisymmetric evaluator ties `w(class) = −w(swap(class))` and halves the free parameters
at zero inference cost. At `C(359,3) = 7.6 M` (M-2) a factor of two is not the fix, but it is a
symmetry the memo's own §5.4 paragraph stops one line short of.

---

## 3. QUESTIONS

**Q1.** §3/§4 score EMPTY cells only. What carries the value of the stones? `handcrafted_v0`
sums 18 window contributions **per stone** (`eval_families` §A6). §9 defers *"how a code's
weight enters a score"* to Phase 2b, but §4's choice of cell population is not deferrable — it
fixes the summand. Is the intended model `Σ_{empty scored cells} w(code)` with nothing at all
from occupied cells?

**Q2.** §6.6 promises the measured half is *"how many `k(11)` classes each `k(L)` class merges
for `L < 11`"*. Measured (this reviewer): L = 6 → 3–104 (mean 28.2); L = 7 → 3–104 (32.4);
L = 8 → 1–36 (7.2); L = 9 → 2–36 (9.4); L = 10 → **1–1** (1.0). What does a merge curve mean
when its L = 10 point is 1.0 because L = 10 is *finer* than L = 11 (B-1)?

**Q3.** §7 will land `k`, codes, observations and purity at 5 lengths × 4 rungs, and §7 says
*"NO SELECTION"* and §6.5 says `η²` *"gates nothing"*. What number, at what value, would make
the architect **not** price R-A4-CLASS? If the answer is "none", D-706's *"the only basis on
which A4-CLASS is priced"* is not operational.

**Q4.** D-483 says design documents carry no measured numbers. §5.2's bound and §5.3's cubic
ladder are on the page as DERIVED arithmetic. Does that exemption not also cover
`C(1371,3) = 428 558 605` (M-2), which is the number a reader most needs?

**Q5.** Does the memo intend `min_opp` to range over windows open for OPP (no own stone)? §5's
wording (*"`min` over open `w`"*) inherits "open" from `open_X`'s definition and my
implementation read it that way; the text never says it twice, and the two readings give
different enums. (The shipped `hexenum.side_terms` agrees with my reading — but a definition
that needs its implementation consulted is B-2's shape again.)

**Q6.** Which reading of §3 is intended (B-2's (A) or (B))? The answer decides `k` at every even
length, decides whether §6.6's derivation is true, and decides whether an even-`L` class indexes
a cell or a pair of adjacent cells. One sentence settles it, and it is not written.

**Q7.** `tools/hex_enum/` is untracked — at no revision. `process.md` binds a registered number
to *"the revision that lands"* its instrument, and §6 says *"at the revision that lands them"*.
What revision does §7 cite, and does the instrument's own review run before or after the memo's
fix round?

---

## 4. What I attacked and it SURVIVED

This section is not a courtesy; several of these were the attacks I expected to land.

1. **§5.4's reversal invariance, claimed a priori.** Enumerated at `L = 11`, all 59 049
   patterns: **0 mismatches** (F4). D-706's a-priori claim is correct and needed no measurement.
2. **§3's pair-of-alignments convention as a mechanism.** It **does** restore reversal
   invariance at even `L`, and under **both** readings B-2 distinguishes: 0 mismatches at
   `L = 6, 8, 10, 12` over the `(L+1)`-cell neighbourhood (reading A, F2), and 0 mismatches at
   the same lengths over `hexenum.patterns(L)` (reading B). Reversal swaps the two alignments
   and the class is their unordered multiset, so invariance follows either way. The mechanism is
   right; what is wrong is that the memo never says which object it applies to (B-2). I looked
   hard for a counterexample and there is none.
3. **§5.1's `r4` redundancy.** `k` = 357 with and without both components (F4). The memo is
   right, is right to say it first, and is right that keeping it changes nothing — the class
   set is identical, not merely the same size (I compared the sets, not the counts).
4. **§5's `|W_L| = min(6, L − 5)`** — exact for L = 6…14 at a single alignment (F1).
5. **§6.2's fold ceiling formula** — exact at L = 7, 9, 11 by enumeration: 378, 3 321, 29 646
   (F8). Its justification for dropping §0.2's `−1` (that the `−1` excludes the all-empty
   *window*, and a single axis of a scored *cell* may legitimately be empty) is sound.
6. **§5.2's range derivation.** `min_X ∈ {1..6, ∞}`; `min_X ≥ 1` because `c` is empty and must
   be filled; `|X| = 6` impossible in a window holding the empty `c`; `open_X = 0 ⟺ min_X = ∞`.
   All correct; the bound 37² = 1 369 holds against the realised 357.
7. **`min_X` counting `c` in the cost** — the dispatch's own suspicion. It is right at every
   reading I could construct: `min_own = 1` is exactly rule 4's *"one placed stone completes
   six"*, and `min_opp = 1` is exactly "the opponent wins by playing here", because `c` is empty
   for both sides. `min_X = 6` means every open window through `c` is all-empty, which is a real
   and distinct state and not an accident of the encoding.
8. **§6.5's `η²` is computable as defined.** Within `= Σ_C (n_C/N)·Var(label | C)`, between
   `= Σ_C (n_C/N)·(mean_C − mean)²`, and their sum is the population variance exactly; self-check
   3 will pass to machine precision. The *honest caveat* paragraph about non-independence and the
   effective `n` being nearer 3 487 than the observation count is correct and is the kind of
   thing most documents omit.
9. **§8.** The `LAW-DECOMP` limitation is quoted against the memo's own family, extended to
   `handcrafted_v0`, and not softened. This is the paragraph a selectively-quoting document
   would not have written.
10. **§1.1's identification of the import.** `C(18,3) = C(16+2,3)`, cross-checked against
    Rapfi's `PCODE_NB = C(19,4) = 3 876`: the arithmetic is right and the memo is the only
    document in this repository that names 816 as a consequence of an imported `k` rather than
    as a hex figure. B-3 is that the ladder then re-derives it by a different route; it is not
    that §1.1 is wrong.
11. **Every `file:line` and every verbatim quotation.** 13 calculus anchors, 17 quoted strings,
    4 code anchors, 5 corpus counts, 2 digests — all reproduced at `0585e25` with commands and
    scopes of my choosing (ledger rows 1–16). The memo's author had already fixed one round of
    wrong line numbers; I found no surviving one. The two slips in m-7 are cosmetic.
12. **The `T3`, `T2` and `T4` rungs are genuine quotients of `T4`,** and every rung inherits
    §5.4's invariance as §6.4 claims — checked by construction and by enumeration (F5, F2). The
    ladder's *structure* is sound; B-3 and M-4/M-5 are about two of its four rungs, not about
    the idea of projecting.
13. **§5.5's list of what the tuple omits is accurate about the TUPLE.** No threshold, no name,
    no phase, nothing off-axis: I checked each of the six components and none reads a constant
    other than `WIN_LEN = 6`, which is game rule 2. m-4 is that §6.4 then adds thresholds; §5.5
    itself is true.
14. **The corpus counts, re-derived at a scope the memo did not take.** `sort -u` over the
    non-comment manifest lines returns 89 805, so the file is genuinely deduped and not merely
    89 805 lines long; and the 3 487 figure is confirmed independently by
    `book_v3_ledger.md:51`'s disjointness control, which the memo does not cite.

---

## 5. VERDICT: **FAIL**

Three BLOCKING, seven MAJOR, eight MINOR. **The document's premise discipline is the best this
reviewer has seen in this repository** — every `file:line`, every quotation, every corpus count
reproduced at a scope of my choosing — and the failure is in none of them. It is in what §3, §6.4
and §6.6 **license**. Two of D-706's five receipts would be artifacts of the definition rather
than facts about hex, and the third would land on the imported figure through an ambiguity the
memo never resolves. The proof that these are not pedantic readings is that the implementing
session, writing `tools/hex_enum/` from this very document within the hour, had to resolve all three
ambiguities — and resolved every one of them **against the memo's own words**: it enumerates
`3^(L−2)` where §6.1 says `3^(L−1)`, it keeps `∞` out of T1's clip where §6.4 says *"clipped at
≥ 4"*, and it steps `L → L+2` where §6.1 says the test *"needs `k(L+1)` beside `k(L)`"*. A
definition that its own author's instrument cannot follow is not yet a definition.

### The shortest route to a fix, and it is deletion in six of nine places

1. **Delete the even-`L` convention and every even `L`** (B-2). Enumerate odd
   `L ∈ {7, 9, 11, 13}` only. This deletes B-2 whole; it makes §5's `|W_L|`, §6.2's ceiling,
   §6.7's self-check 4 and §6.1's `3^(L−1)` all correct as written; and it removes the parity
   artifact that produces B-1(c). Nothing is lost — the only reason the memo wanted even `L` is
   §6.6's `L+1` step, which item 2 deletes, and the instrument already steps by 2.
2. **Delete the stabilisation length as a receipt** (B-1), and amend D-706's receipt list to
   drop it. On odd steps its value is 11 by §5's one-line derivation and can be nothing else.
   Keep the merge counts for `L < 11` and the instrument's *"neither refines the other"* line —
   those are measured and do carry content. Replace §1.2's *"the owed enumeration §6 discharges"*
   with the question the memo actually answers (M-7).
3. **Delete the T1 rung** (B-3), or state the `∞` rule and move the clip to `≥ 5`. Deleting is
   stronger: T1's `k` is capped by its own clip, and T2/T3/T4 already span 47 → 357.
4. **Delete T2's two boolean components** (M-4) — §5.1's own rule, applied to §6.4.
5. **Delete T3's justification sentence** (M-5), replacing it with the derived fact that
   single-axis DEF-T never exceeds 2 (F6) — the honest reason `open` may be clipped low.
6. **Add one line to §5.2** (M-2): `C(1371,3) = 428 558 605` at the memo's own bound, beside the
   816, so the reader's only anchor is not the import.
7. **§6.5** (M-3): register what the output must show, or delete the `process.md` citation and
   say plainly that §6.5 is a D-614 diagnostic. Relabel referent 1 as the identity it is.
8. **§1 header** (M-6): re-derive the memo against a revision that contains D-706, or state on
   the memo's face that the ADR and the memo are one act and that compliance with it is
   therefore not evidence.
9. **Re-dispatch the instrument's own review** once 1–3 land — `process.md`'s *"Instrument
   governing revision"* binds it, `tools/hex_enum/` is currently untracked (at no revision at
   all), and `tools/SHELL_CHECKLIST.md`'s coverage rule reaches `tools/hex_enum_tests.sh`. This
   report is not that review.

**The one item that is not a deletion, and is not overrulable under D-424**, is M-1. The tuple
does not separate `t = 1` from `t = 2` at the simplest four-shapes in the game — a way the enum
gives a wrong answer, not prose that constrains nothing. It need not be fixed by a new component:
recording the witness in §5.5 beside *"what the tuple deliberately does NOT carry"*, so the
matrix prices the row knowing it, is enough. What is not enough is §7 landing without it.

---

## Appendix A — the reproducer, inlined so it survives the scratchpad

Pure stdlib, no project code, written from the memo's prose. `python3 <this>`; ≈4 min.
Outputs are tagged `F1`…`F8` and are cited by tag throughout.

```python
from itertools import product, combinations
from collections import defaultdict
WIN, INF = 6, 99

def wins_through(L, m):
    return [tuple(range(s, s+WIN)) for s in range(L-WIN+1) if s <= m <= s+WIN-1]

def half(pat, ws, me, other):
    op, best = 0, INF
    for w in ws:
        cs = [pat[i] for i in w]
        if other in cs: continue
        op += 1
        best = min(best, WIN - cs.count(me))
    return best, op, 1 if best == 1 else 0

def T4(pat, m):                       # memo §5's 6-tuple
    ws = wins_through(len(pat), m)
    return half(pat, ws, 1, 2) + half(pat, ws, 2, 1)

def odd_pats(L):                      # memo §3, odd L: centre fixed empty
    m = (L-1)//2
    for r in product((0,1,2), repeat=L-1):
        yield r[:m] + (0,) + r[m:], m

def pair_cls(L, s):                   # memo §3, even L: s holds L+1 cells
    h = L//2
    return frozenset([T4(s[1:L+1], h-1), T4(s[0:L], h)])

def dt(pat, me=1, other=2):           # DEF-T: exact min hitting set over DEF-PLAN
    fam = set()
    for s in range(len(pat)-WIN+1):
        w = pat[s:s+WIN]
        if other in w or w.count(me) < 4: continue
        e = frozenset(s+i for i, v in enumerate(w) if v == 0)
        if len(e) <= 2: fam.add(e)
    if not fam or frozenset() in fam: return 0
    u = sorted(set().union(*fam))
    for k in range(1, len(u)+1):
        for p in combinations(u, k):
            if all(set(p) & f for f in fam): return k
    return len(u)

def C3(k): return (k+2)*(k+1)*k//6
def show(p): return "".join(".XO"[v] for v in p)

print("F1  |W_L| = min(6, L-5):",
      all(len(wins_through(L, (L-1)//2)) == min(6, L-5) for L in range(6, 15)))

print("\nF2  k(L) under the memo's OWN conventions")
seq = {}
for L in range(6, 15):
    if L % 2:
        seq[L] = len({T4(p, m) for p, m in odd_pats(L)}); read, strs = L, 3**(L-1)
    else:
        h = L//2
        seq[L] = len({pair_cls(L, r[:h]+(0,)+r[h:]) for r in product((0,1,2), repeat=L)})
        read, strs = L+1, 3**L
    print(f"     L={L:2d}  cells read={read:2d}  strings={strs:<8d} k={seq[L]:4d}"
          f"   §6.1 claims 3^(L-1)={3**(L-1)}")
print("     sequence:", [seq[L] for L in range(6, 15)])

print("\nF3  §6.6 criterion: smallest L at which the L-class DETERMINES the (L+1)-class")
CEN = 6
def cls_at(s, L):
    if L % 2:
        m = (L-1)//2; return T4(s[CEN-m:CEN+m+1], m)
    h = L//2
    return frozenset([T4(s[CEN-(h-1):CEN+h+1], h-1), T4(s[CEN-h:CEN+h], h)])
fan = {L: defaultdict(set) for L in range(6, 13)}
for r in product((0,1,2), repeat=12):
    s = r[:CEN] + (0,) + r[CEN:]
    prev = cls_at(s, 6)
    for L in range(6, 13):
        nxt = cls_at(s, L+1); fan[L][prev].add(nxt); prev = nxt
for L in range(6, 13):
    mx = max(len(v) for v in fan[L].values())
    print(f"     L={L:2d} -> {L+1:2d}: max (L+1)-classes per L-class = {mx:2d}"
          f"  {'DETERMINES' if mx == 1 else 'splits'}")

L, m = 11, 5
allp = [p for p, _ in odd_pats(L)]
allt = [T4(p, m) for p in allp]
k11 = len(set(allt))
print(f"\nF4  k(11) = {k11};  C(k+2,3) = {C3(k11):,}  (45 271 quiet positions"
      f" => {45271/C3(k11):.5f} obs/param)")
print(f"     memo §5.2's own bound k <= 37^2 = 1369  ->  C(1371,3) = {C3(1369):,}")
print(f"     reversal mismatches at L=11: {sum(1 for p in allp if T4(p[::-1], m) != T4(p, m))}")
print(f"     r4 redundancy: k without r4 = {len({(t[0],t[1],t[3],t[4]) for t in allt})}")

print("\nF5  coarsening ladder (§6.4)")
for n, f in [("T4", lambda t: t),
             ("T3 open clipped at >=2", lambda t: (t[0], min(t[1],2), t[3], min(t[4],2))),
             ("T2 as written", lambda t: (t[0], t[3], int(t[1] >= 1), int(t[4] >= 1))),
             ("T2 with the two booleans deleted", lambda t: (t[0], t[3])),
             ("T1 min clipped at >=4", lambda t: (min(t[0],4), min(t[3],4))),
             ("T1 clipped at >=5 instead", lambda t: (min(t[0],5), min(t[3],5)))]:
    kk = len({f(t) for t in allt}); print(f"     {n:34s} k={kk:4d}  C(k+2,3)={C3(kk):>11,}")

print("\nF6  classes MERGING different exact threat numbers after a stone at c")
b = defaultdict(list)
for p in allp: b[T4(p, m)].append((p, dt(p[:m] + (1,) + p[m+1:])))
sp = [c for c, v in b.items() if len({t for _, t in v}) > 1]
print(f"     {len(sp)} of {k11} classes; {sum(len(b[c]) for c in sp)} of {3**10} patterns")
for c in [(3,6,0,6,1,0), (2,6,0,6,1,0)]:
    d = {p: t for p, t in b[c]}
    lo = min((p for p in d if d[p] == 1), key=lambda p: sum(1 for v in p if v))
    hi = min((p for p in d if d[p] == 2), key=lambda p: sum(1 for v in p if v))
    print(f"     class {c}:  {show(lo)} t={d[lo]}   vs   {show(hi)} t={d[hi]}")
print("     max DEF-T over ALL 3^11 single-axis lines:",
      max(dt(p) for p in product((0,1,2), repeat=11)))

print("\nF7  own/opp swap equivariance")
sw = lambda p: tuple({0:0, 1:2, 2:1}[v] for v in p)
mir = lambda t: (t[3], t[4], t[5], t[0], t[1], t[2])
print("     mismatches:", sum(1 for p in allp if T4(sw(p), m) != mir(T4(p, m))))
cls = set(allt); fx = sum(1 for t in cls if t == mir(t))
print(f"     swap-fixed classes {fx}; orbits {(len(cls)+fx)//2}")

print("\nF8  §6.2 fold formula, enumerated")
for L in (7, 9, 11):
    print(f"     L={L}: enumerated {len({min(p, p[::-1]) for p, _ in odd_pats(L)})}"
          f"  formula {(3**(L-1) + 3**(L//2))//2}")
```

Expected output (this reviewer's run, single thread):

```
F1  |W_L| = min(6, L-5): True
F2  sequence: [33, 25, 177, 98, 920, 357, 357, 357, 357]
F3  L=6->7 DETERMINES; 7->8 splits(20); 8->9 DETERMINES; 9->10 splits(36);
    10->11 DETERMINES; 11->12 DETERMINES; 12->13 DETERMINES
F4  k(11) = 357; C(k+2,3) = 7,647,059; 0.00592 obs/param; C(1371,3) = 428,558,605;
    reversal mismatches 0; k without r4 = 357
F5  T4 357 / 7,647,059 · T3 121 / 302,621 · T2 47 / 18,424 · T2-minus-booleans 47 / 18,424
    · T1(>=4) 16 / 816 · T1(>=5) 25 / 2,925
F6  27 of 357 classes; 5348 of 59049 patterns; max single-axis DEF-T = 2
F7  0 mismatches; 17 swap-fixed; 187 orbits
F8  378 / 3321 / 29646, formula agrees
```
