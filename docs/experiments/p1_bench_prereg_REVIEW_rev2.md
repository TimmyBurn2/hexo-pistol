# REVIEW — `docs/experiments/p1_bench_prereg.md` (round 2 of this gate)

| | |
|---|---|
| document | `docs/experiments/p1_bench_prereg.md`, "revision 2" per its own title |
| revision reviewed | `1ee0ddd91b6e0cee095fd17b4b7848b1cd6281d7` (a `git stash create` commit on `dev`) |
| matches the working tree? | **yes** — `cmp -s <(git show 1ee0ddd9…:docs/experiments/p1_bench_prereg.md) docs/experiments/p1_bench_prereg.md` → identical bytes |
| HEAD at review | `ffc5c10f4d16356f574e3221a023f78399d2e3bb` (`dev`); the file is untracked at HEAD |
| date | 2026-09-04 |
| round | **round 2 of this document's gate.** Round 1's report is `p1_bench_prereg_REVIEW.md` (*"VERDICT: FAIL (B1, B2; MAJOR M1, M2, M3)"*, ten minors). The governed run has still not been taken |
| execution | §2.1's block was EXECUTED against the two pre-existing release binaries; the registered idle receipt was TESTED in four spellings; the §1.2 dry-run artifact was digested and read; the block actually executed for the §2.4 receipt was FOUND ON DISK and diffed against the printed one. No `cargo` was run anywhere (another agent's `cargo test --workspace --locked` was live in `/home/tom/pistol-wt/review-p1-impl` throughout — pid 1649186). Everything adjudicated used `/usr/bin/grep`, `git show`/`git log` pinned to `ffc5c10`, `LC_ALL=C sort`, or my own `awk`/`python3` (D-265) |

**VERDICT: FAIL (B3; MAJOR M4, M5, M6)**

Round 1's five headline findings are all genuinely remedied and I verified each by
execution or re-derivation, not by reading. Three of the fifteen remedies, however,
carried a new defect in with them, and one of those is blocking.

---

## 0. What holds, stated first, because the substance of B1 and B2 is now real

- **B1's dry run was actually taken and its artifact is what the document says it is.**
  `sha256sum artifacts/p1_dryrun_rev_mode_v1.txt` → `a2f20ad5a02b8327266de307d70356307cc187826c3aabd436c96ce3c1cde69c`,
  matching the claimed `a2f20ad5…`. The artifact records the registered command
  (`tools/bench_delta.sh rev:ffc5c10 rev:4298ecd 5`), both `rev:` resolutions, both
  `--release --locked` builds in throwaway worktrees, `node identity holds per position,
  both budgets, all reps`, `BENCH_EXIT=0`, and `idle` before and after. Its two ratios are
  the document's two ratios (early 1.258, late 1.231) — nothing transcribed wrong.
- **The strongest single fact in revision 2 is true.** The binaries `bench_delta.sh`
  built itself in `rev:` mode hash to `78a7600adcf099de…` and `f333d1010f520976…`, which
  are exactly the digests of `/home/tom/pistol-wt/p1-measure/pistol-ffc5c10` and
  `pistol-E` (`sha256sum` on both, mine) and exactly the two digests printed in
  `artifacts/p1_mx_bench_E_v1.txt:3-4`. The matrix's fourth attack — *"binaries tied to
  sources by this session's word"* (`matrix_P1_threat_state.md:593`) — is genuinely closed
  by the instrument. Every number in the matrix was taken on bytes the script has now
  independently reproduced from a named revision.
- **B2's two rows are correct and nothing else the run reads is still missing.** I read
  `tools/bench_delta.sh` for every path it opens: `CONFIG` (`:92`), `WEIGHTS` (`:93`),
  `FIXTURE` (`:94`) — those three and nothing else. I read all three identity seats for
  every file they name: each names exactly one, `weights_file =
  "configs/eval_v0_weights.toml"`. All eight file revisions in §0 re-derive correctly with
  my own `git log -1 --format=%h ffc5c10 -- <path>`.
- **§2.1's block, as printed in revision 2, works and produces the registered answer.**
  I ran it from the live tree root with the two supplied binaries: `BASE lines 533 bestmove
  128 error 0`, `CAND lines 533 bestmove 128 error 0`, both normalised transcripts
  `284f70afab0ac8f16343da78418b07de19572c1680e58961e8447717647dc3c6`, `RESULT: IDENTICAL`,
  exit 0. `cmp` says my `out.BASE` is byte-identical to the one the §2.4 v2 dry run left in
  `/tmp/tmp.qYeSMVisVB`. **m3 is closed by execution**: `git status --porcelain` before and
  after are identical and no `out.BASE`/`out.CAND` exists in the repository root.
- **M2's new receipt spelling does what the document says.** Tested three ways below; it
  can print `idle`, it does not match its own invoking shell, and it detected both a live
  `cargo` and a live `target/release/pistol` belonging to another agent.
- **M1's arithmetic is now exact.** `1.257 ± 0.05` = `[1.207, 1.307]`; `1.240 ± 0.05` =
  `[1.190, 1.290]`; both are what §1.1 registers, unrounded.
- **M3 is fixed and the six gate numbers are right.** My own enumeration (numbering `step`
  invocations in file order at `ffc5c10`) gives 8 tactical fixture, 9 cross-process
  determinism, 10 differential search oracle, 11 staged generator soundness, 12 solver
  oracle, 13 solver determinism, and `readonly GATE_TOTAL=20`.

---

## 1. Remedy verification (round 1's fifteen)

| finding | remedy in revision 2 | how I executed / re-derived it | holds? |
|---|---|---|---|
| **B1** dry run never taken | §1.2 records the run, a result table, and `artifacts/p1_dryrun_rev_mode_v1.txt` `a2f20ad5…` | `sha256sum` the artifact; read it whole; compared every number in §1.2's table against the artifact's own lines; compared the referent against `p1_mx_bench_E_v1.txt`'s `nps ratio` lines; `sha256sum` of the two hand-built binaries against the two the script built | **yes** — digest matches, 1.258/1.231 are the artifact's, differences 0.001/0.009 are within ±0.03, node identity held, exit 0, both receipts `idle`, self-built digests equal the hand-built ones |
| **B2** `eval_v0_weights.toml` / `ci.sh` missing | two rows added | `git log -1 --format=%h ffc5c10 -- configs/eval_v0_weights.toml` → `64c336f`; `… -- tools/ci.sh` → `9390b48`; `/usr/bin/grep -nE '…' tools/bench_delta.sh` for every path it opens; `/usr/bin/grep -nE '(file\|path\|\.toml\|\.txt)'` on all three seat configs | **yes** for the run's own inputs (see n8 for the gate scripts §3 runs and for `tools/determinism.sh`, still unnamed) |
| **M1** brackets not their ground | per-band `[1.207, 1.307]` / `[1.190, 1.290]` | `python3 -c "print(round(1.257-0.05,3), round(1.257+0.05,3), round(1.240-0.05,3), round(1.240+0.05,3))"` | **yes** — and the abort at `< 1.10` is still reachable below both lower bounds |
| **M2** receipt cannot print `idle`, self-matches | one bracketed spelling in §1.3 and §2.1 | `sed -n '207p;258p;297p' <doc> \| LC_ALL=C sort -u` → one line; ran the shape from a file (prints `idle`, exit 0); ran the registered pattern tagging `$$`/`$PPID` against every hit (no SELF, no PARENT); planted `exec -a ".../target/release/pistol --config x" sleep 5` and the receipt reported it | **yes** — all three spellings byte-identical, `idle` reachable, no self-match, detection works (see n4, n5 for residuals) |
| **M3** "five CI gates", §3 names six | §0 now says six | `awk '/^## 3\./,/^## 4\./' <doc> \| /usr/bin/grep -o '(gate [0-9]*)' \| LC_ALL=C sort -u` → 6; `/usr/bin/grep -n -iE '(five\|six) (CI )?gates' <doc>` → only "six" | **yes** in the document — **but D-603 still says "the five gates that read this state" (M6)** |
| m1 §2.4 no registered consequence | *"Registered consequence of a miss: the identity leg is not run at the landing…"* | read at §2.4 | yes |
| m2 falsifier's third outcome | *"ANYTHING ELSE IS A DEFECTIVE LEG — `RESULT: IDENTICAL`, or neither line"* | read at §2.3 | yes |
| m3 block writes into the tree | `OUT="$(mktemp -d)"`, all four paths rewritten | **executed the block**; `diff` of `git status --porcelain` before/after is empty; `ls out.BASE out.CAND` → no such file | **yes, by execution** |
| m4 "the committed solver-on seat" | names `configs/bench_wp18c_solver_on.toml`, "one of the three" | `git grep -l 'on_search_path = true' ffc5c10 -- configs/` → three (`bench_wp18c_solver_on`, `gate_staged_solver_v0`, `play_staged_solver_v0`); `matrix_P1_threat_state.md:573` names `bench_wp18c_solver_on.toml` | yes |
| m5 artifacts unnamed | `p1_dryrun_rev_mode_v1.txt` (+digest), `p1_identity_landing_v1.txt`, `p1_landing_bench_v1.txt` | read at §1.2, §2.2 | yes (see n2: §1.3 points at the wrong section for one of them) |
| m6 `ci.sh` row / citation gate | row added; paragraph says the document is not on gate 20's list | `/usr/bin/grep -n '\.md' tools/governing_citation_check.sh` — the list is `CLAUDE.md`, `docs/ROADMAP.md`, `docs/process.md`, `anchor_v3_openings_design.md`, `matrix_label_cache_key.md`, `sealbot_anchor_v3_prereg.md`, `wp21_label_cache_design.md`, `wp21_prereg.md`, `wp21_throughput_prereg.md` — no `p1_bench_prereg.md` | yes |
| m7 "both fixture sets" | ONE FIXTURE SET paragraph | read; `git show ffc5c10:tools/bench_delta.sh \| /usr/bin/grep -n '^FIXTURE=\|^EARLY_MAX='` | **substantively yes, citation no** — `FIXTURE=` is at `:94`, not `:91` (n1) |
| m8 unbounded identity claim | bounded to *"the 128 searches this section registers, at the three seats it names"* | read at §2 | yes |
| m9 abort overlaps the harness's 1.15 | new paragraph naming `[1.10, 1.15)` | read; `python3` on the retained-gain fractions | **no — the remedy introduced B3** |
| m10 ±0.03 has no ground | ground stated as the pair's 0.001 drift | compared against the document's own new dry run and both referent artifacts | **no — the stated ground is falsified by the run added in the same section (M5)** |

---

## 2. New findings

### BLOCKING

#### B3 — §1.1's abort paragraph now registers two opposite dispositions for `[1.10, 1.15)`, and the arithmetic in the sentence that creates the conflict is wrong

The m9 remedy was inserted **between** the abort's trigger and the abort's consequence.
As printed (reviewed revision, §1.1):

> **Abort:** an nps ratio **< 1.10** in either band. **THE INSTRUMENT'S OWN ABORT IS 1.15
> …** `tools/bench_delta.sh:452` prints `VERDICT ABORT` below 1.15, so a landing in
> **[1.10, 1.15)** is a below-bracket FINDING by this document and an ABORT by the
> harness's printed line. … a landing in that interval is reported with both readings **and
> lands**, because a change measured at a fifth of its prototype's gain is a finding about
> the landing and not a wrong answer. **The landing would then have lost more than half of
> what its own prototype measured**, which says something other than the design landed.
> **Consequence: the landing does NOT go to `dev`, the package STOPs** … and the finding is
> the number.

"The landing would **then** have…" now takes the interval just discussed, `[1.10, 1.15)`,
as its antecedent, and attaches to it the consequence *does NOT go to `dev`, the package
STOPs* — four words after the same interval was told it **lands**. A landing at 1.13 is
licensed by this paragraph both to land and to STOP the package, and D-374 forbids
choosing between them after the number is seen. That is precisely what a
pre-registration exists to prevent.

**Minimal reproducer.**

```
$ sed -n '/^\*\*Abort:\*\*/,/finding is the number\./p' docs/experiments/p1_bench_prereg.md \
    | /usr/bin/grep -o 'and lands\|does NOT go to `dev`\|package STOPs'
and lands
does NOT go to `dev`
package STOPs
```

Three dispositions, one paragraph, one interval between them.

**And the sentence that creates the conflict is arithmetically false.** *"a change measured
at a fifth of its prototype's gain"*:

```
$ python3 -c "
for r in (1.10,1.15):
  for p,b in ((1.257,'early'),(1.240,'late')):
    print(b, r, 'retains %.1f%% of the prototype gain' % ((r-1)/(p-1)*100))
print('a fifth would be', round(1+0.257/5,4), '/', round(1+0.240/5,4))"
early 1.1 retains 38.9% of the prototype gain
late 1.1 retains 41.7% of the prototype gain
early 1.15 retains 58.4% of the prototype gain
late 1.15 retains 62.5% of the prototype gain
a fifth would be 1.0514 / 1.048
```

The interval `[1.10, 1.15)` retains **two to three fifths** of the prototype's gain, not a
fifth. A landing at "a fifth" would be ≈1.05 — below the abort line, where the document's
own consequence is STOP. So the reason given for letting the interval land is a number that
would not let it land.

**A third incoherence falls out of the same edit.** `[1.10, 1.15)` is now the only
below-bracket interval with an explicit *"lands"*. The strictly better interval
`[1.15, 1.190)` (late band) or `[1.15, 1.207)` (early) is covered only by *"Below bracket
but above abort is a FINDING, reported against the bracket"*, which states no disposition
at all. The document explicitly licenses landing at a worse measured ratio than one whose
disposition it leaves open.

**What would clear it.** Give `< 1.10` its consequence in its own sentence, adjacent to its
trigger; give `[1.10, bracket-low)` one disposition, once; and either drop the "a fifth"
clause or replace it with the fraction the arithmetic gives.

### MAJOR

#### M4 — §2.4's `block sha256 60b8972d…` is not the digest of the block this document prints, and the file that was actually executed is on disk with three extra lines

§2.4 states, of revision 2's rewritten block:

> **the changed block was re-run before this revision was dispatched**:
> `artifacts/p1_identity_dryrun_E_v2.txt`, block sha256 `60b8972d…`

The block the document prints does not hash to that.

```
$ awk '/^```sh$/{n++; if(n==2){f=1; next}} f && /^```$/{f=0} f' \
    docs/experiments/p1_bench_prereg.md | sha256sum
e343608a3d9e7402ae9080784477c4b0eb4fdd3f86fc3b2db2d3122ef55d03b5  -

$ awk '/^```sh$/{f=1;next} /^```$/{f=0} f' docs/experiments/p1_bench_prereg.md | sha256sum
13804e8c3ae82a5a9b7d53fc4b9b46d41881f5515b263e7d9b4b68b0b3b7063d  -
```

The second command is the extraction round 1 used and whose output matched revision 1's
recorded `12748c31…` exactly; neither it nor the natural "second fenced block" extraction
produces `60b8972d…`. I then found the file that does:

```
$ sha256sum /home/tom/pistol-wt/p1-patches/block_rev2.sh
60b8972d2dfbdb2d013ef162be921c7a7f6c40a300782de519d290914bff55e8  …/block_rev2.sh

$ diff /home/tom/pistol-wt/p1-patches/block_rev2.sh <extracted §2.1 block>
1,3d0
< { pgrep -af '[c]argo|[r]ustc|[b]ench_delta|target/[r]elease/pistol' || echo idle; }
< ```
< ```sh
```

The executed file is the two fenced blocks of revision 2 **concatenated with the fence
markers left in**: §1.3's standalone receipt line, then a literal ` ``` ` line and a
literal ` ```sh ` line, then the real block. Bash ran the two fence lines as commands
(`command not found`, before `set -euo pipefail` on line 4, so they were not fatal). That
is also why `artifacts/p1_identity_dryrun_E_v2.txt` carries **three** `idle` lines where
§2.4 speaks of *"both idle receipts"*.

**Why it is MAJOR and not merely cosmetic.** §0 registers *"the identity leg's command
block, §2 | this document's own revision"* as an instrument under `docs/process.md`'s
instrument-revision rule, and the §2.4 digest is the only thing that ties the instrument
that ran to the instrument that is registered. As recorded, it ties it to a different file.
Worse, the defect is forward-looking: now that the document prints **two** ` ```sh ` blocks,
the extraction convention its own previous receipt used yields a script with markdown fences
in it, and the landing session is the next party to extract this block. The
`R3-M1` history (a previously executed spelling that differed from the printed one) is
exactly this shape.

**What saves the substance.** I executed the block **as printed** and it produced the
recorded answer byte for byte — `BASE/CAND lines 533 bestmove 128 error 0`, transcripts
`284f70af…`, `RESULT: IDENTICAL`, exit 0, and `cmp` says my `out.BASE` equals the v2 dry
run's. So §2.4's *conclusion* ("the rewrite changed where the output is written and not what
is asked") is true; its *receipt* does not establish it.

**What would clear it.** Re-run the printed block, record its own digest, and register that
digest in §0's instrument row so the block has one unambiguous identity rather than an
extraction convention.

#### M5 — the ground registered for ±0.03 is falsified by the run recorded three paragraphs below it, and "Replication is the 5 reps" does not reach the drift that matters

§1.2 registers:

> the observed run-to-run drift on this exact pair is **0.001** early and **0.001** late
> (1.257 against 1.256, 1.240 against 1.241, the two artifacts above), so ±0.03 is **thirty
> times the drift the pair has shown**

The document then adds a third measurement of the same pair, and it is 0.009 off the
referent in the late band. Because §1.2 also proves the two binaries are **byte-identical**
to the hand-built pair, there is no build difference left to attribute the gap to: it is
run-to-run drift, by the document's own strongest evidence.

**Minimal reproducer** (three runs, one binary pair `78a7600a…` / `f333d101…`, one config,
one fixture, one script):

```
$ /usr/bin/grep -H 'band \(early\|late\): nps ratio' \
    artifacts/p1_mx_bench_E_v1.txt artifacts/p1_rt_round1/bench_E.txt \
    artifacts/p1_dryrun_rev_mode_v1.txt | sed 's/, time.*//'
p1_mx_bench_E_v1.txt:band early: nps ratio 1.257
p1_mx_bench_E_v1.txt:band late: nps ratio 1.240
p1_rt_round1/bench_E.txt:band early: nps ratio 1.256
p1_rt_round1/bench_E.txt:band late: nps ratio 1.241
p1_dryrun_rev_mode_v1.txt:band early: nps ratio 1.258
p1_dryrun_rev_mode_v1.txt:band late: nps ratio 1.231

$ python3 -c "print('early spread', round(1.258-1.256,3), '| late spread', round(1.241-1.231,3))"
early spread 0.002 | late spread 0.01
```

So: ±0.03 is **three** times the late-band drift the pair has shown, not thirty; and the
sentence "the observed run-to-run drift on this exact pair is 0.001 late" is contradicted by
this document's own table, which prints `late | 1.231 | 1.240 | 0.009`, in the same
subsection. D-423's "a claim the document makes twice" has fired inside one section.

**The second limb.** §0 answers `docs/process.md`'s cheap-run replication rule with
*"Replication is the 5 reps the terms fix."* The 5 reps are **inside one invocation**; their
spread is what the script's IQR gate polices (10 % of median). They cannot see between-run
drift, which is the only drift ±0.03 and ±0.05 exist to absorb, and which this pair has now
shown at 0.010. By §0's own cost paragraph the run is about a minute, so a second landing
bench is nearly free. Either register a second landing-bench run with an agreement criterion
and its registered consequence, or state that a half-width of 0.05 against a demonstrated
0.010 makes a single sample sufficient — but the present sentence answers the rule with a
replication that is not one.

Note the outcome is unaffected: 0.009 < 0.03, the §1.2 criterion holds, and 1.231 sits
inside `[1.190, 1.290]`. The finding is against the registered ground, not the number.

#### M6 — D-603, staged in the working tree to land with this package, cites a bracket revision 2 withdrew and a gate count revision 2 corrected

`docs/decisions.md` is modified in the working tree; the added line is `D-603`, whose slot
paragraph reads:

> the landing bench `tools/bench_delta.sh rev:ffc5c10 rev:<landed> 5` → `<nps early> /
> `<nps late>` against **the bracket [1.20, 1.30] registered in `p1_bench_prereg.md` before
> any run**

and whose verification paragraph reads *"**the five gates** that read this state green at
`3ef6706`"*.

```
$ git diff --stat docs/decisions.md
 docs/decisions.md | 2 ++
$ tail -1 docs/decisions.md | /usr/bin/grep -o 'bracket \[[0-9., ]*\]\|the five gates that read this state'
bracket [1.20, 1.30]
the five gates that read this state
$ /usr/bin/grep -n '\[1\.207, 1\.307\]\|\[1\.190, 1\.290\]' docs/experiments/p1_bench_prereg.md | cut -c1-40
97:| early | **[1.207, 1.307]**
98:| late | **[1.190, 1.290]**
```

`[1.20, 1.30]` is exactly the pairing revision 2 deletes, in its own words, as *"neither
ground's arithmetic"* — the ADR now asserts as fact a registration the prereg has withdrawn,
and it is the ADR line that will report the landing. The gate count is round 1's M3 fixed on
only one side of the pair: round 1 traced the prereg's "five" **to this ADR line**, the
prereg is now six, and the ADR still says five.

**Consistency of the slots, otherwise.** The prereg's own slot claim holds exactly:
`/usr/bin/grep -o '<[a-z][a-z ]*>' docs/experiments/p1_bench_prereg.md | LC_ALL=C sort |
uniq -c` → `9 <landed>` and nothing else, matching §0's *"Nothing else in this document is a
slot."* D-603's three slots line up with the prereg's three registered outputs — nps pair
(§1), `<RESULT>` over 128 searches at three seats (§2), twenty gates at `<landed>` (§3) —
in every respect except the two above.

### MINOR

- **n1 — `bench_delta.sh:91` does not resolve.** §0's ONE FIXTURE SET paragraph cites
  `(FIXTURE=…/bench_positions_v1.txt, `:91`)`.
  `git show ffc5c10:tools/bench_delta.sh | /usr/bin/grep -n '^FIXTURE='` → **`94`**. (`:93`
  for `WEIGHTS`, `:272` for its digest, `:452` for `VERDICT ABORT`, `determinism.sh:76`,
  `:85`, `:204` all resolve correctly.) The document itself notes it is outside gate 20's
  coverage, which is why this had to be checked by hand.
- **n2 — §1.3 points at the wrong section for the landing bench's artifact.** *"The receipt
  is printed into the run's own artifact, named per run in §1.2 and §1.1's results
  document."* §1.1 names no artifact; `artifacts/p1_landing_bench_v1.txt` is named in
  **§2.2**, tucked under the identity leg's criterion. `awk '/^### 1\.1/,/^### 1\.2/' <doc>
  | /usr/bin/grep -c 'artifacts/p1_landing'` → 0.
- **n3 — §0's cost paragraph still prices the §1.2 dry run as unspent.** *"The landing
  bench, **its dry run**, the identity leg and its falsifier together are under half an hour
  of machine time."* Round 1 quoted that sentence as evidence B1's run lay in the future; it
  is unchanged, and the run is now taken.
- **n4 — the receipt prints `idle` on any `pgrep` failure, not only on an idle box.**
  `{ pgrep … || echo idle; }` cannot distinguish "no match" (exit 1) from a broken or absent
  `pgrep`. Reproduced: `bash -c "{ pgrep -af '[' || echo idle; }"` prints
  `pgrep: regex error: Invalid regular expression` on stderr and **`idle`** on stdout, outer
  exit 0. The registered pattern is valid, so the live hazard is a missing `pgrep` (exit
  127) on another box — but the receipt's output alphabet says "idle" for both, which is the
  `SHELL_CHECKLIST` exit-0-wrong-answer shape.
- **n5 — the receipt matches command lines, so VOID still needs a judgement, and §2.1's copy
  has no registered consequence at all.** My run of the registered pattern returned
  `1649184 /usr/bin/bash -c … cargo test --workspace --locked …` — a *shell* whose command
  line names `cargo`, not a build. And §1.3 scopes VOID to *"every timing run below — the
  dry run and the landing bench"*, while §2.1's block prints the same receipt twice and §2.4
  cites its `idle` as part of what the dry run showed. Demonstrated: my execution of §2.1
  printed two **non-idle** receipts (that live `cargo test`, plus
  `/home/tom/pistol-wt/review-p1-impl-mut/target/release/pistol --config
  configs/gate_staged_heuristics_v0.toml`) and the registered §2.4 criterion still held in
  full. One clause — that the identity leg is not a timing run and its receipt is context,
  not a gate — closes it. (Revision 1's `artifacts/p1_identity_dryrun_E_v1.txt` needed a
  hand-written `# NOTE:` for exactly this.)
- **n6 — a paragraph break lands mid-sentence in §1.2.** Lines 156-159 of the reviewed
  revision read `…and the criterion is that` / blank / blank / `the dry run's nps ratio
  lands within **±0.03…**`, so the registered criterion renders as two paragraphs split
  across its own subject and predicate. `sed -n '156,159p' <doc>`.
- **n7 — `artifacts/p1_identity_dryrun_E_v2.txt` records three `idle` lines where §2.4 says
  "both idle receipts".** A direct consequence of M4's extraction; listed separately because
  a reader comparing the artifact to the prose will see the mismatch without knowing why.
- **n8 — §0's completeness sentence still does not cover what §3 runs.** *"the revisions
  above are the instrument for every run below"* is affirmative and reaches §2 and §3. §3's
  six gates are executed by `tools/tactical_check.sh`, `tools/determinism.sh`,
  `tools/search_oracle_check.sh` and the solver checks, none of which is named with a
  revision — only their caller `tools/ci.sh` is. `tools/determinism.sh` in particular is
  cited by `file:line` three times in this document (`:76`, `:204`, and the budgets at
  `:85`) and is unnamed in §0; `git log -1 --format=%h ffc5c10 -- tools/determinism.sh` →
  `7fbc1ff`. This is the weaker half of round 1's m6 left standing, not a new defect.
- **n9 — revision 2 narrates its own history in six places.** The header paragraph restates
  B1, B2, M1, M2 and M3, and each is restated again at its section (*"TAKEN, AND THIS IS
  WHAT ROUND 1's B1 FOUND MISSING"*, *"EACH BAND'S BRACKET IS ITS OWN GROUND…"*, *"The
  bracket spelling is not decoration"*, *"THE INSTRUMENT'S OWN ABORT IS 1.15…"*). D-423's
  rule is state-once-and-point; the one place where the two statements diverge is the abort
  paragraph, which is B3.

---

## 3. Execution record — §2.1's block

Extracted with `awk '/^```sh$/{n++; if(n==2){f=1; next}} f && /^```$/{f=0} f'` from the
reviewed revision, run from `/home/tom/Projects/HeXO-AlphaBeta` with
`BASE=/home/tom/pistol-wt/p1-measure/pistol-ffc5c10` and
`CAND=/home/tom/pistol-wt/p1-measure/pistol-E`. No `cargo`, no build, nothing written under
`/home/tom/pistol-wt`.

| check | result |
|---|---|
| block sha256 (mine) | `e343608a3d9e7402ae9080784477c4b0eb4fdd3f86fc3b2db2d3122ef55d03b5` — **≠** the `60b8972d…` §2.4 records (M4) |
| `bestmove` / `error` per side | `BASE lines 533 bestmove 128 error 0`; `CAND lines 533 bestmove 128 error 0` |
| binary digests | `78a7600adcf099de…` ≠ `f333d1010f520976…` |
| transcript digest | `284f70afab0ac8f16343da78418b07de19572c1680e58961e8447717647dc3c6` on **both** sides — the `284f70af…` §2.4 claims |
| result / exit | `RESULT: IDENTICAL`, `BLOCK_EXIT=0` |
| writes into the tree | **none** — `git status --porcelain` before and after are identical; `ls out.BASE out.CAND` → no such file; transcripts landed in `/tmp/tmp.AzKxKozluT` |
| idle receipts | both printed, and both **non-idle**: `cargo test --workspace --locked` (pid 1649186, another agent's mutation suite) and, on the closing receipt, `/home/tom/pistol-wt/review-p1-impl-mut/target/release/pistol --config configs/gate_staged_heuristics_v0.toml`. The registered §2.4 criterion held anyway — see n5 |
| cross-check | `cmp /tmp/tmp.AzKxKozluT/out.BASE /tmp/tmp.qYeSMVisVB/out.BASE` → identical to the transcript the v2 dry run left behind |

---

## 4. Re-derivation table

Every row below was produced with a command I chose, printed with its scope. No row was
produced by running a command the document supplies.

| claim | my command (scope) | my number | document's | agree? |
|---|---|---|---|---|
| `tools/bench_delta.sh` revision | `git log -1 --format=%h ffc5c10 -- tools/bench_delta.sh` | `ab369b0` | `ab369b0` | ✔ |
| …WP-1.9b names the same | `/usr/bin/grep -n 'ab369b0' docs/experiments/wp19b_bench_prereg.md` (whole file) | `:15` | "names the same revision" | ✔ |
| `configs/instrument_v0.toml` | `git log -1 --format=%h ffc5c10 -- configs/instrument_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `bench_positions_v1.txt` | `git log -1 --format=%h ffc5c10 -- crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` | `70cc465` | `70cc465` | ✔ |
| **`configs/eval_v0_weights.toml`** | `git log -1 --format=%h ffc5c10 -- configs/eval_v0_weights.toml` | `64c336f` | `64c336f` | ✔ (B2 closed) |
| **`tools/ci.sh`** | `git log -1 --format=%h ffc5c10 -- tools/ci.sh` | `9390b48` | `9390b48` | ✔ (B2 closed) |
| `configs/tactical_staged_v0.toml` | `git log -1 --format=%h ffc5c10 -- configs/tactical_staged_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `configs/gate_staged_solver_v0.toml` | `git log -1 --format=%h ffc5c10 -- configs/gate_staged_solver_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `tactical_staged_v0.txt` | `git log -1 --format=%h ffc5c10 -- crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt` | `538b3e5` | `538b3e5` | ✔ |
| every file `bench_delta.sh` opens | `/usr/bin/grep -nE '^(CONFIG\|WEIGHTS\|FIXTURE)=' tools/bench_delta.sh` + read of the whole script | exactly those three | §0 names all three | ✔ |
| every file the three seats name | `/usr/bin/grep -nE '(file\|path\|\.toml\|\.txt)' configs/{instrument_v0,tactical_staged_v0,gate_staged_solver_v0}.toml` | one each: `weights_file = configs/eval_v0_weights.toml` | §0 names it | ✔ |
| `tools/determinism.sh` revision | `git log -1 --format=%h ffc5c10 -- tools/determinism.sh` | `7fbc1ff` | **absent from §0** | ✘ n8 |
| `FIXTURE=` line number | `git show ffc5c10:tools/bench_delta.sh \| /usr/bin/grep -n '^FIXTURE='` | **94** | **`:91`** | **✘ n1** |
| `WEIGHTS=` line number | same, `'^WEIGHTS='` | 93 | `:93` | ✔ |
| weights digest line | `git show ffc5c10:tools/bench_delta.sh \| sed -n '272p'` | `digest "$WEIGHTS" …` | `:272` | ✔ |
| harness ABORT threshold line | `… sed -n '452p'` | `if (nps < 1.15) … VERDICT ABORT` | `:452`, 1.15 | ✔ |
| third seat's config + budgets | `git show ffc5c10:tools/determinism.sh \| sed -n '76p'` | `staged-solver configs/gate_staged_solver_v0.toml …/tactical_staged_v0.txt depth_turns-2 nodes-10000` | matches | ✔ |
| WP-1.9 seats' budgets | `… sed -n '85p'` | `BUDGETS=("depth_turns 4" "nodes 200000")` | matches | ✔ |
| `:204` is the position extraction, count cross-checked | `git show ffc5c10:tools/determinism.sh \| sed -n '198,215p'` | `mapfile … sed -n 's/^position //p'` then `[ "${#positions[@]}" -eq "$cases" ]` | ":204 reads it the same way and cross-checks the count against the cases" | ✔ |
| gates 8–13 | `git show ffc5c10:tools/ci.sh \| awk '/^step /{n++; print n": "$0}' \| sed -n '8,13p'` | 8 tactical fixture, 9 cross-process determinism, 10 differential search oracle, 11 staged generator soundness, 12 solver oracle, 13 solver determinism | identical | ✔ |
| twenty gates | `git show ffc5c10:tools/ci.sh \| /usr/bin/grep -n 'GATE_TOTAL='` | `readonly GATE_TOTAL=20` | "twenty gates" | ✔ |
| number of gates named in §3 | `awk '/^## 3\./,/^## 4\./' <doc> \| /usr/bin/grep -o '(gate [0-9]*)' \| LC_ALL=C sort -u \| wc -l` | 6 | "six" | ✔ (M3 closed) |
| three committed solver-armed configs | `git grep -l 'on_search_path = true' ffc5c10 -- configs/` | `bench_wp18c_solver_on`, `gate_staged_solver_v0`, `play_staged_solver_v0` | "one of the three" | ✔ |
| matrix attack 1's config | `/usr/bin/grep -n 'bench_wp18c_solver_on' docs/experiments/matrix_P1_threat_state.md` | `:565`, `:573` | named in §1's Seat bullet | ✔ |
| matrix attack 4's wording | `sed -n '593p' docs/experiments/matrix_P1_threat_state.md` | *"binaries this session built by hand"* | quoted as attack 4 | ✔ |
| `p1_dryrun_rev_mode_v1.txt` digest | `sha256sum artifacts/p1_dryrun_rev_mode_v1.txt` | `a2f20ad5a02b8327…` | `a2f20ad5…` | ✔ |
| …its two ratios | `/usr/bin/grep 'nps ratio' artifacts/p1_dryrun_rev_mode_v1.txt` | early 1.258, late 1.231 | 1.258 / 1.231 | ✔ |
| …node identity + exit | `/usr/bin/grep 'node identity\|BENCH_EXIT' …` | `holds per position, both budgets, all reps`; `BENCH_EXIT=0` | "held … exit 0" | ✔ |
| …it built both sides itself | `/usr/bin/grep 'revision \|build:' …` | `rev:ffc5c10 -> ffc5c10f4d…`, `rev:4298ecd -> 4298ecd332…`, two `cargo build --release --locked` lines | as claimed | ✔ |
| …self-built digests = hand-built | `sha256sum /home/tom/pistol-wt/p1-measure/pistol-ffc5c10 …/pistol-E` vs the artifact's two `->` lines | `78a7600adcf099de…`, `f333d1010f520976…` both sides | "the same two digests" | ✔ |
| …and the matrix was taken on them | `sed -n '3,4p' artifacts/p1_mx_bench_E_v1.txt` | same two digests | "every number in the matrix" | ✔ |
| referent artifact digests | `sha256sum artifacts/p1_mx_bench_E_v1.txt artifacts/p1_rt_round1/bench_E.txt` | `d0b1b38480b740ce…`, `e3eaed80794c81eb…` | `d0b1b384…`, `e3eaed80…` | ✔ |
| …their ratios | `/usr/bin/grep -H 'nps ratio' <both>` | 1.257/1.240 and 1.256/1.241 | same | ✔ |
| early bracket from its ground | `python3 -c "print(round(1.257-0.05,3), round(1.257+0.05,3))"` | 1.207, 1.307 | `[1.207, 1.307]` | ✔ (M1 closed) |
| late bracket from its ground | `python3 -c "print(round(1.240-0.05,3), round(1.240+0.05,3))"` | 1.19, 1.29 | `[1.190, 1.290]` | ✔ (M1 closed) |
| **drift of this exact pair** | the three artifacts' `nps ratio` lines, `python3` on the extremes | early **0.002**, late **0.010** | **"0.001 early and 0.001 late"**, "thirty times" | **✘ M5** |
| retained gain at the abort/harness interval | `python3 -c "print((1.10-1)/(1.257-1), (1.15-1)/(1.257-1), (1.10-1)/(1.240-1), (1.15-1)/(1.240-1))"` | 0.389 / 0.584 / 0.417 / 0.625 | **"a fifth"** | **✘ B3** |
| WP-1.9's ±0.05 precedent | `/usr/bin/grep -n '0.002\|±0.05' docs/experiments/wp19b_bench_prereg.md` | `:123` *"the three WP-1.9 runs composed to within 0.002, so ±0.05 is the drift allowance and not a fitted margin"* | quoted accurately | ✔ |
| `bench_positions_v1.txt` = 24 entries | `git show ffc5c10:…/bench_positions_v1.txt \| awk 'NF && $0 !~ /^#/' \| wc -l` | 24 | 24 | ✔ |
| `tactical_staged_v0.txt` = 20 cases, 20 `position` | `git show ffc5c10:…/tactical_staged_v0.txt \| awk '/^case /{c++} /^position /{p++} END{print c,p}'` | 20 20 | "20 cases, keyed" | ✔ |
| 128 searches a side | `python3 -c "print((20+24+20)*2)"` | 128 | 128 | ✔ |
| …and the block really sends 128 | my execution; `wc -l`, `/usr/bin/grep -c` on my own transcripts | 533 / 128 / 0 both sides | 128 / 0 | ✔ |
| §2.4's transcript digest | `sha256sum` on my own `out.BASE`, `out.CAND` | `284f70afab0ac8f1…` both | `284f70af…` | ✔ |
| **§2.4's block digest** | `awk` extraction (two spellings) + `sha256sum`; then `sha256sum` on the file that does match, found by scanning `/tmp` and `/home/tom/pistol-wt` | `e343608a…` / `13804e8c…`; the `60b8972d…` file is `/home/tom/pistol-wt/p1-patches/block_rev2.sh`, 3 lines longer | **`60b8972d…`** | **✘ M4** |
| citation-gate list | `/usr/bin/grep -n '\.md' tools/governing_citation_check.sh` | nine documents, not this one | "not among them" | ✔ |
| slots in the prereg | `/usr/bin/grep -o '<[a-z][a-z ]*>' <doc> \| LC_ALL=C sort \| uniq -c` | `9 <landed>`, nothing else | "Nothing else in this document is a slot" | ✔ |
| **D-603's bracket** | `tail -1 docs/decisions.md \| /usr/bin/grep -o 'bracket \[[0-9., ]*\]'` | `[1.20, 1.30]` | prereg registers `[1.207,1.307]`/`[1.190,1.290]` | **✘ M6** |
| **D-603's gate count** | `tail -1 docs/decisions.md \| /usr/bin/grep -o 'the five gates that read this state'` | "five" | prereg §3: six | **✘ M6** |
| D-603's other two slots | read of the tail line | 128 searches at three seats → `<RESULT>`; twenty gates at `<landed>` | §2, §3 | ✔ |
| the receipt can print `idle` | `bash <file>` holding the bracketed shape with non-matching alternatives | `idle`, exit 0 | "can print idle" | ✔ |
| the receipt does not self-match | `bash -c 'me=$$; par=$PPID; … pgrep … \| awk` tagging `$1==me` / `$1==par'` | no SELF, no PARENT among four hits | "the receipt is the machine's state and not the receipt's" | ✔ |
| the receipt detects an engine | planted `exec -a "/home/tom/pistol-wt/fake/target/release/pistol --config x" sleep 5` | reported | implied | ✔ |
| the receipt on `pgrep` failure | `bash -c "{ pgrep -af '[' \|\| echo idle; }"` | prints `idle`, outer exit 0 | (not claimed) | ✘ n4 |
| §1.3 / §2.1 spellings identical | `sed -n '207p;258p;297p' <doc> \| LC_ALL=C sort -u \| cat -A` | one line | "the one spelling" | ✔ |
| §1.2's paragraph break | `sed -n '156,159p' <doc>` | two blank lines mid-sentence | (not claimed) | ✘ n6 |

---

## 5. Environment

`/usr/bin/grep`, `git show`/`git log`/`git grep` pinned to `ffc5c10`, `LC_ALL=C sort`, and
my own `awk`/`python3`/`sha256sum` (CLAUDE.md Environment, D-265). One engine workload was
run — §2.1's block, from the live tree, against the two pre-existing release binaries,
read-only. No `cargo` was invoked (another agent's `cargo test --workspace --locked` was live
in `/home/tom/pistol-wt/review-p1-impl` and was left alone); nothing under
`/home/tom/pistol-wt` was written or built; no worktree was created. All scratch went to this
session's scratchpad and to `mktemp -d`. `git status --porcelain` is unchanged apart from
this report, which is the only file written into the repository.

**VERDICT: FAIL (B3; MAJOR M4, M5, M6)**
