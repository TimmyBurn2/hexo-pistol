# WP-1.5a — OBSERVATION RECORD (H1 RETIRED)

**REVISION 12. Supersedes revisions 1-11. This document registers no hypothesis, no
threshold and no governed run, and adjudicates nothing.** Revisions 1-11 pre-registered H1 —
that WP-1.5a adds nothing a shipped binary can observe, because `p = 0` — with a two-build
counterfactual (H1-a), an invariant-block comparison (H1-b) and a second-instrument agreement
criterion. **The operator retired the adjudicating version on 2026-08-21 under D-276**, on an
OPTION MATRIX attacked by a fresh-context DECISION-RED-TEAM. What remains is this record and
two standing CI gates that check the claim on every commit instead of once.

Registered across D-249 and D-263; retired under **D-276**. The claim itself is not withdrawn —
it is now `crates/pistol-cli/tests/solver_link_check_tests.rs` and
`crates/pistol-cli/tests/solver_edge_check_tests.rs`.

## 0. Why the adjudicating version was retired

Six fresh contexts across two sessions — three governing-revision reviews, three red-teams and
one decision red-team — and **every one found a new defect class**. Revisions 7, 8, 9, 10 and 11
each fixed a defect and shipped another; two of those were introduced by the fix, in the same
commit. The findings below are recorded here rather than in the ADR line alone, because the next
session to reach for an instrument of this shape will meet all of them again.

**0.1 THE PRIMARY HYPOTHESIS WAS NOT FALSIFIABLE ACROSS PART OF ITS OWN REGISTERED DEFECT
CLASS.** §3 named H1-a's class as *"solver content that reaches codegen by ANY route — a
dependency edge, an `include_str!`, a build script, a path that cargo auto-discovers"*. Measured
by the decision red-team: a build script that reads the subject without declaring
`rerun-if-changed`, with both builds sharing one target directory — **which is how
`tools/wp15a_h1.sh` ran them** — leaves a stale `OUT_DIR` artefact, so the second build compiles
nothing and two identical digests are reported for a binary whose behaviour moved (`22` → `77`
with the digest unchanged; a fresh target directory catches it). **Exit 0, CONFIRMED.** The edge
check simultaneously reported no edge, so the registered agreement criterion was SATISFIED while
both instruments were wrong together — which is T8's bite, instantiated on this document's own
registered pair, and under D-242's boundary that is the EXCLUDED class rather than bucket
material.

**0.2 THE AGREEMENT CRITERION WAS UNSOUND THREE TIMES, THE THIRD TIME IN THE DIRECTION THAT
COSTS MOST.** Revision 9's biconditional was violated at the registered bindings (TRUE ⇔ FALSE).
Revision 10's replacement read `cargo tree -i`'s EXIT STATUS, which is not normal-edge
membership — a `[dev-dependencies]` entry exits 0 with an empty tree, so the route this document
elsewhere calls legitimate became a refutation. Revision 11's one-directional form rested on
*"a crate outside the resolved graph cannot reach codegen"*, which is FALSE: reproduced by two
independent contexts via `include_str!` with no manifest edge and via a build script. Its
consequence is the worst available: **a genuine refutation of H1 is reported as RUN VOID**, the
instrument blaming itself, on exactly the accidental-route case §9 kept H1 for.

**0.3 THE INSTRUMENT COULD NOT BE RUN AT ITS OWN REGISTERED BINDINGS, AND WOULD BREAK AGAIN BY
CONSTRUCTION.** The drift guard forbids movement under `crates` between `LANDING` and `HEAD`;
the commit that promoted the instrument added its test files there. Measured: `REGISTERED RUN
EXIT: 2`. Every future instrument fix lands in the same place, so option "repair once more" was
never one round — it was one round per fix, forever, unless the pathspec grew an exclusion
maintained by memory, which is D-275's own lesson.

**0.4 THE TEST SUITE BOUND FAR LESS THAN THE DOCUMENT CLAIMED.** Two reviewers, independently:
11 of 16 and 31 of 40 mutations survived. Survivors included BOTH assertions §3.1a says replace
the dropped `--locked`, the pristine-rebuild attestation and the `BASE_SHA` digest binding. The
promotion to `tools/` was still worth it — the suite caught two defects before any reviewer did
— but it was not the guarantee this document asserted.

**0.5 THE DOCUMENT DRIFTED FROM ITS OWN INSTRUMENT, REPEATEDLY.** A `--locked` guard the block
did not contain; a provenance file the trap deleted; section numbers off by one against five
prose references; a comment saying SEVEN bindings beside a block printing eight; and an
instrument pinned at the blob of the PARENT commit — the version whose verdict ordering the same
revision was written to replace. Caught before any governed run, so no verdict inherits it
(D-275).

**0.6 AND THE ANSWER WAS ALWAYS LARGELY KNOWN.** §9 said so from revision 3: *"`p = 0` is a
dependency fact and a binary built from a crate nothing links is the same binary."* It kept H1
for one thing — an accidentally created edge — and 0.1 and 0.2 are that one thing, mis-adjudicated
in both directions.

## 0a. WHAT SURVIVES AS OBSERVATION

**These are measurements, not licences. Nothing below adjudicates anything.**

- **The controlled repetition, which exists nowhere else.** `7b9e904` and `8618012` both rebuild
  to `a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce` in a pristine clone,
  while `crates/pistol-solver` moved by **232 insertions and 28 deletions across 6 files**
  between them (`cdbcbf0..7b9e904` is 4421 insertions under that path; `cdbcbf0..8618012` is
  4625). The shipped binary was invariant not merely under removing the solver at one revision
  but under substantively rewriting it across seven commits. Its binding limit is recorded with
  it: `grep -c 'Compiling pistol-solver'` over the run's own stderr is **0** — cargo never
  compiles the crate — so this is corroboration and not evidence about the rewrite.
- **The baseline provenance** (§0.1): the W3 record at `cdbcbf05…`, 5764 bytes, digest
  `7faa074c…`, whose `binary_sha256 ff018398…` reproduces from a pristine checkout; the sidecar,
  892 bytes, digest `8be24055…`, `rustc 1.97.1` / LLVM 22.1.6. Both live in
  `~/Work/pistol-wp15a/`, outside the repository (rule 8).
- **The W3 digest is not reproducible across independent migrations** — this session's
  `daf1deb3…`, a reviewer's `691f7766…`, the landed `ff018398…`, all search-identical to stock.
  Only the stock side is ever pinned in advance (D-253).
- **`binary_sha256` is insensitive to dead code even in a linked crate**: an unreferenced
  `pub const` appended to `pistol-core` did not move the digest. Any successor reaching for a
  digest comparison meets this first.
- **The environment finding** (§2.4), now D-265, and **the dry-run finding** (§7.1), now D-269 and
  a clause in CLAUDE.md. Both outlived the document that produced them.

## 0b. WHAT REPLACES IT

| claim | where it lives now | cost |
|---|---|---|
| no crate in this workspace takes a normal dependency on the solver | `solver_edge_check_tests.rs`, on cargo's resolved graph, workspace-wide | 0.2 s, every commit |
| **no solver source is an input to any of the five shipped binaries** | `solver_link_check_tests.rs`, from rustc's dep-info | 0.6 s, every commit |

The second is strictly stronger than H1-a and covers what H1-a could not: it sees `include!`,
`include_str!` and `include_bytes!` with no manifest edge, it covers **five** binaries where
H1-a compared one of five, it **names the offending file** rather than printing two hexes, and
it refuses to answer at all when the workspace grows a build script — the blind spot 0.1 is
about, declared rather than hidden.

**What is genuinely lost, stated plainly.** A governed run would have proved the property held
**at a named revision, with a recorded transcript, under a document reviewed in advance**. A CI
gate proves it holds **now, on every commit**, and carries no transcript. That is a real
difference and the ground for accepting it is narrow: H1 was never a strength claim — §8
de-licenses every one — it is an evidence-of-absence claim, and rule 6's judge does not reach it.

---

**THE INSTRUMENT IS REMOVED.** `tools/wp15a_h1.sh` and its suite were deleted with this
retirement: a script CI still tests, for a hypothesis nothing claims, is drift under rule 10, and
this one carries the measured false-CONFIRMED of §0.1. It is in the history at `70103ec^`.
`tools/solver_edge_check.sh` STAYS — its blind spots are disjoint from the link gate's (§0b), and
both are kept for that reason. The instrument-blob table in the retained text below is therefore
HISTORICAL and pins nothing.

**Everything below this line is the retired revision 11, kept unedited as the record of what was
registered and why the findings above are findings. It licenses nothing.**

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

### 0.1 THE THIRTEEN BINDINGS (revision 11)

Every value verified this session rather than transcribed. The instrument takes all of them
from the environment and supplies **no defaults**: a missing binding is a named refusal, never a
fallback (CLAUDE.md rule 1, rule 3), and a test asserts it.

```sh
BASE=/home/tom/Work/pistol-wp15a/baseline_cdbcbf0.txt
BASE_SHA=7faa074c21a2d7d28e4ca681e05ed95942436d639642c088a73032febd33159a
BASE_REV=cdbcbf05bd9d792ac7a6af709970f11b95796b81
BASE_TC=/home/tom/Work/pistol-wp15a/baseline_cdbcbf0.toolchain.txt
BASE_TC_SHA=8be240559c27b2b163347cba8f2266f7877ceee0ef8d72095e2d73537a6adc2a
LANDING=861801247df5c1a73480b5153e11c399aa752750
REPO=/home/tom/Projects/HeXO-AlphaBeta
SUBJECT_CRATE=pistol-solver
SUBJECT_PATH=crates/pistol-solver
BUILD_PKG=pistol-cli
BUILD_BIN=pistol
SNAPSHOT_REL=tools/baseline_snapshot.sh
EDGE_CHECK=/home/tom/Projects/HeXO-AlphaBeta/tools/solver_edge_check.sh
```

| variable | source, verified |
|---|---|
| `BASE` | the W3 baseline record, 5764 bytes, at a DURABLE path; digest recomputed and matching |
| `BASE_SHA` | recomputed `7faa074c…` |
| `BASE_REV` | the record's own `revision` line; its `binary_sha256 ff018398…` reproduces from a pristine checkout. 40-hex spelling and `rev-parse` identity both checked |
| `BASE_TC` | the sidecar, 892 bytes: `rustc 1.97.1`, `cargo 1.97.1`, LLVM 22.1.6 |
| `BASE_TC_SHA` | recomputed `8be24055…` |
| `LANDING` | `86180124…`, seventeen commits after `BASE_REV`. **Not required to equal `HEAD`.** 40-hex spelling checked, because a BRANCH NAME satisfied every git call in revision 9 — measured: `LANDING=dev` passed the ancestor test, the drift diff and the attestation |
| `REPO` | the repository root. Registered at revision 10 after a red-team control run **in a clone** produced stdout byte-identical to a run in the live tree, so a transcript could not be attributed to its bindings. Printed by the run, guarded as an allow-list |
| `SUBJECT_CRATE` / `SUBJECT_PATH` | the crate under test and its path. Arguments rather than literals, so the tests can drive the shipped instrument against workspaces of their own |
| `BUILD_PKG` / `BUILD_BIN` | the package and binary H1-a compares. **This is a NARROWNESS the document states rather than hides**: `pistol-cli` ships three binaries and `pistol-arena` two, and H1-a compares one of the five. `p = 0` is what covers the rest, and it is workspace-wide |
| `SNAPSHOT_REL` | repository-RELATIVE, resolved against the **pristine clone**. `baseline_snapshot.sh` roots itself at `dirname($0)/..`, so an absolute path to the working tree's copy would make it read the working tree's config and corpus wherever it was invoked from — the gap that reached a verdict at revision 10 |
| `EDGE_CHECK` | absolute, and deliberately the WORKING TREE's copy: the adjudicator does not exist at `LANDING`, it takes its subject as an argument rather than rooting itself, and it is pinned by blob in the table above |

**The two baseline artifacts live in `~/Work/pistol-wp15a/`** — outside the repository (rule 8),
which is WP-1.3's own idiom (`~/Work/pistol-wp13/*.matchlog`). Revision 9 had copied them from
one session-scoped `/tmp` scratchpad to another, which a reviewer correctly called the same
property with a different UUID. **A path is a locator and the digest is the binding**: had a
copy differed by a byte, the instrument refuses before H1 asks anything, and a test asserts it.

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

### 0.3 THE INSTRUMENTS, NAMED WITH THEIR REVISIONS (D-268) — AND NOW TESTED (D-272)

D-268 requires that an artefact producing a registered number be named in the pre-registration
WITH ITS REVISION, and that a change to it reopen the review exactly as an amendment to the
document does. The blobs are in the header's table. **What is new at revision 11 is that three
of the four are now driven by tests in a suite CI runs**, which is `SHELL_CHECKLIST` item 10's
rule and the one thing four rounds of review could not substitute for:

| instrument | tests | where |
|---|---|---|
| `tools/solver_edge_check.sh` | 9, including a CONTROL and a standing `p = 0` invariant over the real workspace | `crates/pistol-cli/tests/solver_edge_check_tests.rs` |
| `tools/wp15a_h1.sh` | 15, including a CONTROL that reaches CONFIRMED and regressions for all three revision-10 blockers | `crates/pistol-cli/tests/wp15a_h1_tests.rs` |
| `tools/baseline_snapshot.sh` | its own suite (D-230's) | `crates/pistol-cli/tests/baseline_snapshot_tests.rs` |
| the baseline record + sidecar | not a script: pinned by digest, §0.1 | — |

**THE CONTROL IS THE POINT AND IT IS NAMED HERE** so a reviewer can check it exists rather than
taking the count on trust: `a_clean_workspace_reaches_confirmed` and
`a_workspace_with_no_edge_at_all_is_accepted`. Item 10's named failure mode is a suite every
assertion of which is satisfied by a gate that refuses everything, and a count of tests is no
defence against it.

**THE MUTATION EVIDENCE**, taken in a separate worktree because a deliberate break in the tree
being edited is indistinguishable from a regression: re-adjudicating on `cargo tree -i`'s EXIT
STATUS — revision 10's defect verbatim — turns **four** tests red, including the
dev-dependency case and the standing invariant; narrowing `--workspace` to one member — revision
10's other defect — turns **three** red.

**ONE GAP REMAINS AND IS NAMED RATHER THAN CLOSED**: nothing pins the instrument on the
BASELINE side. The record at `BASE_REV` was taken by whatever `tools/baseline_snapshot.sh`
existed then. For this pair the gap is latent and not live —
`git diff --name-only cdbcbf0 8618012 -- tools` is empty, so one instrument took both sides —
and a future pair straddling a `tools/` change owes a second pin.

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
tools/solver_edge_check.sh <workspace-root> pistol-solver
```

pinned by blob in §0's instrument table, run by `tools/wp15a_h1.sh` against the
**pristine clone at `LANDING`**. It answers on the STDOUT of a workspace-wide inverted
normal-edge tree, and **never on an exit status**:

| stdout | reading |
|---|---|
| **one line** — the crate's own root line | no normal reverse-dependency anywhere in the workspace: `p = 0` |
| **more than one line** — the extra lines NAME the dependents | `p = 0` **REFUTED**, exit 1 = ABORT |

**WHY NOT THE EXIT STATUS, WHICH IS WHAT REVISION 10 REGISTERED AND WHAT A RED-TEAM BROKE.**
Measured three ways on this workspace, `cargo tree --locked --workspace --edges normal -i`:

```
no edge anywhere        exit 101   stdout empty      stderr: error: … did not match any packages
a normal edge           exit 0     stdout: <tree>    stderr empty
a [dev-dependencies]    exit 0     stdout empty      stderr: warning: nothing to print.
```

**Exit status conflates the real edge with the dev-dependency; stdout separates them exactly.**
Revision 10 discarded stdout with `>/dev/null` and read the status, so a legitimate
dev-dependency — the route the solver's own oracle would take from a test tree, which §3 relies
on NOT moving the digest — became a refutation of `p = 0`, with the refusal naming a diagnostic
command that prints `warning: nothing to print`. Status also collides three ways at 101: no such
package, an AMBIGUOUS specification, and any other cargo failure.

**`--workspace`, NOT `-p <one-member>`.** §2 and §5 both state the claim as *no binary in this
workspace*, and this workspace ships five binaries across two packages — `pistol-cli` has
`pistol`, `corpus-extract` and `random-openings`; `pistol-arena` has `arena` and
`arena-stub-engine`. Revision 10 probed one package, and a red-team drove a normal edge into
`pistol-arena` to **exit 0 CONFIRMED**, with the offending manifest line printed two lines above
the verdict.

**READABILITY IS A SEPARATE QUESTION WITH A SEPARATE REFUSAL** (`SHELL_CHECKLIST` item 8), and
so is "the crate is not in this workspace": one status cannot mean three things, so the script
asks all three and refuses each by name.

**A REFUTATION IS ABORT, NOT RUN VOID.** An accidental edge is not an instrument failure — the
instrument answered. It means `p` is no longer 0, which is §6's abort protocol exactly: the WP
does not land, the finding is recorded, and this document is **wrong rather than failed**.

**AND IT IS TESTED**, which no predecessor was: nine tests, a control, a standing `p = 0`
invariant over the real workspace, and mutation evidence that re-adjudicating on exit status
turns four of them red (§0.3).

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

### 3.1 The registered instrument

**There is no block here any more.** The instrument is `tools/wp15a_h1.sh`, which calls
`tools/solver_edge_check.sh`. **Both are pinned by blob in §0's instrument table and NOWHERE
ELSE**, as D-268 requires, and a change to either reopens this review exactly as an amendment to
this document does.

**The pin lives in one place because it was wrong in two.** Revision 11 registered
`tools/wp15a_h1.sh` at the blob of the PARENT commit — the version whose verdict ordering §6 of
that same revision was written to replace — and repeated the hex in this section, so the document
registered an instrument contradicting its own §6 table. Two reviewers found it independently.
Caught before any governed run: no results document exists and no ADR records an H1 verdict, so
no verdict inherits it. **A pin maintained by memory drifts exactly like a guard applied by
memory**, so `the_instrument_pins_in_this_document_match_the_shipped_scripts` now compares the
table against `git hash-object` on every commit, and the duplicate hexes are gone.

**The registered invocation**, with the thirteen bindings of §0.1 in the environment:

```sh
tools/wp15a_h1.sh
```

**Every refusal exits 2 through one `refuse()` helper; `0` is CONFIRMED, `1` is ABORT.** The
readings are §3.2's table, and each row there names the TEST that produces it — which is the
difference this revision makes. Through revision 10 each row named an arm a session had run by
hand once; now each names a test `cargo test --workspace --locked` runs on every commit.

**WHY THE DOCUMENT NO LONGER PRINTS ITS OWN INSTRUMENT.** Printing it was not neutral: a block
inside a pre-registration can be read and cannot be run, so every defect in four revisions was
found by an agent executing it by hand, and each fix shipped a new one (the header's table).
It also made the document claim things about itself that drifted — a `--locked` guard the block
did not contain, a provenance file the trap deleted, section numbers off by one against five
prose references, a comment saying SEVEN bindings beside a block that printed eight. **A script
cannot drift from itself.** What this document registers is the BINDINGS, the READINGS and the
CONSEQUENCES; what the instrument does is the instrument's, and the tests are what hold it to it.

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

### 3.2 Adjudication — every row names the TEST that produces it

Revision 7's table had five of eight rows with no reachable producer; revision 9 added one more;
revision 10 removed it. **The standard has moved:** a row used to name an arm a session ran by
hand once, and now names a test in `crates/pistol-cli/tests/` that CI runs on every commit. A
row whose test does not exist is a row this document may not carry.

| exit | Reading | Verdict | test that produces it |
|---|---|---|---|
| **0** | `p = 0` holds, H1-a's digests identical, H1-b's behaviour lines identical | **CONFIRMED** | `a_clean_workspace_reaches_confirmed`; and `a_dev_dependency_on_the_subject_still_confirms` |
| **1** | the workspace's inverted normal-edge tree names a dependent | **ABORT** — `p` is no longer 0, before any build is paid for | `a_normal_dependency_on_the_subject_aborts_before_any_build` |
| **1** | H1-a's digests differ | **ABORT** — the subject reached codegen by some route | (H1-a's own break test: `include_str!` from a called path, `570dc5d8…` vs `1ed322ea…`) |
| **1** | H1-b's behaviour lines differ | **ABORT**, named, and replication-stable | `a_moved_behaviour_line_aborts_with_a_named_refusal`; `the_abort_path_is_replication_stable` |
| **2** | a binding is unset | **RUN VOID** | `an_unset_binding_is_a_named_refusal_and_never_a_default` |
| **2** | `LANDING` is not a 40-hex object name | **RUN VOID** | `a_landing_that_is_not_an_object_name_is_refused_before_any_git_call_can_accept_it` |
| **2** | a binding holds a non-printable character | **RUN VOID** | `a_binding_holding_a_newline_is_refused_before_it_reaches_the_record` |
| **2** | the baseline record is not a COMPLETE `baseline_snapshot` | **RUN VOID** (D-160) | `an_incomplete_baseline_record_voids_the_run_rather_than_aborting` |
| **2** | tracked files are modified | **RUN VOID** | `a_modified_tracked_file_voids_the_run_by_its_own_reason` |
| **2** | untracked files on build-reaching paths | **RUN VOID** | `an_untracked_file_on_a_build_reaching_path_voids_the_run` |
| **2** | build-reaching paths moved between `LANDING` and `HEAD` | **RUN VOID** | `build_reaching_drift_above_landing_voids_the_run` |
| **2** | `LANDING` is not an ancestor of `HEAD` | **RUN VOID** | `a_landing_that_is_not_an_ancestor_of_head_voids_the_run` |
| **2** | `LANDING` changes nothing under the subject | **RUN VOID** — with `p = 0` an empty diff is also what "the WP never landed" looks like | `a_landing_that_changes_nothing_under_the_subject_voids_the_run` |
| **2** | the crate is absent from the workspace, or its specification is ambiguous | **RUN VOID** — not the same observation as "no edge" | `a_crate_absent_from_the_workspace_is_a_refusal_and_not_an_acceptance` |
| **2** | cargo cannot resolve the workspace | **RUN VOID** | `an_unreadable_workspace_voids_the_run_by_its_own_reason` |
| **2** | any other `refuse` | **RUN VOID**, not a verdict in either direction | missing/misdigested/unreadable baseline; a failed build |
| — | `toolchain matches baseline: no` | **advisory.** H1-a rebuilds both sides on one toolchain and is immune; it bears only on H1-b | — |

**AND ONE ROW THAT IS NOT A VERDICT AT ALL, recorded because it is the fix for a defect that
reached one**: tampering with the instrument's config in the WORKING TREE does not move the
verdict, because the snapshot runs inside the pristine clone —
`tampering_with_the_instruments_config_in_the_worktree_does_not_reach_the_verdict`.

**THE EXIT CODES ARE DISJOINT ONLY IF THE TRAP LETS THEM BE**, which revision 9's did not:
measured at `requested 0 → got 1, requested 1 → got 1, requested 2 → got 1` on that
construction with an unremovable path. The repaired form returns 0, 1 and 2, and a red-team
confirmed it across four modes.

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

**SECOND INSTRUMENT — AND THE ORDERING DEFECT REVISION 11 HAD TO FIX BEFORE THE CRITERION MEANT
ANYTHING.**

Revisions 6-9 registered *"H1-b reports the invariant blocks identical **if and only if**
instrument 2 refuses"*, and it was **violated at the registered bindings**:
`tools/bench_delta.sh:274` refuses only when two digests are EQUAL; `cdbcbf0` gives `ff018398…`
and `8618012` gives `a7f519fa…`, so it does not refuse while H1-b confirms. TRUE ⇔ FALSE, and
the registered consequence then forbade any verdict. It defended itself with *"H1-b is `N/A`"*,
a sentence dead since revision 6.

**And the deeper fault decided the fix**: under **D-269** — the clause this document's own §7.1
produced — **H1-b is a criterion H1's target defect PRESERVES** (§3). The second-instrument duty
was anchored to the one criterion in the document that cannot falsify what it watches.

**THE REGISTERED SECOND INSTRUMENT IS `tools/solver_edge_check.sh`, AND IT IS H1-a's.**

> **AGREEMENT CRITERION, ONE-DIRECTIONAL.** If the workspace-wide inverted normal-edge tree for
> `pistol-solver` at `LANDING` is ONE LINE, then H1-a **must** report the two binaries
> bit-identical. A crate outside the resolved graph cannot reach codegen, so a difference means
> the two instruments contradict each other. **The converse does not hold and is not
> registered.**

**A BICONDITIONAL HERE WOULD BE UNSOUND, AND THIS DOCUMENT LEARNED THAT FROM A TEST RATHER THAN
FROM A REVIEWER — which is the first time that has happened in this work package.** Revision 11
was drafted with the biconditional above, and `tools/wp15a_h1.sh`'s own suite failed on it
immediately: a fixture whose app takes a normal dependency on the subject but never calls it
gives `edge = 1` with two **bit-identical** binaries, which the biconditional calls a
disagreement and voids. **The two instruments do not ask the same question.** The graph asks
whether the subject is LINKED; H1-a asks whether the subject's CONTENT CHANGE between `BASE_REV`
and `LANDING` reaches codegen. §2.5 already records the gap in its own words — `binary_sha256`
is insensitive to dead code **even in a linked crate** — and the criterion had been written as
though it did not.

So the readings compose in one direction only:

| graph | H1-a | verdict |
|---|---|---|
| a dependent is named | either | **ABORT.** `p != 0`. H1-a's reading is RECORDED and adjudicates nothing: a linked crate whose diff happens to be dead is still linked |
| one line (no dependent) | identical | proceed to H1-b |
| one line (no dependent) | differs | **RUN VOID** — the instruments contradict each other |

**Neither reads the other's input** — one compiles and digests bytes, the other resolves a
dependency graph — which is the independence T8 (`docs/process_readings.md`) observes the
proportionality paragraph never required. It is also cheap: seconds, no build, against the
6 m 18 s of the instrument it displaced.

**AND REVISION 11 FIXED AN ORDERING DEFECT THAT MADE THE CRITERION UNEVALUABLE WHERE IT MATTERS,
a review finding and a genuine one.** At revision 10 the edge check ran BEFORE H1-a and exited on
refutation, so whenever instrument 2 said "edge", **H1-a never reported** and the case where the
two could disagree was decided on one of them alone. **The instrument now takes both readings
before it acts on either**, so both reach the record and the contradiction branch can fire —
exercised by `a_moved_binary_with_no_edge_in_the_graph_voids_the_run`, with
`a_used_subject_with_a_truthful_edge_check_aborts_rather_than_contradicting` as its control, so
the branch is reached by a lying instrument and not by the fixture.

**`tools/bench_delta.sh` IS DEMOTED TO A RECORDED CORROBORATION WITH NO ADJUDICATING ROLE.** Its
refusal condition is digest-EQUALITY, which is not the proposition; it compares a different
pair; and it emits `VERDICT ABORT` twice per run meaning **D-215's** verdict against a different
pre-registration. Two ABORTs meaning contradictory things in one transcript is a hazard these
rounds have been spent removing. Recorded reading, provenance only: at `(cdbcbf0, 7b9e904)` it
built both sides in throwaway worktrees and reported `ff018398…` and `a7f519fa…`, independently
reproducing both digests obtained from a pristine clone.

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

### 7.6 REVISION 11 — THE DRY RUN IS NOW A TEST SUITE, PLUS ONE REAL PAIR

**WHAT THE DRY-RUN RULE ATTACHES TO NOW.** D-227 requires a pre-registration's LITERAL COMMANDS
to be exercised before its review passes, on an input of the SAME KIND as the registered
workload. The literal command is now `tools/wp15a_h1.sh`, and it is exercised two ways:

1. **By a suite CI runs, on every commit** — 26 tests over the two instruments, in scratch git
   repositories and scratch cargo workspaces, WITH CONTROLS. That is stronger than a dry run in
   the one respect a dry run has always been weak: it does not depend on a session remembering
   to take it. **DEFECT CLASS IT EXCLUDES** (D-269): an instrument that answers a different
   question than the one registered, or a guard that stops firing. **FALSIFIABLE BY IT**:
   demonstrated, twice, by mutation — re-adjudicating on `cargo tree -i`'s exit status turns
   four tests red, narrowing `--workspace` to one member turns three.
2. **By DR-11, on a real non-registered pair of the same kind** — `(5fdbf52, 60b5c44)`, a
   baseline record at one revision of this repository against a landing revision above it, in a
   throwaway clone. **exit 0, 43 s.** The registered workload is `(cdbcbf0, 8618012)` and is not
   touched.

```
solver_edge_check: NO normal reverse-dependency on pistol-solver anywhere in the workspace
wp15a_h1: p = 0 — no normal reverse-dependency on pistol-solver at 60b5c44b…
wp15a_h1: H1-a with subject    a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
wp15a_h1: H1-a without subject a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
wp15a_h1: H1-a counterfactual lock delta: none
wp15a_h1: H1-a reading identical
wp15a_h1: H1-a CONFIRMED — pistol-solver contributes nothing to the shipped binary
wp15a_h1: toolchain candidate rustc 1.97.1 (8bab26f4f 2026-07-14)
wp15a_h1: toolchain matches baseline: yes
wp15a_h1: H1-b comparing 54 behaviour lines (revision and binary_sha256 excluded)
wp15a_h1: H1-b CONFIRMED — every behaviour line is byte-identical
```

**THE TWO DEFECTS THE SUITE FOUND BEFORE ANY REVIEWER DID**, which is the whole of what the
promotion buys and is recorded here rather than in a summary, because a claim that tests help is
worth exactly the defects it can name:

- **`cargo tree` prints ABSOLUTE PATHS**, so the new adjudicator was writing the `mktemp` clone's
  name into stdout. That is the same class two reviewers had independently graded BLOCKING one
  revision earlier — a `diff -u` header leaking its temporary paths — reintroduced through the
  instrument built to fix it, and this time on the **CONFIRMED** path, where it would have voided
  every replicated run rather than only a failing one. Caught by
  `the_abort_path_is_replication_stable` on the suite's first execution. Fixed by substituting
  the workspace root for a fixed token; two clones at different paths now print identically.
- **THE AGREEMENT CRITERION THIS REVISION WAS DRAFTED WITH WAS UNSOUND.** It read as a
  biconditional, and a fixture whose app takes a normal dependency it never calls gives
  `edge = 1` with two BIT-IDENTICAL binaries — which a biconditional calls a disagreement and
  voids. The two instruments ask different questions (§6), and §2.5 had said so in its own words
  all along. The criterion is one-directional now, and it was the TEST that said so.

**REPLICATION, MEASURED.** Three independent processes at DR-11's bindings:
**one distinct stdout, `6803ce5a…`, exit 0 each.** The ABORT path's stability is separately
pinned by `the_abort_path_is_replication_stable` — the arm that had no producer until revision 11
and the one whose instability was BLOCKING at revision 9, found there by two reviewers
independently.

**WHAT THE EARLIER DRY RUNS ARE NOW.** DR-1 to DR-10 and their arms (§7.2-§7.5) were taken
against blocks that no longer exist. They are kept because the FINDINGS they produced are why
the current instrument is shaped as it is — every guard in it is a defect one of them found —
and because a document that deleted the evidence for its own repairs would be asking to be
trusted rather than read. **They certify nothing about revision 11's instrument.** The suite and
DR-11 do.

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
