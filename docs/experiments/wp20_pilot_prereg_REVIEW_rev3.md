# SCOPED RE-REVIEW — `docs/experiments/wp20_pilot_prereg.md` revision 3

## Header

- **Revision adjudicated**: `ff290ab806fe7cd5ad188c8bfaf03f8887fab07c` (branch `dev`).
- **Matches HEAD**: YES. `git rev-parse HEAD` → `ff290ab806fe7cd5ad188c8bfaf03f8887fab07c`.
- **Tree state**: `git status --porcelain` is EMPTY.
- **Scope**: the 14 findings of `docs/experiments/wp20_pilot_prereg_REVIEW.md`
  (revision 2 = `6e1fea3`, verdict FAIL), the remedy-spends-a-true-thing class
  (D-548, D-549), the two NEW instrument claims, and §6.3's numbers against
  `artifacts/wp20pilot_dryrun_85e6261_v1.txt`.
- **Reviewer**: fresh context. I wrote none of this and owe it no deference. Every
  verdict below is derived from `git diff 6e1fea3..ff290ab` and from the tree, not
  from §0.1's change table (D-550).

**What I read.** `docs/experiments/wp20_pilot_prereg.md` (all 949 lines);
`docs/experiments/wp20_pilot_prereg_REVIEW.md` (all 667);
`git diff 6e1fea3..ff290ab` in full (619 lines of document diff, plus code, ledger and
`docs/decisions.md`); `crates/pistol-arena/src/bin/corpus-check.rs` (all 148 lines);
`crates/pistol-arena/tests/labels_tests.rs` (helpers, `checked`, `corpus_of`, `rows`,
and the six `corpus_check`-driving tests plus the direct-call digest sibling);
`crates/pistol-arena/src/bin/arena.rs` (`main`, `dispatch`, `run`'s two exit branches);
`crates/pistol-arena/src/summary.rs:1-125`; `crates/pistol-arena/src/passes.rs:1-90`;
`crates/pistol-arena/src/capture.rs` (`asked_prefixes`, print inventory);
`crates/pistol-arena/src/labels_file.rs` (column order, the four token sets);
`crates/pistol-arena/src/usage.rs:50-90`; `tools/cold_label_check.py` (void inventory);
`configs/arena_wp20_label_pilot.toml`; `configs/arena_wp20_label_pilot_dryrun.toml` at
`85e6261` and at `f297eab`; `docs/book_v2_ledger.md`; `docs/experiments/wp20_dispatches.md`
(the "sub-range" and "zero forfeits" clauses); `docs/decisions.md` D-552, D-553, D-554;
`CLAUDE.md`; `.gitignore`; `artifacts/wp20pilot_dryrun_85e6261_v1.txt` (all 82 lines).

**What I ran** (read-only; **no `cargo` in any form and no `tools/ci.sh`**):

- `git rev-parse HEAD`, `git status --porcelain`, `git log --format="%H %cI %s"`
- `git show --stat 85e6261`, `git show --stat ff290ab`
- `git diff --stat 85e6261 ff290ab -- configs/ tools/ crates/` → **empty**
- `git diff f297eab 85e6261 -- configs/arena_wp20_label_pilot_dryrun.toml`
- `git show 85e6261:configs/arena_wp20_label_pilot_dryrun.toml`
- `git ls-files artifacts/` → empty; `git grep -n wp20pilot_dryrun`
- `python3 tools/design_citation_check.py docs/experiments/wp20_pilot_prereg.md`
  → `61 citation(s) checked, 0 unreproduced`, exit 0
- a read-only `python3` script over the four surviving corpora in
  `/home/tom/pistol-runs/wp20pilot-dryrun2/`, re-deriving every median, mean, min, max
  and closed-set spread §6.3 tabulates, and the even-count middle pair for each
- `sha256sum target/release/pistol` → `180b4c406b225fc8…`; `stat` on the run files
- `/usr/bin/grep` and `git grep` throughout (D-265)

**Where a build would be needed.** No finding below rests on one. The claim I did not
settle by execution is that the two new tests in `crates/pistol-arena/tests/labels_tests.rs`
pass; the run that would settle it is
`cargo test -p pistol-arena --locked --test labels_tests` in a detached
`git worktree add --detach` on `/home` with its own `CARGO_TARGET_DIR`. I read both
tests instead and report what they pin.

---

## VERDICT: **FAIL**

| severity | count |
|---|---|
| **BLOCKING** | **0** |
| **MAJOR** | **5** |
| **MINOR** | **10** |

The BLOCKING finding is properly dead: the exit-code conflation that would have
recorded an aborted pilot as `V2 — PASS WITH A FINDING` is gone, and the two receipts
that replace it are both true of the tree. Ten of the fourteen findings are cleanly
applied and two of the remedies are better than the remedy asked for. It fails on five
things: two remedies that introduced a new defect while fixing the old one, two
retractions the document announces but does not perform, and a claim that was TRUE in
revision 2 and is now contradicted by the revision that replaced its source.

---

## DISPOSITION OF THE 14 PRIOR FINDINGS

| # | finding | disposition | evidence now standing |
|---|---|---|---|
| **B1** | pass-1 failure registered as V2 | **APPLIED** | §4C:251-282 reads two receipts; §5:367 adds `V7-A`. Confirmed against `arena.rs:234-243` — `summary::render` is called ONLY on the completing branch. (One minor residue, MIN-10.) |
| **M1** | RULE-2 depth table, no artifact, no instrument | **APPLIED — and beyond the remedy** | §1:69 names `corpus-check` as *"the instrument RULE-2's depth table is read from"*; §6.3:544-564 quotes four printed `corpus_check:   depth_turns median …` lines; a shipped-binary test exists. **I recomputed all four medians and means from the corpus files and they match to the last digit.** (Residue MIN-9.) |
| **M2** | void class not total | **APPLIED BUT INTRODUCED A NEW DEFECT** | class re-stated at §5:378-385, `V7-B` added at :368 — but the discriminator registered at :389-391 does not exist. **MAJ-1.** |
| **M3** | RULE-1's false ground, the totality claim | **PARTIALLY APPLIED** | floor (a)'s ground withdrawn (§6.1:465-472) ✓, sensitivity table present and arithmetically correct ✓ — but the retracted sentence still stands verbatim at :412. **MAJ-2**, and §2:112 still carries the superseded rule, **MAJ-3**. |
| **M4** | §7.2's footer exempted finding 1 | **PARTIALLY APPLIED** | footer rewritten at §7.2:733-741 ✓, instrument prints the spread ✓ — but §4E's run 1 (:316-321) is untouched, which was the other half of the remedy. **MIN-1.** |
| **M5** | only the grammar injection drove the binary | **APPLIED** | `a_corpus_whose_body_digest_is_wrong_is_refused_by_the_shipped_loader` (labels_tests.rs:633) calls `checked()` → `env!("CARGO_BIN_EXE_corpus-check")`; the direct-call sibling `…_refused_by_name` still stands at :325. §8:855-869 corrects the sentence. **D-553 satisfied on both limbs.** |
| **M6** | dry run at `f297eab` against an uncommitted edit | **APPLIED** | re-run at `85e6261`, whose committed dry-run config carries `180b4c40…`. Commit `85e6261` at 07:52:24; first run output 07:53:00; `ff290ab` at 08:06:10 — consistent. `git diff --stat 85e6261 ff290ab -- configs/ tools/ crates/` is **empty**, so SLOT R1 governs at HEAD. |
| **M7** | ledger asserted RULE-1 predated the costs | **APPLIED** | `docs/book_v2_ledger.md:43-53` now discloses the amendment and points at §6.1. (Residue MIN-6: the row above still cites *"revision 2"*.) |
| **m1** | replay and transform terms asserted, not measured | **APPLIED** | SLOT A prints `replay seconds=6` (:73) and `labels-transform seconds=0` (:61); §6.3's arithmetic uses both. (Residues MIN-3, MIN-4.) |
| **m2** | §7's extrapolation defence was false | **APPLIED** | §7:657-662 now argues `p = turn_cap + 1`. Verified against `capture::asked_prefixes` — `(0..=last)` less the last when decided. |
| **m3** | C-B's whole-range deviation unnamed | **APPLIED** | §4B:214-220 names it as a deviation from both dispatches. (Residue MIN-5: the quotation and the 65 %.) |
| **m4** | the forfeit relaxation unnamed | **APPLIED** | §4C:275-282 names the DONE clause; verified against `docs/experiments/wp20_dispatches.md:392`. |
| **m5** | C-B read from two digests by eye | **APPLIED** | §8:801,809 add `cmp -s …; echo "…-determinism exit=$?"`; SLOT A shows both at `exit=0`. |
| **m6** | `corpus-check` printed a caller path unguarded | **APPLIED BUT INTRODUCED A NEW DEFECT** | guard added at `corpus-check.rs:95-97,107-111` — with **no test of any kind**. **MAJ-5.** |

---

# MAJOR

## MAJ-1 — M2's remedy registers a discriminator that does not exist: neither pass prints anything before its refusal, and the partial output is deleted

**File**: `docs/experiments/wp20_pilot_prereg.md:389-391` (§5, exclusion (i)); the row
it governs is `:368` (`V7-B`).

**What the document now says**:

> That is the pipeline refusing its own input and it is **V7-B, a STOP**, not a void.
> **The two are told apart by whether any output was produced before the refusal,
> which each pass prints.**

**What is wrong.** No output is produced before either refusal, and the document's own
tree says so twice.

1. `/usr/bin/grep -n "println!\|print!\|eprintln!" crates/pistol-arena/src/capture.rs
   crates/pistol-arena/src/labels.rs` returns **nothing** — neither module prints at all.
   Every print for these two modes lives in `crates/pistol-arena/src/passes.rs:49-58`
   (`capture`) and `:79-84` (`labels`), and in both functions **every `println!` is
   after the walk has completed and the output file has been written**. A failure
   inside `crate::capture::run(...)` or `crate::labels::run(...)` returns `Err` before
   the first `println!` is reached.
2. `crates/pistol-arena/src/bin/arena.rs:76-88` then calls
   `outpath::abandon(&out_path)`, **removing the claimed output file**, and its own
   comment states the consequence:

   > This branch is every pre-game refusal AND a report write that failed partway —
   > **in both cases the file holds no report**.

So a mid-walk `--capture` failure and a pre-work `--capture` refusal are identical in
everything the registered rule looks at: no stdout, no output file, `arena: <error>` on
stderr, exit `2`. The only thing that differs is the **error text** — and that is not
what §5 registers.

**Why it matters.** `V7-B` and `V8` have opposite consequences: STOP versus ONE re-run
of ~55 minutes. M2's whole point was that the class had a reachable ending with no
verdict; the remedy gives it a verdict and then registers an inoperable test for
reaching it. An operator at the run, following §5 literally, finds no output in either
case and has no reading — which is the state M2 found, restored in different words. The
prior review noted that *"A void with no receipt is a STOP"* rescues the outcome by
accident; that clause is still the only thing doing the work, and the remedy did not
change that.

**How I reproduced it.** `/usr/bin/grep -n "println!\|print!\|eprintln!"
crates/pistol-arena/src/capture.rs crates/pistol-arena/src/labels.rs` (empty);
`sed -n '1,90p' crates/pistol-arena/src/passes.rs`; `sed -n '60,90p'
crates/pistol-arena/src/bin/arena.rs`.

**Minimal remedy.** Replace the discriminator with the one that exists: `arena`'s exit-2
refusals are told apart by **the reason `arena: <error>` names** — a document refused on
read (budget kind, seat identity, engine digest, `--out` claimed) is V8; a refusal
naming a channel, an engine's protocol behaviour, a timeout or a record is V7-B. One
sentence, and it is true of the tree.

## MAJ-2 — the retracted totality claim is still standing, so §6.1 asserts it and denies it, and the retraction's own account of the document is false

**File**: `docs/experiments/wp20_pilot_prereg.md:412-413` against `:473-475`.

**What is wrong.** Line 412 still reads, unchanged from revision 2:

> **Both rules are total: applied to §7's measurements they leave no free choice.**

Sixty-one lines later the document says:

> **(ii) THE RULE IS NOT TOTAL, AND THE EARLIER CLAIM THAT IT WAS IS RETRACTED.**
> This section **previously said** *"Both rules are total: applied to §7's
> measurements they leave no free choice."* **That is true of RULE-2 and RULE-3 and
> FALSE of RULE-1** …

The section does not *previously* say it. It **currently** says it, in the paragraph
that introduces both rules, where a reader meets it first.

**Why it matters.** Two ways, and the second is worse than the first. (a) §6.1 now
carries a proposition and its negation, which is the defect class D-554 itself names
one paragraph earlier — *"a document answering one question twice and differently"* —
and which `CLAUDE.md` D-423 forbids by name. (b) The retraction's own factual claim
about the document is false, so a reader who takes the retraction at its word believes
a deletion happened that did not. That is precisely the failure mode this arc has been
bitten by twice (D-550): a change described from a fix list rather than from the diff.
`git diff 6e1fea3..ff290ab` shows the sentence in no deletion hunk.

**How I reproduced it.**
`/usr/bin/grep -n "Both rules are total\|previously said" docs/experiments/wp20_pilot_prereg.md`
→ lines 412 and 473. `git diff 6e1fea3..ff290ab -- docs/experiments/wp20_pilot_prereg.md
| /usr/bin/grep '^-.*Both rules are total'` → no hit.

**Minimal remedy.** Delete the clause at :412 or narrow it to *"RULE-2 and RULE-3 are
total; RULE-1 is not, and (ii) below says where its free parameter is"*; then change
*"previously said"* to a form that is true of the amended text.

## MAJ-3 — §2, which owns the slice, still states the pre-amendment RULE-1, and what it states is false of SLOT S1 = 13

**File**: `docs/experiments/wp20_pilot_prereg.md:108-113` (§2, *"THE SIZE IS DERIVED,
NOT CHOSEN"*).

**What the document says**:

> §6 fixes a wall budget for the whole pilot and §7's dry run measures the per-unit
> cost of each pass on this machine at these budgets; **SLOT S1 is the largest take
> whose derived wall fits that budget**, with the arithmetic shown in §6.

**What is wrong.** That is the rule §6.1 was amended to abolish. §6.1:440-441 records it
as *"What it said before the dry run"*, and §6.3:604-606 records what it returns:
**56**, not 13. So §2 tells a reader that SLOT S1 is the largest take fitting four hours
— which is `T = 56` — while the config, the ledger and §6.3 all say `13`. The sentence
is false of the value it describes.

**Why it matters.** §2 is the section that owns the slice and the ledger row; it is the
first place a successor looks for how `openings_take = 13` was arrived at, and it is
the section the fix round edited (the `openings_take = 13` fill at :107 is in the
diff). The amendment M3 went to trouble to disclose in §6.1 and M7 went to trouble to
carry into the ledger does not reach the section between them. This is the same drift
M7 was raised against, one document upstream.

**How I reproduced it.** `/usr/bin/grep -n "largest take whose derived wall"
docs/experiments/wp20_pilot_prereg.md` → 112, against `:440-441` and `:604-606`, and
against `configs/arena_wp20_label_pilot.toml:26` (`openings_take = 13`).

**Minimal remedy.** Replace the clause with the amended rule — *"SLOT S1 is the
SMALLEST take satisfying §6.1's three floors"* — and point at §6.1 for the amendment.

## MAJ-4 — the remedy that re-measured `c` at the chosen budget spent a claim that was TRUE in revision 2, and left the spent claim standing as a finding of a run that did not produce it

**File**: `docs/experiments/wp20_pilot_prereg.md:747` (§7.2, finding 3) against
`:573-582` (§6.3).

**What is wrong.** §7.2 still reads:

> 3. **THE COLDNESS COST IS 2.4 %.** Recorded in §6.3 and repeated nowhere else.

§6.3 no longer records that. It records the opposite reading:

> **`c` IS MEASURED AT THE CHOSEN BUDGET AND IS NO LONGER DERIVED.** … the two costs
> are **1.006 s each** … **The coldness overhead is therefore below this instrument's
> one-second resolution** … (The earlier `200000` reading put it at 2.4 %; **both are
> in SLOT A's predecessor** and neither changes a value here.)

Three separate problems fall out of that:

1. **§7.2's heading is "THREE MEASURED FINDINGS THE DRY RUN PRODUCED".** SLOT A
   (`artifacts/wp20pilot_dryrun_85e6261_v1.txt`) produced two of them. It contains no
   cold reading at `200000` at all — its only cold line is `cold seconds=165` at
   `400000` (:68). Finding 3's sole source is
   `artifacts/wp20pilot_dryrun_f297eab_v1.txt`, and §6.3 says so.
2. **That source is the artifact §7.1 has just disqualified.** §7.1:702-708 declares of
   it: *"it in fact ran against an uncommitted working-tree edit and its artifact's
   name attributed it to a revision that could not have produced it. That artifact is
   superseded."* A document may not disqualify an artifact in §7.1 and quote a
   measurement out of it as a finding in §7.2 without saying which it means.
3. **The two readings disagree, and the document does not reconcile them.** At
   `200000` the predecessor gave `capture 83 s` / `cold 85 s` over 164 asks — a
   +0.012 s per-ask overhead. At `400000` SLOT A gives `capture_400000 seconds=165`
   and `cold seconds=165` over the same 164 asks — a **zero-second** difference. A
   fixed 0.012 s memset overhead would have shown as ~2 s at `400000` too, which
   `$SECONDS` resolves easily; it did not appear. Revision 2's own physics argument
   (*"a memset's cost does not scale with a node budget"*) predicts the 2 s and the
   new run refutes it. The document asserts both numbers and calls neither into
   question.

**Why it matters.** This is exactly the class D-548 and D-549 record and the class my
scope was told to hunt: a remedy repaired one finding (`c` extrapolated rather than
measured) and destroyed a claim that was correct in revision 2 — the prior review
verified finding 3 explicitly, *"THE COLDNESS COST IS 2.4 % — VERIFIED"* — without
withdrawing it where it stands. §7.2's findings are stated to *"bound what the closure
may conclude"*, and the coldness cost is the number `docs/experiments/wp20m_design.md`
§12 declines to guess and D-542 flags. A closure quoting §7.2 gets 2.4 %; a closure
quoting §6.3 gets "below one second". Neither the citation checker nor a passed-section
freeze can see this.

**How I reproduced it.** `cat -n artifacts/wp20pilot_dryrun_85e6261_v1.txt` (lines 46-47
and 63-68, one cold run, at `400000`) against
`/usr/bin/grep -n "COLDNESS COST" docs/experiments/wp20_pilot_prereg.md` → 747, and
§6.3:573-582.

**Minimal remedy.** Finding 3 becomes: *"THE COLDNESS OVERHEAD IS BELOW THIS
INSTRUMENT'S RESOLUTION AT THE CHOSEN BUDGET (§6.3). The superseded dry run read
2.4 % at `200000`; the two do not agree and the disagreement is recorded rather than
averaged."* — or delete finding 3 and let §6.3 own the number outright, which the
"repeated nowhere else" clause was already reaching for.

## MAJ-5 — the guard m6 asked for landed with no test of any kind, in the round whose own §8 invokes D-553 by name

**File**: `crates/pistol-arena/src/bin/corpus-check.rs:89-97` (`printable`) and
`:107-111` (its call site).

**What is wrong.** The new guard has **no unit test and no call-site driver**:

```
$ /usr/bin/grep -rn "control character\|printable\|is_control" \
      crates/pistol-arena/tests/ crates/pistol-arena/src/
crates/pistol-arena/src/bin/corpus-check.rs:107:        if !printable(path) {
crates/pistol-arena/src/bin/corpus-check.rs:108:            eprintln!("corpus_check: RUN VOID: a named path carries a control character");
crates/pistol-arena/src/bin/corpus-check.rs:91: /// A newline or a control character …
crates/pistol-arena/src/bin/corpus-check.rs:95: fn printable(path: &std::path::Path) -> bool {
crates/pistol-arena/src/bin/corpus-check.rs:96:     path.display().to_string().chars().all(|c| !c.is_control())
```

Nothing in `crates/pistol-arena/tests/` mentions it. Both the guard-removed mutant and
the call-removed mutant survive the whole suite.

**Why it matters.** `docs/decisions.md` D-553 is **standing law** and is unambiguous:
*"for every guard or invariant, the mutation set includes a call-REMOVED mutant, and it
must die at a test that drives the call site with reachable input."* Its corollary makes
*"a review finding a mutant would have caught"* recorded author debt. This guard is a
guard by its own doc comment, it landed at `85e6261`, and §8:855-869 of the same commit
argues at length that the digest test's D-553 gap *"is closed rather than the sentence
softened"* — while a second guard in the same file went in with nothing. The prior
review's m6 is the reason it exists at all.

I record what it does **not** cost: nothing in §8's block reaches it, because every path
there is author-fixed, so no pilot number is at risk. It is a landed-code coverage
breach, not a wrong answer.

**Minimal remedy.** Four lines in `labels_tests.rs`, driving the shipped program the way
its five siblings do — `checked(&[Path::new("corpus\nok.txt")])` asserting `Some(2)` and
that stderr carries `control character`. The guard is reachable from `main` with a
command-line argument, so the call site is drivable directly.

---

# MINOR

## MIN-1 — M4's remedy landed in §7.2 and in the instrument, but not in §4E, which is the criterion

**File**: `docs/experiments/wp20_pilot_prereg.md:316-321` (§4E run 1); the false
cross-reference is at `:739`.

The prior review's remedy had two halves: *"§4E's run 1 additionally records the
observed value-counts of the four token columns, and states that C-E's reach over
`result` and `end` is whatever that run exercised. Delete finding 1 from the footer's
'not for this pilot' list."* The second half is done. §4E's run 1 is untouched by the
diff — it still registers only `exit 0` and the `ok, <n> record(s), capture_sha256 <hex>`
line, so **nothing in the registered criterion obliges the run to record the spread**,
and C-E's registered "WHAT IT MUST SHOW" still does not state its own reach. The
instrument prints the line unconditionally, which makes this cheap to fix and easy to
miss.

Separately, §7.2:739 says *"§7.2's footer and §10 both say so"* of C-E's narrowness.
§10(ii) says the corpus may carry no outcome signal; it says nothing about C-E's guard
coverage. Remedy: add one clause to §4E run 1 registering the summary line and C-E's
reach; and either extend §10 or drop the §10 half of :739.

## MIN-2 — C-D names an instrument §8 explicitly rules out

**File**: `:287` against `:769-773`.

§4D: *"**THE INSTRUMENT.** The `date +%s.%N` pair bracketing each pass in §8's command
block."* §8: *"**TIMING IS `SECONDS`, bash's own integer counter.** Not `EPOCHREALTIME`
… "*, and the block contains no `date` at all. The document names two different
instruments for one measurement, and the one C-D names is not in the registered block.
(Pre-existing from revision 2 and not previously found.) Remedy: §4D says `SECONDS`.

## MIN-3 — "the four measured costs" is now six, and one of them is not positive

**Files**: `:403`, `:534`, `:669`, `:724`.

§6:403 *"The dry run measures four per-unit costs"*; §6.3:534 *"**THE FOUR MEASURED
COSTS**"* — followed by a six-row table. §7:669 registers the dry run's criterion as
*"the four per-unit costs of §6 come back as finite **positive** numbers"*, and §7.1:724
claims *"§6's per-unit costs came back finite and positive … **including the two
revision 2 asserted rather than timed**"*. SLOT A:61 reads `labels-transform seconds=0`.
Zero is not positive, and only one of the two transforms was timed. The wall arithmetic
is unaffected — charging 1 s each is a sound upper bound — but the sentence claiming the
criterion was met over six terms is not true of the artifact. Remedy: make the counts
six, and state the transform term as an upper bound rather than a positive measurement.

## MIN-4 — §8's "differs only in" is false: the dry run's block carries timing brackets §8 does not

**File**: `:869-872`.

> the dry run's instance of the same block is SLOT A (§7.1), which **differs only in
> the config it names and in carrying the four-budget RULE-2 sweep** the pilot does not
> repeat.

SLOT A also carries `replay seconds=6` (:73) and `labels-transform seconds=0` (:61).
§8's registered block has **no** `t=$SECONDS` around `--replay` or either `--labels`
call — the brackets m1 asked for went into the dry run and not into §8. Consequence: the
pilot as registered will not measure its own replay or transform wall, so the next
document inherits the dry run's numbers rather than the pilot's. Remedy: add the two
brackets to §8 (which also makes the sentence true), or correct the sentence.

## MIN-5 — §4B's deviation paragraph mis-sizes its own cost by 2×, and misquotes both dispatches

**File**: `:214-220`.

(a) *"costs a second full capture pass — **65 % of the pilot's wall** (§6.3)"*. §6.3's
arithmetic gives `165.0 T` for **both** capture passes out of `253.5 T` = 65 %; the
*second* pass alone is `82.5 T` = **32.5 %**. The deviation is overstated twofold. (The
same 65 % at :496 is correct, because there it is the label budget's share.)

(b) The quotation *"the re-run determinism receipt on a sub-range"* is verbatim in
neither dispatch: `docs/experiments/wp20_dispatches.md:124` reads *"determinism receipt
on a sub-range"* and `:331-332` reads *"the determinism re-run receipt on a
sub-range"*. The substance is right; the quotation marks are not earned, and this is
the one place the document quotes a governing text it is deviating from.
`tools/design_citation_check.py` does not catch it (61 citations, 0 unreproduced).

## MIN-6 — the ledger row still cites revision 2

**File**: `docs/book_v2_ledger.md:41`.

`| 0 | 13 | 0..12 | … | docs/experiments/wp20_pilot_prereg.md **revision 2** |`, in the
table immediately above the gloss M7's remedy rewrote. The durable record names a
superseded revision of the document that governs it.

## MIN-7 — floor (b) is stated in a quantity `p = 41` only bounds from above, with 6 % of slack

**File**: `:490` (floor (b)), `:606` (*"1 066 asked positions"*), against `:614-621`.

RULE-1's floor (b) is *"at least 1 000 asked positions"* and `T = 13` delivers
`2 x 13 x 41 = 1 066`. §6.3:614-621 then records that **`p = 41` is an UPPER bound** —
a decided game contributes fewer — and that *"whether the pilot's own games are also all
capped is not known from the dry run"*. Three decided games out of 26 put the run below
its own floor. The document registers no reading for that: floor (b) is not a criterion,
so §5 records nothing, and §6.1's stated ground (*"a rate over a thousand searches"*)
would simply be false of the run. Remedy: one sentence in §6.1 — either the floor is
read against the pilot's actual asked-position count with a stated consequence, or the
document records that it is applied to the upper bound deliberately and why that is
enough.

## MIN-8 — the summary line omits `book`, one of the four token sets its own usage block promises

**Files**: `crates/pistol-arena/src/bin/corpus-check.rs:21-25` and `:59-87`, against
`crates/pistol-arena/src/labels_file.rs:258-261`.

The usage block promises *"how many values **each** of the closed-set columns actually
takes"*. `summarise` reports `score_kind`, `to_move`, `result` and `end` — but `book`
(`yes`/`no`, `labels_file.rs:259`) is one of the loader's four token sets and is not
reported, while `score_kind` (not a token set) is. Revision 2's §7.2 carried the true
measured counts *"`book` 20 yes / 144 no"*; the fix round deleted them and the
instrument that replaced them does not print `book` at all, so that column's coverage is
now on no run's face. Small, and a one-line fix in `summarise`.

## MIN-9 — SLOT A is gitignored and sha-indexed by no committed manifest

**File**: `:696`, against `.gitignore` (`/artifacts/`) and `git ls-files artifacts/`
(empty).

The core of M1 is fixed — there is a named instrument and a cited artifact, and the
number is now reproducible by re-running `corpus-check` over the corpora. But the "better
still" half of the remedy was not taken: `artifacts/wp20pilot_dryrun_85e6261_v1.txt` is
untracked, and `git grep wp20pilot_dryrun` finds its digest recorded nowhere committed.
The four corpora it summarises live in `/home/tom/pistol-runs/wp20pilot-dryrun2/`, also
untracked. The number that fixes 65 % of the pilot's wall is one `rm -rf` from having no
evidence. Remedy: a `sha256sum` receipt row for SLOT A under CLAUDE.md rule 8.

## MIN-10 — "zero forfeits" has no positive receipt in the block B1's remedy sends the reader to

**File**: `:268-270` (§4C receipt 2).

> **How many forfeits?** Read from that same printed summary block, **which reports the
> tally**, and never inferred from the exit status.

`crates/pistol-arena/src/summary.rs:60-68` prints the forfeit count **only when it is
nonzero** (` — CONDITIONAL: <n> forfeited game(s) excluded`), and `:116-121`'s
`*** <n> game(s) were forfeited` fires only on `Verdict::InvalidForfeit`. At zero the
block says nothing about forfeits at all, so "zero forfeits" is read from an absence —
which §8:775-778 forbids by name for exactly this reason. The criterion is still
operable (pass 1 exiting `0` is `forfeits == 0` by `arena.rs:236-242`), so this is
wording, not a hole. Remedy: say that a **nonzero** count is read off the CONDITIONAL
clause and that exit `0` on a completed pass is the receipt for zero.

---

# What I checked and found SOUND

Recorded so a successor does not re-derive it.

- **The depth table is correct and independently reproducible.** I recomputed every
  figure from `/home/tom/pistol-runs/wp20pilot-dryrun2/corpus_{50000,100000,200000,400000}.txt`,
  parsing `depth_turns` out of field 10:
  `50000: n=164 median=3.0 mean=2.7195 min=1 max=4`;
  `100000: median=3.0 mean=3.0366 min=1 max=4`;
  `200000: median=3.0 mean=3.3049 min=1 max=5`;
  `400000: median=4.0 mean=3.6341 min=1 max=5`. Every value matches SLOT A's printed
  lines and §6.3's table exactly, as do all four closed-set spreads.
- **`median` is correct, including the even case.** `corpus-check.rs:37-47` averages
  `sorted[n/2-1]` and `sorted[n/2]` on an even count, over a slice sorted at `:61`. The
  registered numbers ran through the even branch (`n = 164`), and I confirmed the middle
  pair is equal at all four budgets, so the branch could not have changed an answer here.
- **The summary test's referent is the file, not the program.**
  `a_corpus_summarises_the_columns_a_pre_registration_reads_a_number_off`
  (labels_tests.rs:589) parses the corpus with `rows(&corpus)`, reads column 10, sorts,
  and computes the median itself before asserting `stdout.contains("depth_turns median
  {expected:.1} ")`. Column indices verified against `labels_file.rs:279-291` (f[10]
  depth, f[14] result, f[15] end). One honest caveat: the test re-implements the same
  branch structure, so it pins agreement rather than the formula — it would not
  discriminate a wrong even-branch on a fixture whose middle pair is equal. The mean is
  quoted in §6.3 and asserted by no test; I verified it by recomputation instead.
- **M5's D-553 obligation is genuinely discharged.** The new test drives
  `env!("CARGO_BIN_EXE_corpus-check")` through `checked()` (labels_tests.rs:535-547) and
  the direct-call sibling still stands at `:325`, so both the function and the call are
  pinned — which is exactly what D-553 asks and what its own text says a direct test
  alone does not give.
- **The artifact's provenance claim holds.** `85e6261` was committed 07:52:24; the run's
  first output (`report_v1.txt`) is 07:53:00 and its last (`corpus_digest.txt`) 08:03:52;
  `ff290ab` is 08:06:10. The committed dry-run config at `85e6261` carries
  `binary_sha256 = "180b4c40…"`, matching the live `sha256sum target/release/pistol` and
  the artifact's line 2. `git diff --stat 85e6261 ff290ab -- configs/ tools/ crates/` is
  empty, so SLOT R1 = `85e6261` governs every instrument at HEAD and MAJOR 6 is fully dead.
- **§6.3's arithmetic reproduces.** `6/4 = 1.5`; `164/4 = 41`; `165/164 = 1.006` for both
  `l` and `c`; `3.0 + 165.0 + 82.5 + 3.0 = 253.5`; `2 x 13 x 41 = 1066 >= 1000` with
  `T = 12.2` rounding to 13; `253.5 x 13 + 2 = 3 297.5 s = 54.96 min`; the pre-amendment
  `253.5 T + 2 <= 14 400` gives `T <= 56.79`, i.e. 56.
- **Every row of the sensitivity table re-derives.** floor 500 → `T = 7`, 574 positions,
  1 776.5 s = 29.6 min; floor 1000 → 13, 1 066, 3 297.5 s = 55.0 min; floor 2000 → 25,
  2 050, 6 339.5 s = 1 h 45.7; ceiling → 56, 4 592, 14 198 s = 3 h 56.6, and 57 gives
  14 451.5 s, over.
- **B1's first receipt is true of the tree.** `arena.rs:234-243` calls `summary::render`
  only on the `None` branch, so a `VERDICT` line on stdout is a sound proof of completion,
  and no path prints one after a death.
- **`p = turn_cap + 1` is true.** `capture::asked_prefixes` returns `(0..=last)` with the
  last dropped only when the game is decided, so a capped game contributes exactly
  `len(moves) + 1 = 41` at `turn_cap = 40`, under either book — m2's remedy is the right
  argument.
- **The void class's third example is true.** `tools/cold_label_check.py:102-121`
  (`body_of`) raises `Void` — exit 2 — for a capture whose body does not digest to its
  header, so §5's bullet is accurate.
- **m4's quotation is exact.** `docs/experiments/wp20_dispatches.md:392` reads
  *"replay_check green, zero forfeits, determinism receipt"*.
- **§9.1's slot table agrees with the config at HEAD** — `openings_take = 13`,
  `openings_skip = 0`, `turn_cap = 40`, `n_workers = 4`, `hang_timeout_ms = 120000`,
  `nodes 50000`, both seats `180b4c40…`, labels `a`/`b`.
- `python3 tools/design_citation_check.py` → `61 citation(s) checked, 0 unreproduced`,
  exit 0. As the instrument itself prints, that means the citations are real, not that
  the claims are right — MAJ-2, MAJ-3, MAJ-4 and MIN-5 are all claims whose cited paths
  exist and whose content says something else.

---

# THE STRONGEST ATTACK THE DOCUMENT SURVIVED IN THIS ROUND

**The attack.** *RULE-2's new instrument is a number the document had already decided,
dressed as a measurement. The depth table was retyped prose in revision 2; revision 3
makes `corpus-check` print the same four numbers, cites the print, and calls the
citation defect fixed. But the instrument was written by the same session, in the same
commit, after the numbers were known — and its test compares the program's median
against a median computed by the test using the same branch structure, on a fixture
whose middle pair is equal, so the test cannot discriminate a wrong formula. That is a
mirror, not a referent: M1 is discharged by an instrument that agrees with the author by
construction, and the number that fixes 65 % of the pilot's wall still rests on nobody
having checked it.*

**Why the document survives it — and this is the reviewer's answer, not the
document's.** The attack is right about the test's discriminating power and wrong about
what it would take to make the numbers false. I did not accept the printed line and I
did not accept the test. I went to the four corpora the run wrote, parsed field 10 out
of each of 164 records with an implementation that shares no code with either the
program or its test, and recomputed the median, mean, min, max and every closed-set
spread. **All twenty-odd figures match to the last digit printed.** The one place a
mirror could have hidden — the even-count branch, which is the branch `n = 164` takes —
I checked directly: the two middle values are equal at all four budgets (`3,3`, `3,3`,
`3,3`, `4,4`), so no averaging rule could have moved the answer, and the median steps to
`4.0` at `400000` for the reason RULE-2 names and not for a formatting reason.

And the attack's real target — that the instrument was authored after the numbers —
misses what the instrument bought. Revision 2's defect was not that `3.0/3.0/3.0/4.0`
were wrong; the prior review verified they were right and said so. It was that they
lived nowhere but in a human's retyping, from four files outside the repository. What
revision 3 changed is that the number is now emitted **by the shipped loader, from the
corpus's own column, on the artifact's own face**, so that any successor with the corpus
can re-derive it in one command — which is what I just did, without trusting a line of
this document. That is the difference between a claim and an instrument, and D-483's
requirement is met in substance and not only in citation. The remedy went past what the
review asked for, and it holds under an independent recomputation.

**What this does not rescue.** Everything above is about the numbers being TRUE. Four of
the five MAJOR findings are not about numbers at all: they are about the document
asserting, in one place, what it retracts in another — §6.1 against §6.1, §2 against
§6.1, §7.2 against §6.3 — and about a discriminator §5 registers that the tree does not
provide. A pre-registration whose sections disagree does not fail because a value is
wrong. It fails because a reader at the run cannot tell which sentence governs.
