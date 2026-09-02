# tools/wp21_assemble.py — REVIEW-impl + RED-TEAM against tools/SHELL_CHECKLIST.md

**Revision reviewed**: `5c77a8f` (dev). **HEAD at review end**: `5c77a8f` — matches.
**Files**: `tools/wp21_assemble.py`, `crates/pistol-arena/tests/wp21_assemble_tests.rs`,
their entry in `docs/rule9_justifications.md`. Governing text: `wp21_prereg.md` rev 4
§6/§8, D-560, D-562, `crates/pistol-arena/src/labels_file.rs`.

**VERDICT: FAIL — BLOCKING 0, MAJOR 6, minor 7.** Nothing here produces a wrong
D-562(2) representative; every MAJOR is an exit-0 number the instrument will print
for an input the reader of record would refuse, a registered number no test defends,
or a document that does not say which of two numbers it registered.

**Method.** The shipped script was driven by hand over scratch corpora written by a
generator of my own (`/home/tom/pistol-wt/asm-review/mk.py`, header in the tree's
shape, body digest computed), and over the pilot corpus. Re-derivation (docs/process.md):
the ledger §2c's shakedown figures were re-derived with `awk`, not the script —
`awk -F'\t' 'NR>17{...}' corpus_v1.txt` gives **347 distinct triples, 191 decided
among the deeper-or-first representatives, 0.5504**, and the script over the pilot
twice prints `records 1484 / distinct positions 347 / decided 191 / outcome coverage
0.5504 / key disagreements 0`: agreement. The same awk gives the RAW decided fraction
**414 / 742 = 0.5580**, and **32** same-position pairs in the pilot disagree on
`result` (0 on depth) — which is what makes finding F1 a finding. No cargo was run;
the test suite is reviewed by reading and by the ledger's own 6-of-6 log claim.

---

## SHELL_CHECKLIST items, by name

- **1 (discarded substitution status)** — Python, no `$(...)`. Every read is a
  statement whose failure is either a named `Void` or an uncaught exception; the
  uncaught ones are F3. Not clean.
- **2 (pipeline in a condition)** — n/a, no shell pipelines.
- **3 (`grep` under pipefail)** — n/a. Token-vs-substring: the body marker is matched
  at line start for the COUNT but `text.index(BODY_MARKER)` finds the first occurrence
  anywhere; a header line containing the marker mid-line makes the two disagree and
  the result is a digest VOID, never a wrong body. Survives.
- **4 (locale, and which way a guard moves)** — `readable()` is a DENY-list
  (`ord(c) < 0x20 or == 0x7F`): NEL U+0085 and LINE SEPARATOR U+2028 pass into the
  raw manifest's `corpus_file` column. F8, minor.
- **5 (index vs worktree)** — n/a, the script reads no git state.
- **6 (sweep by prefix)** — no sweep; the only unlink is of a path the script created
  this run under `O_EXCL`. Clean.
- **7 (traps)** — n/a. The equivalent, "what happens after the claim", is F11.
- **8 (one spelling per number, one refusal per reason)** — depth spelling is checked
  as `str(int(x)) == x and >= 0`: `+3`, `03`, ` 3`, `3 `, `-1`, `3.0`, `` all VOID by
  name (reproduced). Clean for depth. Three reasons collapse into one traceback in F3.
- **9 (what reaches a record is caller-controlled)** — `path.name` is interpolated
  into the raw manifest; TAB/LF/CR are refused by name (reproduced: `path carries a
  control character: '.../good.txt\n'`), the two Unicode line breaks are not (F8).
- **10 (THE COVERAGE RULE)** — a CI-run suite drives the shipped script with a control
  run and an independent tally: yes for `records`, `distinct_positions`, the
  representative, `key_disagreements`, VOID and REFUSED. **No assertion anywhere
  touches `decided` or `outcome_coverage`**, a §6-registered MEASURED number. F2.
- **11 (caller path feeding a delete/overwrite)** — enumeration: one `os.unlink`, at
  `claim():136`, target `held_path = raw_path`, origin `out_dir / RAW_NAME` where the
  script itself just created it with `O_CREAT|O_EXCL`. Script-created; a pre-existing
  file or a dangling symlink at either name is REFUSED before creation (reproduced,
  exit 1, `raw_manifest.txt already exists`, the foreign file left untouched). Relative
  `--out-dir` resolves against the caller's cwd (no `cd`). Clean.
- **12 (VOID vs FAIL by name)** — (1) three codes are documented and mostly kept: 0 /
  1 REFUSED / 2 VOID. Broken by F3 (traceback exit 1 for an unreadable input) and F9
  (argparse exit 2 without the VOID line). (2) no preflight of the output filesystem;
  F11. (3) the test's `meaning()` spells all three codes in every failure message —
  yes, item 12(3) is met.

---

## FINDINGS

### F1 — MAJOR — §6 does not say which denominator `outcome_coverage` has, and the instrument chose one the pilot's comparable figure did not use
§6: *"outcome coverage — the fraction of records whose game was decided"*, listed
beside a `records` count that is RAW. D-559/D-562(1)'s 56 % is 414/742, over raw
records. The script divides by the DEDUPED count (`decided / distinct`, :245). Over
the pilot these are **0.5580 vs 0.5504**, and they differ because 32 pilot positions
are reached by one decided and one capped game and tie-to-first picks one. Both are
defensible; the document licenses either, and a closure that quotes 0.5504 beside
D-559's 56 % compares two quantities. Reproducer: the awk lines in Method above.
**FIX**: one clause in §6 — *"the fraction of DEDUPED records"* — filed as the
amendment §8 says reopens review; the script's header already says it (deduped
manifest line 4), so the code needs no change. Do not have the script print both:
that hands the closure a choice.

### F2 — MAJOR — the registered `decided` / `outcome_coverage` numbers have no test (item 10)
No test reads `# derived decided` or `# derived outcome_coverage`; a script printing
`0.0000` always passes all six. Every fixture corpus comes from the honest stub at
`TURN_CAP 8`, so `decided` is whatever the stub games happen to give and nothing pins
it. **FIX**: one test using the existing `rebuild()` helper: set `result` to `p1_win`
/ `end normal` on k records of corpus b and `capped` on the rest of both corpora, then
assert `decided == k` and `outcome_coverage == format!("{:.4}", k / distinct)`, with
the deeper-wins and tie-to-first interaction chosen deliberately (a decided record
that LOSES the tie must not count).

### F3 — MAJOR — an unreadable input or output raises a Python traceback and exits 1, the REFUSED code (item 12(1), item 8)
Three reproducers, all at `/home/tom/pistol-wt/asm-review`:
- non-UTF-8 corpus (`in/latin1.txt`, one byte `0xe9` in the header): exit **1**,
  `UnicodeDecodeError: 'utf-8' codec can't decode byte 0xe9 in position 3`.
- mode-000 corpus (`in/noperm.txt`): exit **1**, `PermissionError: [Errno 13]`.
- mode-500 `--out-dir` (`out/ro`): exit **1**, `PermissionError` from `claim():131`
  — after every input was read.
Exit 1 is documented as *"a named refusal — an output that already exists"*; a reader
of a tranche log sees a refusal that names no output. **FIX**: wrap `read_text` and
the two `os.open`s: `OSError`/`UnicodeDecodeError` → `Void` naming the path and the
errno text. Optionally a top-level `except Exception` → VOID with the traceback on
stderr, so no code path can reach exit 1 without the word REFUSED.

### F4 — MAJOR — the instrument accepts what `labels_file::read` refuses, and the number it then prints is wrong at exit 0
`labels_file.rs:175-266` refuses a schema other than 1, an empty field, and a
`result`/`end` outside `{p1_win,p2_win,capped}`/`{normal,forfeit}`. The script checks
none. Reproducers:
- `in/v2.txt` (`corpus_schema_version 2`, otherwise valid): exit **0**, counted.
- `in/res.txt` (`result P1_WIN` on record 1, `end Normal` on record 2, both games
  wins): exit **0**, `decided 0`, `outcome coverage 0.0000`.
- `in/emptyfield.txt` (`key_seq` empty): exit **0**, `distinct positions 2`.
The body digest does not protect against these: a differently versioned writer signs
its own body. **FIX**: refuse, by name, `corpus_schema_version != 1`, any empty field,
and a `result`/`end` outside the closed sets — the same three checks the reader of
record makes, or (cleaner) have the script call `target/release/corpus-check` on each
input and VOID on its non-zero, so there is one grammar.

### F5 — MAJOR — the same corpus given twice is assembled twice, and `records` doubles at exit 0
`--corpus in/good.txt --corpus in/good.txt`: exit **0**, `records 4`, raw manifest
two rows with one digest. Same bytes under two names (`in/good_copy.txt`): identical.
§6's RAW manifest is *"every passing tranche's corpus"*; two rows with one
`corpus_body_sha256` is one tranche listed twice and the registered `records` is
wrong by that corpus. The pilot shakedown in ledger §2c (`corpus_v1.txt` twice) is
exactly this input, run on purpose. **FIX**: VOID on a repeated body digest (and on a
repeated `capture_sha256`), naming both indices. The shakedown then needs a second
distinct corpus, e.g. `corpus_v1.txt` and a `--skip/--take` slice re-labelled.

### F6 — MAJOR — `key_disagreements` depends on the order the corpora are given
Records A=(s1,p1,f1), B=(s2,p2,f2), C=(s1,p2,f3). Order A,B,C (`in/o1.txt`):
`key disagreements 1`. Order C,A,B (`in/o2.txt`): `key disagreements 2`. Same
multiset, same `distinct positions 3`. The count is "triples that share a key with an
EARLIER triple", which is not a property of the corpus. The ledger already quotes
this number (`disagreements 0`) and D-562(2) names it as the sweep's first reading of
the open transposition question, so it will be read. **FIX**: count something
order-free — the number of distinct triples that share at least one key value with
any OTHER distinct triple (two passes: collect per-key multiplicities over distinct
triples, then count triples with any key of multiplicity > 1) — and say so in the
header. The registered numbers (`records`, `distinct_positions`, coverage) are
order-free already; only the representative depends on order, which is D-562's rule.

### F7 — minor — `label_go` is not required to agree across corpora
`in/good.txt` (`go nodes 400000`) with `in/go.txt` (`go nodes 5000`): exit **0**. The
deeper-wins comparison of `depth_turns` across two budgets compares nothing. §1 fixes
one seat for the whole sweep, so a disagreeing corpus is not a passing tranche.
**FIX**: read `# param label_go` and VOID on a second value, naming both corpora.

### F8 — minor — NEL and LINE SEPARATOR in a basename pass the control-character guard (item 4)
`in/x<U+0085>y-nel.txt`: exit 0; the raw manifest read back with Python's
`str.splitlines()` has 7 lines where the file has 6 and the row reads `1\tx`. The
script's own readers use `split("\n")` and the Rust `lines()` in the tests are safe;
a future Python reader is not. **FIX**: allow-list — refuse any basename character
that is not `str.isprintable()`, and refuse U+0085, U+2028 and U+2029 explicitly.

### F9 — minor — argparse errors exit 2 without the `RUN VOID` line (item 12(1))
`--out-dir out` alone: exit 2, `error: the following arguments are required:
--corpus`. Semantically a void; the vocabulary is argparse's. **FIX**: a parser
subclass whose `error()` raises `Void`.

### F10 — minor — `record_line` counts blank body lines; the reader of record numbers non-empty records
`in/blank.txt` (A, empty line, B): exit 0, B's row carries `record_line 3`;
`labels_file::read` would call B "record 2" in any refusal. The writer never emits a
blank line, but the index the DEDUPED manifest is — *"an INDEX into the raw
corpora"* — should say which it means. **FIX**: VOID on an empty line inside the body
(the writer never produces one), which makes the two numberings coincide.

### F11 — minor — a write failure after both claims leaves two files behind (item 12(2))
`claim()` creates both files, then `write()` runs (:250-253) with no handler: an
`ENOSPC` here tracebacks (exit 1, F3) with an empty or partial pair on disk, which
the next run then REFUSES as somebody else's. Reasoned, not reproduced (filling a
filesystem is not a scratch operation). **FIX**: render both texts before either
claim (they already are computed first — move the two `claim()` calls below the
`manifest()` calls), write inside one `try`, and unlink both on any exception.

### F12 — minor — the deeper-wins test alone does not exclude "last record wins"
`the_deeper_label_wins_a_position_two_records_share` puts the deeper record in
corpus 2, record 1 — the LAST record for that position. A script that ignored depth
and kept the last record passes it. The suite as a whole does exclude that script,
via the all-rows-are-corpus-1 loop in `the_deduped_manifest_holds_one_row_...`, so
this is a coupling, not a hole. **FIX**: in the same test also raise corpus a's
record 1 to `9` on a second position and assert corpus 1 wins there.

### F13 — minor — the rule-9 entry's "splitting would copy the three-stage fixture" is not true of the tree
`corpus()` is a function; `tests/common/mod.rs` exists to hold exactly such helpers
and already holds `run`, `Scratch`, `ConfigSpec`. The entry's other claims — every
case builds the same fixture, the dedup cases sit beside the independent tally, the
refusal cases beside a run that writes — are true of the file. **FIX**: either move
`corpus()`/`assemble()`/`rebuild()` into `common` and split, or reword the entry to
the argument that is true (the void/refusal cases are worthless without the writing
cases in the same suite).

---

## WHAT SURVIVED ATTACK

- **D-562(2)'s rule, exactly.** Same file, later record deeper: deeper wins
  (`in/c1.txt`, line 3 depth 5 over line 1 depth 3). Across files, equal depth: the
  first corpus wins (`c1` line 3 over `c2` line 1, both depth 5). Shallower later
  record loses (`c2` line 2 depth 2 vs `c1` line 2 depth 4). Ties go to FILE order,
  not game index (`in/c4.txt`: game 5 at line 1 beats game 0 at line 2). Transposition
  (`key_pos` equal, `key_seq` differs) and symmetry image (`key_full` equal only) are
  kept distinct and counted. The `-` spelling at k=0 dedups two games' roots to one
  position. The chosen row names the right corpus index and record line in every case.
- **The pilot reproduces the ledger** and agrees with an independent awk tally.
- **Malformed corpora VOID by name, exit 2, nothing written**: digest mismatch,
  comment in body, 17 fields, every bad depth spelling, zero records, CRLF (caught by
  the digest), a directory, a missing file, a path with LF, out-dir missing or a file.
  Every refusal names its input.
- **The exclusive claim**: raw exists → REFUSED; dangling symlink at raw → REFUSED
  (`O_EXCL` follows nothing); deduped exists → REFUSED and the freshly claimed raw is
  removed (the test pins this). No input is read after an output is created; no
  output is created before every input is read.
- **Determinism**: two runs over the same two corpora produce byte-identical
  manifests; `dict`/`list` insertion order, `.4f` formatting, basenames not paths.
- **The tests drive the shipped script** (`repo().join("tools/wp21_assemble.py")`),
  have four exit-0 runs as controls, tally distinct triples independently with a
  `BTreeSet`, and spell all three exit codes in every failure. `python3` missing
  panics loudly rather than skipping.

## ATTACKS I ATTEMPTED AND REJECTED

- **Division by zero in coverage**: unreachable — every corpus must hold ≥ 1 record
  (`records_of` VOIDs on none) so `distinct ≥ 1`. Rejected.
- **Body marker inside a body record**: a record line cannot start with `#` and the
  marker is matched at line start for the count; a mid-line occurrence in a header
  line gives a digest VOID. Rejected.
- **Header `derived records N` disagreeing with the body**: not read by the script,
  not read by `labels_file::read` either; the digest binds the body. Not a finding.
- **`the_deduped_manifest…` passing on a one-key script**: a and b are identical so
  any key gives the same count there — but the disagreement test (`key_seq` edited,
  `distinct + 1`) fails a key_pos-only script. Rejected.
- **Stub depth making the deeper test vacuous**: the honest stub reports
  `depth_turns: 1` (`stub_engine.rs:261`), so `9` exceeds it and a tie is impossible.
  Rejected.
- **Symlinked corpus escaping `is_file`**: a symlink to a regular file is read; a FIFO
  is refused as not a regular file (item 8's three-reasons case). Rejected.
- **Scratch naming** (item 6): `Scratch` uses `pistol-testscratch-arena-`. Rejected.
