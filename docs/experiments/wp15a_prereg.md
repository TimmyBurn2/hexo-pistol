# WP-1.5a — PRE-REGISTRATION (revision 10)

**Revision 10.** Revision 9 went to a governing-revision review and a red-team, in parallel, both
fresh contexts, both against `2673da6`. **The review returned FAILS.** The two of them, with no
shared state, **found the same BLOCKING defect** — which is the strongest evidence this document
has produced about its own instrument, and it is why the finding is described here before the
fixes are.

**B-1, found twice, independently: a genuine ABORT was voided by this document's own replication
rule.** §6 registers that the block is replicated and its stdout must be byte-identical. Line 235
of revision 9's block was `diff -u "$WORK/inv.base" "$WORK/inv.cand"`, and `diff -u` writes both
file PATHS and their mtimes to **stdout** — where `$WORK` is a `mktemp -d` name. On the CONFIRMED
path stdout is empty and stable, which is why every replication ever taken passed. On the ABORT
path three processes produce **three distinct stdouts with identical exit codes and identical diff
bodies**, so §6 voids the run while §3.2 calls it ABORT, and §6's abort protocol attaches only to
ABORT. That is revision 9's own headline defect with a different mechanism: a verdict rewritten by
a temporary directory name instead of by a trap. **And the same path printed no named refusal at
all** — a bare `set -e` on `diff`, where H1-a's failure branch prints one (rule 3;
`SHELL_CHECKLIST` item 1's *"name the refusal"*). Fixed with `--label` and an explicit `cmp`;
**measured after the fix: three replications, ONE distinct stdout, exit 1 each, and
`prereg: H1-b FAILED — a behaviour line moved between …` on stderr.**

**B-2: the second-instrument agreement criterion was violated at the registered bindings, and its
own registered consequence forbade taking a verdict.** The criterion read *"H1-b reports the
invariant blocks identical **if and only if** instrument 2 refuses"*. `tools/bench_delta.sh:274`
refuses only when the two sides resolve to the SAME binary; `cdbcbf0` gives `ff018398…` and
`8618012` gives `a7f519fa…`, so it does not refuse — while H1-b confirms. TRUE ⇔ FALSE. The
paragraph defended itself with *"H1-b is `N/A`"*, which stopped being reachable at revision 6 when
H1-b became unconditional; the same stale sentence was still in §3.

**The deeper half of B-2 is self-inflicted and is why the fix is not a rewording.** Under **D-269
— the clause this document's own §7.1 produced** — a registered criterion must be one the named
defect class could falsify. H1-b is a criterion H1's target defect **preserves**: a solver crate
that is linked but never called moves no `nodes`, no `depth_turns`, no `score`, no `bestmove` and
no ladder line. The agreement duty was anchored to the one criterion in the document that cannot
falsify what it watches.

**THE FIX, and it closes a MAJOR at the same time: `cargo tree` becomes H1-a's second instrument,
and `p = 0` is adjudicated on the RESOLVED GRAPH rather than on manifest text.** The red-team broke
the substring count in both directions and reproduced both: a `# … pistol-solver …` **comment** in
an unrelated manifest produced `2 Cargo.toml files … 2 or more means an edge` — a RUN VOID naming
an edge that does not exist — and a real `[dependencies."pistol-solve\u0072"]` edge that cargo
resolves was **invisible** to the count. By §7.1's own rule the count shares its input, manifest
TEXT, with the defect it must exclude, while the externally derived referent — cargo's resolved
graph, which §2 named and the block did not call — was the one left out.
`cargo tree -p pistol-cli --edges normal -i pistol-solver` answers by **exit status**: `101`,
`package ID specification did not match any packages`, when the solver is absent from the shipped
binary's graph, `0` with a reverse-dependency tree when it is present. It parses nothing, and it
does not read the input H1-a reads.

> **THE AGREEMENT CRITERION, RESTATED:** H1-a reports the two binaries bit-identical **if and only
> if** the resolved normal-edge graph of `pistol-cli` at `LANDING` contains no `pistol-solver`.
> Both instruments evaluate the same proposition — *does the solver reach the shipped binary* —
> and neither reads the other's input: one compiles bytes, the other resolves a graph. The
> registered consequence of disagreement is unchanged and is in §6.

**`tools/bench_delta.sh` is demoted to a RECORDED corroboration with no adjudicating role**, and
6 m 18 s leaves the critical path with it. Its refusal condition is digest-EQUALITY, which is not
the proposition; it compares a different pair; and §6 already had to register that its twice-printed
`VERDICT ABORT` belongs to D-215 and means something else. Two ABORTs meaning contradictory things
in one transcript is a hazard this document has spent rounds removing, and the cheapest removal is
to stop asking it a question it does not answer.

**Eight more repairs, each from a reproduced finding:**

- **The instrument's EXECUTED bytes are hashed** (`git hash-object tools/baseline_snapshot.sh`
  against a registered blob). §0.3 pinned it by blob and step 6 asserts `tools` did not move — both
  claims about COMMITTED trees, while step 10 executes the WORKTREE file.
  `git update-index --assume-unchanged` leaves both assertions empty while the executed bytes
  differ (`SHELL_CHECKLIST` item 5: the index is what commits, the working tree is not).
- **All eight bindings are PRINTED.** A red-team control run **in a clone** produced stdout
  byte-identical to the transcript of a run in the live tree — so a governed transcript could not
  be attributed to its bindings, and §6's replication could not tell three processes run against
  three repositories from three run against one. Of seven bindings, exactly one — `LANDING` —
  reached the transcript, implicitly, via the attestation line's `<rev>:` prefix.
- **`REPO` is registered as a binding.** It selects the repository the entire run measures and was
  environment-controlled and unregistered.
- **`LANDING` and `BASE_REV` are checked for SPELLING and identity** (item 8). A branch name
  satisfied every git call in revision 9 while §0.1 registers a revision that does not move, and
  the attestation then recorded `dev:` where a revision belongs.
- **The `--locked` guard the block CLAIMED and did not have now exists.** Revision 9's step 7
  comment said the dropped `--locked` was *"replaced by the assertion below that the lock moves in
  exactly that one way"*; below it were an assignment with `|| true` and an `echo`. That is
  revision 7's own M-3 defect — a cited guard absent from the registered block — transposed into
  the block's comments. Two real assertions replace it, and the `|| true` is gone: it rendered a
  failed probe, an absent directory and a genuinely empty delta as the same ` none`.
- **The kind token is checked on BOTH records.** Revision 9 checked only the candidate, so an
  incomplete BASELINE record reached the diff and was adjudicated **ABORT** where §3.2 registers
  **RUN VOID** (D-160).
- **Path bindings are guarded as an ALLOW-LIST** (`*[![:print:]]*`, item 4's direction, item 9's
  subject) now that they are printed into a record.
- **Three surviving item-1 command substitutions are guarded**: the pristine-clone status probe (a
  FAILING probe read as CLEAN), both `rev-parse` calls, and the `rustc -vV` slice.

**Two corrections to revision 9's own prose, both mine:** the trap I added **deleted**
`$WORK/toolchain.cand` while §3.1a claimed it was *"kept as the run's own provenance record"* — the
toolchain lines are now printed into the transcript instead, where replication can see them; and
§7.4 said *"287 files"* where 287 was the LINE count of `git diff --stat`, which prints a summary
line, so it was 286 files. The byte count was right and the conclusion — far inside a 64 KiB pipe
buffer — is unaffected.

**Two fair hits on revision 9's arms:** ARM H was run on a **retyped** two-line excerpt while §0.3
claims everything is extracted by script, and the `cannot diff` row added to §3.2 has **no
reachable producer**, because step 6's ancestor test and step 2's revision check catch both
bad-revision inputs first. ARM H is re-run through the extracted block, and that row is gone —
§3.2 carries rows only for refusals an input can reach, which is M-2's standard.

**WHAT SURVIVED, recorded because it is the larger half.** The red-team found **no input producing
a false CONFIRMED**. Its highest-value attack — the escaped dependency edge — passed the substring
count and was **caught by H1-a at exit 1**. The trap repair held across twelve probes in four modes.
No `refuse` is called inside a command substitution. The cargo JSON parse is sound (one
`"executable"`, twenty-five `"executable":null`, correctly skipped). Both sides of H1-b cannot be
made vacuously empty.

**Nothing about H1 itself changed.** The hypothesis, the instruments, the adjudication readings
and the abort protocol are revision 8's; what moved is the routing around them.

**Scope: the pre-registration only.** DESIGN.md is revision 4 and is untouched.

**Machine:** AMD Ryzen 7 3700X, 16 threads, single-thread instrument mode.

---

## 0. Landing order

The window enumeration moves from `pistol-eval` to `pistol-core` (DESIGN.md §2, D-67's flip).
**That move changes `binary_sha256`** while leaving the rest of the invariant block unchanged
on all 24 positions — verified twice, by this session and independently by a reviewer whose
migration was textually different.

Two commits, in this order; H1 is a claim about the second only:

1. **W3 commit** — the enumeration move plus `pistol-eval`'s re-export. A fresh baseline
   snapshot is taken **at this revision, after it exists**, and becomes H1's baseline record.
2. **WP-1.5a commit** — the threat generator, compared against that snapshot.

**Only the STOCK side may be pinned in advance.** The W3 binary digest is **not reproducible
across independent migrations** — this session's migration produced `daf1deb3…` and a
reviewer's produced `691f7766…`, both search-identical to stock. A digest that depends on
which edits an implementer made cannot be predicted, so §3's `BASE`/`BASE_SHA` are filled in
**by amendment after the W3 commit exists**, which reopens this review (D-195). Nothing in
this document predicts the W3 digest.

**Wording note (m-3).** §0 says the W3 move leaves "the rest of the invariant block"
unchanged, **not** "the invariant block byte-identical". The block **includes**
`binary_sha256` and W3 differs on exactly that line. §3's stricter sense is the operative one
and this section does not relax it. **The non-reproducibility is now confirmed a THIRD time**:
the landed W3 digest is `ff018398…`, distinct from this session's `daf1deb3…` and a reviewer's
`691f7766…`, all search-identical to stock — which is what D-253 records and why only the stock
side is ever pinned in advance.

### 0.1 THE EIGHT BINDINGS (six at revision 7; `REPO` and `SNAP_BLOB_PIN` registered at revision 10)

Every value was verified this session rather than transcribed.

**`REPO` was a binding all along and was not registered (revision 10).** It selects the repository
the entire run measures, it is read from the environment, and revision 9 validated it only by
whether `cd` succeeded. A red-team control run **in a clone**, with `REPO` exported, produced
stdout byte-identical to the transcript of a run in the live tree — so a governed transcript could
not be attributed to its bindings, and §6's replication could not distinguish three processes run
against three repositories from three run against one. It is registered here and **printed by the
run**, along with the other seven.

**`SNAP_BLOB_PIN` is new (revision 10)** and exists because §0.3's blob pin and step 6's `tools`
drift assertion are both claims about COMMITTED trees while step 10 executes the WORKTREE file. It
is the blob the instrument must hash to, checked with `git hash-object` against the bytes that
will actually run.

**The two baseline artifacts moved to a DURABLE location, and their digests did not (revision
10).** Revision 9 copied them from one session-scoped `/tmp` scratchpad to another, which a
reviewer correctly called the same property with a different UUID: `/tmp` does not survive a
reboot and no session owns the next session's directory. They now live in `~/Work/pistol-wp15a/`,
outside the repository (rule 8), which is WP-1.3's own idiom for run artifacts
(`~/Work/pistol-wp13/*.matchlog`). **Re-verified against the digests already registered here** —
`sha256sum -c` reports `OK` for both, the record's `revision` line still reads `cdbcbf05…`, its
`binary_sha256` still reads `ff018398…`, and the sizes are 5764 and 892 bytes. **A path is a
locator and the digest is the binding**, which is the whole reason a digest was pinned rather than
a path; had the copy differed by a byte, step 2 would refuse before H1 asked anything.

```sh
BASE=/home/tom/Work/pistol-wp15a/baseline_cdbcbf0.txt
BASE_SHA=7faa074c21a2d7d28e4ca681e05ed95942436d639642c088a73032febd33159a
BASE_REV=cdbcbf05bd9d792ac7a6af709970f11b95796b81
BASE_TC=/home/tom/Work/pistol-wp15a/baseline_cdbcbf0.toolchain.txt
BASE_TC_SHA=8be240559c27b2b163347cba8f2266f7877ceee0ef8d72095e2d73537a6adc2a
LANDING=861801247df5c1a73480b5153e11c399aa752750
REPO=/home/tom/Projects/HeXO-AlphaBeta
SNAP_BLOB_PIN=a7c0ed4367a6893f70b776732bd68ada77c19483
```

| variable | source, verified |
|---|---|
| `BASE` | the W3 baseline record, 5764 bytes, at its durable path; digest recomputed and matching |
| `BASE_SHA` | recomputed `7faa074c…` |
| `BASE_REV` | the record's own `revision` line; and its `binary_sha256 ff018398…` **reproduces from a pristine checkout of that revision** (§3.1 step 4). Checked for 40-hex SPELLING and for `rev-parse` identity at revision 10 |
| `BASE_TC` | the sidecar, 892 bytes: `rustc 1.97.1`, `cargo 1.97.1`, LLVM 22.1.6, `stable-x86_64-unknown-linux-gnu`. It carries `snapshot_revision` and `snapshot_binary_sha256` matching the record, which is what ties the two artifacts to each other rather than to a claim |
| `BASE_TC_SHA` | recomputed `8be24055…` |
| `LANDING` | `86180124…`, seventeen commits after `BASE_REV`. **No longer required to equal `HEAD`** (§3.1 step 6). Checked for 40-hex SPELLING and `rev-parse` identity at revision 10, because a BRANCH NAME satisfied every git call in revision 9 — measured: `LANDING=dev` passed the ancestor test, the drift diff and the attestation, and recorded `dev:` where a revision belongs |
| `REPO` | the repository root. Printed by the run; guarded as an ALLOW-LIST against non-printable characters, since it now reaches a record (`SHELL_CHECKLIST` items 4 and 9) |
| `SNAP_BLOB_PIN` | `git ls-tree 8618012 tools/baseline_snapshot.sh` → `a7c0ed43…`, and the same blob at `HEAD`. The run hashes the file it is about to execute and refuses on a mismatch |

### 0.2 A SECOND READING OF `p = 0`, AND WHAT IT IS AND IS NOT WORTH

H1-a says: *at one revision, deleting the solver changes no byte of the binary.* The fix round
supplies a second reading of the same claim. **Revision 7's headline called it "stronger than
H1-a", which contradicted this section's own closing sentence; the closing sentence was right
and its stated reason was not.**

Both endpoints of the fix round, rebuilt in a pristine clone:

```
7b9e904 -> a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
8618012 -> a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
```

**Byte-identical** — while `crates/pistol-solver` moved by **232 insertions and 28 deletions
across 6 files** between them (`cdbcbf0..7b9e904` is 4421 insertions; `cdbcbf0..8618012` is
4625). So the shipped binary was invariant not merely under *removing* the solver at one
revision, but under **substantively rewriting it across seven commits**. That is the difference
between one observation and a controlled repetition, and it is why the number is recorded here
rather than left implicit in the transcript.

**THE BINDING REASON IT IS ONLY CORROBORATION, measured rather than argued:**
`grep -c 'Compiling pistol-solver'` over the run's own stderr returns **0**. Cargo never
compiles the crate — in any of the three builds; the five it does compile are `pistol-core`,
`pistol-eval`, `pistol-search`, `pistol-engine` and `pistol-cli`. **Invariance of a binary under
rewriting a crate that is never compiled is not an observation about the rewrite.**

Apply §7.1's own test: what defect could falsify this criterion? Only one that puts the solver
into the build graph — and any such defect already fails H1-a, loudly. **So it is a criterion
the defect class preserves**, which is the vacuity §7.1 legislates against, and it is recorded
here as corroboration precisely so nobody promotes it to evidence. The same-build-path,
same-machine point stands and is the weaker of the two reasons.

**The record's `timing tree` reads `dirty`, and revision 9 states the reason more sharply than
"expected and not disqualifying":** `clean` was **unreachable** while the threat calculus sat
untracked, because `tools/baseline_snapshot.sh:417` sets that token from a bare
`git status --porcelain` — so the token could take one value and reading it as a check was reading
a constant. D-266 tracked the file and the token is a measurement again; the baseline record
predates that commit and still reads `dirty`, which is now a fact about WHEN it was taken rather
than about this tree. Nothing rests on it either way: the token is replaced by a pristine rebuild
(§3.1 step 4), which the baseline passes, and that substitution was made for exactly this reason.

### 0.3 THE INSTRUMENTS, NAMED WITH THEIR REVISIONS (D-268; the pin became EXECUTABLE at revision 10)

D-268 amended CLAUDE.md in the same session as revision 9: *an artefact that produces a registered
number — a `tools/` script, a scratchpad harness, or a command block the document prints — is NAMED
IN THE PRE-REGISTRATION WITH ITS REVISION, and a change to it reopens the review exactly as an
amendment to the document does.* This document was one of the two instances that produced the rule,
so it states its own instruments rather than being the first document to ignore the clause it
caused.

| instrument | what it produces | revision, pinned |
|---|---|---|
| the §3.1 block | H1-a's two digests, the `p = 0` graph verdict, H1-b's line comparison, all verdicts | printed in this document; its revision IS this document's, and §7.2/§7.3/§7.5 extract it BY SCRIPT rather than retyping it |
| `tools/baseline_snapshot.sh` | the candidate invariant block at step 10 | blob `a7c0ed4367a6893f70b776732bd68ada77c19483`, **hashed at run time by step 3** and asserted against the registered `SNAP_BLOB_PIN` |
| `cargo tree` | the `p = 0` verdict and H1-a's agreement comparand (§6) | the pinned toolchain, `cargo 1.97.1`; the graph it reads is the PRISTINE CLONE at `LANDING`, not the working tree |
| `tools/bench_delta.sh` | **nothing this document adjudicates on** (demoted at revision 10, §6) | blob `130b4acbd87f1413776da7b1354f00a90bbc41d6` at `LANDING`, recorded for provenance only |
| the baseline record + sidecar | H1-b's comparand | not a script: pinned by digest, `7faa074c…` and `8be24055…`, §0.1 |

**REVISION 9 SAID "THE PIN IS AN EXECUTABLE ASSERTION AND NOT A TABLE" AND WAS OVERSTATING BY
EXACTLY ONE GAP, WHICH BOTH REVIEWERS FOUND.** Step 6 diffs `tools` between `LANDING` and `HEAD`
and step 5 requires a clean tree — both are claims about COMMITTED trees, and step 10 executes the
**worktree** file of that name (`SHELL_CHECKLIST` item 5: the index is what commits, the working
tree is not). Measured: `git update-index --assume-unchanged tools/baseline_snapshot.sh` plus a
worktree edit leaves `git status --porcelain` **EMPTY** and the `LANDING..HEAD` diff **EMPTY**,
while the file that would run hashes to something else. Revision 10 hashes the bytes that run:

```
### ARM N — exit 2
  git status --porcelain: []        <- the old guards see nothing
prereg: the snapshot instrument on disk hashes to 42463e436f4003cebe70cc18bd487e5e23a157c1,
        not the registered a7c0ed4367a6893f70b776732bd68ada77c19483 — the bytes that will run
        are not the bytes that were reviewed
```

**The `LANDING..HEAD` drift assertion stays** and is not made redundant by the hash: it covers
`Cargo.toml`, `Cargo.lock`, `crates` and `configs` as well, and it is what makes the run takeable
at all with docs commits above `LANDING`. **One gap remains and is recorded rather than closed:**
nothing pins the instrument on the BASELINE side — the record at `BASE_REV` was taken by whatever
`tools/baseline_snapshot.sh` existed then. For this pair the gap is latent and not live:
`git diff --name-only cdbcbf0 8618012 -- tools` is empty, so the same instrument took both sides.
A future pair that straddles a `tools/` change owes a second pin.

## 1. The change under test

`crates/pistol-solver` gains the threat state, its queries, an oracle and the golden fixtures
(DESIGN.md §3, §7, §8). It takes one dependency, `pistol-core`, and **no dev-dependencies**.
**Nothing in the engine calls it.**

## 2. `p = 0` — ADJUDICATED ON THE RESOLVED GRAPH (revision 10), attested on the manifests

**The claim:** no binary in this workspace links `pistol-solver`, so the work package cannot change
one. Revision 10 changes WHAT ANSWERS IT, because a red-team broke the previous answer in both
directions and reproduced both.

### 2.1 Why the substring count stopped being the adjudicator

Revisions 6-9 adjudicated on `git grep -l -e 'pistol-solver' -e 'pistol_solver' "$LANDING" --
'*Cargo.toml'` with `n == 1`. It is wrong in both directions:

- **A false RUN VOID on a good tree.** One line appended to `crates/pistol-engine/Cargo.toml` —
  `# The threat generator (pistol-solver) is deliberately NOT linked here yet.` — produced
  `2 Cargo.toml files mention pistol-solver, expected exactly 1 … 2 or more means an edge`. The run
  is voided and the message asserts an edge that does not exist. A `[dev-dependencies]` entry,
  which §3 relies on NOT moving the digest and calls *"why the oracle's test-tree home is safe"*,
  trips the same wire.
- **A real edge it cannot see.** `[dependencies."pistol-solve\u0072"]` with
  `path = "../pistol-solve\u0072"` is valid TOML, cargo resolves it, and the count still returns
  **1**. The block then proceeded, and H1-a caught it at exit 1 — which is the document's win and
  is recorded as such in §7.5 — but §3.2 registered that input as *"refused at step 6, exit 2,
  before H1-a builds anything"*, and that was false.

**By §7.1's own rule the count was the wrong instrument.** It shares its input — manifest TEXT —
with the defect it must exclude, and manifest text is exactly what an escape or a comment perturbs
without changing the graph. The externally derived referent was named in this section three
revisions ago and never called: *"`cargo tree -p pistol-cli --edges normal` corroborates on the
resolved graph and is recorded, not adjudicated on."* It is now the adjudicator.

### 2.2 The registered test

```sh
cargo tree --locked -p pistol-cli --edges normal -i pistol-solver
```

It answers by **EXIT STATUS** and parses nothing:

| observation | reading |
|---|---|
| exit `101`, `error: package ID specification 'pistol-solver' did not match any packages` | the solver is ABSENT from the shipped binary's normal-edge graph — `p = 0` |
| exit `0`, a reverse-dependency tree | the solver IS reachable — **`p = 0` REFUTED, exit 1 = ABORT** |

**Readability is a separate question with a separate refusal** (`SHELL_CHECKLIST` item 8), because
`101` is also cargo's generic error status: the block first requires
`cargo tree --locked -p pistol-cli --edges normal` to succeed, so *"cargo could not answer"* is a
RUN VOID and *"there is no edge"* is a verdict, and one status cannot mean both.

**A REFUTATION IS ABORT, NOT RUN VOID, AND THAT IS A DELIBERATE CHANGE FROM REVISION 9.** An
accidental dependency edge is not an instrument failure — the instrument answered. It means `p` is
no longer 0, which is §6's abort protocol exactly: the WP does not land, the finding is recorded,
`cargo tree` is the first diagnostic, and this document is **wrong rather than failed**.

### 2.3 The manifest attestation, RECORDED and no longer adjudicated on

```sh
git grep -n -e 'pistol-solver' -e 'pistol_solver' "$LANDING" -- '*Cargo.toml' | LC_ALL=C sort
```

At `LANDING` it returns exactly one line, the crate's own `name =`:

```
861801247df5c1a73480b5153e11c399aa752750:crates/pistol-solver/Cargo.toml:2:name = "pistol-solver"
```

It is printed into the transcript as evidence and nothing turns on its count.

### 2.4 THE ENVIRONMENT FINDING THAT MADE THIS COMMAND HARD TO RECORD — now D-265

**Three revisions of this document printed a "verbatim" grep and three reviews found it wrong.**
It is now **D-265**, and revision 9 corrected the attribution that line was drafted with, because
the wrong attribution implied a mitigation that would have done nothing.

**What is true, measured.** In the shell an agent's tool runs, `type -t grep` reports `function`,
and that function execs `${CLAUDE_CODE_EXECPATH:-}` under `exec -a ugrep` with
`-G --ignore-files --hidden -I --exclude-dir=.git …` prepended. It is multithreaded and its output
ORDER is nondeterministic: **eight runs of `grep -rn 'WINDOW_LEN' crates/` through the function gave
EIGHT distinct sha256 sums; eight runs of `/usr/bin/grep` gave ONE.**

**What revision 8 got wrong.** It said this was *"a shell function wrapping ugrep in a shell
initialised from the user's profile"*. **It is not the profile.** `~/.bashrc`, `~/.bash_profile`
and `~/.profile` carry no `grep` alias or function, `~/.bash_aliases` does not exist, **`ugrep` is
not on `PATH` at all**, and a genuine login-interactive shell disagrees outright:
`bash -lic 'type grep'` reports `grep is /usr/bin/grep`. The wrapper is **injected by the tool
harness** and by nothing else — which moves the mitigation from *"do not use a login shell"*,
which would have bought nothing, to **"a transcript an agent captured is not the output of the
command a script runs"**.

**And the hazard is not only order.** The injected flags change WHICH FILES ARE VISITED:
`grep -rl pistol target` through the function reaches **1474** files where `/usr/bin/grep` reaches
**24455**. An agent-shell COUNT and a script's count of the same command are answers to different
questions — D-221/D-223's class by a second route.

**Why it hid:** a query with a SINGLE HIT is stable through both paths — ten runs of the
attestation above gave one hash — so the defect is invisible on exactly the small outputs a
document pastes.

**§3.1 is a SCRIPT** — `#!/usr/bin/env bash`, run as a file — so every `grep` in it is
`/usr/bin/grep` and the wrapper cannot reach it; §7.2/§7.3/§7.5 record the block being extracted
and run rather than pasted into a shell.

### 2.5 What H1 can and cannot see

**H1 is falsifiable by its target defect**, confirmed by a reviewer's break test: a bare
`[dependencies] pistol-solver` edge with no call site moves the digest `62c102cc…` → `eeefee04…`,
while a dev-dependency correctly does not.

**ONE INSTRUMENT CAVEAT, carried because the document should not overclaim.** `binary_sha256` is
**insensitive to dead code even in a linked crate**: an unreferenced `pub const` appended to
`pistol-core` did not move the digest. H1's *target* defect does move it, so H1 is sound — but the
instrument detects "a change that reaches codegen", not "any change", and a reader must not treat a
CONFIRMED H1 as attesting that no code was added.

## 3. H1 — the whole-engine claim, with each hypothesis's DEFECT CLASS named

**The claim is unchanged: WP-1.5a adds nothing a shipped binary can observe, because `p = 0`.**
What changed in revision 6 is the instrument; what changes in revision 10 is that each hypothesis
**names the defect class its criterion must be able to falsify**, which is D-269 — the clause this
document's own §7.1 produced — applied to this document's GOVERNED criteria and not only to its dry
runs. A reviewer found them missing one revision after the clause landed, and for H1-b the absence
was not a formality: see the note under §6.

- **H1-a (PRIMARY, counterfactual).** At `LANDING`, in a pristine clone, the binary built from the
  tree **is bit-identical** to the binary built from the same tree with `crates/pistol-solver`
  reverted to its `BASE_REV` content.
  **DEFECT CLASS: solver content that reaches codegen by ANY route** — a dependency edge, an
  `include_str!`, a build script, a path that cargo auto-discovers. **Falsifiable by it:** verified
  at exit 1 with an `include_str!` of solver source referenced from a called path in `pistol.rs`,
  digests `570dc5d8…` against `1ed322ea…`; and again by the red-team's escaped
  `[dependencies."pistol-solve\u0072"]` edge. The comparand is **derived by an independent build**,
  not read from a file, which is the externally derived referent §7.1 asks for.
- **H1-b (SECONDARY, UNCONDITIONAL since revision 6).** The invariant blocks of the baseline record
  and of a snapshot taken on the candidate binary must be byte-identical, excluding `revision` and
  `binary_sha256`.
  **DEFECT CLASS: any behaviour change in the shipped engine between `BASE_REV` and `LANDING` —
  which is NOT H1's defect class.** A `BUCKET_ENTRIES` change in `pistol-search` is an instance.
  **AND H1's TARGET DEFECT CANNOT FALSIFY IT**, which is why it is secondary and why it carries no
  agreement duty (§6): a solver crate that is LINKED BUT NEVER CALLED moves `binary_sha256` — which
  is excluded — and moves no `nodes`, `depth_turns`, `seldepth`, `hashfull`, `score`, `bestmove`,
  `pv` or ladder line, which are the 54 that remain. Under D-269 that makes H1-b a criterion the
  defect class PRESERVES, and a criterion like that adjudicates nothing about H1.

**Revision 9 still carried revision 5's conditional wording for H1-b — "If it does touch such a
path, H1-b is `N/A`" — in this section and in §6, three revisions after the `N/A` branch ceased to
exist.** There is no `N/A` path in the block: step 10 runs unconditionally, `binary_sha256` is
excluded exactly as `revision` is, and §7.2 records H1-b confirming where revision 7 declared
`N/A`. The dead sentences are removed. This mattered: §6's agreement criterion was defending itself
with one of them.

**Why H1-a is the stronger instrument, in §7.1's own terms.** H1-b compares against a *stored
artifact* that shares an input with the thing under suspicion — the tree — so a defect in the tree
can survive it. H1-a compares two builds that differ **only** in the suspect content.

### 3.1 The registered block

Every refusal exits **2** through one `refuse()` helper; `0` is CONFIRMED, `1` is ABORT.

```sh
#!/usr/bin/env bash
set -euo pipefail

refuse() { echo "prereg: $*" >&2; exit 2; }

# Object-name SPELLING, not just value (SHELL_CHECKLIST item 8). A branch name
# passes every git call below while `LANDING` is registered as a fixed revision,
# and the attestation then records `dev:` where a revision belongs.
is_hex() { case "$2" in ''|*[!0-9a-f]*) return 1 ;; esac; [ "${#2}" -eq "$1" ]; }
# An ALLOW-LIST, so pinning the locale makes this refuse MORE and never less
# (SHELL_CHECKLIST item 4). A newline in a path INJECTS LINES into the record
# these bindings are now printed into (item 9).
printable() { case "$1" in *[![:print:]]*) return 1 ;; esac; }

# ---- 1. Bindings. SEVEN, all registered in §0.1, all shape-checked, and all
#         PRINTED — a transcript that does not name its inputs cannot be
#         attributed to them, and §6's replication then cannot tell three
#         processes run against three different repositories from three run
#         against one. Measured: a red-team control run in a CLONE produced
#         stdout byte-identical to the transcript of a run in the live tree. ----
BASE=${BASE:-}               ; [ -n "$BASE" ]        || refuse "BASE unset"
BASE_SHA=${BASE_SHA:-}       ; [ -n "$BASE_SHA" ]    || refuse "BASE_SHA unset"
BASE_REV=${BASE_REV:-}       ; [ -n "$BASE_REV" ]    || refuse "BASE_REV unset"
BASE_TC=${BASE_TC:-}         ; [ -n "$BASE_TC" ]     || refuse "BASE_TC unset"
BASE_TC_SHA=${BASE_TC_SHA:-} ; [ -n "$BASE_TC_SHA" ] || refuse "BASE_TC_SHA unset"
LANDING=${LANDING:-}         ; [ -n "$LANDING" ]     || refuse "LANDING unset"
SNAP_BLOB_PIN=${SNAP_BLOB_PIN:-} ; [ -n "$SNAP_BLOB_PIN" ] || refuse "SNAP_BLOB_PIN unset"

is_hex 40 "$BASE_REV" || refuse "BASE_REV is not a 40-hex object name: $BASE_REV"
is_hex 40 "$LANDING"  || refuse "LANDING is not a 40-hex object name: $LANDING — a branch or tag \
name satisfies every git call below while §0.1 registers a revision that does not move"
is_hex 64 "$BASE_SHA"    || refuse "BASE_SHA is not a 64-hex digest: $BASE_SHA"
is_hex 64 "$BASE_TC_SHA" || refuse "BASE_TC_SHA is not a 64-hex digest: $BASE_TC_SHA"
is_hex 40 "$SNAP_BLOB_PIN" || refuse "SNAP_BLOB_PIN is not a 40-hex object name: $SNAP_BLOB_PIN"

# `git rev-parse` exits 128 outside a repository, which is a fourth status the
# adjudication table does not define; route it to 2 like every other refusal.
REPO=${REPO:-}
if [ -z "$REPO" ]; then
  REPO="$(git rev-parse --show-toplevel 2>/dev/null)" || refuse "not inside a git repository"
fi
[ -n "$REPO" ] || refuse "cannot resolve the repository root"
for path in "$REPO" "$BASE" "$BASE_TC"; do
  printable "$path" || refuse "a path binding contains a non-printable character and would inject \
lines into the record it is printed into"
done
# AND ENTER IT. Every `git ls-files -- <pathspec>` below resolves its pathspec
# RELATIVE TO THE CURRENT DIRECTORY, so the same command run from `crates/` looks
# for `crates/crates` and reports "no untracked files on build-reaching paths"
# with a stray file sitting in one. MEASURED: from `crates/` with
# `crates/pistol-core/ZZZ_untracked_probe.rs` present, `git status --porcelain`
# reported it and the registered enumeration returned EMPTY. EXIT-0-WRONG-ANSWER
# selected by working directory (SHELL_CHECKLIST item 5).
cd "$REPO" || refuse "cannot enter the repository root $REPO"

echo "prereg: binding REPO          $REPO"
echo "prereg: binding BASE          $BASE"
echo "prereg: binding BASE_SHA      $BASE_SHA"
echo "prereg: binding BASE_REV      $BASE_REV"
echo "prereg: binding BASE_TC       $BASE_TC"
echo "prereg: binding BASE_TC_SHA   $BASE_TC_SHA"
echo "prereg: binding LANDING       $LANDING"
echo "prereg: binding SNAP_BLOB_PIN $SNAP_BLOB_PIN"

WORK="$(mktemp -d -t wp15a_inv.XXXXXX)"     || refuse "cannot create the work directory"
PRISTINE="$(mktemp -d -t wp15a_pristine.XXXXXX)" || refuse "cannot create the pristine directory"
# 61 MB per run, and ten runs left 605 MB behind before this trap existed.
# SHELL_CHECKLIST item 7, and this one CHANGES VERDICTS: revision 8 wrote
# `cleanup() { rm -rf -- "$WORK" "$PRISTINE"; }`, whose last command decides the
# script's status. Measured on that construction with an unremovable path:
# requested 0 -> got 1, requested 1 -> got 1, requested 2 -> got 1. CONFIRMED and
# RUN VOID both became ABORT, by housekeeping. The form below returns 0, 1 and 2
# for the same three requests, in all four modes a red-team probed (normal,
# unremovable, already-gone, cwd-inside-WORK).
cleanup() { local rc=$?; rm -rf -- "$WORK" "$PRISTINE" 2>/dev/null || true; return "$rc"; }
trap cleanup EXIT

# ---- 2. Baseline record: present, pinned, at the registered revision. ----
[ -s "$BASE" ] || refuse "baseline record missing or empty at $BASE"
# `[ -s ]` STATS and `sha256sum -c` READS: an unreadable record reached the digest
# comparison and was reported as "does not match its registered digest" when it had
# never been read at all (SHELL_CHECKLIST item 8, one refusal per reason).
[ -r "$BASE" ] || refuse "baseline record at $BASE is not readable"
printf '%s  %s\n' "$BASE_SHA" "$BASE" | sha256sum -c - >/dev/null \
  || refuse "baseline record does not match its registered digest"
[ "$(sed -n 's/^revision //p' -- "$BASE")" = "$BASE_REV" ] \
  || refuse "baseline record is not at the registered baseline revision"
[ -s "$BASE_TC" ] || refuse "baseline toolchain sidecar missing at $BASE_TC"
[ -r "$BASE_TC" ] || refuse "baseline toolchain sidecar at $BASE_TC is not readable"
printf '%s  %s\n' "$BASE_TC_SHA" "$BASE_TC" | sha256sum -c - >/dev/null \
  || refuse "baseline toolchain sidecar does not match its registered digest"

# ---- 3. The INSTRUMENT'S EXECUTED BYTES, hashed. §0.3 pins
#         `tools/baseline_snapshot.sh` by blob and step 6 asserts `tools` did not
#         move between LANDING and HEAD — but both are claims about COMMITTED
#         trees, and step 10 executes the WORKTREE file of that name.
#         `git update-index --assume-unchanged` leaves both assertions empty
#         while the executed bytes differ (SHELL_CHECKLIST item 5: the index is
#         what commits, the working tree is not). Hash what runs. ----
SNAP_BLOB="$(git hash-object -- tools/baseline_snapshot.sh)" \
  || refuse "cannot hash the snapshot instrument"
[ "$SNAP_BLOB" = "$SNAP_BLOB_PIN" ] \
  || refuse "the snapshot instrument on disk hashes to $SNAP_BLOB, not the registered \
$SNAP_BLOB_PIN — the bytes that will run are not the bytes that were reviewed"

# ---- 4. The baseline's cleanliness, attested by REBUILD not by its own token. ----
BASE_DIGEST_RECORDED="$(sed -n 's/^binary_sha256 //p' -- "$BASE")"
[ -n "$BASE_DIGEST_RECORDED" ] || refuse "baseline record carries no binary_sha256"
git clone --quiet --no-hardlinks "$REPO" "$PRISTINE/repo" || refuse "cannot clone for the rebuild attestation"
( cd "$PRISTINE/repo" && git checkout --quiet "$BASE_REV" ) || refuse "baseline revision not in the clone"
# Taken into a variable so a FAILING probe cannot read as CLEAN (item 1).
CLONE_DIRT="$(cd "$PRISTINE/repo" && git status --porcelain)" \
  || refuse "cannot read the pristine clone's status"
[ -z "$CLONE_DIRT" ] || refuse "the pristine clone is not pristine: $CLONE_DIRT"
BASE_BIN="$(cd "$PRISTINE/repo" && cargo build --release --locked -p pistol-cli --bin pistol \
    --message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
  || refuse "baseline rebuild failed"
[ -s "$BASE_BIN" ] || refuse "baseline rebuild produced no binary"
BASE_BIN_DIGEST="$(sha256sum -- "$BASE_BIN" | cut -d' ' -f1)" || refuse "cannot digest the baseline rebuild"
[ "$BASE_BIN_DIGEST" = "$BASE_DIGEST_RECORDED" ] \
  || refuse "baseline binary_sha256 does not reproduce from a pristine checkout of $BASE_REV — \
something uncommitted reached the baseline binary"
echo "prereg: baseline rebuild attests $BASE_DIGEST_RECORDED"

# ---- 5. The candidate tree is CLEAN, which became a reachable state at D-266.
#         The three-way split is the DIAGNOSIS, because the refusal has three
#         reasons and one combined test gives one wrong answer (item 8). ----
DIRT="$(git status --porcelain)" || refuse "cannot read the working tree status"
if [ -n "$DIRT" ]; then
  TRACKED="$(git status --porcelain --untracked-files=no)" || refuse "cannot read the tracked-file status"
  [ -z "$TRACKED" ] || refuse "tracked files are modified: $TRACKED"
  STRAY="$(git ls-files --others --exclude-standard -- Cargo.toml Cargo.lock crates configs tools)" \
    || refuse "cannot enumerate untracked files"
  [ -z "$STRAY" ] || refuse "untracked files on build-reaching paths: $STRAY"
  refuse "untracked files outside the build-reaching set: $DIRT"
fi

# ---- 6. Revision assertions. HEAD is NOT required to equal LANDING: the closure
#         session lands ADR lines above the work package, and an equality test
#         would void every governed run over docs commits that cannot reach a
#         binary. What the run needs is that nothing between LANDING and HEAD can
#         reach the binary OR the instrument, and that is a DIFF, not a token. ----
LAND_OBJ="$(git rev-parse --verify --quiet "$LANDING^{commit}")" \
  || refuse "LANDING $LANDING is not a commit in this repository"
[ "$LAND_OBJ" = "$LANDING" ] || refuse "LANDING $LANDING resolves to $LAND_OBJ"
BASE_OBJ="$(git rev-parse --verify --quiet "$BASE_REV^{commit}")" \
  || refuse "BASE_REV $BASE_REV is not a commit in this repository"
[ "$BASE_OBJ" = "$BASE_REV" ] || refuse "BASE_REV $BASE_REV resolves to $BASE_OBJ"
[ "$LANDING" != "$BASE_REV" ] || refuse "candidate and baseline are the same revision"
anc=0
git merge-base --is-ancestor "$LANDING" HEAD || anc=$?
case "$anc" in
  0) ;;
  1) refuse "LANDING $LANDING is not an ancestor of HEAD — this tree is not a continuation of the landing revision" ;;
  *) refuse "git merge-base could not decide whether $LANDING is an ancestor of HEAD (status $anc)" ;;
esac
DRIFT="$(git diff --name-only "$LANDING" HEAD -- Cargo.toml Cargo.lock crates configs tools)" \
  || refuse "cannot diff $LANDING against HEAD"
[ -z "$DRIFT" ] || refuse "build-reaching or instrument paths moved between LANDING and HEAD: $DRIFT"
# THE SOLVER-DIFF GUARD. Revision 8 wrote `git diff --stat … | grep -q . || refuse`,
# which is SHELL_CHECKLIST item 3 in the position where it costs the most: this is
# §7.1's externally derived fix for M-4 and the one guard standing between "the WP
# landed" and "the WP never landed". MEASURED on that construction: `deadbeef…`
# printed `fatal: bad object` and the run announced `changes nothing under
# crates/pistol-solver` — this guard's own conclusion, about an invocation that
# never answered. No pipeline, and the value's SHAPE is checked (items 1 and 8).
# Its "cannot diff" arm is UNREACHABLE BY CONSTRUCTION now that both revisions are
# verified above; it stays because unreachable-and-routed beats unreachable-and-fatal,
# and §3.2 carries no row for it precisely because no input reaches it.
SOLVER_DIFF="$(git diff --name-only "$BASE_REV" "$LANDING" -- crates/pistol-solver)" \
  || refuse "cannot diff crates/pistol-solver between $BASE_REV and $LANDING — the guard did not \
answer, which is not the same as answering that the diff is empty"
[ -n "$SOLVER_DIFF" ] || refuse "$LANDING changes nothing under crates/pistol-solver: with p = 0 an \
empty diff is also what 'the WP never landed' looks like"

# ---- 7. p = 0, ADJUDICATED ON THE RESOLVED GRAPH.
#         Revision 9 adjudicated on a substring count over manifest TEXT, and a
#         red-team broke it in both directions: a `# … pistol-solver …` COMMENT in
#         an unrelated manifest produced `2 Cargo.toml files … 2 or more means an
#         edge` — a RUN VOID naming an edge that does not exist — while a real
#         `[dependencies."pistol-solver"]` edge that cargo resolves was
#         INVISIBLE to the count. By §7.1's own rule the count shares its input
#         (manifest text) with the defect it must exclude. `cargo tree -i` asks
#         cargo's RESOLVED graph and answers by EXIT STATUS, parsing nothing. ----
( cd "$PRISTINE/repo" && git checkout --quiet "$LANDING" ) || refuse "landing revision not in the clone"
# Readability first, so "cargo could not answer" and "no edge" are two reasons with
# two refusals rather than one status 101 meaning either (item 8).
( cd "$PRISTINE/repo" && cargo tree --locked -p pistol-cli --edges normal ) >/dev/null 2>&1 \
  || refuse "cargo could not resolve pistol-cli's normal-edge graph at $LANDING; the p = 0 \
attestation was not taken"
if ( cd "$PRISTINE/repo" && cargo tree --locked -p pistol-cli --edges normal -i pistol-solver ) >/dev/null 2>&1; then
  echo "prereg: p = 0 REFUTED — pistol-solver IS in pistol-cli's resolved normal-edge graph; \
run 'cargo tree -p pistol-cli --edges normal -i pistol-solver' for the reverse-dependency path" >&2
  exit 1
fi
echo "prereg: p = 0 — pistol-solver is absent from pistol-cli's resolved normal-edge graph"
# The manifest attestation is RECORDED and no longer ADJUDICATED ON, for the two
# reasons above. `LC_ALL=C sort` is not decoration: D-265 records that an agent
# shell's `grep` is a harness-injected multithreaded wrapper whose output ORDER is
# nondeterministic, so a recorded transcript is sorted or it is not a transcript.
ATTEST="$(git grep -n -e 'pistol-solver' -e 'pistol_solver' "$LANDING" -- '*Cargo.toml' | LC_ALL=C sort)" \
  || refuse "the p = 0 manifest attestation could not be taken at $LANDING"
printf '%s\n' "$ATTEST"

# ---- 8. H1-a: build LANDING, then build it with the solver crate REMOVED. ----
CAND_BIN="$(cd "$PRISTINE/repo" && cargo build --release --locked -p pistol-cli --bin pistol \
    --message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
  || refuse "candidate build failed"
[ -s "$CAND_BIN" ] || refuse "candidate build produced no binary"
D_WITH="$(sha256sum -- "$CAND_BIN" | cut -d' ' -f1)" || refuse "cannot digest the candidate build"
cp -- "$CAND_BIN" "$WORK/with_solver.pistol" || refuse "cannot preserve the candidate binary"
# `rm -rf` first: `git checkout BASE_REV -- <path>` MERGES the old files in and
# leaves every file the WP added on disk, and cargo auto-discovers `build.rs`,
# `src/bin/`, `benches/`, `examples/` and `tests/` by convention. The result is a
# REVERT to the baseline's content, not an absence — §3.1a says so, and a future WP
# that CREATES the crate cannot be measured this way at all (the checkout refuses).
( cd "$PRISTINE/repo" && rm -rf -- crates/pistol-solver \
    && git checkout --quiet "$BASE_REV" -- crates/pistol-solver ) \
  || refuse "cannot restore the solver crate to its baseline content"
# `--locked` is deliberately DROPPED here and only here: removing the solver's
# dependency edge necessarily moves Cargo.lock, so `--locked` would refuse.
COUNTER_BIN="$(cd "$PRISTINE/repo" && cargo build --release -p pistol-cli --bin pistol \
    --message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
  || refuse "counterfactual build failed"
[ -s "$COUNTER_BIN" ] || refuse "counterfactual build produced no binary"
D_WITHOUT="$(sha256sum -- "$COUNTER_BIN" | cut -d' ' -f1)" || refuse "cannot digest the counterfactual build"
# THE GUARD --locked GAVE UP, WHICH REVISION 9's COMMENT CLAIMED AND DID NOT HAVE.
# Two assertions, after the build so cargo has re-resolved. `|| true` is gone: it
# rendered a failed probe, an absent directory and a genuinely empty delta as the
# same ` none` (item 8), and ` none` is a value DR-8 recorded as legitimate.
LOCK_DELTA="$(cd "$PRISTINE/repo" && git diff --shortstat -- Cargo.lock)" \
  || refuse "cannot read the counterfactual's Cargo.lock delta"
case "$LOCK_DELTA" in
  "") ;;
  *insertion*) refuse "the counterfactual ADDED lines to Cargo.lock ($LOCK_DELTA): dropping the \
solver's edge can only delete, so cargo re-resolved something else — which is exactly the guard \
--locked gave up" ;;
  *deletion*) ;;
  *) refuse "unregistered Cargo.lock delta shape from the counterfactual: $LOCK_DELTA" ;;
esac
COUNTER_DIRT="$(cd "$PRISTINE/repo" && git status --porcelain -- ':(exclude)crates/pistol-solver' ':(exclude)Cargo.lock')" \
  || refuse "cannot read the counterfactual tree's status"
[ -z "$COUNTER_DIRT" ] || refuse "the counterfactual moved files outside the solver and Cargo.lock: $COUNTER_DIRT"
echo "prereg: H1-a with solver    $D_WITH"
echo "prereg: H1-a without solver $D_WITHOUT"
echo "prereg: H1-a counterfactual lock delta:${LOCK_DELTA:- none}"
[ "$D_WITH" = "$D_WITHOUT" ] || { echo "prereg: H1-a FAILED — the solver reaches the binary" >&2; exit 1; }
echo "prereg: H1-a CONFIRMED — the solver contributes nothing to the shipped binary"

# ---- 9. Toolchain comparison, whole-line and anchored. ----
# `rustc -vV | head -1` closes the pipe early, rustc takes SIGPIPE and pipefail
# propagates it — the cmd | head trap. Capture whole, then slice.
RUSTC_VV="$(rustc -vV)" || refuse "cannot read rustc -vV"
RUSTC_LINE="$(printf '%s\n' "$RUSTC_VV" | sed -n '1p')" || refuse "cannot slice rustc -vV"
[ -n "$RUSTC_LINE" ] || refuse "rustc -vV produced no first line"
CARGO_LINE="$(cargo --version)"     || refuse "cannot read cargo --version"
# Printed into the transcript rather than written to a file under $WORK, which the
# EXIT trap deletes — revision 9 kept both the file and §3.1a's claim that it is
# "kept as the run's own provenance record", and the trap removed it every run.
echo "prereg: toolchain candidate $RUSTC_LINE"
echo "prereg: toolchain candidate $CARGO_LINE"
if grep -Fxq -- "$RUSTC_LINE" "$BASE_TC" && grep -Fxq -- "$CARGO_LINE" "$BASE_TC"; then TC=yes; else TC=no; fi
echo "prereg: toolchain matches baseline: $TC"

# ---- 10. H1-b, UNCONDITIONAL. `binary_sha256` is excluded exactly as `revision`
#          is: it is the ONE line a linked-crate change is guaranteed to move. The
#          other 54 lines carry the behaviour. H1-b IS SECONDARY and its defect
#          class is NOT H1's — see §3. ----
tools/baseline_snapshot.sh --binary "$WORK/with_solver.pistol" --out "$WORK/cand.snapshot" \
  || refuse "the candidate snapshot could not be taken"
inv() {
  local f="$1" n kind
  [ -s "$f" ] || { echo "prereg: snapshot missing or empty at $f" >&2; return 2; }
  # The kind token, on BOTH sides. Revision 9 checked only the candidate, so an
  # INCOMPLETE BASELINE record reached the diff and was adjudicated ABORT where
  # §3.2 registers RUN VOID (D-160). A bare `sed -n 1p` and a string compare,
  # because `head -1 | grep -qx` is a pipeline and a grep where a comparison does
  # the work (items 1 and 3).
  kind="$(sed -n '1p' -- "$f")" || { echo "prereg: cannot read $f" >&2; return 2; }
  [ "$kind" = 'baseline_snapshot 1' ] \
    || { echo "prereg: $f is not a COMPLETE baseline_snapshot record: $kind" >&2; return 2; }
  # An empty result is not legitimate here: the marker's ABSENCE is the refusal.
  grep -q '^# timing' -- "$f" || { echo "prereg: no '# timing' marker in $f" >&2; return 2; }
  # `grep -c` prints 0 AND exits 1 (item 3), so `|| true` is load bearing; and
  # because `|| true` would equally mask a failing `sed`, the SPELLING of what came
  # back is validated rather than only its value (item 8).
  n="$(sed -n '1,/^# timing/p' -- "$f" | grep -c . || true)"
  case "$n" in ''|*[!0-9]*) echo "prereg: could not count the invariant block of $f" >&2; return 2 ;; esac
  [ "$n" -ge 50 ] || { echo "prereg: invariant block short ($n) in $f" >&2; return 2; }
  sed -n '1,/^# timing/p' -- "$f" | sed '/^# timing/d' | grep -v '^revision \|^binary_sha256 ' || true
}
inv "$BASE" > "$WORK/inv.base" || refuse "baseline record failed its shape checks"
inv "$WORK/cand.snapshot" > "$WORK/inv.cand" || refuse "candidate record failed its shape checks"
# `echo "… $(wc -l < …) behaviour lines"` is item 1 exactly: the substitution's
# status is the echo's ARGUMENT and an unreadable file prints an empty field with
# exit 0. Take it into a variable, check its shape, refuse.
INV_LINES="$(wc -l < "$WORK/inv.base")" || refuse "cannot count the baseline behaviour lines"
case "$INV_LINES" in ''|*[!0-9]*) refuse "the baseline behaviour-line count is not a number: $INV_LINES" ;; esac
[ "$INV_LINES" -ge 50 ] || refuse "only $INV_LINES behaviour lines survive the exclusions; the comparison would be vacuous"
echo "prereg: H1-b comparing $INV_LINES behaviour lines (revision and binary_sha256 excluded)"
# Informational only, never a gate: which build-reaching paths outside the solver
# moved. `tools` is in the pathspec because step 6 calls it instrument-reaching.
OTHER="$(git diff --name-only "$BASE_REV" "$LANDING" -- Cargo.toml Cargo.lock crates configs tools \
         ':(exclude)crates/pistol-solver' | LC_ALL=C sort)" || true
[ -z "$OTHER" ] || { echo "prereg: note — build-reaching paths outside the solver also moved:";
                     echo "$OTHER" | sed 's/^/prereg:   /'; }
# `diff -u` writes the two file PATHS and their mtimes to STDOUT, and `$WORK` is a
# `mktemp -d` name. On the ABORT path three replications therefore produced THREE
# DISTINCT STDOUTS with identical exit codes and identical diff bodies — measured
# independently by two reviewers — and §6 voids a run whose replications disagree.
# So a genuine ABORT was converted into a voided run by a temporary directory name.
# `--label` fixes it; `cmp` gives the verdict its own named line, which revision 9's
# bare `set -e` on `diff` did not (rule 3, item 1's "name the refusal").
if cmp -s -- "$WORK/inv.base" "$WORK/inv.cand"; then
  echo "prereg: H1-b CONFIRMED — every behaviour line is byte-identical"
else
  diff -u --label baseline --label candidate -- "$WORK/inv.base" "$WORK/inv.cand" || true
  echo "prereg: H1-b FAILED — a behaviour line moved between $BASE_REV and $LANDING" >&2
  exit 1
fi
```

### 3.1a What H1-a's second build actually does, said plainly

**The "revert" is a REVERT, not an absence — revision 9's wording here was wrong and a red-team
said so.** `git checkout BASE_REV -- <path>` merges the old files in and leaves every file the work
package added on disk, so §3.1 does `rm -rf -- crates/pistol-solver` **first**; cargo auto-discovers
`build.rs`, `src/bin/`, `benches/`, `examples/` and `tests/` by convention, and without the `rm` a
future WP adding any of those would have them survive. But what the two steps together produce is
the crate **at its BASELINE content**, not the crate's absence — H1-a's own statement says so
correctly and §3.1a's summary line did not. **The structural consequence is worth writing down:
if a future work package CREATES the crate, H1-a cannot answer at all** — measured against the
parent of the crate's creating commit, `git checkout` refuses with
`error: pathspec 'crates/pistol-solver' did not match any file(s) known to git`, status 1 →
`refuse` → exit 2. That is the correct disposition (RUN VOID, not a verdict) and it is a limit on
where this instrument can be reused.

**`--locked` is dropped on the counterfactual build, deliberately and only there — AND THE GUARD
THAT REPLACES IT NOW EXISTS.** Removing the solver's `pistol-core` edge necessarily moves
`Cargo.lock`, so `--locked` would refuse the build outright. Revision 9's block comment said the
guard was *"replaced by the assertion below that the lock moves in exactly that one way"*, and
below it were an assignment with `|| true` and an `echo`. **That is revision 7's own M-3 defect —
a cited guard absent from the registered block — transposed into the block's comments**, where a
reader auditing the mechanism meets it first. Two real assertions replace it:

1. the `Cargo.lock` delta may be empty or may contain DELETIONS, and **any insertion refuses** —
   dropping an edge can only remove lines, so an insertion means cargo re-resolved something else,
   which is precisely the guard `--locked` gave up;
2. **nothing outside the solver and `Cargo.lock` moved** in the counterfactual tree.

And the `|| true` is gone. It rendered a failed probe, an absent directory and a genuinely empty
delta as the same ` none` (`SHELL_CHECKLIST` item 8) — and ` none` is a value DR-8 recorded as
legitimate, so the substitute for a dropped guard was a printed field whose failure was
indistinguishable from its success value.

**The delta is pair-dependent, which is why the assertion is on SHAPE and not on a string.**
`(cdbcbf0, 8618012)` and DR-9's `(72316a7, 60b5c44)` both give `1 file changed, 3 deletions(-)` —
the three lines of the solver's `pistol-core` edge. DR-8's `(5fdbf52, 60b5c44)` gives **none**,
because that baseline already carries the edge, so the counterfactual reverts content without
changing the graph.

**And when `p = 0`, the second build compiles nothing.** Cargo finds the crate graph unchanged —
the solver is not in it — and returns `Finished release profile in 0.02s`, handing back **the same
file** as the first build: same inode, same mtime. So `[ "$D_WITH" = "$D_WITHOUT" ]` compares a
file's digest to its own. **The mechanism is still sound**, and that is not a rescue: when the
solver *does* reach codegen, cargo rebuilds and the digests differ, demonstrated at exit 1 with
`570dc5d8…` against `1ed322ea…`. **What H1-a actually asserts is: *cargo, given the tree with the
solver reverted, produces a binary it considers identical — and when that is false it rebuilds and
says so.***

**The candidate toolchain is PRINTED INTO THE TRANSCRIPT, not written to a file (revision 10).**
Revision 9 wrote it to `$WORK/toolchain.cand` and claimed it was *"kept as the run's own provenance
record"* — while the EXIT trap added in the same round deleted `$WORK` on every exit, so the file
was destroyed every run and the sentence was false the moment it was written. Two `prereg:
toolchain candidate …` lines carry it instead, where §6's replication can compare them. The
comparison itself reads `$BASE_TC` directly with `grep -Fxq`, whole-line

### 3.2 Adjudication — every row has a reachable producer, and each was RUN

Revision 7's table had **five of eight rows with no reachable producer**, all downstream of H1-b's
unconditional `N/A`. That is hardening converting a test into a formality, and **revision 9 did it
again in miniature**: it added a `cannot diff crates/pistol-solver` row that no input can reach,
because step 6's `merge-base --is-ancestor` and step 2's `revision`-line comparison catch an
unresolvable `LANDING` and an unresolvable `BASE_REV` respectively, three lines earlier and one
step earlier. **That row is gone.** The guard stays in the block — unreachable-and-routed beats
unreachable-and-fatal — but a table row is a claim that an input produces it, and no input does.
The table below carries only refusals something reached, and §7.5 names what reached each.

| exit | Reading | Verdict | produced by, and RUN |
|---|---|---|---|
| **0** | H1-a digests identical **and** H1-b's 54 behaviour lines identical | **CONFIRMED** | DR-10a, DR-9, ARM P |
| **1** | the resolved graph contains a `pistol-solver` edge | **ABORT** — `p` is no longer 0; the message carries its own diagnostic command | ARM Q, an escaped `[dependencies."pistol-solve\u0072"]` edge the substring count could not see |
| **1** | H1-a digests differ | **ABORT.** Solver content reached codegen | an `include_str!` from a called path: `570dc5d8…` vs `1ed322ea…` |
| **1** | H1-b's behaviour lines differ | **ABORT** — a behaviour change the solver did not cause is still a change the run must not pass over. **Named**, and its stdout is now replication-stable | ARM K, a mutated baseline behaviour line: exit 1 ×3, ONE distinct stdout, `prereg: H1-b FAILED — a behaviour line moved between …` |
| **2** | `LANDING is not a 40-hex object name` | **RUN VOID** | ARM L, `LANDING=dev` — which revision 9 ACCEPTED |
| **2** | `the snapshot instrument on disk hashes to … not the registered …` | **RUN VOID** | ARM N, `assume-unchanged` + a worktree edit, with `git status --porcelain` EMPTY |
| **2** | `… is not a COMPLETE baseline_snapshot record: …` | **RUN VOID** (D-160) | ARM M, an incomplete BASELINE record — which revision 9 adjudicated ABORT |
| **2** | `a path binding contains a non-printable character …` | **RUN VOID** | ARM O, a newline in `BASE` |
| **2** | `cargo could not resolve pistol-cli's normal-edge graph …` | **RUN VOID** — cargo did not answer, which is not the same as answering that there is no edge | a manifest edge absent from `Cargo.lock`, where `--locked` refuses |
| **2** | `build-reaching or instrument paths moved between LANDING and HEAD: …` | **RUN VOID** | ARM D |
| **2** | `tracked files are modified: …` | **RUN VOID** | ARM G |
| **2** | `untracked files on build-reaching paths: …` | **RUN VOID** | ARM F; and ARM J, the same input from a subdirectory |
| **2** | `untracked files outside the build-reaching set: …` | **RUN VOID** | ARM E |
| **2** | `LANDING … is not an ancestor of HEAD` | **RUN VOID** | ARM C |
| **2** | `… changes nothing under crates/pistol-solver` | **RUN VOID** — with `p = 0` an empty diff is also what "the WP never landed" looks like | ARM H2 |
| **2** | any other `refuse` | **RUN VOID**, not a verdict in either direction | missing/misdigested/unreadable baseline; unbound binding; non-repository cwd; a failed build |
| — | `toolchain matches baseline: no` | **advisory.** H1-a rebuilds both sides on one toolchain and is immune; it bears only on H1-b | — |

**THE STEP NUMBERS IN THIS TABLE ARE THE BLOCK'S OWN.** They were not until revision 9: the

### 3.3 The untracked-file tolerance is RETIRED, and the two checks become the diagnosis

**Revisions 6-8 registered a tolerance because a bare emptiness test could not pass.**
`git status --porcelain` was non-empty forever in this tree: `docs/research/threat_calculus_v1.md`
sat untracked, and the previous session was told it would stay that way. The closure session
committed it (**D-266**), so the condition is gone and the bare test is a reachable check again.
It is the registered check as of revision 9.

**This was not a cosmetic condition, and the same untracked file was disabling an instrument
elsewhere.** `tools/baseline_snapshot.sh:417` sets its `TREE` token from a bare
`git status --porcelain`, so for as long as the file sat there every snapshot this project took
recorded `timing tree dirty` and `clean` was **unreachable** — not failing, unreachable, which is
worse, because a token that can only take one value reads as a passing check. §0.2's "the record's
`timing tree` reads `dirty`, which is expected" was true and was a description of a broken
instrument, and the pristine-rebuild attestation was carrying the whole load alone.

**The two checks survive as the DIAGNOSIS, not as the gate.** The refusal has three reasons and a
single combined test gives one wrong answer (`SHELL_CHECKLIST` item 8), so step 5 refuses on a
non-empty `git status --porcelain` and then says WHICH:

1. **A tracked modification anywhere** — `git status --porcelain --untracked-files=no` non-empty.
   Any edit to a file git knows about, whatever its path.
2. **An untracked file on a build-reaching path** — `git ls-files --others --exclude-standard`
   restricted to `Cargo.toml`, `Cargo.lock`, `crates`, `configs`, `tools`. The paths are
   enumerated rather than described.
3. **Anything else untracked** — which is no longer tolerated, only named.

**AND THAT ENUMERATION IS CWD-RELATIVE, WHICH IS WHY THE BLOCK NOW `cd`s (revision 9).**
`git ls-files -- <pathspec>` resolves its pathspec against the current directory, so the same
command run from `crates/` asks about `crates/crates`. Measured with
`crates/pistol-core/ZZZ_untracked_probe.rs` present: from `crates/`, `git status --porcelain`
reported the file and the registered enumeration returned **EMPTY**. A gate that passes because of
where it was invoked is EXIT-0-WRONG-ANSWER with no bad input at all, and `tools/` at step 10 is a
relative path besides. The block enters `$REPO` before it enumerates anything.

**What the retirement does NOT change**: H1-a rebuilds both sides in a *pristine clone* of
committed content, where untracked files do not exist. That was always the reason a rebuild is
preferred to a tidiness token, and it is unaffected by which tidiness token is registered.

## 4. T1 — disposition, and the deviation named

**This is a T1 hit.** Revision 1 declared T1 impossible on the ground that no `tools/` harness
is modified; but the harness producing the registered number was a scratchpad bench, unpinned,
depending on a sibling scratchpad directory by relative path, and revision 1's own text had
the governed run *substitute the shipped type into it*. That is a harness modified, for the
run it governs, reviewed by nobody.

**Substance, endorsed by review:** the dry-run and governing-revision rules attach to **any
artefact that produces a registered number**, not to `tools/` alone.

**Form, corrected in revision 3.** D-245 is the one worked precedent and it **amends
CLAUDE.md's Process section in the same commit** as its ADR line, for D-228's reason — a rule
whose limits live only in a decision log is one the next reader applies without them.
Revision 2 proposed only an ADR line. **Both are now proposed**: an ADR line citing T1 and
describing this instance, and a clause in CLAUDE.md's Process section generalising "harness"
beyond `tools/`.

**THE DEVIATION IS CLOSED, AND T1 IS DISPOSITIONED — D-268.** Revisions 3-8 held T1 for
operator confirmation and named the hold as a deliberate conservative deviation by a session
without commit authority — not a claim that the ruling was the operator's by right. A reviewer
judged revision 1's situation to fall in **D-242's excluded class** (a defect licensing a run
that should have been refused), which takes an amendment immediately rather than being deferred;
on that reading the deviation cost a delay and nothing else, and the delay is now spent. The
closure session landed both halves in one commit per D-245's precedent: **D-268** as the ADR
line, and a clause in CLAUDE.md's Process section beside the governing-revision paragraph saying
that an artefact producing a registered number is named with its revision and that a change to it
reopens the review. **This document then complied with it in §0.3** rather than being the first
document to ignore the clause its own history caused.

**One thing D-268 records that this section could not know:** the same reading was produced
independently by the fresh-context re-derivation of the lost T-bucket items (D-270), from the
paragraph text alone, by a reader who had never seen T1 — *"the literal commands can be identical
while the INSTRUMENT they name changes underneath them"*. Two grounds, one clause.

**Note that H2's removal does not moot this.** WP-1.5b will register the storage measurement
and will need the same rule.

---

## 5. The storage measurement is deferred to WP-1.5b — what it must do

Registered here as an obligation so it is not lost, and so WP-1.5b does not repeat this round.

**Why it could not be measured here.** `p = 0`: no binary in this workspace calls
`pistol-solver`, so no whole-engine instrument can observe the storage choice, and every
isolated instrument this session built proved unable to resolve the margin (§6 of DESIGN.md
§5.4.3, and the contradiction between two harnesses in §5.4.5).

**What WP-1.5b must measure.** These bind away from four known defects; **they deliberately
name no bracket, and item 1's share figure is context, not a target** — quantities, the
instrument and the comparand are WP-1.5b's to register, and pre-judging them here would be
this document making a registration it cannot review (prereg-MINOR-3):

1. **At the level of the shipped structure**, not an isolated table — the maintained sets are
   **30.3–31.4 %** of the per-stone path and no table choice touches them.
2. **With plies coloured through `GameState`**, never `i % 2`. The instrument this WP
   registered coloured `i % 2`, mis-coloured **7 of 15 stones on position 0**, and agreed with
   the true board on **0 of 24 corpus positions**.
3. **With store construction hoisted out of the timed region** — the S-6 defect, present in
   both harnesses this session used.
4. **With all ten maintained sets**, not six.
5. **With the comparand in the same run**, so the document can fail the decision it defends.
6. **Against an external check**, not internal agreement — see §7's DR-6 finding.

**And a warning WP-1.5b should inherit:** a table-only `k` overstates what a whole structure
delivers, here by roughly a factor of two (4.4–4.9 against 1.649–1.681).

---

## 6. Cost, replication, second instrument, abort

**Cost, measured** (two revision-2 rows were labelled measured and did not reproduce, m-2;
both were conservative, so T4 is unaffected, and both are corrected here):

| Instrument | Unit cost, MEASURED at the governed bindings | Reps | Total |
|---|---|---|---|
| the §3 block, end to end | **43 s** (3 clones+builds, 1 snapshot, 2 comparisons) | 3 | **≈ 2 min 10 s** |
| ~~`tools/bench_delta.sh` on the governed pair~~ | **6 min 18 s** — DEMOTED at revision 10; it adjudicates nothing and is not on the critical path | 0 | **0** |
| `cargo tree --locked -p pistol-cli --edges normal -i pistol-solver` | **under a second**, inside the block; no build | 3 | negligible |
| disk, per block run | **61 MB** (`PRISTINE` + `WORK`) | 3 | 183 MB, **now reclaimed by an `EXIT` trap** |

**Revision 7's cost table was wrong in both directions and is corrected (m-4).** It billed
`baseline_snapshot.sh` at 33 s × 4 for a run that took **zero** snapshots, and billed
`bench_delta.sh` at its 9 s refusal path when on the governed pair it does not refuse and runs
**6 m 18 s**. The headline survives — still well under an hour, one workstation — so T4 is
unaffected; but proportionality asks the face of the document to be right about which
instrument costs what. **Also previously unbilled: 61 MB of disk per run, never cleaned.** Ten
accumulated runs had left 605 MB; §3.1 now sets `trap cleanup EXIT` and leaks nothing.

**Machine hours: about 2 min 10 s at revision 10 (was under 10 minutes with `bench_delta` in the
set), one workstation, single thread. Operator attention: one invocation, no judgement call during
the run. Wall time: under 5 minutes.**

**Revision 10 REMOVES an instrument and adds a cheaper one, so the run got shorter.** Demoting
`tools/bench_delta.sh` takes **6 m 18 s** off the total; the `cargo tree` probe that replaces it as
the second instrument costs under a second and runs inside the block, so the governed run is now
three replications of a ~43 s block and nothing else. **Total machine time: about 2 min 10 s.**

**Revision 9 added no instrument and no rep, so the cost table stood unchanged then.** The four
repairs are routing: one trap, one guard rewritten from a pipeline to a variable, one equality
test replaced by an ancestor test and a diff, one tolerance retired. None of them builds anything
or takes a snapshot. The one measurable difference is that the block now `cd`s to `$REPO` before
enumerating, which costs nothing. **The dry runs this revision added (§7.3) cost 197 s of wall
clock, measured** — DR-8 79 s, DR-9 82 s, the refusal arms 36 s — plus about a minute of
re-runs after two arms of my own leaked state into each other (§7.3, ARM G). They are recorded
there with their inputs and their outputs and are not governed samples.

**The build environment is part of what is pinned (prereg-MINOR-1).** This workspace has no
`rust-toolchain.toml` and `rust-version` is only a floor, so a `rustup update` between the
§0-step-1 baseline and the candidate changes `binary_sha256` **alone** — as does a
`[build] target = <triple>` with identical source. **The `rustc -vV` comparison does NOT cover
that second case and revision 7 implied it did (m-5):** `rustc -vV` reports `host:`, the
compiler's own host triple, not a configured build target, so a `[build] target` change leaves
it identical. What actually covers it is the **pristine rebuild attestation** — a configured
target that moved the digest would fail the baseline rebuild — and the untracked-file check,
which refuses a stray `.cargo/config.toml` on a build-reaching path. The toolchain comparison
covers the compiler-version case only. `rustc -vV` is therefore recorded beside **each**
snapshot and compared; the run this document was drafted against records `1.97.1`.

The run is **cheap**, so this document **applies** the proportionality rule's cheap-run clause
rather than arguing out of it — T4's binding direction (D-245). **No derived margin appears
anywhere**; with H2 gone there is no bracket to defend.

**Replication — REGISTERED AGAINST WHAT THE RUN ACTUALLY DOES (B-2).** Revision 7 required
"three candidate snapshots" identical to one another. **The governed run took zero snapshots**,
because the only `baseline_snapshot.sh` call sat inside H1-b's dead `else` branch — a
replication clause the run could not perform, against a proportionality rule that requires one.

**What is registered instead: the BLOCK is replicated.** ≥ 3 independent processes, and their
**stdout must be byte-identical**. That covers everything the run produces — both digests, the
lock delta, the toolchain token, H1-b's line count and both verdicts. Measured at the governed
bindings: three processes, **12 lines, one distinct output, exit 0 each**, 43/45/42 s. A
disagreement between replications indicts the instrument and voids the run.

Under H1-b's unconditional form the run *does* now take one snapshot per process, so the older
clause is also satisfiable — but the block-level identity is the stronger statement and is the
registered one.

**AND AT REVISION 9 THIS CLAUSE VOIDED A GENUINE ABORT, WHICH IS B-1 AND WHICH TWO FRESH-CONTEXT
REVIEWERS FOUND INDEPENDENTLY.** `diff -u "$WORK/inv.base" "$WORK/inv.cand"` writes both file PATHS
and their mtimes to **stdout**, and `$WORK` is a `mktemp -d` name. On the CONFIRMED path stdout is
empty and stable — which is why every replication ever taken passed and why reading the block did
not find this. On the ABORT path three processes produce **three distinct stdouts with identical
exit codes and identical diff bodies**:

```
--- /tmp/wp15a_inv.CdAOH6/inv.base   2026-08-20 20:50:47.785221010 +0200
--- /tmp/wp15a_inv.QanZkk/inv.base   2026-08-20 20:52:20.842961647 +0200
--- /tmp/wp15a_inv.7DrmBq/inv.base   2026-08-20 20:41:53.892759656 +0200
```

So this clause **voids** a run that §3.2 calls **ABORT**, and §6's abort protocol attaches only to
ABORT — a verdict rewritten by a temporary directory name, which is revision 9's own headline
defect with a different mechanism. It also collides with the determinism law (rule 4) and with
D-265's principle that a transcript is reproducible or it is not a transcript.

**Fixed and re-measured at revision 10:** `diff -u --label baseline --label candidate`, and the
verdict itself moved to an explicit `cmp` so the ABORT path prints a NAMED refusal where a bare
`set -e` on `diff` printed none (rule 3). **ARM K: three independent processes on a mutated
baseline behaviour line — exit 1 each, ONE distinct stdout, `prereg: H1-b FAILED — a behaviour line
moved between …` on stderr, and the diff header reading `--- baseline` / `+++ candidate`.**

**SECOND INSTRUMENT — REPLACED AT REVISION 10, BECAUSE THE REGISTERED ONE WAS VIOLATED AT THE
REGISTERED BINDINGS AND ITS OWN CONSEQUENCE THEN FORBADE A VERDICT.**

Revisions 6-9 registered:

> *H1-b reports the invariant blocks identical **if and only if** instrument 2
> (`tools/bench_delta.sh`) refuses with `the two sides resolve to the same binary`.*

**Measured at the registered bindings, both sides:** `tools/bench_delta.sh:274` is
`[ "$BASE_SHA" != "$CAND_SHA" ] || fail "the two sides resolve to the same binary …"` — it refuses
**only when the two digests are EQUAL**. `cdbcbf0` gives `ff018398…` and `8618012` gives
`a7f519fa…`, so it does **not** refuse, while H1-b **CONFIRMS**. TRUE ⇔ FALSE is false, the
criterion is violated, and the registered consequence below then says in terms that no verdict may
be taken. §6's own cost table said the same thing in the same section — *"it does **not** refuse
here, because the digests differ"* — and the criterion was never re-read against it.

**The paragraph defended itself with a sentence that had been dead for three revisions:** *"H1-b is
`N/A`. Both sides of the biconditional are false, so the criterion is satisfied."* H1-b stopped
being able to be `N/A` at revision 6. There is no `N/A` path in the block.

**AND THE DEEPER FAULT IS THE ONE THAT DECIDES THE FIX.** Under **D-269** — the clause this
document's own §7.1 produced — a registered criterion must be one its named defect class could
falsify. **H1-b is a criterion H1's target defect PRESERVES** (§3): a solver crate that is linked
but never called moves `binary_sha256`, which H1-b excludes, and moves none of the 54 lines that
remain. The proportionality rule's second-instrument duty was anchored to the one criterion in the
document that cannot falsify what it watches. Rewording the biconditional would have left that
intact.

**THE REGISTERED SECOND INSTRUMENT IS NOW `cargo tree`, AND IT IS H1-a's.**

```sh
cargo tree --locked -p pistol-cli --edges normal -i pistol-solver
```

> **AGREEMENT CRITERION.** H1-a reports the two binaries bit-identical **if and only if** the
> resolved normal-edge graph of `pistol-cli` at `LANDING` contains no `pistol-solver` — that is,
> the command above exits non-zero with `package ID specification … did not match any packages`.

**Why this is a genuine second instrument and not a second reading of the first.** Both evaluate
one proposition — *does the solver reach the shipped binary* — and **neither reads the other's
input**: H1-a compiles and digests bytes, `cargo tree` resolves a dependency graph from manifests
and the lockfile. Neither shares an input with the other, which is what T8 (`docs/process_readings.md`)
observes the proportionality paragraph never required and what makes agreement mean anything. It is
also **cheap**: no build, seconds, against the 6 m 18 s the displaced instrument cost.

**Both sides were run at the registered pair's shape.** DR-10a and DR-9 give
`p = 0 — pistol-solver is absent from pistol-cli's resolved normal-edge graph` together with
`H1-a CONFIRMED`; ARM Q gives the other corner — an escaped `[dependencies."pistol-solve\u0072"]`
edge invisible to a substring count, where `cargo tree -i` exits 0 with the reverse-dependency path
and the block aborts at exit 1. **Both corners of the biconditional have producers**, which the
displaced criterion never did.

**`tools/bench_delta.sh` IS DEMOTED TO A RECORDED CORROBORATION WITH NO ADJUDICATING ROLE.** Its
refusal condition is digest-EQUALITY, which is not the proposition; it compares a different pair
(`BASE_REV` against `LANDING`, not the landing tree with and without the solver); and it emits
`VERDICT ABORT` twice per run meaning **D-215's** verdict against a different pre-registration —
two ABORTs meaning contradictory things in one governed transcript is a hazard these rounds have
been spent removing, and the cheapest removal is to stop asking an instrument a question it does
not answer. Its recorded reading, for provenance only: run at `(cdbcbf0, 7b9e904)` it built both
sides in throwaway worktrees and reported `ff018398…` and `a7f519fa…`, **independently reproducing
both digests obtained from a pristine clone**. Nothing it prints is a verdict of this document, and
this revision removes the last place one of its outputs was read as one.

**REGISTERED CONSEQUENCE OF DISAGREEMENT (D-245).** No verdict is taken; the WP lands on
neither instrument alone; a finding names which disagreed and quotes both outputs; a bare
`sha256sum` of both binaries is recorded as a localising third read that adjudicates nothing;
and this document is amended before any further run, reopening its review.

**Abort produces a recorded finding — never a silent retry, never a moved threshold.** On an
H1 abort the WP does not land, the finding records the differing lines verbatim, and
`cargo tree` is the first diagnostic. If an edge is real and intended, `p` is no longer 0,
this document is **wrong rather than failed**, and its successor registers a real bracket
against a measured `p`.

---

## 7. DRY RUNS

All on **real revisions of this repository** and never on the registered workload. Each states in
advance what it must **SHOW**, and from revision 9 also **names the defect class that criterion
excludes** (D-269, which this document's own §7.1 finding produced).

**T3 does not arise, and revision 9 states why more carefully than "the workload does not exist
yet".** Earlier revisions said the registered workload had not been created; it has been since
`8618012`, so that reading has expired. What holds now is the original point: the registered
workload is ONE PAIR — a baseline record at one revision against a landing revision — and this
repository has many pairs of that kind, so a same-kind input differing only in identity is
available and was used twice (§7.3). T3 is the case where the workload is the ONLY instance of
its kind; this workload is not.

**DR-1 — digest, negative control.** Input `1c27974` (docs/fixture/test commits above
`050961d`). Must show a digest identical to `050961d`'s. Output
`62c102cc30e017d16e282efe3e9f307397b9704f34b504a07d3cc4b427574e6c`, identical. **PASS.**
Re-verified at HEAD by two reviewers across four commits.

**DR-2 — digest, positive control.** Input `f31cffe` and `bdfca3f`. Must show two different
digests each equal to D-220's record. Output `09068541…` and `681e1a2…`. **PASS.**

**DR-3 — the invariant comparison. FAILED FIRST AND FIXED THE COMMAND.** Two real revisions
with the same binary; must show identical blocks excluding `revision`. First attempt passed
`--nodes 50000` and the blocks **differed** on `budget nodes 50000 registered` vs `… OVERRIDE`
— same number, differing in how it was supplied, a **false ABORT**. Corrected command omits
`--nodes`; re-run **IDENTICAL**, 55 lines each side. **PASS after correction.**

**DR-4 — second instrument.** `rev:050961d rev:1c27974` in a throwaway clone. Must show the
same-binary refusal by name after the script builds and digests both sides itself. Output:
both digested to `62c102cc…`, then `FAIL: the two sides resolve to the same binary`. **PASS.**

**DR-5 — the baseline guard.** Input: a **missing** baseline path — a real instance of the
failure §3.2 names. Must show the guard **refuses** rather than adjudicating. Unguarded,
`inv()` exits 0 with 0 bytes and the diff is maximal → ABORT on a missing file. Guarded: exit
**2**, `prereg: baseline record missing at …`, **no verdict**. Also verified accepting the
pinned baseline (55 lines, `sha256sum -c` OK) and refusing a truncated record (exit 2).
**PASS**, confirmed independently by a reviewer in all three directions.

**DR-7 — the cargo-derived binary path.** Input: two real builds in this repository. Must show
(a) that the path is **derived from the build** rather than assumed, and (b) that a build
producing no executable **refuses** rather than proceeding.

**Revision 7's version of this entry cited a guard that did not exist in the registered block
(M-3)** — it claimed *"the registered `[ -n "$BIN" ]` guard takes its exit-2 branch"*, and
§3.1 contained no such guard; with none, an empty path exits **1**, not 2. That is revision 4's
shape exactly: a dry run certifying a comparand absent from the thing it governs. **Both halves
are fixed.** §3.1 now guards every derived path and every digest — `[ -s "$BASE_BIN" ]`,
`[ -s "$CAND_BIN" ]`, `[ -s "$COUNTER_BIN" ]`, each followed by a `|| refuse` on the digest
assignment — and the behaviour is re-measured: an empty binary path now exits **2** with
`prereg: candidate build produced no binary`, where before the fix the same input exited **1**
unrouted. **PASS, on the corrected text.**

### 7.2 RULE 2 — the registered block, EXECUTED after amendment, in THREE ARMS

Two rounds running, the findings that mattered came from *running* the blocks rather than
reading them. Revision 3's BLOCKING was a `grep -qx` that refused every real record; revision
4's was a composed run that **CONFIRMED across two revisions eight commits apart**. So the
block is executed after every amendment, in arms that exercise each verdict, and the transcript
is pasted here.

**The block below was extracted from this document by script — not retyped — and run.**

### REVISION 8 — THE GOVERNED-SHAPE RUN, AND EVERY OTHER PATH

Block extracted from this document by script, not retyped, and run at the six registered
bindings (`BASE_REV=cdbcbf0…`, `LANDING=8618012…` = HEAD). **exit 0, 43 s.**

```
prereg: baseline rebuild attests ff018398a88673c6929efe875768f299358acf22b112ecfa1273e5a845e427ef
861801247df5c1a73480b5153e11c399aa752750:crates/pistol-solver/Cargo.toml:2:name = "pistol-solver"
prereg: H1-a with solver    a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a without solver a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a counterfactual lock delta: 1 file changed, 3 deletions(-)
prereg: H1-a CONFIRMED — the solver contributes nothing to the shipped binary
prereg: toolchain matches baseline: yes
prereg: H1-b comparing 54 behaviour lines (revision and binary_sha256 excluded)
prereg: note — build-reaching paths outside the solver also moved:
prereg:   Cargo.lock
prereg:   crates/pistol-core/src/window.rs
prereg: H1-b CONFIRMED — every behaviour line is byte-identical
```

**H1-b now confirms** where revision 7 declared N/A, and the two paths it names are demoted from
a gate to a note. The `lock delta: 1 file changed, 3 deletions(-)` line is the counterfactual's
removal of the solver's `pistol-core` edge — exactly the three lines of `Cargo.lock` that made
the old conditional unreachable, now printed rather than tripped over (m-2).

**B-2's REPLICATION: three independent processes, byte-identical.**

```
run 1: exit=0 wall=43s  md5=9f7e68c1c137277bf020defb99c5994d
run 2: exit=0 wall=45s  md5=9f7e68c1c137277bf020defb99c5994d
run 3: exit=0 wall=42s  md5=9f7e68c1c137277bf020defb99c5994d
distinct outputs: 1     lines each: 12
```

**EVERY PATH, RUN — which is what B-1 asked for instead of a restated disjointness claim.**

| path | before the fix | after |
|---|---|---|
| step 6, `n = 0` (nothing matches) — *revision 8's numbering; the count is no longer adjudicated on (§2.1)* | **exit 1, stdout empty, stderr empty** → adjudicated ABORT | **exit 2**, `0 Cargo.toml files mention pistol-solver, expected exactly 1` |
| step 6, `n = 2` (real edge in `pistol-cli`) | exit 2 | **exit 2**, `2 Cargo.toml files … 2 or more means an edge` |
| `sha256sum` on an empty path | **exit 1**, unrouted | **exit 2**, `candidate build produced no binary` |
| H1-a fails (solver reaches codegen) | — | **exit 1**, `H1-a FAILED`, `570dc5d8…` vs `1ed322ea…` |
| happy path | exit 0 | **exit 0**, ×3 identical |
| non-repository cwd | exit **128** from `git rev-parse` | **exit 2**, `not inside a git repository` |

**Two defects I introduced while fixing these, both caught by running and neither by reading:**
a path assertion that ignored `git grep -l`'s `<rev>:` prefix, and `rustc -vV | head -1` taking
**SIGPIPE** under `pipefail` — the `cmd | head` trap, which is `SHELL_CHECKLIST`'s own class,
introduced *in the commit fixing a `SHELL_CHECKLIST` finding*. Both refused at exit 2 with a
named message rather than passing, which is the routing working; both are fixed.

**So the disjointness claim is now a measurement rather than an assertion:** `0` CONFIRMED,
`1` ABORT, `2` RUN VOID, and no path observed producing anything else.

**Exit codes are now disjoint and match §3.2**: `0` CONFIRMED, `1` ABORT, `2` RUN VOID, with
every refusal routed through one `refuse()` helper. Revision 4's `${LANDING:?…}` exited **1**
and collided with ABORT; measured before the fix, and gone after it.

### 7.1 A FINDING ABOUT THE DRY-RUN RULE ITSELF — from DR-6's failure

Revision 2 registered **DR-6** on the whole-state instrument. It used a **real instance of the
kind** — the actual pinned corpus — and it **PASSED**. The instrument it certified was
nonetheless reading a different game: it coloured plies `i % 2` instead of rule 3's turn
structure, mis-colouring 7 of 15 stones on corpus position 0 and agreeing with the true board
on **0 of 24 positions**.

**DR-6 could not have caught this, because its registered check was that all three backends
agree on a checksum — and all three shared the same wrong board.** Internal agreement is
invariant under a defect in a shared input.

**The statute says a real instance of the kind exercises ATTRIBUTION where a synthetic
stand-in exercises only syntax. That is necessary and not sufficient.** DR-6 satisfied the
letter and exercised neither: a dry run must check the instrument against **something outside
the instrument** — here, the board `GameState::from_plies` builds, which this session's own
§5.2 reference used correctly and which the harness never consulted.

**THE SHARPENED FORM, which is what should land.** The operative object is not "the check"
but **the registered SHOW-criterion**:

> **A pre-registration that states what a run must SHOW also NAMES THE DEFECT CLASS that
> criterion is meant to exclude** — without one the clause below cannot be applied by a
> reviewer, or by its author. A registered criterion passes vacuously whenever it is a property
> that defect class **preserves**: internal agreement between components sharing an input,
> output shape, plausible magnitude, and exit status are all such properties. **The criterion
> must therefore be one the named defect could falsify.** An externally derived referent — a
> value computed by something that does not share the suspect input — is the operationalisation
> that reliably achieves this, and is what a reviewer should look for first; it is sufficient,
> not necessary, and a criterion falsifiable by other means satisfies the rule. **This applies
> to any registered criterion, dry-run or governed alike.**

Rule first, ground second — *a real instance of the kind is necessary and not sufficient* —
with the ground (*internal agreement is invariant under a defect in a shared input*) as its
instance, not its statement. Stated this way the rule is falsifiable by construction. **A THIRD INSTANCE, from the fix round, and it is the same class rather than a new one.** The
golden threat fixture is now *derived* — regenerated from the from-scratch reference — and a
regeneration test asserts the committed file matches. **That test cannot catch a drifted ply
list**, because the ply list is an **input to the derivation**: a wrong position yields a
self-consistent fixture that regenerates identically. The check is true, passes, and constrains
nothing about the question it is asked. That is exactly *"internal agreement is invariant under
a defect in a shared input"* with the shared input being the ply list, so the clause covers it
without extension — **a derived artifact protects its ANSWERS while its QUESTION stays
hand-maintained**, and the defect class the criterion must name is "the question drifted", not
"the answers drifted".

**And the operationalisation follows from the rule rather than needing a new one.** The
externally derived referent for a fixture's *question* is its **purpose**: RULE 1's named
mutant. A ply list that drifts stops killing the mutant its row registers, and the mutation run
is independent of the fixture file entirely. So RULE 1 is the external referent for fixture
questions exactly as RULE 2 is for command blocks, and the two rules are one rule applied to
the two kinds of artifact a pre-registration owns. **This is stated as a consequence, not as a
new clause**: extending the CLAUDE.md text to legislate fixture derivation would over-reach
past what the finding supports.

**Three craft defects in revision 4's
wording are fixed above**: it asserted that falsifiability *means* an externally derived
referent, where externality is a sufficient operationalisation and not a necessary one; it never
required the pre-registration to **name the defect class**, leaving "a property the defect class
preserves" uncheckable; and it was scoped to dry runs.

**AND IT IS NOT SCOPED TO DRY RUNS — because this document's own GOVERNED criterion failed the
same test (M-4).** Revision 4's §3 registered four revision assertions as its SHOW-criterion,
and a reviewer ran the composed block at HEAD with every guard passing: candidate `72316a7`,
baseline `050961d`, eight commits apart, **diff EMPTY, CONFIRMED** — on a revision containing
no WP-1.5a code at all. The assertions closed P-2's enumerated sub-cases and did not close
*"any revision whose binary matches"*, which with `p = 0` is **every** revision, because the
WP-1.5a binary is identical to W3's **by construction** and that is the same observable as the
work package never having landed. A rule that would not have caught §3 is not the rule this
finding supports, so the clause reads **"a registered criterion"**, dry-run or governed
alike. §3's fix — `git diff --stat "$BASE_REV" "$LANDING" -- crates/pistol-solver` must be
non-empty — is externally derived, costs one line, and is exercised as ARM A in §7.2.

**T2 IS HIT, AND IS DISPOSED OF HERE (P-5).** D-235's T2 reads: *"'records the dry-run input
and its output' requires RECORDING and not CHECKING."* **DR-6 is exactly that instance** — a
compliant dry run, on a real instance of the kind, input and output recorded, passing while
certifying an instrument that mis-coloured 7 of 15 stones on every corpus position. Revision 3
stated T2's content almost word for word and never named it. Under D-242 the first session to
hit one resolves it then, citing the T-number; this is that citation.

**And it partially rebuts D-245's disposition of T2 as "OVER-satisfied".** D-245 held that
stating in advance what a dry run must SHOW is stricter than the statute. DR-6 **did** register
what it must show — three backends agreeing on a whole-state checksum — and it bought nothing,
**because the registered criterion was one the defect preserved**. So registering a
SHOW-criterion is not automatically stronger than recording; it is stronger only when the
criterion is externally derived. That is a limit on D-245's own reasoning and is recorded as
one.

**THIS AMENDS CLAUDE.md, NOT ONLY THE LOG (P-6).** Revision 3 sent this to the decision log
alone while sending T1's much narrower ruling to CLAUDE.md — applying D-228's reason (*"a rule
whose limits live only in a decision log is one the next reader applies without them"*) to the
lesser finding and withholding it from the greater. **The reason applies here with more force**:
this is a limit on CLAUDE.md's dry-run paragraph itself, and it contradicts that paragraph's
own emphasis, since CLAUDE.md says only a real instance exercises attribution and DR-6 is a
real instance that exercised neither. So the proposal is a clause in CLAUDE.md's Process
section beside the dry-run paragraph, landing in the same commit as its ADR line, per D-245's
worked precedent.

---

### 7.3 REVISION 9's DRY RUNS — DR-8, DR-9, and every refusal the repairs touch

**Under D-269, which this document's own §7.1 finding produced and which landed in CLAUDE.md
before these runs, a dry run states what it must SHOW *and names the defect class the criterion
excludes*.** So:

> **DEFECT CLASS UNDER TEST: a routing repair that breaks a path it did not intend to touch** —
> a refusal that no longer fires, a verdict that no longer reaches its exit code, or a guard
> that now refuses the happy path. Revision 9 changed four routing sites and added a `cd`; each
> is a place where a fix can silently disable a check.
>
> **CRITERION: the exit status and the named message of every path, against the table §3.2
> registers.** §7.1 warns that exit status is normally a property a defect class PRESERVES and
> is therefore a vacuous criterion — that warning does not apply here and the exception is worth
> stating, because **the defect class under test IS the routing, and an exit status is what
> routing produces.** This is the one class for which the status is the observable rather than
> an accident of it. The happy-path arms carry an externally derived referent besides: H1-a's
> comparand is a binary built by cargo, not a value read from a file.

**INPUT: a NON-REGISTERED pair of the same kind.** The registered workload is the baseline
record at `cdbcbf0` against `LANDING` `8618012`. Both dry runs use a different record and a
different landing revision of this same repository — the same sort of artefact, differing only
in identity — and neither touches the registered pair.

**DR-8 — the governed shape on `(5fdbf52, 60b5c44)`.** Baseline record taken in a throwaway
clone at `5fdbf52` (binary `ff018398…`, record digest `e3c36e13…`, sidecar `2429a1e1…`), block
extracted from this document by script and run at those bindings. **exit 0, 79 s.** Output:

```
prereg: baseline rebuild attests ff018398a88673c6929efe875768f299358acf22b112ecfa1273e5a845e427ef
60b5c44b4c18eb65c20c952223ef367aa209d1a7:crates/pistol-solver/Cargo.toml:2:name = "pistol-solver"
prereg: H1-a with solver    a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a without solver a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a counterfactual lock delta: none
prereg: H1-a CONFIRMED — the solver contributes nothing to the shipped binary
prereg: toolchain matches baseline: yes
prereg: H1-b comparing 54 behaviour lines (revision and binary_sha256 excluded)
prereg: note — build-reaching paths outside the solver also moved:
prereg:   crates/pistol-core/src/window.rs
prereg: H1-b CONFIRMED — every behaviour line is byte-identical
```

**AND DR-8 EXPOSED A LIMIT IN ITSELF, WHICH IS WHY DR-9 EXISTS.** `lock delta: none`, where the
registered pair prints `1 file changed, 3 deletions(-)`. The cause is a real difference in kind:
`5fdbf52` ALREADY CARRIES the solver's `pistol-core` dependency edge, so H1-a's counterfactual
*reverts the crate's content* rather than *removing it from the dependency graph* — the
`--locked`-dropped rationale and the lock-delta assertion are exercised only in their trivial
form. A dry run whose stand-in silently differs from the workload in the mechanism under test is
§7.1's own finding wearing a new coat, so it is recorded rather than glossed.

**DR-9 — the same block on `(72316a7, 60b5c44)`, where the counterfactual really removes the
crate.** `72316a7` predates the W3 commit and carries the solver only as its doc-only stub with
no `pistol-core` edge, so the counterfactual drops that edge exactly as the registered pair
does. Baseline record taken there, binary `62c102cc…`. **exit 0, 82 s.**

```
prereg: baseline rebuild attests 62c102cc30e017d16e282efe3e9f307397b9704f34b504a07d3cc4b427574e6c
prereg: H1-a with solver    a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a without solver a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a counterfactual lock delta: 1 file changed, 3 deletions(-)
prereg: H1-a CONFIRMED — the solver contributes nothing to the shipped binary
prereg: toolchain matches baseline: yes
prereg: H1-b comparing 54 behaviour lines (revision and binary_sha256 excluded)
prereg: H1-b CONFIRMED — every behaviour line is byte-identical
```

**THE REFUSAL ARMS, each RUN, 36 s for the set.** Every row §3.2 gained at revision 9 has a
producer here, which is M-2's standard applied to the rows this revision added:

| arm | input | result |
|---|---|---|
| C | `LANDING` a DESCENDANT of `HEAD` (`8618012` in a tree at `60b5c44`) | **exit 2**, `LANDING 86180124… is not an ancestor of HEAD — this tree is not a continuation of the landing revision` |
| D | `LANDING` = `5178ad9`, so `crates/` moved between it and `HEAD` | **exit 2**, `build-reaching or instrument paths moved between LANDING and HEAD: crates/pistol-solver/src/cover.rs …` |
| E | an untracked file at `docs/ZZZ_stray_probe.md` | **exit 2**, `untracked files outside the build-reaching set: ?? docs/ZZZ_stray_probe.md` |
| F | an untracked file at `crates/ZZZ_stray_probe.rs` | **exit 2**, `untracked files on build-reaching paths: crates/ZZZ_stray_probe.rs` |
| G | a TRACKED file modified (`Cargo.toml`) | **exit 2**, `tracked files are modified:  M Cargo.toml` |
| H1 | the solver-diff guard, `LANDING` a revision git cannot resolve — **run STANDALONE, see the correction below** | **exit 2**, `cannot diff crates/pistol-solver between … — the guard did not answer, which is not the same as answering that the diff is empty` |
| H2 | the solver-diff guard, a genuinely empty diff (`95ff602`→`2ac4286`) | **exit 2**, `… changes nothing under crates/pistol-solver: with p = 0 an empty diff is also what 'the WP never landed' looks like` |
| H3 | the solver-diff guard, the happy path | **exit 0** |
| I | the whole block invoked from `crates/`, clean tree | **exit 0** — the `cd "$REPO"` fix; before it the block could not even find `tools/` |
| J | **the killer**: a stray at `crates/ZZZ_stray_probe.rs`, block invoked FROM `crates/` | the old cwd-relative enumeration answers `[]`; the revision-9 block **exits 2**, `untracked files on build-reaching paths: crates/ZZZ_stray_probe.rs` |

**H1 IS THE HEADLINE DEFECT'S BEFORE AND AFTER.** Revision 8's construction on the same input
printed `fatal: bad object deadbeef…` to stderr and then announced **"changes nothing under
crates/pistol-solver"** — its own conclusion, asserted about an invocation that never answered.
Revision 9 names the reason instead. **J is the cwd defect's before and after** and it is the
one that would have passed silently: the enumeration returns EMPTY from a subdirectory with a
stray file sitting on a build-reaching path.

**CORRECTION AT REVISION 10 — ARM H WAS RETYPED, NOT EXTRACTED, AND §0.3 SAYS THE OPPOSITE.** ARMs
C-G and I-J invoke the extracted block; ARM H re-typed the guard's two lines into a standalone
subshell, while §0.3 states that *"§7.2/§7.3 extract it BY SCRIPT rather than retyping it, so the
thing run is the thing reviewed"* — and ARM H is the arm certifying the repair this revision's
header calls the single guard between "the WP landed" and "the WP never landed". A reviewer found
it. ARM H2 could not have gone through the block in any case: `BASE_REV=95ff602` has no baseline
record, so step 2 refuses first. **§7.5's arms all go through the extracted block**, and the
`… changes nothing under crates/pistol-solver` refusal keeps its §3.2 row on ARM H2's standalone
evidence with that provenance stated rather than implied.

**A DEFECT I INTRODUCED IN THE ARMS THEMSELVES, recorded because RULE 2's whole point is that
running finds what reading does not.** My first ARM G appended to `README.md` — a file this
repository does not track — so it created an UNTRACKED file and silently re-ran ARM E, and the
"tracked files are modified" path had no producer while the table said it did. Worse, `git
checkout -- .` does not remove an untracked file, so the leak survived into ARM I and made a
`cd`-fix arm report exit 2. Both were re-run on a clean tree with a genuinely tracked file. **A
refusal arm that fires for the wrong reason is indistinguishable from one that fires for the
right one, unless the message is read**, which is why every row above carries its message and
not only its status.

### 7.4 ONE READING RECORDED AS NOT REPRODUCING, WITH ITS REPRODUCER

The 141/SIGPIPE half of the solver-diff finding — a producer whose reader has already exited
takes `SIGPIPE`, `pipefail` propagates it, and `|| refuse` fires on a NON-EMPTY result — is
**not reproducible at any scale this workload can reach**, and is recorded as rejected rather
than quietly folded into the fix it partly motivated.

- **Attempted at the largest `--stat` this repository can produce**: the whole-history diff,
  **287 LINES — 286 files plus `--stat`'s summary line** — 17 879 bytes, well inside a 64 KiB pipe
  buffer. `git diff --stat … | grep -q .` under `set -euo pipefail`, **20 trials, 20 × exit 0.** No
  SIGPIPE. *(Revision 9 wrote "287 files"; 287 was the line count. The byte figure was right, both
  are far inside the buffer, and the conclusion is unaffected — but a count described as the wrong
  noun is D-221's class in miniature, and a reviewer reading it against the repository got a
  different number and was right to.)*
- **Control, to show the construction IS defective in kind rather than the probe being wrong**:
  the same construction with a producer that must write past the buffer (`git log -p`, 4 654 354
  bytes), **20 trials, 20 × exit 141** — which under `|| refuse` is a refusal on a non-empty
  result, the EXIT-nonzero-WRONG-ANSWER shape.

So the guard was bounded by an accident of output size and not by anything the document
registered, which is a reason to remove the construction and not a reason to claim it bit. The
exit-128 conflation (§7.3 arm H1) is the half that reproduces, and it is what the repair is
justified by.

### 7.5 REVISION 10's DRY RUNS AND ARMS — the repairs, exercised

**DEFECT CLASS UNDER TEST (D-269): a repair that breaks a path it did not intend to touch**, and
— new at revision 10 — **an adjudicator that answers a different question than the one registered**,
which is what the substring count was doing. **CRITERION: the exit status and the named message of
every path, against §3.2's table**, plus, for the two arms that replaced adjudicators, a
demonstration that the OLD instrument and the NEW one give DIFFERENT answers on the same input —
which is a criterion the defect class cannot preserve, because if both instruments agreed
everywhere the replacement would be a no-op.

**INPUT: non-registered pairs of the same kind**, in a throwaway clone. The registered workload is
`(cdbcbf0, 8618012)`; nothing below touches it.

**DR-10a — the governed shape on `(5fdbf52, 60b5c44)`. exit 0, 43 s.** All eight bindings printed,
the graph probe answering, the toolchain in the transcript:

```
prereg: binding REPO          …/dryrun/repo
prereg: binding BASE_REV      5fdbf52510d7f17303175d2ac34bc635b26cf7ae
prereg: binding LANDING       60b5c44b4c18eb65c20c952223ef367aa209d1a7
prereg: binding SNAP_BLOB_PIN a7c0ed4367a6893f70b776732bd68ada77c19483
prereg: baseline rebuild attests ff018398a88673c6929efe875768f299358acf22b112ecfa1273e5a845e427ef
prereg: p = 0 — pistol-solver is absent from pistol-cli's resolved normal-edge graph
60b5c44b4c18eb65c20c952223ef367aa209d1a7:crates/pistol-solver/Cargo.toml:2:name = "pistol-solver"
prereg: H1-a with solver    a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a without solver a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
prereg: H1-a counterfactual lock delta: none
prereg: H1-a CONFIRMED — the solver contributes nothing to the shipped binary
prereg: toolchain candidate rustc 1.97.1 (8bab26f4f 2026-07-14)
prereg: toolchain candidate cargo 1.97.1 (c980f4866 2026-06-30)
prereg: toolchain matches baseline: yes
prereg: H1-b comparing 54 behaviour lines (revision and binary_sha256 excluded)
prereg: H1-b CONFIRMED — every behaviour line is byte-identical
```

**THE ARMS, each RUN, and each with its BEFORE:**

| arm | input | revision 9 | revision 10 |
|---|---|---|---|
| **K** | a mutated baseline behaviour line, ×3 processes | exit 1 ×3, **THREE distinct stdouts**, no named refusal | exit 1 ×3, **ONE distinct stdout**, `prereg: H1-b FAILED — a behaviour line moved between …`, header `--- baseline` / `+++ candidate` |
| **L** | `LANDING=dev` | **ACCEPTED** — passed the ancestor test, the drift diff and the attestation, recording `dev:` where a revision belongs | **exit 2**, `LANDING is not a 40-hex object name: dev — a branch or tag name satisfies every git call below` |
| **M** | an INCOMPLETE baseline record | **exit 1 = ABORT**, where §3.2 registers RUN VOID | **exit 2**, `… is not a COMPLETE baseline_snapshot record: baseline_snapshot_incomplete 1` |
| **N** | `assume-unchanged` + a worktree edit to the instrument | **passes** — `git status --porcelain` is EMPTY and the `LANDING..HEAD` diff is EMPTY | **exit 2**, `the snapshot instrument on disk hashes to 42463e43…, not the registered a7c0ed43…` |
| **O** | a newline inside `BASE` | printed nowhere, so unguarded | **exit 2**, `a path binding contains a non-printable character and would inject lines into the record it is printed into` |
| **P** | a `# … pistol-solver …` COMMENT in `crates/pistol-engine/Cargo.toml` | **exit 2**, `2 Cargo.toml files … 2 or more means an edge` — a RUN VOID naming an edge that does not exist | **exit 0**, CONFIRMED; the comment is noted under "build-reaching paths outside the solver also moved" and adjudicates nothing |
| **Q** | a real `[dependencies."pistol-solve\u0072"]` edge, lock re-resolved | the count returns **1** — INVISIBLE; the block proceeded and H1-a caught it at exit 1, in a bucket §3.2 said was exit 2 | **exit 1**, `p = 0 REFUTED — pistol-solver IS in pistol-cli's resolved normal-edge graph; run 'cargo tree …' for the reverse-dependency path`, before any build |

**ARMS P AND Q ARE THE CRITERION THE DEFECT CLASS CANNOT PRESERVE.** On the same two inputs the old
adjudicator and the new one give *opposite* answers in *both* directions: P is a false refusal the
count produced and the graph does not; Q is a real edge the graph catches and the count cannot see.
At Q's revision the substring attestation returns exactly one file — the solver's own manifest —
while `cargo tree -i` returns

```
pistol-solver v0.0.1 (…/crates/pistol-solver)
└── pistol-cli v0.0.1 (…/crates/pistol-cli)      exit 0
```

**ONE ARM MISMATCHED FIRST AND THE REASON IS RECORDED, because it is a finding about the harness
rather than about the block.** My first ARM Q added the manifest edge WITHOUT re-resolving
`Cargo.lock`, and the block exited **2** with `cargo could not resolve pistol-cli's normal-edge
graph … the p = 0 attestation was not taken` — `--locked` refusing a manifest the lockfile does not
reflect. That is the block behaving correctly and the arm being wrong: *"cargo could not answer"*
and *"there is no edge"* are two reasons with two refusals, exactly as §2.2 registers, and the arm
had constructed the first while claiming to test the second. Re-run with the lock re-resolved, it
gives exit 1. **A refusal arm that fires for the wrong reason is indistinguishable from one that
fires for the right one unless the message is read** — §7.3's own sentence, earning its place a
second time.

## 8. What this document does NOT license

- **No strength claim.** Rule 6's judge is SPRT; this WP integrates with no search.
- **No storage claim of any kind.** H2 is withdrawn; DESIGN.md §5.4's `k` figures are
  **recorded design input**, not a registered hypothesis, and are known to come from
  instruments with named defects.
- **No eval-side change.** That is WP-1.9 (D-249).
- **No whole-engine perf claim in either direction.** A confirmed H1 is evidence of absence of
  effect, not of gain.
- **No conclusion about `p'`.**
- The dry runs consume no governed sample.

## 9. The reading this document cannot defend against

Neither the proportionality rule nor the dry-run rule is mechanized, and neither catches a run
whose answer is already known before it is taken. **H1's answer is largely known:** `p = 0` is
a dependency fact and a binary built from a crate nothing links is the same binary.

**Revision 3 states this more plainly than its predecessors could, because with H2 gone this
document registers nothing whose answer is in doubt.** H1 is kept because the one thing it
cannot predict is whether IMPL accidentally creates the edge — that is the failure it exists
to catch, §3.1 is what makes it able to catch it, and it costs two minutes. A reviewer who
thinks a hypothesis with a known answer should not be registered at all has a fair point; the
counter is that H1 is cheap, mechanical, and the only thing standing between an accidental
`Cargo.toml` edge and a silent `p > 0`.
