# tools/wp21_assemble.py — REVIEW-impl + RED-TEAM, ROUND 2, against tools/SHELL_CHECKLIST.md

**Revision reviewed**: `9c4366c030d0c48f85f3d5a09066e149ba978f9b` (dev). **HEAD at review
end**: `037b196` — does NOT match; two commits landed under the review (`eb3dc6c`, `610225b`,
`037b196`). `git diff --quiet 9c4366c 037b196 -- tools/wp21_assemble.py
crates/pistol-arena/tests/wp21_assemble_tests.rs docs/rule9_justifications.md` is silent:
**the three files under review are byte-identical at every revision from `9c4366c` to
`037b196`** (script sha256 `8dfdb19c2f0699c3…` at both), so every finding below holds at
HEAD. `eb3dc6c` deleted `wp21_prereg_rev5_DRAFT.md` and landed it as `wp21_prereg.md`
revision 5; `diff` of §6 between the draft at `9c4366c` and the landed file at `037b196`
is empty, so the F1 disposition holds at HEAD too.
**Round 1**: `wp21_assemble_REVIEW.md` at `5c77a8f`, FAIL 0 B / 6 M / 7 m.
**Governing text**: `wp21_prereg.md` rev 5 §6, D-560, D-562, D-590..592,
`crates/pistol-arena/src/labels_file.rs`, `docs/process.md`.

**VERDICT: PASS — BLOCKING 0, MAJOR 0, minor 7 (plus 1 nit).** All thirteen round-1
findings are CLOSED, each by execution. No exit-0 wrong number was found; no D-562(2)
representative is wrong. The seven minors are: one remaining traceback path that exits
with the REFUSED code, two header values that reach a manifest unguarded, a header
sentence that overclaims what the script refuses, two recorded numbers no test defends
(one mutant survives for each), no filesystem preflight, and two bare `Some(0)` asserts in
the suite. Five of the seven are closed by a 30-line patch that I executed against the
suite (11 of 11) and against each reproducer; its diff is appended.

---

## METHOD

Per D-590/D-591/D-592, everything below was RUN, in a detached worktree
`/home/tom/pistol-wt/asm-review2` at `9c4366c` with its own `CARGO_TARGET_DIR`, never the
live tree. The suite: `CARGO_TARGET_DIR=…/asm-review2/target cargo test -p pistol-arena
--locked --test wp21_assemble_tests` → `test result: ok. 11 passed; 0 failed` (39.55 s,
rustc as installed). Hand reproducers were driven against a pristine copy of the script
extracted with `git show 9c4366c:tools/wp21_assemble.py`, whose sha256
`8dfdb19c2f0699c3e4bc1b1aaf5c8e47ec1b92d2f9a634e85feca0a581e61a2f` equals the live-tree
file's — a copy because the mutation runner patches the worktree's own copy in place, and
my first pass at three reproducers raced it (a `decided 0` that was mutant m2, not the
script); every number below is from the re-take against the pristine copy. Corpora were
written by a generator of my own (`scratch/gen.py`, 37 corpora in the tree's fixture
shape, body digest computed), not the suite's fixture and not round 1's `mk.py`. The
pilot corpus is `/home/tom/pistol-runs/wp20pilot-artifacts/corpus_v1.txt`, sha256
`493f4fa8…` as `wp20_pilot_artifacts.md` records.

**Re-derivation (docs/process.md), with awk I chose, scope `NR` after the body marker:**

```
awk 'f{n++} /^# body_sha256 /{f=1} END{print n}'                                    → 742
awk -F'\t' 'f{t[$4 FS $5 FS $6]=1;s[$4]=1;p[$5]=1;u[$6]=1} … END{print length(t),…}' → 347 347 347 347
raw decided / raw records                                                           → 414 / 742 = 0.5580
deduped decided (deeper wins, tie first) / distinct                                 → 191 / 347 = 0.5504
same-triple pairs disagreeing on result / on depth                                  → 32 / 0
records with search_nodes == 0 (proof depth)                                        → 0
order-free key disagreements (distinct triple sharing any single key)               → 0
```

The script over the pilot once: `records 742 / distinct positions 347 / decided 191 /
outcome coverage 0.5504 / key disagreements 0`, exit 0 — **agreement on every count**.
Two runs into two directories: `cmp` silent on both manifests — deterministic. Both
manifests' own `# body_sha256` re-computed with `sed … | sha256sum`: MATCH (raw
`0022220f…`, deduped `0a1001ac…`). Every deduped row names corpus 1 (single corpus). The
deduped manifest is 165 304 bytes for 347 rows → about **54 MiB at the sweep's ~119 800**
(ESTIMATED by scaling; the row width is the key strings').

**Mutation runs of the SUITE** (seven mutants of the shipped script, applied in the
worktree, suite run, script restored from git each time):

| mutant | what it breaks | result |
|---|---|---|
| m1 decided + denominator over RAW records | F1/F2's alternative reading | KILLED by `decided_and_outcome_coverage_…` |
| m2 `decided` always 0 | F2's "prints 0.0000 and passes" | KILLED by `decided_and_outcome_coverage_…` |
| m3 last record wins (`elif depth > held[2]` → `else`) | F12 | KILLED by 4 tests, both deeper tests among them |
| m4 manifest self-digest wrong (`sha256(body + "x")`) | the recorded `body sha256` | **SURVIVED 11/11** — N4 |
| m5 order-dependent disagreements (round 1's definition) | F6 | KILLED by `…does_not_depend_on_the_order…` and `…kept_distinct` |
| m6 tie to LAST (`>` → `>=`) | D-562(2)'s tie clause | KILLED by `the_deduped_manifest_holds_one_row…` and coverage |
| m7 capture-digest duplicate check removed (`if False:`) | F5's second half | **SURVIVED 11/11** — N4 |

---

## DISPOSITION OF F1–F13

| # | round-1 finding | disposition | command run (against the pristine copy unless noted) and output |
|---|---|---|---|
| F1 | §6 does not say which denominator `outcome_coverage` has | **CLOSED (document)** | `wp21_prereg_rev5_DRAFT.md` §6 at `9c4366c`, and identically `wp21_prereg.md` rev 5 §6 at `eb3dc6c`/`037b196`: *"**outcome coverage** = decided / distinct"*, with `decided` defined in the same sentence and `distinct positions` the row count. One clause, in the section that owns it; `grep -n -iE 'coverage\|56 ?%\|0\.55'` over the draft finds no second statement and no comparison with D-562's raw 56 %. The script's header (line 30–35) and its manifest header line (`outcome_coverage: decided over DEDUPED positions, not over raw records`) say the same thing; the script prints ONE number, as round 1 asked. Mutant m1 (raw denominator) is killed. |
| F2 | `decided` / `outcome_coverage` had no test (item 10) | **CLOSED** | `decided_and_outcome_coverage_are_tallied_over_the_deduped_representatives` rewrites game 0 of corpus b to `p1_win`, tallies from each ROW's named record (not from the row), asserts `0 < decided < rows`, `decided` and `format!("{:.4}", decided/rows)`. The tie interaction is exercised: b's game-0 records at k=0 and k=1 tie with a's and LOSE, so a decided record that loses the tie must not count — m1 and m2 both die here. |
| F3 | unreadable input/output → traceback exit 1 | **CLOSED** (residual → N1) | `in/latin1.txt` (0xe9 in header): exit **2**, `RUN VOID: latin1.txt is not UTF-8: 'utf-8' codec can't decode byte 0xe9 in position 6`. `in/noperm.txt` mode 000: exit **2**, `noperm.txt cannot be read: [Errno 13] Permission denied`. `--out-dir out/ro` mode 500: exit **2**, `cannot create out/ro/raw_manifest.txt: [Errno 13]`, 0 files. The optional top-level catch round 1 suggested was NOT adopted, and one traceback path remains (N1). |
| F4 | accepts what `labels_file::read` refuses | **CLOSED** (header overclaims → N3) | `in/v2.txt`: exit 2, `is corpus schema 2, and this instrument reads schema 1`. `in/res.txt` (`P1_WIN`): exit 2, `record 1: \`P1_WIN\` is not a result this corpus writes`. `in/emptyfield.txt`: exit 2, `record 1 carries an empty field`. Also `end Normal` (same file, record 2) is behind record 1's refusal; the suite's `a_corpus_the_reader_would_refuse_…` covers `P1_WIN`, an empty `key_seq`, schema 2, `label_go`, non-UTF-8 and no `--corpus`. |
| F5 | same corpus twice doubles `records` | **CLOSED** (one path untested → N4) | `--corpus good --corpus good`: exit 2, `corpus 2 (good.txt) has the same body digest as corpus 1: one tranche given twice would count twice`. Same body under a different header (`good_otherhdr.txt`): exit 2, same refusal. Different body, same `capture_sha256` (`good_samecap.txt`): exit 2, `was derived from the same capture as corpus 1`. Pilot twice: exit 2, 0 files — the ledger §2c shakedown line (`corpus_v1.txt` twice, 1 484 records) is now an input the shipped script refuses, as round 1 said it would have to. |
| F6 | `key_disagreements` order-dependent | **CLOSED** | `o1.txt` (A,B,C) and `o2.txt` (C,A,B), A and C sharing `key_seq`: both `key disagreements 2` (A and C; B shares nothing — round 1's "1 vs 2" was the old definition). Pilot: 0, equals my awk. Mutant m5 (old definition) killed by two tests. Definition stated once in the script header, once in the manifest header, once in §6, all three agreeing: *distinct positions sharing at least one key value with another distinct position*. |
| F7 | `label_go` not required to agree | **CLOSED** | `good.txt` (`go nodes 400000`) + `go.txt` (`go nodes 5000`): exit 2, `corpus 2 (go.txt) was labelled at \`go nodes 5000\` and corpus 1 at \`go nodes 400000\`: one training input cannot carry two teachers`. Suite covers it (`two teachers`). |
| F8 | NEL / LINE SEPARATOR in a basename pass (item 4) | **CLOSED** | Basenames created and passed from Python so the shell cannot mangle them: `x\x85y`, `x y`, `x y`, `x\xa0y`, `x\ty`, `x\ny`, `x\x0by` — all exit 2, `the corpus path carries a character a receipt cannot hold: 'in8/x y.txt'` (each quoted back in repr). `café.txt` and `日本.txt`: exit 0, raw manifest `split("\n")` 8 lines / `splitlines()` 7 — consistent. The guard is `str.isprintable()`, an ALLOW-list; the explicit `LINE_BREAKERS` tuple is redundant with it (all three are non-printable) but harmless. |
| F9 | argparse exits 2 without `RUN VOID` | **CLOSED** (nit below) | `--out-dir out` alone: exit 2, `RUN VOID: arguments: the following arguments are required: --corpus`. |
| F10 | blank body lines counted in `record_line` | **CLOSED** | `blank.txt` (record, empty line, record): exit 2, `carries an empty line inside its body, after record 1`. With that void, the script's numbering and `labels_file::read`'s (`filter(!is_empty)`) coincide on every accepted corpus; the manifest column is now `record_number`. §6 still says "record line" — after F10 both words name the same integer, so this is not a distinction (D-424) and I do not file it. |
| F11 | write failure after both claims leaves a pair | **CLOSED** | Fault-injected (`scratch/enospc2.py`: `os.fdopen` patched so the SECOND handle's `write` raises `ENOSPC`), shipped script otherwise untouched: exit **2**, `RUN VOID: writing the manifests failed and both were removed: [Errno 28] No space left on device`, `ls out_f11 | wc -l` → 0. Both texts are rendered before either claim (`manifest()` twice at :300–344, `write_pair` at :345). The other direction — second file exists → first removed — is the suite's `an_existing_manifest_is_refused_…`, and the raw-first REFUSED path creates nothing. |
| F12 | deeper-wins test alone admits "last record wins" | **CLOSED** | `the_deeper_label_wins_whichever_corpus_holds_it` puts depth 9 in corpus a, record 1 — the FIRST record seen — and asserts `("1","1","9")`. Mutant m3 (last wins) is killed by both deeper tests plus two more; m6 (tie to last) by two. |
| F13 | rule-9 entry's "splitting would copy the fixture" untrue | **CLOSED** | `docs/rule9_justifications.md:91` now argues what round 1 said was true — every case builds the same three-stage fixture because a hand-written corpus would let the script agree with a fixture rather than the pipeline; the dedup cases are worthless without the control run and the refusal cases without a run that writes, "one argument … both in one place". States no line count. |

---

## SHELL_CHECKLIST ITEMS, BY NAME

- **1 (discarded substitution status)** — Python; every read and every `os.open` is a
  statement whose failure is a named `Void` (`text_of`, `claim`) — F3's three tracebacks
  are gone. One `ValueError` remains uncaught (N1). Otherwise clean.
- **2 (pipeline in a condition)** — n/a.
- **3 (`grep` under pipefail; substring vs token)** — n/a for grep. Header markers are
  matched with `startswith` at line start and each must occur exactly once
  (`twomarker.txt`: exit 2, `carries 2 \`# body_sha256\` line(s)`); `text.index(BODY_MARKER)`
  still finds a mid-line occurrence first, and the result is a digest VOID, never a
  wrong body. Survives.
- **4 (locale; which direction a guard moves)** — the basename guard is now an
  allow-list (`isprintable()`), Unicode-database based, locale-independent: refused NEL,
  LS, PS, NBSP, TAB, LF, VT; admitted `é` and CJK. Reads and writes pin `encoding="utf-8"`.
  The one locale-shaped residue is stdout (N7): under a forced legacy stdout encoding a
  non-ASCII `--out-dir` name raises `UnicodeEncodeError` in the final `say()` AFTER both
  manifests are written — exit 1 with two good files on disk. Under `LC_ALL=C` Python
  coerces to UTF-8 and it does not happen (measured: exit 0).
- **5 (index vs worktree)** — n/a; the script reads no git state.
- **6 (sweep by prefix)** — no sweep. The only unlinks are of paths this run created
  under `O_EXCL`. Clean.
- **7 (traps)** — n/a; the Python equivalent is `write_pair`'s cleanup, whose exit is the
  `Void`'s 2 or the REFUSED 1 after the unlink, never the unlink's own status. One
  theoretical wrinkle: an `OSError` from the REFUSED branch's `os.unlink` would be caught
  by the enclosing `except OSError` and re-spelled as a VOID (exit 2 for what was a
  refusal); it needs the directory to become unwritable between create and unlink.
  Rejected as unreachable in practice, recorded so nobody re-finds it.
- **8 (one spelling per number, one refusal per reason)** — depth: `+3`, `03`, ` 3`,
  `3 `, `-1`, `3.0`, `1_0`, `٣` (Arabic-Indic three, which `int()` accepts) all exit 2
  naming the spelling. `result`/`end` are closed sets. `readable()` gives one message for
  four reasons (missing, directory, FIFO, socket: "is not a regular file") — the path is
  quoted, a reader can `ls` it; round 1 accepted this and so do I.
- **9 (what reaches a record is caller-controlled)** — `path.name`: guarded (F8). Two
  header values are NOT: `capture_sha256` reaches a raw-manifest ROW and `label_go`
  reaches both manifests' HEADER unguarded — N2, reproduced both ways.
- **10 (THE COVERAGE RULE)** — every §6-registered number now has a test against an
  independent tally, and the tests drive the SHIPPED script
  (`repo().join("tools/wp21_assemble.py")`, four exit-0 controls); five of seven mutants
  die. Two recorded-but-not-registered outputs have no test: the manifests' own
  `body_sha256` (m4 survives) and the `capture_sha256`-duplicate VOID (m7 survives) — N4.
  CI runs the suite (`tools/ci.sh` gate 3 is `cargo test --workspace --locked`).
- **11 (caller path feeding a delete or an overwrite)** — enumeration of every destructive
  site, traced to origin: `os.unlink(made)` at :237 (REFUSED branch) and :249 (OSError
  branch), both over `created`, a list the script appends to ONLY after
  `os.open(… O_CREAT|O_EXCL)` succeeded on `out_dir / RAW_NAME` or `out_dir / DEDUPED_NAME`
  — fixed basenames, script-created this run. No `rm`, no `mv`, no overwrite: the
  exclusive create is the overwrite guard (`an_existing_manifest_is_refused_…` pins it;
  a pre-existing `raw_manifest.txt` is REFUSED with nothing created). `--out-dir` relative
  paths resolve against the caller's cwd; there is no `cd`. A caller-supplied `out_dir`
  that is a symlink is followed — into a directory where only NEW files are created.
  Clean.
- **12 (VOID vs FAIL by name)** — (1) codes: 0 / 1 REFUSED / 2 VOID are documented in the
  usage block and kept on every path I could reach except two: N1 (a `ValueError`
  traceback, exit 1) and N7 (an encode error after success, exit 1); `--help` exits 0
  without writing (nit). (2) preflight: **none** — the script discovers a full filesystem
  through the OS's error text after claiming both files (cleanly, F11), which is the
  item's "the tool's vocabulary, not the gate's" — N5, with an executed `statvfs` fix.
  (3) the seam: `meaning()` spells all three codes in every failure message — except two
  bare `assert_eq!(…status.code(), Some(0))` at suite lines 460–461, the item's own named
  anti-pattern — N6.

---

## NEW FINDINGS

### N1 — minor — one traceback path remains, and it exits 1, the REFUSED code (items 12(1), 8)
`in/nobody_nonl.txt`: a corpus whose `# body_sha256 <sha of "">` line is the file's LAST
line with no trailing newline. `body_of()` :123 does `text.index("\n", at)`, finds none,
and raises `ValueError: substring not found` — uncaught, **exit 1**, traceback. The same
file WITH a trailing newline VOIDs correctly (`holds no records`, exit 2). A degenerate
input (the writer never produces it), so minor; but round 1's F3 recommended, as an
option, a top-level `except Exception → VOID` precisely so that no path can reach exit 1
without the word REFUSED, and the remedy did not take it. **FIX, EXECUTED** (appendix):
`find` instead of `index` (→ `holds no records`, exit 2, reproduced) plus the backstop.

### N2 — minor — `label_go` and `capture_sha256` reach a manifest unguarded (item 9)
Two reproducers, both exit 0:
- `in/captab.txt`, header `# derived capture_sha256 dead<TAB>beef`: the raw manifest's one
  row has **6** TAB-separated fields (`awk -F'\t' '{print NF}'`) where the columns line
  declares 5.
- `in/goinject2.txt`, header `# param label_go go nodes 400000<U+2028># derived decided 999`:
  both manifests carry that value in `# param label_go …`, which precedes the derived
  lines. Read back with `str.splitlines()`, the first `# derived decided` line is
  **`999`**; with `split("\n")` it is the true `2`. (`\r` in the same place is neutralised
  by `Path.read_text`'s universal-newline translation — measured — so the header's
  `LINE_BREAKERS` comment is exactly right about which characters matter.)

A pipeline corpus cannot carry either (`label_go` is `go nodes <integer>`, the capture
digest is hex), so the blast radius is a doctored corpus; but the script guards
`path.name` for this reason and not the two header values beside it. **FIX, EXECUTED**
(appendix): the same allow-list applied to both values — `captab` → exit 2 `the
captab.txt capture_sha256 carries a character a receipt cannot hold: 'dead\tbeef'`;
`goinject2` → exit 2 naming `label_go`; the pilot's deduped manifest byte-identical
before and after the patch.

### N3 — minor — the header says "Anything the corpus reader refuses" and the script refuses five of the reader's twelve
`labels_file::read` also refuses: a missing `experiment_sha256` / `source_sha256` /
`opening_turns` / `score_units` / `score_sign` / `mate_counts` / `depth_meaning` param
(`in/noexp.txt`: **exit 0**, records 3); `key_pos` not 32 hex digits (`in/keypos.txt`,
`zz`: **exit 0**, distinct 1); `score_kind` outside its three (`CP`: exit 0); `to_move`
outside `{p1,p2}` (`P1`: exit 0); `book` outside `{yes,no}` (`maybe`: exit 0); and the
`key_seq`/`key_full` token shapes and negative mate counts (not run). None of these
touches a registered number — the keys are compared as strings and the numbers printed
are right for the records present — so this is a false sentence in the header, not a
wrong answer, and it is minor. **FIX (wording, unexecuted)**: line 37–40 should say
*"what the corpus reader refuses THAT A COUNT DEPENDS ON: …"*, or the script should call
the reader (`corpus-check` exists at `crates/pistol-cli/src/bin/corpus-check.rs`) — the
latter buys one grammar at the price of a built binary the script must find.

### N4 — minor — two recorded outputs no test defends (item 10)
Mutant m4 (each manifest's own `# body_sha256` computed over `body + "x"`) passes all
eleven tests; mutant m7 (the `capture_sha256`-duplicate VOID deleted) passes all eleven.
The first is the identity the closure will cite for each manifest — a wrong one is
refused by any fixture-shape reader, so it is a loud failure later rather than a silent
one, hence minor. The second is half of F5's remedy, unpinned. **FIX (unexecuted)**: in
`two_corpora_assemble_…`, verify each manifest's claimed digest against
`sha256_hex(body_of(text))` as `records()` already does for corpora; in
`the_same_corpus_given_twice_…`, add a `rebuild()`ed copy whose body differs by one depth
and whose header keeps the capture digest, expecting `same capture`.

### N5 — minor — no preflight of the output filesystem (item 12(2))
The script writes ~54 MiB at the sweep's scale (ESTIMATED above) and asks nothing before
claiming both files; a shortage is found through `[Errno 28]` and cleaned up (F11). Item
12(2) asks for the void BEFORE the work, naming the filesystem, what is available and what
was wanted. **FIX, EXECUTED** (appendix): `os.statvfs(out_dir)` after both texts are
rendered, `f_bavail * f_frsize < wanted` → VOID naming both numbers; suite 11 of 11 with
the patch.

### N6 — minor — two asserts in the suite report a void as a regression (item 12(3))
`wp21_assemble_tests.rs:460–461`:
```
assert_eq!(assemble(&forward, &[&a, &doctored]).status.code(), Some(0));
assert_eq!(assemble(&backward, &[&doctored, &a]).status.code(), Some(0));
```
— the checklist's own quoted anti-pattern, in the one test that lacks `meaning()`. A void
here (python3 missing, a full `TMPDIR`) reads as the order-dependence regressing. **FIX
(unexecuted, mechanical)**: bind each `Output` and pass `meaning(&output)` as the message,
as the other nine call sites do.

### N7 — minor — a forced legacy stdout encoding exits 1 after both manifests are written (item 4)
`PYTHONIOENCODING=ascii python3 … --out-dir out/日本 --corpus in/good.txt`: **exit 1**,
`UnicodeEncodeError` from the final `say(f"raw manifest {raw_path} …")`, **2 files
written**. Contrived — needs a forced non-UTF-8 stdout AND a non-ASCII output path; under
`LC_ALL=C` Python's own coercion makes it exit 0 (measured). Filed because the failure
mode is the worst shape: success on disk, REFUSED in the log. **FIX, EXECUTED**
(appendix): `stream.reconfigure(errors="backslashreplace")` on both streams → exit 0.

### nit — `--help` exits 0
`python3 wp21_assemble.py --help` → exit 0 with no manifest written; the usage block says 0
is "both manifests written". No tranche log will contain it. Recorded, not counted.

---

## WHAT SURVIVED ATTACK

- **D-562(2)'s rule, exactly, and now pinned from both sides.** Deeper wins wherever it
  sits (two tests; m3 dies four times); ties go to the FIRST (m6 dies twice); a decided
  record that loses a depth tie does not count (`tie_loses.txt`: representative `capped`,
  decided 0); a deeper capped record beats a shallower decided one (`proof.txt`: depth 4
  capped over depth 2 win, decided 0) — by the rule as written.
- **The pilot reproduces the ledger and my awk on every count**, twice, byte-identically.
- **The three-key exact triple**: transposition (`key_pos` differs, `key_seq` equal) is
  kept distinct and both members counted; `key_disagreements` is order-free (A,B,C =
  C,A,B = 2).
- **Every refusal is a VOID by name, exit 2, nothing written**: non-UTF-8, unreadable,
  schema 2, empty field, bad result/end, eight depth spellings, blank body line, CRLF
  (digest), two markers, duplicate body, duplicate capture, two `label_go` values, missing
  `--corpus`, seven kinds of unprintable basename, unwritable out-dir.
- **The exclusive pair**: injected ENOSPC on the second write → exit 2 and both removed;
  existing deduped → exit 1 and the fresh raw removed (suite); existing raw → exit 1,
  nothing created.
- **Rounding**: Python `f"{x:.4f}"` and Rust `{:.4}` agree at exact binary ties
  (1/32 → `0.0312`, 3/32 → `0.0938`, 5/32 → `0.1562`, 1/160 → `0.0063`; `rustc -O`
  program in scratch), so the coverage test cannot flake on a tie.
- **`int()`'s permissiveness** (`1_0`, Arabic-Indic digits, leading `+`, whitespace) is
  closed by the `str(depth) == field` round-trip.

## ATTACKS ATTEMPTED AND REJECTED

- **`\r` in `label_go` injecting a header line**: neutralised by `read_text`'s universal
  newlines (the CR becomes a line break in the CORPUS, and the forged line is an ignored
  header comment there). Only the three Unicode breakers survive, which is N2.
- **NUL or an undecodable byte in argv**: `isprintable()` is False for NUL and for
  surrogate-escaped bytes → VOID before `Path.is_file()` can raise. Rejected.
- **Division by zero in coverage**: `records_of` VOIDs on zero records, so
  `distinct ≥ 1`. Rejected (as round 1).
- **A body marker mid-line in a header comment**: digest VOID, never a wrong body. Rejected.
- **`--corpus ./a.txt --corpus a.txt`** (one file, two spellings): same digest → VOID.
  Rejected.
- **Symlinked `--out-dir`**: followed; only new files are created there. Rejected.
- **Item 6 scratch naming**: `Scratch` uses `pistol-testscratch-arena-`. Rejected.
- **Rule 9 for the script itself** (364 lines, no `rule9_justifications.md` entry): the
  gate `tools/file_justification_check.sh` enumerates `.rs` and `.sh` only (its header,
  D-234), and six other `tools/*.py` over the cap carry no entry either — a tree-wide
  question about the gate's scope, not this script's. Recorded, not filed.
- **D-562(2)'s "deeper" across a proof depth and a search depth**: the corpus header
  says `depth_turns` is a proof depth where `search_nodes` is zero, and the rule compares
  the two as one number — the script does what the rule says (`proof.txt` above). The
  pilot has **0** proof-depth records, so nothing turns on it today; it is the ADR's
  question if a solver-labelled sweep ever meets a search-labelled one, not this
  instrument's. Recorded for the successor, not filed.

---

## STATE OF THE RECORD

- The ledger §2c shakedown line (`corpus_v1.txt` twice, 1 484 records) describes an input
  the shipped script now VOIDs; the counts it quotes (347 / 191 / 0.5504 / 0) are
  reproduced by the pilot ONCE. A successor quoting §2c should quote the once-run.
- The ledger's state table names the round-2 suite log as `…_test_r5.log` at one row and
  `…_test_r4.log` at another; not adjudicated here.
- My worktree's logs — suite `cargo_test_run1.log` (sha256 `e5c6d297ac9e3316…`), mutants
  `mutant_m4…log` (`ca29f2c54835cbea…`), `mutant_m7…log` (`b9a787b4dcf6dfef…`), patched
  suite `patched_suite.log` (`b9d16cc41d42ac0e…`) — were removed with the worktree, which
  held no `artifacts/` and whose `sessions/` was the one TRACKED file
  (`sessions/WP-1.8b/2026-08-28-WP18B-CLOSED-bench-abort.md`, checked out, not produced);
  the logs' load-bearing lines are quoted above, as round 1's were.

---

## APPENDIX — THE EXECUTED FIX (N1, N2, N5, N7), 30 lines, suite 11 of 11 with it applied

Applied to the worktree's `tools/wp21_assemble.py` at `9c4366c`, `cargo test -p
pistol-arena --locked --test wp21_assemble_tests` → `test result: ok. 11 passed; 0 failed`
(29.68 s); reproducers `nobody_nonl` → exit 2 `holds no records`, `captab` → exit 2,
`goinject2` → exit 2, `PYTHONIOENCODING=ascii` → exit 0 with 2 files, the pilot's deduped
manifest byte-identical to the unpatched script's. Then the worktree copy was restored
from git (`git diff --quiet` silent). This is a claim EXECUTED, per D-591; N3, N4 and N6
are unexecuted and labelled so above.

```diff
--- i/tools/wp21_assemble.py
+++ w/tools/wp21_assemble.py
@@ -91,9 +91,14 @@ def say(what):
-def readable(word, what):
+def receipt_safe(word, what):
     if not word.isprintable() or any(c in word for c in LINE_BREAKERS):
-        raise Void(f"the {what} path carries a character a receipt cannot hold: {word!r}")
+        raise Void(f"the {what} carries a character a receipt cannot hold: {word!r}")
+    return word
+
+
+def readable(word, what):
+    receipt_safe(word, f"{what} path")
     path = Path(word)
@@ -120,7 +125,8 @@ def body_of(text, source):
     at = text.index(BODY_MARKER)
-    body = text[text.index("\n", at) + 1:]
+    end = text.find("\n", at)
+    body = "" if end < 0 else text[end + 1:]
@@ -171,9 +177,9 @@ def corpus_of(index, word):
-    label_go = one_header(text, LABEL_GO_MARKER, source)
+    label_go = receipt_safe(one_header(text, LABEL_GO_MARKER, source), f"{source} label_go")
     digest, body = body_of(text, source)
-    capture = one_header(text, CAPTURE_MARKER, source)
+    capture = receipt_safe(one_header(text, CAPTURE_MARKER, source), f"{source} capture_sha256")
@@ -342,6 +348,17 @@ def main():
+    wanted = len(raw_text.encode("utf-8")) + len(deduped_text.encode("utf-8"))
+    try:
+        vfs = os.statvfs(out_dir)
+    except OSError as why:
+        raise Void(f"cannot preflight the output directory `{out_dir}`: {why}")
+    available = vfs.f_bavail * vfs.f_frsize
+    if available < wanted:
+        raise Void(
+            f"the filesystem holding `{out_dir}` has {available} bytes available and the two "
+            f"manifests want {wanted}"
+        )
     write_pair(raw_path, raw_text, deduped_path, deduped_text)
@@ -356,9 +373,17 @@ def main():
 if __name__ == "__main__":
+    for stream in (sys.stdout, sys.stderr):
+        stream.reconfigure(errors="backslashreplace")
     try:
         sys.exit(main())
     except Void as why:
         print(f"wp21_assemble: RUN VOID: {why}", file=sys.stderr)
         print("wp21_assemble: no manifest was written; this is NOT an answer", file=sys.stderr)
         sys.exit(VOID)
+    except Exception:  # noqa: BLE001 — the backstop that keeps exit 1 meaning REFUSED
+        import traceback
+
+        traceback.print_exc()
+        print("wp21_assemble: RUN VOID: the instrument failed, see the traceback above", file=sys.stderr)
+        sys.exit(VOID)
```

**HEAD at review end**: `037b196ff69f56607b98acb52ea70a3b70e6ebb4` ≠ `9c4366c`; the three
files reviewed are identical at both, and §6 is identical between the draft and the
landed revision 5. The review's worktree `/home/tom/pistol-wt/asm-review2` was removed.
