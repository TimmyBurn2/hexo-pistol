# REVIEW-impl — R19 (`tools/baseline_snapshot.sh` path-resolution base)

**Reviewer:** fresh-context REVIEW-impl, dispatched against a named revision, not the implementer.

**Named revision under review:** `63eac4c2d30e51958bb673d17ce95c66fd208551` (both files: `tools/baseline_snapshot.sh`, `crates/pistol-cli/tests/baseline_snapshot_tests.rs`).

**Revision match at the end of this review:** repository HEAD is `7dfd047b365576379fcc612c53eedd83f0a548d5`, one commit ahead of `63eac4c` (`7dfd047` is `docs(experiments): U2 reaches u-rev 4 …`, unrelated docs-only work). `63eac4c` **is an ancestor of HEAD**, and `git diff 63eac4c HEAD -- tools/baseline_snapshot.sh crates/pistol-cli/tests/baseline_snapshot_tests.rs` is **empty** — the two subject files are byte-identical at HEAD and at the named revision. `git status --porcelain` at the end of this review shows exactly one modified file, `docs/experiments/U3_tier_t.md`, which predates this review (I made no writes into the live tree; all reproducers ran from a scratch directory under `/home/tom/scratch_r19review/`, and mutation testing ran in a separate worktree that has been removed).

---

## 1. The rule the script now claims, vs. what the code does

`usage()` states: *"Every path YOU pass on the command line -- --out, --corpus, --binary -- is resolved against THE DIRECTORY YOU RAN THIS SCRIPT FROM"*; defaults are root-relative; a slash-less `--binary` is a bare PATH name; and where a relative `--corpus`/`--binary` disagrees between the two bases, the script refuses and names both paths — `--out` is explicitly **not** part of that ambiguity clause.

Reading `caller_path()` against that text:
- `/*` values return immediately as-is (absolute — no claim to check).
- `kind = exec` with no `/` returns the bare value untouched (PATH-resolved later by `command -v`) — matches the bare-name clause exactly, and **only** for `exec`.
- Otherwise `here=$CALLER_PWD/$value`, `there=$ROOT/$value`; the ambiguity/root-only refusal fires only when `$CALLER_PWD != $ROOT` **and** `kind != write` — which is exactly "every path resolves against the caller, and `--corpus`/`--binary` additionally refuse a disagreement; `--out` does not."

Verified live (`/home/tom/scratch_r19review/work`, real binary, real fixture):
- `--corpus <path only under ROOT>` from a non-root cwd → **exit 1**, `... where nothing of that name exists. It DOES exist at <ROOT path>...` — matches usage exactly.
- `--corpus ..` (both readings exist, differ) → **exit 1**, `is AMBIGUOUS: ... names one file at <here> and a DIFFERENT file at <there>`.
- `--out relative_probe.txt` from a non-root cwd → written into the **caller's** directory; nothing appears at `$REPO/relative_probe.txt`; this is the original R19-adjacent bug (measured pre-fix, cited in the commit body) and it is fixed.
- `--binary ./stub-engine` (slash present) → resolved against the caller's directory and actually exec'd (handshake failure came from the stub script, proving it ran the caller's file, not a PATH lookup).
- `--binary stub-engine` (no slash, file present in CWD) → **not** found — `no engine at stub-engine`, proving it is genuinely PATH-resolved and CWD is not silently consulted, exactly as the bare-name clause says.

No divergence found between `usage()`'s claim and `caller_path()`'s behavior for any of these shapes. **Verdict on item 1 of the attack brief: the code does what the usage text says.**

## 2. The bare-name exemption, and other value shapes

- `..`: goes through the normal read/exec branch (contains no `/`? — it has none, but `..` is not `exec`-exempted since the exemption only fires for `kind=exec`; for `--corpus ..` both readings exist as directories and differ → refused AMBIGUOUS (reproduced above). Correct — the usage text's "you get a named refusal" holds.
- Trailing slash (`--corpus subdir/`, directory exists only under caller, not under root): `caller_path` resolves it silently as `$CALLER_PWD/subdir/`, no ambiguity fires (root side doesn't exist), then the pre-existing `[ -f "$CORPUS" ]` downstream check refuses it as "no corpus at .../subdir/" (a directory is not a file). No divergence from the resolution claim — the *base* was still exactly what the usage text predicts.
- Symlink whose target is the ROOT-side file (`--corpus` value pointing, via `here`, at the same inode as `there`): `[ ! "$here" -ef "$there" ]` is false, so **no** ambiguity fires — the run proceeds and actually completes (verified: real `record.txt`-shaped output on stdout, 85 lines, a full `baseline_snapshot 1` record). Correct: same bytes, no disagreement to refuse.
- `./x` (explicit `./` prefix, contains `/`): goes through the *same* path as any other relative value (not exempted), including for `--binary` — confirmed above with `./stub-engine`. Matches usage (only a **slash-less** `--binary` value is a bare name).
- Path containing a space (`--corpus "spaced dir/corpus.txt"`, existing only under caller): resolved and consumed successfully (exit 0) — no divergence; the later basename-printable-ASCII/space guard (unchanged by this commit, downstream of `caller_path`) governs whether the *name* is refused for the *record*, which is an orthogonal, pre-existing rule this commit does not touch.
- Broken symlink as the value (`--corpus brokenlink.txt` pointing at a nonexistent target): `[ -e "$here" ]` is false (correctly, since `-e` follows the link and the target is absent) → falls through to the ordinary `[ -f "$CORPUS" ]` refusal ("no corpus at ..."). No divergence.

**REJECTED lead:** I looked for a value shape where `--out`, `--corpus` or `--binary` resolves somewhere the usage text does not predict. None of the eight shapes tried (`..`, trailing slash, symlink-to-same-file, `./x`, space, broken symlink, bare name for `--corpus` [not exec-exempted, confirmed by code inspection], absolute value) produced a resolution outside what `usage()` states. I record this as a reproducer set with no positive finding, per the instruction to record rejected leads.

**Finding 1 (MINOR) — permission-masked existence is folded into "does not exist."**
`[ -e "$here" ]`/`[ -e "$there" ]` cannot distinguish "genuinely absent" from "present but unstatable because a parent directory denies traversal" (`chmod 000` on the parent). Reproduced:

```
$ mkdir -p crates/pistol-cli/tests/fixtures
$ cp "$REPO/crates/pistol-cli/tests/fixtures/bench_positions_v1.txt" crates/pistol-cli/tests/fixtures/bench_positions_v1.txt   # byte-identical to the ROOT copy
$ chmod 000 crates/pistol-cli/tests/fixtures
$ bash "$REPO/tools/baseline_snapshot.sh" --corpus crates/pistol-cli/tests/fixtures/bench_positions_v1.txt --binary "$REPO/target/release/pistol" --ladder-depth 1
baseline_snapshot: FAIL: --corpus `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt` is a relative path and this script resolves yours against the directory you ran it from, which is /home/tom/scratch_r19review/work, where nothing of that name exists. It DOES exist at /home/tom/Projects/HeXO-AlphaBeta/crates/pistol-cli/tests/fixtures/bench_positions_v1.txt. ...
```

The claim *"where nothing of that name exists"* is **false** — the file exists, with the same bytes as the ROOT copy, in a directory the invoking user made unreadable. The script still fails safe (exit 1, no silent misread — it does not choose either reading), so this is a wrong *diagnosis*, not a soundness break, and it does not violate the ambiguity contract itself. Filed under SHELL_CHECKLIST item 8 ("one refusal per reason": permission-denied and absent are two different reasons folded into one message). Severity **MINOR**: it fails loud, just not for the reason it states.

## 3. `set -euo pipefail` behaviour in `caller_path`

No pipelines, no command substitutions with discarded status inside `caller_path` or `usage()`. `fail` is called as a plain statement (not inside `$(...)`), so `set -e`'s subshell-swallows-exit trap (checklist item 1) does not apply here — consistent with the header comment's own citation of that reason. `[ -e ... ]` and `[ ... -ef ... ]` as bare `if` conditions are exempt from `-e` by ordinary bash semantics and are not pipelines, so item 2 does not engage either. I tried to force a nonzero-but-silent path through `-ef` on a nonexistent/broken-symlink target (item 3's spirit): `-ef` is only ever evaluated after both operands have already passed `-e`, so it is never invoked on a path proven not to exist — no EXIT-0-WRONG-ANSWER found here. **REJECTED lead**: no reproducer for a `caller_path` path that returns nonzero while the caller does not die, or dies with a materially wrong message (Finding 1 is a wrong *reason*, not a wrong *exit code* or a swallowed failure).

## 4. Every caller of `tools/baseline_snapshot.sh` in the tree

Enumerated via `grep -rn "baseline_snapshot\.sh"` over the whole tree (excluding `.git`, `target`):

| caller | invokes it with args? | affected by the base change? |
|---|---|---|
| `crates/pistol-cli/tests/baseline_snapshot_tests.rs` | yes, extensively | this is the suite under review; passes (§5) |
| `tools/artifact_check.sh`, `tools/bench_delta.sh` | no — comment references only | not affected |
| `docs/decisions.md`, various `docs/experiments/*.md` | no — prose references / historical record | not affected |
| `docs/experiments/wp15b_sprt_prereg.md` §7A.2/§10 | registers `tools/baseline_snapshot.sh --config configs/gate_v0.toml` | `--config` **does not exist** in this script (it is option N-E, selected by D-329, explicitly not built — confirmed absent from the diff and from the current flag set). This invocation cannot be exercised regardless of R19; it is out of this commit's scope and D-329 tracks its own conditions separately. Not a regression introduced here. |
| `docs/experiments/wp15a_prereg.md` §0.1 `SNAPSHOT_REL=tools/baseline_snapshot.sh` | binds the **script's own path**, not a `--corpus`/`--out`/`--binary` value; the doc's own table says it is "resolved against the pristine clone" and used only to *locate* the script inside a clone, not as a workload-scope flag value | the harness that would have driven it, `tools/wp15a_h1.sh` (and its test `crates/pistol-cli/tests/wp15a_h1_tests.rs`), **no longer exists in the tree** — H1 was retired to an observation record at `407d662` (`docs(experiments): H1 is retired ...`, D-276/277/278). Confirmed by `git log -- tools/wp15a_h1.sh`: last commit touching it is the retirement commit; `ls tools/wp15a_h1.sh` → no such file. The invocation is **dead**, not live. |

**No live caller in the tree invokes `--corpus` or `--binary` with a relative path from a non-root working directory.** The only real exercise of the new code paths is the shipped test suite itself. This is worth naming as a residual: the fix is currently proven only by its own tests, not by any production caller re-confirming the new base under real conditions — consistent with D-230's own framing of this script as measurement infrastructure with a narrow, test-suite-bounded set of callers.

## 5. The tests

```
$ cargo test -p pistol-cli --test baseline_snapshot_tests
running 34 tests
...
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 43.61s
```

All four new tests are behavior-named (rule 7): `a_relative_out_and_a_relative_corpus_resolve_from_the_same_base`, `a_relative_input_that_exists_only_under_the_repository_root_is_refused_not_silently_read`, `a_relative_input_naming_two_different_files_is_refused_as_ambiguous`, `the_usage_text_states_the_resolution_base_and_the_exit_status_classes`.

**Mutation verification**, in a separate git worktree (`/home/tom/r19_mutation_wt`, `git worktree add --detach` at `63eac4c`; `CARGO_TARGET_DIR` left unexported; worktree removed with `git worktree remove --force` after use, confirmed gone from `git worktree list`):

- Mutant 1 — neutered the "exists only under root" refusal (`fail ...` → `here="$there"`, i.e. silently prefer the root copy): `a_relative_input_that_exists_only_under_the_repository_root_is_refused_not_silently_read` **went RED** (`test result: FAILED. 0 passed; 1 failed`).
- Mutant 2 — neutered the ambiguity refusal (`if [ -e "$there" ] && ...` → `if false && ...`): `a_relative_input_naming_two_different_files_is_refused_as_ambiguous` **went RED** (`panicked ... "and the refusal says which defect it is: baseline_snapshot: FAIL: the corpus states no positions: /tmp/.../snapshot-ambiguous-input-0/crates/.../bench_positions_v1.txt"` — proving it silently walked into reading the caller's twin file and failed for an unrelated downstream reason instead of refusing the ambiguity).

Both refusal tests are confirmed non-vacuous by mutation.

### Item 10, the coverage rule, answered directly

The **control run is `a_relative_out_and_a_relative_corpus_resolve_from_the_same_base`** — it is explicitly commented in the test file as the control ("This is the control run as well as the claim: it SUCCEEDS, so a guard that refused everything could not produce it"), it drives the shipped script (`Command::new("bash").arg(repo("tools/baseline_snapshot.sh"))`), runs in a scratch directory (`scratch("snapshot-one-base")`), and asserts a full successful record with the caller's own digests. **Discharged.**

**Finding 2 (MINOR) — coverage gap: the shared refusal branches in `caller_path` are proven only through `--corpus`, never through `--binary`.** Both new refusal tests (`root-only` and `ambiguous`) pass an **absolute** path for `--binary` specifically to isolate the `--corpus` behavior, and no new test drives a *relative, slash-containing* `--binary` value through the root-only or ambiguous branches. Because `caller_path` is one shared function and the branch structure is identical for `read` and `exec` kinds (differing only in the bare-name shortcut, which *is* tested — `a_binary_named_without_a_slash_is_digested_as_the_file_that_will_run`), this is a low-risk gap: a regression that broke the shared branches would still be caught via `--corpus`. But if a future change special-cased `exec` inside the shared `if` block (as the bare-name shortcut already does), a `--binary`-specific defect in the root-only/ambiguous branches could land untested. Severity **MINOR**.

## 6. Anything the change should have done and did not

**Finding 3 (MINOR) — a stale comment pointer left by the change itself.** `tools/baseline_snapshot.sh:170-172` (unchanged text, sitting just above `CALLER_PWD="$PWD"`):

```
# The CALLER'S directory, captured BEFORE the `cd` below, because the `cd` is what
# makes a relative `--out` mean something the caller did not ask for. See the
# resolution after the argument loop.
```

Before this commit, `--out`'s resolution genuinely *was* "after the argument loop" — a `case "$OUT" in '' | /*) ;; *) OUT="$CALLER_PWD/$OUT" ;; esac` block that sat below the `while` loop (visible in the removed side of the diff). This commit deletes that block and moves `--out`'s resolution **inline**, into the loop itself, via `caller_path --out "$ARG" write`. The comment was not updated and now points at a location that no longer contains a resolution step — a reader following "see the resolution after the argument loop" finds the count-spelling validation code instead. This is exactly the class of dangling pointer this project's own D-331 line exists to catch (a claim whose referent moved and which nothing re-reads), though the stakes here are a source comment rather than a load-bearing design-document claim, so I file it as **MINOR** rather than BLOCKING/MAJOR. It does not affect behavior, only a maintainer's ability to trust the comment.

No other should-have-done gap found: `--config` is correctly left unbuilt (D-329's own scope, not R19's), the ambiguity refusal is correctly restricted to `read`/`exec` and correctly excludes `write` (there is nothing to disambiguate about a target you are about to create), and the usage text's exit-status section correctly answers checklist item 12 by name.

---

## SHELL_CHECKLIST.md — all twelve items, answered by name

1. **Discarded command-substitution status** — NOT ENGAGED. `caller_path`/`usage()` introduce no new command substitutions; the pre-existing `digest()`/`argument()`/`score_checked()` idiom this rule is about is untouched by this diff.
2. **Pipeline in a `then` body vs. in a condition** — NOT ENGAGED. No pipelines added.
3. **`grep` under `pipefail`** — NOT ENGAGED. No new `grep` calls.
4. **`LC_ALL` and which direction it moves a guard** — NOT ENGAGED. `export LC_ALL=C` predates this diff and is untouched; nothing new in this diff builds a character-class guard.
5. **Index vs. working tree** — NOT ENGAGED. No `git ls-files`/tracked-content reads in this diff.
6. **A sweep by prefix must own the prefix** — NOT ENGAGED. No deletion sweep in this diff; the new tests use the existing `scratch()` helper (pre-existing prefix convention), not a new one.
7. **Traps** — NOT ENGAGED. No traps added or touched.
8. **One spelling per number, one refusal per reason** — ENGAGED, **PARTIALLY DISCHARGED**. The two new refusal messages *are* one reason each (root-only vs. ambiguous) and are textually distinct, which is the primary intent. Not fully discharged: Finding 1 shows `-e`'s inability to distinguish "absent" from "present-but-permission-masked" folds two different reasons into the "root-only" message (MINOR).
9. **What reaches a record is caller-controlled** — NOT ENGAGED by this diff. The values interpolated into the new `fail` messages go to stderr, not into the invariant block/record (the process exits before writing anything); the caller-controlled values that *do* reach the record — the `CORPUS`/`OPENINGS` basenames — are guarded by the pre-existing, unmodified control-character/space loop further down the script, outside this diff's hunks.
10. **THE COVERAGE RULE** — ENGAGED, **DISCHARGED**, with the named control run (§5) and mutation confirmation of both refusal branches (§5). Finding 2 (MINOR) notes a narrower gap: the shared branches are proven through `--corpus` only, not independently through `--binary`.
11. **Containment guard on a delete/overwrite** — ENGAGED for `--out` (the only write site touched here), **DISCHARGED**: `--out`'s target is exactly the caller's own CALLER_PWD-relative (or absolute) path — there is no "root" `--out` is meant to be contained under (its whole job, and the bug R19 partly addresses, is writing wherever the caller says), and this is verified live: `--out relative_probe.txt` from `/home/tom/scratch_r19review/work` landed at `.../work/relative_probe.txt` and nothing appeared at `$REPO/relative_probe.txt` (git status on the live repo stayed clean of it). No `rm`/`mv` site is touched by this diff.
12. **RUN VOID vs. FAIL, by name** — ENGAGED, **DISCHARGED**. The new `usage()` text's EXIT STATUS section states explicitly, by name, that this script "declares no VOID class (tools/SHELL_CHECKLIST.md item 12)" and that every non-write outcome is `FAIL` at exit 1 — matching obligation 1's requirement that a gate with no void class say so rather than leave it inferred. (This restates, in the new `--help` surface, a claim the pre-existing top-of-file comment block already made outside this diff's hunks; the new copy is consistent with the old one, not a divergence.)

---

## Gates (own log output cited, not a wrapper's exit status)

```
$ cargo test -p pistol-cli --test baseline_snapshot_tests
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 43.61s

$ cargo clippy --workspace --all-targets -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.21s
(no warnings/errors emitted)

$ cargo fmt --all --check
(no output; exit 0)

$ ./tools/file_justification_check.sh
file_justification_check: self-test passed on 8 seeded cases (cap 300)
...
file_justification_check: over the cap and justified: tools/baseline_snapshot.sh
...
file_justification_check: 257 tracked .rs/.sh files, 24 over the cap, all justified
```

All four gates pass.

---

## Verdict

**PASS.**

No BLOCKING or MAJOR finding. Three MINOR findings, none of which affects correctness, safety, or the ambiguity-refusal contract R19 requires:

1. **MINOR** — `[ -e ]` folds "genuinely absent" and "present but permission-masked" into the same "root-only" refusal message, so the stated reason can be false even though the script still fails safe (item 8).
2. **MINOR** — the shared `caller_path` refusal branches are proven by mutation only through `--corpus`; no test drives a relative, slash-containing `--binary` value through the same root-only/ambiguous branches independently (item 10's spirit, narrowly).
3. **MINOR** — a stale comment (`tools/baseline_snapshot.sh:172`, "See the resolution after the argument loop") still points at a resolution step this very commit deleted from that location; the resolution is now inline in the argument loop, not after it.

The implementer's reading of R19 — applying "one base, refuse the disagreement" to the live `--out` vs. `--corpus`/`--binary` asymmetry rather than to the unbuilt `--config` flag — answers the ruling's actual intent (one documented, consistent resolution base for every caller-supplied path) against the codebase's actual current state, and is not a different question from the one R19 asked. The usage text is accurate against the code for every value shape tried, the driving test (`a_relative_out_and_a_relative_corpus_resolve_from_the_same_base`) is the required control and a genuine one, both refusal tests are confirmed non-vacuous by mutation in a separate worktree, and no live caller in the tree is broken by the base change (`wp15b_sprt_prereg.md`'s `--config` invocation cannot be exercised regardless, since `--config` is out of scope by D-329; `wp15a_prereg.md`'s `SNAPSHOT_REL` binding is dead — its harness was retired at `407d662`).
