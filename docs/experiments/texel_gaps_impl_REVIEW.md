# tools/texel instrument gaps — REVIEW-impl

**Revision under review**: `8e7fdf08c8ac9cc77894520a297b1f2bcc1531bc` (branch `dev`).
**Does it match HEAD?** YES — `git rev-parse HEAD` → `8e7fdf08c8ac9cc77894520a297b1f2bcc1531bc`.

**BUT THE WORKING TREE IS NOT THE PINNED REVISION AND MOVED TWICE WHILE THIS REVIEW RAN.**
At the start: `M docs/experiments/texel_gaps_CLOSURE.md` (286 lines at `8e7fdf0`, 574 in
the tree). At the end: `M docs/decisions.md`, `M docs/experiments/texel_gaps_CLOSURE.md`
(659 lines), `M tools/texel/test_texel.py`. **Everything below adjudicates `8e7fdf0`.**
The uncommitted continuation is not under review and is not evidence for it; where a
finding below says "recorded nowhere", it means nowhere *at the pinned revision*, and
several of them appear to be what that unlanded text addresses. A re-review at whatever
revision carries it is owed regardless — an amendment reopens the review however small
the diff.

Reviewer did not implement this work and did not fix anything. No file in the repository
was edited except this report. Mutation and gate work was done in reviewer-owned
worktrees on `/home` (`/home/tom/review-wt/texel_8e7fdf0` at `8e7fdf0`,
`/home/tom/review-wt/at_6749754` at `6749754`), each with its own
`CARGO_TARGET_DIR=/home/tom/review-wt/target-texel`, never the live tree. Both worktrees
have been removed; neither held a gitignored `artifacts/` or `sessions/` — the only files
in them that were not tracked were two scratch mutation harnesses whose contents are
quoted or described in full below, and the run logs, which live in this session's
scratchpad and are transcribed here rather than referenced.

---

## VERDICT

**FAIL** — on items **B** and **D**, and on the package's record (see BLOCKING-1).

| item | ruling | verdict | one line |
|---|---|---|---|
| **A** | D-658 | **PASS** | The third relation is derived correctly, binds exactly where the test says, and survives two independent mutants (theirs M1, mine H6). |
| **B** | D-657 | **FAIL** | The receipt is pinned over a **19-column** fixture while this same package made `extract.py` write **20** — the receipt's input is no longer of the kind the pipeline produces; and the retirement guard is escapable by basename (H7 ALIVE). |
| **C** | D-658 | **PASS**, with MAJOR-3 | All six rows are reached and a subject-side rescale really does die (my H1). But the 8e7fdf0 fix moved detection out of the negative control into the positive check, so the control is vacuous under exactly the mutant it was registered against; the harness's own exit code for the package's evidence run is **1**. |
| **D** | D-659 | **FAIL** | The per-row verification is sound (H2, H5 die), but the number the item exists to produce — `from 3487 distinct game(s)` — is defended by nothing: `games.add((index, game))` → `games.add(game)` survives the whole suite (H8 ALIVE). |
| **E** | D-660 / D-663 | **PASS** | The condition is measured right (bytes), no golden pins the digest (three sites recompute, not the two named), and the pre-registered DEFER branch is honoured rather than re-argued. |
| **F** | D-661 | **PASS**, with MINORs | The widening from two sites to eight is right — D-661's ground is a property of every site — and **no BUILD/PROTOTYPE/SKIP verdict moved**. The CLOSURE's own count of the sites is wrong, and the false premise it discovered is recorded nowhere at this revision. |
| **G** | (no ADR) | **PASS** | Verified against my own enumeration of all four committed openings fixtures; the claim is exactly right. No ADR line, and the owed-item ledger still says it is owed. |

---

## Commands I ran (scope stated; none of them the document's own)

Independence of scope is the point (`docs/process.md`, "Re-derivation"), so where the
package used Python I used `awk`/`sha256sum`, where it used `git grep` on a path I used
`git grep` on the basename and on a *different revision*, and where it asserted a gate
would refuse I made the gate refuse.

```
# the join counts — awk, tranche by tranche, never extract.py
for i in $(seq 1 16); do
  awk -v T=$i 'FNR==NR { if (substr($0,1,1)=="#") next; if ($1+0==T) want[$2]=1; next }
               substr($0,1,1)=="#" {next}
               { n++; if (n in want) printf "%s\t%s\n", T, $1 }' \
      artifacts/arc3r_sweep_deduped_manifest.txt \
      /home/tom/Projects/pistol-corpus/arc3r-sweep/tranche-$i/corpus.txt
done > game_pairs.txt
wc -l game_pairs.txt ; LC_ALL=C sort -u game_pairs.txt | wc -l
cut -f2 game_pairs.txt | LC_ALL=C sort -u | wc -l

# the triple enumeration — awk triple loop, not the test's list comprehension
awk 'BEGIN{for(a=1;a<=20;a++)for(b=a+1;b<=20;b++)for(c=b+1;c<=20;c++){n++;if(a+b>c)k++}
     printf "triples=%d can_bind=%d\n",n,k}'

# the receipt digest — sha256sum over the lifted body, not FIT.receipt_digest
python3 tools/texel/fit.py tools/texel/fixtures/fit_rows_v1.txt > fitout.txt
awk '/^fit-receipt BEGIN$/{f=1;next} /^fit-receipt END$/{f=0}
     f && /^fit-receipt /{l=substr($0,13); if (l !~ /^sha256 /) print l}' fitout.txt \
  | sha256sum

# the reference count — at THREE revisions, not the one the document names
git grep -n -F 'eval_v0_quiet_fit_weights' beb91b1 -- | wc -l   # and 6749754, 8e7fdf0

# the CTSS count — the document's own literal command, in a worktree at 6749754
cd /home/tom/review-wt/at_6749754 && /usr/bin/grep -rn 'CTSS' docs/ | wc -l

# the arena seat enumeration — every arena config, not the three the ADR names
for f in $(git ls-tree -r --name-only 8e7fdf0 -- configs/ | /usr/bin/grep '^configs/arena'); do
  git show 8e7fdf0:$f | /usr/bin/grep '^config = '; done
for f in $(git ls-tree -r --name-only 8e7fdf0 -- configs/); do
  git show 8e7fdf0:$f | /usr/bin/grep -E '^[[:space:]]*weights_file'; done

# "was the deletion forced?" — make the gate answer, twice
git show 6749754:configs/instrument_quiet_fit_v0.toml > configs/instrument_quiet_fit_v0.toml
bash tools/config_check.sh ; echo $?          # -> 1
git show 6749754:configs/arena_wp22_phase1_quiet_dryrun.toml > configs/...
bash tools/config_check.sh ; echo $?          # -> 0
git show 6749754:docs/experiments/wp22_phase1_design.md > docs/experiments/wp22_phase1_design.md
bash tools/governing_citation_check.sh ; echo $?   # -> 1, three named refusals

# mutation — their harness re-run, then seven of my own, then three more
python3 ./mutants_texel.py                    # in my worktree at 8e7fdf0
python3 ./reviewer_mutants.py                 # H1..H7
python3 h8.py ; python3 h11.py                # H8..H12
```

---

## BLOCKING

### BLOCKING-1 — the package's own closure record stops in the middle, and no gate log is cited anywhere in it

**Claim.** At `8e7fdf0`, `docs/experiments/texel_gaps_CLOSURE.md` is 286 lines and ends at
`### P8`. It forward-references sections that do not exist, four times:

```
$ git show 8e7fdf0:docs/experiments/texel_gaps_CLOSURE.md | /usr/bin/grep -n '§2\|§3'
122:what retires; see §2 item B for the disposition and the coverage replacement.
194:second one that does not come from the manifest. §2 item D takes it and reports
227:- `docs/research/threat_calculus_v1.md` §9 — quoted in §2 item F below.
286:both are exercised in §3.
$ git show 8e7fdf0:docs/experiments/texel_gaps_CLOSURE.md | /usr/bin/grep -n '^## '
12:## §0 First actions
47:## §1 Premises, quoted at HEAD (D-477)
```

So at the reviewed revision there is **no per-item disposition**, **no record of the
measurement §1 P4 promises for item D's second referent**, **no quotation of
`threat_calculus_v1.md` §9 for item F**, and **no §3** — which is where the package's
gate evidence was to live. CLAUDE.md Closure: *"A gate or test claim in any report cites
the gate's own log output."* At `8e7fdf0` this package's record cites none, for any gate.

**What it would take to fix.** Land §2 and §3. The working tree already holds a 574-line
version; it is not this revision and a review of `8e7fdf0` cannot credit it. Per
CLAUDE.md, *"A WP is not landable while its reviews are outstanding"* — and a review
cannot adjudicate dispositions that are not in the tree it was pinned to.

---

## MAJOR

### MAJOR-1 — the fit receipt is pinned over a fixture that is no longer the kind of file the pipeline produces, and this package is what made it so

**Claim.** Item B installs "a fitted table is pinned by a fit RECEIPT over a committed
fixture" (D-657). Item D, in the same package, widened `extract.py`'s output from 19
columns to 20. The committed fixture was not widened.

**Reproducer.**
```
$ /usr/bin/grep -v '^#' tools/texel/fixtures/fit_rows_v1.txt | awk -F'\t' '{print NF}' | LC_ALL=C sort -n -u
19
$ /usr/bin/grep -v '^#' artifacts/texel_gaps/rows_real.txt | head -20000 | awk -F'\t' '{print NF}' | LC_ALL=C sort -n -u
20
$ head -1 artifacts/texel_gaps/rows_real.txt
# wp22 texel rows — a1..a6 tab … tab key_full_sha256 tab game
$ head -1 tools/texel/fixtures/fit_rows_v1.txt
# texel fit fixture v1 — the committed input the fit RECEIPT is pinned on.
```
`synth_rows` (`tools/texel/test_texel.py:35-60`) emits no `game` column, and
`test_the_committed_fixture_is_what_the_stated_generator_writes` pins that generator, so
the fixture and the shipped extractor are now pinned *apart* by two green tests.

**Why it matters.** `docs/process.md`, Dry-run discipline: *"on an input of the SAME
KIND as the registered workload — the same sort of artefact, differing only in identity
… A synthetic stand-in exercises syntax; only a real instance of the kind exercises
ATTRIBUTION."* The receipt is the mechanism that replaces property-checks-on-digits; it
is exercised on an artefact shape `extract.py` can no longer emit. Nothing catches the
skew because `read_rows` is width-blind (MAJOR-8).

**Fix.** Either regenerate the fixture through the widened writer (update `synth_rows`,
re-pin `FIT_RECEIPT_SHA256`), or state in the fixture header that it deliberately
predates the `game` column and add the width assertion of MAJOR-8 so the two shapes are
distinguishable.

### MAJOR-2 — item D's headline number is defended by nothing

**Claim.** `extract.py` now prints `extract: {written} row(s) written from {len(games)}
distinct game(s)`. `artifacts/texel_gaps/extract_real.log` records
`extract: 89805 row(s) written from 3487 distinct game(s)`, and `3 487 distinct games` is
the measured claim `docs/research/training_pipeline_2026-09.md` §1 rests its
effective-`n` argument on. **No test covers that number.**

**Reproducer** (mutant H8, run in my worktree with the package's own purge/restore
discipline):
```
tools/texel/extract.py:  games.add((index, game))  ->  games.add(game)
python3 tools/texel/test_texel.py  ->  exit 0, ALIVE
```
On the real corpus that mutant prints `218` where the receipt says `3487`, with every
gate green — and 218 is precisely the collapse the module's own comment says the
composite key exists to prevent.

`test_extract_is_driven_and_keeps_what_the_design_says_it_keeps` asserts `written == 6`
(the return value) and the `keys` column of the file; it never reads the printed line and
never touches `games`. `tools/SHELL_CHECKLIST.md` item **10**: *"A number nothing tests
is a number nothing defends."*

**Fix.** Capture `EX.main`'s stdout in that test and assert `from 4 distinct game(s)` on
the two-tranche fixture (whose two tranches carry game 0 and game 1 each, so 4 is the
answer a bare index cannot give).

### MAJOR-3 — the 8e7fdf0 fix disarmed the negative control it was fixing, and the package's own mutation run exits 1

**Claim.** `8e7fdf0`'s message says the mutant that emptied the `both` row's free
directions had survived because the intercept condition alone rejected a rescale. The fix
added `bool(directions) and` to `free_ok`. That makes the **positive** check fire — and
makes the **negative control** vacuous, because `not control_free` is `True` whenever
`directions` is empty. So M2's *registered* check still does not fire.

**Reproducer** — their harness, re-run by me in my own worktree at `8e7fdf0`:
```
$ sha256sum mutants_texel.py
9013f09ec85587c8d2734ceaba07bc3e89bb17bc6c4de203cd08c1f8c1ecb555
$ python3 ./mutants_texel.py ; echo HARNESS_EXIT=$?
…
WRONG   M2 item C: orthogonality replaced by pin-holds in the `both` branch
        registered: and the rescaled free solve FAILS the FREE-DIRECTION condition for 'T   w3=60 & sum=74', …
        got: ["row 'T   w3=60 & sum=74' is a MINIMISER over the directions its pin leaves free (…, False, True)"]
5 of 6 dead at their registered check; 1 died elsewhere; 0 alive
HARNESS_EXIT=1
```
byte-for-byte the outcome the committed log records — and the harness's own verdict is
**1**, i.e. FAIL, on the package's own evidence run.

And the vacuity is demonstrable in the other direction (mutant H4): remove the
`bool(directions)` guard *and* empty the `both` directions, and it is the **negative
control** that fires instead. Exactly one of the two checks can catch this mutant at a
time; the fix chose which one, it did not make both hold.

**This is the shape the dispatch warned about**: the finding's sentence ("the check does
not tell a minimiser from a rescale in the `both` row") is discharged, and its property
("a check that passes because there is nothing to check") is re-created one step to the
left, in the control.

**Fix.** `check(f"and the rescaled free solve FAILS the FREE-DIRECTION condition …",
bool(directions) and not control_free, …)`. Then M2 dies at its registered check and the
harness exits 0. **Not** a re-registration of M2's check after the fact — that would be
the post-hoc move CLAUDE.md forbids.

Note the underlying property IS defended: see the REJECTED section — my H1, a
subject-side mutant that makes `options.solve_pinned`'s `both` row return a rescale of
the free solve, **dies**. The defect is in the evidence's registration, not in the check.

### MAJOR-4 — the mutation harness has no RUN VOID class

**Claim.** `artifacts/texel_gaps/mutants_texel.py` spells four different outcomes as
exit 1:

| situation | line | exit |
|---|---|---|
| a mutation site has moved (harness rot) | `sys.exit(f"{label}: the mutation site occurs …")` | 1 |
| the baseline is red (environment or tree) | `sys.exit("BASELINE IS RED …")` | 1 |
| the restore did not land (harness defect) | `sys.exit(f"RESTORE DID NOT LAND …")` | 1 |
| a mutant survived (**the answer is no**) | `sys.exit(0 if dead == len(MUTANTS) … else 1)` | 1 |

`tools/SHELL_CHECKLIST.md` item **12**, obligation 1: *"A code per kind. 0 the answer is
yes, 1 the answer is no, 2 no answer was taken."* Three of the four above are "no answer
was taken". A reader of a red harness run cannot tell a surviving mutant from a full
`/tmp`. Obligation 2 (preflight) is also unmet: the harness does not check `python3`, the
suite's presence, or scratch room before it starts.

**Reproducer.** Read the four `sys.exit` sites; or run the harness with any source file
absent and observe exit 1 with a message about a mutation site, indistinguishable in a
log from a live mutant.

**Fix.** `sys.exit(2)` with a `RUN VOID:` prefix for the three environmental/harness
kinds, and say so in the module docstring.

### MAJOR-5 — the harness's restore assertion shares the referent it is checking

**Claim.** `restore()` does `git checkout -- <paths>` and then asserts
`git diff --quiet -- <paths>`. `git checkout --` restores **from the index**;
`git diff --quiet -- <paths>` compares **worktree against the index**. The check and the
thing it checks share the index, so the assertion cannot answer the question "is the
tree back at the reviewed revision?" — only "did the checkout write what the index
holds?". A staged mutant would be restored and certified.

`docs/process.md`, Criterion and defect class: *"A criterion that is a property the named
defect class PRESERVES — internal agreement between components sharing an input … passes
vacuously and is not a criterion."* D-650's own lesson is that a restore nobody checked
is the trust the defect exploited; this checks it against itself.

**Reproducer.** In a scratch worktree: mutate a file, `git add` it, run `restore()`'s two
commands — `git diff --quiet` exits 0 with the mutant still in the tree and in the index.
(The harness also never asserts the tree is clean before it starts, so it cannot notice.)

**Fix.** `git diff --quiet HEAD -- <paths>`, plus a clean-tree assertion at start-up
(voided per MAJOR-4, not failed).

The rest of the D-650-in-Python discipline is **sound and I could not break it**: `purge()`
removes every `__pycache__` under the worktree and `touch`es every `tools/texel/*.py`
before the baseline and after every restore; the touch happens after the `git checkout`
so it cannot be undone by it; each suite run is a fresh interpreter. The `.pyc`
(mtime, size) validation channel D-650's Python analogue names is closed.

### MAJOR-6 — D-662's `MEASURED` reference count is taken at the wrong revision

**Claim.** D-662 (`docs/decisions.md:1390`) states, verbatim: *"MEASURED, `git grep` at
`6749754`: 8 references in 6 files."* `docs/experiments/texel_gaps_CLOSURE.md:89-90`
makes the same claim under the same revision.

**Reproducer.**
```
$ git grep -n -F 'eval_v0_quiet_fit_weights' 6749754 -- | wc -l ; git grep -l -F 'eval_v0_quiet_fit_weights' 6749754 -- | wc -l
9
7
$ git grep -n -F 'eval_v0_quiet_fit_weights' beb91b1 -- | wc -l ; git grep -l -F 'eval_v0_quiet_fit_weights' beb91b1 -- | wc -l
8
6
```
The extra line is `docs/decisions.md:1380` — **D-657 itself**, appended in `6749754`. The
number is true at `beb91b1` and false at the revision both documents name. The CLOSURE's
P2 listing shows eight lines and silently omits the ninth, so the transcript does not
catch it either.

**Why it matters and why it is MAJOR and not MINOR.** It changes no disposition, but it
is verbatim the class `docs/process.md` says this project pays for — *"a claim CHECKED
AGAINST THE WRONG POPULATION … In every one of those the command was run and its output
transcribed faithfully"* — sitting in an append-only ADR line under the word MEASURED, in
the package whose whole subject is instruments that report the wrong number.

**Fix.** Amend D-662 to name `beb91b1`, or restate as 9-in-7 at `6749754`.

### MAJOR-7 — D-662's "FALSE of three committed documents" is false of eighteen

**Claim.** D-662: *"a blanket 'an arena config's two seats resolve to different weight
documents' would be FALSE of three committed documents — `configs/arena_wp20_label_pilot.toml`
and its dry run … and `configs/arena_smoke_v0.toml` …"*

**Reproducer.** Enumerate, rather than recall.
```
$ git ls-tree -r --name-only 8e7fdf0 -- configs/ | /usr/bin/grep -i weight
configs/eval_v0_weights.toml                      # ONE weight table is committed
$ for f in $(git ls-tree -r --name-only 8e7fdf0 -- configs/); do
    git show 8e7fdf0:$f | /usr/bin/grep -E '^[[:space:]]*weights_file'; done | LC_ALL=C sort -u
weights_file = "configs/eval_v0_weights.toml"     # all 21 engine configs name it
```
So **all eighteen** committed arena configs' seats resolve to the same weight document,
not three. On the weaker reading — seats naming the same *engine config* — it is **five**,
not three:

| arena config | seat a | seat b | same? |
|---|---|---|---|
| `arena_smoke_v0.toml` | `gate_v0` | `gate_v0` | SAME (named in D-662) |
| `arena_wp13_fair_corpus.toml` | `instrument_v0` | `instrument_v0` | **SAME (not named)** |
| `arena_wp13_fair_random.toml` | `instrument_v0` | `instrument_v0` | **SAME (not named)** |
| `arena_wp20_label_pilot.toml` | `instrument_v0` | `instrument_v0` | SAME (named) |
| `arena_wp20_label_pilot_dryrun.toml` | `instrument_v0` | `instrument_v0` | SAME (named) |
| the other 13 | — | — | differ |

The *conclusion* — the guard has no subject and is OWED — is strengthened, not weakened.
The **enumeration was never taken**, and D-662's own final clause (*"flips if a committed
arena config ever pairs two seats that must differ"*) is quantified over a population the
line did not count. Same class as MAJOR-6, one paragraph later.

**Fix.** Amend D-662 with the enumeration above.

### MAJOR-8 — `read_rows` is width-blind, and the package widened the file without adding the guard

**Claim.** `extract.py` now writes 20 columns; `fit.read_rows` reads indices 0-18 and
performs no arity check. It cannot tell a pre-package rows file from a post-package one,
and a truncated row raises a bare `IndexError` — hard rule 3 requires a *named* error for
wrong-shape input.

**Reproducer** (in my worktree at `8e7fdf0`):
```
  20 cols (what extract writes now)      -> ACCEPTED, key='abc'
  19 cols (a PRE-package rows file)      -> ACCEPTED, key='abc'
  21 cols (a future widening)            -> ACCEPTED, key='abc'
  18 cols (a truncated row)              -> IndexError (UNNAMED): list index out of range
```
The package's own new assertion is
`check("and read_rows still reads what extract writes", len(FIT.read_rows(out)) == 6)` —
it pins *tolerance* where an arity assertion was the thing to add. This is what lets
MAJOR-1's skew be invisible.

**Fix.** `if len(w) != 20: raise FitError(f"fit: {path} line {number} has {len(w)} \
column(s); extract.py writes 20 …")` and update the fixture (MAJOR-1) so the two agree.

### MAJOR-9 — the retirement guard searches a narrower string than the instrument that measured the retirement

**Claim.** `test_the_retired_candidate_chain_is_named_by_RECORDS_and_by_nothing_live`
searches `git grep -l -F -- "configs/eval_v0_quiet_fit_weights.toml"` — the full relative
path. A re-add spelling the basename escapes it. The CLOSURE's own P2 measuring command
was `git grep -n 'eval_v0_quiet_fit_weights'` — the **basename**. The defence is narrower
than the measurement.

**Reproducer** (mutant H7):
```
configs/instrument_v0.toml  +=  '# successor: see eval_v0_quiet_fit_weights.toml and instrument_quiet_fit_v0.toml'
python3 tools/texel/test_texel.py  ->  exit 0, ALIVE
```
Their own M6 adds the same reference **with** the `configs/` prefix, so it only exercises
the case the search was written for.

**Fix.** Search the basenames (`eval_v0_quiet_fit_weights`, `instrument_quiet_fit_v0`,
`arena_wp22_phase1_quiet_dryrun`). Measured at `8e7fdf0`, the basename search returns
**exactly the same file set** as the full-path search for each of the three (5, 4 and 4
files, all inside `RETIRED_MAY_NAME`), so the widening costs nothing today and closes H7.

**On `tools/SHELL_CHECKLIST.md` item 5 (index vs worktree).** The test's docstring argues
the distinction "belongs to a gate adjudicating what is ABOUT to be committed" and names
no such gate; none exists. The residual hole is narrow — `git grep` without `--cached`
reads the worktree bytes of tracked paths, so a *newly added* file is still seen; only
"staged content differs from the worktree copy" escapes, which is precisely item 5's
stated attack. I rate the choice **defensible but unargued**: it should say `--cached` is
declined because the suite is a developer-facing test rather than a pre-commit gate, and
name the residual hole. Related and **also unaddressed**: `RETIRED_MAY_NAME` is a bare
path list with no property distinguishing a record from a live document, so adding a live
path to it silences the test (mutant H10, ALIVE). A property — "nothing under `configs/`,
`crates/` or `tools/` other than this file" — is checkable; a list is not.

### MAJOR-10 — the tripwire D-663 leans on does not carry the obligation it exists to carry

**Claim.** D-663 defers the `configs/eval_v0_weights.toml` header edit and says the
deferral is *"MECHANICAL rather than remembered: `tools/texel/fit.py`'s receipt digests
`configs/eval_v0_weights.toml`, so `FIT_RECEIPT_SHA256` … goes red on any edit to it."*
The tripwire fires. Its message does not say why:

```
tools/texel/test_texel.py:
    f"{printed[0]} — re-derive with tools/texel/fit.py "
    f"tools/texel/fixtures/fit_rows_v1.txt; the fixture, "
    f"configs/eval_v0_weights.toml, the pin rule and the answer all move it"
```

A Phase 2 session that edits the weight file sees a red check whose remedy, as stated, is
"re-derive and update the constant". It does that, is green, and the provenance link at
`docs/experiments/matrix_wp22_quiet_scale.md:68` rots exactly as D-663 predicts — because
the mechanism reminds of the constant, not of the obligation.

**Reproducer.** Mutant H9 (drop `weights_sha256` from `receipt_lines`) reproduces the
failure message verbatim; it names no citing document.

**Fix.** Put the obligation in the message: *"…; if you changed
`configs/eval_v0_weights.toml`, D-663 also owes a re-quote of its digest at
`docs/experiments/matrix_wp22_quiet_scale.md:68` and four other sites."*

---

## MINOR

**MINOR-1 — CLOSURE P6's CTSS count is 12, not 9.** The document states its own command
and its own revision; I ran both.
```
$ cd /home/tom/review-wt/at_6749754 && /usr/bin/grep -rn 'CTSS' docs/ | wc -l
12
```
(`ROADMAP.md:501`; `decisions.md:1388`; `minimax_report.md:14,48,57,138,141,164`;
`sealbot_notes.md:88`; `search_next_2026-09.md:240,244,406`.) Third mis-stated MEASURED
count in this package's documents; the same class as MAJOR-6 and MAJOR-7.

**MINOR-2 — gate 21 FORCED the §9 edit; D-662 frames it as precedent.** D-662 says §9
"stops naming the three retired documents **on this document's own precedent**". Measured,
it had no choice:
```
$ git show 6749754:docs/experiments/wp22_phase1_design.md > docs/experiments/wp22_phase1_design.md
$ bash tools/governing_citation_check.sh ; echo $?
docs/experiments/wp22_phase1_design.md: 15 citation(s) checked, 3 unreproduced
  `configs/instrument_quiet_fit_v0.toml` names no file in the tree, and is not --proposes'd
  `configs/eval_v0_quiet_fit_weights.toml` names no file in the tree, and is not --proposes'd
  `configs/arena_wp22_phase1_quiet_dryrun.toml` names no file in the tree, and is not --proposes'd
1
```
The outcome is right and the amendment is honest (a REVISION 6 banner records the
retirement, points at `wp22_phase1_impl_REVIEW.md` as the surviving record, and says §1-§8
are untouched — verified: the only §9 changes are the three de-namings). But the ADR
records a discretionary reason where a mechanical one applied.

**The revision bump 5 → 6 is SAFE.** No governing document cites this document's revision;
`docs/decisions.md:1324` (D-629) cites "revision 3" and `decisions.md` is not on gate 20's
GOVERNING list, so `tools/revision_citation_check.py` never compares it — and it was
equally stale at revision 5. Gate 21 green at `8e7fdf0` (`wp22_phase1_design.md: 16
citation(s) checked, 0 unreproduced`; `0 revision citation(s) checked, 0 stale`;
`GATE21_EXIT=0`). **§9 says nothing false** that I could find: the withdrawn power
analysis, the seats paragraph and the dry-run paragraph all now describe rather than name,
and each says the document that still names it.

**MINOR-3 — gate 6 forces link 2's deletion but NOT link 3's, and that is a hole worth
recording.** Measured, one restore at a time:
```
instrument_quiet_fit_v0.toml alone       -> config_check.sh EXIT=1
  FAIL configs/instrument_quiet_fit_v0.toml  config: `eval.weights_file`: cannot read
  configs/eval_v0_quiet_fit_weights.toml: No such file or directory (os error 2)
arena_wp22_phase1_quiet_dryrun.toml alone -> config_check.sh EXIT=0
  validate_arena_config: configs/arena_wp22_phase1_quiet_dryrun.toml ok
```
`ArenaConfig::validate` checks seat *spelling* only, never existence
(`crates/pistol-arena/src/validate.rs:193-195`, D-21). So D-662's *"with
`tools/config_check.sh` reading both"* is true of reading and false of refusing: link 3's
deletion was a judgement, not a compulsion, and the chain claim's second half rests on it.
The deletion is still right (a dry-run config for a run D-637 closed unlaunched, naming a
config that no longer exists). What the package walked past is the finding underneath:
**an arena config may name an engine config the tree does not hold and every gate is
green** — the same seam shape as D-635/MAJOR-2, which is the very defect D-662 is
recording as OWED.

**MINOR-4 — "68 of 268" stands unchallenged at this revision.** I tried independently and
**could not reproduce it either**: no natural space over the schema's triples or pin pairs
has cardinality 268 (`C(n,3)` never hits it; distinct triples with a fixed sum, with a
bounded sum, with a bounded max, and with a bounded max-and-sum never hit it; the only hit
in a wide sweep is the unmotivated box `top ≤ 28, total ≤ 33`). So I second the
implementer's judgement that defining a stated space was right rather than transcribing.
**But** CLAUDE.md's Process says *"a finding that cannot be reproduced is recorded as
rejected with the attempted reproducer"*, and at `8e7fdf0` that record exists nowhere:
`docs/experiments/wp22_phase1_review_rounds_4_5.md:140` and
`docs/experiments/wp22_HANDOFF.md:167` both still carry it, D-658 does not mention it, and
the retraction is in the unlanded CLOSURE text. The sharper half — that the review said
"the flatness refusal" where the guard at issue is the *input* check at `fit.py:255`
(`training_pipeline_2026-09.md:282-283` names the input check) — is likewise unrecorded.

**MINOR-5 — item G has no ADR line and the owed-item ledger still says it is owed.**
`docs/experiments/book_v3_SUMMARY.md:166-170` lists *"the stale D-143 citation on the
arena's load path"* under **"Two things this package did not take"**; commit `8e8dda5`
took it; `sed -n '/^D-651/,$p' docs/decisions.md | /usr/bin/grep 'openings\.rs\|emission\|D-143'`
returns nothing. The change itself is correct — I enumerated every committed openings
fixture rather than trusting the four the CLOSURE names, and there are exactly four:
`openings_v1.txt:32 game_hash_asc`; `random_openings_v1.txt:38`, `_v2.txt:53`,
`_v3.txt:66` all `generation_order`. Rule 10.

**MINOR-6 — hard rule 9.** `tools/texel/test_texel.py` (1469 lines at `8e7fdf0`, grown
~380 by this package) and `tools/texel/fit.py` (520) are far over the soft cap and have no
entry in `docs/rule9_justifications.md` (`/usr/bin/grep -n 'texel' docs/rule9_justifications.md`
→ nothing). `tools/file_justification_check.sh:245` enumerates `git ls-files -s -z '*.rs'
'*.sh'`, so `.py` is outside the mechanism's scope entirely — the gate is green forever on
this class, which is the mechanised form of the very defect `docs/process.md` names.
Pre-existing; widened here.

**MINOR-7 — `tempo_constraints`' docstring under-describes the singularity.** It says
*"Two rows share the normal `[1, 0]`, so an active set holding both is singular"*. All
three normals are parallel (`[1,0]`, `[-2,0]`, `[1,0]`), so **all three** size-2 active
sets are singular; `skipped` goes from 1 to 3, not 1 to 2. The conclusion (the optimum is
still found, since a slab's optimum is interior or on one face) is unaffected.

**MINOR-8 — the receipt excludes one path for machine-independence and includes another.**
`receipt_lines`' docstring: *"The rows file's PATH is deliberately absent and its digest
present: a receipt that changed with the directory a run happened in would pin the machine
rather than the fit."* The receipt then emits `weights_file configs/eval_v0_weights.toml`.
Harmless in fact (the path is a module constant and `fit.py` fails outright if run from
elsewhere), but the stated principle and the emitted body disagree.

**MINOR-9 — D-663 names one citing site for the digest; there are five.**
`git grep -n -F 41ef549666d787bf 8e7fdf0` returns `decisions.md:482` (D-220, the OFFICIAL
`Eval::delta` bench verdict's instrument pin), `decisions.md:1320` (D-627),
`matrix_M4_snapshot_config_seam_rev3.md:75`, `matrix_wp22_quiet_scale.md:68` and
`matrix_wp22_quiet_scale_REDTEAM.md:37`. D-663's "at every citing site,
`matrix_wp22_quiet_scale.md:68` first" is correct in form and never enumerates — one
command away, D-291's shape.

**MINOR-10 — item F's `threat_calculus_v1.md` response is additive where the ruling was
corrective.** Verified: `git grep -c -F 'CTSS' 6749754 -- docs/research/threat_calculus_v1.md`
exits 1 — the document **never contained "CTSS"**, so `search_next_2026-09.md:240`'s
premise about it is false. The implementer's substitute is two **new** §9 ADOPT rows.
`ADOPT-CONSERV` is honest and says of itself *"adopts nothing new"*. `ADOPT-VC` creates a
new adoption identifier in a design source of record for forms §4/§7 already referenced
(`threat_calculus_v1.md:76-77, 82, 144`) — defensible as documentation, but it is an
addition made under a citation-correction ruling, and the false premise it was responding
to is recorded nowhere at this revision (BLOCKING-1 again: §2 item F does not exist).

**On item F's widening from two sites to eight, which the dispatch asks about
directly: the widening is RIGHT.** D-661's ground — *"'CTSS' is cited nowhere as a
technique"* — is a property of the acronym, not of two lines, so fixing two and leaving six
would leave the document asserting the retracted claim. And **no verdict moved**: line by
line, `BUILD (CTSS) / PROTOTYPE (CRZS)` → `BUILD (conservative defence) / PROTOTYPE
(CRZS)`; `BUILD CTSS-style conservative defense` → `BUILD conservative defence`;
threat-only quiescence stays **BUILD**; `sealbot_notes.md:88` stays **BUILD**. The only
substantive change is the CRZS evidence cell, which weakens the evidence and marks it
**UNVERIFIED** exactly as D-661 directs, without moving the PROTOTYPE verdict it supports.

---

## Suspicions I raised and then REJECTED, with the reproducers that killed them

Recorded per CLAUDE.md Process (*"a finding that cannot be reproduced is recorded as
rejected with the attempted reproducer"*).

- **"Item C's property is only defended by a mutant of the test itself."** M2 mutates
  `test_texel.py`, which is weaker than a call-site mutant. I built the subject-side one:
  **H1** rewrites `options.solve_pinned`'s `both` branch to return a rescale of the free
  solve holding both pins. It **dies**, at exactly the intended check
  (`row 'T   w3=60 & sum=74' is a MINIMISER over the directions its pin leaves free`,
  answer `[0.8027, 13.1973, 60.0]`). The property is defended; only M2's registration is
  wrong (MAJOR-3).
- **"The item-A test transcribes `fit()`'s composition, so a change to `fit()` would not
  be caught."** `_solved_under` restates `fit.py:404`'s
  `quiet = [answer[0], total - top - answer[0], top]`. **H11** swaps `w1`/`w2` in `fit()`
  only, **H12** swaps them in the test only. **Both die** — H11 through
  `test_fit_END_TO_END…` (`FitError: … not schema-feasible`), H12 through the item-A test
  itself (`the two-constraint set reaches the refusal … (1140, 525)`). Rejected.
- **"`_tracked_naming`'s `git grep` is CWD-scoped, so running the suite from
  `tools/texel/` would make the retirement check vacuous."** It would — but the test's own
  control (`the ruling still names {path}, or this search found nothing`, requiring
  `docs/decisions.md` in the results) fails first. Rejected; the control does its job.
- **"The added third constraint could make `constrained_min` miss the optimum, since all
  active sets of size 2 are now singular."** They are (MINOR-7), but the feasible region is
  a slab in `w1` crossed with a free `c`, whose minimiser is attained at size 0 or size 1.
  Rejected analytically and empirically (H6, an off-by-one in the same bound, dies at the
  registered check).
- **"The `bool(directions)` guard is a guard that cannot fire, of the kind
  `round_to_schema`'s own comment condemns."** It cannot fire in normal operation, but
  `all([])` is `True` and a vacuous universal is a real defect; the guard is correct. The
  finding is where it was *not* applied (MAJOR-3).

---

## Re-derivation table

Every number this package leans on, the command **I** chose, the scope, my value, and
whether it matches.

| # | claim | where it is asserted | my command (scope) | my value | match |
|---|---|---|---|---|---|
| 1 | 89 805 rows | `extract_real.log`; D-621 | `/usr/bin/grep -vc '^#' artifacts/arc3r_sweep_deduped_manifest.txt` (whole manifest) | **89805** | ✅ |
| 2 | 3 487 distinct games | `extract.py:52` comment; `extract_real.log`; `training_pipeline` §1 | awk join of manifest→corpus over all 16 tranches, `sort -u` on (tranche, field 0) | **3487** | ✅ |
| 3 | 218 distinct bare `game` | `extract.py:52` comment | same join, `sort -u` on field 0 alone | **218** | ✅ |
| 4 | "sixteen different games in one group" | `extract.py:55` comment | `sort -u \| cut -f2 \| uniq -c` histogram | 217 indices collapse **16** games, 1 collapses 15 | ✅ |
| 5 | 1140 triples, entries ≤ 20 | `test_texel.py` item-A test | awk triple loop; cross-checked `math.comb(20,3)` | **1140** | ✅ |
| 6 | 525 can bind | same | awk loop on `w1+w2>w3`; **and separately** on `total-2*top+1 > 1` | **525** both ways | ✅ |
| 7 | `FIT_RECEIPT_SHA256 = e49569ca…` | `test_texel.py:1208` | `awk` lift of the printed body + `sha256sum` (shares nothing with `fit.py`) | **e49569ca5781ffa1b7265e28e00fc75ba34874fde6b03108e967db4569835d33** | ✅ |
| 8 | fixture holds 400 rows, 19 cols | `test_the_committed_fixture…` | `grep -v '^#' \| awk -F'\t' '{print NF}' \| sort -u` | 400 rows, **19** cols (extract writes **20**) | ⚠️ MAJOR-1 |
| 9 | "8 references in 6 files" **at 6749754** | D-662; CLOSURE P2 | `git grep -n/-l -F` at `6749754` **and** at `beb91b1` | **9 in 7** at `6749754`; 8 in 6 at `beb91b1` | ❌ MAJOR-6 |
| 10 | the three-link chain | D-662; CLOSURE P2 | `git grep -n -F` on each link at `6749754` | chain confirmed: arena:47 → instrument:118 → weights | ✅ |
| 11 | "config_check reads both" ⇒ all three retire | D-662 | restored each config alone, ran `tools/config_check.sh` | engine cfg → **exit 1**; arena cfg → **exit 0** | ⚠️ MINOR-3 |
| 12 | blanket seat test "FALSE of three documents" | D-662 | enumerated all 18 arena configs' seats and all 21 engine configs' `weights_file` | **18** by weight doc; **5** by seat config | ❌ MAJOR-7 |
| 13 | "`grep -rn 'CTSS' docs/` at 6749754 returns 9 lines" | CLOSURE P6 | that literal command, in a worktree at `6749754` | **12** | ❌ MINOR-1 |
| 14 | `threat_calculus_v1.md` §9 treats CTSS as a technique | `search_next` §3.3 (premise) | `git grep -c -F 'CTSS' 6749754 -- docs/research/threat_calculus_v1.md` | **no match** — premise false | ✅ (implementer's finding, unrecorded) |
| 15 | no BUILD/PROTOTYPE/SKIP verdict moved | item F | line-by-line diff of every verdict cell in `minimax_report.md` | none moved | ✅ |
| 16 | `weights_sha256` hashes BYTES | D-663 | read `pistol.rs:97` and `:193-201` | `std::fs::read` + `sha256_hex` — bytes | ✅ |
| 17 | no golden pins the digest | D-663 | `git grep -n -F 41ef549666d787bf 8e7fdf0 -- crates/` and read all three test sites | 0 goldens; **3** sites recompute (D-663 names 2) | ✅ (under-claimed) |
| 18 | `41ef5496…` cited at `matrix_wp22_quiet_scale.md:68` | D-663 | `git grep -n -F 41ef549666d787bf 8e7fdf0` (whole tree) | confirmed; **5** citing sites total | ✅ / MINOR-9 |
| 19 | four books, one hash-ordered, three generation-ordered | item G / CLOSURE P7 | `git grep -n -F emission_order 8e7fdf0` + my own enumeration of `*openings*.txt` | exactly 4 fixtures, 1 + 3 | ✅ |
| 20 | 6 mutants, 5 dead at their registered check | `mutants_texel_8e7fdf0.log` | re-ran their harness (`sha256 9013f09e…`) in my worktree at `8e7fdf0` | identical; **harness exit 1** | ✅ (see MAJOR-3) |
| 21 | "68 of 268 admissible committed triples" | `wp22_phase1_review_rounds_4_5.md:140`; `wp22_HANDOFF.md:167` | swept fixed-sum, bounded-sum, bounded-max, bounded-max-and-sum triples and pin-pair boxes for cardinality 268 | **not reproducible** — only an unmotivated box `top≤28,total≤33` hits it | ✅ (agree: not transcribable) |
| 22 | gate 18 green at `8e7fdf0` | — | `bash tools/texel_tests.sh` in my worktree | `test_texel: all checks passed (including census_classes)`, `GATE18_EXIT=0` | ✅ |
| 23 | gate 21 green at `8e7fdf0` | — | `bash tools/governing_citation_check.sh` in my worktree | `…DESIGN_CITATION_CHECK_DONE`, `…REVISION_CITATION_CHECK_DONE`, `GATE21_EXIT=0` | ✅ |
| 24 | gate 6 green at `8e7fdf0` | — | `bash tools/config_check.sh` in my worktree (after restoring the tree) | `21 engine config(s), 1 weight table(s), 18 arena config(s)…`, exit 0 | ✅ |
| 25 | gate 3 green at `8e7fdf0` | — | `cargo test --workspace --locked` in my worktree; then the one voided binary re-run without my env override | **86** binaries `test result: ok`, **0** real failures; the single red one was a VOID my own `CARGO_TARGET_DIR` caused and it passes without it (`EXIT=0`) | ✅ |

---

## `tools/SHELL_CHECKLIST.md`, answered by number

The package's `tools/` changes are all Python (`tools/texel/{fit,extract,options,test_texel}.py`)
and one gitignored harness (`artifacts/texel_gaps/mutants_texel.py`); no `tools/*.sh`
changed. Items 1-4 and 6-9 are shell-specific and I record them as such rather than
skipping them.

1. **Command substitution whose status is discarded** — N/A (no shell). The Python analogue
   is present and correct: `subprocess.run(...).returncode` is inspected everywhere except
   `restore()`'s `git checkout`, which uses `check=True`.
2. **Pipeline in a `then` body** — N/A.
3. **`grep` under `pipefail`** — N/A, but the *substance* applies and is a finding:
   *"a substring is not a token"*, inverted. `_tracked_naming` matches a **longer** string
   than the instrument that measured the retirement, and misses the basename form
   (MAJOR-9).
4. **`LC_ALL` and guard direction** — N/A; no character classes.
5. **The index is what commits** — see MAJOR-9's closing paragraph (the test's
   worktree choice) and **MAJOR-5** (the harness's `git diff --quiet` against the index,
   which is the same item pointed at the restore).
6. **A sweep by prefix must own the prefix** — the harness deletes only `__pycache__`
   directories under its own worktree root; no shared prefix. **OK.**
7. **Traps** — N/A (no shell traps).
8. **One spelling per number, one refusal per reason** — `extract.py`'s two new refusals
   are separate and separately named (`is not a game index` / `below the previous
   record's`), and each is separately reproduced by its own test. **OK, and well done.**
9. **What reaches a record is caller-controlled** — the receipt interpolates
   `rows_path`'s *digest* and a constant `weights_path`, never a caller string, so a
   newline cannot inject a receipt line. **OK.**
10. **THE COVERAGE RULE** — `fit.py` (receipt digest) and `options.py` (option tables,
    interiority verdict) are each driven by a subprocess test on the shipped script:
    **satisfied**. `extract.py` is driven by importing and calling `main()` rather than by
    subprocess — weaker, and pre-existing. **The new recorded number is not covered at
    all: MAJOR-2.**
11. **Destructive-site containment** — the harness's only destructive site is
    `git checkout -- <paths>` where `<paths>` come from the hardcoded `MUTANTS` table, not
    from an argument or environment. **Satisfied**, and the enumeration is: one site, one
    origin, script-created.
12. **RUN VOID vs FAIL** — **MAJOR-4**. The harness has no void class at all; four
    distinct kinds exit 1. The Python suite likewise has no void class (`git` unavailable
    surfaces as an `AssertionError`, i.e. a FAIL), though `tools/texel_tests.sh` — which
    this package did not change — does void (exit 2) on a missing `python3` or suite.

    **And I produced a live instance of obligation 3 while running gate 3**, worth
    recording because item 12 exists for exactly this and the last instance cost a
    session: my `cargo test --workspace --locked` came back `TEST_EXIT=101` with one red
    binary, and the red binary was
    `crates/pistol-cli/tests/solver_determinism_gate_tests.rs:18` asserting the shipped
    script exits 0 while the script had correctly answered
    `solver_determinism: RUN VOID: no binary at target/release/solver-selftest after a
    green build` — because I had exported `CARGO_TARGET_DIR`, so the relative path the
    script looks in was empty. **The gate was honest and the test reported its void as a
    regression**, which is verbatim the D-281/D-285 reading item 12 was written to stop.
    Re-run without the override: `test result: ok`, `EXIT=0`. Out of this package's scope
    — but it is the item's own example still standing in the tree, and I am recording it
    rather than dropping it because I have the reproducer in hand.

---

## Mutation evidence: my judgement on each of their six, and on whether it could have been harder

Re-run at `8e7fdf0` in `/home/tom/review-wt/texel_8e7fdf0`, baseline GREEN from a purged
cache, output identical to `artifacts/texel_gaps/mutants_texel_8e7fdf0.log`.

| mutant | fair call-site mutant? | could it have been harder? |
|---|---|---|
| **M1** deletes the third constraint | **Yes** — the exact item-A change reverted, in the subject. | Yes, and I made it: **H6** loosens the same bound by one (`+1.0` → `0.0`) instead of deleting it, and dies at the same check with a set difference rather than a total one. A deletion mutant and an off-by-one mutant test different things; both should be in the set. |
| **M2** empties the `both` free-direction list | **No — this mutates the TEST, not the subject.** It answers "is the direction list used", not "would a wrong `solve_pinned` be caught". | **Yes, decisively.** My **H1** mutates `options.solve_pinned`'s `both` branch into the rescale the negative control constructs; it dies. That is the mutant item C's claim needs and the package did not seed. M2 should be kept *as well* (a vacuous-`all` guard is worth pinning) but re-registered against the positive check — see MAJOR-3. |
| **M3** `interior()` → `return True` | **Yes**, and clean. | Marginally: `slack > tol` → `slack >= -tol` would test the boundary semantics rather than removing the function; the current mutant kills four checks, which is a sign it is coarse. |
| **M4** receipt digest over the `pins` line alone | **Yes** — a genuine scope-narrowing of a recorded number, and the check that dies re-derives the digest rather than reading it back. Good mutant. | Slightly: dropping a *body line* (my **H9**) is the likelier real defect and also dies. |
| **M5** disables the non-decreasing check | **Yes.** | **Yes.** It only reverts the guard. The two mutants that test whether the guard is pointed at the right thing are **H2** (`GAME = 0` → `1`, the adjacent-field misread the comment names) and **H5** (the `previous_game` reset hoisted out of the tranche loop). Both die — but neither was seeded, so the package's evidence does not show that the referent is aimed correctly, only that it exists. And **H8** — the mutant that matters for item D's recorded number — is **ALIVE**. |
| **M6** re-adds the retired path to a live config | **Yes in form**, straw in spelling: it uses the full `configs/…` path, which is the only form the test can see. | **Yes.** My **H7**, the same re-add by basename, is **ALIVE**. M6 as written cannot distinguish a working guard from one that matches too narrowly. |

**On the per-row referent (dispatch question 2).** `game` non-decreasing over ascending
record numbers within a tranche is **externally derived in `docs/process.md`'s sense**: it
comes from the corpus's emission order, which the manifest (the join's own referent) does
not carry — the manifest's stated columns are `corpus_index, record_number, key_seq,
key_pos, key_full, depth_turns, result, end`, so the referent and the suspect input do not
share a source. And `turns_played` is a *measured* negative control, not an asserted one:
it restarts at every game, and H2 shows the check fires on it. **This is the strongest
piece of methodology in the package.**
What it does **not** catch: (a) any misread whose column is also non-decreasing within a
tranche — `depth_turns` is not, `record_number` would be but is not adjacent; (b) a
constant column (all zeros passes trivially); (c) an off-by-one *record* offset that
preserves ordering (the `key_full` join covers that, so the two checks together are
complementary rather than redundant); (d) the composite spelling itself — H3 covers that
and dies, but on the *keys* check, not the ordering check.

---

## The specific calls, answered

1. **Composite `<corpus_index>:<game_index>` vs the bare field.** Both numbers reproduce
   (3487 / 218) and the collapse really is 16-to-1 for 217 of 218 indices, so a bare column
   would not be a grouping key. **The composite is the right call in substance.** Two
   things are wrong with it as executed: `corpus_index` reaches the row file **only** inside
   that string, so a trainer that wants the raw field must parse `extract.py`'s spelling —
   two columns (`corpus_index`, `game`) would have carried the same information without
   editorialising, and D-659's words are "keeps … `game`". And yes, **20 columns where
   `fit.py` reads 19 is a latent trap**, and it has already bitten: MAJOR-1 and MAJOR-8.
2. **The per-row verification.** Externally derived; measured negative control. See above.
   Best thing in the package.
3. **TX-1 as a chain of three.** The chain is real (verified). Link 2's deletion is
   **forced** (gate 6, exit 1, measured). Link 3's is **not forced by any gate** (exit 0,
   measured) — it is right, but it is scope D-657 did not authorise and D-662, which
   authorises it, is self-appended by the implementing session. The Rust edit under
   `crates/` is a **test deletion plus a doc comment**, not engine behaviour, so the
   dispatcher's "no engine crate changes" is met in substance and breached in letter; both
   edits are recorded (the test deletion in D-662, the comment nowhere — MINOR-5).
4. **The revision-6 amendment.** Honest, forced by gate 21, safe to bump, and §9 says
   nothing false. See MINOR-2.
5. **The seat-pair guard recorded as OWED.** The disposition is right; the count is wrong
   by a factor of six (MAJOR-7).
6. **Item E deferred.** Right, and for the reason given. The condition is measured
   correctly, no golden churns, and honouring a pre-registered conditional branch is
   exactly the discipline. A stale "Stage 4" comment in a config is *not* worse than
   breaking the only content link between the labelled corpus and the weights that labelled
   it — especially since the engine reads values, not bytes, so the edit would move an
   identity without moving a move. The gap is MAJOR-10.
7. **1140 / 525 vs 68 / 268.** I reproduced 1140 and 525 independently and **failed to
   reproduce 268 by any principled enumeration**, so defining a stated space was the right
   move and not a dodge — the structural finding (the constraint set is not the schema) is
   fixed and mutation-verified, and materiality is re-established on a space anyone can
   recount. `A = I, b = 0` **is** a fair stand-in: it forces the unconstrained optimum below
   every lower bound the schema can state, which is the only regime in which the missing
   relation decides anything; the constraint set is data-independent, so a row-driven
   instrument would only sample that regime by luck. The residual weakness is that the test
   restates `fit()`'s composition rather than calling it — checked (H11/H12) and rejected.
   What is owed is the **record** of the non-reproduction (MINOR-4).
8. **The receipt digesting `configs/eval_v0_weights.toml`.** A feature, not a booby trap:
   the weight file is a genuine input to the fit (it supplies both pins), so a receipt that
   ignored it would pin less than the run. It becomes a trap only because the failure
   message does not carry the obligation (MAJOR-10).
9. **F changing seven sites where two were named.** Right, and no verdict moved. The
   `threat_calculus_v1.md` premise **is** false and the implementer found it; two new ADOPT
   rows are a defensible but additive response, and the false premise is recorded nowhere
   at this revision (MINOR-10, BLOCKING-1).
10. **`git grep` without `--cached`, and the allowlist.** The worktree choice is
    defensible but its stated argument names a gate that does not exist; the residual item-5
    hole is narrow and real. The **allowlist** and, more importantly, the **search string**
    are the weaker halves: H7 (basename) and H10 (allowlist widening) are both ALIVE.
    MAJOR-9.

---

## What I could not check

- Nothing about gate 3 — I ran it to completion; see the note below. The package's own
  record asserts gates 3, 6, 18 and 21 in a §3 that does not exist at `8e7fdf0`, so **the
  only cited gate logs for this revision are the ones in this report**.
- **`extract.py` was not re-run over the 16 real tranches.** I re-derived its two headline
  counts by an independent awk join of the manifest against the corpus instead, and
  verified the committed `artifacts/texel_gaps/rows_real.txt` header and column count.
  Digests as I read them: `rows_real.txt` `eb4264a945e9…` (matches
  `RECEIPT_texel_gaps.sha256`), `mutants_texel.py` `9013f09ec855…` (matches). Note the
  receipt file itself changed on disk during the review (a
  `mutants_texel_d48174f_M2ALIVE.log` line was added), which is the implementing session
  still working; `artifacts/` is gitignored and none of it is in the reviewed revision.
- **Whether "CTSS" has a defining paper, and whether CRZS solves 100 % of its puzzle set.**
  No PDF access. The package marks the second UNVERIFIED, which is the right disposition
  for an unread source.
- **Whether D-633's "six admissible tables" is exactly six.** Not re-enumerated; the item-B
  argument does not turn on the count.
- **The unlanded 288-line CLOSURE continuation.** Out of scope for a review pinned to
  `8e7fdf0`; several findings above (BLOCKING-1, MINOR-1, MINOR-4) appear to be what that
  text addresses, and a re-review at the revision that carries it is owed either way,
  because an amendment reopens the review however small the diff.
- **Whether the operator intends `docs/research/training_pipeline_2026-09.md` to be a
  record rather than governing.** I treated it as a record (D-651: "none selects, none is a
  governed run"), which is what makes its presence in `RETIRED_MAY_NAME` acceptable. If it
  is ever listed in `tools/governing_citation_check.sh`'s GOVERNING array, that allowlist
  entry becomes a defect.
