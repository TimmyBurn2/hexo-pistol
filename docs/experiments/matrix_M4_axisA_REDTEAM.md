# MATRIX M4, AXIS A, ROUND 4 — FRESH-CONTEXT DECISION-RED-TEAM

**SUBJECT REVISION:** `docs/experiments/matrix_M4_axisA_round4.md` at **`7866bcf`**.

**DOES IT STILL MATCH HEAD? NO.** HEAD was `7866bcf` when this attack began and is
**`84ff8d7`** as it is written. The intervening commit adds one new file
(`docs/experiments/wp15b_trackC_REVIEW_impl.md`, 718 lines) and **`git diff --stat
7866bcf 84ff8d7 -- docs/experiments/matrix_M4_axisA_round4.md
tools/baseline_snapshot.sh` is empty** — neither the subject nor the script it costs
options against has changed. The attack therefore stands at HEAD.

**GROUND:** every measurement below was taken in an isolated worktree pinned at
**`b067d47`**, the revision the matrix pins its own measurements at, using the
measurer's own artefacts at `/home/tom/.cache/m4_matrix_measurement/` (`patch.py`,
`N-E.diff`, `N-M.diff`, `N-Q.diff`) read but never modified. The worktree was
removed; `git status --porcelain` in the repository is empty; no repository file was
edited and no git write command was run against the repository.

**SCOPE, AS DISPATCHED:** the new row **N-Q** and its interaction with the tied set
**{N-E, N-M}**. Axis B / N-K is not reopened. The ten rows carried from round 3 are
not reopened. Whether M4 should have stopped is not re-argued.

---

# 1. PER-FACT REPRODUCTION VERDICT (R7)

| fact | verdict |
|---|---|
| FACT 1 | **REPRODUCES** |
| FACT 2 | **REPRODUCES** — every cell, exactly |
| FACT 3 | **REPRODUCES** |
| FACT 4 | **REPRODUCES** — all eleven cases, including (i) and (j) |
| FACT 5 | **PARTIALLY — THE PASTED OUTPUT DOES NOT REPRODUCE.** Conclusion survives; the evidence line is wrong. See **F1** |
| FACT 6 | **REPRODUCES**, and I re-derived it independently rather than accepting FACT 4 as its proof |
| FACT 7 | **REPRODUCES** |
| FACT 8 | **REPRODUCES** |
| FACT 9 | **REPRODUCES** for N-Q and N-M (N-E not re-run; see the note) |
| FACT 10 | **REPRODUCES** |

### FACT 1 — REPRODUCES

```
$ grep -n 'CONFIG=' tools/baseline_snapshot.sh
182:CONFIG="configs/instrument_v0.toml"
$ ./tools/baseline_snapshot.sh --config configs/instrument_v0.toml
baseline_snapshot: FAIL: unknown argument `--config`
EXIT:1
$ grep -n -- '--config' tools/baseline_snapshot.sh
465:printf 'pistol\nquit\n' | timeout "$HANDSHAKE_TIMEOUT" "$BINARY" --config "$CONFIG" >"$WORK/hs" 2>/dev/null || HANDSHAKE_RC=$?
481:	# caller-named and the guard above does not cover it. If a `--config` flag is
514:timeout "$CORPUS_TIMEOUT" "$BINARY" --config "$CONFIG" <"$WORK/corpus.session" >"$WORK/corpus.out" || CORPUS_RC=$?
581:		timeout "$LADDER_CAP_S" "$BINARY" --config "$CONFIG" >"$out" 2>/dev/null || rc=$?
```

Four occurrences: three engine invocations plus the emit-block comment, as stated.

### FACT 2 — REPRODUCES, every cell

I applied the measurer's `patch.py` to a clean `b067d47` checkout of the script
alone and applied the stated counting rule myself (first non-whitespace `#` ⇒
COMMENT, else CODE):

```
--- N-E ---  numstat: 22  8   added CODE: 7   added COMMENT: 15   blank added: 0   bash -n: OK
--- N-M ---  numstat: 21  6   added CODE: 10  added COMMENT: 11   blank added: 0   bash -n: OK
--- N-Q ---  numstat: 32  8   added CODE: 12  added COMMENT: 20   blank added: 0   bash -n: OK
```

Identical to the matrix's table in all twelve cells. The code arithmetic is also
internally consistent: N-E's 7 = `CONFIG=""` + flag arm + `[ -n ]` + 4 guard lines;
N-Q's 12 = those 7 + exactly 5 containment lines; N-M's 10 = `CONFIG=""` +
8-line enum arm + `[ -n ]`.

### FACT 3 — REPRODUCES

```
N-E  run: config configs/instrument_v0.toml 3579855e7cf2…ec13   EXIT:0
N-E  absent --config: baseline_snapshot: FAIL: --config is required and has no default   EXIT:1
N-M  run: config configs/instrument_v0.toml 3579855e7cf2…ec13   EXIT:0
N-M  absent --config: baseline_snapshot: FAIL: --config is required and has no default   EXIT:1
N-Q  run: config configs/instrument_v0.toml 3579855e7cf2…ec13   EXIT:0
N-Q  absent --config: baseline_snapshot: FAIL: --config is required and has no default   EXIT:1
```

N-M's unrecognised-token refusal also reproduces verbatim: ``--config takes
`instrument` or `staged`, got `bogus` ``.

### FACT 4 — REPRODUCES, all eleven cases

Run one at a time against the shipped patched script (accept cases on a four-entry
two-band corpus, which changes nothing the predicate sees):

```
(a) configs/instrument_v0.toml            EXIT:0  config configs/instrument_v0.toml 3579855e…
(b) ../etc/passwd                         EXIT:1  … resolves to /home/tom/.cache/etc/passwd, which is not under …/configs/
(b) ../../../etc/passwd                   EXIT:1  … resolves to /home/etc/passwd, which is not under …/configs/
(c) /etc/hostname                         EXIT:1  … resolves to /etc/hostname, which is not under …/configs/
(d) configs/../configs/instrument_v0.toml EXIT:0  config configs/instrument_v0.toml 3579855e…   ← NORMALISED
(e) configs/evil_link.toml -> /etc/hostname EXIT:1 … resolves to /etc/hostname, which is not under …/configs/
(f) configs/nonexistent.toml              EXIT:1  no config at configs/nonexistent.toml
(g) configs/spaced name.toml              EXIT:1  … has a SPACE, and it is written into a whitespace-delimited field …
(h) /home/…/configs/instrument_v0.toml    EXIT:0  config configs/instrument_v0.toml 3579855e…   ← root-relative
(i) configs                               EXIT:1  … resolves to …/configs, which is not under …/configs/
(j) configs_evil/…                        EXIT:1  … not under …/configs/
(j) configs/../configs_evil/…             EXIT:1  … not under …/configs/
(k) configs/tab<TAB>name.toml             EXIT:1  … has a character outside printable ASCII …
(k) configs/nl<LF>name.toml               EXIT:1  … has a character outside printable ASCII …
```

Case (g)'s attribution — refused *by the guard, not by containment* — is correct.
I then tried nine escapes the measurer did not: they are **F4, F5, F6** and the
rejected list in §4.

### FACT 5 — **PARTIALLY. THE PASTED OUTPUT DOES NOT REPRODUCE.** See **F1**

The small half reproduces: at `b067d47` the guard carries two arms, so a copy of it
is four code lines and not three, and the matrix's N-E/N-Q "4 whole-path guard
lines" is right. The large half's *conclusion* reproduces — the basename loop cannot
guard a whole path. **The pasted probe line does not.** Full detail in **F1**.

### FACT 6 — REPRODUCES, independently re-derived

I did not take FACT 4(g)/(k) as the proof. I built N-Q **with the guard removed and
the line-289 loop reused** — the round-3 attack's actual "one word" proposal — and
drove it:

```
$ ./tools/baseline_snapshot.sh --config 'configs/spaced dir/instrument_v0.toml' …
EXIT:0
baseline_snapshot 1
config line NF=4, $3=dir/instrument_v0.toml
```

Exit 0, COMPLETE kind token, digest displaced out of its field. Containment does not
subsume the guard; the costs are additive; the round-3 "one word" claim is dead.

### FACT 7 — REPRODUCES

```
bogus                                   -> FAIL: --config takes `instrument` or `staged`, got `bogus`         EXIT:1
configs/spaced dir/instrument_v0.toml   -> FAIL: … got `configs/spaced dir/instrument_v0.toml`                EXIT:1
/etc/hostname                           -> FAIL: … got `/etc/hostname`                                        EXIT:1
$'nl\nname'                             -> FAIL: … got `nl                                                    EXIT:1
$ grep -n 'CONFIG=' <N-M patched>
187:CONFIG=""     251:instrument) CONFIG="configs/instrument_v0.toml"     252:staged) CONFIG="configs/instrument_staged_v0.toml"
```

Two committed literals and an empty initialiser; no caller byte reaches `$CONFIG`.

### FACT 8 — REPRODUCES

```
$ ls configs/instrument_staged_v0.toml
ls: cannot access 'configs/instrument_staged_v0.toml': No such file or directory
```

The full `configs/` listing carries no `staged` document. The matrix's correction of
D-324's framing — this blocks all three equally — is right and is worth having.

### FACT 9 — REPRODUCES

`git diff --numstat` on the retrofitted `crates/pistol-cli/tests/baseline_snapshot_tests.rs`
is `2  0` for both options I re-ran, and both suites are green against the SHIPPED
script:

```
N-Q: 2  0    test result: ok. 30 passed; 0 failed; … finished in 43.79s
N-M: 2  0    test result: ok. 30 passed; 0 failed; … finished in 43.69s
```

Two notes, neither of which unseats the fact. First, N-E was **not** re-run: its
retrofit is byte-identical to N-Q's, so the run would have added nothing; stated
rather than implied. Second, the two sites needing the flag are `Run::go()` (line
171) and `a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root`
(line 1205); there are **three** invocation sites of the script in that file, the
third being line 1036, which asserts a refusal that fires in the argument loop before
the required-config check and so needs no retrofit. FACT 9's "a second invocation
site exists outside the `go()` funnel" is true as written, but D-318 already recorded
an invocation-site miscount in this work package, so the third site is named here.
**What FACT 9 does NOT cost is F7.**

### FACT 10 — REPRODUCES

```
unpatched b067d47   WALL 33.002 s
N-Q patched         WALL 32.916 s
recorded timing:    base1 corpus_wall_ms 14437 / base2 14483 / nq 14374 / nq2 14372
```

And the stronger half of the claim holds under a diff I ran myself — the invariant
block (everything above `# timing`) is byte-identical across replicates **and across
the seam**:

```
=== base1 vs base2 invariant ===  IDENTICAL
=== base1 vs N-Q  invariant ===  IDENTICAL
=== nq    vs nq2  invariant ===  IDENTICAL
```

---

# 2. FINDINGS

## F1 — **MAJOR.** FACT 5's pasted output does not reproduce, and the defect is D-322's ATTRIBUTION class in the round's own headline

The matrix's FACT 5 pastes:

```
4th token a reader would take as the digest: dir/instrument_v0.toml
```

**Reproducer.** I built the literal reading exactly as FACT 5 describes it — the
N-E patch with the whole-path `case` deleted and `"$CONFIG"` added to the line-289
`for named in` loop — and printed every field of the emitted `config` line for the
clean and the spaced case:

```
########## --config 'configs/instrument_v0.toml' ##########
EXIT:0
config line: config configs/instrument_v0.toml 3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13
  $1 = config
  $2 = configs/instrument_v0.toml
  $3 = 3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13
  NF = 3
########## --config 'configs/spaced dir/instrument_v0.toml' ##########
EXIT:0
config line: config configs/spaced dir/instrument_v0.toml 3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13
  $1 = config
  $2 = configs/spaced
  $3 = dir/instrument_v0.toml
  $4 = 3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13
  NF = 4
```

**The `config` line carries THREE fields, and its digest is `$3`.** At `$4` — the
position the matrix's probe says it read — the shipped script returns the digest,
not `dir/instrument_v0.toml`. The value the matrix pastes is `$3`, printed under a
label naming the fourth token.

The four-token indexing is the **`corpus`** line's (`corpus <name> sha256 <hex>
positions N`), where `b067d47`'s own commit message correctly says "a reader taking
the digest from the line's fourth token got the literal string `sha256`". The probe
carried that reasoning across to a line with a different shape. That is exactly
D-322's diagnosis — *"an ATTRIBUTION defect and not a bias"* — and it is exactly what
D-322 says a synthetic check cannot catch.

**What survives, stated fairly:** the defect FACT 5 claims is REAL. The digest moves
out of its field, at exit 0, under the COMPLETE kind token. The honest demonstration
is that `$3` — where every reader of this record takes the config digest — becomes
`dir/instrument_v0.toml`. FACT 5's conclusion stands on that; its quoted evidence
does not stand at all.

**Why this is MAJOR and not MINOR.** D-328/R11 exists because three consecutive
sessions shipped MEASURED cells that did not reproduce. This round's own §"WHO
AUTHORED THIS" argues that *"the split earned its place in this round rather than
merely being observed"* — and cites **FACT 5** as the proof. FACT 5 is the cell whose
pasted output does not reproduce. **The split did not catch it; a red team did.**
That is the fourth consecutive instance of the pattern D-318 said was the finding,
and it is D-328's own registered flip condition (*"Flips if a split round nonetheless
ships a MEASURED cell that does not reproduce"*) on the round D-328 was written to
protect.

## F2 — **CRITICAL to the selection.** "One schema home" is a coined phrase, hard rule 1's clause is about DEFAULTS, and the directory it names is measurably not one schema

Three separate defeats, any one of which is sufficient.

**(a) The phrase does not exist in this repository outside the matrix that selects on it.**

```
$ grep -rn "schema home" CLAUDE.md docs/ tools/
docs/experiments/matrix_M4_axisA_round4.md:228, 233, 240, 270, 277
```

Five hits, all in the subject. Nowhere in CLAUDE.md, nowhere in `docs/decisions.md`,
nowhere in the three prior M4 revisions or their three attacks.

**(b) Hard rule 1's fourth clause is a rule about DEFAULTS, and none of the three has one.**
Its verbatim text is *"NO code-side default for any tunable — a default lives in
exactly one schema place."* MEASURED (FACT 3, reproduced above): all three options
refuse an absent `--config` by name at exit 1. A clause governing where a default
lives is silent where there is no default, and cannot discriminate three rows none of
which has one. The round-3 red team said precisely this, in terms, at
`docs/experiments/matrix_M4_REDTEAM_round3.md:243`:

> *"Rule 1 names defaults and N-M has none, so this is a WOUND, not a KILL."*

Round 4 takes that same clause, renames it **"one schema home"**, and promotes it
from a wound that could not kill N-M into the criterion that SELECTS N-Q. The rename
is doing the work.

**(c) Even granting the re-reading, `configs/` is not a schema home — it is a filesystem location holding at least three schemas.**

```
$ ls configs/
arena_smoke_v0.toml  arena_wp13_fair_corpus.toml  arena_wp13_fair_random.toml
arena_wp13_r2_vs_r3_confirm.toml  arena_wp13_r2_vs_r3.toml  arena_wp15b_dryrun.toml
eval_v0_weights.toml  gate_v0.toml  instrument_r2_v0.toml  instrument_v0.toml
play_v0.toml  random_openings_v1.toml
```

Driven through N-Q:

```
configs/arena_smoke_v0.toml  -> FAIL: the engine … answered the `pistol` handshake with no id lines (it exited 2 …)
configs/eval_v0_weights.toml -> FAIL: the engine … answered the `pistol` handshake with no id lines (it exited 2 …)
configs/play_v0.toml         -> config configs/play_v0.toml e2b7d3ed…   EXIT:0
```

Six arena match configs, one weights table, and four engine configs sit in one
directory; the engine exits 2 on two of the three I sampled. The bound N-Q enforces
is **directory membership**, and directory membership is not schema membership. The
actual "one schema place" for a pistol config in this tree is the
`serde(deny_unknown_fields)` `Config` type in pistol-engine — a place all three
options share equally and none of them touches.

The matrix's own concession is therefore triggered on its own terms: *"if a directory
is not a 'schema home' in the sense hard rule 1 means — then rung (a) does not
discriminate."*

## F3 — **MAJOR.** The tiebreak ladder is coined in the document that applies it, and where it stops is what picks the row

```
$ grep -rn "rung (a)\|rung (b)\|tiebreak ladder\|config law conformance\|fewest MEASURED added lines" docs/ CLAUDE.md
docs/experiments/matrix_M4_axisA_round4.md:226,227,246,248,250,271,272,278
```

Every hit is in the subject. No prior M4 revision, no stop record, no ADR line and no
clause of CLAUDE.md registers a rung ordering, or registers that rung (a) is hard
rule 1 and rung (b) is added lines, or registers that the ladder stops at the first
rung that discriminates. The matrix states plainly that at rung (b) **N-E wins**.
So the ordering of the ladder, authored after the numbers were in, is the whole of
what selects N-Q over N-E.

CLAUDE.md's Process section: *"Pre-register verdicts before experiments; no post-hoc
threshold moves."* A ranking rule authored in the same document as the measurements
it ranks, whose stopping point is what decides the outcome, is a post-hoc threshold
however honestly its consequence is disclosed. The matrix's disclosure — *"had the
ladder reached rung (b) … N-E would have won it"* — is real candour about the
consequence and no answer at all to the provenance. **It inoculates against the
objection it can survive while leaving the one it cannot unstated.**

## F4 — **MAJOR.** "Record names a committed, re-runnable document — BY CONSTRUCTION" is FALSE for N-Q

This is the matrix's field-table cell for N-Q and its stated "second ground"
(provenance, D-198). Containment bounds by DIRECTORY. It does not bound by COMMIT.

**Reproducer** — a file matching a `.gitignore` entry, dropped into `configs/`:

```
$ cp configs/instrument_v0.toml configs/ghost_v0.bin
$ git status --porcelain -- configs/
              ← empty: invisible to the tree-state check at line 474
$ git ls-files --error-unmatch configs/ghost_v0.bin
error: pathspec 'configs/ghost_v0.bin' did not match any file(s) known to git
$ ./tools/baseline_snapshot.sh --config configs/ghost_v0.bin --corpus … --ladder-depth 1
EXIT:0
baseline_snapshot 1
schema 1
revision b067d47083282ab7b66fb053a5b644d6a8487a26
binary_sha256 a7f519fa…
config configs/ghost_v0.bin 3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13
…
engine_id config configs/ghost_v0.bin
…
# timing — machine-, schedule- and worktree-dependent; excluded from every comparison
timing tree dirty
```

Exit 0, COMPLETE kind token, and the invariant block names `configs/ghost_v0.bin` at
`revision b067d47…`, a revision at which no commit contains that file. `git status
--porcelain -- configs/` is empty for it, so it cannot even move the tree state — and
`timing tree` sits **below** the `# timing` marker, outside the block every
before/after comparison reads, so tree state was never carrying this property anyway.

An ordinary untracked `configs/x.toml` reaches the same invariant block; it merely
also flips a line the comparison ignores.

So N-Q's provenance property holds **by caller discipline**, exactly as N-E's does.
The field table's `by construction` / `by caller discipline` column is wrong in the
cell it was built to win.

(The hardlink case is the same hole with a louder ending: `ln $SRC configs/hardlink_v0.toml`
passes containment, `[ -f ]` and `digest`, and only dies at the engine's handshake
with a message about the engine.)

## F5 — **MAJOR.** A new N-Q-only failure mode: `$ROOT` is LOGICAL, `realpath` is PHYSICAL, so N-Q refuses the repository's own registered config through a symlinked path

`ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"` (line 174) uses bash's
logical `pwd`. `realpath -m` returns the physical path. N-Q compares one against the
other.

**Reproducer, with an N-E control on the identical invocation:**

```
$ ln -sfn /home/tom/.cache/redteam-m4-axisA /home/tom/.cache/rt-m4-link
$ bash -c 'cd /home/tom/.cache/rt-m4-link/tools/.. && echo "ROOT (pwd)       = $(pwd)"; echo "realpath configs = $(realpath -m -- configs)"'
ROOT (pwd)       = /home/tom/.cache/rt-m4-link
realpath configs = /home/tom/.cache/redteam-m4-axisA/configs

$ # N-Q
$ /home/tom/.cache/rt-m4-link/tools/baseline_snapshot.sh --config configs/instrument_v0.toml --corpus … --ladder-depth 1
baseline_snapshot: FAIL: the config `configs/instrument_v0.toml` resolves to /home/tom/.cache/redteam-m4-axisA/configs/instrument_v0.toml, which is not under /home/tom/.cache/rt-m4-link/configs/
EXIT:1

$ # N-E, same command, same symlink
$ /home/tom/.cache/rt-m4-link/tools/baseline_snapshot.sh --config configs/instrument_v0.toml --corpus … --ladder-depth 1
config configs/instrument_v0.toml 3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13
EXIT:0
```

The refusal tells the operator that `configs/instrument_v0.toml` "is not under"
`…/configs/` — a directory it plainly IS under. **This is the wrong-diagnosis class
of SHELL_CHECKLIST item 8 and the void-read-as-fail class of item 12**: the answer
was not taken, and the message describes it as a bad config path. A reader acts on it
by looking for a defect in the config.

Not hypothetical for this project. `crates/pistol-cli/tests/common/mod.rs:107` builds
every scratch repository under `std::env::temp_dir()` — `$TMPDIR` or `/tmp`, **not
canonicalised** — and this repository's own operating memory records that `/tmp` here
is a 24 GiB tmpfs whose exhaustion kills shells, so pointing `TMPDIR` elsewhere is
routine practice. A `TMPDIR` whose path carries one symlink component breaks every
`ScratchRepository` test under N-Q and none under N-E or N-M.

Cost of the fix: `ROOT` must be canonicalised, or `realpath` must be logical. That is
a further N-Q-only line, on top of the 32 already counted, and neither the matrix nor
its failure-mode list contains it.

## F6 — **MODERATE.** N-Q resolves a relative `--config` against `$ROOT` while the same invocation resolves a relative `--out` against `$CALLER_PWD`

The containment block sits at line ~285, well after `cd "$ROOT"` at line 175, so a
relative `--config` is resolved against the repository root. `--out` is explicitly
resolved against `CALLER_PWD` captured before the `cd`, at lines 254–264 — a fix that
`tools/SHELL_CHECKLIST.md` **item 11** records in its own overwrite paragraph
(*"A caller's relative path is resolved against the directory the CALLER was standing
in, captured before the `cd`."*).

**Reproducer — two relative caller paths, two different bases, one invocation:**

```
$ ( cd /home/tom/.cache/rt-m4-cwd && "$W/tools/baseline_snapshot.sh" \
      --config configs/instrument_v0.toml --corpus … --ladder-depth 1 --out ./probe_out.txt )
EXIT:0
$ ls /home/tom/.cache/rt-m4-cwd/
probe_out.txt                       ← --out resolved against CALLER_PWD
$ grep '^config ' /home/tom/.cache/rt-m4-cwd/probe_out.txt
config configs/instrument_v0.toml … ← --config resolved against ROOT
```

And the consequence for the operator standing where the config is:

```
$ ( cd "$W/configs" && "$W/tools/baseline_snapshot.sh" --config instrument_v0.toml … )
baseline_snapshot: FAIL: the config `instrument_v0.toml` resolves to /home/tom/.cache/redteam-m4-axisA/instrument_v0.toml, which is not under /home/tom/.cache/redteam-m4-axisA/configs/
EXIT:1
```

N-E and N-Q share the base; only N-Q makes the base *load-bearing for a refusal*, so
only N-Q converts the inconsistency into a named wrong answer. Item 11's own remedy
for `--out` is unapplied to the binding this matrix is about.

**And the converse cuts harder against N-Q's whole premise.** `--out` is this
script's one genuine item-11 binding — a caller-supplied path consumed by a **write**
— and it is deliberately *resolved and not contained*: E5's `probe_out.txt` landed
outside the repository at exit 0, by design. **N-Q proposes to contain a READ binding
more tightly than this script contains its WRITE binding.** That is not the
checklist's shape borrowed; it is the checklist's shape inverted.

## F7 — **MAJOR.** Every row's cost cell omits the coverage cost, and the omission is NOT identical across rows

FACT 9 counts 2 test lines, "identical for all three", and stops. That retrofits
existing callers. It buys **no test for any new refusal**.

`tools/SHELL_CHECKLIST.md` **item 10, THE COVERAGE RULE**: *"Any `tools/` script that
produces a recorded number carries at least one test … driving the SHIPPED script …
with a control run so a pass cannot come from a gate that refuses everything."*

The precedent is one commit old and is the very revision these options are costed
against:

```
$ git show b067d47 --numstat -- crates/pistol-cli/tests/baseline_snapshot_tests.rs tools/baseline_snapshot.sh
91	0	crates/pistol-cli/tests/baseline_snapshot_tests.rs
54	4	tools/baseline_snapshot.sh
```

**ONE new guard arm cost 91 test lines**, in two halves, refusal and control, exactly
as item 10 requires.

What each row newly owes under item 10, and it is asymmetric:

| row | new refusal classes needing a driving test |
|---|---|
| N-M | 1 — the unrecognised token |
| N-E | 2 — non-printable whole path, spaced whole path |
| **N-Q** | **5** — those 2, plus absolute-escape, plus `..`-escape, plus symlink-out — **plus a control for the NORMALISATION**, a silent rewrite of a caller value into the invariant block that nothing pins |

I confirmed the retrofitted N-Q suite drives none of these: 30 tests, the same 30 as
before, with two extra `--config` arguments. **N-Q is the most expensive row on the
matrix's own measured counts and the gap widens once item 10 is costed** — which the
matrix does not do for any row, and which no reader can compute from FACT 9 because
FACT 9 reports the one component that is identical.

## F8 — **MODERATE.** The differing-ground test is passed by counting a shared ground as a differing one

The test the matrix states: *"A ground on which the candidate merely joins one tied
row against another is not differing ground."* Its application: *"against N-M, N-Q
differs twice."* The two are (i) it admits any `configs/<basename>` rather than two
enumerated names, and (ii) *"it does not raise the hard-rule-1 question AT ALL."*

**N-E does not raise the hard-rule-1 question either, and N-E admits any path.** Both
of N-Q's stated differences from N-M are differences N-E already has. On the test's
own words, both are grounds on which N-Q "merely joins one tied row against another".
The only ground N-Q holds alone is containment — *"against N-E, N-Q differs on what
can reach the record"* — one ground, not three.

That matters because containment is also the whole of the selector at rung (a). The
matrix presents admission and selection as two independent arguments; they are one
argument used twice.

## F9 — **MODERATE.** The matrix IS unfair to N-M: it escalates an admitted-unresolved judgement into a stated failure, with no new evidence

The matrix writes **"N-M fails 'one schema home'."** Read first-hand:

- **D-324** calls it *"a judgement rather than a measurement … The red team could not
  call N-M a breach and could not call the distinction sound."*
- **The round-3 red team** calls it *"a WOUND, not a KILL."*
- **Hard rule 1's text** forbids a *code-side default*. N-M has none — reproduced:
  `--config is required and has no default`, exit 1.
- `Budget` itself is a closed set of variant names living in **Rust code**
  (`crates/pistol-engine/src/budget.rs`), not in a schema document. N-M's two literals
  are the admissible *set*, structurally the same object, with the caller supplying
  the token. The disanalogy D-324 states is real as a disanalogy; it is not a breach
  of the rule's text, and no measurement in round 4 converts it into one.

Round 4 adds no evidence and upgrades the verdict. That is the *forcing a survivor
under another name* that D-318 named. Its own sentence — *"Round 4 does not resolve
that judgement by argument; it declines to need it"* — is not what the surrounding
paragraph does: the paragraph's first clause is a verdict of FAILURE against N-M.

**N-M does nevertheless fall, on a measured ground the matrix never gives — see F10.**

## F10 — **MAJOR, and it is an omitted GROUND rather than an omitted cost.** A registered downstream pre-registration already names the flag's shape, and N-M cannot take it

`docs/experiments/wp15b_sprt_prereg.md` §7A.2 registers this script as the instrument
for DOUBT 2 and states the invocation:

> *"MEASURED at this document's revision, `tools/baseline_snapshot.sh --config
> configs/gate_v0.toml` answers `baseline_snapshot: FAIL: unknown argument --config`"*

and §10 registers the flip: *"The DOCUMENT flips … if `tools/baseline_snapshot.sh`
lands `--config` in a shape the §7A.2 criterion cannot be taken under."*

**Driven against all three rows:**

```
N-E with 'configs/gate_v0.toml': config configs/gate_v0.toml d3f852e423f8…    EXIT:0
N-M with 'configs/gate_v0.toml': FAIL: --config takes `instrument` or `staged`, got `configs/gate_v0.toml`   EXIT:1
N-Q with 'configs/gate_v0.toml': config configs/gate_v0.toml d3f852e423f8…    EXIT:0
```

N-M lands `--config` in precisely the shape §7A.2's criterion cannot be taken under.
Selecting it fires a registered flip clause in a pre-registration whose §11 records
*"This document has never passed a review"* — an amendment there reopens a review that
has failed three times.

This is a real, measured, asymmetric cost. **The matrix contains no reference to
`wp15b_sprt_prereg.md` at all**, in either its cost cells or its flip clauses. It
tells against N-M and does not separate N-E from N-Q, so it does not rescue the
recommendation; it does mean the cost table is incomplete in both directions.

## F11 — **MODERATE.** D-324's stop record set four conditions on a fourth round; two are not discharged

- **Condition 3 — "`tools/SHELL_CHECKLIST.md` ANSWERED ITEM BY ITEM AND BY NAME …
  item 12 (VOID vs FAIL) is unclassified for every naming option."**

  ```
  $ grep -o "item [0-9]*" docs/experiments/matrix_M4_axisA_round4.md | sort | uniq -c
        3 item 11
        4 item 9
  ```

  Two of twelve. **Item 12 is named nowhere**, though the stop record singles it out.
  This is better than round 3, which claimed the checklist was answered and answered
  nothing, and its treatment of items 11 and 9 is substantive and correct — but the
  condition is not discharged. (I answer the items in §6.)

- **Condition 4 — "No MEASURED cell without its command AND its complete output."**
  FACTS 1 and 8 carry their commands. **FACTS 2, 3, 4, 6, 7, 9 and 10 carry neither a
  literal command nor a complete output** — FACT 2 is a bare table, FACT 4 is
  one word per case, FACT 10 is two numbers. D-328 clause (3) is explicit: *"a cell
  whose log does not appear is not MEASURED."* Every one of them did in fact
  reproduce when I ran it, so nothing is falsified here — but they reproduced because
  I reconstructed the commands, which is the work the condition existed to remove.
  And F1 is what a reader who could not reconstruct one would have missed.

## F12 — **MINOR.** N-Q's four failure modes are incomplete; five are missing

(v) the containment predicate carries no test, and its normalisation carries no
control (F7, item 10); (vi) the logical/physical root mismatch, a refusal of the
repository's own config (F5); (vii) containment does not bound to committed documents
(F4); (viii) `configs/` is not one schema and containment admits documents the engine
exits 2 on (F2c); (ix) a relative `--config` and a relative `--out` in one invocation
resolve against different bases (F6). Failure mode (iii) — the unmeasured
normalisation — is honestly named and is the closest the list comes to any of these.

## F13 — **MINOR, and it is a point in the matrix's favour that I checked and confirmed.** The matrix's two charges against the round-3 attack are BOTH correct

- **"is SHELL_CHECKLIST item 11" — over-claim, and round 4 is right to refuse it.**
  Item 11's scope sentence is *"Any binding consumed by `rm`, `mv`, or a **write**"*
  and its sweep paragraph says *"enumerate every **destructive** site"*. `$CONFIG` is
  consumed by `[ -f ]`, `digest`, three engine invocations and one `echo` of its
  VALUE into a record. None is a write to the path. **Item 9** governs it. Round 4's
  refusal to inherit the citation, in a round licensed by a stop record that itself
  repeats the mis-citation, is the best thing in the document.
- **"adding one word" — over-claim, and FACT 6 kills it.** Independently
  re-derived above: the loop guards `${named##*/}`, the emit block writes `$CONFIG`,
  and `configs/spaced dir/instrument_v0.toml` walks through at exit 0 with the digest
  displaced.

The consequence, though, runs against the recommendation. If item 11 does not reach
`$CONFIG` and item 9 is satisfied by the guard **both** N-E and N-Q owe, then N-Q's
five containment lines are required by no rule in this tree, and the matrix says so
itself in its weakest-cell paragraph. Once F2 removes rung (a), nothing is left
holding them up.

---

# 3. VERDICT ON THE RUNG-(a) "ONE SCHEMA HOME" READING

**IT DOES NOT HOLD. RUNG (a) DOES NOT FIRE.**

Three independent grounds, each sufficient (F2):

1. The phrase is coined in the document that selects on it — five occurrences in this
   repository, all in the subject.
2. Hard rule 1's fourth clause governs **defaults**, and MEASURED, none of the three
   rows has a default. The round-3 red team already stated this as the reason the
   clause could not kill N-M; round 4 renames the clause and uses it to select.
3. Even on the matrix's re-reading, a directory is not a schema. MEASURED, `configs/`
   holds engine instrument configs, an engine play config, six arena configs and a
   weights table, and the engine exits 2 on two of the three I sampled.

The matrix's own weakest-cell paragraph is therefore triggered verbatim: *"then rung
(a) does not discriminate, the ladder falls through to rung (b), and N-E wins on
measured lines."*

**But the fall-through is not licensed either**, because F3 shows rung (b) is as
unregistered as rung (a). The honest reading of this round is that it has produced
**no rule-1 discrimination on axis A**, and the tie D-324 recorded stands with N-Q
added to it — unless the architect selects on a ground stated outside the ladder. On
the measured record as it now stands, the ground that does that is cost: N-E is
cheapest on every count the round took (22/7 against 32/12), owes the same guard, and
is the only row of the three against which I could not reproduce a defect that the
other rows do not have.

---

# 4. THE STRONGEST ATTACK SURVIVING AGAINST N-Q

> **N-Q's selector is a phrase this repository does not contain.** Hard rule 1's
> fourth clause reads *"NO code-side default for any tunable — a default lives in
> exactly one schema place"*; it is a rule about DEFAULTS, and MEASURED, none of N-E,
> N-M or N-Q has one — all three refuse an absent `--config` by name at exit 1.
> *"One schema home"* occurs nowhere in CLAUDE.md, nowhere in `docs/decisions.md`, and
> nowhere in the three prior M4 revisions or their three attacks; its only occurrences
> in this tree are the five inside the matrix that selects on it, and the round-3 red
> team had already ruled the same clause *"a WOUND, not a KILL"* precisely because
> rule 1 names defaults and these rows have none. The re-reading is then falsified by
> the directory it names: MEASURED, `configs/` holds four engine configs, six arena
> match configs and a weights table, and the engine exits 2 on two of the three
> documents I sampled from it — so what N-Q enforces is directory membership, not
> schema membership, and the one schema place for a pistol config is the
> `deny_unknown_fields` `Config` type all three rows share and none of them touches.
> Nor does containment deliver the provenance the second ground claims for it:
> MEASURED, a gitignored `configs/ghost_v0.bin` is admitted at **exit 0** under the
> COMPLETE kind token, is invisible to `git status --porcelain`, and its name reaches
> the invariant block beside `revision b067d47…`, a revision at which no commit
> contains it — so *"the record names a committed, re-runnable document BY
> CONSTRUCTION"* is false for N-Q exactly as it is for N-E. Rung (a) therefore does not
> fire; the ladder that would carry the fall-through was itself coined in the document
> that applies it, and stopping it at (a) rather than (b) is the whole of what picks
> N-Q over the row the matrix concedes wins the next rung down. What the five extra
> containment lines actually buy is a refusal class the field did not have: MEASURED,
> invoked through a symlinked checkout path — `ROOT` is bash's logical `pwd`,
> `realpath -m` is physical — N-Q refuses the repository's own
> `configs/instrument_v0.toml` with *"resolves to …, which is not under
> …/configs/"*, on an invocation N-E completes at exit 0.

---

# 5. RECOMMENDATION — **NOT N-Q. N-E.**

Stated plainly, as dispatched.

**The recommendation should be N-E**, with the four whole-path guard lines FACT 5's
small half correctly re-costs, and with the item-10 driving test F7 says every row
owes and no row was costed for.

Why, in the order the evidence supports:

1. **N-Q's selector fails** (F2). With rung (a) gone, the matrix's own text hands the
   round to N-E.
2. **N-Q's supporting ground fails on its own measurement** (F4). Containment does not
   deliver "a committed document by construction"; it delivers "a document under
   `configs/`", which is a weaker property than the ground needs and than the field
   table claims.
3. **N-Q's extra lines buy a property no rule in this tree requires** — the matrix's
   own weakest-cell paragraph concedes this, and F13 confirms both halves of it
   first-hand: item 11 does not reach a read binding, and item 9 is discharged by the
   guard N-E owes anyway.
4. **N-Q's extra lines cost a defect N-E does not have** (F5, F6): a false refusal of
   the repository's own registered config through a symlinked path, and a
   resolution base that contradicts the one item 11 made this script adopt for `--out`.
5. **N-Q is the most expensive row on every count taken** (FACT 2, reproduced), and
   the gap widens once item 10 is costed (F7).
6. **N-M falls, but not for the reason the matrix gives** (F9, F10). Its rule-1
   objection does not survive first-hand reading and is an escalation of an admitted
   judgement. What does kill it is measured and unstated: `wp15b_sprt_prereg.md`
   §7A.2 registers `--config configs/gate_v0.toml`, and N-M refuses it at exit 1.

**Two things N-E must ship that this round did not cost for anybody:** the item-10
driving test for both new guard arms, in two halves with a control (b067d47's
precedent is 91 test lines for one arm), and an item-12 sentence in the usage block
saying what a config refusal is — a FAIL, since this script declares no void class.

**If the architect selects N-Q anyway**, F5 must be fixed in the same commit — `ROOT`
canonicalised before the prefix comparison — because as measured, N-Q refuses the
registered config on a symlinked checkout path and the message blames the config.

---

# 6. `tools/SHELL_CHECKLIST.md` — ITEMS ANSWERED BY NAME

D-324's stop record made this a condition of the fourth round and the round discharged
two of twelve. Answered here for the three rows.

**Item 1 — a command substitution whose status is DISCARDED.** N-E, N-M: none added.
**N-Q adds one**, `CONFIG_RESOLVED="$(realpath -m -- "$CONFIG")"`. It is the assigning
form, which does propagate under `set -e`, so it is not item 1's primary defect — but
it is item 1's second paragraph: a bare `set -e` death prints nothing, and this one
would die without a named refusal. I could not make it die (see §7, rejected), and
`realpath` is already a dependency at line 379, so this is a **widened exposure, not a
new one** — N-Q runs it on an arbitrary caller string where line 379 runs it on a path
`command -v` has already resolved.

**Item 2 — a pipeline in a `then` body.** No row adds a pipeline. Clean for all three.

**Item 3 — `grep` under `pipefail`.** No row adds a `grep`. Clean for all three.

**Item 4 — `LC_ALL`, and which direction it moves a guard.** N-E and N-Q both copy the
existing arm as an ALLOW-LIST, `*[![:print:]]*`, which is the direction item 4
requires: the `LC_ALL=C` pin makes the refusal as WIDE as possible. Verified by
measurement — FACT 4(k) reproduced, TAB and LF both refused. **Correct in both rows.**
N-M reaches no character class. Clean for all three.

**Item 5 — the index is what commits.** No row reads `git ls-files`. Not engaged —
**but F4 is item 5's concern arriving through a different door**: N-Q's cost cell sells
a property about what is *committed* while its predicate tests only what is on the
*filesystem*, and the two differ exactly as item 5 says a path and a blob differ.

**Item 6 — a sweep by prefix must own the prefix.** No row deletes anything. Clean.
N-Q's containment predicate `"$ROOT/configs/"?*` is prefix-matching in the *safe*
direction and does not have item 6's bug — FACT 4(j) reproduced,
`configs_evil/…` refused.

**Item 7 — traps.** No row touches the EXIT trap. Clean for all three.

**Item 8 — one spelling per number, one refusal per reason.** N-E and N-Q: each new
refusal names one reason and the guard is separate from the containment check, which
is right. **N-Q fails the diagnosis half in one case**: F5's message says the config
"is not under" a directory it is under. Three reasons — escaped the root, named a
directory, reached through a symlinked invocation — share one refusal, and the third
gets the first's wording. N-M: one refusal, correctly named, quoting the token back.

**Item 9 — what reaches a record is caller-controlled.** *This is the governing item
for `$CONFIG` and the matrix is right that it is.* N-E and N-Q both discharge it, with
the same four lines, on the whole path the record actually writes and not on a
basename — FACT 5 and FACT 6 are exactly item 9's argument and both survive.
N-M discharges it by construction: no caller byte reaches the record (FACT 7,
reproduced). **All three pass item 9, and item 9 does not separate them.**

**Item 10 — THE COVERAGE RULE.** **All three rows fail it as costed.** FACT 9 buys
retrofit lines and no driving test for any new refusal. The rule is not satisfied by
30 pre-existing tests continuing to pass; it wants a test of the new number-affecting
behaviour, with a control. b067d47 paid 91 test lines for one arm. The debt is
asymmetric — N-M 1 refusal, N-E 2, **N-Q 5 plus an unpinned normalisation** — and this
is F7, the largest omission in the round's cost table.

**Item 11 — a caller's path that feeds a DELETE or an OVERWRITE.** **Round 4's reading
is correct and I confirm it independently.** Item 11's scope is *"consumed by `rm`,
`mv`, or a write"*, and its sweep paragraph says *"enumerate every DESTRUCTIVE site"*.
`$CONFIG` feeds `[ -f ]`, `digest`, three engine `--config` arguments and one `echo`
of its value. **No row's `$CONFIG` is an item-11 binding, N-Q's included.** The
round-3 attack's citation is an over-claim and round 4 is right to refuse to inherit
it. And the converse is F6's sharper half: this script's one real item-11 binding is
`--out`, a caller-supplied path consumed by a write, and it is deliberately resolved
and **not contained** — E5 wrote `probe_out.txt` outside the repository at exit 0, by
design. N-Q would contain a read binding more tightly than this script contains its
write binding.

**Item 12 — a gate distinguishes RUN VOID from FAIL, by name.** *Named nowhere in the
subject, though the stop record singles it out. Answered here.*
- *Obligation 1, a code per kind.* `b067d47` states in the usage block that this
  script **has no void class**: 0 wrote a record, 1 did not. All three rows respect
  it — every new refusal exits 1, verified for all three (FACT 3 and every case of
  FACT 4 reproduced above).
- *Obligation 2, preflight and void early.* Not engaged by N-M. Engaged by N-Q
  through `realpath`, which is not preflighted — but neither is line 379's existing
  `realpath`, so this is pre-existing and not a row cost (§7, rejected).
- *Obligation 3, the distinction survives the seam.* The retrofitted tests assert
  `success()`, not a code. That is pre-existing and identical across rows.
- **The row-discriminating answer is F5.** N-Q creates a case where the script *could
  not take the answer* — the checkout was reached through a symlink — and reports it
  as *the answer is no, your config is not under `configs/`*. A script with no void
  class cannot signal the difference by code, so item 12's burden falls entirely on
  the message, and N-Q's message reports the void as a fail. **N-Q is the only row of
  the three that introduces a new instance of item 12's defect.**

---

# 7. RECORDED AS REJECTED, with the attempted reproducer

Per CLAUDE.md, a finding that cannot be reproduced is recorded as rejected with the
attempt.

- **"`realpath` is a new unpreflighted dependency N-Q adds."** REJECTED.
  `grep -n 'realpath' tools/baseline_snapshot.sh` at `b067d47` returns line 379,
  `BINARY="$(realpath -- "$BINARY")"`, and line 151's comment. The dependency
  pre-exists N-Q. Only the exposure widens; recorded under item 1 rather than as a
  finding.
- **"A `realpath -m` failure kills N-Q with no named refusal."** REJECTED — could not
  make it fail. Symlink loop: `ln -sfn loopB.toml configs/loopA.toml; ln -sfn
  loopA.toml configs/loopB.toml; realpath -m -- configs/loopA.toml` returns
  `/…/configs/loopA.toml` at **rc=0**, and the script then refuses at
  `no config at configs/loopA.toml`, exit 1, named. Over-long component (300 chars):
  `realpath -m` returns it at **rc=0**; script refuses `no config at …`, exit 1, named.
- **"TOCTOU between the containment check and the digest."** REJECTED as
  unreproduced. The window is real — `realpath` resolves once, then `[ -f ]`,
  `digest` and three engine invocations each re-open the root-relative path — but I
  built no minimal reproducer, and a race requiring write access to `configs/` is
  weaker than F4, which needs no race at all.
- **"Unicode lookalike path components escape containment."** REJECTED as
  non-discriminating. Under the `LC_ALL=C` pin, `*[![:print:]]*` refuses every
  non-ASCII byte, in N-E and N-Q alike; a lookalike outside `configs/` dies at
  containment first. Item 4 is satisfied in the wide direction for both rows.
- **"A repeated `--config` silently last-wins."** REJECTED as non-discriminating.
  Reproduced (`--config /etc/hostname --config configs/instrument_v0.toml` → exit 0,
  `config configs/instrument_v0.toml`), but it is the behaviour of every flag in this
  script and identical across all three rows.
- **"Containment is defeated by a hardlink."** Reproduced (a hardlink in `configs/`
  passes containment, `[ -f ]` and `digest`, and dies at the engine with a message
  about the engine) but **folded into F4** rather than rated separately: F4's
  gitignored-file case is the same hole reaching exit 0, which is strictly stronger.

---

# 8. CLOSING STATE

```
$ git status --porcelain
                        ← empty
$ git rev-parse HEAD
84ff8d72a2bc4e8c36e9aa95a90efc2444cf8091
```

No repository file was edited. No git write command was run against the repository.
The measurement worktree at `/home/tom/.cache/redteam-m4-axisA` was created at
`b067d47`, used, and removed with `git worktree remove --force` + `git worktree prune`.
`/home/tom/.cache/review-trackC` was not touched.
`/home/tom/.cache/m4_matrix_measurement/` was read and not modified.

*Fresh-context DECISION-RED-TEAM of `docs/experiments/matrix_M4_axisA_round4.md` at
`7866bcf`. Scope: N-Q and its interaction with {N-E, N-M}. Ten MEASURED facts
re-run; nine reproduce, one does not. Thirteen findings. Rung (a) does not hold.
**Recommends N-E, not N-Q.***
