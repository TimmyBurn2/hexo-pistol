# tools/ REVIEW-impl + RED-TEAM — `tools/revision_citation_check.py` and gate 20's wiring, at `ab9d2a9`.

REVISION REVIEWED: `ab9d2a9` on `dev` (worktree `/home/tom/pistol-wt/revcheck`, detached).
`git rev-parse HEAD` at the end of this review: `ab9d2a99563480373c5db1e1c1199aa2ee84e4d9` — unchanged,
still `ab9d2a9`.
REVIEWED ON: Sonnet, per D-597.

**VERDICT: FAIL — 2 BLOCKING, 1 MAJOR, 4 minor.**

Both BLOCKING findings are exit-0-wrong-answers in `tools/revision_citation_check.py`'s
matcher, reproduced below with minimal fixtures. All 7 tests in
`crates/pistol-arena/tests/governing_citation_check_tests.rs` pass (`cargo test -p
pistol-arena --test governing_citation_check_tests`, 7 passed; 0 failed), the gate
wiring correctly threads PASS/FAIL/VOID (0/1/2) from the new Python script through
`tools/governing_citation_check.sh` and would thread through `tools/ci.sh`'s existing
`gate()` wrapper unchanged, and the D-599/D-600 spec is followed on every point the
review was asked to check except where the two BLOCKING findings undercut it. Neither
finding is exercised by the current nine-document GOVERNING list (both require a
document shape the tree does not currently contain), but both are reproducible today
against the shipped script with no changes to the tree's real documents, and both are
exactly the shape of attack this checklist review was asked to try.

---

## 1. SHELL_CHECKLIST items 1-12, by name

The new code is `tools/revision_citation_check.py` (Python) and 15 added lines in
`tools/governing_citation_check.sh` (bash). Most of the bash-specific items (1, 2, 4,
5, 6, 7, 9, 11) have no new surface to fail on: the diff adds no command substitution
in a discarded position, no pipeline, no character class, no git-vs-worktree read, no
delete/sweep, no trap, and no value that reaches a written record. Going through all
twelve by name:

1. **Command substitution status discarded.** N/A — no new command substitution.
   The added `printf 'governing_citation_check: %d revision exemption(s)\n'
   "${#REVISION_EXEMPT[@]}"` and the final `python3 tools/revision_citation_check.py
   "${EXEMPT_ARGS[@]}" "${GOVERNING[@]}"` are plain statements; the latter is the
   script's last command, so under `set -e` its exit code IS the script's exit code
   (proven in §4).
2. **Pipeline in a `then` body vs. a statement.** N/A — no pipelines added.
3. **`grep` under `pipefail` / substring-is-not-a-token.** No `grep` in the bash diff.
   Applied to the Python regexes instead (the closest analogue): the `CITATION` regex
   requires `` ` `` + name + `` ` `` + `\s+` + literal `revision` + `\s+` + `\d+`,
   which defends against the plural/prefix substring case (`revisions 3` fails to
   match because the character after `revision` must be whitespace, not `s`) — traced
   and confirmed by hand, item PASSES for `CITATION`. `TITLE_REVISION` is a bare
   `revision\s+(?P<revision>\d+)` with no leading `\b` or backtick anchor, so it is
   vulnerable to a first-line compound word like `prerevision 3` in principle (F6,
   minor) — no title in the tree's nine GOVERNING documents contains such a word.
4. **`LC_ALL` / character-class direction.** N/A — the Python name pattern
   (`[A-Za-z0-9_./-]+`) is an ALLOW-list already, not a negated class, and has no
   locale dependency.
5. **Index vs. working tree.** N/A — no git operations; the checker reads working-tree
   files via `Path.read_text`, same as the pre-existing `design_citation_check.py`
   sibling it is wired beside. This is a documentation-consistency check, not a
   tamper check on staged content, so reading the worktree is the correct target, not
   a violation of this item.
6. **Sweep-by-prefix ownership.** N/A — no deletion anywhere in the diff.
7. **Traps.** N/A — `tools/governing_citation_check.sh` has no `trap` at all, and this
   diff doesn't add one.
8. **One spelling per number.** Python's `int()` does not read a leading-zero string
   as octal (unlike bash's `[ 010 -ge 1 ]`), confirmed by reproducer: `` `cited.md`
   revision 007 `` against a document at revision 7 matches cleanly (exit 0, 0 stale).
   No octal-style defect. PASSES.
9. **Caller-controlled record injection.** N/A — nothing in this diff interpolates a
   caller-influenced string into a document another tool parses; output is stderr/
   stdout prose only.
10. **THE COVERAGE RULE.** SATISFIED, with one narrow gap (F5, minor) — detailed in
    §5 below.
11. **Containment-guarded deletes/overwrites.** N/A — no `rm`, `mv`, or write-to-path
    anywhere in the diff.
12. **VOID vs FAIL, by name, all three obligations.** SATISFIED at both the Python
    script and the bash wrapper, proven in §4 by execution, not by reading:
    1. **A code per kind** — the module docstring states 0/1/2 exactly
       (`revision_citation_check.py:29-33`), matching the wrapper's own existing
       0/1/2 usage block.
    2. **Preflight and void early** — the wrapper's existing preflight block gained
       `[ -f tools/revision_citation_check.py ] || void "tools/revision_citation_check.py
       is missing"`, run before any document is touched, same pattern as the sibling
       check.
    3. **The distinction survives the seam** — every new Rust test uses a `meaning()`
       helper that names all three codes in the failure message
       (`governing_citation_check_tests.rs:94-101`), so a test failure never misreads
       a VOID as a FAIL or the reverse; and `a_cited_document_that_states_no_revision_
       is_a_void_and_not_a_refusal` asserts `Some(2)` by name, exactly the shape item
       12 demands. Verified live in §4 that the wrapper script itself (not just the
       Python script standalone) exits 2, not 1, when the inner check voids, and that
       `tools/ci.sh`'s existing `gate()` function (unmodified by this diff, already
       generic over any gate's 0/2/other) would carry that further unchanged.

## 2. Does it do what D-599/D-600/D-602 specify?

**Only citations BETWEEN documents on the governing list are checked** — YES.
`main()` builds `governing` from exactly the positional `documents` argv (which the
wrapper populates from `GOVERNING[@]`), and `check()` skips any citation whose name is
`not in governing`. Confirmed live: with `citer.md` citing both a governing `gov1.md`
revision correctly and a non-governing `not_governed.md` revision 99 (wrong for
nothing, since it isn't checked), the run reports 1 citation checked, 0 stale, exit 0
— the second citation isn't even counted. Confirmed again against the real tree: the
regex's 8 raw matches across the nine GOVERNING documents all happen to name other
GOVERNING documents, so this path isn't exercised by production data, but the scratch
reproducer above exercises it directly.

**The exemption is named per document with its reason** — YES, and it matches
D-600's RE-GROUND rather than D-599's superseded one. The wrapper's comment reads "The
exemption is by that reason and never by a pin disclaimer, which covers `file:line`
citations alone (D-600)" — citing D-589 (gate closed at revision 6) rather than the
`adb2012` pin D-599 originally (wrongly) cited. Confirmed the exemption is load-bearing,
not vacuous: running `revision_citation_check.py` over `wp21_label_cache_design.md`
plus its citation targets WITHOUT `--exempt` gives 2 stale (it cites
`wp21_throughput_prereg.md` revision 3 and `wp21_prereg.md` revision 4, both now at
revision 10) — exactly the frozen-at-gate-close state D-600 describes. With the shipped
`--exempt`, the real gate run reports `docs/experiments/wp21_label_cache_design.md:
EXEMPT, its citations are not checked` and exits 0.

**The title revision is read from the first line only** — YES for the ordinary case:
a stray `revision 99` appearing later in a document's body is confirmed NOT read as
its title revision (reproducer: a fixture with `revision 2` on line 1 and `revision 99`
in the body correctly resolves to 2, not 99). The narrower defect is not about reading
past line 1 — it's about what happens when line 1 itself contains more than one
`revision N` occurrence, which is exactly the "title line whose revision is not the
document's own" attack the review brief named, and it is a real defect (F1, BLOCKING,
§3).

## 3. RED-TEAM the matcher

| Attack | What the script does | Safe direction? |
|---|---|---|
| Citation inside a fenced code block / blockquote, reproducing a historical quotation verbatim | NOT skipped — checked identically to prose. A verbatim quote of a stale historical citation is flagged stale (reproduced: a quotation inside `` ``` `` and inside `> ` both trigger `1 stale` against the true current revision). | **Safe direction.** Over-refusal (false FAIL, a human reads it and can exempt or rephrase) rather than under-refusal (silent false PASS). Consistent with the sibling `design_citation_check.py`'s own stated stance that it does not distinguish a quotation from a live claim. (F4, minor.) |
| `revision` in a different case, or extra/odd whitespace | Both `CITATION` and `TITLE_REVISION` use `re.IGNORECASE`; `\s+` matches multiple spaces/tabs. Reproduced: `` `cited.md`   REVISION    7 `` matches correctly. | Correct, no finding. |
| A citation split across a line break | `\s+` in Python matches `\n` regardless of `re.DOTALL` (which only affects `.`). Reproduced: `` `cited.md`\nrevision 7 `` still matches. | Correct, no finding. |
| A document citing itself | `check()` explicitly skips `name == Path(document).name`. Reproduced: a document whose body says "earlier this was `selfcite.md` revision 2, now bumped" (its own name, an old revision) produces 0 checked, exit 0 — not flagged as citing itself falsely. | Deliberate, reasonable scoping (a document's own revision-history narrative about itself is not the drift class D-599 names — that class is ONE document making a claim about ANOTHER's title). Not a defect. |
| `revision 007` (leading zero) | `int("007")` == 7 in Python; no bash-style octal reading. Reproduced: matches a document at revision 7 cleanly. | Correct, no finding (checklist item 8's octal trap is bash-specific and doesn't reach Python's `int()`). |
| A citation naming a path form where the GOVERNING list holds a bare name, and the reverse | Both `CITATION`'s matched name and the `governing` dict's keys are normalized to `Path(...).name` before comparison, so either form resolves to the same basename. Reproduced both directions: governing list has the bare name / citation uses the full path, and vice versa — both resolve correctly (exit 0, 1 checked, 0 stale). | Correct, no finding. |
| Two GOVERNING documents with the same basename in different directories | **BLOCKING.** `governing = {Path(d).name: title_revision(...) for d in documents}` is a dict keyed by basename — the second entry silently overwrites the first. Reproduced: `dirA/foo.md` (revision 5) and `dirB/foo.md` (revision 1) both on the governing list; a citation of `` `dirA/foo.md` revision 1 `` — actually WRONG, dirA is really 5 — is reported clean, exit 0. The reverse (citing dirA's TRUE revision 5) is wrongly refused as stale ("foo.md is at revision 1"). | **Wrong direction — a silent false PASS.** Not exercised by the current 9-document GOVERNING list (all basenames distinct today), so latent, but nothing detects or guards a future collision. See F2. |
| A title line whose revision is not the document's own (quotes another document's revision in its title) | **BLOCKING.** `TITLE_REVISION.search(first_line)` returns the FIRST `revision \d+` match anywhere on line 1, not the one attached to the document's own self-declaration. Reproduced: `self.md`'s title is `` # a title citing `other.md` revision 9, and stating this document itself is revision 2. `` — `title_revision(self.md)` returns **9**, not 2. Consequence A: a correct citation of `` `self.md` revision 2 `` elsewhere is wrongly refused as stale (`self.md is at revision 9`). Consequence B, worse: a WRONG citation of `` `self.md` revision 9 `` (matching the miscomputed number, not self.md's true revision 2) is reported clean, **exit 0**. | **Wrong direction — a silent false PASS**, the exact class item 12 opens with ("a gate that prints a verdict, exits 0, and is wrong"). See F1. |
| `revision N of `X.md`` (reverse word order) | **MAJOR.** `CITATION` only matches `` `name` `` immediately followed by `revision N`; the reverse phrasing isn't matched at all — the citation isn't even counted. Reproduced: `` GOVERNING: revision 3 of `cited.md` `` against a document truly at revision 10 gives **0 citations checked, exit 0** — invisible, not merely unflagged. This exact reversed phrasing already occurs in the tree today (`matrix_label_cache_key.md:33-34`, `:41`), though there it narrates history rather than making a live claim, so it happens not to be live-wrong at `ab9d2a9`. | **Wrong direction for this phrasing** — a real stale claim in this word order gets no signal at all. D-599's own spec text quotes the citing form name-first (`` `X.md` revision N ``), so this is arguably within the letter of what was asked for, but not the spirit ("every ... citation"). See F3. |

## 4. The gate wiring, proven by execution

Built a throwaway copy `tools/governing_citation_check_TEST.sh` (never the tracked
file) with its `GOVERNING`/`REVISION_EXEMPT` arrays repointed at scratch fixtures,
deleted after use, worktree left clean (`git status --short` empty, confirmed).

- **PASS case** (clean citation): wrapper exit **0**. `governing_citation_check:
  0 revision exemption(s)` / `... 1 revision citation(s) checked, 0 stale` /
  `REVISION_CITATION_CHECK_DONE` all printed.
- **FAIL case** (stale citation, `docA.md` revision 4 cited where the tree holds 5):
  wrapper exit **1**. `` `docA.md` revision 4 — docA.md is at revision 5 `` printed.
- **VOID case** (cited document states no revision on its title line): wrapper exit
  **2**. `revision_citation_check: RUN VOID: scratch/wire/docE_norev.md states no
  revision on its first line` / `... this is NOT a failure` printed, and — critically
  — the FIRST half (`design_citation_check.py`) had already exited 0 cleanly on the
  same two fixtures, so the VOID is unambiguously the new half's own, not swallowed
  or relabeled by the old one.

All three propagate as the exact code the inner Python script itself returned — the
wrapper does nothing after the final `python3 tools/revision_citation_check.py ...`
call, so under `set -euo pipefail` its own exit status is the last command's,
unmodified. `tools/ci.sh`'s `gate()` function (unmodified by this diff) already
switches on `0`/`2`/other generically for every gate including gate 20
(`gate "governing citations" tools/governing_citation_check.sh`, `tools/ci.sh:203`),
so the VOID/FAIL distinction demonstrated above would reach `ci.sh`'s own exit code
unchanged. (A full `tools/ci.sh` run was not executed here — one is already running
in a separate, untouched worktree per the assignment's own note — but `gate()`'s logic
is read and is generic, not gate-20-specific, so this inference doesn't need that run.)

One incidental, non-blocking observation from building the wiring proof: if the
FIRST half (`design_citation_check.py`) itself fails or voids, the script exits
immediately under `set -e` and the revision check never runs at all that invocation —
expected, sequential-gate behavior, not a defect.

## 5. The tests

`cargo test -p pistol-arena --test governing_citation_check_tests` (7 passed, 0
failed) at `ab9d2a9`, `CARGO_TARGET_DIR=/home/tom/pistol-wt/revcheck/target`:

- All 4 new tests **drive the shipped script**: 3 call `tools/revision_citation_check.py`
  directly as a subprocess with fixtures of the test's own making (never re-implement
  its logic), 1 (`the_gate_runs_the_revision_check_over_the_documents_it_governs`)
  drives `tools/governing_citation_check.sh` itself.
- **Control run present**: `a_citation_of_a_revision_the_cited_document_does_not_hold_
  is_refused_by_name` runs the SAME pair of documents once clean (asserts exit 0) and
  once one-revision-stale (asserts exit 1), in one test — a checker that refused
  everything would fail the clean half; one that refused nothing would fail the stale
  half. This is exactly item 10's "control run so a pass cannot come from a gate that
  refuses everything."
- **Exit codes spelled out**: every assertion uses `output.status.code()` compared by
  name (`Some(0)`, `Some(1)`, `Some(2)`) with a `meaning()` helper naming all three in
  the failure text — satisfying item 12 obligation 3 directly in the test code, not
  just in prose.
- Gap: the wrapper's own new recorded number — `printf 'governing_citation_check: %d
  revision exemption(s)\n'` — is not independently asserted by any test (tests check
  for `"revision citation(s) checked"` and `"REVISION_CITATION_CHECK_DONE"`, not
  `"revision exemption(s)"`). The exemption's functional behavior IS well covered
  (`a_document_whose_citations_are_exempt_is_not_checked_and_says_so` drives the
  Python script directly with `--exempt`), so this gap is narrow — the wrapper's own
  printed count, not the mechanism — but it is a "recorded number" per item 10's own
  wording with no test naming it. (F5, minor.)

## Findings

**F1 — BLOCKING.** `TITLE_REVISION.search(first_line)` returns the first `revision
\d+` match anywhere on line 1, not the document's own self-declared one. A title that
quotes another document's revision before stating its own causes (a) a correct
citation elsewhere to be wrongly refused, and (b) a wrong citation matching the
miscomputed number to pass silently at exit 0.
Reproducer: `self.md` first line `` # a title citing `other.md` revision 9, and
stating this document itself is revision 2. ``; `` `self.md` revision 9 `` cited
elsewhere → exit 0, 0 stale (should be flagged: self.md's real revision is 2). Not
present in the tree's nine current title lines — none of them quote another document's
revision in their own title — but nothing in the code or a test prevents it, and the
review brief named this exact attack.
Fix direction: anchor `TITLE_REVISION` to the LAST `revision N` on the line (titles in
this tree consistently end `..., revision N.`), or require it follow the document's
own name/kind rather than taking the first match; either needs a test with a
self-quoting title before it can be trusted.

**F2 — BLOCKING (latent).** `governing` is a dict keyed by `Path(d).name` (bare
basename); two GOVERNING-list documents sharing a basename in different directories
collide, the later one silently winning. Reproducer: `dirA/foo.md` (revision 5),
`dirB/foo.md` (revision 1); a citation of `` `dirA/foo.md` revision 1 `` (wrong — dirA
is really 5) passes at exit 0. Dormant today — all nine real GOVERNING basenames are
distinct — but unguarded: nothing refuses at startup if a future GOVERNING addition
collides.
Fix direction: key `governing` by the full document path (or refuse at startup if two
GOVERNING entries share a basename), matching how `check()` normalizes CITATIONS to
basename but need not force the GOVERNING index itself to lose the directory.

**F3 — MAJOR.** `CITATION` only matches `` `name.md` `` immediately followed by
`revision N`; the reverse phrasing `revision N of `name.md`` is not matched at all —
invisible, not merely unflagged. This exact reversed phrasing already exists in the
tree (`matrix_label_cache_key.md:33-34,41`), currently narrating history rather than
making a live claim, so it is not live-wrong at `ab9d2a9`, but the class this gate
exists to catch could recur in this word order with zero signal.
Fix direction: widen `CITATION` to match both orders, or state the narrower scope
explicitly in the module's own docstring (it currently claims to check "every
`<doc>.md` revision N a GOVERNING document states," which oversells what it does).

**F4 — minor, by design / safe direction.** Citations inside fenced code blocks or
blockquotes are checked identically to prose, so a verbatim historical quotation of a
since-superseded revision number is flagged stale. This is the safe direction
(over-refusal, human-visible) rather than the dangerous one, and matches
`design_citation_check.py`'s own stated stance. No fix required; noted so a future
reviewer doesn't rediscover it as a surprise.

**F5 — minor.** The wrapper's new `"N revision exemption(s)"` printed count has no
test naming it, though the mechanism it reports on is otherwise well tested.

**F6 — minor, theoretical.** `TITLE_REVISION` (unlike `CITATION`) has no word-boundary
anchor before `revision`, so a contrived compound word (`prerevision 3`) on a title
line would false-match. No title line in the tree's nine GOVERNING documents is
affected today.

**F7 — minor.** `--exempt` matching in `main()` (`document in exempt`) is exact-string
against the argv form, while citation-name matching is basename-normalized. The
wrapper always spells both arrays identically today, so this doesn't misfire, but a
future edit that spells the same document two different ways between `GOVERNING` and
`REVISION_EXEMPT` would silently fail to exempt (the check would just run — the safer
of the two possible failure directions, but undocumented).

## Attacks tried and rejected (no defect found)

- Case variation and whitespace variation in "revision" (`REVISION`, extra spaces) —
  correctly normalized via `re.IGNORECASE` and `\s+`.
- Citation split across a line break — correctly matched, `\s+` spans `\n`.
- Document citing itself — deliberately and correctly excluded from its own count.
- `revision 007` (leading zero) — Python's `int()` parses it as 7, no bash-style
  octal trap.
- Path form vs. bare basename mismatch between a GOVERNING entry and a citation, both
  directions — correctly normalized to basename on both sides.
- Citation to a document not on the GOVERNING list — correctly skipped and not
  counted, confirmed against both a scratch fixture and the real tree's 8 raw
  citation matches (all 8 happen to name GOVERNING documents at `ab9d2a9`).
- Exemption vacuity — checked that `wp21_label_cache_design.md`'s exemption is load-
  bearing, not decorative: run without `--exempt`, its own two citations (of
  `wp21_throughput_prereg.md` revision 3 and `wp21_prereg.md` revision 4, both now
  really at revision 10) are flagged stale, confirming D-600's frozen-at-gate-close
  rationale is real and not vacuous.
- Wrapper exit-code seam (design_citation_check.py failing/voiding before the
  revision check runs) — confirmed sequential, expected `set -e` behavior, not a
  defect.
