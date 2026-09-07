# Pre-Phase-2 sweep — GROUP T, REVIEW-impl

## Header

**Revision under review:** `cb6e853`
**Matches HEAD:** YES — `git rev-parse HEAD` → `cb6e8532e163bcb41497993da3e9518c719cf448`.
Working tree carries two untracked documents (`docs/experiments/pre_phase2_sweep_CLOSURE.md`,
`docs/experiments/pre_phase2_sweep_C_REVIEW.md`) and nothing else.

**Commits in scope:** `86a7dc7` (T1), `b60c3d3` (T2), `42ab538` (T3), `1996d05` (T4),
`d93a5a4` (T5), `c105ad4`, `cb6e853`.

**Documents read:** `CLAUDE.md`, `docs/process.md`, `tools/SHELL_CHECKLIST.md`,
`docs/experiments/pre_phase2_sweep_CLOSURE.md` §1 P4–P5 and §2 groups H and T.

**What I ran.** All counts below were re-derived with commands I chose, printed with
their scope beside the number (`docs/process.md`, re-derivation clause). Mutation and
build work happened in a detached worktree at `/home/tom/pistol-wt/T-review`
(`git worktree add --detach cb6e853`), restored to clean and removed; `CARGO_TARGET_DIR`
was never exported around `cargo test`. Executed: `tools/file_justification_check.sh`,
`tools/config_check.sh`, `tools/artifact_check.sh`, `tools/decision_key_check.sh`,
`tools/label_consistency_check.sh`, `tools/governing_citation_check.sh`,
`tools/solver_determinism.sh`, `tools/determinism.sh` (stubbed), ten `pistol-cli` gate
suites (99 tests), `cargo clippy` in both profiles, and four deliberate mutations.

---

## Re-derivation of the load-bearing counts

Each row: my command, my scope, my number, and whether it agrees with the document.

| claim (CLOSURE / gate) | my command and scope | my number | verdict |
|---|---|---|---|
| "36 `command -v` sites in 20 files" at `b60c3d3` | `git grep -o -- "command -v" b60c3d3 -- 'tools/*.sh' 'tools/**/*.sh' \| wc -l`; files via `git grep -l` | **36 / 20** | AGREES |
| same, my wider scope | `git grep -o -- "command -v" b60c3d3 -- 'tools/'` (every tracked file under `tools/`, any extension) | 37 / 21 | the extra file is `tools/SHELL_CHECKLIST.md`, which the document explicitly excludes as prose. AGREES |
| same, whole repo | `git grep -l -- "command -v" b60c3d3 -- .` | 63 sites / 35 files | 15 of the files are `docs/`; 3 are `crates/…/tests`. Outside item 8's subject. No finding |
| "nine of thirteen `mktemp -d` scripts unpreflighted" at `b60c3d3` | `git grep -n -E "mktemp( \|$)" b60c3d3 -- tools crates`, then `git grep -l scratch_preflight b60c3d3 -- tools` | **13 scripts; 4 preflighted (`arena_smoke`, `bench_block`, `ci`, `staged_cover_bench`); 9 not** | AGREES |
| same, my wider scope: any `mktemp` (not just `-d`) | as above | one extra scratch writer: `tools/solver_oracle_check.sh:32` `OUT="$(mktemp)"` — a temp FILE, unpreflighted | see MINOR-7 |
| "12 tracked `.py` files over the cap, of 30" at `b60c3d3` | `git ls-tree -r --name-only b60c3d3 \| grep -E '\.py$'`, then `git cat-file blob \| wc -l` per file | **12 of 30** | AGREES |
| gate summary "406 tracked .rs/.sh/.py files, 84 over the cap, 84 entries" | `git ls-tree -r --name-only <rev>` + per-blob `wc -l`; registry via `grep -cE '^- \`[^\`]+\`: '` | at `b60c3d3`: **406 / 84 / 84**. **At `cb6e853`: 409 / 88 / 84** | AGREES at `b60c3d3`; **FALSE at HEAD** — see BLOCKING-1 |
| "ten `info totals` consumer sites in nine files" | every row read at HEAD; then `git grep -n " totals " -- 'tools/*.sh' 'tools/**/*.sh' 'tools/*.py' 'crates/*/src'` | all ten rows exist and do what the row says; **an eleventh exists** | see MAJOR-5 |
| "five undriven gate scripts, still five" | for every `git ls-files 'tools/*.sh' 'tools/**/*.sh'`, `git grep -l -- "<basename>" -- crates`, then each hit read | five at `b60c3d3`; **four at HEAD** (`config_check` now driven). Every hit for `determinism`, `movetime_check`, `perft_check`, `search_oracle_check` is a comment or an `#[ignore]` reason; not one is a driver | AGREES |
| "`perft_check.sh` and `search_oracle_check.sh` produce no recorded number" | both files read in full | `perft_check.sh` = 1 echo + 1 `cargo test`; `search_oracle_check.sh` = 4 echoes + **7** `cargo test` | claim AGREES; the arithmetic in the CLOSURE does not — see MINOR-2 |
| register's own re-derivation command | `git grep -n "info totals\|' totals '\|\" totals \"" -- crates tools`, at both `b60c3d3` and HEAD | finds **neither** `tools/bench_delta.sh` (its own row 8) **nor** `tools/staged_cover_bench.sh` | see MAJOR-5 |

Also re-derived: `configs/` holds 45 `.toml` in the worktree and 45 in the index, and
`tools/config_check.sh` reports `21 + 1 + 18 + 3 + 2 = 45` — the cross-check in
`config_check_gate_tests` is sound.

---

## Findings

### BLOCKING-1 — gate 17 is RED at the revision under review, and T3's own commit turned it red

`tools/file_justification_check.sh` exits **1** at `cb6e853`. Four tracked files are over
the 300-line cap with no entry in `docs/rule9_justifications.md`:

```
crates/pistol-cli/src/random_openings/mod.rs   301
crates/pistol-cli/tests/bench_delta_tests.rs   302
tools/determinism.sh                           308
tools/file_justification_check.sh              301
```

Reproducer:

```
$ cd /home/tom/Projects/HeXO-AlphaBeta && ./tools/file_justification_check.sh; echo "EXIT=$?"
...
file_justification_check: crates/pistol-cli/src/random_openings/mod.rs: over the cap with no entry in docs/rule9_justifications.md
file_justification_check: crates/pistol-cli/tests/bench_delta_tests.rs: over the cap with no entry in docs/rule9_justifications.md
file_justification_check: tools/determinism.sh: over the cap with no entry in docs/rule9_justifications.md
file_justification_check: tools/file_justification_check.sh: over the cap with no entry in docs/rule9_justifications.md
file_justification_check: FAIL: 4 finding(s) against CLAUDE.md rule 9's soft cap
EXIT=1
```

Dated per commit (my own enumeration: `git ls-tree -r --name-only <rev>` filtered to
`.rs/.sh/.py`, per-blob `wc -l`, registry entries by anchored regex):

```
b9429fb  tracked=406 over=84 entries=72 UNREGISTERED=12   (gate did not read .py yet — green)
86a7dc7  tracked=406 over=84 entries=72 UNREGISTERED=12   (T1 — green)
b60c3d3  tracked=406 over=84 entries=84 UNREGISTERED=0    (T2 — green, and the CLOSURE's number)
1f6e388  tracked=406 over=84 entries=84 UNREGISTERED=0    (green)
42ab538  tracked=407 over=87 entries=84 UNREGISTERED=3    (T3 — RED from here on)
1996d05  tracked=408 over=87 entries=84 UNREGISTERED=3
c586837  tracked=409 over=87 entries=84 UNREGISTERED=3
f4f4a5c  tracked=409 over=88 entries=84 UNREGISTERED=4
c105ad4  tracked=409 over=88 entries=84 UNREGISTERED=4
cb6e853  tracked=409 over=88 entries=84 UNREGISTERED=4
```

Three of the four crossed in `42ab538` — T3's own preflight blocks pushed
`tools/determinism.sh` (290 → 308) and `tools/file_justification_check.sh` (287 → 301)
over, and the copy-the-preflight edit pushed `crates/pistol-cli/tests/bench_delta_tests.rs`
(297 → 302). The fourth, `crates/pistol-cli/src/random_openings/mod.rs` (295 → 301),
crossed in `f4f4a5c`. The CLOSURE's H1 green is cited at `007f821`, which is before
every one of these, so nothing in the document is evidence about the state under review
(CLAUDE.md Closure: a gate claim cites the gate's own log output).

**The irony is load-bearing, not decoration:** the gate T2 widened is itself one of the
four files it now refuses, and the package that widened it broke it in the next commit.
Fix: four registry entries (or four files trimmed). Nothing here says the code is wrong;
it says the gate the package hardened does not pass at the revision the package closes at.

### MAJOR-1 — `tools/solver_determinism.sh` preflights AFTER the build it claims to protect

Item 12 obligation 2 is *"PREFLIGHT WHAT THE RUN NEEDS AND VOID EARLY … Discovering the
shortage through a tool's own error message is discovering it in the tool's vocabulary."*
In `tools/solver_determinism.sh` the release build is line 46 and the preflight is lines
81–86. It is the only one of the nine where the order is inverted: `determinism.sh`
(preflight 122–125, build 141), `book_v3_disjointness.sh` (64–67, build 79) and
`arena_smoke.sh` (71–75, build 94) all get it right.

The block's own comment asserts the opposite, and that assertion is false:

```
tools/solver_determinism.sh:75-80
# SCRATCH SPACE, ASKED FOR BEFORE THE WORK … BOTH filesystems … the build goes to this
# repository's target tree … A shortage on either otherwise reaches the log in `mktemp`'s
# or `cargo`'s vocabulary, which describes those tools rather than this gate.
```

Paired reproducer (a stub `cargo` first on PATH that records a marker; `TMPDIR` set to a
directory the preflight must refuse):

```
$ MARKER=$SP/m1 PATH="$SP/stubbin:$PATH" TMPDIR=/nonexistent-preflight-probe ./tools/solver_determinism.sh
solver_determinism: RUN VOID: cargo built no executable for --bin solver-selftest
EXIT=2
$ cat $SP/m1
STUB CARGO INVOKED: build --release --locked -p pistol-solver --bin solver-selftest --message-format=json-render-diagnostics

$ MARKER=$SP/m2 PATH="$SP/stubbin:$PATH" TMPDIR=/nonexistent-preflight-probe ./tools/determinism.sh
scratch_preflight: RUN VOID: no such directory to preflight: `/nonexistent-preflight-probe`
determinism: RUN VOID: no scratch room; the lines above name the filesystem
EXIT=2
$ cat $SP/m2
(no marker: build did NOT run)
```

Consequence with the real `cargo`: on a full filesystem this gate voids with
`"the build failed — cargo's own words are above"`, which is cargo's vocabulary — the
exact D-281 reading the item exists to close. Under `tools/ci.sh` the top-of-script
preflight masks it; run standalone (which is how D-672 was reproduced) it does not.

Second-order: the new test is named
`the_gate_asks_for_its_scratch_before_it_writes_any` and asserts only
`out.contains("scratch_preflight:")`. It observes presence, never order, so its name
states a property it cannot see — and for this one gate the property is false. Moving the
preflight block above line 46 fixes both.

### MAJOR-2 — `tools/config_check.sh`'s new "no void class" block asserts a property the script does not have

T3 added (`tools/config_check.sh:30-34`):

> `THERE IS NO VOID CLASS, stated rather than left to be inferred from silence … This gate
> writes no scratch and reads only committed documents, so it has no way to be short of
> anything: every non-zero answer is a document that did not load.`

Both halves are false. The gate runs five `cargo run --quiet --locked … --example …`
invocations (lines 108–129); `cargo run` builds, and a build writes to `$ROOT/target`. An
environmental build failure exits 1, and `tools/ci.sh`'s `gate()` maps 1 to `fail`:

```
$ cat $SP/stubbin2/cargo
#!/usr/bin/env bash
echo "error: failed to write to `…/target`: No space left on device (os error 28)" >&2
exit 101
$ PATH="$SP/stubbin2:$PATH" ./tools/config_check.sh; echo "EXIT=$?"
config_check: 21 engine config(s), 1 weight table(s), 18 arena config(s), 3 book config(s), 2 solver config(s)
error: failed to write to : No space left on device (os error 28)      [x5]
EXIT=1
```

The gate emits no refusal of its own; the only diagnostic is cargo's. The usage block now
tells a reader that this exit 1 means a committed document did not load. Item 12
obligation 1 asks a gate with no void class to *say so* — this one says so and the
statement is wrong, which is worse than silence, because a reader who trusts it goes
looking for a broken config.

### MAJOR-3 — `tools/bench_delta.sh` has no "no void class" statement, and the exemption is claimed on `baseline_snapshot.sh`'s evidence

The CLOSURE's T3 note 3 declines the void for two scripts with one paragraph:
*"`baseline_snapshot.sh`'s usage block already states — by decision, with its reason —
that it has no void class. Introducing one would also move an exit code its own suite and
D-220's recorded verdicts read."*

Checked separately:

```
$ /usr/bin/grep -n -i void tools/baseline_snapshot.sh
150:#        THERE IS NO VOID CLASS, stated rather than left to be inferred from
253:EXIT STATUS. This script declares no VOID class (tools/SHELL_CHECKLIST.md item …
482:# REFUSED AS a `fail` AND NOT AS A VOID, deliberately. …
$ /usr/bin/grep -n -i void tools/bench_delta.sh
146:# REFUSED AS A `fail` AND NOT AS A VOID, deliberately. …
148:# either wrote its record or it did not. Introducing a void class here would
$ sed -n '84,85p' tools/bench_delta.sh
# Exit:  0 measured and verdict printed, 1 a precondition or the run failed.
```

`bench_delta.sh`'s usage block does **not** state it has no void class; the only two
mentions of the word are inside the new preflight comment, which a reader consulting the
usage block never reaches. Item 12 obligation 1 is unmet for it.

The stated *cost* also does not hold for `bench_delta.sh`. Its own suite and
`binary_binding_tests.rs` read only `success()` / `!success()` / `code() != 0` — no test
reads a specific code:

```
$ /usr/bin/grep -n "code()\|status.success" crates/pistol-cli/tests/bench_delta_tests.rs \
    crates/pistol-arena/tests/binary_binding_tests.rs
crates/pistol-arena/tests/binary_binding_tests.rs:110:    assert_ne!(ran.code(), 0, …
crates/pistol-cli/tests/bench_delta_tests.rs:64,162,221,258,295: ran.status.success()
```

Whereas for `baseline_snapshot.sh` the cost is real and documented —
`baseline_snapshot_tests.rs:1267` asserts `Some(1)` with a comment that turns on the
absence of a void class. So the paragraph is sound for one script and unsupported for the
other. Minimum fix: one usage line in `tools/bench_delta.sh`. This lands on the instrument
`tools/SHELL_CHECKLIST.md` item 10 names by name as producing D-220's official perf verdict.

### MAJOR-4 — two edits to `tools/ci.sh` change gate 4's real behaviour and leave `clippy_gate_flag_tests` green

`gate_four_flags()` finds the first line starting with `cargo clippy --workspace`, takes
everything after the first `" -- "`, and stops at `||`. It therefore constrains the
post-`--` flag string and nothing else — not the cargo arguments before `--`, and not what
the shell does with the exit status. Both mutations run in the detached worktree, suite
re-run after each, then restored:

**M1 — the gate stops linting tests and examples, and drops `--locked`:**

```
-cargo clippy --workspace --all-targets --locked -- -D clippy::all -D warnings || fail "clippy"
+cargo clippy --workspace --lib -- -D clippy::all -D warnings || fail "clippy"

$ cargo test -p pistol-cli --locked --test clippy_gate_flag_tests
test result: ok. 3 passed; 0 failed; …
```

**M2 — the gate can no longer fail at all:**

```
-cargo clippy --workspace --all-targets --locked -- -D clippy::all -D warnings || fail "clippy"
+cargo clippy --workspace --all-targets --locked -- -D clippy::all -D warnings || true

$ cargo test -p pistol-cli --locked --test clippy_gate_flag_tests
test result: ok. 3 passed; 0 failed; …
```

M2 is the serious one: gate 4 becomes decorative and the test that exists to defend it
says nothing. The CLOSURE's claim — *"dropping either flag from the gate is what makes the
test fail"* — is true and is also the whole of what the test can see. A control that ran
the shipped `tools/ci.sh` gate-4 line against a planted warning (rather than re-running
`cargo clippy` with harvested flags) would close both. What the test *does* verify is
sound: I confirmed a flag-level weakening (`-A unused_imports` appended) does go red.

### MAJOR-5 — the `info totals` register misses an eleventh consumer, and its own stated command could not have produced the table

Two separate defects in the T5 appendix.

**(a) The eleventh consumer.** `tools/staged_cover_bench.sh:147-158` parses the grammar
with the identical `awk` the register lists as row 8 (`tools/bench_delta.sh:379-390`):

```
tools/staged_cover_bench.sh:147
	awk '/ totals /{
		nodes=""; time="";
		for (i=1; i<NF; i++) {
			if ($i=="nodes") nodes=$(i+1);
			if ($i=="time") time=$(i+1);
		}
		if (nodes=="" || time=="") { print "PARSE" > "/dev/stderr"; exit 1 }
		print nodes, time;
	}' "$WORK/raw" >"$WORK/totals" || fail "engine on $budget_name: unparseable totals line"
```

Byte-identical at `b60c3d3`, the register's own stated revision, so it is not a later
arrival. It meets every criterion the register uses for row 8: a `tools/` script, keying
on the ` totals ` discriminator, extracting `nodes` and `time` by field name, with a named
refusal. `TEN SITES IN NINE FILES` should be eleven in ten.

**(b) The command as printed finds neither the eleventh nor row 8.** Re-running the
register's own citation:

```
$ git grep -n "info totals\|' totals '\|\" totals \"" b60c3d3 -- crates tools | grep -E "bench_delta\.sh|staged_cover_bench\.sh"
  (nothing)
$ git grep -n "info totals\|' totals '\|\" totals \"" -- crates tools | grep -E "bench_delta\.sh|staged_cover_bench\.sh"
crates/pistol-cli/tests/bench_delta_tests.rs:17,37   ← the TEST, not the script
crates/pistol-cli/tests/staged_cover_bench_gate_tests.rs:23,43
```

`awk '/ totals /{` puts the substring between slashes, not between quotes, so neither
alternative in the pattern matches it. The register's most-advertised row — *"A-08 missed
it; it produces this project's OFFICIAL perf verdict (D-220)"* — cannot have come from the
command the register cites, and the same blind spot is exactly what hid the eleventh.
`git grep -n " totals "` over `'tools/*.sh' 'tools/**/*.sh' 'tools/*.py' 'crates/*/src'`
finds all of them. This is `docs/process.md`'s named class — *a claim checked against the
wrong population* — inside the register whose closing line demands re-derivation.

Also under-described: rows for `tools/baseline_snapshot.sh` name lines 501 and 648, but the
same file parses the same grammar's per-depth form at `:739` (`grep '^info depth_turns '`)
and `:780` (an `awk /^info depth_turns /` reading `depth_turns` and `time`). A grammar
change following the register's row would check two of four sites in that file.

### MAJOR-6 — the rule-9 gate is the only one of the three with no live-tree control, which is why BLOCKING-1 survived four commits

`crates/pistol-cli/tests/file_justification_gate_tests.rs` builds a scratch git repository
for every case (`scratch_repo()` at :21, `git init` at :39) and never drives the gate
against the repository under test. Its eleven tests pass at `cb6e853` while the shipped
gate exits 1 on the real tree.

```
$ cargo test -p pistol-cli --locked --test file_justification_gate_tests
test result: ok. 11 passed; 0 failed; …
$ ./tools/file_justification_check.sh; echo $?
… FAIL: 4 finding(s) against CLAUDE.md rule 9's soft cap
1
```

Its two siblings do carry the control: `config_check_gate_tests::
every_committed_document_loads_and_the_summary_counts_them_all` and
`solver_determinism_gate_tests::the_shipped_solver_determinism_script_passes_and_says_so`
both run the shipped script on the live repository. Adding the same shape here would have
turned `cargo test --workspace` red at `42ab538` instead of leaving the finding for a
reviewer. Item 10 asks for a control run against a gate that refuses everything; this
suite has that. What it lacks is the control that the gate *accepts the tree it ships in*.

### MINOR-1 — the EXIT trap can overwrite a deliberate void with a fail

`tools/SHELL_CHECKLIST.md` item 7: *"Take `local rc=$?` as the trap's first statement and
`return "$rc"` as its last."* Three of the changed scripts use the bare form —
`tools/solver_determinism.sh:89`, `tools/determinism.sh:132`,
`tools/file_justification_check.sh:168` — all `trap 'rm -rf "$X"' EXIT`. Bash lets the
trap's last command decide the status:

```
$ cat trapprobe2.sh          # trap 'false' EXIT ; exit 3
$ bash trapprobe2.sh; echo $?
1
$ # and with a real rm -rf that cannot remove (read-only parent), script chose exit 2:
$ bash trapprobe4.sh; echo $?
rm: cannot remove '…/ro/inner': Permission denied
1
```

So a chosen `void` (2) becomes `fail` (1), which `ci.sh`'s `gate()` reports as a
regression — the very reading item 12 exists to prevent. The trap form is **pre-existing**
in all three; what T1/T3 changed is that two of them now *have* a void to lose.
`book_v3_disjointness.sh:73`, `decision_key_check.sh:112` and
`label_consistency_check.sh:127` already use the `rc=$?; …; exit "$rc"` form and are clean.
Reachability is low (an unremovable scratch directory), hence MINOR.

### MINOR-2 — the CLOSURE miscounts `search_oracle_check.sh`

CLOSURE T3: *"`search_oracle_check.sh` is four echoes and six [cargo tests]"*.

```
$ /usr/bin/grep -c '^echo ' tools/search_oracle_check.sh        → 4
$ /usr/bin/grep -c '^cargo test ' tools/search_oracle_check.sh  → 7
```

Seven, at lines 44, 51, 54, 64, 68, 76, 86. `perft_check.sh` ("one echo and one cargo
test") is correct. The conclusion — that the verdict is cargo's and item 10's letter does
not bind them — is unaffected, and I agree with it.

### MINOR-3 — gate 4's printed description is not the command it runs

```
tools/ci.sh:105  step "gate 4/$GATE_TOTAL: cargo clippy --workspace --all-targets -- -D clippy::all -D warnings"
tools/ci.sh:106  cargo clippy --workspace --all-targets --locked -- -D clippy::all -D warnings || fail "clippy"
```

`--locked` is in the command and not in the log line. CLAUDE.md's Closure says a gate
claim in any report cites the gate's own log output; here that output understates the
gate. One word.

### MINOR-4 — `tools/puzzle_corpus/tests/run_tests.sh:33` is item 3's shape

```
python3 tools/puzzle_corpus/tests/test_extract.py | grep -v '^  ok' || fail "extractor tests"
```

`grep -v` exits 1 when it filters everything out — i.e. exactly when every extractor test
printed `  ok`. A fully passing run would be recorded as `FAIL: extractor tests`. Latent
today only because the script prints five section headers:

```
$ python3 tools/puzzle_corpus/tests/test_extract.py | /usr/bin/grep -vc '^  ok'
5
```

Pre-existing, in a script T3 changed, and this file is not a CI gate — hence MINOR.

### MINOR-5 — T4 prices one option for closing A-19's class and there is a cheaper one

CLOSURE T4: *"What would close it is a release-profile lint pass … a second full `clippy`
of the workspace in a second profile."* Measured, with a control, in the worktree:

```
# A-19 restored, plus [workspace.lints.rust] unused_imports = "deny":
$ cargo build --release -p pistol-solver --bin solver-selftest --locked
error: unused import: `generate_turns`  … REL_BUILD_EXIT=101
# control, same tree without the lints entry:
warning: unused import: `generate_turns` … REL_BUILD_EXIT_NO_DENY=0
```

CI already runs release builds inside gates 9 and 13, so a lints-table deny closes the
class at no extra CI time. It is not a full substitute — the failure would arrive as
`solver_determinism: RUN VOID: the build failed`, which is the wrong class — and that cost
belongs in the matrix rather than being the reason not to write one. The item is recorded
as *owed* rather than adopted, so no OPTION MATRIX is due yet; this is guidance for the
resume.

Related, on the second half of the `-D warnings` question: there is no `rust-toolchain`
file (CLAUDE.md Environment; `rust-version = "1.97"`, running 1.98.0), so a rustc upgrade
that adds a warning or a deprecation turns gate 4 red on unchanged code, and gate 4 is not
wrapped in the void-aware `gate()` helper. I judge this **acceptable**: `[workspace.lints.clippy]
all = "deny"` in `Cargo.toml` already gave the tree the same exposure to new *clippy*
lints, `-D warnings` widens rather than creates it, and a lint break on a toolchain bump
is loud, local and cheap to triage. Worth a sentence in the gate's comment, not a change.

### MINOR-6 — the cap is measured with `wc -l`, so an unterminated last line escapes it

`verdict()` uses `lines="$(wc -l <"$bytes")"`, which counts newlines. A 301-line file whose
last line carries no newline reads as 300 and is `under`. Live instance of the shape (not
over the cap, so harmless today):

```
$ git cat-file blob :tools/sealbot/matchserver/src/main.rs | wc -l   → 216
$ git cat-file blob :tools/sealbot/matchserver/src/main.rs | awk 'END{print NR}'  → 217
```

Note the gate already handles this correctly for the *registry* (`|| [ -n "$line" ]` at
:112, with a comment saying why). The same care is not applied to the measurement.

### MINOR-7 — `tools/solver_oracle_check.sh` writes scratch and does not preflight

The `mktemp -d` enumeration was keyed on `-d`, so it did not reach
`tools/solver_oracle_check.sh:32` `OUT="$(mktemp)" || void "mktemp refused"` — a scratch
*file* under `$TMPDIR`, in a CI gate (gate 12) with a void class already defined. Item 12
obligation 2 says "a gate that writes scratch", not "a gate that writes a scratch
directory". One line, and it is the same population-scope error as MAJOR-5(b).

---

## What I verified clean

- **Item 1 (discarded command-substitution status).** The T1 ladder is exemplary:
  `mapfile` takes the `sed` output as a *value*, `NAMED="$(grep -c … || true)"` is checked
  for spelling before arithmetic, and `[ "$NAMED" -eq "${#BUILT[@]}" ]` is the
  named-vs-read cross-check the item asks for. `PREFLIGHT="$(cd -- … && pwd)/…"` does
  propagate under `set -e` (bash takes the assignment's status from its last substitution),
  so it is a bare death rather than an unnamed pass; the following `[ -x … ] || void` is
  the named refusal for every value that survives. `fail "run A refused: $(cat …)"`
  discards the `cat` status, but only to build a message, and the refusal still fires.
- **Item 2 (pipeline in a `then` body).** No new statement-position pipeline in the T diffs
  except MINOR-4, which is pre-existing and in a script without `set -e`.
- **Item 3 (`grep` under pipefail).** Every new `grep` in a statement position carries
  `|| true` with a comment (`solver_determinism.sh:58,97,98,102`), and the `-c` cases check
  the *spelling* of the count, not only its value.
- **Item 4 (`LC_ALL`).** Unchanged. `scratch_preflight.sh` pins `LC_ALL=C` around `stat`
  for determinism and deliberately does not pin it around the `*[![:print:]]*` allow-list.
  Still correct.
- **Item 5 (index vs worktree).** `file_justification_check.sh` reads
  `git ls-files -s -z` + `git cat-file blob`, including the registry's own blob, and counts
  the file set with the same enumeration the loop uses. The `.py` widening did not weaken
  it: the summary suffix list and the `ls-files` pathspec are the same three suffixes, and
  `the_justification_gate_counts_every_suffix_its_summary_says_it_counts` pins that.
  (`config_check_gate_tests` names its independent enumeration `tracked` while counting
  worktree files — the two agree at HEAD, 45 = 45, so it is a naming nit, not a defect.)
- **Item 6 (sweep by prefix).** No new sweep. `SCRATCH_PREFIX` unchanged.
- **Item 7 (traps).** One trap per script, second scratch nested inside the first
  (`file_justification_check.sh:246-249` says so), `git worktree prune` ordering unchanged
  in `bench_delta.sh`. Exit-status half is MINOR-1.
- **Item 8 (one spelling per number, one refusal per reason).** T1's ladder is the best
  instance of this rule in the tree: the artifact-record count's spelling checked before
  arithmetic, then *separate* refusals for none, several, absent, not-a-regular-file and
  not-executable, with the `126` case called out by name. The `command -v` triple-refusal
  sweep is STOPPED with a reason I accept: item 8's own text is scheduled to change, and
  36 sites rewritten against a moving rule is the sweep written twice.
- **Item 9 (what reaches a record).** New values interpolated into refusals (`$BIN`,
  `${BUILT[*]}`, `$PREFLIGHT`, `$SCRATCH_FS`) are all script-derived; nothing
  caller-supplied newly reaches a parsed record.
- **Item 10 (THE COVERAGE RULE).** Every script that gained a preflight now has at least
  one test driving the shipped script with a control, and all five "asks for its scratch"
  assertions are **real observations, not decoration** — I removed the preflight *call*
  from each of the five gates (leaving `PREFLIGHT=` and the `-x` guard) and each suite went
  red on exactly the one test:
  ```
  book_v3_disjointness_tests   a_disjoint_book_answers_yes_and_names_both_terms_on_every_line ... FAILED
  decision_key_check_tests     the_key_gate_asks_for_its_scratch_before_it_seeds_any ... FAILED
  file_justification_gate_tests the_justification_gate_asks_for_its_scratch_before_it_seeds_any ... FAILED
  label_consistency_check_tests the_label_gate_asks_for_its_scratch_before_it_seeds_any ... FAILED
  solver_determinism_gate_tests the_gate_asks_for_its_scratch_before_it_writes_any ... FAILED
  ```
  (5 failures, 40 other tests still passing; worktree restored.) They observe presence, not
  order — see MAJOR-1.
- **Item 11 (containment guard).** Enumerated every destructive site in the ten changed
  scripts and traced each target to its origin: every `rm`/`>` target derives from
  `mktemp -d` (`$WORK`, `$SEED`, `$SCRATCH`, `$OUT`) or from `$WORK/...`. The one
  caller-supplied write target in the set, `baseline_snapshot.sh:819 cp "$WORK/record"
  "$OUT"`, is resolved against `CALLER_PWD` (`:171`, `:296-300`) and guarded, unchanged by
  this package. `$PREFLIGHT` is executed, not deleted, and is derived from `BASH_SOURCE`.
  No new caller-supplied binding reaches a delete or an overwrite.
- **Item 12 (VOID vs FAIL, by name).** Obligation 3 survives the CI seam: every gate 5–21
  goes through `ci.sh`'s void-aware `gate()` wrapper, so a new exit 2 reads as
  `void "… took no answer … this is NOT a regression"` rather than as `fail`. Obligation 1
  is met by `solver_determinism.sh`, `determinism.sh`, `file_justification_check.sh` and
  `puzzle_corpus/tests/run_tests.sh` (new usage lines) — and unmet by `bench_delta.sh`
  (MAJOR-3) and mis-stated by `config_check.sh` (MAJOR-2).
- **Attack 2 — T1's void ladder, no fail dressed as a void.** Every refusal from line 46 to
  line 73 is a genuine "no answer was taken": a refused build, a quote/backslash in an
  artifact path, no executable, several executables, an absent/irregular/non-executable
  binary. None can be reached by a solver that has lost determinism. The determinism
  *comparison* (lines 92–109) and the positive-content check (97–104) all use `fail`, so no
  regression path was converted. A refused build being a void is D-281's own precedent and
  cannot hide anything, because gates 3 and 4 use `fail` and run first. `assert_code` names
  what the other codes mean, which is obligation 3 in the crate.
- **Attack 3 — T4's central claim, verified independently.** Restoring A-19's crate-level
  import in the worktree and running gate 4's own flags:
  ```
  $ cargo clippy -p pistol-solver --all-targets --locked -- -D clippy::all -D warnings   → EXIT 0
  $ cargo clippy --release -p pistol-solver --all-targets --locked -- (same flags)       → EXIT 101
    error: unused import: `generate_turns` … `-D unused-imports` implied by `-D warnings`
  ```
  Gate 4 is a dev-profile run, `debug_assertions` is on, the import is live, and the gate
  would **not** have caught the warning it is named for. The CLOSURE says so plainly
  ("the class is half closed"), which is the right disclosure. The `-D warnings`
  redundancy question also checks out: `[workspace.lints.clippy] all = "deny"` already
  covers the clippy half for workspace members, and `-D warnings` is what adds the rustc
  half to the flags the test harvests.
- **T2's claims.** `b60c3d3` touched exactly one `.py` file (`tools/puzzle_corpus/
  extract.py`), deleting a marker whose last sentence the widening falsified; its registry
  entry at `docs/rule9_justifications.md:96` carries the marker's text verbatim minus that
  sentence. `tools/wp16_warm_attribution_check.py`'s marker is deliberately left with its
  now-false parenthetical and is the only in-file `RULE9-JUSTIFICATION` marker remaining in
  a source file — recorded as owed, which I accept.
- **The register's three producers** exist at the cited locations:
  `crates/pistol-cli/src/report.rs:39` (`TOTALS_MARKER`),
  `crates/pistol-arena/src/bin/stub_engine.rs:631`, `tools/sealbot/tests/stub_pistol.py:86`.
  `crates/pistol-arena/src/record.rs:113` is correctly excluded (it folds parsed numbers);
  `crates/pistol-arena/src/capture_file.rs:27` is a field doc, not a parser, and is
  correctly absent.
- **Suites at `cb6e853`, unmutated:** 99 tests across `config_check_gate_tests` (4),
  `file_justification_gate_tests` (11), `decision_key_check_tests` (10),
  `label_consistency_check_tests` (13), `book_v3_disjointness_tests` (6),
  `solver_determinism_gate_tests` (5), `clippy_gate_flag_tests` (3),
  `scratch_preflight_tests` (6), `baseline_snapshot_tests` (37), `bench_delta_tests` (4) —
  all green. Gates run green at HEAD: `artifact_check` (859 tracked files),
  `config_check` (45 documents), `decision_key_check` (682 keys),
  `label_consistency_check` (6 documents), `governing_citation_check`,
  `solver_determinism` (61 cases). Gate 17 is the exception (BLOCKING-1).
- **No caller or harness broken by the BASH_SOURCE resolution.** Every suite that copies a
  gate into a scratch tree now copies `scratch_preflight.sh` beside it
  (`arena_smoke`, `baseline_snapshot`, `bench_delta`, `book_v3_disjointness`,
  `decision_key`, `file_justification`, `label_consistency` ×2, `staged_cover_bench`,
  `staged_soundness`), and `tools/puzzle_corpus/tests/run_tests.sh` correctly uses
  `$ROOT/tools/…` because it sits two levels down and has no sibling preflight — that is
  the same rule, not an exception to it.

## What I could not verify

- **A full `tools/ci.sh` run at `cb6e853`.** Not taken: gate 17 is red, so the run would
  stop there and tell me nothing about gates 18–21 that I did not get from running them
  individually. The claim "all 21 gates green" is not established at this revision by
  anything I did, and the CLOSURE's H1 log is from `007f821`.
- **Gates 8, 10, 11, 12, 14, 15, 16, 18** — minutes to tens of minutes each and untouched
  by group T. Not run.
- **`tools/determinism.sh` end-to-end** (72 s per the CLOSURE's own measurement) with a
  real engine. I drove it only far enough to establish the preflight ordering (stub cargo).
  Its recorded numbers therefore stand on the CLOSURE's measurement, not on mine.
- **Whether the `--include-ignored` risk the CLOSURE names for `perft_check.sh` /
  `search_oracle_check.sh` is live.** I confirmed the flags are present today and that a
  string-reading test would be the weak shape MAJOR-4 demonstrates; I did not design a
  better instrument.
- **The T3 STOP on `determinism.sh` / `movetime_check.sh` seams.** I agree a control needs
  a seeded violation and that both scripts currently offer no seam (`determinism.sh:192`
  takes budget overrides only; `movetime_check.sh` takes nothing), so I agree an OPTION
  MATRIX is owed. I did not attempt to design the seam, which is out of a reviewer's remit.
- **Whether `docs/decisions.md` D-677/D-678/D-679 say what the CLOSURE says they say.**
  Out of scope for group T's tools/gates remit; I checked the code against the checklist and
  the CLOSURE, not the ADR log against either.

---

## Verdict

**FAIL**
