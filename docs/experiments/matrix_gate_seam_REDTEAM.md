# DECISION-RED-TEAM — `docs/experiments/matrix_gate_seam.md`

## Header

- **Revision attacked**: the matrix is UNTRACKED at `21f8833`. The subject it
  measures is the uncommitted working tree, pinned for this attack as
  `git stash create` = **`9e1f4ea`** (`9e1f4eabe4aa7cb68e7cfe508a51a585f4cb3567`).
  `git rev-parse HEAD` = `21f8833`. **`9e1f4ea` does NOT match HEAD**: the tree
  carries uncommitted changes to 29 files, including BOTH gates under decision.
- **What I ran**: `git worktree add --detach /home/tom/pistol-wt/redteam 9e1f4ea`,
  four seeded-violation experiments against the shipped gates inside it, one
  timing measurement with the committed release binary, and re-derivations of
  P1-P4 by commands of my own choosing (`git show HEAD:…`, `/usr/bin/grep -n`,
  `git grep -ln`, `wc -l`, `git merge-base --is-ancestor`). The worktree is
  removed; every reproducer below is inlined verbatim because it produced no
  artifact worth keeping.

---

## ATTACK 1 — §2's "MEASURED at `21f8833`" is false, and false at exactly the two files under decision

P1 cites `tools/determinism.sh:69-86` for `SEATS`/`BUDGETS`, `run_seat` at
`:199-206`, and `tools/movetime_check.sh:38-39`. Re-derived:

```
$ git show HEAD:tools/determinism.sh | grep -n 'SEATS=(\|^)\|BUDGETS=(\|run_seat()'
68:SEATS=(   82:)   86:BUDGETS=(...)   193:run_seat() {
$ /usr/bin/grep -n 'SEATS=(\|BUDGETS=(\|run_seat()' tools/determinism.sh   # working tree
69:SEATS=(   87:BUDGETS=(...)   199:run_seat() {
$ git show HEAD:tools/movetime_check.sh | grep -n 'SEATS=(\|FIXTURE='  ->  37, 38
$ /usr/bin/grep -n 'SEATS=(\|^FIXTURE=' tools/movetime_check.sh          ->  38, 39
```

Every cited number is the WORKING TREE's, not `21f8833`'s. The offset is exactly
the uncommitted `require_tool.sh` change (`git diff HEAD -- tools/determinism.sh
tools/movetime_check.sh`: +1 blank line, `command -v cargo` replaced by a
five-line resolver block). CLAUDE.md's Process: *"Each review is dispatched
against a NAMED REVISION — a commit SHA, or a `git stash create` SHA where the
work is uncommitted."* The matrix names a commit SHA for measurements taken from
uncommitted work, on the two files whose interface is the decision.

**Verdict: LANDS.** Not fatal on its own; fatal to the claim that §2 is
re-derivable at the revision it names.

## ATTACK 2 — P4's timing figures are from a superseded revision of the very script they describe

P4: *"**CI time cost of every option: zero.** MEASURED, warm: `determinism.sh`
72.6 s, `movetime_check.sh` 22.5 s."* The cited file says otherwise about its
own provenance — `pre_phase2_sweep_CLOSURE.md`: *"The five gates' warm wall
costs at `b60c3d3`"*.

```
$ git merge-base --is-ancestor b60c3d3 21f8833 && echo ANCESTOR      -> ANCESTOR
$ git rev-list --count b60c3d3..21f8833                              -> 12
$ git diff --stat b60c3d3 21f8833 -- tools/determinism.sh
 tools/determinism.sh | 18 ++++++++++++++++--
```

Twelve commits and an eighteen-line change to the timed script separate the
measurement from the revision §2 says it was measured at. The artifact is also
gitignored (`git check-ignore -v` → `.gitignore:19:/artifacts/`), so the
citation cannot be re-derived from the repository at all.

**Verdict: LANDS** (minor). The 72.6/22.5 figures are not load-bearing for any
option's ranking; the mark is.

## ATTACK 3 — the premise the whole matrix rests on is FALSE, and I refuted it three ways

§1: *"Both gates root themselves at `dirname/..` and read committed configs and
fixtures; **neither can be pointed at anything a test controls**."* P1 repeats
it: *"neither reads a binary path from anywhere today."*

Rooting at `dirname/..` is not a constraint — it IS a root override, and PATH is
a second one. Three experiments, all against `9e1f4ea`, all driving the shipped
script with **zero edits to it**:

**Reproducer (shared).** Two files:

```bash
# $D/bin/fake_engine — speaks exactly what check_content demands
#!/usr/bin/env bash
set -euo pipefail
COUNTER="${FAKE_COUNTER:?}"
n=$(( $(cat "$COUNTER" 2>/dev/null || echo 0) + 1 )); echo "$n" >"$COUNTER"
mv="5,0"
if [ "${FAKE_MODE:-honest}" = violate ] && [ "$n" = 2 ]; then mv="6,0"; fi
while IFS= read -r line; do case "$line" in
  go*) printf 'info totals depth_turns 1 seldepth 1 nodes 1 hashfull 0 nps 1 time 0 score cp 0 pv %s\nbestmove %s\n' "$mv" "$mv" ;;
  quit) exit 0 ;; esac; done

# $D/bin/cargo — a shim FIRST on PATH, naming the stub on the artifact stream
#!/usr/bin/env bash
printf '{"reason":"compiler-artifact","executable":"'"$D"'/bin/fake_engine"}\n'; exit 0
```

**F-1 (reach).** `env PATH="$D/bin:$PATH" FAKE_MODE=honest bash tools/determinism.sh`
→ **exit 0**, and the WHOLE script ran including the recorded number:

```
determinism: seat staged-safety-net-cap: ok — 40 searches, 20 positions, no difference outside nps/time
determinism: ok — 5 seat(s), no difference outside nps/time in any of them
```

**F-2 (the seeded violation the gate is NAMED for).** Same command,
`FAKE_MODE=violate` → **exit 1**:

```
determinism: FAIL: radius: two processes disagreed on the same input:
--- /tmp/tmp.d2kNfkUuDh/radius.run.A   +++ /tmp/tmp.d2kNfkUuDh/radius.run.B
-bestmove 5,0   +bestmove 6,0
```

**F-3 (item 10's own control clause, verbatim).** A stub that answers nothing →
**exit 1**: `determinism: FAIL: radius: run A: 0 bestmove lines carrying a turn
token, for 40 searches`.

**F-4 (the second gate).** A shim cargo answering `cargo test` with 0 and naming
a stub that reports `time 99999` → `bash tools/movetime_check.sh` **exit 1**:

```
  stones  movetime_ms   elapsed_ms    verdict
      11          500        99999       OVER
movetime: FAIL: radius: movetime 500 on 11 stones took 99999 ms (bound 550 ms)
```

**G / H (the root, and the documents).** `tools/determinism.sh` copied into a
scratch tree holding `tools/{determinism,require_tool,scratch_preflight}.sh`,
`configs/` and `crates/pistol-cli/tests/fixtures/` and nothing else. `cmp -s`
says the copied script is **byte-identical**. With a fixture whose first `case`
line was commented out, the gate refuses:

```
determinism: FAIL: radius: extracted 20 positions from crates/pistol-cli/tests/fixtures/tactical_v0.txt but it states 19 cases
```

So a test controls the ROOT (by copying the script), the DOCUMENTS (by what it
puts under that root) and the BINARY (by PATH) — today, with no flag.

**Verdict: LANDS. This is the attack that kills the matrix**, because §1's
"WHY THIS IS A DECISION AND NOT AN IMPLEMENTATION" paragraph is the premise from
which the entire field of five options is derived.

## ATTACK 4 — the field is not the field: a sixth option exists, it is the house pattern, and it dominates A

**F — shadow the build tool on PATH; change nothing.** This is not my invention.
Four committed suites already do it:

```
$ git grep -ln 'env("PATH"' -- crates/*/tests | LC_ALL=C sort
crates/pistol-cli/tests/config_check_gate_tests.rs
crates/pistol-cli/tests/require_tool_gate_tests.rs
crates/pistol-cli/tests/solver_determinism_gate_tests.rs
crates/pistol-cli/tests/solver_link_check_tests.rs
```

`solver_determinism_gate_tests.rs` is **a compliant item-10 test suite for a
determinism gate**, in CI, that seeds its controls by shadowing `cargo`. Its own
comment states the reason the matrix would have needed:

> *"A stub named `cargo` FIRST on PATH, rather than a PATH with cargo removed:
> where the real one lives is machine-dependent … and a driver that depends on
> that answers differently on two machines. **Shadowing depends on nothing.**"*

Scored on the matrix's own axes:

| | A (`--engine`) | F (PATH shim) |
|---|---|---|
| shipped-code cost | ESTIMATED ~15 lines **per gate** | **0 lines** |
| reach | whole script | whole script (**MEASURED**, F-1/F-2/F-4) |
| seeds the named violation | yes | **yes (MEASURED, F-2 and F-4)** |
| failure mode | *"an operator pointing a gate at a binary that is not the subject"* — the matrix's own strongest objection to itself | **none: no committed seam. The seam is the test's own `env`** |
| CI green rests on | a flag not being passed | the real `cargo` being first on PATH — already load-bearing for every gate in `tools/ci.sh` |
| precedent | a flag added for a DIFFERENT reason (P3) | **a determinism gate's own compliant suite** |

F dominates A on every axis the matrix scores. **A matrix whose field omits the
dominating option is not the field**, and CLAUDE.md's Process makes adopting an
option out of such a field the same breach as silent architecture drift.

**Verdict: LANDS.**

## ATTACK 5 — A's flag would BYPASS the code D-250 exists to defend, so A shrinks what its own test can reach

`determinism.sh:137-171` and `movetime_check.sh:70-104` are the D-250 resolution
ladder, and their comment calls the alternative a **REPRODUCED** defect:
*"a hardcoded `target/release/pistol` then runs whatever STALE binary sits at
that path while the build goes elsewhere — a gate that passes for a binary
nobody built."*

A's `--engine` is, by construction, a branch that skips that ladder. Every
seeded test written under A therefore runs with the ladder switched off, and the
ladder — twelve refusals, `NAMED` vs `${#BUILT[@]}`, the `-e`/`-f`/`-x` triple —
stays exactly as untested as it is today.

F does the opposite: because the seam is the artifact stream, every seeded test
**drives the ladder**. The existing suite already banks that —
`a_green_build_that_names_no_executable_is_a_void_naming_that` shims
`{"executable":null}` and asserts the ladder's own refusal. That test is
impossible under A.

So A does not merely carry a residual risk; it **buys less coverage than F while
paying a permanent widening of the gate's trust surface**.

**Verdict: LANDS.**

## ATTACK 6 — P2's conclusion does not follow from P2's observation

P2 is half-right. Verified: `stub_engine.rs:180-184` does list nineteen
behaviours (I counted them), and none is literally "answer differently on the
second run". But P2's conclusion — *"so every option that uses the stub also
adds a behaviour"* — assumes determinism.sh has one comparison. It has two, and
the second is C-vs-D: *"a position answered differently on its own than in a
session with the others, so newgame does not clear everything it must."*

`StrayAfterNewGame(n)`, quoted from its own doc comment: *"the answer to the `go`
that follows its n-th `newgame` carries a SECOND `bestmove` line."* Run C sends
`position/go/quit` — **no `newgame`**. Run D sends `newgame` before every
position. So `stray_after_newgame 1` makes D deviate and C not, which is the
C-vs-D subject exactly. And the cross-crate reach P2 would need is already
established: `git grep -ln stub_engine` shows `pistol-cli`'s
`baseline_snapshot_tests`, `bench_block_tests` and `bench_delta_tests` driving it.

**Verdict: LANDS** on the conclusion; the narrow observation FAILS to be wrong.

## ATTACK 7 — C is rejected for a reason that covers only one of the two gates

§3 C: *"a seeded config and fixture cannot make two runs of the SAME engine
disagree — determinism is a property of the engine, not of the documents."* True,
and irrelevant to `movetime_check.sh`, whose subject is not "two runs agree" but
`elapsed <= budget + epsilon` — and **the gate reads `epsilon` out of the config
the seat names** (`movetime_check.sh:118`). That bound IS a property of a
document.

MEASURED, committed release binary, `configs/play_v0.toml`, spread fixture:

```
budget=1 elapsed=0 | budget=1 elapsed=0 | budget=1 elapsed=1 | budget=1 elapsed=3
budget=50 elapsed=49 (x4) | budget=500 elapsed=499 (x4)
```

At the committed `BUDGETS_MS` entry `1` with a seeded `movetime_epsilon_ms = 1`
the bound is 2 ms and the fourth position took 3 → `OVER`. So a seeded CONFIG
alone makes the shipped gate refuse. (`movetime_epsilon_ms = 0` does not work:
`crates/pistol-engine/src/validate.rs:185` refuses it, so the engine exits and
the gate says "the engine exited nonzero" — a different refusal.)

C should have been rejected for **flakiness** — a 3-vs-2 ms margin is not a
control any suite should stand on — and rejected **per gate**. It was instead
rejected once, on a determinism argument, for both. The matrix bundles two
decisions into one and scores them with one gate's reasoning.

**Verdict: LANDS.** The recommendation's rejection of C is unsound as written,
even though C remains the wrong choice.

## ATTACK 8 — D-291 mark discipline, and the measurement that would have found option F

- Option E: *"**Cost**: zero."* — **no mark at all**. D-291 asks every numeric
  claim be marked; zero is a number.
- Option D: *"**Cost**: ESTIMATED a new script plus its test"* — the mark tags a
  phrase with no number in it. A mark on a non-number is decoration.
- A/B/C's `~15` / `~10` / `~8` lines are estimates of a thing whose two nearest
  in-tree analogues are measurable in one command:

```
$ wc -l crates/pistol-cli/tests/solver_determinism_gate_tests.rs   ->  193
$ wc -l crates/pistol-cli/tests/config_check_gate_tests.rs         ->  233
```

That is the D-291 finding proper, and it is not pedantic: **the seconds-long
measurement the matrix skipped is the one that would have surfaced option F.**
Opening `solver_determinism_gate_tests.rs` to price the test half of any option
puts a compliant, shipped, PATH-shimmed item-10 suite for a determinism gate in
front of the author, and the matrix would not have been written.

**Verdict: LANDS.**

## ATTACK 9 — is item 10 binding at all? I attacked the line and it holds

The closure exempts `perft_check.sh` and `search_oracle_check.sh` as
*"sequencers … The verdict is cargo's."* I tried to put `determinism.sh` on that
side of the line: its verdict is `diff`'s, and `%d seat(s)` is `${#SEATS[@]}` —
the cardinality of a literal array. It does not hold. `check_content` computes
`errors`, `moves` and `depths` **in the script**, by shape-anchored `grep -c`,
and refuses on each; `run_seat` cross-checks `${#positions[@]}` against the
fixture's own `case` count. `movetime_check.sh` computes `worst` and prints the
overshoot table. Both produce numbers no `cargo test` produced.

The line is drawn in the same place for both, and correctly.

**Verdict: FAILS.** The matrix's premise 5 survives.

## ATTACK 10 — does E beat A? Yes on the axis the matrix names, and it still loses

I was asked to make this as hard as I can, so: A's green is conditional on a
negative — *no flag was passed* — and a conditional green degrades every reading
downstream of it, including closures that cite the gate's log. E's cost is a
recorded gap, which is honest and static. On the matrix's own scoring, "a gate
whose green means less, permanently" is a worse trade than "two numbers nothing
defends, recorded as such".

But this is a choice between two losing options. F closes the gap with **no**
green-weakening at all, so the E-vs-A trade never has to be made.

**Verdict: LANDS as an argument against A. It does not elect E.**

---

## Premises I could not verify

1. **P3's stated reason for `--keys-bin`.** The quotation is exact
   (`tools/book_v3_disjointness.sh:23-25`) and the flag is real
   (`:43,51,81-90`). Whether the *lock* was the true motive is a claim about
   history I have no evidence for beyond the comment itself; I take the comment
   at its word.
2. **"CI time cost of every option: zero."** I verified CI passes no flags
   (`tools/ci.sh:136` `gate "determinism" tools/determinism.sh`; `:180`
   `gate "movetime ceiling" tools/movetime_check.sh`). The *inference* that the
   cost is therefore zero is sound for A and F; for B it is not obviously so and
   I did not test it.
3. **Whether `stray_after_newgame` clears `check_content` before reaching the
   C-vs-D diff.** I read the emission site (`stub_engine.rs:578-600`) and reason
   it fires a `moves != expected_goes` refusal first. Either way the gate
   refuses on a seeded existing behaviour, which is all ATTACK 6 needs, but I did
   not run it.
4. **The ~15-line estimate for A.** I did not write A's patch. ATTACK 8 is about
   the mark and the missed measurement, not about the number being wrong.

---

## Verdict

**FALLS.**

The recommendation does not survive. The matrix must be REWRITTEN, because the
premise its whole field is derived from — *"neither can be pointed at anything a
test controls"* — is false, and was falsifiable in this repository by opening one
committed test file. The rewritten matrix should carry **option F — shadow
`cargo` on PATH, change neither gate** — and F should be selected: it has A's
full reach (MEASURED, F-2 and F-4), costs zero shipped lines, drives the D-250
resolution ladder that A's flag would bypass, and is already this project's
committed pattern for testing a determinism gate.

Options B and C need re-scoring in the rewrite rather than re-arguing: B's
`--root` is redundant because `dirname/..` already relocates with the script
(EXPERIMENT G/H), and C is available document-side without a flag for the same
reason — its real defect is that its `movetime` control would rest on a 3-vs-2
millisecond margin.
