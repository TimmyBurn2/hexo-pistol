# tools/wp21_assemble.py — REVIEW, ROUND 3, SCOPED to the round-2 remedy diff, against tools/SHELL_CHECKLIST.md

**Revision reviewed**: `5b17132` — the diff `git diff 9c4366c 5b17132 -- tools/wp21_assemble.py
crates/pistol-arena/tests/wp21_assemble_tests.rs` (the round-2 reviewer's executed 30-line patch for
N1/N2/N5/N7, N3's wording, N4's two assertions, N6's two messages). **HEAD at review start**:
`c4963b3` (branch `arc3-stopped`) — does NOT match `5b17132`, and `git diff --stat 5b17132 HEAD --
tools/wp21_assemble.py crates/pistol-arena/tests/wp21_assemble_tests.rs` is EMPTY: both files are
byte-identical from `5b17132` to HEAD (script sha256
`a367d84754162b65e24bbff98b8a791dc3e9d2678e7f653a71457e4853fee39b`), so every finding holds at HEAD.
**Round 2**: `wp21_assemble_REVIEW_round2.md` at `9c4366c`, PASS 0 B / 0 M / 7 m (+1 nit).
**Scope**: exactly that diff, per docs/process.md (a change to an instrument reopens its review).
Nothing outside the diff is re-adjudicated; round 2's dispositions of F1–F13 stand.

**VERDICT: PASS — BLOCKING 0, MAJOR 0, minor 2 (plus 4 nits).** All seven round-2 minors N1–N7
are CLOSED, each by execution against the shipped script. Both surviving round-2 mutants (m4, m7)
are now KILLED, each by the test round 2 named for it. The suite is 11 of 11. The pilot reproduces
`742 / 347 / 191 / 0.5504 / 0`, byte-identically twice, with the same two manifest digests round 2
recorded. The two minors are two new exit paths the diff's own additions open — one spells a
refusal as exit 1 with a traceback (the class N1 closed), one spells a completed run as a VOID —
neither writes a wrong number, neither writes a wrong file. No exit-0-wrong-answer was found.

---

## METHOD

Per D-590/D-591/D-592, everything below was RUN in a detached worktree
`/home/tom/pistol-wt/asm-review3` at `5b17132` with `CARGO_TARGET_DIR=/home/tom/pistol-wt/asm-review3/target`
per command, never exported, never the live tree; scratch under the worktree, nothing on `/tmp`.
Python `3.14.7`. The suite: `cargo test -p pistol-arena --locked --test wp21_assemble_tests` →
`test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 24.54s`
(log sha256 `a60a8af143ed9dac3…`). Hand reproducers were driven against `scratch/pristine.py`, a copy
extracted with `git show 5b17132:tools/wp21_assemble.py` whose sha256 equals the worktree file's
(`a367d847…`, checked before the reproducers and again after the mutant runs restored the script) —
a copy because the mutation step patches the worktree's own file in place. Corpora were written by a
generator of my own (`scratch/gen.py`, the tree's fixture shape, body digest computed; the injected
code points verified as `U+2028`, `U+0085`, `U+2029`, `U+00A0`, `U+0009` by reading the files back).
The pilot corpus is `/home/tom/pistol-runs/wp20pilot-artifacts/corpus_v1.txt`, sha256
`493f4fa8b6fb3e395555a578e480917eb3ae05e53727953aae6febe873af9c4f`. Fault injection was by a
`sitecustomize.py` on `PYTHONPATH` (patching `os.statvfs` / `pathlib.Path.read_text`), the shipped
script otherwise untouched — round 2's technique. A control run (`in/good.txt` + `in/good2.txt`)
exits 0 with 2 files, so no refusal below comes from a script that refuses everything.

**The pilot once** (question 4): `corpora 1 / records 742 / distinct positions 347 / decided 191 /
outcome coverage 0.5504 / key disagreements 0`, exit 0 — **reproduced**. A second run into a second
directory: `cmp` silent on both manifests. Each manifest's claimed `# body_sha256` re-computed with
`sed '1,/^# body_sha256 /d' | sha256sum`: raw `0022220f55161d281930fa0c1b951b41819044ee76727aba37ff7e71a17a7e85`,
deduped `0a1001acd7ff196eaa5151b2921f5bff3c16e035e7c0003e55455fe16e8b5b0c` — both MATCH the claim and
both equal round 2's digests, so the diff changed no byte of either output. Round 2's awk over the
pilot: `records 742`, `distinct 347` — agree. The pilot given twice: exit 2, `has the same body digest
as corpus 1`, 0 files.

---

## N1–N7: DOES THE DIFF CLOSE EACH? (question 1)

| # | round-2 minor | diff hunk | closed? | command (against the pristine copy) → output |
|---|---|---|---|---|
| N1 | `text.index("\n", at)` raises `ValueError`, traceback, exit 1 | `body_of`: `find`, `body = "" if end < 0` | **CLOSED** | `--corpus in/nobody_nonl.txt` (the `# body_sha256 <sha("")>` line is the file's last, no trailing newline) → exit **2**, `RUN VOID: nobody_nonl.txt holds no records`, 0 files. Same file with a trailing newline → exit 2, same void. |
| N2 | `label_go` / `capture_sha256` reach a manifest unguarded | `receipt_safe()` split out of `readable()`, applied to both header values in `corpus_of` | **CLOSED** | `captab.txt` (capture `dead<TAB>beef`) → exit 2, `the captab.txt capture_sha256 carries a character a receipt cannot hold: 'dead\tbeef'`. `goinject2.txt` (label_go `go nodes 400000<U+2028># derived decided 999`) → exit 2 naming `label_go`, value quoted back as `'… # derived decided 999'`. Also refused: `U+0085` (`gonel`), `U+2029` (`gops`), `U+00A0` in the capture (`capnbsp`). 0 files in each. The pilot's manifests are byte-identical to round 2's, so the guard admits every value the pipeline writes. |
| N3 | header said "Anything the corpus reader refuses" | header lines 37–47 reworded: "…refuses THAT A COUNT HERE DEPENDS ON: …The reader's other refusals — a missing param, a key's token shape, `to_move`, `book`, `score_kind` — are not re-checked, because no number printed here reads those fields" | **CLOSED** (one wording nit, below) | The sentence now matches the behaviour it describes: `noexp.txt` (no `experiment_sha256` param) → exit 0; `keypos.txt` (`key_pos` = `zz`) → exit 0 — both the cases round 2 listed as accepted, and both now named as not re-checked. |
| N4 | m4 and m7 survived 11/11 | two assertions: each manifest's claimed digest vs `sha256_hex(body_of(text))` in `two_corpora_assemble_…`; a `rebuild()`ed copy with record 0's depth set to `7` (new body digest, same capture) expecting `same capture` in `the_same_corpus_given_twice_…` | **CLOSED** — see the mutant re-takes below | m4 → `FAILED. 10 passed; 1 failed`, the failing test `two_corpora_assemble_into_a_raw_manifest_naming_each_ones_own_digest_and_count`, panic `raw_manifest.txt's header names bytes it does not hold` (:205). m7 → `FAILED. 10 passed; 1 failed`, the failing test `the_same_corpus_given_twice_is_a_void_naming_both`, panic `exit Some(0) — 0 is \`both manifests written\`, …` (:495, via `void_named` → `meaning()`). |
| N5 | no filesystem preflight (item 12(2)) | `os.statvfs(out_dir)` after both texts are rendered and before either file is claimed; `f_bavail * f_frsize < wanted` → VOID naming both numbers; `OSError` → VOID `cannot preflight` | **CLOSED** | Injected `f_bavail = 0`: exit 2, `the filesystem holding \`out/inj_bavail0\` has 0 bytes available and the two manifests want 1347`, **0 files** (so the preflight fires before any claim). Injected `statvfs` → `EIO`: exit 2, `cannot preflight the output directory … [Errno 5] Input/output error`, 0 files. Boundary: `wanted` for the control pair MEASURED as 1347 bytes (`wc -c` over both files); injected `available == 1347` → exit 0, 2 files; `available == 1346` → exit 2 naming `1346 … want 1347`. The comparison is bytes to bytes, strict, and correct at the edge. |
| N6 | two bare `assert_eq!(…, Some(0))` (item 12(3)) | both bound to an `Output` and given `meaning(&…)` as the message | **CLOSED** (by reading the diff — the hunk is the mechanical change round 2 specified; the suite compiles and runs it 11/11) | `wp21_assemble_tests.rs:474–477` now read `let first = assemble(…); assert_eq!(first.status.code(), Some(0), "{}", meaning(&first));` and the same for `second`. No bare `Some(0)` assert remains in the file: `/usr/bin/grep -n 'Some(0)'` over it finds nine, every one carrying `"{}", meaning(&…)`. |
| N7 | forced legacy stdout encoding → `UnicodeEncodeError` after both writes, exit 1 | `stream.reconfigure(errors="backslashreplace")` on stdout and stderr before the `try` | **CLOSED** (and it opens minor m1, below) | `PYTHONIOENCODING=ascii python3 pristine.py --out-dir out/日本 --corpus in/good.txt` → exit **0**, 2 files, the final lines print the path as `out/日本/raw_manifest.txt`. Control at `9c4366c` (`git show 9c4366c:tools/wp21_assemble.py`): exit 1, `UnicodeEncodeError`, 2 files — round 2's reproducer confirmed, and closed. |

---

## SHELL_CHECKLIST ITEMS, BY NAME, OVER THE DIFF (question 2)

- **1 (a status DISCARDED)** — the diff's `os.statvfs` is a statement inside `try`/`except OSError`
  → named VOID (measured, EIO). `find` returns `-1`, which the next line checks — the value is
  taken and its shape tested, as the item asks. The two `reconfigure` calls are statements whose
  failure is caught by nothing: they sit ABOVE the `try` (minor m1). Otherwise clean.
- **2 (pipeline in a condition)** — n/a.
- **3 (`grep` under pipefail)** — n/a; the Python analogue (`find` → `-1` as a normal answer) is
  handled explicitly.
- **4 (locale; which direction a guard moves)** — the diff's purpose. `isprintable()` is an
  allow-list over the Unicode database, locale-independent, now applied to two more values; measured
  refusing `U+0009`, `U+0085`, `U+00A0`, `U+2028`, `U+2029` in both, admitting the pipeline's own
  values (pilot byte-identical). `reconfigure(errors="backslashreplace")` closes the stdout residue
  (N7) and is measured to survive a pipe (`| cat` → exit 0), `/dev/null` (exit 0) and a legacy
  encoding. The `LINE_BREAKERS` tuple stays redundant with `isprintable()` (round 2 said so; harmless).
- **5 (index vs worktree)** — n/a; no git state read.
- **6 (sweep by prefix)** — the diff adds no delete.
- **7 (traps)** — the `except Exception` backstop is the Python analogue of an EXIT trap, and the
  three things the prompt asked were measured: **`SystemExit(REFUSED)` passes through** — a
  pre-existing `raw_manifest.txt` → exit **1**, `REFUSED: … already exists and is not overwritten`,
  the pre-existing file untouched (1 file); a pre-existing `deduped_manifest.txt` → exit 1, the
  fresh raw removed (only the deduped remains). **`KeyboardInterrupt` passes through** — injected
  from `Path.read_text`: the process dies by SIGINT (`returncode -2` under a subprocess wrapper; my
  own shell batch was cut short by it the first time, which is the same fact), the traceback ends in
  `KeyboardInterrupt`, no `RUN VOID` on stderr, 0 files. A **generic exception** (injected
  `TypeError` from `os.statvfs`) → traceback, then `RUN VOID: the instrument failed, see the
  traceback above`, exit **2**, 0 files. `except Exception` does not catch `BaseException`
  subclasses, and the measurements agree. Two wrinkles, both counted below: the reconfigure loop is
  outside the trap (m1) and the trap has no way to know whether the pair was already written (m2).
- **8 (one spelling per number)** — the preflight compares `f_bavail * f_frsize` bytes to
  `len(text.encode("utf-8"))` bytes, boundary measured exact. On a bind mount `statvfs` reports the
  bound superblock's numbers (this machine's `/home`, `/var/log`, `/var/cache/pacman/pkg` are btrfs
  subvolume mounts of `/` and report `/`'s counts — surveyed over `/proc/mounts`), which is the right
  filesystem. Root-reserved blocks: `f_bavail` is what THIS uid may use, so as non-root the guard is
  exact; as root it refuses a write root could make — a false VOID, named with numbers, not a wrong
  answer; accepted. A filesystem that reports NO statistics is refused as a full one (nit n2). The
  two guarded header values are shape-unchecked (nit n4 — pre-existing, and the header now says so).
- **9 (what reaches a record is caller-controlled)** — closed (N2). The values are quoted back in
  the refusal in `repr`, which is what the item asks.
- **10 (THE COVERAGE RULE)** — the two new assertions kill m4 and m7 (measured, below); every
  registered number keeps its round-2 test; the suite drives the SHIPPED script. The preflight and
  the backstop themselves have no test — neither produces a recorded number, and a `statvfs` fault
  needs an injected filesystem the Rust suite cannot make; not a finding.
- **11 (a caller's path feeding a delete or an overwrite)** — the diff adds one filesystem call,
  `os.statvfs(out_dir)`, which reads. The destructive-site enumeration of round 2 (two `os.unlink`
  over `created`, both script-created under `O_EXCL`) is unchanged. Clean.
- **12 (RUN VOID vs FAIL, by name)** — (1) codes: the diff REMOVES the N1 exit-1 traceback and the
  N7 exit-1-after-success, and ADDS two mis-spelled paths of its own — m1 (exit 1 with a traceback
  when a standard stream is closed) and m2 (exit 2 "VOID" with both manifests correctly on disk).
  Every injected fault (bavail 0, statvfs EIO, TypeError) is a VOID by name with 0 files. (2)
  preflight: present, placed after render and before claim, and measured to fire with 0 files.
  (3) the seam: N6 closed; the two new assertions carry messages (m4's names the manifest and the
  defect; m7's goes through `void_named` → `meaning()`, which spells all three codes).

---

## MUTANT RE-TAKES (question 3)

Applied to the worktree's `tools/wp21_assemble.py` at `5b17132`, suite run with the worktree's
`CARGO_TARGET_DIR`, script restored with `git checkout --` and `git diff --quiet` confirmed after
each; the restored file's sha256 is `a367d847…`, the pristine copy's.

| mutant | diff applied | round 2 | round 3 |
|---|---|---|---|
| m4 | `manifest()`: `digest = hashlib.sha256((body + "x").encode("utf-8")).hexdigest()` | SURVIVED 11/11 | **KILLED** — `test result: FAILED. 10 passed; 1 failed` (24.7 s), `two_corpora_assemble_into_a_raw_manifest_naming_each_ones_own_digest_and_count` panicked at `:205`: `raw_manifest.txt's header names bytes it does not hold` (log sha256 `9f6f5bf5bceec83e0…`) |
| m7 | `if capture in seen_capture:` → `if False and capture in seen_capture:` | SURVIVED 11/11 | **KILLED** — `test result: FAILED. 10 passed; 1 failed` (24.7 s), `the_same_corpus_given_twice_is_a_void_naming_both` panicked at `:495`: `exit Some(0) — 0 is \`both manifests written\`, 1 is \`REFUSED, an output exists\`, 2 is \`RUN VOID, an input is not a corpus\`` (log sha256 `641239cde9d7c7489…`) |

The two N4 assertions are the right shape: the first verifies the claim with `pistol_cli`'s own
`body_of`/`claimed_body_digest`/`sha256_hex` — a reader independent of the script's split, and the
same helpers `records()` already trusts for corpora; the second keeps the capture header and
re-digests the body (`rebuild()` does exactly that, `:135–154`), so it reaches the capture branch
and not the digest branch — confirmed by which mutant it kills.

---

## NEW FINDINGS — against the diff only

### m1 — minor — a closed standard stream is exit 1 with a traceback, before the `try` (items 1, 7, 12(1))
`sys.stdout` is `None` when fd 1 is closed at interpreter start, and the diff's
`for stream in (sys.stdout, sys.stderr): stream.reconfigure(…)` runs ABOVE the `try`. Reproducer:
`python3 pristine.py --out-dir out/closed --corpus in/good.txt >&-` → **exit 1**,
`AttributeError: 'NoneType' object has no attribute 'reconfigure'`, 0 files; `2>&-` → exit 1, 0 files.
Control at `9c4366c`: `>&-` → exit **0**, 2 files (`print` to a `None` stream is a silent no-op).
So the N7 remedy turned a working run into a traceback with the REFUSED code — the exact class N1
was closed for, one line above the backstop that closes it. Contrived (a driver closing fd 1 is
odd; `>/dev/null` and a pipe are both measured fine), no wrong number, no file written; minor.
**FIX (unexecuted, one line)**: `if stream is not None: stream.reconfigure(…)`, or move the loop
inside the `try` so the backstop spells it as a VOID.

### m2 — minor — the backstop spells a COMPLETED run as a VOID when reporting fails (item 12(1))
The usage block says exit 2 means "no manifest was written and no answer was taken". Reproducer:
`python3 -u pristine.py --out-dir out/full --corpus in/good.txt >/dev/full` → the first `say()`
after `write_pair` raises `OSError: [Errno 28]`, the backstop prints `RUN VOID: the instrument
failed, see the traceback above`, **exit 2, 2 files** — both manifests complete and correct (their
digests are inside them). At `9c4366c` the same run was an uncaught traceback, exit 1 — also wrong,
so the diff re-spells a pre-existing wrong code rather than opening a new window; but the new
spelling is the one the checklist says sends a reader to the wrong place (a "void" whose answer is
on disk; a re-run then REFUSES on the existing pair). Without `-u` the same stdout failure is
Python's own exit **120** at shutdown (`Exception ignored while flushing sys.stdout`), outside any
code the script owns — recorded, not this diff's. Needs an unbuffered stdout on a full disk or a
broken pipe; minor. **FIX (unexecuted)**: a module-level flag set at the end of `write_pair`, read by
the backstop: with it set, say "both manifests were written; the failure is in reporting them" and
do not print the word VOID — the code is the operator's call (it is neither 0 nor 2 as documented).

### nits — recorded, not counted
- **n1** — header line 39–40, "a depth that is not one": the ellipsis reads as "depth ≠ 1"; the
  script accepts depth 3 (control run). "a depth spelled as anything but a non-negative decimal
  integer" is what the code does (`:167–172`).
- **n2** — a filesystem reporting no statistics is refused as full. Surveyed over `/proc/mounts`: two
  FUSE mounts on this machine (`/run/user/1000/gvfs`, `/run/user/1000/doc`) report `f_blocks 0,
  f_bavail 0`; `--out-dir` on either → exit 2, `has 0 bytes available and the two manifests want
  1347`. Neither can actually take a file (`touch` → `No such file or directory`), so the VOID is
  moot here, but a FUSE filesystem without `statfs` would be refused while writable. A `f_blocks ==
  0` → "no statistics, not preflighted" branch would distinguish unknown from full. A false VOID with
  its numbers printed, never a wrong answer.
- **n3** — `--help` exits 0 without writing (carried from round 2's nit; unchanged by the diff).
- **n4** — `receipt_safe` is an injection guard, not a shape guard: an EMPTY `capture_sha256`
  (`capempty.txt`) → exit 0 and the raw manifest's one row ends in an empty fifth column
  (`1<TAB>capempty.txt<TAB>c970…<TAB>3<TAB>` measured with `cat -A`); an empty `label_go` → exit 0
  and `# param label_go ` with nothing after it in both headers; `dead beef` (a space) and `#dead`
  → exit 0. None injects a row or a column, which is all the guard's docstring claims; the reader
  (`labels_file.rs:162–168`, `strip_prefix` then `trim`) accepts the same values, so the script's
  new header sentence ("a key's token shape … not re-checked") covers it honestly. The pipeline
  writes 64 hex there. Pre-existing; an item-8 shape check (64 lowercase hex; `go nodes <integer>`)
  would be the remedy if a successor wants it.

---

## ATTACKS ATTEMPTED AND REJECTED

- **Does the backstop swallow `SystemExit(REFUSED)`?** No — measured exit 1 with the REFUSED line
  and the pre-existing file untouched, both directions (raw exists → nothing created; deduped exists
  → the fresh raw removed). `except Exception` does not see `SystemExit`.
- **Does it swallow `KeyboardInterrupt`?** No — measured death by SIGINT (`-2`), no `RUN VOID`.
- **Does the preflight VOID after a claim?** No — injected `f_bavail 0` leaves 0 files; the
  `statvfs` sits between `manifest()` twice and `write_pair`, and the write path is unchanged from
  round 2's F11 (ENOSPC mid-write → both removed).
- **Does `reconfigure` exist on a pipe / a file / `/dev/null`?** Yes — all three are
  `TextIOWrapper`s; measured exit 0 with 2 files for each. The one stream Python hands the script
  that lacks it is `None` (fd closed) — m1.
- **Did the guard's widening change any output?** No — the pilot's two manifests are byte-identical
  to round 2's digests.
- **`find` vs `index` semantic drift**: for a body marker with a newline after it the two agree
  (every accepted corpus); the only divergence is the no-newline tail, where `find` yields the empty
  body the file actually has and the digest check adjudicates it. Rejected.
- **Item 6 / item 11 over the diff**: no new delete, no new write target; `statvfs` reads.
- **`label_go` with a `\r`**: still neutralised by `read_text`'s universal newlines (round 2); not
  re-run, since the diff touches nothing on that path.

---

## STATE OF THE RECORD

- The worktree `/home/tom/pistol-wt/asm-review3` held no `artifacts/` directory and its `sessions/`
  was the one TRACKED file (checked out, not produced); its logs — `suite_run1.log`
  (`a60a8af143ed9dac3…`), `mutant_m4.log` (`9f6f5bf5bceec83e0…`), `mutant_m7.log`
  (`641239cde9d7c7489…`) — were removed with it; the load-bearing lines are quoted above, as rounds
  1 and 2 did.
- Nothing in the live tree was written except this file.

**HEAD at review end**: `735fc37` on `dev` (the live tree changed branch from `arc3-stopped` at `c4963b3` during the review) ≠ `5b17132`; `git diff --quiet 5b17132 735fc37 -- <the two files>` is silent, so the two files reviewed are byte-identical at start-HEAD, end-HEAD and the revision reviewed.
