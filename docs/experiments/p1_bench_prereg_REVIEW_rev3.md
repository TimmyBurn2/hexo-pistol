# REVIEW — `docs/experiments/p1_bench_prereg.md` (round 3 of this gate, REMEDIES-ONLY)

| | |
|---|---|
| document | `docs/experiments/p1_bench_prereg.md`, **revision 3** per its own title |
| revision reviewed | `189c6d6e73e17deab45cc36ffb8aa929f4a8f7a7` (a `git stash create` commit whose first parent is `dev`) |
| matches the working tree? | **yes, verified at the start AND at the end of the review** — `git show 189c6d6e…:<path> \| sha256sum` against `sha256sum <path>` for the document and all four pinned siblings; all five MATCH both times (the F-P1.14 guard: nothing moved underneath this reviewer) |
| HEAD at review | `ffc5c10f4d16356f574e3221a023f78399d2e3bb` (`dev`); the document is untracked at HEAD (`git cat-file -e ffc5c10:…` → *"exists on disk, but not in `ffc5c10`"*) |
| diff adjudicated | `git diff 1ee0ddd91b6e0cee095fd17b4b7848b1cd6281d7 189c6d6e73e17deab45cc36ffb8aa929f4a8f7a7 -- docs/experiments/p1_bench_prereg.md` |
| round | **round 3, remedies-only (D-597), the last round the loop grant holds** (`opt_arc_ledger.md:15-16`). Round 1: `p1_bench_prereg_REVIEW.md`. Round 2: `p1_bench_prereg_REVIEW_rev2.md` (*"VERDICT: FAIL (B3; MAJOR M4, M5, M6)"*, nine minors) |
| date | 2026-09-04 |
| model | Opus 5 (1M context) — recorded per D-597's remedies-only clause |
| execution | the registered extraction command was RUN; §2.1's block was EXECUTED from the live tree root against the two prebuilt binaries; the registered receipt line was tested in five spellings across three box states; the three drift artifacts were digested and re-read; all fourteen §0 revisions were re-derived. **No `cargo` was run anywhere** (another agent's `cargo test --workspace --locked` was live in `/home/tom/pistol-wt/review-p1-impl`, pid 1730762, and a second agent's `…/review-p1-impl-mut/target/release/pistol` was cycling configs throughout); **nothing under `/home/tom/pistol-wt` was written**; `git status --porcelain` is byte-identical before and after everything below except this report. Everything adjudicated used `/usr/bin/grep`, `git show`/`git log` pinned to `ffc5c10` or to the stash, `LC_ALL=C sort`, or my own `awk`/`python3`/`sha256sum` (D-265, D-602) |

**VERDICT: FAIL (B4; MAJOR M7)**

Twelve of the thirteen remedies hold, and eleven of them hold **on execution rather than on
reading**: the block runs, the receipt distinguishes all three of its cases without killing a
`set -e` script, the extraction command yields exactly the block whose digest §2.4 records, the
drift arithmetic re-derives exactly, D-603's bracket and gate count are corrected, and all
fourteen instrument revisions — the six new gate-script rows included — are their own
`git log -1` answer. The thirteenth remedy, B3's, fixed the one-band table completely and then
reintroduced the identical defect one level up, in the sentence added to reconcile the two
bands' different brackets.

---

## 1. Remedy table

| round-2 finding | remedy in revision 3 | what I ran | holds? |
|---|---|---|---|
| **B3** — `[1.10, 1.15)` licensed both to land and to STOP; `[1.15, 1.190)` unassigned; *"a fifth"* arithmetically false | §1.1 replaced the abort paragraph with a four-row interval table plus a two-band governing sentence; the "a fifth" clause replaced by **38.9 %–62.5 %** | exhaustion over 1.05 / 1.10 / 1.14 / 1.15 / 1.19 / 1.25 / 1.31 in **both** bands against `[1.207, 1.307]` and `[1.190, 1.290]` (my own `python3`, table §3.1); `python3` on `0.10/0.257` and `0.15/0.240` | **the one-band table: YES, all 14 checks land in exactly one row.** The **two-band rule: NO — it is the new B4** |
| **M4** — recorded block digest belonged to an extraction wrapper; a second `sh` fence made "the block" ambiguous | §1.3's receipt made inline so §2.1's is the only `sh` fence; §2.1 REGISTERS `sed -n '/^```sh$/,/^```$/p' … \| sed '1d;$d'`; §2.4 records `block sha256 2755b900…` | `/usr/bin/grep -n '^```'` on the pinned revision → exactly **two** fence lines (`286`, `336`), so exactly one block; ran the registered command verbatim from the live tree AND pinned via `git show` — both give `2755b900d0475a370b1cf23993f87f3efedc3a00f5848b75612bd01d72a49cad`; **then executed the block** (§2 below) | **yes, by execution.** Digest matches §2.4 exactly; `artifacts/p1_identity_dryrun_E_v3.txt` (`09b615f2…`, my digest matches) records that same block digest on its own second line |
| **M5** — ±0.03's stated ground (0.001/0.001) falsified by the run added in the same section | §1.2 re-states the ground as **0.002 early / 0.010 late** over all three runs, "±0.03 is three times the wider"; §1.1's ground re-stated as "five times the widest spread … (0.010)" | `sha256sum` all three artifacts (all match `d0b1b384…`, `e3eaed80…`, `a2f20ad5…`); my own `/usr/bin/grep -nE 'nps ratio'` over each whole file; `python3` on the extremes | **first limb yes** — early spread `1.258-1.256 = 0.002`, late `1.241-1.231 = 0.010`; `0.03/0.010 = 3.0`, `0.05/0.010 = 5.0`, both exact. **Second limb no — it is M7** |
| **M6** — D-603 cited the withdrawn `[1.20, 1.30]` and "the five gates that read this state" | D-603's slot paragraph now names the per-band brackets; the gate sentences rewritten | read D-603 whole at the pinned revision; `git show ffc5c10:tools/ci.sh \| awk '/^step /{n++}…'` for gates 8–13 and their `gate "<name>" tools/<script>` lines; `readonly GATE_TOTAL=20` | **yes.** D-603 now reads *"the per-band brackets [1.207, 1.307] early and [1.190, 1.290] late"* — §1.1's two rows exactly — and *"all six gates the pre-registration names green by their own log lines"* at the landing revision. Its three slots match §1/§2/§3 (§3.4 below) |
| **n1** — `bench_delta.sh:91` does not resolve | `:94` | `git show ffc5c10:tools/bench_delta.sh \| /usr/bin/grep -nE '^(CONFIG\|WEIGHTS\|FIXTURE\|EARLY_MAX)='` → `92`/`93`/`94`/`99` | **yes** (`:93`, `:272`, `EARLY_MAX=17` all correct too) |
| **n2** — §1.3 pointed at §1.1 for an artifact §2.2 names | all three artifact names listed together at §1.3:247-250; §2.2 points there | `/usr/bin/grep -n 'artifacts/'` over the whole pinned file | **yes** — one place, three names, and §2.2 now says *"the three artifact names are listed together in §1.3"* |
| **n3** — cost paragraph priced the §1.2 dry run as unspent | *"§1.2's dry run and §2.4's are TAKEN and cost about six minutes between them; what remains unspent is the landing bench, the identity leg and its falsifier"* | read at §0:81-84 | **yes** |
| **n4** — receipt prints `idle` on any `pgrep` failure | three-way `case` on `rc`, with `out="$(…)" \|\| rc=$?` | five executions across three box states (§3 below) | **yes, by execution** — `idle` only on exit 1; `RECEIPT BROKEN: pgrep exited 2` and `… 127` on the two failure modes |
| **n5** — VOID needs a judgement and §2.1's receipt had no consequence | new §1.3 paragraph: VOID is a reader's judgement, scoped to **timing runs only**, *"a non-idle receipt there voids nothing"* | read at §1.3:232-240; and **demonstrated**: my execution of §2.1 printed two **non-idle** receipts and still produced `284f70af…`, the digest the idle v3 run produced | **yes** — and the paragraph's own ground (*"a busy box cannot change a `bestmove`"*) is now measured, not asserted: two runs of the same block, one idle and one on a box running a full `cargo test` plus a live engine, same transcript digest |
| **n6** — paragraph break mid-sentence in §1.2 | sentence rejoined | `awk 'prev=="" && $0=="" {print NR} {prev=$0}'` over the whole file → nothing | **yes**, and no double blank line remains anywhere |
| **n7** — the v2 artifact carried three `idle` lines where §2.4 said "both" | v3 artifact re-taken from the registered extraction | `cat artifacts/p1_identity_dryrun_E_v3.txt` → exactly **two** `idle` lines, `BLOCK_EXIT=0` | **yes** |
| **n8** — §0's completeness sentence did not cover what §3 runs | six gate-script rows added, each with a revision, plus *"each revision is `git log -1 --format=%h ffc5c10 -- <path>`'s answer and not a hand-written prefix"* | **all fourteen rows re-derived with my own command** (§3.3 below) | **yes — 14/14 OK**, including all six new ones, and the six scripts are exactly what `ci.sh` invokes at gates 8–13 under exactly the six names §3 uses. Two new minors ride in with it (n10, n14) |
| **n9** — the document narrated its rounds in six places | one header paragraph, *"THE GATE'S HISTORY IS HERE AND NOWHERE ELSE"*; the four narrating paragraphs round 2 quoted are gone | `/usr/bin/grep -nE "round [0-9]\|round-[0-9]\|revision [0-9]"` over the pinned file | **substantively yes** (all four quoted paragraphs deleted or rewritten) — **but the exclusivity sentence is false, n13** |

---

## 2. Execution record

### 2.1 The registered extraction (M4)

```
$ /usr/bin/grep -n '^```' <(git show 189c6d6e…:docs/experiments/p1_bench_prereg.md)
286:```sh
336:```
```

Two fence lines in the whole document, so the registered `sed` range can only pair one block.

```
$ sed -n '/^```sh$/,/^```$/p' docs/experiments/p1_bench_prereg.md | sed '1d;$d' > block_live.sh
$ wc -l block_live.sh                      →  49
$ sha256sum block_live.sh
2755b900d0475a370b1cf23993f87f3efedc3a00f5848b75612bd01d72a49cad

$ git show 189c6d6e…:docs/experiments/p1_bench_prereg.md \
    | sed -n '/^```sh$/,/^```$/p' | sed '1d;$d' | sha256sum      # D-602: pinned, not the tree
2755b900d0475a370b1cf23993f87f3efedc3a00f5848b75612bd01d72a49cad
```

§2.4 records `block sha256 2755b900…`. **It matches**, from the live tree and from the pinned
revision alike, and the artifact `p1_identity_dryrun_E_v3.txt` carries the same digest on its
own line 2. M4 is closed: the instrument that ran and the instrument that is registered are
now provably the same bytes.

### 2.2 The block, EXECUTED

Run from `/home/tom/Projects/HeXO-AlphaBeta` with
`BASE=/home/tom/pistol-wt/p1-measure/pistol-ffc5c10`,
`CAND=/home/tom/pistol-wt/p1-measure/pistol-E`, read-only.

| check | result |
|---|---|
| binary digests (mine, before the run) | `78a7600adcf099de…` and `f333d1010f520976…` — the two §2.4 names; **they differ**, so the same-binary vacuous pass is refused |
| `bestmove` / `error` per side | `BASE lines 533 bestmove 128 error 0`; `CAND lines 533 bestmove 128 error 0` — **128 and 0 on both sides**, as §2.2 and §2.4 register |
| transcript digest | `284f70afab0ac8f16343da78418b07de19572c1680e58961e8447717647dc3c6` on **both** sides — the `284f70af…` §2.4 claims |
| result / exit | `RESULT: IDENTICAL`; `BLOCK_EXIT=0` |
| receipts | both printed. Both **non-idle** (the other agents' `cargo test` and `…/review-p1-impl-mut/target/release/pistol`) — which §1.3's n5 remedy expressly says voids nothing here, and which the identical transcript digest confirms |
| `git status --porcelain` across the run | **byte-identical** before and after; `ls out.BASE out.CAND` → no such file; transcripts landed in `/tmp/tmp.cmNw2HPyKF` |

Every registered criterion in §2.2 and §2.4 holds on my own execution.

### 2.3 The receipt line, tested three ways (plus two counterexamples)

Line 213 (§1.3, backticks stripped) and line 294 (inside the block) are **byte-identical**
(`cmp` → identical), so there is one spelling. All five runs below are `bash <file>` where the
file begins `set -euo pipefail`.

| case | how I produced it | printed | script survived? | exit |
|---|---|---|---|---|
| **(a) pattern matches** | the registered pattern on this box (a live `cargo test`, a live `target/release/pistol`) | the matching command lines | **yes** | 0 |
| **(b) pattern matches nothing** | same shape, alternatives `[z]zqqx1\|…` | `idle` | **yes** | 0 |
| **(c1) `pgrep` fails, exit 2** | invalid regex `pgrep -af '['` | `RECEIPT BROKEN: pgrep exited 2` | **yes** | 0 |
| **(c2) `pgrep` fails, exit 127** | `PATH` set to an empty directory | `RECEIPT BROKEN: pgrep exited 127` | **yes** | 0 |
| **(d) counterexample — the condemned `{ pgrep … \|\| echo idle; }` on a broken `pgrep`** | `pgrep -af '['` | **`idle`** | yes | 0 |
| **(e) counterexample — a truly bare `pgrep` under `set -e`** | `pgrep -af '[z]zqqx1'` with no guard | nothing | **NO — killed before the next line** | **1** |

**The registered line distinguishes all three states and kills nothing.** (d) reproduces
exactly the exit-0-wrong-answer shape §1.3 attributes to the old spelling — a broken `pgrep`
reported as a quiet box — and (e) reproduces exactly the failure §2.4 narrates as revision 3's
first execution: a bare `pgrep` returning 1 exits a `set -euo pipefail` script. Both grounds
§1.3 states are measured true. `SHELL_CHECKLIST.md` item 12 resolves at `ffc5c10`
(*"## 12. A GATE DISTINGUISHES **RUN VOID** FROM **FAIL**, BY NAME"*, whose sub-item 1 is the
code-per-kind rule), so the citation is accurate.

---

## 3. Re-derivation

Every number below was produced with a command I chose, at a named revision, printed with its
scope. No row was produced by re-running a command the document supplies (docs/process.md's
re-derivation clause) — except §2.1's extraction, which the round exists to adjudicate and
which I also ran in a second, pinned form.

### 3.1 The interval table, by exhaustion (item 1)

Scope: `python3`, my own predicate per row, against `[1.207, 1.307]` and `[1.190, 1.290]`.

| ratio | early → rows matched | late → rows matched |
|---|---|---|
| 1.05 | 1 — ABORT | 1 — ABORT |
| 1.10 | 1 — BELOW BRACKET (lands) | 1 — BELOW BRACKET (lands) |
| 1.14 | 1 — BELOW BRACKET | 1 — BELOW BRACKET |
| 1.15 | 1 — BELOW BRACKET | 1 — BELOW BRACKET |
| 1.19 | 1 — BELOW BRACKET | 1 — **PASS** (`1.19 = 1.190`, the closed lower bound) |
| 1.25 | 1 — PASS | 1 — PASS |
| 1.31 | 1 — ABOVE BRACKET | 1 — ABOVE BRACKET |

**Fourteen of fourteen fall in exactly one row.** No interval is doubled and none is
unassigned, in either band; the `< 1.10` / `1.10 to …` boundary is disjoint and exhaustive, and
`[1.15, 1.190)` — round 2's unassigned interval — now has a disposition in both bands. The
single-band half of B3 is genuinely and completely remedied.

### 3.2 The 38.9 % / 62.5 % arithmetic (item 1)

```
$ python3 -c "print('%.4f %.4f' % (0.10/0.257*100, 0.15/0.240*100))"
38.9105 62.5000
```

Both exact. The pair is the correct envelope **across both bands over the whole interval**:
the minimum retained fraction anywhere in `[1.10, 1.15)` is the early band's at 1.10 (38.9 %)
and the supremum is the late band's at 1.15 (62.5 %); the intermediate values (late at 1.10 =
41.7 %, early at 1.15 = 58.4 %) both lie inside. Round 2's "two to three fifths" is what the
document now prints. One nit only, not rated: `[1.10, 1.15)` is half-open, so 62.5 % is an open
supremum the interval never attains.

### 3.3 Every §0 instrument revision, re-derived (item 6, n8)

`git log -1 --format=%h ffc5c10 -- <path>`, mine, one row per §0 row:

| path | mine | document | |
|---|---|---|---|
| `tools/bench_delta.sh` | `ab369b0` | `ab369b0` | ✔ |
| `configs/instrument_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` | `70cc465` | `70cc465` | ✔ |
| `configs/eval_v0_weights.toml` | `64c336f` | `64c336f` | ✔ |
| `tools/ci.sh` | `9390b48` | `9390b48` | ✔ |
| `configs/tactical_staged_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `configs/gate_staged_solver_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt` | `538b3e5` | `538b3e5` | ✔ |
| **`tools/determinism.sh`** | `7fbc1ff` | `7fbc1ff` | ✔ new |
| **`tools/tactical_check.sh`** | `0a80a7b` | `0a80a7b` | ✔ new |
| **`tools/search_oracle_check.sh`** | `55e4b56` | `55e4b56` | ✔ new |
| **`tools/staged_soundness_check.sh`** | `8ce13ff` | `8ce13ff` | ✔ new |
| **`tools/solver_oracle_check.sh`** | `e668dfa` | `e668dfa` | ✔ new |
| **`tools/solver_determinism.sh`** | `3916afd` | `3916afd` | ✔ new |

**14 of 14.** And the six new rows' gate attributions are right, which the document asserts and
did not show:

```
$ git show ffc5c10:tools/ci.sh | awk '/^step /{n++} n>=8 && n<=13' \
    | /usr/bin/grep -nE '^step |tools/[a-z_]+\.sh'
gate  8 "tactical fixture"     tools/tactical_check.sh
gate  9 "determinism"          tools/determinism.sh
gate 10 "search oracle"        tools/search_oracle_check.sh
gate 11 "staged soundness"     tools/staged_soundness_check.sh
gate 12 "solver oracle"        tools/solver_oracle_check.sh
gate 13 "solver determinism"   tools/solver_determinism.sh
$ git show ffc5c10:tools/ci.sh | /usr/bin/grep -n 'GATE_TOTAL='
21:readonly GATE_TOTAL=20
```

Each of the six gate labels is byte-for-byte the name §3 uses, and `GATE_TOTAL=20` is §3's
"twenty gates".

### 3.4 D-603's three slots (item 5, M6)

Read whole at the pinned revision.

| slot | D-603 | prereg | |
|---|---|---|---|
| 1 | `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5` → `<nps early>` / `<nps late>` against **the per-band brackets [1.207, 1.307] early and [1.190, 1.290] late** | §1's instrument bullet + §1.1's two rows | ✔ — the withdrawn `[1.20, 1.30]` is gone |
| 2 | the identity leg over **128 searches at three seats** → `<RESULT>` | §2's bound, §2.1's three `SEATS` lines, §2.2's 128/0 criterion | ✔ |
| 3 | **CI's twenty gates** at `<landed>` | §3's six named gates + *"The full `tools/ci.sh` at `<landed>`, twenty gates, is the closure's receipt"* | ✔ |

Gate count: D-603's remaining "five" is now scoped — *"the five gates REVIEW-impl round 1
ran"* — which is accurate (`p1_impl_REVIEW.md:363` is headed *"Check 6 — the five gates that
read this state"*), and the landing-revision sentence reads *"all six gates the
pre-registration names green by their own log lines"*. M6 is closed on both limbs.

### 3.5 The other new claims in the diff

| claim (new in the diff) | my check | |
|---|---|---|
| `bench_delta.sh:94` is `FIXTURE=` | `/usr/bin/grep -nE '^(CONFIG\|WEIGHTS\|FIXTURE)='` at `ffc5c10` → 92/93/94 | ✔ (n1 closed) |
| `:452` prints `VERDICT ABORT` below 1.15 | `git show ffc5c10:tools/bench_delta.sh \| sed -n '452p'` | ✔ |
| …and `BELOW-BRACKET` below 1.4 | that branch is at **`:453`** | ✘ **n11** |
| `determinism.sh` "cited three times here (`:76`, `:85`, `:204`)" | my `/usr/bin/grep -nE 'determinism\.sh:[0-9]+'` over the whole pinned file → `:76` at 41, `:76` at 102, `:76` at 282, `:204` at 299; **`:85` appears nowhere but inside the claim itself** | ✘ **n10** |
| `SHELL_CHECKLIST.md` item 12's shape | `git show ffc5c10:tools/SHELL_CHECKLIST.md \| /usr/bin/grep -nE '^#'` → `## 12. A GATE DISTINGUISHES **RUN VOID** FROM **FAIL**, BY NAME` | ✔ |
| three artifacts' digests | `sha256sum` all three → `d0b1b384…`, `e3eaed80…`, `a2f20ad5…` | ✔ |
| `p1_identity_dryrun_E_v3.txt` `09b615f2…` | `sha256sum` → `09b615f2310dead2…` | ✔ |
| "Nothing else in this document is a slot" | `/usr/bin/grep -o '<[a-z][a-z ]*>' \| LC_ALL=C sort \| uniq -c` → `9 <landed>` **and `1 <path>`** | ✘ **n14** |

---

## 4. New findings

### BLOCKING

#### B4 — the sentence added to reconcile the two bands' different brackets states three incompatible rules, and a landing that keeps ~80 % of the prototype's gain falls in the conflicted zone

The remedy for B3 replaced a prose paragraph with a table, which is right, and then added one
sentence to say how the two bands combine. As printed (§1.1:117-119):

> **THE DISPOSITION, ONE ROW PER INTERVAL, THE WORST BAND GOVERNING.** Read the lower of the
> two bands' ratios against its own band's bracket; where the two bands fall in different rows,
> the lower row governs the package.

Three rules are stated in twenty-eight words: **(a)** the heading — the WORST band governs;
**(b)** clause one — the band with the LOWER RATIO is the one read; **(c)** clause two — where
the rows differ, the LOWER ROW governs. Because the two brackets have different floors
(`1.207` vs `1.190`, seventeen thousandths apart — the M1 remedy's whole point), **a lower
ratio is not the worse band**, and (a)/(b)/(c) come apart.

**Minimal reproducer 1 — PASS against BELOW BRACKET, at the most likely landing outcome there
is.**

```
$ python3 -c "
early, late = 1.205, 1.192
print('early', early, 'vs [1.207,1.307] ->', 'BELOW BRACKET' if early < 1.207 else 'in')
print('late ', late,  'vs [1.190,1.290] ->', 'PASS' if 1.190 <= late <= 1.290 else 'out')
print('(b) lower of the two ratios is', min(early,late), '(late) -> read against [1.190,1.290] -> PASS')
print('(a)/(c) the worse band / the earlier row is early -> BELOW BRACKET, a FINDING')"
early 1.205 vs [1.207,1.307] -> BELOW BRACKET
late  1.192 vs [1.190,1.290] -> PASS
(b) lower of the two ratios is 1.192 (late) -> read against [1.190,1.290] -> PASS
(a)/(c) the worse band / the earlier row is early -> BELOW BRACKET, a FINDING
```

One measurement, two dispositions. And this is not a contrived corner: `1.205 / 1.192` is
**exactly a landing that keeps 80 % of the prototype's gain in both bands** —
`1 + 0.80×0.257 = 1.2056`, `1 + 0.80×0.240 = 1.1920`. The window where the conflict lives is
`f ∈ [0.7917, 0.8054)` of retained gain for an equal-fraction landing, and more generally any
early ratio in `[1.190, 1.207)` with the late ratio below it — which is the ordering all three
banked runs show. A landing that recovers most but not all of the prototype's gain is the
single most likely non-PASS outcome this bench has, and it is the outcome the paragraph cannot
dispose of.

**Minimal reproducer 2 — PASS against "explain it or the package STOPs", under EVERY reading of
"the lower row".**

```
$ python3 -c "
early, late = 1.310, 1.280
print('early', early, '> 1.307 -> ABOVE BRACKET: names it, or the package STOPs')
print('late ', late,  'in [1.190,1.290] -> PASS')
print('(b) lower ratio is', min(early,late), '(late) -> PASS ; the above-bracket band is never read')
print('(c) lower row: earlier-in-table = ABOVE BRACKET; further-down-the-page = ABOVE BRACKET')"
early 1.31 > 1.307 -> ABOVE BRACKET: names it, or the package STOPs
late  1.28 in [1.190,1.290] -> PASS
(b) lower ratio is 1.28 (late) -> PASS ; the above-bracket band is never read
(c) lower row: earlier-in-table = ABOVE BRACKET; further-down-the-page = ABOVE BRACKET
```

Here `[1.10, 1.15)`'s successor has arrived: clause (b) says PASS and clauses (a) and (c) say
*"the results document names it, or the package STOPs"* — and "the lower row" is ambiguous in
its own right (the four rows are in ABORT / BELOW / PASS / ABOVE order, which is neither a
severity order nor a ratio order, so "lower" cannot be resolved from the table's shape).

**Why BLOCKING and not MAJOR.** This is round 2's B3 exactly: a single pair of measured numbers
for which the pre-registration licenses two different dispositions, one of them a STOP, with
nothing but a reading choice between them — and D-374 forbids making that choice after the
numbers are seen. It is the defect the round exists to remove, reintroduced one level up by the
remedy for it, in the one sentence written to handle what round 1's M1 established. The
document's own claim two paragraphs later, *"No interval has two dispositions and none is
unassigned"*, is true of one band read alone and false of the pair the run actually produces.

**What would clear it.** One rule, once. The natural one, and the one the heading already
intends: *read each band's ratio against its own band's bracket; the worse of the two
dispositions governs the package*, with the severity order of the four rows stated explicitly
(ABORT worst, then ABOVE BRACKET and BELOW BRACKET as findings, then PASS) so "worse" is not
inferred from the table's layout. Delete "the lower of the two bands' ratios" and "the lower
row" — both are proxies for severity that the per-band brackets break.

### MAJOR

#### M7 — round 2's M5 second limb was not taken, and the M5 remedy turned it into a contradiction between §0 and §1.2

Round 2's M5 had two limbs. The first — the falsified drift ground — is fully and exactly
remedied (§3 above). The second was:

> §0 answers `docs/process.md`'s cheap-run replication rule with *"Replication is the 5 reps the
> terms fix."* The 5 reps are **inside one invocation** … They cannot see between-run drift,
> which is the only drift ±0.03 and ±0.05 exist to absorb … Either register a second
> landing-bench run with an agreement criterion and its registered consequence, or state that a
> half-width of 0.05 against a demonstrated 0.010 makes a single sample sufficient — but the
> present sentence answers the rule with a replication that is not one.

Revision 3 took neither option. It left §0 untouched and added, in §1.2, a sentence conceding
the premise.

**Minimal reproducer.**

```
$ sed -n '73,74p' <the pinned document>
the IQR gate, the refusal of two identical binary digests). Replication is the
5 reps the terms fix. What §1.1's dry run adds is ATTRIBUTION against a referent

$ sed -n '178,179p' <the pinned document>
(round 2's M5). **Between-run drift is the only drift these allowances are
about, and the 5 reps cannot see it**: the reps bound the noise within one run
```

`git diff 1ee0ddd9… 189c6d6e… -- docs/experiments/p1_bench_prereg.md` shows §0's paragraph is
not in any hunk: the sentence is unchanged from the revision round 2 quoted it from.

So the document now asserts, in the section that answers `docs/process.md`'s
cost-and-replication rule, that replication **is** the 5 reps, and asserts in the section that
grounds the allowances that the 5 reps **cannot see** the variation the allowances exist for.
Both cannot be true, and one of them is the document's answer to a binding methodology rule.
This is D-423's shape — a claim made in two places, diverging — inside the round dispatched to
remove it, and it is the second consecutive round in which the drift ground has produced a
finding.

**Why MAJOR and not BLOCKING.** No number moves and no disposition changes: the landing bench
is a single run under either sentence, and §1.1's ground now states the ratio that would justify
a single sample (`0.05` against a demonstrated `0.010`, five times). The defect is that
`docs/process.md`'s replication rule is left answered by a sentence the document itself
falsifies, which a reviewer of the results will read and cannot resolve.

**What would clear it.** Either limb round 2 named, and the cheaper one is available for free:
delete §0's *"Replication is the 5 reps the terms fix."* and have §0 point at §1.2, which now
owns the drift ground and states the sufficiency argument in measured terms (D-423's
state-once-and-point; D-424 reaches this sentence, since both readings license the same single
run). If instead a second landing-bench run is wanted, it needs its agreement criterion and its
registered consequence **before** either run — `docs/process.md` is explicit that a criterion
without a registered consequence leaves standing the after-the-numbers decision it exists to
forbid.

### MINOR

- **n10 — §0's new `tools/determinism.sh` row asserts a citation the document does not contain.**
  The row reads *"cited three times here (`:76`, `:85`, `:204`)"*. My own sweep over the whole
  pinned file (`/usr/bin/grep -nE 'determinism\.sh:[0-9]+'`) returns four citations at three
  distinct lines: `:76` at 41, `:76` at 102, `:76` at 282, `:204` at 299. **`:85` is cited
  nowhere outside the claim itself** — §2.1's budgets are given as `depth_turns 4` /
  `nodes 200000` with no line reference. `:85` is where round 2's *reviewer* checked the
  budgets; the row transcribed the reviewer's list rather than deriving it against this
  document. `:85` does resolve at `ffc5c10` (it is `BUDGETS=(…)`), so the defect is the count
  and the enumeration, not a dangling line. §0 states that this document is outside gate 20's
  citation check, so nothing mechanical will catch it.

- **n11 — the new harness paragraph attributes two verdict branches to one line.**
  *"`tools/bench_delta.sh:452` prints `VERDICT ABORT` below **1.15** and `BELOW-BRACKET` below
  1.4"*. `git show ffc5c10:tools/bench_delta.sh | sed -n '452,453p'` puts `VERDICT ABORT` at
  `:452` and `VERDICT BELOW-BRACKET` at `:453`. Every other `file:line` in this document is
  exact to the line, which is the convention a reader will assume.

- **n12 — "In `[1.10, 1.15)` the two readings differ" is true but not exclusive, and the
  document's own artifacts show it.** The harness prints `VERDICT BELOW-BRACKET` for every ratio
  in `[1.15, 1.4)` (`:453`), so it disagrees with this document across the whole of **both**
  registered brackets, not only below 1.15. Reproduced from the three referent artifacts the
  document cites: each records `band early: VERDICT BELOW-BRACKET — nps ratio 1.25x …` and
  `band late: VERDICT BELOW-BRACKET — nps ratio 1.2xx …` at the very ratios §1.1 grounds PASS
  on. The governing rule (*"THE HARNESS PRINTS ITS OWN VERDICT AND IT IS NOT THIS TABLE"*) is
  stated and correct, so nothing is mis-licensed; but a reader of the results document told the
  readings differ *in `[1.10, 1.15)`* will meet a `BELOW-BRACKET` line beside a PASS and think
  something went wrong.

- **n13 — the header's exclusivity claim is false on its face.** *"THE GATE'S HISTORY IS HERE
  AND NOWHERE ELSE."* My own sweep
  (`/usr/bin/grep -nE "round [0-9]|round-[0-9]|revision [0-9]"` over the pinned file, then
  `awk 'NR>17'`) returns **18 further lines** outside the header paragraph, several of which are
  full re-narrations rather than attributions: B3's history at `:129-130` (*"an earlier revision
  said of `[1.10, 1.15)` both that it lands and that it STOPs, and left `[1.15, 1.190)`
  unsaid"*), M5's at `:176-178`, M4's at `:268-270`, and revision 3's own failed first execution
  at `:226-230` and again at `:423-429`. The four paragraphs round 2 quoted are genuinely gone,
  so n9's substance improved; the new sentence claims more than the document delivers. Under
  D-424 the sentence is prose that constrains nothing and deletion is the cheaper fix than
  moving five more narrations.

- **n14 — the n8 remedy broke the slot pass D-427 mandates.** §0 registers *"Nothing else in
  this document is a slot."* Round 2 verified that with
  `/usr/bin/grep -o '<[a-z][a-z ]*>' <doc> | LC_ALL=C sort | uniq -c` → `9 <landed>` and nothing
  else. The same command on revision 3 returns `9 <landed>` **and `1 <path>`**, from the new §0
  note *"each revision is `git log -1 --format=%h ffc5c10 -- <path>`'s answer"*. The claim is
  still substantively true — a metavariable inside a backticked command is not a slot to fill at
  launch — but the exhaustiveness claim no longer survives the check a slot pass performs, and
  this project has an ADR (D-602) about a search returning the wrong population. One character
  fixes it.

### Noted, outside the remedies, NOT rated

- §0's *"What **§1.1's** dry run adds is ATTRIBUTION…"* — the dry run is **§1.2's**; §1.1 is the
  bracket. The sentence is unchanged from revision 2 and outside the diff.
- `docs/experiments/p1_impl_REVIEW.md:363` is still headed *"Check 6 — the five gates that read
  this state"*, which is where round 2 traced D-603's "five". D-603 is now correct; the impl
  review, a record, is not, and a record's citations were true when written.
- §0's completeness sentence (*"`bench_delta.sh` reads the LIVE tree's config and fixture for
  both sides … so the revisions above are the instrument for every run below"*) justifies itself
  from a property of `bench_delta.sh` alone, and now reaches six gate scripts that run from
  `<landed>`'s tree rather than from the live one. The registration is still the right one under
  `docs/process.md` — naming them at `ffc5c10` means a change to any of them reopens the review,
  which is the rule working — but the justifying clause does not carry the six new rows.

---

## 5. Environment

`/usr/bin/grep`, `git show`/`git log` pinned to `ffc5c10` and to the stash
`189c6d6e73e17deab45cc36ffb8aa929f4a8f7a7`, `LC_ALL=C sort`, and my own
`awk`/`python3`/`sha256sum`/`cmp` (CLAUDE.md Environment, D-265, D-602). Every document was read
with `git show <rev>:<path>`; the working tree's copy was used only where the document's own
registered command names a working-tree path (§2.1's extraction), and I ran that extraction a
second time pinned via `git show` to confirm the two agree — they do, byte for byte. One engine
workload was run: §2.1's block, from the live tree root, against the two pre-existing release
binaries, read-only. **No `cargo` was invoked**; nothing under `/home/tom/pistol-wt` was written
or built; no worktree was created. Two other agents' processes were live throughout and were
left alone. All scratch went to this session's scratchpad and to `mktemp -d`.
`git status --porcelain` is unchanged apart from this report, which is the only file written
into the repository.

**VERDICT: FAIL (B4; MAJOR M7)**
