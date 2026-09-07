# Pre-Phase-2 sweep — GROUP T, fresh-context CONFIRMATION of the fix round

## Header

**Revision confirmed:** `90a77a8` (dev HEAD at dispatch).
**Matches HEAD at close:** YES — `git status --porcelain` shows only the dispatcher's
own out-of-scope hunk (`docs/decisions.md`, the D-686/D-687 ADR lines appended after
dispatch); `git rev-parse HEAD` is unchanged at `90a77a89b90310e8fa365fb2baf050b17a52927f`.
**Fix-round commit read:** `9ce9a7c` ("fix(sweep): the three reviews' findings — gate 17
was red and nothing re-ran it, five configs kept a deleted key's comment, and a
wrong-partner mutant was surviving").
**Also read, as CLAUDE.md's Closure rule requires (a governing revision's own gates, not
a superseded citation):** `87305f7` (require_tool.sh conversion, D-683/D-685) and
`0fedfa8` (search/solver-proof test only; no group-T file touched), both between
`9ce9a7c` and `90a77a8`.
**Worktree:** `git worktree add --detach /home/tom/pistol-wt/confirm-t 90a77a8`, on
`/home` (930G nvme partition, 795G free at start), `CARGO_TARGET_DIR` never exported.
Removed with `git worktree remove --force` after copying its only untracked
files (`_test1.log`, `_test2.log`, already duplicated into `artifacts/`) — no other
modified or untracked files were present, so nothing was lost by the removal.

**Findings source:** `docs/experiments/pre_phase2_sweep_T_REVIEW.md` (revision under
review `cb6e853`), 14 findings.

---

## Per-finding table

| id | severity | verdict | reproducer re-run (this session) and quoted output | reasoning |
|---|---|---|---|---|
| BLOCKING-1 | BLOCKING | **CONFIRMED** | `./tools/file_justification_check.sh; echo EXIT=$?` at `90a77a8` → `file_justification_check: 413 tracked .rs/.sh/.py files, 87 over the cap, all registered in docs/rule9_justifications.md (87 entries)` / `EXIT=0`. Of the four originally-unregistered files: `crates/pistol-cli/src/random_openings/mod.rs` is now 300 lines (at, not over, the cap) and `crates/pistol-cli/tests/bench_delta_tests.rs` is now 294 (also under); `tools/file_justification_check.sh` (304 lines) and `tools/determinism.sh` (310 lines) each carry a new registry entry (`docs/rule9_justifications.md:107-108`, confirmed by `/usr/bin/grep -n`). Log: `t_blocking1_file_justification_check.log`, `t_blocking1_registry_state.log`. | The gate the package hardened, and that the package's own commits pushed red, is green at HEAD by the two shapes CLAUDE.md rule 9 allows (trim, or registry entry) — not by loosening the gate. |
| MAJOR-1 | MAJOR | **CONFIRMED** | Paired stub-cargo reproducer at HEAD, `TMPDIR=/nonexistent-preflight-probe`: `solver_determinism: RUN VOID: no scratch room; the lines above name the filesystem` / `EXIT=2`, and the stub's marker file is **absent** ("no marker: build did NOT run"). Source: `PREFLIGHT=` block now at `tools/solver_determinism.sh:49`, `cargo build` at `:70` — preflight before build. Log: `t_major1_solver_determinism_order.log`. | The void now fires before the build runs, matching `determinism.sh`'s already-correct order; a full filesystem now voids in the preflight's own vocabulary, not cargo's. |
| MAJOR-2 | MAJOR | **CONFIRMED** | Stub-cargo-fails-to-build reproducer at HEAD: `config_check: RUN VOID: the validate_config validator does not build; nothing was adjudicated and no document is implicated` / `EXIT=2` (not 1). The usage block now states a real void class with its own exit code 2 (`tools/config_check.sh:26-42`), and `config_check_gate_tests::a_validator_that_will_not_build_is_a_void_and_not_a_rejected_document` passed (`cargo test`, 1 of 5 new). Log: `t_major2_config_check_void.log`, `t_cargo_test_batch1.log`. | The build is now split from the run (`build_validator`), so a build failure is `void()` (exit 2) and never reaches `cargo run`'s conflated exit 1; the false "no void class" claim is replaced with an accurate one. |
| MAJOR-3 | MAJOR | **CONFIRMED** | `sed -n '83,95p' tools/bench_delta.sh` shows the usage block itself now reads: `Exit: 0 measured and verdict printed, 1 a precondition or the run failed. THERE IS NO VOID CLASS, stated rather than left to be inferred from silence (tools/SHELL_CHECKLIST.md item 12 obligation 1)...`. Log: `t_major3_minor3_bench_delta_and_gate4_line.log`. | The statement a reader consulting the usage block would see now exists in that block, not only in a preflight comment below it; the finding's complaint (the two words "void" only appeared past where a reader stops) is closed. |
| MAJOR-4 | MAJOR | **CONFIRMED** | Both cited mutations re-run in the worktree, tests restored after each: **M1** (drop `--all-targets`, keep `--locked` dropped too) → `clippy_gate_flag_tests`: 3/3 FAILED, message `gate 4 must lint tests and examples too, or the flags below reach less than this gate claims`. **M2** (`|| fail "clippy"` → `|| true`) → 3/3 FAILED, message `gate 4 must FAIL the run when clippy refuses; a gate whose refusal is swallowed is a gate that does not exist`. Unmutated suite: 3/3 ok. Log: `t_major4_clippy_gate_mutations.log`. | Both mutations the review demonstrated as invisible to the suite now turn it red; the test reads `--all-targets` and `|| fail` off the same harvested line rather than restating them, so the check and the checked stay one document. |
| MAJOR-5 | MAJOR | **FAIL** | The checklist's own re-derivation command, run verbatim: `git grep -l " totals " -- 'tools/*.sh' 'tools/**/*.sh' 'tools/*.py' 'crates/*/src'` finds 10 files (rows 5-11, 7 of 11 rows) and **cannot** reach rows 1-4 (`crates/pistol-arena/src/exchange.rs` ×2, `crates/pistol-arena/src/capture.rs`, `tools/sealbot/matchserver/src/pistol_client.rs`). Independently, `git grep -c "" -- 'crates/*/src'` returns **0** lines — the pathspec `crates/*/src` matches zero tracked files under this git (2.55.0); it needs a further glob segment (`crates/*/src/**` or `crates/**/*.rs`) to descend into the directory at all. Log: `t_major5_register_command_population.log`. | Part (a) of the finding — the missing eleventh consumer (`tools/staged_cover_bench.sh`) — is genuinely added to the table and independently re-derivable. Part (b) — "the register's own stated command could not have produced the table" — is **not discharged**: the new command does find row 8 and the new row 11 (closing the specific complaint cited), but it is checked against a **different wrong population** than before, silently dropping rows 1-4. The document's own new sentence, "The command above finds all eleven," is false when run. This is exactly the shape the verdict rule names: a fix that patches the cited instance while leaving the class (checked against the wrong population) open. |
| MAJOR-6 | MAJOR | **CONFIRMED** | `cargo test -p pistol-cli --locked --test file_justification_gate_tests`: 12/12 ok, including the new `the_shipped_gate_accepts_this_repository`, which runs `tools/file_justification_check.sh` against the real repository (not a scratch git repo) and asserts success + the `all registered in` summary line. Log: `t_cargo_test_batch1.log`. | The suite's two siblings already had this control; this one now does too, and it is a real observation — it is the same assertion that would have caught BLOCKING-1 at `42ab538` had it existed then. |
| MINOR-1 | MINOR | ~~CONFIRMED~~ → **FAIL, OVERTURNED — see ERRATUM 1** | `/usr/bin/grep -n "trap " tools/solver_determinism.sh tools/determinism.sh tools/file_justification_check.sh` → all three now read `trap 'rc=$?; rm -rf "$X"; exit "$rc"' EXIT` (lines 103, 134, 171 respectively). Log: `t_minor1_trap_forms.log`. | The form the finding asked for is present in all three, which is what this row checked — and it is the finding's SENTENCE, not its PROPERTY. Measured afterwards, the form does not preserve the status under `set -e`. |
| MINOR-2 | MINOR | **CONFIRMED** | `/usr/bin/grep -c '^cargo test '` and `'^echo '` on `tools/search_oracle_check.sh` → 7 and 4, matching the finding. `docs/experiments/pre_phase2_sweep_CLOSURE.md:365-367` now reads "SEVEN (`grep -c '^cargo test '` — the closure first said six, counted by eye)". Log: `t_minor2_search_oracle_count.log`. | The document's own arithmetic is corrected and cites the counting command, not "by eye"; the underlying conclusion (item 10's letter does not bind this script) is unchanged and was already agreed sound. |
| MINOR-3 | MINOR | **CONFIRMED** | `/usr/bin/grep -n "gate 4\|cargo clippy --workspace" tools/ci.sh` → both the `step` log line and the executed command now read `--all-targets --locked`. Log: `t_major3_minor3_bench_delta_and_gate4_line.log`. | The printed description now matches the command it describes; a gate claim citing this log line is no longer understating the gate. |
| MINOR-4 | MINOR | **CONFIRMED** | Real run, `./tools/puzzle_corpus/tests/run_tests.sh`: the T1/T2/NEG section prints "all checks passed" with **no** `FAIL: extractor tests` line (the old `grep -v '^  ok'`-exits-1-on-full-pass shape is gone; the script now checks the Python process's own exit status and uses `grep -v ... || true` only to trim noise from a captured log). The script's overall exit is 1, but for an unrelated, pre-existing reason: the T4 determinism section needs a `--offline` cache this fresh worktree does not have populated (`7d7ver4: no cache entry and --offline was given`), not part of this finding and not introduced by the fix. Log: `t_minor4_puzzle_corpus_run_tests.log`. | The specific false-FAIL shape the finding named is gone; the unrelated cache-miss failure is an environment gap in a fresh worktree, not a group-T regression. |
| MINOR-5 | MINOR | **CONFIRMED** (accepted-rather-than-fixed, both halves) | `docs/experiments/pre_phase2_sweep_CLOSURE.md:425-433` now records the reviewer's cheaper option verbatim ("AND THERE IS A CHEAPER OPTION... measured by the group-T reviewer with a control... Recorded so the resume starts from two options, not one.") without adopting it. `tools/ci.sh:108-113` adds the toolchain-exposure sentence the review suggested ("no `rust-toolchain` file is pinned... Accepted: ... a lint break on a toolchain bump is loud and local."). Log: excerpts in `t_major3_minor3_bench_delta_and_gate4_line.log`'s neighboring context and quoted above. | Neither half names a way the code can produce a wrong answer (D-424): the actual A-19 instance is fixed at the source (`policy.rs`, import moved inside `#[cfg(debug_assertions)]`); what remains open is a documented, honestly-priced gap for a **future** instance, which is exactly what "recorded as owed" is for. |
| MINOR-6 | MINOR | **CONFIRMED** | `tools/file_justification_check.sh:132-134` now reads `lines="$(awk 'END { print NR }' <"$bytes")"`. Direct reproducer: a 301-line file (300 newline-terminated + 1 unterminated) reads as `300` under `wc -l` (the old method, "under" cap) and `301` under `awk 'END{print NR}'` (the new method, "over" cap). Log: `t_minor6_unterminated_line_count.log`. | The exact escape the finding demonstrated is closed for the cap measurement; the registry-line reader already handled this case separately and is unchanged. |
| MINOR-7 | MINOR | **CONFIRMED** | `tools/solver_oracle_check.sh:37-45` now carries the same preflight block (`scratch_preflight.sh` over both `${TMPDIR:-/tmp}` and `$ROOT`) as its siblings, ahead of `OUT="$(mktemp)"`. Log: `t_minor7_solver_oracle_check.log` (full file). See also NEW-1 below. | The `mktemp -d` sweep's blind spot (a scratch *file*, not a directory) is closed for this script specifically. |

---

## NEW FINDINGS (introduced by, or newly visible in, `9ce9a7c`)

### NEW-1 — MAJOR — the register's "fixed" re-derivation command is itself checked against the wrong population

This is the detail behind the MAJOR-5 FAIL above, stated as its own defect because the
text is new (not a carry-over) and independently reproducible.

`tools/SHELL_CHECKLIST.md`'s appendix used to cite, at `cb6e853`, a command run **over
the whole `crates` and `tools` trees** (`git grep -n "info totals\|' totals '\|" totals ""
… over crates and tools`) — wrong *pattern*, correct *scope*. `9ce9a7c` replaces it with:

```
git grep -n " totals " -- 'tools/*.sh' 'tools/**/*.sh' 'tools/*.py' 'crates/*/src'
```

— correct pattern, but a narrowed *scope* that introduces a fresh instance of the same
defect class the edit exists to close. Reproducer (git 2.55.0, at `90a77a8`):

```
$ git grep -c "" -- 'crates/*/src' | wc -l
0
```

The pathspec `crates/*/src` matches zero tracked files: `*` does not cross a `/`, so
`crates/*/src` names only the directory path `crates/<one-segment>/src` and, being a
directory with no further pattern, matches nothing under `git grep`'s pathspec rules — it
needed a trailing `/**` or `/*.rs`. Consequently the four rows the register itself lists
as living under `crates/pistol-arena/src/` or `tools/sealbot/matchserver/src/` (rows 1-4)
are unreachable by the command the document says "finds all eleven." A grammar change
someone runs the stated command against would silently check 7 of 11 sites and get a
clean bill.

Reproducer file: `artifacts/pre_phase2_confirm/t_major5_register_command_population.log`.

**Fix shape (not applied — outside a reviewer's remit):** `crates/**/*.rs` (verified
above to reach all of rows 1-3) plus a pattern that reaches `tools/**/*.rs` for row 4,
which none of the four stated globs cover at all, in either the old or new command.

### NEW-2 — MINOR — `tools/solver_oracle_check.sh`'s scratch trap is still the bare form the fix applied to its siblings

`9ce9a7c` gives this script the same preflight block (and hence the same void-class
reachability) as `solver_determinism.sh`, `determinism.sh` and
`file_justification_check.sh`, but its cleanup trap at line 48 remains
`trap 'rm -f "$OUT"' EXIT` — the pre-`rc=$?`-preserving form MINOR-1 fixed in the other
three. This script was not one of MINOR-1's three named scripts (it wasn't changed by any
T-package commit at `cb6e853`, so it was outside that finding's search), but `9ce9a7c`
itself increases this file's void-class exposure right above the same bare trap. If
`rm -f "$OUT"` ever fails (e.g. an unremovable parent directory), a chosen `void`(2) or
`fail`(1) from lines 41/43-44/55/59/64/67 would be overwritten by `rm`'s own status —
the exact MINOR-1 class, reachability low, in a file the fix round touched but did not
carry the same idiom into.

Reproducer file: `artifacts/pre_phase2_confirm/t_minor7_solver_oracle_check.log` (full
script; line 48).

---

## Also verified clean at HEAD (not separately claimed by any of the 14 findings, carried over from the review's own coverage)

- Full re-run, unmutated: `clippy_gate_flag_tests` (3), `config_check_gate_tests` (5),
  `decision_key_check_tests` (10), `file_justification_gate_tests` (12),
  `label_consistency_check_tests` (13), `solver_determinism_gate_tests` (5),
  `bench_delta_tests` (4) — all green (`t_cargo_test_batch1.log`).
- Gates run directly and green at HEAD: `file_justification_check.sh` (exit 0),
  `decision_key_check.sh` (687 keys, exit 0), `label_consistency_check.sh` (6 documents,
  exit 0), `config_check.sh` (45 documents, exit 0), `governing_citation_check.sh`
  (exit 0), `artifact_check.sh` (exit 0) — `t_config_gov_artifact_checks.log`.
- `87305f7`'s `require_tool.sh` conversion does not reopen MAJOR-1 or MAJOR-2: the
  preflight-before-build order in `solver_determinism.sh` and `determinism.sh` is
  unchanged by it (`require_tool.sh` only resolves a name via `type`/`command -v`, never
  executes the tool, confirmed by reading it in full), and it does not touch the `info
  totals` appendix section of `tools/SHELL_CHECKLIST.md` at all (confirmed by
  `git show 87305f7 -- tools/SHELL_CHECKLIST.md`).
- `0fedfa8` touches no group-T file (`crates/pistol-search/src/quiescence.rs`,
  `crates/pistol-search/src/search.rs`, and the CLOSURE doc only).

## Not re-verified (out of this pass's remit or infeasible in the time available)

- A full `tools/ci.sh` 21-gate run was not taken (as the original T review also did not);
  the individual gates it wraps for group T were run directly instead, all green.
- Gates 8, 10, 11, 12, 14, 15, 16, 18 (untouched by group T) were not run.
- `tools/determinism.sh` and `tools/solver_determinism.sh` end-to-end with the real
  engine were not re-timed; only their preflight-vs-build ORDER was re-driven (which is
  what MAJOR-1 is about).

---

## Group verdict

One of six MAJOR findings (MAJOR-5) does not hold at `90a77a8`: its own replacement text
is independently falsifiable by running the exact command it prints, and it fails for the
same reason (wrong population) the original finding named, just against a different
subset of rows. BLOCKING-1 and the other five MAJORs, plus all seven MINORs, are
CONFIRMED with a re-run reproducer. Two new findings surface from the fix diff itself:
NEW-1 (MAJOR, the mechanism behind the MAJOR-5 FAIL) and NEW-2 (MINOR, a sibling instance
of MINOR-1's class the fix round did not carry over).

**GROUP T: FAIL — MAJOR-5**


---

## ERRATUM 1 — MINOR-1's verdict is overturned, by the dispatcher, after this report closed

**This report scored MINOR-1 CONFIRMED and it is a FAIL.** The row above checked
that the three named scripts carry `rc=$?` … `exit "$rc"`, which is what item 7
prescribes and what the finding asked for. It did not check that the form WORKS.

MEASURED afterwards (bash 5.3.15, a script exiting 2 whose cleanup `rm` is
refused, `set -euo pipefail` throughout):

| trap body | cleanup fails | cleanup succeeds |
|---|---|---|
| `rm -rf "$W"` (bare, pre-fix) | **1** | 2 |
| `rc=$?; rm -rf "$W"; exit "$rc"` (the fix) | **1** | 2 |
| `cleanup() { local rc=$?; rm -rf "$W"; return "$rc"; }` | **1** | 2 |
| `rc=$?; rm -rf "$W" \|\| echo … >&2; exit "$rc"` | **2** | 2 |

Under `set -e` the failing `rm` terminates the shell where it stands, so the
`exit "$rc"` added to preserve the status is **never reached** — the fix and the
defect are one behaviour, and the fix round changed nothing in any case. Without
`set -e` the status survives all four, which is the reading that made the wrong
mechanism look right.

**THE LESSON IS THIS REPORT'S OWN SUBJECT MATTER, TURNED ON ITSELF.** Every
finding in the group-T review was a claim checked against the wrong thing, and
this row checked a remedy against the rule that prescribed it rather than against
the behaviour the rule exists to produce. A verdict that reads the fix instead of
running it is the same defect one level up. The reproducer was four lines of
bash and would have taken a minute.

Fixed under `docs/decisions.md` D-689: all fourteen trap sites in `tools/`
converted to the guarded form, and `tools/SHELL_CHECKLIST.md` item 7 rewritten
with the measured table, because the rule was wrong and not only its application.
NEW-2 in this report (the one unconverted script) is closed by the same change.
