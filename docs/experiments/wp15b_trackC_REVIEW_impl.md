# WP-1.5b Track C — REVIEW-impl

**Pinned revision:** `a102c6a06247c9590430ae378b671537d344e680`.
**Matches HEAD:** at the START of this review, yes — `git rev-parse HEAD` returned
`a102c6a06247c9590430ae378b671537d344e680`. **By the END it no longer did:** HEAD is
`7866bcf0e84072dd66728e713708332b0f11c1ba`, one commit ahead. That commit is
`docs(experiments): MATRIX M4 round 4 authors N-Q into axis A …`, and
`git diff --stat a102c6a..HEAD` shows it adds exactly one file,
`docs/experiments/matrix_M4_axisA_round4.md` (+305), touching **no** file under review.
Every measurement in this report was taken against the pinned revision's bytes, and the
pinned bytes are still HEAD's bytes for all five reviewed files.
`git status --porcelain` was empty at the start of this review and empty at its end.

**Subject:** two commits reviewed together as one defect class —
`b067d47` (`tools/baseline_snapshot.sh`, `crates/pistol-cli/tests/baseline_snapshot_tests.rs`)
and `a102c6a` (`crates/pistol-cli/src/report.rs`, `crates/pistol-cli/src/bin/pistol.rs`,
`crates/pistol-cli/tests/handshake_identity_tests.rs`).

**Reviewer:** fresh context, did not implement either change. No repository file was
edited and no git write command was run in the live tree; all mutation work happened
in a throwaway worktree at `/home/tom/.cache/review-trackC`, since removed.

**Verdict summary:** both fixes hold when driven directly, both tests were confirmed
to fail against the pre-fix sources, and both control halves were confirmed to bind by
mutation. Three MINOR findings, all in the *prose and seam* that `b067d47` newly added
around its guard, none in the guards themselves. **0 BLOCKING, 0 MAJOR, 3 MINOR.**

---

## 1. Do the fixes hold? (shipped artefacts, driven by hand)

Build, first, so every drive below is against the shipped release binary:

```
cargo build --release -p pistol-cli --bin pistol
    Finished `release` profile [optimized] target(s) in 0.01s
```

`SP` below is this review's scratchpad; `$SP/drive/mini.txt` is a two-entry corpus,
one position per band, cut from the committed `bench_positions_v1.txt` the same way
the suite's `band_entries()` cuts it.

### 1a. A spaced `--corpus` BASENAME is refused, exit 1, named

```
$ cp "$SP/drive/mini.txt" "$SP/drive/mini corpus.txt"
$ ./tools/baseline_snapshot.sh --corpus "$SP/drive/mini corpus.txt"; echo "exit=$?"
exit=1
baseline_snapshot: FAIL: the corpus path `…/scratchpad/drive/mini corpus.txt` has a
SPACE in its file name, and its name is written into a whitespace-delimited field of
the record's invariant block, where it would shift every field after it
```

HOLDS. Exit 1, the script's own `baseline_snapshot: FAIL:` prefix, the offending path
quoted back (SHELL_CHECKLIST item 9), nothing written.

### 1b. Control — a spaced DIRECTORY with an unspaced basename is NOT refused

```
$ mkdir -p "$SP/drive/corpus dir"; cp "$SP/drive/mini.txt" "$SP/drive/corpus dir/mini.txt"
$ ./tools/baseline_snapshot.sh --corpus "$SP/drive/corpus dir/mini.txt" \
      --nodes 1000 --ladder-depth 1 --out "$SP/drive/b.record"; echo "exit=$?"
exit=0
$ grep -n '^corpus \|^openings ' "$SP/drive/b.record"
16:corpus mini.txt sha256 252fa0d8ec761ea9870d02c479e995b617bc7c8df22a95794126f05a8b7fb527 positions 2
17:openings openings_v1.txt sha256 5ccc3dc0ccfc9ed8df1135c74cc161fafcc0fd8bd8ff750149ddd0e8e2bcd530
```

HOLDS, and the guard is a guard rather than a wall: field 2 is the basename, field 3
is `sha256`, field 4 is 64 hex digits.

### 1c. The pre-existing printable-ASCII arm was NOT narrowed

Three names, all still refused by the *original* arm (`*[![:print:]]*`), each exit 1:

```
$ for name in $'mini\nx.txt' $'mini\tx.txt' $'mini x.txt'; do
      cp "$SP/drive/mini.txt" "$SP/drive/$name"
      ./tools/baseline_snapshot.sh --corpus "$SP/drive/$name" --nodes 1000 \
          --ladder-depth 1 --out "$SP/drive/e.record"; echo "exit=$?"; done
exit=1  baseline_snapshot: FAIL: … has a character outside printable ASCII in its file name …   (LF)
exit=1  baseline_snapshot: FAIL: … has a character outside printable ASCII in its file name …   (TAB)
exit=1  baseline_snapshot: FAIL: … has a character outside printable ASCII in its file name …   (U+2028)
```

U+2028 refusing is the direct evidence for SHELL_CHECKLIST item 4: the allow-list form
plus the `LC_ALL=C` pin refuses *more*, not less. And a legitimate printable name still
runs — `mini?x.txt` gave exit 0 with a complete record.

### 1d. The engine refuses an unechoable `--config`, exit 2, no `id config`

```
$ cp configs/instrument_v0.toml "$SP/cfg/$(printf 'inst\001v0.toml')"
$ printf 'pistol\nquit\n' | ./target/release/pistol --config "$SP/cfg/$(printf 'inst\001v0.toml')"
exit=2
stdout: (empty)
stderr: pistol: --config `…/scratchpad/cfg/inst\u{1}v0.toml`: the path holds a control
character, and the `id config` line of the handshake has to be the path a reader can
re-run this with
$ cat d.out d.err | grep -c 'id config '
0
```

HOLDS. Exit 2 is the binary's own `REFUSED` code, the refusal names what it found, the
control character is `escape_debug`d in the message (so the refusal cannot itself carry
a raw control byte), and no `id config` line was emitted on either stream.

### 1e. Control — the ordinary path is unchanged

```
$ printf 'pistol\nquit\n' | ./target/release/pistol --config configs/instrument_v0.toml | grep '^id config'
id config configs/instrument_v0.toml
exit=0
```

HOLDS, byte-identical to the pre-fix spelling.

---

## 2. Do the tests genuinely bind?

All four runs below are in a **git worktree under `/home`** —
`git worktree add --detach /home/tom/.cache/review-trackC a102c6a` — with
`CARGO_TARGET_DIR` unset and its own `target/`. The worktree was removed with
`git worktree remove --force` afterwards.

### 2a. `b067d47`'s test against the UNPATCHED script — FAILS, as claimed

```
$ cd /home/tom/.cache/review-trackC
$ git show 369d43a:tools/baseline_snapshot.sh > tools/baseline_snapshot.sh
$ cargo test -p pistol-cli --test baseline_snapshot_tests a_corpus_name_carrying_a_space
…
corpus mini corpus.txt sha256 e3b79bdc423f202c5d01618dcaaff9da76e14ea652b9e71934017e3d72337e7f positions 2
…
  left: Some(0)
 right: Some(1)
test result: FAILED. 0 passed; 1 failed; 29 filtered out
```

VERIFIED INDEPENDENTLY. `left: Some(0) / right: Some(1)` exactly as the commit message
states, and the failure message carries the mangled record — the fourth token of the
`corpus` line is the literal string `sha256`.

### 2b. `b067d47`'s CONTROL half is not vacuous — mutation kill

Guard mutated to refuse everything (`*' '*)` → `*)`), fixed script otherwise:

```
$ sed -i "s/\t\*' '\*) fail \"the corpus path/\t*) fail \"the corpus path/" tools/baseline_snapshot.sh
$ cargo test -p pistol-cli --test baseline_snapshot_tests a_corpus_name_carrying_a_space
panicked at crates/pistol-cli/tests/baseline_snapshot_tests.rs:204:9:
the snapshot script must succeed:
stderr: baseline_snapshot: FAIL: the corpus path `/tmp/pistol-testscratch-…/corpus dir/mini.txt`
        has a SPACE in its file name …
test result: FAILED.
```

The control half KILLS a refuse-everything guard. Not vacuous.

### 2c. `a102c6a`'s test against the UNFIXED sources — FAILS, as claimed

```
$ git show b067d47:crates/pistol-cli/src/report.rs      > crates/pistol-cli/src/report.rs
$ git show b067d47:crates/pistol-cli/src/bin/pistol.rs  > crates/pistol-cli/src/bin/pistol.rs
$ cargo test -p pistol-cli --test handshake_identity_tests the_handshakes_config_line
id config /tmp/pistol-testscratch-…-handshake-control-character-config-0/inst?v0.toml
  left: Some(0)
 right: Some(2)
test result: FAILED. 0 passed; 1 failed; 2 filtered out
```

VERIFIED INDEPENDENTLY. `left: Some(0) / right: Some(2)` and `id config …/inst?v0.toml`
in the failure message, exactly as claimed.

### 2d. `a102c6a`'s CONTROL half is not vacuous — mutation kill

Predicate mutated to refuse every config (`if !report::travels_verbatim(…)` → `if true`):

```
$ cargo test -p pistol-cli --test handshake_identity_tests the_handshakes_config_line
panicked at crates/pistol-cli/tests/handshake_identity_tests.rs:156:5:
the engine refused an ordinary config path: pistol: --config
`/home/tom/.cache/review-trackC/configs/instrument_v0.toml`: the path holds a control character …
test result: FAILED.
```

The control half KILLS a refuse-everything engine. Not vacuous.

### 2e. Exit CODE vs `!success` in `baseline_snapshot_tests`

The new test asserts `assert_eq!(ran.status.code(), Some(1), …)` and its message spells
out what `0` and "anything else" would have meant — SHELL_CHECKLIST item 12 obligation 3
is met **for this test**. See finding **F3** for the seam it does not cover.

### 2f. Live-tree suites, at HEAD

```
$ cargo test -p pistol-cli --test baseline_snapshot_tests --test handshake_identity_tests --test report_tests
baseline_snapshot_tests  test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 43.84s
handshake_identity_tests test result: ok.  3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
report_tests             test result: ok.  6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 3. `tools/SHELL_CHECKLIST.md`, item by item, for `b067d47`

My own answers, derived from the file at `a102c6a`, not restated from the implementer.

**Item 1 — a command substitution whose status is DISCARDED.**
PASS, and the change is a net removal in this class. The two lines the commit touched
were `echo "corpus $(basename "$CORPUS") …"` and `echo "openings $(basename "$OPENINGS") …"`:
two substitutions in `echo` ARGUMENT position, statuses unreachable by `set -e`. Both are
gone, replaced by pure parameter expansions that cannot fail. I enumerated the remaining
substitution-in-argument sites in the file — line 502 (`$(tail_of "$entry")`, feeding the
engine session, not a record), line 611 and line 656 (`$(field …)`, both writing to
`$TIMING`, i.e. **below** the `# timing` marker and excluded from every comparison by the
header's own rule). No substitution-in-argument remains above the marker. `digest()`
still takes its value into a variable, checks its shape and refuses by name; it is
called as a statement, not inside a substitution, so its `fail` can exit the script.

**Item 2 — a pipeline in a `then` body is not a pipeline in a condition.**
PASS, not touched. The change adds no pipeline. The pipelines that exist
(`entries()`, the handshake `printf | timeout …`, the ladder `printf | timeout …`) carry
`|| true`, an explicit `rc=$?` capture, or a named `fail`, unchanged by this commit.

**Item 3 — `grep` under `pipefail`.**
PASS, not touched. `entries()` still ends `|| true` with the reason stated; the `grep`s
at 466, 521, 523 each carry a named `fail`. The new arm is a `case`, not a `grep`, so it
cannot introduce the no-match-is-not-an-error hazard.

**Item 4 — `LC_ALL`, and which direction it moves a guard.**
PASS, and this is the item the new arm turns on. `export LC_ALL=C` is at line 169, above
the guard. The pre-existing arm is written as an ALLOW-LIST (`*[![:print:]]*`), so the C
pin makes the refusal as WIDE as it can be — a narrower locale can only refuse more. I
verified the direction empirically rather than by reading: **U+2028 is refused**
(§1c), which is the exact character the checklist records as having walked through the
`[[:cntrl:]]` spelling. The new arm `*' '*` is locale-invariant: the space is `[:print:]`
in C and in every other locale, which is precisely why the allow-list could not catch it
and why a separate arm is the right shape. The commit's claim that the new arm "narrows
nothing" is correct — it is a second `case` pattern in the same `case`, and the first arm
is unmodified; §1c shows all three of its inputs still refused.

**Item 5 — the index is what commits; the working tree is not.**
NOT APPLICABLE to this change; no `git ls-files`, no path-versus-blob reading anywhere in
`baseline_snapshot.sh`. (The one gate in the repo that does this correctly,
`tools/file_justification_check.sh`, is untouched.)

**Item 6 — a sweep by prefix must own the prefix.**
PASS, not touched. `baseline_snapshot.sh` sweeps nothing: its only removal is
`rm -rf "$WORK"` where `$WORK` is its own `mktemp -d`. On the test side, the harness's
scratch sweep uses `SCRATCH_PREFIX = "pistol-testscratch-"`
(`crates/pistol-cli/tests/common/mod.rs`), the workspace-owned token the item requires;
the new test obtains its directories through `scratch(…)` and adds no prefix of its own.
I confirmed the scratch directories the mutation runs produced were all
`/tmp/pistol-testscratch-…`.

**Item 7 — traps.**
PASS, not touched. One `trap 'rm -rf "$WORK"' EXIT` at line 366, the only `trap` in the
file, installed immediately after the `mktemp -d` that creates its target. The new code
is above it and installs no second trap.

**Item 8 — one spelling per number, one refusal per reason.**
PASS. `count()` still validates the SPELLING of `--nodes`, `--ladder-depth`,
`--ladder-cap-s`. The one-refusal-per-reason discipline is *extended* by this commit
rather than eroded: a spaced name and a non-printable name are now two arms with two
messages, where one combined arm would have told a caller with a space that their name
was "outside printable ASCII" — a wrong diagnosis of the kind the item names. See F2 for
the one place the new *comment* states a criterion the code does not apply.

**Item 9 — what reaches a record is caller-controlled.**
PASS, and this is the item the commit closes. `--corpus` is caller-supplied
(`argument --corpus … ; CORPUS="$ARG"`, line 246), its basename is interpolated into a
whitespace-delimited field of the invariant block, and a space in it shifted every field
after it — reproduced at §2a. Both halves the item asks for are now present: the value is
guarded **at the boundary** (before `mktemp`, before the engine runs, before a byte is
written), and the refusal **quotes the input back** in backticks. The `basename` →
`${x##*/}` swap is the item's own literal instruction, and it removes the trailing-newline
strip the control-character refusal exists for. `$OPENINGS` gets the same guard; `$CONFIG`
does not — reachability of that gap is analysed in §7 and its comment in F2.

**Item 10 — THE COVERAGE RULE.**
PASS. `baseline_snapshot.sh` produces a recorded number and now carries
`a_corpus_name_carrying_a_space_is_refused_rather_than_shift_the_records_fields` in
`crates/pistol-cli/tests/baseline_snapshot_tests.rs` — a test in a suite CI runs (gate
3/14, confirmed in the `tools/ci.sh` log below), driving the **shipped** script through
`bash <root>/tools/baseline_snapshot.sh`, in scratch directories, **with a control run**.
I did not take the control run on trust: §2b mutates the guard to refuse everything and
the control half kills it. This is the item's requirement met in full.

**Item 11 — a caller's path that feeds a delete or an overwrite is containment-guarded.**
PASS. Enumerated from the file at `a102c6a`, not from memory. Every destructive or
writing site, with its target's ORIGIN:

| line | site | target | origin | class |
|---|---|---|---|---|
| 366 | `trap 'rm -rf "$WORK"' EXIT` | `$WORK` | `mktemp -d` (365) | script-created |
| 457 | `: >"$INVARIANT"`, `: >"$TIMING"` | `$WORK/invariant`, `$WORK/timing` | `$WORK/…` | script-created |
| 465 | `>"$WORK/hs"` | `$WORK/hs` | `$WORK/…` | script-created |
| 466 | `>"$WORK/id"` | `$WORK/id` | `$WORK/…` | script-created |
| 497 | `} >>"$INVARIANT"` | `$WORK/invariant` | `$WORK/…` | script-created |
| 500,502,504 | `>` / `>>"$WORK/corpus.session"` | `$WORK/corpus.session` | `$WORK/…` | script-created |
| 514 | `>"$WORK/corpus.out"` | `$WORK/corpus.out` | `$WORK/…` | script-created |
| 521,523 | `>"$WORK/corpus.totals"`, `>"$WORK/corpus.best"` | `$WORK/…` | `$WORK/…` | script-created |
| 545,609,648 | `>>"$INVARIANT"` | `$WORK/invariant` | `$WORK/…` | script-created |
| 546,611,656 | `>>"$TIMING"` | `$WORK/timing` | `$WORK/…` | script-created |
| 581 | `>"$out"` | `$WORK/ladder.$name` | `local out="$WORK/ladder.$name"`, `$name` a script literal (`opening`/`early_mid`/`late_mid`) | script-created |
| 689 | `} >"$WORK/record"` | `$WORK/record` | `$WORK/…` | script-created |
| **692** | **`cp "$WORK/record" "$OUT"`** | **`$OUT`** | **`--out`, caller-supplied** | **caller-supplied** |

There is no `mv` and no second `rm` in the file. Exactly one caller-supplied destructive
target exists — `$OUT` at line 692 — and it is the OVERWRITE instance the checklist item
names by name. Its remedy is present and is the one the item prescribes: `CALLER_PWD` is
captured at line 172 *before* the `cd "$ROOT"` at line 175, and lines 262–265 resolve a
relative `--out` against `CALLER_PWD` so the `cd` cannot silently redirect the caller's
path into the repository root. `b067d47` adds no destructive site and changes no target's
origin. `--corpus` and `--binary` are caller-supplied but are read-only inputs, never
`rm`/`mv`/write targets, so they fall outside this item (they are item 9's business, and
`--corpus` is what the commit closes there).

**Item 12 — a gate distinguishes RUN VOID from FAIL, by name.**
PARTIAL — obligation 1 met, obligation 2 not met, obligation 3 met for the new test only.
- *Obligation 1, a code per kind.* MET. The usage block now states, in the file rather
  than by inference from silence, that there is **no void class** and that `1` is the
  only refusal code. Declaring the absence is exactly what the item permits.
- *Obligation 2, preflight what the run needs and void early.* **NOT MET** — finding
  **F1**. The script writes scratch through `mktemp -d` into `$TMPDIR`, which on this
  machine is the 24 GiB tmpfs the item was written about, and it neither preflights nor
  calls `tools/scratch_preflight.sh` (only `tools/ci.sh` does, at its line 57). The
  usage block's own universal sentence is falsifiable in one command; see F1.
- *Obligation 3, the distinction survives the seam.* MET for the commit's own test —
  `assert_eq!(ran.status.code(), Some(1), …)` with a message saying what `0` and other
  codes would have meant. NOT met across the rest of the suite; see finding **F3**.

---

## 4. The `basename` → `${X##*/}` swap

**Claim A — "the guarded expression and the written expression are now the same one."**
UPHELD, with one precision. The guard is `case "${named##*/}"` where `named` iterates
`"$CORPUS" "$OPENINGS"`; the record writes `${CORPUS##*/}` and `${OPENINGS##*/}`. The
suffix-removal operator is character-for-character identical; the difference is the loop
binding's name, and I confirmed neither variable is reassigned between the guard (line
329) and the emit (lines 487–488): `CORPUS` is written only at line 183 (default) and
line 246 (`--corpus`), `OPENINGS` only at line 184. So guard and record read the same
value through the same expansion. No remaining path exists where the guard checks one
expression and the record writes another **for these two bindings**.

The one asymmetry left is `$CONFIG`: the record writes it raw at line 483 and the guard
loop does not cover it. That is deliberate, commented, and currently unreachable (§7),
but the comment's stated reason is inaccurate — finding **F2**.

**Claim B — "changed no recorded byte on an unspaced corpus."**
UPHELD, MEASURED, not inferred. In the worktree, the same corpus and flags were run
through the shipped script and through the `369d43a` script placed at the same path (so
`ROOT` resolves identically), and the invariant blocks — everything above `# timing` —
were diffed:

```
$ git show 369d43a:tools/baseline_snapshot.sh > tools/baseline_snapshot.sh
$ ./tools/baseline_snapshot.sh --corpus "$SP/drive/mini.txt" --nodes 1000 --ladder-depth 1 --out "$SP/old.record"   # exit 0
$ git checkout -- tools/baseline_snapshot.sh
$ ./tools/baseline_snapshot.sh --corpus "$SP/drive/mini.txt" --nodes 1000 --ladder-depth 1 --out "$SP/new.record"   # exit 0
$ awk '/^# timing/{exit} {print}' old.record > old.inv; awk '/^# timing/{exit} {print}' new.record > new.inv
$ diff old.inv new.inv && echo IDENTICAL
IDENTICAL
```

28 lines, byte-identical, including `corpus mini.txt sha256 … positions 2`.

**One latent divergence, blocked upstream — recorded, not a finding.** `${x##*/}` and
`basename` are *not* the same function on a path with a trailing slash:

```
$ bash -c 'X="a/b/"; echo "[${X##*/}] vs [$(basename "$X")]"'
[] vs [b]
```

An empty basename would slip *both* guard arms (neither `*[![:print:]]*` nor `*' '*`
matches the empty string) and would write `corpus  sha256 …` — a double space, i.e. the
very field shift this commit closes. It is **not reachable**: `[ -f "$CORPUS" ]` at line
289 runs before the guard and rejects any trailing-slash path, verified:

```
$ ./tools/baseline_snapshot.sh --corpus "$SP/drive/mini.txt/" …   ; echo exit=$?
exit=1
$ bash -c '[ -f "…/mini.txt/" ] && echo yes || echo no'
no
```

Recorded here so a future change that relaxes the `-f` check knows what it would unblock.

---

## 5. Over- and under-reach

**The engine's `--config` refusal rejects nothing that legitimately worked before, except
by design.** The predicate is Unicode `Cc` only (`char::is_control`), so the refusal is
exactly the set of paths whose `id config` echo would have been folded. Probed:

```
$ printf 'pistol\nquit\n' | ./target/release/pistol --config "$SP/cfg2/inst?v0.toml" | grep '^id config'
id config …/cfg2/inst?v0.toml
$ printf 'pistol\nquit\n' | ./target/release/pistol --config "$SP/cfg2/insté v0.toml" | grep '^id config'
id config …/cfg2/insté v0.toml
```

A literal `?`, a non-ASCII letter, and a SPACE all still work and echo verbatim — the
refusal is not a printable-ASCII allow-list smuggled in from the shell side, and the
engine's contract (a handshake VALUE may hold spaces) is preserved. A path with a control
character *did* work before and is now refused: that is the intended, documented trade.

**Under-reach: is anything else echoing a caller path into an identity line?** No.
`grep -rn '"config {' crates/ --include=*.rs` returns exactly one producer,
`crates/pistol-cli/src/bin/pistol.rs:157`, and it is the guarded one. Every other identity
line is an enum token, an integer, or a hex digest. `weights_file` is echoed only as a
digest, never as a path. `selftest` prints config paths through `println!`, not through
`one_line`, and emits no `id config` line, so it is outside this seam.

**The space guard rejects nothing beyond a spaced basename.** §1b is the direct control:
a space one directory up still runs. §1c confirms the other arm is unnarrowed.

**`travels_verbatim` and `one_line`'s early return cannot drift.** `one_line` does not
duplicate the condition — it *calls* the predicate:

```rust
pub fn travels_verbatim(text: &str) -> bool { !text.chars().any(char::is_control) }

fn one_line(text: &str) -> String {
    if travels_verbatim(text) { return text.to_string(); }
    …
}
```

One expression, one truth condition, and the doc claim ("whether `one_line` would leave
this text exactly as it is") is exactly what the early return does. This is the strongest
part of `a102c6a`: the cross-language analogue of the same-expression discipline
`b067d47` established inside the script.

**One ordering nit, not a finding.** `echoable_config` runs before
`only(&flags, &["--config"])`, so `pistol --config <ctrl> --bogus 1` now reports the
config refusal rather than the unknown flag. Both are exit 2 and both are named; no
behaviour a caller can be misled by.

---

## 6. Rule 3 (fail loud) and rule 9 (file cap)

**Rule 3.** Both new refusals are named, both quote the offending input, neither falls
back and neither swallows a status. The engine's refusal `escape_debug`s the path, so a
control byte cannot ride out on the refusal itself. The script's new arm is a `fail`
called as a statement, not inside a command substitution, so it can exit the script. The
one place where the fail-loud contract is *asserted* more broadly than it holds is F1.

**Rule 9.** `crates/pistol-cli/src/bin/pistol.rs` is 299 lines — under the 300 soft cap
the gate enforces (`SOFT_CAP=300`, `tools/file_justification_check.sh:60`), so no
justification comment is owed; `crates/pistol-cli/src/report.rs` is 185. No file over the
cap lost its justification: the gate reports **24 over the cap, all justified**, including
`tools/baseline_snapshot.sh` and `crates/pistol-cli/tests/baseline_snapshot_tests.rs`, both
of which grew in this work and both of which still carry a `RULE9-JUSTIFICATION:` comment
(script line 107, test line 42). Neither justification states a line count.

---

## 7. Reachability honesty (`a102c6a`'s claim)

**CLAIM: the control-character disagreement is NOT reachable through the shipped script
today. VERIFIED, and it is true more broadly than claimed.**

```
$ grep -n 'CONFIG=\|--config' tools/baseline_snapshot.sh
182:CONFIG="configs/instrument_v0.toml"
465:… "$BINARY" --config "$CONFIG" …
514:… "$BINARY" --config "$CONFIG" …
581:… "$BINARY" --config "$CONFIG" …
$ grep -n '\${[A-Z_]*:-\|PISTOL_' tools/baseline_snapshot.sh
(no output)
```

`CONFIG` is assigned once, unconditionally, from a literal. The flag loop (lines 242–251)
has arms for `--out`, `--nodes`, `--corpus`, `--ladder-depth`, `--ladder-cap-s`,
`--binary` and a `*) fail "unknown argument"` catch-all; there is no `--config` arm. There
is no `${CONFIG:-…}` and no environment read anywhere in the file, so not even an exported
variable can reach it. The claim holds.

Beyond the claim: `grep -rn -- '--config' tools/` shows the same pattern in every other
script that drives the engine — `tools/bench_delta.sh:99`, `tools/determinism.sh:39`,
`tools/movetime_check.sh:32` all set `CONFIG` from a literal, and `grep '\-\-config)'`
finds no flag arm in any of them. `tools/arena_smoke.sh` passes `--config` to the *arena*,
not to `pistol`. So the disagreement was unreachable through **every** shipped tools/
script, not only `baseline_snapshot.sh`. The commit understated its own safety margin;
nothing is hidden by it.

---

## Findings

### F1 — MINOR. The newly-added usage block's universal claim is false, and item 12 obligation 2 is not discharged

`b067d47` added to `tools/baseline_snapshot.sh`:

> `THERE IS NO VOID CLASS … every way of not writing one — a missing input, an engine
> that would not answer, a refused name — is a 1 with a named reason on stderr.`

A scratch failure is a way of not writing a record, exits 1, and has **no named reason**:

```
$ TMPDIR=/nonexistent-dir-for-review ./tools/baseline_snapshot.sh \
      --corpus "$SP/drive/mini.txt" --nodes 1000 --ladder-depth 1 --out "$SP/g.record"
exit=1
mktemp: failed to create directory via template '/nonexistent-dir-for-review/tmp.XXXXXXXXXX': No such file or directory
$ grep -c '^baseline_snapshot: FAIL' g.err
0
```

`WORK="$(mktemp -d)"` (line 365) carries no `|| fail`, so the diagnosis reaches the reader
in `mktemp`'s vocabulary — "discovering the shortage through a tool's own error message",
which SHELL_CHECKLIST item 12 obligation 2 names verbatim as the defect. The script writes
scratch into `$TMPDIR` and does not call `tools/scratch_preflight.sh`, which exists for
exactly this and which `tools/ci.sh` already uses at its line 57. On this machine `/tmp`
is a 24 GiB tmpfs that has been filled before (D-281, D-285), and it stood at 66 % during
this review.

Consequence, concretely: the *refusal* half of the new test is safe, because the guard
fires at line 329 before `mktemp` at line 365. The *control* half is not — a full `/tmp`
makes it fail on `assert!(ran.status.success(), "the snapshot script must succeed")`,
which reads in a log exactly like a regression in the guard. That is the reading failure
item 12 exists to prevent.

MINOR rather than MAJOR: no record is written wrongly, no guard is weakened, and
obligation 1's declaration is legitimately made. What is wrong is that the commit's own
new prose states a universal that one command falsifies, and it does so in the same
paragraph that cites item 12. Either preflight (`tools/scratch_preflight.sh` before
`mktemp -d`, or a `|| fail` on the `mktemp`), or narrow the sentence to the three cases it
actually enumerates.

### F2 — MINOR. The `$CONFIG` comment states a criterion the code does not apply

`b067d47` added, above the record's `config` line:

> `` `$CONFIG` is a constant of this script and no flag sets it, so it is not caller-named
> and the guard above does not cover it. ``

`$OPENINGS` is also a constant of this script that no flag sets — there is no
`--openings` arm in the flag loop — and the guard **does** cover it
(`for named in "$CORPUS" "$OPENINGS"`). So "not caller-named" is not the criterion the
loop applies; the criterion the loop actually applies is "reaches the record as a
whitespace-delimited field". Reproducer, textual and exact:

```
$ grep -n 'OPENINGS=' tools/baseline_snapshot.sh
184:OPENINGS="crates/pistol-cli/tests/fixtures/openings_v1.txt"
$ grep -n -- '--openings' tools/baseline_snapshot.sh
(no output)
$ sed -n '329p' tools/baseline_snapshot.sh
for named in "$CORPUS" "$OPENINGS"; do
```

This matters because this codebase treats such comments as binding on the next change —
the same comment ends "If a `--config` flag is ever added, this line joins that guard in
the same commit", which is the right binding. But a reader who takes the stated rule
("caller-named ⇒ guarded") literally will under-guard the next constant that starts
reaching the record. State the criterion the code uses. MINOR: no behaviour is wrong
today, and §7 confirms nothing is reachable.

### F3 — MINOR. The script now asserts a seam rule the suite does not keep

The usage block `b067d47` added ends:

> `A test that drives it therefore asserts the CODE and not merely` `!success`

The new test does. The rest of the suite does not: `Run::refusal()` — the shared helper —
still asserts `!ran.status.success()`, and it is used by **18** tests, against **1** test
asserting an exact code:

```
$ grep -c '\.refusal()' crates/pistol-cli/tests/baseline_snapshot_tests.rs
18
$ grep -c 'status.code(),' crates/pistol-cli/tests/baseline_snapshot_tests.rs
1
```

`Run::record()` likewise asserts `ran.status.success()` in 11 tests. Under `set -e` a
script death from any cause is non-zero, so those 18 tests report a bash death or a signal
as a satisfied refusal. This is pre-existing and is **not** a defect the commit
introduced; what the commit introduced is a sentence in the shipped script saying the
suite does something it does 1 time in 19. Either tighten `refusal()` to take an expected
code, or scope the sentence to the tests that meet it. MINOR: no test is currently wrong,
and the commit's own test is the one that matters for the guard under review.

---

## Verified with no finding

- **Both fixes hold when driven by hand against the shipped artefacts** — spaced basename
  refused at exit 1 with a named error (§1a); spaced directory NOT refused and producing a
  well-formed `corpus` line (§1b); LF, TAB and U+2028 basenames still refused by the
  unnarrowed printable arm (§1c); control-character `--config` refused at exit 2 with no
  `id config` line on either stream (§1d); `configs/instrument_v0.toml` echoing unchanged
  (§1e).
- **Both tests bind, independently verified against restored pre-fix files in a `/home`
  worktree** — `left: Some(0) / right: Some(1)` for the script test (§2a) and
  `left: Some(0) / right: Some(2)` for the handshake test (§2c), matching both commit
  messages exactly.
- **Both control halves are non-vacuous, verified by mutation** — a refuse-everything
  guard and a refuse-everything engine each kill their test (§2b, §2d).
- **The `basename` → `${X##*/}` swap changed no recorded byte** on an unspaced corpus:
  28 invariant lines, `diff` clean, measured old-script-vs-new-script in the worktree (§4).
- **Guard and record read one expression** for `$CORPUS` and `$OPENINGS`; neither variable
  is reassigned between the guard and the emit (§4).
- **`travels_verbatim` cannot drift from `one_line`** — `one_line` calls it rather than
  restating it (§5).
- **No over-reach in the engine guard** — `?`, non-ASCII and SPACE config paths still work
  and echo verbatim (§5).
- **No second producer of a `config <path>` identity line** exists in the workspace (§5).
- **Reachability claim is true, and true more broadly than stated** — no tools/ script has
  a `--config` flag, and `baseline_snapshot.sh` reads no environment variable (§7).
- **Rule 9 clean** — `pistol.rs` 299 and `report.rs` 185 are under the cap; the gate
  reports 24 files over the cap, all justified; no justification states a line count (§6).
- **Latent `${x##*/}` vs `basename` divergence on a trailing-slash path is unreachable**,
  blocked by the `[ -f "$CORPUS" ]` at line 289 — reproducer in §4, recorded rather than
  raised.

## Rejected, with the attempted reproducer

- **"An empty basename slips both guard arms and shifts the record's fields."** Real as an
  expression-level divergence (`[${X##*/}]` is empty where `basename` gives `b`), but
  **not reproducible against the shipped script**: every trailing-slash `--corpus` is
  rejected one check earlier. Attempted reproducer:
  `./tools/baseline_snapshot.sh --corpus "$SP/drive/mini.txt/" --nodes 1000 --ladder-depth 1 --out "$SP/f.record"`
  → `exit=1`, and `bash -c '[ -f "…/mini.txt/" ]'` → false. REJECTED as a finding,
  recorded in §4 as a latent divergence.
- **"The engine's new refusal narrows paths that legitimately worked."** Attempted
  reproducer: handshakes with `--config` at `inst?v0.toml`, `insté v0.toml` (space and
  non-ASCII) and the committed relative `configs/instrument_v0.toml`. All four exit 0 and
  echo `id config` verbatim. REJECTED.
- **"The `!said.contains(\"id config \")` assertion in the handshake test can never fire,
  because the refusal message itself contains `id config`."** Attempted reproducer: read
  the refusal string — it is ``the `id config` line``, a backtick where the assertion needs
  a space, so the substring does not occur; the live run at §1d greps 0 occurrences of
  `id config ` on both streams. The assertion is genuine (and would fire if a folded line
  were emitted). Fragile to a future edit that drops the backticks, but not a defect.
  REJECTED.
- **"The disagreement is reachable through another tools/ script that takes a
  `--config`."** Attempted reproducer: `grep -rn -- '--config' tools/` and
  `grep -n -- '--config)' tools/*.sh`. `bench_delta.sh`, `determinism.sh` and
  `movetime_check.sh` all set `CONFIG` from a literal and none has a flag arm;
  `arena_smoke.sh`'s `--config` goes to the arena binary. REJECTED.

---

## Gate table — `tools/ci.sh`, cited from its own log

Run at `a102c6a` in the live tree with `CARGO_TARGET_DIR` unset. **Not partial — all 14
gates.** Log at `…/scratchpad/ci.log`; `ci exit=0`.

```
ci: building 339 tracked files in /tmp/tmp.7b9zkmFyEs/repo
=== gate  1/14: cargo fmt --all --check
=== gate  2/14: build from the git-tracked file set
=== gate  3/14: cargo test --workspace --locked
=== gate  4/14: cargo clippy --workspace --all-targets -- -D clippy::all
=== gate  5/14: artifact rejection
=== gate  6/14: config validation
=== gate  7/14: perft oracle
=== gate  8/14: tactical fixture at its pre-registered threshold
=== gate  9/14: cross-process determinism
=== gate 10/14: differential search oracle
=== gate 11/14: movetime ceiling on the D-95 reproducer class
=== gate 12/14: arena self-match smoke
=== gate 13/14: file-justification check
=== gate 14/14: decision-key uniqueness
ci: all gates passed
```

Two gate outputs quoted directly, being the ones this review turns on:

```
file_justification_check: self-test passed on 8 seeded cases (cap 300)
file_justification_check: over the cap and justified: tools/baseline_snapshot.sh
file_justification_check: over the cap and justified: crates/pistol-cli/tests/baseline_snapshot_tests.rs
file_justification_check: 257 tracked .rs/.sh files, 24 over the cap, all justified

(gate 3/14, the suites under review)
baseline_snapshot_tests   test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
handshake_identity_tests  test result: ok.  3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
report_tests              test result: ok.  6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Housekeeping

- `git status --porcelain` is **empty** at the end of this review; no repository file was
  edited and no git write command was run in the live tree.
- The review worktree `/home/tom/.cache/review-trackC` was created with
  `git worktree add --detach … a102c6a` and removed with `git worktree remove --force`.
- **Note on `/home/tom/.cache/m4-measure`.** It was listed by `git worktree list` at the
  start of this review and is absent from the listing at the end. Its directory no longer
  exists on disk. `git worktree prune` never deletes an existing directory — it only
  clears admin metadata for a directory that is already gone — and `git worktree remove`
  was invoked with exactly one path, `review-trackC`. So that worktree's directory had
  already been removed by its own agent before my prune ran; my prune cleared the stale
  metadata it left behind. Nothing of its content was touched by this review. A different
  worktree, `/home/tom/.cache/redteam-m4-axisA` (detached at `b067d47`), appeared in the
  listing during this review; it is another session's and was not touched either.

---

**VERDICT: PASS**

**Findings: 0 BLOCKING, 0 MAJOR, 3 MINOR (F1, F2, F3).**

Both defects D-324 recorded OPEN are closed by code that does what its commit message
says, with tests that were independently confirmed to fail against the pre-fix sources
and controls that were independently confirmed to kill a refuse-everything mutant. All
three findings are in prose and in a test-suite seam, not in either guard; none blocks
landing. F1 is the one worth fixing soonest, because it is a false universal in a shipped
script's own usage block about the exact failure mode this machine has already suffered.
