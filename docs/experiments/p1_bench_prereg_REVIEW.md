# REVIEW — `docs/experiments/p1_bench_prereg.md` (round 1 of this gate)

| | |
|---|---|
| document | `docs/experiments/p1_bench_prereg.md`, "revision 1" per its own title |
| revision reviewed | `988c572b530aa86c1e3ee7221ab6c71ec0698de1` (a `git stash create` commit on `dev`) |
| matches the working tree? | **yes** — `cmp -s <(git show 988c572b…:docs/experiments/p1_bench_prereg.md) docs/experiments/p1_bench_prereg.md` → identical bytes |
| HEAD at review | `ffc5c10f4d16356f574e3221a023f78399d2e3bb` (`dev`); the file is untracked at HEAD |
| date | 2026-09-04 |
| round | **round 1 of this document's gate.** The governed run has not been taken (`opt_arc_ledger.md` §P1: *"landing bench, identity leg, solver oracle gates \| not started"*) |
| execution | the §2.1 command block was EXECUTED (D-591) against the two existing release binaries; no `cargo` was run and nothing under `/home/tom/pistol-wt` was written. Everything adjudicated used `/usr/bin/grep`, `git log`/`git show` pinned to `ffc5c10`, or my own `awk`/`python3`, per CLAUDE.md's Environment note (D-265) |

**VERDICT: FAIL (B1, B2; MAJOR M1, M2, M3)**

---

## 0. What holds, stated first because most of it does

The document is materially better than the revision R3-M1 was raised against, and
the R3-M1 fix is real and verified, not asserted:

- **The §2.1 block as printed is the block the §2.4 receipt records.** I extracted
  the single fenced `sh` block from the reviewed revision and hashed it:
  `sha256 12748c3106cd837cfc342325134535fd81f122c2adbce7f54a54dc61a58239db`, which is
  byte-for-byte the `block.sh sha256` line inside `artifacts/p1_identity_dryrun_E_v1.txt`.
  §2.4's "the block above was executed" is true of *this* spelling.
- **I ran it.** `BASE=/home/tom/pistol-wt/rt-base/target/release/pistol` (`78a7600a…`),
  `CAND=/home/tom/pistol-wt/p1-measure/pistol-E` (`f333d101…`), from the live tree:
  `BASE lines 533 bestmove 128 error 0`, `CAND lines 533 bestmove 128 error 0`, both
  normalised transcripts `284f70afab0ac8f16343da78418b07de19572c1680e58961e8447717647dc3c6`,
  `RESULT: IDENTICAL`, exit 0. That digest equals the one in the recorded receipt, so
  §2.4's numbers are reproduced by an independent run, not transcribed.
- **R3-M1 does not repeat and the fix introduced no new one.** The `entries()` keyed
  branch reads 20 lines from `tactical_staged_v0.txt` and refuses if they disagree
  with the case count; the bare branch reads 24 from `bench_positions_v1.txt`;
  `(20 + 24 + 20) × 2 = 128` is the count the run actually produced on both sides.
- **The normalisation does not elide too much.** Raw: `info depth_turns 1 seldepth 1 nodes 1 nps 16763 time 0 hashfull 0 score mate 1 pv 5,-5`;
  after `sed -E 's/ nps [0-9]+ time [0-9]+//'`: the same line minus ` nps 16763 time 0`.
  `nodes`, `seldepth`, `hashfull`, `score` and `pv` all survive.
- **The leg is not trivial.** Over my 128 `info totals` lines: min 1 node, median 4 913,
  max 953 167, sum 11 858 135; 62 of 128 searches exceed 10 000 nodes.
- **Every revision in §0 is correct.** All seven re-derived below with my own command.
- **Every gate number in §3 is correct**, and `GATE_TOTAL=20`.
- **The ±0.05 is an allowance, not a fitted margin.** Two independent runs of the same
  binary pair (`p1_mx_bench_E_v1.txt`, `p1_rt_round1/bench_E.txt`) differ by 0.001 in
  each band; the allowance is fifty times the observed drift. (What is *not* right is
  the interval the allowance was turned into — M1.)
- **§4's "no book slice" is true.** `bench_positions_v1.txt`'s header derives it from an
  external human corpus `b2fe61eb360b91d7…`, 8 698 games — not `random_openings_v2.txt`.
  `tactical_staged_v0.txt` has no book provenance, and `bench_delta.sh` reads no other
  position file.
- The abort's prose is arithmetically true: `<1.10` is 61.1 % / 58.3 % of the prototype's
  gain lost, i.e. *"more than half"*.

---

## 1. BLOCKING

### B1 — §1.2's dry run has never been taken, and the document records no output for it

`docs/process.md`, dry-run discipline: *"A pre-registration's literal commands are
exercised **before its review passes** … The pre-registration records the dry-run input
**and its output**."*

§1.2 records an input (`tools/bench_delta.sh rev:ffc5c10 rev:4298ecd 5`), a defect class,
a criterion (±0.03 of 1.257 / 1.240) and a consequence — and **no output, no artifact
name, and no statement that it was run.** The asymmetry with §2.4 is decisive: §2.4 ends
with *"**Taken, before this revision was committed:** `artifacts/p1_identity_dryrun_E_v1.txt`
… The criterion holds."* §1.2 has no such paragraph, and §0's cost sentence puts the dry
run in the future alongside the run it precedes:

> The landing bench, **its dry run**, the identity leg and its falsifier together are
> under half an hour of machine time

**Reproducer.**

```
$ awk '/^### 1.2 Dry run/,/^### 1.3/' <(git show 988c572b…:docs/experiments/p1_bench_prereg.md) \
    | /usr/bin/grep -c -i 'taken\|artifacts/'
0
$ awk '/^### 2.4/,/^---/' <(git show 988c572b…:docs/experiments/p1_bench_prereg.md) \
    | /usr/bin/grep -c -i 'taken\|artifacts/'
2
$ /usr/bin/grep -n 'path mode' docs/experiments/opt_arc_ledger.md
28:| matrix benches, `tools/bench_delta.sh` path mode, 5 reps | done | …
```

**Why it is blocking and not a formality.** The registered instrument for the *only*
number this package banks is `bench_delta.sh` in **`rev:` mode**, and every P1 bench taken
to date — the six matrix benches and round 1's re-measurement — was **PATH mode** on
hand-built binaries. The `rev:` path (worktree creation, `--release --locked` build,
digest, teardown) has not been exercised once in this package. That is precisely the stage
§1.2 names as under doubt (*"a wrong or stale build in a `rev:` worktree"*) and precisely
the stage the matrix's own attack 4 says the landing bench will *"tie … a third way"*. A
review that passes here passes a document whose load-bearing command has never run.

**What would clear it.** Take the run, record its two nps ratios and its exit status
against the ±0.03 criterion, and name the artifact it lands in (see m5). Note the run is
cheap by the document's own cost paragraph and the branch is present locally
(`git cat-file -t 4298ecd` → `commit`; `git branch --contains 4298ecd` → `p1/mx-E`).

### B2 — §0's instrument table omits `configs/eval_v0_weights.toml`, and the table asserts it is complete

§0 closes with a completeness claim:

> `bench_delta.sh` reads the LIVE tree's config and fixture for both sides (its own
> header), so **the revisions above are the instrument for every run below** regardless
> of which revision is being measured.

`configs/eval_v0_weights.toml` is read live by the bench instrument *and* by all three
identity seats, and it is not "above".

**Reproducer.**

```
$ /usr/bin/grep -n 'WEIGHTS\|weights' tools/bench_delta.sh | sed -n '2,5p'
93:WEIGHTS="configs/eval_v0_weights.toml"
138:[ -f "$WEIGHTS" ] || fail "no weights at $WEIGHTS"
272:digest "$WEIGHTS" "the instrument weights"; echo "bench_delta: instrument $WEIGHTS $DIGEST"
282:GUARDED_ID_FIELDS="config eval tt_bytes candidate_policy weights_sha256"

$ for c in configs/instrument_v0.toml configs/tactical_staged_v0.toml configs/gate_staged_solver_v0.toml; do
    /usr/bin/grep -Hn 'weights_file' "$c"; done
configs/instrument_v0.toml:91:weights_file = "configs/eval_v0_weights.toml"
configs/tactical_staged_v0.toml:54:weights_file = "configs/eval_v0_weights.toml"
configs/gate_staged_solver_v0.toml:59:weights_file = "configs/eval_v0_weights.toml"

$ git log -1 --format=%h ffc5c10 -- configs/eval_v0_weights.toml
64c336f

$ /usr/bin/grep -c 'eval_v0_weights' <(git show 988c572b…:docs/experiments/p1_bench_prereg.md)
0
```

The artifact the brackets descend from prints it as an instrument input in its own header:
`bench_delta: instrument configs/eval_v0_weights.toml 41ef5496…`.

**Why it is blocking.** `docs/process.md`: *"An artefact that produces a registered number
… is named in the pre-registration WITH ITS REVISION, and a change to it reopens the review
exactly as an amendment to the document does."* The weights file determines every nps
number in §1.1 and every `bestmove` line in §2. A change to it between this review and the
landing would move all of them while every revision in §0's table stayed put — and the
document's own sentence would then license the claim that the run was governed by the
reviewed instrument. This is a one-row fix, but the row is missing and the completeness
claim is affirmative.

---

## 2. MAJOR

### M1 — neither bracket is what its stated ground produces, and the late band's is off in the direction that matters

§1.1 states the ground as a per-band derivation: the matrix's measured ratio *"widened by
±0.05 for run-to-run drift"*, and for the late row *"same widening"*.

| band | measured | ground applied | **registered** | agree? |
|---|---|---|---|---|
| early | 1.257 | [1.207, 1.307] | [1.20, 1.30] | no |
| late | 1.240 | **[1.190, 1.290]** | [1.20, 1.30] | **no** |

**Reproducer.**

```
$ python3 -c "print((round(1.257-0.05,3),round(1.257+0.05,3)), (round(1.240-0.05,3),round(1.240+0.05,3)))"
((1.207, 1.307), (1.19, 1.29))
```

**What it changes about what the run may conclude.** The document attaches consequences to
bracket membership, so the 0.01 shift is not cosmetic:

- a late landing at **1.195** is inside 1.240 ± 0.05 — inside what the document calls the
  drift allowance — yet the registered bracket makes it *"a FINDING, reported against the
  bracket"*;
- a late landing at **1.295** is outside 1.240 ± 0.05, yet the registered bracket makes it
  a pass, and it therefore escapes *"**Above bracket** is a FINDING TO EXPLAIN … the
  results document names it or the package STOPs"*.

The registered brackets are a single rounded interval applied to both bands, presented as a
per-band derivation. D-374 forbids moving them after the run, which makes it the more
important that they follow from the ground *before* it. Either restate the ground as a flat
stated allowance ("[1.20, 1.30] in both bands, chosen to cover both measured ratios plus a
drift allowance of at least ±0.04") or register [1.19, 1.29] for the late band. Both are
defensible; the present pairing is not.

### M2 — §1.3's registered idle receipt cannot print `idle`, matches its own invoking shell, and is a second, divergent spelling of §2.1's

§1.3 registers `pgrep -af 'cargo|rustc|bench_delta|pistol'`. §2.1's block registers
`pgrep -af 'cargo|rustc|bench_delta|target/release/pistol' | grep -v pgrep || echo idle`.
These are different commands with different alternations and different plumbing, both
described as "the idle receipt" (D-423: *a claim the document makes twice is a defect
waiting*). §1.3's spelling has no `|| echo idle` at all, so `idle` — the word §2.1 relies
on — is not in its output alphabet, and its bare `pistol` alternative matches every process
whose command line names a path under `/home/tom/pistol-wt`.

**Reproducer** (the pattern string itself contains the words, so the shell holding the
pgrep invocation always matches; `pgrep` excludes only its own pid):

```
$ bash -c 'me=$$; pgrep -af "cargo|rustc|bench_delta|pistol" \
    | awk -v m="$me" "\$1==m {print \"SELF-MATCHED: \" substr(\$0,1,90)}"'
SELF-MATCHED: 1512198 bash -c me=$$; pgrep -af "cargo|rustc|bench_delta|pistol" | awk -v m=

$ bash -c "pgrep -af 'zzz_no_such_process_xyz'; echo exit=\$?"
1512215 bash -c pgrep -af 'zzz_no_such_process_xyz'; echo exit=$?
exit=0
```

The second line is the general case: even a pattern matching *nothing* on the machine
returns the shell that holds it, and exit 0. In practice §1.3's receipt will be taken in the
same command line as the run it precedes (`pgrep …; tools/bench_delta.sh rev:… 5`), which
contains `bench_delta` and therefore matches too.

**What it changes.** §1.3 makes VOID sound mechanical — *"a run whose receipt shows another
build, test or engine process alive is VOID and re-taken"* — over a receipt that always
shows at least one matching line. Deciding whether a shown line is "another build, test or
engine process" is then an unregistered judgement made after the numbers are in, which is
the shape D-592 exists to prevent. §2.4's own recorded receipt shows the failure mode
concretely: four of its five lines are agent-harness shell wrappers, and it needed a
hand-written `# NOTE:` to explain them away.

Fix: register one spelling for both sections, `| grep -v pgrep || echo idle` included, and
say in one sentence what a matching line has to *be* for VOID to fire.

### M3 — §0 says "the five CI gates in §3"; §3 names six

**Reproducer.**

```
$ /usr/bin/grep -n 'five CI gates' <(git show 988c572b…:docs/experiments/p1_bench_prereg.md)
45:beyond reading the receipts. The five CI gates in §3 are the ordinary CI cost.

$ awk '/^## 3\./,/^## 4\./' <(git show 988c572b…:docs/experiments/p1_bench_prereg.md) \
    | /usr/bin/grep -o '(gate [0-9]*)' | LC_ALL=C sort -u | wc -l
6
```

The six are gates 8, 9, 10, 11, 12, 13 — all six numbers correct against `tools/ci.sh`
(re-derived below). "Five" is transcribed from `D-603`'s *"the five gates that read this
state green at `3ef6706`"*, which was written before §3 added the tactical-fixture gate. A
count asserted in a governing document and falsified by the same document's next section is
exactly the class `docs/process.md`'s re-derivation clause is addressed to.

---

## 3. MINOR

- **m1 — §2.4 registers a criterion with no registered consequence.** §1.2 and §2.3 both
  end with *"Registered consequence of …"*; §2.4 does not. It has already been taken and
  passed, so nothing turns on it now, but the pairing is what the methodology asks for.
- **m2 — §2.3's falsifier has no reading for "printed neither".** The registered consequence
  covers only `RESULT: IDENTICAL`. The block runs under `set -euo pipefail`, so an M1 build
  that panics or exits non-zero inside the pipeline kills the script before either RESULT
  line is printed. That outcome — plausible for a deliberately broken window read — has no
  registered reading. One clause ("anything other than `RESULT: MISMATCH` is a defective
  leg") closes it.
- **m3 — the block writes two untracked files into the live tree root immediately before the
  landing commit.** `out.BASE` / `out.CAND` are created in `$PWD`, which must be the repo
  root because the SEATS heredoc uses relative paths. Neither `.gitignore` nor
  `tools/artifact_check.sh`'s `ARTIFACT_PATTERNS` (`*.bin`, …, `artifacts/*`, `logs/*`,
  `bench-out/*`) covers them, and `artifact_check.sh` scans `git ls-files`, so it fires only
  *after* a `git add -A` has already committed them (rule 8). My run left both at
  `45529` bytes in the repo root; I moved them to my scratchpad and removed them. Writing
  them under a `$TMPDIR`, or naming them in `.gitignore`, removes the hazard.
- **m4 — "the committed solver-on seat" is definite where three exist.** §1's Seat bullet
  attributes *"about two per cent early and about none late"* to *"the committed solver-on
  seat"*; the matrix's §6 attack 1 took that measurement under
  `configs/bench_wp18c_solver_on.toml`, while §0 and §2.1 use
  `configs/gate_staged_solver_v0.toml` for the third identity seat. Three configs carry
  `on_search_path = true`. Name the config in the Seat bullet.
- **m5 — no artifact is named for the §1.2 dry run or the landing bench.** §2.4 names its
  receipt by path and digest; §1.3 requires a receipt *"printed into the run's own
  artifact"* and never says what that artifact is called. Registering the filenames is what
  makes the D-427 slot-filling checkable.
- **m6 — `tools/ci.sh` is not in §0's table, and the document is not on the citation gate's
  list.** §3 registers `tools/ci.sh`'s gate numbers and *"twenty gates"* as facts about
  `<landed>`; `git log -1 --format=%h ffc5c10 -- tools/ci.sh` → `9390b48`, `GATE_TOTAL=20`.
  Separately, `tools/governing_citation_check.sh`'s named list carries
  `sealbot_anchor_v3_prereg.md`, `wp21_prereg.md` and `wp21_throughput_prereg.md` but not
  `p1_bench_prereg.md`, so this document's `tools/determinism.sh:76` and `:204` citations
  are outside gate 20's coverage. (Both are correct today — verified below.)
- **m7 — the dispatch asks for "both fixture sets".** `opt_arc_DISPATCH.md`'s perf standing
  rule reads *"Bracket registered before measuring, both bands, both fixture sets"*.
  `bench_delta.sh` has exactly one fixture (`FIXTURE=…/bench_positions_v1.txt`), so the
  clause cannot be met by this instrument. The prereg registers one fixture set and does not
  say why; one sentence closes it.
- **m8 — §2 states the identity claim unbounded.** *"the claim is BIT-IDENTITY of search
  output"*. What the leg establishes is bit-identity over the 128 registered searches at
  three seats — the ground on which D-495's no-SPRT exemption rests. Bounding the sentence
  to the registered searches costs nothing and stops the results document from widening it.
- **m9 — the abort overlaps the instrument's own printed ABORT and the document does not say
  so in numbers.** `tools/bench_delta.sh:452` prints `VERDICT ABORT` below **1.15**; §1.1
  registers the package's abort at **1.10**. A landing in [1.10, 1.15) therefore goes to
  `dev` as a below-bracket FINDING while the cited instrument line reads *"the change is
  reverted and this number is the finding"*. §1.1's D-327 paragraph resolves the precedence
  in general terms; naming the overlapping interval makes it unarguable at read time.
- **m10 — §1.2's ±0.03 has no stated ground.** §1.1's ±0.05 cites WP-1.9's three runs
  composing to within 0.002; ±0.03 is asserted. The observed run-to-run drift on this exact
  pair is 0.001, so ±0.03 is generous — but it is also *tighter* than the same document's
  drift allowance, and a miss STOPs the package. One clause of ground removes the asymmetry.

---

## 4. Re-derivation table

Every count and citation below was re-derived with a command I chose, printed with its
scope. Nothing in this table was produced by running a command the document supplies.

| claim | my command (scope) | my number | document's | agree? |
|---|---|---|---|---|
| `tools/bench_delta.sh` revision | `git log -1 --format=%h ffc5c10 -- tools/bench_delta.sh` | `ab369b0` | `ab369b0` | ✔ |
| …UNCHANGED since WP-1.9b, which names the same | `/usr/bin/grep -n 'bench_delta.sh' docs/experiments/wp19b_bench_prereg.md` (whole file) | `:15 \| ab369b0` | "names the same revision" | ✔ |
| `configs/instrument_v0.toml` revision | `git log -1 --format=%h ffc5c10 -- configs/instrument_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `bench_positions_v1.txt` revision | `git log -1 --format=%h ffc5c10 -- crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` | `70cc465` | `70cc465` | ✔ |
| `configs/tactical_staged_v0.toml` revision | `git log -1 --format=%h ffc5c10 -- configs/tactical_staged_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `configs/gate_staged_solver_v0.toml` revision | `git log -1 --format=%h ffc5c10 -- configs/gate_staged_solver_v0.toml` | `e4bb5bf` | `e4bb5bf` | ✔ |
| `tactical_staged_v0.txt` revision | `git log -1 --format=%h ffc5c10 -- crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt` | `538b3e5` | `538b3e5` | ✔ |
| **`configs/eval_v0_weights.toml` revision** | `git log -1 --format=%h ffc5c10 -- configs/eval_v0_weights.toml` | `64c336f` | **absent** | **✘ B2** |
| `bench_positions_v1.txt` = 24 positions | `awk '/^#/{next} /^[[:space:]]*$/{next} {n++} END{print n}' …/bench_positions_v1.txt` | 24 | 24 | ✔ |
| …both bands | `/usr/bin/grep -c '^band centre' …/bench_positions_v1.txt` and the artifact's `24 positions (12 early, 12 late)` | 2 bands, 12+12 | "both bands" | ✔ |
| `tactical_staged_v0.txt` = 20 cases, keyed | `awk '/^case /{c++} /^position /{p++} END{print c,p}' …/tactical_staged_v0.txt` | 20 cases, 20 `position` | "20 cases, keyed" | ✔ |
| the same fixture read as BARE = 115 lines | `/usr/bin/grep -v '^#' …/tactical_staged_v0.txt \| /usr/bin/grep . \| sed 's/ #.*//' \| wc -l` | 115 | 115 | ✔ |
| the old spelling sent 508 a side, 460 refused | `python3 -c "print((115+24+115)*2, 115*2*2)"` | 508, 460 | 508, 460 | ✔ |
| `(20 + 24 + 20) × 2` = 128 searches a side | `python3 -c "print((20+24+20)*2)"` | 128 | 128 | ✔ |
| the block actually sends 128, 0 errors | I ran the extracted block; `wc -l`, `grep -c '^bestmove '`, `grep -c '^error '` on my own `out.BASE`/`out.CAND` | 533 / 128 / 0 both sides | 128 / 0 | ✔ |
| the two binary digests differ | `sha256sum /home/tom/pistol-wt/rt-base/target/release/pistol /home/tom/pistol-wt/p1-measure/pistol-E` | `78a7600a…` ≠ `f333d101…` | same pair | ✔ |
| §2.4 receipt's block is this block | `awk '/^```sh$/{f=1;next} /^```$/{f=0} f' <doc> > block.sh; sha256sum block.sh` vs `/usr/bin/grep 'block.sh sha256' artifacts/p1_identity_dryrun_E_v1.txt` | `12748c31…` = `12748c31…` | "executed before this revision was committed" | ✔ |
| §2.4's transcript digest | my run's `sha256sum out.BASE out.CAND` | `284f70af…` (both) | `284f70af…` | ✔ |
| third seat's config + budgets | `sed -n '76p' tools/determinism.sh` | `"staged-solver configs/gate_staged_solver_v0.toml …/tactical_staged_v0.txt depth_turns-2 nodes-10000"` | `gate_staged_solver_v0.toml`, `depth_turns 2`, `nodes 10000`, `:76` | ✔ |
| the two WP-1.9 seats' budgets | `sed -n '85p' tools/determinism.sh` | `BUDGETS=("depth_turns 4" "nodes 200000")` | `depth_turns 4`, `nodes 200000` | ✔ |
| `determinism.sh:204` is the `position` extraction | `sed -n '204p' tools/determinism.sh` | `mapfile -t positions < <(sed -n 's/^position //p' "$fixture")` | ":204 reads it the same way" | ✔ |
| WP-1.9's own leg = 88 searches (20+24)×2 | `/usr/bin/grep -n 'searches asked' artifacts/wp19b_byte_identity_v2.txt` | 88 | "WP-1.9's two seats (20 + 24)" | ✔ |
| gate numbers 8–13 | `/usr/bin/grep -n 'step "gate' tools/ci.sh` | 8 tactical fixture, 9 determinism, 10 search oracle, 11 staged soundness, 12 solver oracle, 13 solver determinism | identical | ✔ |
| `tools/ci.sh` has twenty gates | `/usr/bin/grep -n 'GATE_TOTAL=' tools/ci.sh` | `readonly GATE_TOTAL=20` | "twenty gates" | ✔ |
| **number of gates named in §3** | `awk '/^## 3\./,/^## 4\./' <doc> \| /usr/bin/grep -o '(gate [0-9]*)' \| sort -u \| wc -l` | **6** | **"five CI gates in §3"** (§0) | **✘ M3** |
| `p1_mx_bench_E_v1.txt` digest | `sha256sum artifacts/p1_mx_bench_E_v1.txt` | `d0b1b38480b740ce…` | `d0b1b384…` | ✔ |
| …its ratios | `/usr/bin/grep 'nps ratio' artifacts/p1_mx_bench_E_v1.txt` | early 1.257, late 1.240 | 1.257 / 1.240 | ✔ |
| round-1 re-measurement digest | `sha256sum artifacts/p1_rt_round1/bench_E.txt` | `e3eaed80794c81eb…` | `e3eaed80…` | ✔ |
| …its ratios | `/usr/bin/grep 'nps ratio' artifacts/p1_rt_round1/bench_E.txt` | early 1.256, late 1.241 | 1.256 / 1.241 | ✔ |
| §2.4 receipt digest | `sha256sum artifacts/p1_identity_dryrun_E_v1.txt` | `197f7d3b0b66728f…` | (not stated) | n/a |
| **early bracket from its stated ground** | `python3 -c "print(round(1.257-0.05,3), round(1.257+0.05,3))"` | [1.207, 1.307] | [1.20, 1.30] | **✘ M1** |
| **late bracket from its stated ground** | `python3 -c "print(round(1.240-0.05,3), round(1.240+0.05,3))"` | **[1.190, 1.290]** | [1.20, 1.30] | **✘ M1** |
| abort = "more than half the prototype's gain lost" | `python3 -c "print((1.257-1.10)/(1.257-1)*100, (1.240-1.10)/(1.240-1)*100)"` | 61.1 % / 58.3 % | "more than half" | ✔ |
| `bench_delta.sh`'s own thresholds | `sed -n '452,456p' tools/bench_delta.sh` | ABORT `<1.15`, bracket `[1.4, 2.5]`, TTD `<1.4` | `[1.4, 2.5]` / `1.15` | ✔ |
| REPS 5 is legal | `sed -n '136p' tools/bench_delta.sh` | `[ "$REPS" -ge 5 ] \|\| fail …` | `5` | ✔ |
| `rev:4298ecd` is resolvable here | `git cat-file -t 4298ecd; git branch --contains 4298ecd` | `commit`; `p1/mx-E` | implied | ✔ |
| matrix §6 attack 1's solver-on numbers | `sed -n '530,585p' docs/experiments/matrix_P1_threat_state.md` | time ratio 1.022 / 1.015 early, 1.005 / 1.001 late | "about two per cent early and about none late" | ✔ (rounds 1.015 up) |
| §4: no book slice | `/usr/bin/grep -n 'corpus_sha256' …/bench_positions_v1.txt` (fixture header, whole file) | `b2fe61eb36…`, 8 698 games, external human corpus — not `random_openings_v2.txt` | "no book slice" | ✔ |
| the identity leg is non-trivial | `/usr/bin/grep '^info totals ' out.BASE \| sed -E 's/.* nodes ([0-9]+) .*/\1/' \| sort -n \| awk …` | n=128, min 1, median 4 913, max 953 167, 62 > 10 000 | (not claimed) | n/a |

---

## 5. Answers to the questions this gate was asked

1. **Brackets and abort.** Derived from a MEASURED number in an artifact I verified by
   digest (`d0b1b384…`, 1.257 / 1.240, replicated at `e3eaed80…` as 1.256 / 1.241). The
   ±0.05 is a **stated allowance**, not a fitted margin — fifty times the observed 0.001
   drift, with the WP-1.9 precedent quoted. **But the intervals registered are not the
   intervals that allowance produces (M1).** The abort is reachable by the run. D-374 is
   quoted at the head and the "no bracket moves" clause appears three times.
2. **Governing revisions.** All seven verified correct with my own `git log`. One artefact
   the run reads is **missing** (`configs/eval_v0_weights.toml`, B2); `tools/ci.sh` is a
   second, weaker case (m6). The "no second instrument" ground **holds**:
   `docs/process.md` conditions the obligation on doubt, no doubt is raised against
   `bench_delta.sh`, replication is the 5 reps, and §1.2's PATH-mode referent — which is in
   substance a second instrument — names the stage under doubt (the `rev:` build path) and
   says how the referent does not share it, with a registered consequence.
3. **Dry runs.** §2.4: input of the same kind, criterion externally derived from the
   fixtures' own grammar, **taken, recorded, and reproduced by me byte for byte**; missing
   only a registered consequence (m1). §1.2: input of the same kind, criterion externally
   derived, consequence registered — **but never taken and no output recorded (B1)**.
4. **The identity block.** Executed. 128 searches a side, 0 errors, digests differ,
   `RESULT: IDENTICAL`, exit 0; the arithmetic matches the fixtures' real entry counts,
   which I derived independently; the third seat's config and budgets match
   `tools/determinism.sh:76`, and the other two match `:85`. **R3-M1 is closed and no new
   one replaces it.** One hygiene defect (m3).
5. **The falsifier.** A leg that cannot fail is genuinely excluded: the normalisation elides
   only ` nps N time N` (demonstrated on a live line), the two sides are distinct binaries
   checked by digest, and the fixture set drives 11.9 M nodes with 62 of 128 searches above
   10 000 nodes — M1's misread masks change `nodes`, `score` and `pv`, all of which the
   transcript preserves, so `RESULT: MISMATCH` is the expected print. The gap is the third
   outcome, not the second (m2).
6. **The idle receipt.** §1.3's registered pattern does **not** do what the document says
   (M2), and it is a second spelling of §2.1's.
7. **§3 / §4.** Gate numbers and the twenty-gate total are correct; §4's "no book slice" is
   true; §0's "five CI gates" is not (M3).
8. **Over-reach.** Two: the late bracket licenses a pass at 1.295 that its own ground
   excludes (M1), and §2's unbounded "BIT-IDENTITY of search output" reads wider than 128
   searches at three seats (m8).

---

## 6. Environment

`/usr/bin/grep`, `git grep`/`git log`/`git show` pinned to `ffc5c10`, `LC_ALL=C sort`, and
my own `awk`/`python3` (CLAUDE.md Environment, D-265). One engine workload was run: the
§2.1 block, from the live tree, against the two pre-existing release binaries, read-only.
No `cargo` was invoked; nothing under `/home/tom/pistol-wt` was written or built; no
worktree was created. The block's `out.BASE`/`out.CAND` were written to the repo root by the
block itself (m3), copied to this session's scratchpad and removed — `git status --porcelain`
shows the tree as it was found. The only file written into the repository is this report.

**VERDICT: FAIL (B1, B2; MAJOR M1, M2, M3)**
