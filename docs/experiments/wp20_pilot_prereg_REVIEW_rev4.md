# SCOPED RE-CHECK — `docs/experiments/wp20_pilot_prereg.md` revision 4

## Header

- **Revision adjudicated**: `950cad1d24008298f80f80137e762104dc50c5ff` (branch `dev`).
- **Matches HEAD**: YES. `git rev-parse HEAD` → `950cad1d24008298f80f80137e762104dc50c5ff`.
- **Tree state**: `git status --porcelain` is **EMPTY**.
- **Scope**: the 15 findings of `docs/experiments/wp20_pilot_prereg_REVIEW_rev3.md`
  (revision 3 = `ff290ab`, verdict FAIL, 0 BLOCKING / 5 MAJOR / 10 MINOR); the
  recurrence class *a remedy that fixes a sentence and leaves its contradiction
  standing elsewhere*; the two new code changes; MAJ-1's replacement enumeration
  against the tree; MIN-9's new manifest; and anything new.
- **Round**: three of the four D-552 allows, remedies-only. This is the check on it.
- **Reviewer**: fresh context. I wrote none of this and owe it no deference. Every
  verdict below is derived from `git diff ff290ab..950cad1` and from the document and
  the tree as they read — **never** from §0.1's change table (D-550).

**What I read.** `docs/experiments/wp20_pilot_prereg.md` (all 1 056 lines);
`docs/experiments/wp20_pilot_prereg_REVIEW_rev3.md` (all 568);
`git diff ff290ab..950cad1` in full (356 lines of document diff plus code, ledger and
the new manifest); `docs/experiments/wp20_pilot_artifacts.md` (all 49);
`crates/pistol-arena/src/bin/corpus-check.rs` (all 157) and the same file at `85e6261`;
`crates/pistol-arena/tests/labels_tests.rs` (helpers, `Scratch`, `checked`, `corpus_of`,
`rows`, and both new tests); `crates/pistol-arena/tests/common/mod.rs` (`Scratch`);
`crates/pistol-arena/src/passes.rs` (all 86); `crates/pistol-arena/src/bin/arena.rs`
(`main`, `dispatch`, `count_of`, `workers_of`, `replay_pass`, `run`'s two exit
branches); `crates/pistol-arena/src/outpath.rs` (all 45);
`crates/pistol-arena/src/capture.rs` (refusal inventory, `run`'s head);
`crates/pistol-arena/src/labels.rs` (refusal inventory, `run`'s two header checks);
`crates/pistol-arena/src/labels_file.rs:240-300` (the four token sets, column order);
`docs/experiments/wp20_dispatches.md:118-130` and `:325-336`;
`docs/book_v2_ledger.md:38-53`; `docs/decisions.md` D-552, D-553, D-554;
`artifacts/wp20pilot_dryrun_85e6261_v1.txt` (all 82 lines); `CLAUDE.md`.

**What I ran** (read-only; **no `cargo` in any form and no `tools/ci.sh`**):

- `git rev-parse HEAD`, `git status --porcelain`, `git log --oneline -6`
- `git diff ff290ab..950cad1` (whole), `git diff --stat 85e6261 950cad1 -- configs/ tools/ crates/`
  → **NOT empty** (see MAJ-A), against `git diff --stat 85e6261 ff290ab -- configs/ tools/ crates/`
  → empty
- `git show 85e6261:crates/pistol-arena/src/bin/corpus-check.rs`
- `sha256sum` over `artifacts/wp20pilot_*.txt` and all of `/home/tom/pistol-runs/wp20pilot-dryrun2/*.txt`
- a read-only `python3` re-derivation over the four dry-run corpora: for each budget,
  `depth_turns` median / mean / min / max, the even-count middle pair, and the observed
  value sets of `to_move`, `book`, `result`, `end`, `score_kind`
- `/usr/bin/grep` and `git grep` throughout (D-265)

**Where a build would settle a claim.** No finding below rests on one. The claim I did
not settle by execution is that the two new tests in
`crates/pistol-arena/tests/labels_tests.rs` pass; the run that would settle it is
`cargo test -p pistol-arena --locked --test labels_tests` in a detached
`git worktree add --detach` on `/home` with its own `CARGO_TARGET_DIR`. I read both
tests and their helpers instead, and report what they can and cannot pin.

---

## VERDICT: **FAIL**

| severity | count |
|---|---|
| **BLOCKING** | **0** |
| **MAJOR** | **5** |
| **MINOR** | **6** |

**Eleven of the fifteen findings are cleanly applied, and three of those remedies are
better than what was asked for.** MAJ-2, MAJ-3, MAJ-5, MIN-5, MIN-6, MIN-7, MIN-9 and
MIN-10 are dead and I could not revive any of them: the retracted totality claim is
corrected at its head, §2 no longer restates a rule it does not own, the item-9 guard
has a call-site driver over reachable input, the deviation cost is 32.5 % and both
dispatch quotations are now exact, every digest in the new manifest recomputes, and
"zero forfeits" is read from a code that is `0` if and only if the tally is zero.

**It fails on four remedies that repaired their own finding and broke something else,
and one previously-unfound false measurement claim.** The class the round was
dispatched to kill is still alive and has moved to new sentences: MIN-8's `book` fix
changed a registered instrument without moving its governing revision (MAJ-A); MAJ-4's
rewrite added a reconciliation whose arithmetic proves the opposite of what it asserts
(MAJ-B); MAJ-1's enumeration contradicts §5's own void class one paragraph above it
(MAJ-C) and is not exhaustive over the refusals it claims to close (MAJ-D); MIN-4's
strengthened sentence is false of the artifact it describes (MAJ-E).

---

## DISPOSITION OF THE 15 PRIOR FINDINGS

| # | finding | disposition | evidence now standing |
|---|---|---|---|
| **MAJ-1** | §5's V7-B discriminator does not exist | **APPLIED BUT INTRODUCED A NEW DEFECT** | the false discriminator is retracted by name at `:441-448` and replaced by a five-item enumeration at `:449-459`. I verified all five fire before the walk (`capture.rs:243-244`, `labels.rs:304-318`, `arena.rs:69`, `passes.rs:13-32`) — **none of the listed items can fire mid-walk**, which was the other half of the check. But the enumeration is stated as an **iff** that contradicts `:428` (**MAJ-C**) and is not exhaustive (**MAJ-D**). |
| **MAJ-2** | the retracted totality claim still standing | **APPLIED** | `/usr/bin/grep -n "Both rules are total"` now returns **one** hit, `:546`, inside the quotation that retracts it. `:484-487` reads *"RULE-2 and RULE-3 are total … RULE-1 is NOT"*. `:546-553` says *"Revision 2 said"* — true — and adds the accurate account that *"revision 3's first attempt at this remedy wrote the denial and left the assertion standing"*. |
| **MAJ-3** | §2 stated the pre-amendment RULE-1 | **APPLIED** | `/usr/bin/grep -n "largest take"` → no hit. `:138-145`: *"THE SIZE IS DERIVED FROM A RULE, AND THE RULE IS §6.1's RULE-1 — WHICH THIS SECTION DOES NOT RESTATE … revision 3 amended RULE-1 in §6.1 and left this paragraph carrying the superseded form, which returns 56 rather than 13."* The remedy went past "point at §6.1" and named the drift. |
| **MAJ-4** | finding 3 kept 2.4 % as a finding of a superseded run | **APPLIED BUT INTRODUCED A NEW DEFECT** | `:840-848` now keeps both readings with the budget and artifact each belongs to, which is the half that was asked for. The added reconciliation *"The two do not disagree"* is false and self-refuting. **MAJ-B.** |
| **MAJ-5** | the item-9 guard landed with no test | **APPLIED** | `a_corpus_path_carrying_a_control_character_is_a_void_before_it_is_printed` (labels_tests.rs:676-700) writes `corpus\nwith-a-newline.txt` through `Scratch::write` (`std::fs::write`, legal on Linux), passes it via `Command::args` (no shell, the newline survives), asserts `Some(2)`, asserts stderr names *"control character"* and *"NOT a refusal"*, **and asserts `!stdout.contains("record(s)")`** — the guarded path did not reach stdout. Both the guard-removed and the call-removed mutant die on the exit code. **D-553 discharged at the call site.** |
| **MIN-1** | §4E untouched; the false §10 cross-reference | **APPLIED — both halves** | `:363-369` registers the summary line and *"C-E's reach is whatever that line says and no more"*; `:830-831` now reads *"§4E's run 1 registers that its reach is whatever the line reports"* — the §10 half of the false cross-reference is gone. |
| **MIN-2** | C-D named `date +%s.%N` | **APPLIED** | `:328-331`: *"The `SECONDS` brackets around each pass in §8's command block — bash's own integer counter, which is what §8 registers and the only timer that block contains"*. `/usr/bin/grep -n "date +"` → no hit. |
| **MIN-3** | "four measured costs" is six, one not positive | **APPLIED** | six at `:475`, `:621`, `:756`, `:813`; the transform row `:631` reads *"an UPPER bound, not a positive measurement"*; `:756-759` and `:813-816` state five positive and one upper bound. One residue at `:678,683` (**MIN-b**). |
| **MIN-4** | §8 had no replay/transform brackets | **APPLIED BUT INTRODUCED A NEW DEFECT** | both brackets are in the registered block (`:908-910` and `:923-925`). The sentence rewritten alongside them, *"differs from it in two ways and no others"* (`:975-977`), is false. **MAJ-E**, and **MIN-a**. |
| **MIN-5** | 65 % overstated 2×; both quotations inexact | **APPLIED** | `:249-253`: `82.5 T` of `253.5 T` = **32.5 %** (I recomputed: 82.5/253.5 = 0.3254), with the parenthetical distinguishing the 65 % at `:583`. Both quotations verified verbatim against `wp20_dispatches.md:123-124` (*"the re-run determinism receipt on a sub-range"*) and `:331-332` (*"the determinism re-run receipt on a sub-range"*). |
| **MIN-6** | ledger row cited revision 2 | **APPLIED** | `docs/book_v2_ledger.md:41` now ends `… wp20_pilot_prereg.md revision 4`. |
| **MIN-7** | floor (b) applied to an upper bound | **APPLIED** | `:563-571` states the slack, why it is accepted rather than guarded (floor (b) is not a criterion), and that *"The closure reports the pilot's ACTUAL asked-position count beside the rate"*. One unstated assumption inside it (**MIN-f**). |
| **MIN-8** | the summary omitted `book` | **APPLIED BUT INTRODUCED A NEW DEFECT** | `corpus-check.rs:78-94` collects and prints `book`; all four loader token sets (`labels_file.rs:258-261`) are now reported, and `the_summary_reports_every_closed_set_column_the_loader_checks` (labels_tests.rs:704-716) pins it. **`median`'s even branch is unchanged and still correct** (`:42-46`, averaging `sorted[n/2-1]` and `sorted[n/2]` over the slice sorted at `:61`). But the instrument was changed at the registration commit while §1 and §4E still pin it at `85e6261`. **MAJ-A.** |
| **MIN-9** | SLOT A sha-indexed nowhere committed | **APPLIED** | `docs/experiments/wp20_pilot_artifacts.md` is a committed manifest under rule 8. **I recomputed all 14 digests and all 14 match**, including the SLOT A row (`4e716a4f…`) and both pairs the document claims equal: `capture_400000.txt` = `capture_400000_b.txt` = `5fe1f1a3…`, `corpus_400000.txt` = `corpus_400000_b.txt` = `099489f0…`. One false sentence in it (**MIN-c**) and one omission (**MIN-e**). |
| **MIN-10** | "zero forfeits" read from an absence | **APPLIED** | `:303-310` reads a nonzero count off the CONDITIONAL clause and zero off the completed pass's exit code. Verified against `arena.rs:234-243`: on the `None` (completed) branch, `ExitCode::SUCCESS` iff `score::tally(...).forfeits == 0`. True of the tree. |

**Counts**: 11 APPLIED, 4 APPLIED-BUT-INTRODUCED-A-NEW-DEFECT, 0 PARTIALLY APPLIED,
0 NOT APPLIED.

**On scope.** The round is scope-compliant under D-552. `git diff --stat ff290ab..950cad1`
touches six files and every one maps to an enumerated remedy: the prereg, the ledger row
(MIN-6), `corpus-check.rs` (MIN-8), `labels_tests.rs` (MAJ-5, MIN-8), the new manifest
(MIN-9), and the rev-3 review report itself. No settled prose was re-opened.

---

# MAJOR

## MAJ-A — MIN-8's remedy changed a REGISTERED INSTRUMENT at the registration commit, and §1 still pins it at `85e6261`; §4E now registers a receipt line that revision cannot print

**Files**: `docs/experiments/wp20_pilot_prereg.md:97,104-108` (§1) and `:348,363-366`
(§4E), against `crates/pistol-arena/src/bin/corpus-check.rs:75-94`.

**What the document says.** §1's instrument table, row 4:

> | the corpus loader, **and the instrument RULE-2's depth table is read from** | `crates/pistol-arena/src/bin/corpus-check.rs` | `85e6261` |

and immediately below it:

> **SLOT R1 IS ONE VALUE AND IT IS FILLED: `85e6261`** … **The dry run of §7.1 ran AT
> that commit with a clean tree** … the commit that transcribes its numbers into this
> section **changes no instrument**, so `85e6261` remains the revision that governs
> every artefact below. … **a change to any of them reopens this document**
> (`docs/process.md`, "Instrument governing revision").

§4E repeats the pin at `:348` — *"`crates/pistol-arena/src/bin/corpus-check.rs` at
`85e6261`"* — and then registers, at `:363-366`:

> the `corpus_check:   depth_turns … ; score_kind …; to_move …; **book …**; result …;
> end …` line names how many values each closed-set column actually took

**What is wrong.** Three things, and they compound.

1. **The commit DOES change an instrument.** `git diff --stat 85e6261 950cad1 -- configs/ tools/ crates/`:

   ```
   crates/pistol-arena/src/bin/corpus-check.rs | 11 ++++++-
   crates/pistol-arena/tests/labels_tests.rs   | 45 +++++++++++++++++++++++++++++
   ```

   The same command at `ff290ab` was empty, which is the fact the prior review verified
   and the fact `:107`'s sentence rests on. MIN-8's remedy spent it. The sentence
   *"changes no instrument"* was true of `ff290ab` and is false of the revision that
   asserts it.

2. **`85e6261`'s `corpus-check` cannot print the line §4E registers.**
   `git show 85e6261:crates/pistol-arena/src/bin/corpus-check.rs` line 76 reads
   `"depth_turns median {:.1} mean {:.4} min {} max {}; score_kind {}; to_move {}; result {}; end {}"`
   — **no `book`**. SLOT A confirms it from the other side: every
   `corpus_check:` summary line in `artifacts/wp20pilot_dryrun_85e6261_v1.txt`
   (lines 26, 34, 42, 50) ends `result 1 (capped); end 1 (normal)` with no `book` field.
   So §4E's registered instrument, at its registered revision, cannot produce §4E's
   registered output. The document registers a criterion against a program that does not
   exist at the revision it names.

3. **By §1's own rule, the document is reopened.** `:108` states the consequence
   itself: *"a change to any of them reopens this document"*. The change is inside the
   revision under review.

**Why it matters.** This is the recurrence class the round was dispatched to kill,
arriving through the smallest finding on the list. `docs/process.md`'s "Instrument
governing revision" and `CLAUDE.md`'s *"a pre-registration is reviewed at the revision
that GOVERNS the run — that revision must itself pass a fresh-context review before the
first run it governs"* both bind here: the depth table that fixes SLOT S2 — 65 % of the
pilot's wall — is attributed to a loader revision that is no longer the one on disk, and
an operator running §8 at HEAD gets a C-E control line that does not match the one §4E
registers while §7.1's cited artifact matches neither. It is not a wrong NUMBER: I
recomputed the four medians, means, minima, maxima and every closed-set spread directly
from the corpora with an implementation sharing no code with the program, and all of
them reproduce (`3.0/2.7195`, `3.0/3.0366`, `3.0/3.3049`, `4.0/3.6341`; `book` takes two
values, `no` and `yes`, at every budget, so the new field would read `book 2 (no,yes)`).
It is a governing-revision defect, and it is the one §1 exists to prevent.

**How I reproduced it.** `git diff --stat 85e6261 950cad1 -- configs/ tools/ crates/`;
`git show 85e6261:crates/pistol-arena/src/bin/corpus-check.rs | sed -n '75,86p'`;
`/usr/bin/grep -n "corpus_check:   depth" artifacts/wp20pilot_dryrun_85e6261_v1.txt`;
`sed -n '104,108p;348p;363,366p' docs/experiments/wp20_pilot_prereg.md`.

**Minimal remedy.** Move SLOT R1 for the loader row (and §4E's `:348`) to the commit
that lands this revision, and replace `:106-108`'s clause with the true one: *"the
commit that transcribes these numbers also changes `corpus-check`'s summary line — the
depth, mean, min and max fields are untouched and SLOT A's four rows stand; the `book`
field is added and is therefore on the PILOT's face and not the dry run's."* One
sentence and two cells, and it makes §4E's registered line reachable.

## MAJ-B — §7.2 finding 3's new reconciliation asserts agreement and its own arithmetic shows disagreement, and SLOT A closes the noise escape

**File**: `docs/experiments/wp20_pilot_prereg.md:840-848`.

**What the document now says**:

> At `nodes 200000` … the cold ask cost 0.518 s against the in-process ask's 0.506 s —
> **2.4 %**. At `nodes 400000` … both are **1.006 s** and the difference is below the
> instrument's one-second resolution. **The two do not disagree**: 2.4 % of 0.5 s is
> 12 ms, which at 164 asks is **about 2 s** and would not separate two integer-second
> readings of 165.

**What is wrong.** The final clause is false, and the paragraph refutes itself in one
sentence. `SECONDS` is an integer counter; `$((SECONDS - t))` carries at most ±1 s of
quantisation. Two true durations differing by ~2 s **cannot both read 165**: if the
capture is 165.x and the cold pass 167.x, the readings are 165–166 and 167–168. A 2 s
systematic difference is exactly what a one-second-resolution instrument resolves —
that is what "one-second resolution" means. The document computes the predicted
difference (≈2 s), observes zero, and declares the two consistent.

The obvious rescue — *run-to-run noise on a 165 s wall exceeds 2 s, so one pair of
readings cannot resolve it* — is not the argument made (the stated ground is the
instrument's resolution), and **SLOT A itself forecloses it**:
`capture_400000 seconds=165` (line 47) and `capture_rerun seconds=165` (line 55) are the
same 164 asks run twice, and they agree to the second. This machine's run-to-run spread
on this workload is under a second, which makes a 2 s systematic offset MORE visible,
not less.

Under the ratio reading the gap is worse: 2.4 % of the `400000` ask cost (1.006 s) is
24 ms, ×164 = ~4 s.

**Why it matters.** The prior review's MAJ-4 remedy offered two endings and the document
took neither: *"the two do not agree and the disagreement is recorded rather than
averaged"*, or delete finding 3 and let §6.3 own the number. Instead the document
manufactures agreement. The coldness cost is the number `docs/experiments/wp20m_design.md`
§12 declines to guess and D-542 flags; a closure reading §7.2 is now told the 200000
reading and the 400000 reading are reconciled, when what the two artifacts actually show
is that the `200000` pair predicts an effect the `400000` pair does not exhibit. That is
a finding about the measurement, and it is being spent to make a paragraph tidy — the
same move MAJ-4 was raised against, one revision later.

Note the true reading is *stronger* for this document, not weaker: at the chosen budget
the coldness overhead is not merely unresolved, it is bounded above by the instrument at
under ~1 s over 164 asks, i.e. under 0.6 % — which is a better answer than 2.4 % and is
what §6.3:664-668 already says without the reconciliation.

**How I reproduced it.** `sed -n '840,848p' docs/experiments/wp20_pilot_prereg.md`;
`cat -n artifacts/wp20pilot_dryrun_85e6261_v1.txt` lines 39, 47, 55, 68; arithmetic:
83/164 = 0.5061, 85/164 = 0.5183, ratio 1.0241; 0.012 × 164 = 1.97 s; 0.024 × 164 = 3.9 s.

**Minimal remedy.** Replace *"The two do not disagree: … would not separate two
integer-second readings of 165"* with the true statement: *"The `200000` pair predicts
about 2 s of cold overhead at 164 asks; SLOT A's `400000` pair shows none, and the same
block's two identical `165` capture readings show the instrument would have separated
2 s. The disagreement is recorded, not averaged: at the chosen budget the overhead is
bounded above by this instrument at under 1 s over 164 asks."*

## MAJ-C — §5's new "if and only if" contradicts §5's OWN void class two paragraphs above: a filesystem filling mid-pass is V8 by `:428` and V7-B by `:449`

**File**: `docs/experiments/wp20_pilot_prereg.md:449-461` against `:425-429`.

**What the document says.** The void class, `:428-429`:

> - the machine taking the run away: **a filesystem filling**, a process killed, a
>   reboot, or the session ending mid-pass. `/tmp` on this machine is a 24 GiB
>   RAM-backed tmpfs and **its exhaustion is the recorded instance** (D-281, D-285);

and, `:449-461`:

> A capture or labels run exiting `2` is a VOID **if and only if** its refusal is one
> of: [five items] … **Every one of those is decided before the first ask. Any other
> refusal is V7-B and STOPS the arc.**

**What is wrong.** A filesystem filling during pass 2 or pass 3 is a capture or labels
run exiting `2`, and its refusal is in none of the five. `passes.rs:45-48` and `:75-78`
map a failed `write_all`/`flush` to `ArenaError::io`; `arena.rs:76-88` abandons the
claim and returns `Err`; `main` prints `arena: <why>` and returns `REFUSED` = `2`. So
the iff classifies it **V7-B, a STOP of the whole arc**, while `:428` classifies the
same event **V8, a VOID with one re-run** — and `:428` calls it *"the recorded
instance"* on this machine.

Exclusion (i) at `:435-438` pushes the same way (*"`arena` also exits `2` when a pass
fails PART-WAY … That is the pipeline refusing its own input and it is V7-B"*), which
makes two sentences against one. The `/tmp` case is not hypothetical here: `CLAUDE.md`'s
Environment section records tmpfs exhaustion as a live failure mode, and the pilot's
`ART` is `/home/tom/pistol-runs/wp20pilot-artifacts` while the cold check spawns 164
processes.

**Why it matters.** V7-B and V8 have opposite consequences — STOP the arc versus ONE
re-run — and this is the precise defect MAJ-1 was raised against, restored by MAJ-1's
own remedy in a stronger logical form. Revision 3's discriminator was inoperable;
revision 4's is operable and gives the **wrong** answer for the one environment fault
the document names as recorded. An operator whose disk fills at minute 40 of a
55-minute pilot reads `:428` and re-runs, or reads `:449` and returns the package to the
architect, and the document supports both.

**How I reproduced it.** `sed -n '420,462p' docs/experiments/wp20_pilot_prereg.md`;
`sed -n '40,60p;62,86p' crates/pistol-arena/src/passes.rs`;
`sed -n '20,30p;76,90p' crates/pistol-arena/src/bin/arena.rs`.

**Minimal remedy.** One clause on the iff: *"…if and only if its refusal is one of the
following, OR the machine took the run away (third bullet above), which reaches `arena`
as an I/O error and is a void wherever it lands."*

## MAJ-D — §5's enumeration claims exhaustiveness over the refusals decided before the first ask, and four such refusals fall outside it

**File**: `docs/experiments/wp20_pilot_prereg.md:449-461`.

**What the document says**: *"**The discriminator is the ENUMERATION, which is short
enough to be exhaustive.** … **Every one of those is decided before the first ask. Any
other refusal is V7-B and STOPS the arc** … The list is the criterion; a refusal a
reader cannot place in it is itself a finding about this document."*

**What is wrong.** The forward direction holds — I checked all five and **none can fire
mid-walk**: `capture.rs:243-244` calls `one_engine` and `replay::verify_engines` as the
first two statements of `run`; `labels.rs:304-318`'s two header checks precede the
record loop; `outpath::claim` is `arena.rs:69`, before `dispatch`'s `match &mode`;
`read_report` (`passes.rs:13-32`) precedes both walks. The converse does not. These four
refusals are decided before any ask and are in none of the five bullets, so the
registered rule makes each of them a STOP:

1. **`--labels` reading the capture file** — `passes.rs:70-71`,
   `std::fs::read_to_string(capture_path)`. A missing, unreadable or non-UTF-8 capture
   yields `reading <path>: <io>` and exit `2`. Bullet 1 covers *"the source"* via
   `read_report`, which in labels mode is the **`--report`** path; the capture path is
   read by a separate call with no regular-file guard at all — so a FIFO there blocks
   forever, which is the D-252 hazard `read_report`'s own doc comment exists to refuse.
2. **`outpath::claim` failing for any reason other than "already claimed"** —
   `arena.rs:69` and `outpath.rs:10-25`. `create_new` also fails on a missing parent
   directory, a permission denial or a full filesystem, and all three land on the same
   message. Bullet 4 names only *"already claimed"*.
3. **Argument refusals** — `arena.rs:59-65` (`_ =>`), `count_of` (`:112-126`) and
   `workers_of` (`:97-104`). A budget word spelled `0400000`, a `--stride` in the wrong
   place, a `--workers 0`: exit `2` before anything.
4. **`read_report`'s `metadata` failure** — `passes.rs:14-15`. A `--report` or
   `--capture` source that does not exist yields `reading <path>: <io>`, which is
   neither *"not a regular file"* nor *"not UTF-8"*.

There is also a **second, differently-populated list of the same closed class in the
same section**: `:425-427`'s void bullet 2 glosses *"a document refused on read"* with
four items (`nodes` budget, two seats, drifted digest, `--out` claimed) where `:449-459`
has five. `CLAUDE.md` D-423 is explicit — *"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT
WAITING — state it once, in the section that owns it, and have every other section point
there instead"* — and the two statements already differ.

**Why it matters.** The document says *"a refusal a reader cannot place in it is itself
a finding about this document"*, which is the right instinct; but these four are
refusals a reader CAN place in it, wrongly, and every one of them is "I could not look"
being read as "the pipeline is wrong". Case 2 in particular is reachable on the first
command of §8: the block does `mkdir -p "$ART"` but a second run over the same `ART`
hits `claim` on `report_v1.txt`, which IS bullet 4, while a typo in `ART` is not.

**Minimal remedy.** Make the enumeration a rule rather than a list, since one exists:
*"a refusal raised before the walk begins — the source or the capture unreadable, the
report not one this build reads, the seats not attesting one engine, the `--out` path
unclaimable, `--labels`' two header checks, or a command line this build will not
parse."* Then delete `:425-427`'s duplicate gloss and point it here (D-423).

## MAJ-E — §8's strengthened SLOT C claim is false, and what it hides is the provenance receipt §7.1 used to supersede the predecessor artifact

**File**: `docs/experiments/wp20_pilot_prereg.md:973-979` against `:787-796` and
`artifacts/wp20pilot_dryrun_85e6261_v1.txt:1-2`.

**What the document now says**:

> the dry run's instance of the same block is SLOT A (§7.1), which **differs from it in
> two ways and no others**: the config it names, and the four-budget RULE-2 sweep the
> pilot does not repeat.

**What is wrong.** SLOT A's first two lines are

```
revision 85e62613c358b105adfb5d068c5ca10084d24c38  tree 0 modified
engine sha256 180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476
```

and §8's registered block **produces neither**. `/usr/bin/grep -n "rev-parse\|tree 0
modified\|engine sha256" docs/experiments/wp20_pilot_prereg.md` returns one hit, `:789`,
inside §7.1's prose; the block at `:885-951` contains no `git rev-parse`, no
`git status`, and no `sha256sum "$P"`. That is a third difference, and the revision
before this one made no claim about the count. (A fourth: SLOT A brackets the transform
around the labels RE-RUN — `labels_rerun exit=0` then `labels-transform seconds=0`,
lines 60-61 — where §8 brackets `labels1`.)

**Why it matters, and this is more than a miscount.** §7.1 spends fifteen lines
establishing that those two lines are what make SLOT A usable:

> **IT RAN AT A COMMITTED REVISION AGAINST COMMITTED BYTES, AND ITS FIRST TWO LINES ARE
> THE RECEIPT FOR THAT** … **REVISION 2's DRY RUN DID NOT** … its artifact's name
> attributed it to a revision that could not have produced it. **That artifact is
> superseded** … the defect is named because it is D-479's class — a measured number is
> bound to the run that produced it.

The pilot's own block, as registered, emits no such receipt. So the run this document
governs will produce an artifact carrying **exactly the defect that disqualified the
predecessor** — a transcript with no binding to the revision or the binary that made
it — and MAJ-A now guarantees the revision in question is not the one §1 names. The
false sentence is what conceals it: a reader checking "does my block do what the dry
run's did" is told the difference is two things and neither is this.

**How I reproduced it.** `sed -n '973,979p;787,796p;885,951p' docs/experiments/wp20_pilot_prereg.md`;
`sed -n '1,2p;53,62p' artifacts/wp20pilot_dryrun_85e6261_v1.txt`;
`/usr/bin/grep -n "rev-parse\|tree 0 modified\|engine sha256" docs/experiments/wp20_pilot_prereg.md`.

**Minimal remedy.** Add the two lines to §8's block —

```bash
echo "revision $(git rev-parse HEAD)  tree $(git status --porcelain | wc -l) modified"
sha256sum "$P" | sed 's/^/engine sha256 /'
```

— which makes the sentence true in the direction that costs nothing and gives the pilot
the receipt §7.1 requires of a dry run. Then say *"differs in the config it names and in
the four-budget sweep"* without the "and no others".

---

# MINOR

## MIN-a — "Both blocks time every pass" is false in both blocks

**File**: `:977`. The second `--labels` call (`:911-912`) carries no bracket, and neither
did the dry run's. §6.3 charges *two* transforms at 1 s each from one reading. Remedy:
either bracket `labels2` too, or say *"both blocks time every pass whose cost §6.2
carries independently"*.

## MIN-b — §6.3's arithmetic block still calls the transform a measurement, which the table three rows above denies

**File**: `:678` and `:683`.

> `two corpus transforms MEASURED at under a second each =    2   s`
>
> **Every term is measured.** … SLOT A times both.

against `:631`'s *"an UPPER bound, **not a positive measurement**"* and §7.1:815's *"the
corpus transform read `0` and is carried as an upper bound of 1 s, which is stated
rather than rounded into the positive column"*. MIN-3's remedy reached four places and
not the arithmetic block that consumes the value. Remedy: *"two corpus transforms,
each BOUNDED above at a second by a timer that read 0"* and *"Every term is measured or
bounded above by a measurement."*

## MIN-c — the new manifest says the dry run is `tools/`-free; §8 runs `tools/cold_label_check.py`

**File**: `docs/experiments/wp20_pilot_artifacts.md:13-15`.

> A successor who has neither can reproduce them: the dry run is `tools/`-free and its
> literal commands are the pre-registration's §8 …

§8:918-919 is `python3 tools/cold_label_check.py --capture … --stride <S3>`, and §1 lists
`tools/cold_label_check.py` as a governed instrument. The charitable reading — *"driven
by no wrapper script in `tools/`"* — is available and is presumably what was meant, but
as written the sentence is false about a file the reproduction recipe depends on.
Remedy: *"the dry run has no wrapper script; its literal commands are §8's block"*.

## MIN-d — §9 and §9.1 read a MAXIMUM off a measurement that is a mean

**Files**: `:1002` and `:1021`.

§9: *"The slot pass confirms it exceeds the dry run's **measured MAXIMUM** label ask with
room"*. §9.1: *"the dry run's **slowest** label ask at `nodes 400000` was about **1.0 s**
(§6.3)"*. §6.3:628 derives 1.006 s as `capture_400000 seconds=165` **over 164** — a mean.
SLOT A records no per-ask timing at all, so no maximum was measured and none can be read
off it; `depth_turns max 5` is a depth, not a duration. The conclusion survives easily
(`hang_timeout_ms = 120000` against a 1 s mean leaves room for a 100× outlier), which is
why this is minor — but the row asserts a measurement the instrument did not take, in the
section whose entire purpose (D-427) is that a value is checked against the prose rather
than assumed. Remedy: *"exceeds the dry run's measured MEAN label ask by five orders of
magnitude; no per-ask maximum was measured and the margin is stated against the mean"*.

## MIN-e — the manifest omits the two C-E injection corpora, the only run outputs it does not index

**File**: `docs/experiments/wp20_pilot_artifacts.md:19-33`.

`/home/tom/pistol-runs/wp20pilot-dryrun2/` holds fifteen files; the manifest indexes
thirteen. Missing: `corpus_grammar.txt` (`3581d0e6…`) and `corpus_digest.txt`
(`38613e6e…`) — the two injections §7.1's last two rows quote refusal messages out of.
The manifest's own claim (*"Every number … is read off one of the files below"*) still
holds, because those messages are quoted from SLOT A, which is indexed. But C-E's two
injections are the runs that prove the loader's guards are distinct, and they are the
only outputs a successor cannot re-verify byte for byte. Two rows.

## MIN-f — "three decided games would put the run under its own floor" is true only under an unstated assumption

**File**: `:565-566` (the sentence spans both lines).

`2 × 13 × 41 = 1 066`, so 66 positions of slack. A decided game contributes
`len(moves)` rather than `len(moves) + 1` (`capture::asked_prefixes` drops the last
prefix only when the game is decided), so a game decided *on the cap* costs one
position and 66 such games would be needed — more than the 26 the pilot plays. Three
games only breach the floor if each is decided by about turn 19. The claim errs the safe
way (it makes the slack look smaller than it is) and changes no registered value, which
is why it is minor; but it is stated as arithmetic and is not. Remedy: *"three games
decided by roughly the halfway point would put the run under its own floor"*.

---

# What I checked and found SOUND

Recorded so a successor does not re-derive it.

- **All 14 digests in the new manifest recompute.** SLOT A =
  `4e716a4f7608485b4ec05cff048c67fa8b4e21deec6c4f61a448b457cefc73b4` ✓; the CI row
  `cb410160…` ✓; every one of the eleven run files ✓. **Both claimed-equal pairs are
  genuinely equal**: `capture_400000.txt` = `capture_400000_b.txt` = `5fe1f1a3…`, and
  `corpus_400000.txt` = `corpus_400000_b.txt` = `099489f0…`. C-B's receipt on the
  stand-in is therefore visible in the committed manifest and not only in the transcript,
  which is what MIN-9 asked for.
- **The depth table reproduces independently, again.** Parsing field 10 out of each of
  164 records in the four corpora with code sharing nothing with `corpus-check` or its
  test: `50000: median 3.0 mean 2.7195 min 1 max 4`; `100000: 3.0 / 3.0366 / 1 / 4`;
  `200000: 3.0 / 3.3049 / 1 / 5`; `400000: 4.0 / 3.6341 / 1 / 5`. The even-count middle
  pair is `(3,3)`, `(3,3)`, `(3,3)`, `(4,4)`, so `median`'s even branch — unchanged this
  round and still `(sorted[n/2-1] + sorted[n/2]) / 2.0` over a sorted slice — could not
  have moved any answer.
- **`summarise` now reports all four of the loader's token sets.**
  `labels_file.rs:258-261` checks `to_move`, `book`, `result`, `end`;
  `corpus-check.rs:83-95` prints all four plus `score_kind`, with a doc comment saying
  why `score_kind` is there and is not one of them. My recomputation of the observed
  spreads matches what the new build would print at every budget: `to_move 2 (p1,p2)`,
  `book 2 (no,yes)`, `result 1 (capped)`, `end 1 (normal)`, `score_kind 3`.
- **The control-character test is a real call-site driver.** `Scratch::write` uses
  `std::fs::write`, so `corpus\nwith-a-newline.txt` is created on tmpfs; `checked` passes
  it through `Command::args`, which spawns without a shell, so the newline survives into
  `std::env::args()`; `printable` (`corpus-check.rs:104-106`) sees a control char and
  `main:116-120` returns `VOID` before `read_to_string`. Remove the guard or its call and
  the file reads fine, prints `ok, N record(s)` and exits `0` — so `assert_eq!(code,
  Some(2))` kills both mutants, and the third assertion (`!stdout.contains("record(s)")`)
  pins that the receipt was never printed. D-553 is discharged at the site, not only at
  the function.
- **MIN-10's exit-code claim is true of the tree.** `arena.rs:224-243`: the `Some(error)`
  branch prints the diagnostic and returns `RUN_FAILED` **without** calling
  `summary::render`; the `None` branch prints the summary and returns `SUCCESS` iff
  `score::tally(...).forfeits == 0`. So "completed" and "zero forfeits" are two
  independent receipts and neither is read from an absence.
- **Both dispatch quotations are exact.** `wp20_dispatches.md:123-124` reads *"the re-run
  determinism receipt on a sub-range"* (line-wrapped after "re-run"); `:331-332` reads
  *"the determinism re-run receipt on a sub-range"*. Quoting them separately is right —
  they are not the same sentence.
- **The 32.5 % / 65 % pair are genuinely different quantities.** `82.5 / 253.5 = 32.54 %`
  is the second capture pass, which is what the whole-range deviation costs;
  `165.0 / 253.5 = 65.09 %` is both capture passes, which is what the label budget costs
  and is what `:583` and the §0.1 row mean. The parenthetical at `:251-253` states the
  distinction correctly.
- **Every row of RULE-1's sensitivity table re-derives.** floor 500 → `T = 7`, 574
  positions, `253.5 × 7 + 2 = 1 776.5 s` = 29.6 min; floor 1 000 → 13, 1 066, 3 297.5 s =
  55.0 min; floor 2 000 → 25, 2 050, 6 339.5 s = 1 h 45.7; ceiling → 56, 4 592,
  14 198 s = 3 h 56.6, and `T = 57` gives 14 451.5 s, over 14 400.
- **§7.2 finding 1's arithmetic is right.** `result` and `end` each hold exactly one
  value across all 164 records at all four budgets, and they are two of the loader's four
  token sets — `to_move` and `book` each hold two. So *"Two of the loader's four token-set
  columns are therefore exercised at one value each"* is exactly true.
- **The five-item enumeration's forward direction holds.** None of the five listed
  refusals can fire mid-walk; I checked each against its site. That half of MAJ-1's
  remedy is correct and is what makes MAJ-D a completeness finding rather than a
  soundness one.
- **The round is scope-compliant.** Six files touched, every one mapping to an enumerated
  remedy; no settled prose re-opened; `docs/decisions.md` untouched, which is right for a
  remedies-only round.

---

# THE STRONGEST ATTACK THE DOCUMENT SURVIVED THIS ROUND

**The attack.** *RULE-1's amendment is convenience wearing method's clothes. The rule
registered before the dry run said "the LARGEST `openings_take` whose derived wall fits
four hours"; the dry run returned `T = 56`, a four-hour session; the session then
rewrote the rule to "the SMALLEST take satisfying three floors" and got `13`, a
55-minute session. The document itself admits the decisive floor is "chosen and not
derived", so the new rule's answer is a judgement made after the numbers arrived — which
is the precise move §6.1 opens by forbidding ("no number below can be chosen after
seeing which number would be convenient"). Confessing the move in capital letters does
not undo it; it launders it. And the confession is cheap, because the amendment happens
to return the answer that ends the session soonest.*

**Why the document survives it — and this is the reviewer's answer, not the
document's.** The attack is right that the rule changed after the numbers and right that
floor (b) is a judgement. It is wrong that the judgement is unchecked, and wrong about
what the amendment buys.

First, the judgement is *tabulated rather than asserted*, and I re-derived every row of
the table independently: 500 → 7 openings / 574 positions / 29.6 min; 1 000 → 13 /
1 066 / 55.0 min; 2 000 → 25 / 2 050 / 1 h 45.7; the ceiling → 56 / 4 592 / 3 h 56.6,
with 57 the only refusal. A reader who thinks the floor should be 500 or 2 000 reads
their own SLOT S1 straight off the document without re-running anything. A judgement
whose whole sensitivity is on the page, with the derivation reproducible in four lines
of arithmetic, is not a hidden choice — it is a stated one, and D-424's test asks whether
a disputed claim changes what anyone may conclude. Here the table means the disagreement
changes the conclusion in a way anyone can compute.

Second, the convenience charge is answered with evidence rather than assertion, and the
evidence is the *other* rule. RULE-2 was not amended, and it fired for `400000` — the
most expensive of its three candidates, 65 % of the pilot's wall — and was left where it
fired. I checked that this was not a rule with one possible answer: the median steps to
4.0 only at `400000` while the mean rises smoothly across all four budgets, so a session
looking for a cheaper answer had an obvious one available (the mean at `100000` already
exceeds 3.0) and did not take it. A session willing to bend a rule for convenience had a
cheaper rule to bend and did not bend it.

Third, the amendment's *direction* is against convenience in the sense that matters
here. `T = 56` would have produced 4 592 labelled positions, and D-539 says the pilot's
games *"count toward no minimum and no score is fitted on them"*. The rule the amendment
deleted maximised a quantity the package declares worthless against a clock, which is a
defect on its face and would have been a defect had the dry run returned four hours or
forty minutes. The amendment removes an objective; it does not choose an answer. And the
floor it replaces it with is defended by something outside the wall arithmetic — a
throughput rate quoted to a corpus plan resting on at least a thousand searches — which
is the quantity §10 says the closure must extrapolate from.

**What this does not rescue.** All of that is about RULE-1, and none of my five MAJOR
findings is. Four of them are the same class the last round found three of and this round
was dispatched to close: a remedy that repaired its own sentence and left, or created,
its contradiction somewhere else — §1 against `corpus-check.rs`, §7.2 against its own
arithmetic, §5 against §5, §8 against SLOT A. The class is not being eliminated; it is
migrating. The document's method for catching it — a change table written from the
diff, a citation checker, a passed-section freeze — cannot see any of the four, because
every one of them is a true citation to a real file whose content says something else.
The instrument that would have caught MAJ-A is the one §1 already names and did not
apply to itself: *a change to any of them reopens this document.*
