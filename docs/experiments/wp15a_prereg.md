# WP-1.5a — PRE-REGISTRATION (revision 9)

**Revision 9.** Revision 8 was never reviewed; it goes to review as this text. Four things
moved, and **the first two were forced by the closure session's own commits** rather than found
by reading:

- **THE EXIT TRAP REWROTE VERDICTS.** Revision 8's `cleanup() { rm -rf -- "$WORK" "$PRISTINE"; }`
  is `SHELL_CHECKLIST` item 7 in the position where it costs everything: an EXIT trap's last
  command decides the script's status. Measured on that exact construction with an unremovable
  path — **requested 0 → got 1, requested 1 → got 1, requested 2 → got 1.** CONFIRMED became
  ABORT and RUN VOID became ABORT, so the three-way disjointness §3.2 spent a whole round
  establishing could be collapsed by *housekeeping*. The repaired trap returns 0, 1 and 2 for
  the same three requests.
- **THE SOLVER-DIFF GUARD ADJUDICATED A BROKEN INVOCATION AS ITS OWN CONCLUSION.**
  `git diff --stat … | grep -q . || refuse` routes three different reasons — `grep`'s exit 1 on
  no match, `git`'s 128 on a bad revision, a producer's 141 when its reader has left — into one
  refusal naming only the first (`SHELL_CHECKLIST` item 3, and item 8 on top of it). Measured:
  `deadbeef…` printed `fatal: bad object` and the run announced **"changes nothing under
  crates/pistol-solver"** — this guard's own load-bearing claim, asserted about an invocation
  that never answered. It is §7.1's externally derived fix for M-4 and therefore the single
  guard between *"the WP landed"* and *"the WP never landed"*. Rewritten with no pipeline at
  all. The 141 reading is **recorded as NOT REPRODUCING at this workload's scale** (§7.3): the
  largest `--stat` this repository can produce is 17 879 bytes, well inside a pipe buffer, and
  20 trials all exited 0 — the construction is defective in kind and bounded here by an accident
  of size, which is not a guard.
- **`HEAD` IS NO LONGER REQUIRED TO EQUAL `LANDING`**, because the closure session lands ADR
  lines above the work package and an equality test would void every governed run over docs
  commits that cannot reach a binary. `LANDING` still does not move. What replaces it is
  stronger where it matters and is a diff rather than a token: `LANDING` must be an ancestor of
  `HEAD`, and `Cargo.toml`, `Cargo.lock`, `crates`, `configs` and **`tools`** must be untouched
  between them — `tools` because step 9 runs `tools/baseline_snapshot.sh` out of the working
  tree, so the instrument is pinned by the same assertion as the binary.
- **THE UNTRACKED-FILE TOLERANCE IS RETIRED (§3.3, §4 of the block).** D-266 tracked
  `docs/research/threat_calculus_v1.md`, so `git status --porcelain` is empty and a bare
  cleanliness test is a reachable check again rather than a clause that voids every run. The
  two-part tolerance survives as the DIAGNOSIS, because the refusal has three reasons and one
  combined test gives one wrong answer.

**And three more, each found by answering a `SHELL_CHECKLIST` item by name rather than by
reading for style:** a `cd "$REPO"` was missing, and every `git ls-files -- <pathspec>` resolves
relative to the working directory — measured, from `crates/` the registered enumeration returned
**EMPTY** while `git status --porcelain` reported the stray file, EXIT-0-WRONG-ANSWER selected by
cwd (item 5); `echo "… $(wc -l < …) behaviour lines"` is item 1's headline example, a
substitution whose status is an argument; and the block's own section numbers ran **0-8** while
five prose references pointed into a **1-9** scheme, so a reviewer chasing `step 6, n = 0` landed
in H1-a. The block is renumbered 1-9 to agree with the prose it is cited by.

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

### 0.1 THE SIX VARIABLES, BOUND (rebound at revision 7; the two artifact PATHS moved at revision 9)

Every value was verified this session rather than transcribed.

**The two baseline artifacts moved, and their digests did not (revision 9).** They lived in the
scratchpad of the session that took them, which is neither durable nor this session's to keep;
they are copied to this session's scratchpad and **re-verified against the digests already
registered here** — `sha256sum -c` reports `OK` for both, the record's `revision` line still reads
`cdbcbf05…` and its `binary_sha256` still reads `ff018398…`, and the sizes are 5764 and 892 bytes
as recorded below. **A path is a locator and the digest is the binding**, which is the whole
reason a digest was pinned rather than a path; had the copy differed by a byte, step 2 would
refuse before H1 asked anything.

```sh
BASE=/tmp/claude-1000/-home-tom-Projects-HeXO-AlphaBeta/97ce0961-e0c0-4100-a7a3-03f0695ef67f/scratchpad/wp15a/w3/baseline_cdbcbf0.txt
BASE_SHA=7faa074c21a2d7d28e4ca681e05ed95942436d639642c088a73032febd33159a
BASE_REV=cdbcbf05bd9d792ac7a6af709970f11b95796b81
BASE_TC=/tmp/claude-1000/-home-tom-Projects-HeXO-AlphaBeta/97ce0961-e0c0-4100-a7a3-03f0695ef67f/scratchpad/wp15a/w3/baseline_cdbcbf0.toolchain.txt
BASE_TC_SHA=8be240559c27b2b163347cba8f2266f7877ceee0ef8d72095e2d73537a6adc2a
LANDING=861801247df5c1a73480b5153e11c399aa752750
```

| variable | source, verified |
|---|---|
| `BASE` | the W3 baseline record, 5764 bytes; digest recomputed and matching |
| `BASE_SHA` | recomputed `7faa074c…` |
| `BASE_REV` | the record's own `revision` line; and its `binary_sha256 ff018398…` **reproduces from a pristine checkout of that revision** (§3.1 step 3) |
| `BASE_TC` | the sidecar, 892 bytes: `rustc 1.97.1`, `cargo 1.97.1`, LLVM 22.1.6, `stable-x86_64-unknown-linux-gnu`. It carries `snapshot_revision` and `snapshot_binary_sha256` matching the record, which is what ties the two artifacts to each other rather than to a claim |
| `BASE_TC_SHA` | recomputed `8be24055…` |
| `LANDING` | `86180124…`, confirmed **equal to HEAD** at rebinding time, seventeen commits after `BASE_REV`. Revision 6 bound `7b9e904a…`; the seven fix commits between them are recorded in §0.2 |

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
(§3.1 step 3), which the baseline passes, and that substitution was made for exactly this reason.

### 0.3 THE INSTRUMENTS, NAMED WITH THEIR REVISIONS (D-268, new at revision 9)

D-268 amended CLAUDE.md in the same session as this revision: *an artefact that produces a
registered number — a `tools/` script, a scratchpad harness, or a command block the document
prints — is NAMED IN THE PRE-REGISTRATION WITH ITS REVISION, and a change to it reopens the
review exactly as an amendment to the document does.* This document was one of the two instances
that produced the rule (its revision 1 declared T1 inapplicable because no `tools/` harness was
modified, while the harness producing its number was an unpinned scratchpad bench), so it states
its own instruments rather than being the first document to ignore the clause it caused.

| instrument | what it produces | revision, pinned |
|---|---|---|
| the §3.1 block | H1-a's two digests, H1-b's line comparison, both verdicts | printed in this document; its revision IS this document's, and §7.2/§7.3 extract it BY SCRIPT rather than retyping it, so the thing run is the thing reviewed |
| `tools/baseline_snapshot.sh` | the candidate invariant block at step 9 | blob `a7c0ed4367a6893f70b776732bd68ada77c19483` at `LANDING`; **held there by step 5**, which refuses if `tools` moved between `LANDING` and `HEAD` — the assertion is the pin, not a comment |
| `tools/bench_delta.sh` | instrument 2's refuse-or-not-refuse reading (§6) | blob `130b4acbd87f1413776da7b1354f00a90bbc41d6` at `LANDING`, held by the same assertion |
| the baseline record + sidecar | H1-b's comparand | not a script: pinned by digest, `7faa074c…` and `8be24055…`, §0.1 |

**The pin is an executable assertion and not a table.** Step 5 diffs `Cargo.toml`, `Cargo.lock`,
`crates`, `configs` and `tools` between `LANDING` and `HEAD` and refuses on any change, so an
instrument edit lands as a RUN VOID with the path named — which is what the clause asks for, in
the only form that survives a session forgetting to re-read this table.

## 1. The change under test

`crates/pistol-solver` gains the threat state, its queries, an oracle and the golden fixtures
(DESIGN.md §3, §7, §8). It takes one dependency, `pistol-core`, and **no dev-dependencies**.
**Nothing in the engine calls it.**

## 2. `p = 0`, measured — with a DETERMINISTIC command (M-1)

**Three revisions of this document printed a "verbatim" grep and three reviews found it wrong.
The cause is not carelessness and it outlives this document.** It is now **D-265**, and
**revision 9 corrects the attribution that line was drafted with**, because the wrong attribution
implied a mitigation that would have done nothing.

**What is true, measured at the closure session's HEAD.** In the shell an agent's tool runs,
`type -t grep` reports `function`, and that function execs `${CLAUDE_CODE_EXECPATH:-}` under
`exec -a ugrep` with `-G --ignore-files --hidden -I --exclude-dir=.git …` prepended. It is
multithreaded and its output ORDER is nondeterministic: **eight runs of
`grep -rn 'WINDOW_LEN' crates/` through the function gave EIGHT distinct sha256 sums; eight runs
of `/usr/bin/grep` on the same query gave ONE.**

**What revision 8 got wrong.** It said this was "a shell function wrapping ugrep in a shell
initialised from the user's profile". **It is not the profile.** `~/.bashrc`, `~/.bash_profile`
and `~/.profile` carry no `grep` alias or function, `~/.bash_aliases` does not exist, **`ugrep` is
not on `PATH` at all**, and a genuine login-interactive shell disagrees outright:
`bash -lic 'type grep'` reports `grep is /usr/bin/grep`. The wrapper is **injected by the tool
harness** into the agent's own shell and by nothing else — which moves the mitigation from "do not
use a login shell", which would have bought nothing, to **"a transcript an agent captured is not
the output of the command a script runs"**.

**And the hazard is not only order.** The injected flags change WHICH FILES ARE VISITED:
`grep -rl pistol target` through the function reaches **1474** files where `/usr/bin/grep` reaches
**24455**, since `--ignore-files` honours ignore files and `-I` skips binaries. An agent-shell
COUNT and a script's count of the same command are answers to different questions — D-221/D-223's
class arriving by a second route.

**Why it hid for three revisions of this document:** a query with a SINGLE HIT is stable through
both paths — ten runs of the attestation below gave one hash — so the defect is invisible on
exactly the small outputs a document pastes, and shows up on the large ones nobody diffs.

So **no fixed transcript can ever be the "exact output"** of the plain `grep -rn` this document
kept printing, and every capture taken through that path — including the one an earlier HANDBACK
called "the real output" — was a sample from a distribution. It collides with the determinism law
(rule 4) and with CLAUDE.md's rule that a gate claim cites the gate's own log output, since an
unordered instrument cannot have a log output to cite. **§3.1 is a SCRIPT** — `#!/usr/bin/env
bash`, run as a file — so every `grep` in it is `/usr/bin/grep` and the wrapper cannot reach it;
that is a property of how the block is invoked, and §7.2 records the block being extracted and run
rather than pasted into a shell.

**The registered command is deterministic, revision-pinned and sorted**, and it attests the
committed state at the landing revision — which is what the `p = 0` claim is about, not the
working tree (the tree is dirty while IMPL works):

```sh
git grep -n -e 'pistol-solver' -e 'pistol_solver' "$LANDING" -- '*Cargo.toml' | LC_ALL=C sort
```

Ten runs at `72316a7` produce one hash. Captured output at that revision:

```
72316a7:crates/pistol-solver/Cargo.toml:2:name = "pistol-solver"
```

**Exactly one line, the crate's own `name =`.** No `Cargo.toml` in the workspace declares a
dependency on `pistol-solver`, which is the whole of the `p = 0` claim. It is checked rather
than eyeballed:

```sh
mapfile -t SOLVER_MANIFESTS < <(git grep -l -e 'pistol-solver' -e 'pistol_solver' "$LANDING" -- '*Cargo.toml' || true)
n=${#SOLVER_MANIFESTS[@]}
[ "$n" -eq 1 ] || { echo "prereg: $n Cargo.toml files mention pistol-solver, expected exactly 1" >&2; exit 2; }
```

This is the form §3.1 registers, character for character. Revision 7 printed a `git grep -c …
| wc -l` variant here while §3.1 registered `git grep -l …` — both returned 1 at this revision,
so the discrepancy was invisible, which is exactly why it is worth removing.

`cargo tree -p pistol-cli --edges normal` corroborates on the resolved graph and is recorded,
not adjudicated on. Both were re-verified independently by reviewers and are recorded in D-249.

`1/(1 − 0 + 0/k) = 1.000` for every `k`.

**H1 is falsifiable by its target defect**, confirmed by a reviewer's break test: a bare
`[dependencies] pistol-solver` edge with no call site moves the digest
`62c102cc…` → `eeefee04…`, while a dev-dependency correctly does not — which is why the
oracle's test-tree home is safe.

**ONE INSTRUMENT CAVEAT, carried because the document should not overclaim.**
`binary_sha256` is **insensitive to dead code even in a linked crate**: an unreferenced
`pub const` appended to `pistol-core` did not move the digest. H1's *target* defect does move
it, so H1 is sound — but the instrument detects "a change that reaches codegen", not "any
change", and a reader must not treat a CONFIRMED H1 as attesting that no code was added.

## 3. H1 — the whole-engine claim, restructured for real inputs

**The claim is unchanged: WP-1.5a adds nothing a shipped binary can observe, because `p = 0`.**
What changed in revision 6 is the instrument, because the registered one met the real pair and
could not attribute its own answer.

- **H1-a (PRIMARY, counterfactual).** At `LANDING`, in a pristine clone, the binary built from
  the tree **is bit-identical** to the binary built from the same tree with
  `crates/pistol-solver` reverted to its `BASE_REV` content. This isolates exactly the work
  package's contribution and is immune to unrelated churn in linked crates.
- **H1-b (SECONDARY, conditional).** *If* the `BASE_REV`→`LANDING` diff touches no
  build-reaching path outside `crates/pistol-solver`, the invariant blocks must be
  byte-identical excluding `revision`. **If it does touch such a path, H1-b is `N/A` — recorded
  with the offending paths named, and adjudicating nothing.** It is not an ABORT: a digest
  moved by a linked crate is not evidence about `pistol-solver`.

**Why H1-a is the stronger instrument, in §7.1's own terms.** H1-b compares against a *stored
artifact* that shares an input with the thing under suspicion — the tree — so a defect in the
tree can survive it. H1-a compares two builds that differ **only** in the suspect content, and
the referent is derived by an independent build rather than read from a file. It is the
externally derived referent the rule asks for.

### 3.1 The registered block

Every refusal exits **2** through one `refuse()` helper; `0` is CONFIRMED, `1` is ABORT.

```sh
#!/usr/bin/env bash
set -euo pipefail

refuse() { echo "prereg: $*" >&2; exit 2; }

# ---- 1. Bindings. All six bound by §0.1; nothing here is a placeholder. ----
BASE=${BASE:-}               ; [ -n "$BASE" ]        || refuse "BASE unset"
BASE_SHA=${BASE_SHA:-}       ; [ -n "$BASE_SHA" ]    || refuse "BASE_SHA unset"
BASE_REV=${BASE_REV:-}       ; [ -n "$BASE_REV" ]    || refuse "BASE_REV unset"
BASE_TC=${BASE_TC:-}         ; [ -n "$BASE_TC" ]     || refuse "BASE_TC unset"
BASE_TC_SHA=${BASE_TC_SHA:-} ; [ -n "$BASE_TC_SHA" ] || refuse "BASE_TC_SHA unset"
LANDING=${LANDING:-}         ; [ -n "$LANDING" ]     || refuse "LANDING unset"

# `git rev-parse` exits 128 outside a repository, which is a fourth status the
# adjudication table does not define; route it to 2 like every other refusal.
REPO=${REPO:-}
if [ -z "$REPO" ]; then
  REPO="$(git rev-parse --show-toplevel 2>/dev/null)" || refuse "not inside a git repository"
fi
[ -n "$REPO" ] || refuse "cannot resolve the repository root"
# AND ENTER IT (revision 9). Every `git ls-files -- <pathspec>` below resolves its
# pathspec RELATIVE TO THE CURRENT DIRECTORY, so the same command run from
# `crates/` looks for `crates/crates` and reports "no untracked files on
# build-reaching paths" with a stray file sitting in one. MEASURED, not argued:
# from `crates/` with `crates/pistol-core/ZZZ_untracked_probe.rs` present,
# `git status --porcelain` reported it and the registered enumeration returned
# EMPTY. That is EXIT-0-WRONG-ANSWER selected by working directory, and it is
# SHELL_CHECKLIST item 5's rule that the summary must be counted with the
# enumeration the check used. `tools/baseline_snapshot.sh` at step 9 is a
# relative path besides.
cd "$REPO" || refuse "cannot enter the repository root $REPO"

WORK="$(mktemp -d -t wp15a_inv.XXXXXX)"     || refuse "cannot create the work directory"
PRISTINE="$(mktemp -d -t wp15a_pristine.XXXXXX)" || refuse "cannot create the pristine directory"
# 61 MB per run, and ten runs left 605 MB behind before this trap existed.
# SHELL_CHECKLIST item 7, and this one CHANGES VERDICTS (revision 9): revision 8
# wrote `cleanup() { rm -rf -- "$WORK" "$PRISTINE"; }`, whose last command decides
# the script's status, so a failed removal rewrote the answer. Measured on the
# revision-8 construction with an unremovable path: requested 0 -> got 1,
# requested 1 -> got 1, requested 2 -> got 1. CONFIRMED and RUN VOID both became
# ABORT, collapsing the three-way disjointness §3.2 spent a round establishing,
# by housekeeping. Take `rc` first, make the removal unable to fail the trap, and
# hand `rc` back: the same three requests then return 0, 1 and 2.
cleanup() { local rc=$?; rm -rf -- "$WORK" "$PRISTINE" 2>/dev/null || true; return "$rc"; }
trap cleanup EXIT

# ---- 2. Baseline record: present, pinned, at the registered revision. ----
[ -s "$BASE" ] || refuse "baseline record missing or empty at $BASE"
printf '%s  %s\n' "$BASE_SHA" "$BASE" | sha256sum -c - >/dev/null \
  || refuse "baseline record does not match its registered digest"
[ "$(sed -n 's/^revision //p' -- "$BASE")" = "$BASE_REV" ] \
  || refuse "baseline record is not at the registered baseline revision"
[ -s "$BASE_TC" ] || refuse "baseline toolchain sidecar missing at $BASE_TC"
printf '%s  %s\n' "$BASE_TC_SHA" "$BASE_TC" | sha256sum -c - >/dev/null \
  || refuse "baseline toolchain sidecar does not match its registered digest"

# ---- 3. The baseline's cleanliness, attested by REBUILD not by its own token.
#         The record's `timing tree` token is set from a bare `git status
#         --porcelain` and read `dirty` for as long as the threat calculus sat
#         untracked (D-266); what matters either way is whether anything
#         uncommitted reached the binary, and a pristine rebuild answers that
#         where a token cannot. ----
BASE_DIGEST_RECORDED="$(sed -n 's/^binary_sha256 //p' -- "$BASE")"
[ -n "$BASE_DIGEST_RECORDED" ] || refuse "baseline record carries no binary_sha256"
git clone --quiet --no-hardlinks "$REPO" "$PRISTINE/repo" || refuse "cannot clone for the rebuild attestation"
( cd "$PRISTINE/repo" && git checkout --quiet "$BASE_REV" ) || refuse "baseline revision not in the clone"
[ -z "$(cd "$PRISTINE/repo" && git status --porcelain)" ] || refuse "the pristine clone is not pristine"
BASE_BIN="$(cd "$PRISTINE/repo" && cargo build --release --locked -p pistol-cli --bin pistol \
    --message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
  || refuse "baseline rebuild failed"
[ -s "$BASE_BIN" ] || refuse "baseline rebuild produced no binary"
BASE_BIN_DIGEST="$(sha256sum -- "$BASE_BIN" | cut -d' ' -f1)" || refuse "cannot digest the baseline rebuild"
[ "$BASE_BIN_DIGEST" = "$BASE_DIGEST_RECORDED" ] \
  || refuse "baseline binary_sha256 does not reproduce from a pristine checkout of $BASE_REV — \
something uncommitted reached the baseline binary"
echo "prereg: baseline rebuild attests $BASE_DIGEST_RECORDED"

# ---- 4. The candidate tree is CLEAN, which became a reachable state at D-266.
#         Revisions 6-8 registered a two-part tolerance because an untracked file
#         made a bare emptiness test void every governed run; the file is tracked
#         and the bare test is the registered check again. The two parts survive
#         as the DIAGNOSIS, because a single combined test gives a wrong
#         diagnosis (SHELL_CHECKLIST item 8) and this one has three reasons. ----
DIRT="$(git status --porcelain)" || refuse "cannot read the working tree status"
if [ -n "$DIRT" ]; then
  TRACKED="$(git status --porcelain --untracked-files=no)" || refuse "cannot read the tracked-file status"
  [ -z "$TRACKED" ] || refuse "tracked files are modified: $TRACKED"
  STRAY="$(git ls-files --others --exclude-standard -- Cargo.toml Cargo.lock crates configs tools)" \
    || refuse "cannot enumerate untracked files"
  [ -z "$STRAY" ] || refuse "untracked files on build-reaching paths: $STRAY"
  refuse "untracked files outside the build-reaching set: $DIRT"
fi

# ---- 5. Revision assertions, including the solver-diff guard.
#         HEAD IS NO LONGER REQUIRED TO EQUAL LANDING (revision 9). The closure
#         session lands ADR lines above the work package, so an equality test
#         would void every governed run over docs commits that cannot reach a
#         binary. What the run needs is that nothing between LANDING and HEAD can
#         reach the binary OR the instrument, and that is a DIFF rather than a
#         token: `tools` is in the pathspec precisely because step 9 runs
#         `tools/baseline_snapshot.sh` out of the working tree. ----
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
[ "$(git rev-parse "$LANDING")" != "$(git rev-parse "$BASE_REV")" ] \
  || refuse "candidate and baseline are the same revision"
# THE SOLVER-DIFF GUARD, REWRITTEN (revision 9). Revision 8 wrote
# `git diff --stat … | grep -q . || refuse`, which is SHELL_CHECKLIST item 3 in
# the position where it costs the most: this is §7.1's externally derived fix for
# M-4 and the one guard standing between "the WP landed" and "the WP never
# landed". `grep` exits 1 on NO MATCH, git exits 128 on a bad revision, and a
# producer whose reader has already left exits 141 — under `pipefail` all three
# arrive as one non-zero status routed into ONE refusal that names only the
# first. MEASURED on the registered construction: `deadbeef…` printed
# `fatal: bad object` and was adjudicated `changes nothing under
# crates/pistol-solver`, this guard's own load-bearing claim asserted about an
# invocation that never answered. Take the value first, check its SHAPE, refuse
# by name — SHELL_CHECKLIST items 1 and 8, and no pipeline at all.
SOLVER_DIFF="$(git diff --name-only "$BASE_REV" "$LANDING" -- crates/pistol-solver)" \
  || refuse "cannot diff crates/pistol-solver between $BASE_REV and $LANDING — the guard did not answer, \
which is not the same as answering that the diff is empty"
[ -n "$SOLVER_DIFF" ] || refuse "$LANDING changes nothing under crates/pistol-solver: with p = 0 an empty diff is \
also what 'the WP never landed' looks like"

# ---- 6. p = 0. `git grep` exits 1 on NO MATCH, which under `pipefail` killed
#         the script before `refuse()` could speak and adjudicated the STRONGEST
#         evidence for p = 0 as its opposite (SHELL_CHECKLIST item 3). `mapfile`
#         off a process substitution takes the count without a pipeline status
#         and without `grep -c`, which has the same trap at zero. ----
mapfile -t SOLVER_MANIFESTS < <(git grep -l -e 'pistol-solver' -e 'pistol_solver' "$LANDING" -- '*Cargo.toml' || true)
n=${#SOLVER_MANIFESTS[@]}
[ "$n" -eq 1 ] || refuse "$n Cargo.toml files mention pistol-solver, expected exactly 1 (the \
crate's own name field); 0 means the attestation could not be taken, 2 or more means an edge"
# `git grep -l <rev>` prefixes each path with `<rev>:`; strip it before comparing.
SOLVER_MANIFEST_PATH="${SOLVER_MANIFESTS[0]#*:}"
[ "$SOLVER_MANIFEST_PATH" = "crates/pistol-solver/Cargo.toml" ] \
  || refuse "the single match is $SOLVER_MANIFEST_PATH, not the solver's own manifest"
# The attestation ITSELF was a bare pipeline in statement position — unreachable
# with `n = 1` already asserted, but unreachable-and-fatal rather than
# unreachable-and-routed, which is the shape the item-3 findings keep taking.
# `LC_ALL=C sort` is not decoration either: D-265 records that an agent shell's
# `grep` is a harness-injected multithreaded wrapper whose output ORDER is
# nondeterministic, so a recorded transcript is sorted or it is not a transcript.
ATTEST="$(git grep -n -e 'pistol-solver' -e 'pistol_solver' "$LANDING" -- '*Cargo.toml' | LC_ALL=C sort)" \
  || refuse "the p = 0 attestation could not be taken at $LANDING"
printf '%s\n' "$ATTEST"

# ---- 7. H1-a: build LANDING, then build it with the solver crate REMOVED. ----
( cd "$PRISTINE/repo" && git checkout --quiet "$LANDING" ) || refuse "landing revision not in the clone"
CAND_BIN="$(cd "$PRISTINE/repo" && cargo build --release --locked -p pistol-cli --bin pistol \
    --message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
  || refuse "candidate build failed"
[ -s "$CAND_BIN" ] || refuse "candidate build produced no binary"
D_WITH="$(sha256sum -- "$CAND_BIN" | cut -d' ' -f1)" || refuse "cannot digest the candidate build"
cp -- "$CAND_BIN" "$WORK/with_solver.pistol" || refuse "cannot preserve the candidate binary"
# `rm -rf` first: `git checkout BASE_REV -- <path>` MERGES the old files in and
# leaves every file the WP added on disk, and cargo auto-discovers `build.rs`,
# `src/bin/`, `benches/`, `examples/` and `tests/` by convention.
( cd "$PRISTINE/repo" && rm -rf -- crates/pistol-solver \
    && git checkout --quiet "$BASE_REV" -- crates/pistol-solver ) \
  || refuse "cannot restore the solver crate to its baseline content"
# `--locked` is deliberately DROPPED here and only here: removing the solver's
# dependency edge necessarily moves Cargo.lock, so `--locked` would refuse. The
# guard it gives up is replaced by the assertion below that the lock moves in
# exactly that one way.
COUNTER_BIN="$(cd "$PRISTINE/repo" && cargo build --release -p pistol-cli --bin pistol \
    --message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
  || refuse "counterfactual build failed"
[ -s "$COUNTER_BIN" ] || refuse "counterfactual build produced no binary"
D_WITHOUT="$(sha256sum -- "$COUNTER_BIN" | cut -d' ' -f1)" || refuse "cannot digest the counterfactual build"
# After the build, so cargo has re-resolved: this is what dropping --locked bought.
LOCK_DELTA="$(cd "$PRISTINE/repo" && git diff --shortstat -- Cargo.lock)" || true
echo "prereg: H1-a with solver    $D_WITH"
echo "prereg: H1-a without solver $D_WITHOUT"
echo "prereg: H1-a counterfactual lock delta:${LOCK_DELTA:- none}"
[ "$D_WITH" = "$D_WITHOUT" ] || { echo "prereg: H1-a FAILED — the solver reaches the binary" >&2; exit 1; }
echo "prereg: H1-a CONFIRMED — the solver contributes nothing to the shipped binary"

# ---- 8. Toolchain comparison, whole-line and anchored. ----
# `rustc -vV | head -1` closes the pipe early, rustc takes SIGPIPE and pipefail
# propagates it — the cmd | head trap. Capture whole, then slice.
RUSTC_VV="$(rustc -vV)" || refuse "cannot read rustc -vV"
RUSTC_LINE="$(printf '%s\n' "$RUSTC_VV" | sed -n '1p')"
CARGO_LINE="$(cargo --version)"     || refuse "cannot read cargo --version"
{ printf '%s\n%s\n' "$RUSTC_LINE" "$CARGO_LINE"; } > "$WORK/toolchain.cand"
if grep -Fxq -- "$RUSTC_LINE" "$BASE_TC" && grep -Fxq -- "$CARGO_LINE" "$BASE_TC"; then TC=yes; else TC=no; fi
echo "prereg: toolchain matches baseline: $TC"

# ---- 9. H1-b, UNCONDITIONAL. `binary_sha256` is excluded exactly as `revision`
#         is: it is the ONE line a linked-crate change is guaranteed to move, and
#         the solver's own dependency edge moves Cargo.lock, so a conditional
#         gate on "paths outside the solver" self-disables on the very change it
#         measures. The other 54 lines carry the behaviour. ----
tools/baseline_snapshot.sh --binary "$WORK/with_solver.pistol" --out "$WORK/cand.snapshot" \
  || refuse "the candidate snapshot could not be taken"
# `head -1 … | grep -qx …` was a pipeline and a `grep` where a string comparison
# does the work (SHELL_CHECKLIST items 1 and 3). Read the line, compare the value.
CAND_KIND="$(sed -n '1p' -- "$WORK/cand.snapshot")" || refuse "cannot read the candidate snapshot"
[ "$CAND_KIND" = 'baseline_snapshot 1' ] \
  || refuse "candidate snapshot is not COMPLETE: $CAND_KIND"
inv() {
  local f="$1" n
  [ -s "$f" ] || { echo "prereg: snapshot missing or empty at $f" >&2; return 2; }
  # An empty result is not legitimate here: the marker's ABSENCE is the refusal.
  grep -q '^# timing' -- "$f" || { echo "prereg: no '# timing' marker in $f" >&2; return 2; }
  # `grep -c` prints 0 AND exits 1 (SHELL_CHECKLIST item 3), so `|| true` is load
  # bearing; and because `|| true` would equally mask a failing `sed`, the SPELLING
  # of what came back is validated rather than only its value (item 8).
  n="$(sed -n '1,/^# timing/p' -- "$f" | grep -c . || true)"
  case "$n" in ''|*[!0-9]*) echo "prereg: could not count the invariant block of $f" >&2; return 2 ;; esac
  [ "$n" -ge 50 ] || { echo "prereg: invariant block short ($n) in $f" >&2; return 2; }
  sed -n '1,/^# timing/p' -- "$f" | sed '/^# timing/d' | grep -v '^revision \|^binary_sha256 ' || true
}
inv "$BASE" > "$WORK/inv.base" || refuse "baseline record failed its shape checks"
inv "$WORK/cand.snapshot" > "$WORK/inv.cand" || refuse "candidate record failed its shape checks"
# `echo "… $(wc -l < …) behaviour lines"` is SHELL_CHECKLIST item 1 exactly: the
# substitution's status is the echo's ARGUMENT and an unreadable file prints an
# empty field with exit 0. Take it into a variable, check its shape, and refuse.
INV_LINES="$(wc -l < "$WORK/inv.base")" || refuse "cannot count the baseline behaviour lines"
case "$INV_LINES" in ''|*[!0-9]*) refuse "the baseline behaviour-line count is not a number: $INV_LINES" ;; esac
[ "$INV_LINES" -ge 50 ] || refuse "only $INV_LINES behaviour lines survive the exclusions; the comparison would be vacuous"
echo "prereg: H1-b comparing $INV_LINES behaviour lines (revision and binary_sha256 excluded)"
# Informational only, never a gate: which build-reaching paths outside the solver moved.
OTHER="$(git diff --name-only "$BASE_REV" "$LANDING" -- Cargo.toml Cargo.lock crates configs \
         ':(exclude)crates/pistol-solver' | LC_ALL=C sort)" || true
[ -z "$OTHER" ] || { echo "prereg: note — build-reaching paths outside the solver also moved:";
                     echo "$OTHER" | sed 's/^/prereg:   /'; }
diff -u "$WORK/inv.base" "$WORK/inv.cand"
echo "prereg: H1-b CONFIRMED — every behaviour line is byte-identical"
```

### 3.1a What H1-a's second build actually does, said plainly (m-1, m-2, m-3)

**The "revert" is a REMOVAL now, not a merge-in (m-1).** `git checkout BASE_REV -- <path>`
merges the old files in and leaves every file the work package added on disk — all 14 of them
— restoring only the two that existed at `BASE_REV`. It happened not to matter, because the
restored `lib.rs` stops declaring the modules; but cargo auto-discovers `build.rs`, `src/bin/`,
`benches/`, `examples/` and `tests/` by convention, so a future work package adding any of
those would have them survive the "revert". §3.1 now does `rm -rf -- crates/pistol-solver`
first, so the counterfactual is a genuine absence.

**`--locked` is dropped on the counterfactual build, deliberately and only there (m-2).**
Removing the solver's `pistol-core` edge necessarily moves `Cargo.lock`, so `--locked` would
refuse the build outright. The guard that gives up is replaced by **printing the lock delta**:
the run reports `1 file changed, 3 deletions(-)`, which is exactly the three lines of the
solver's dependency edge and nothing else. A resolution change of any other shape would show a
different delta in the transcript.

**And when `p = 0`, the second build compiles nothing (m-3).** Cargo finds the crate graph
unchanged — the solver is not in it — and returns `Finished release profile in 0.02s`, handing
back **the same file** as the first build: same inode, same mtime. So `[ "$D_WITH" =
"$D_WITHOUT" ]` compares a file's digest to its own. **The mechanism is still sound**, and that
is not a rescue: when the solver *does* reach codegen, cargo rebuilds and the digests differ,
demonstrated at exit 1 with `570dc5d8…` against `1ed322ea…`. But §3's prose used to say
"compares two builds … derived by an independent build", which is not what executes on the
confirming path, and a reader checking the mechanism deserves the true description. **What
H1-a actually asserts is: *cargo, given the tree with the solver removed, produces a binary it
considers identical — and when that is false it rebuilds and says so.***

The candidate toolchain is written to `$WORK/toolchain.cand` and kept as the run's own
provenance record; the comparison itself reads `$BASE_TC` directly with `grep -Fxq`, whole-line
and anchored, so a `1.97.1` prefix cannot match a hypothetical `1.97.10`. Revision 7's
unanchored BRE failed to fire only because the `cargo --version` conjunct carried a full
string — load-bearing and undocumented, and now neither.

### 3.2 Adjudication — every row has a reachable producer, and each was RUN

Revision 7's table had **five of eight rows with no reachable producer**, all downstream of
H1-b's unconditional N/A. That is hardening converting a test into a formality. With H1-b
unconditional the table shrinks to what can actually happen, and §7.2 records the execution of
each.

| exit | Reading | Verdict | produced by |
|---|---|---|---|
| **0** | H1-a digests identical **and** H1-b's 54 behaviour lines identical | **CONFIRMED** | the governed pair, run 3× |
| **1** | H1-a digests differ | **ABORT.** `p` is no longer 0; first diagnostic `cargo tree -p pistol-cli --edges normal` | solver source reaching the binary via a called path |
| **1** | H1-b's behaviour lines differ | **ABORT** — a behaviour change the solver did not cause is still a change the run must not pass over | a `BUCKET_ENTRIES` change in `pistol-search` |
| **2** | any `refuse` | **RUN VOID**, not a verdict in either direction | step 6 at `n = 0` and at `n = 2`; missing/misdigested baseline; truncated record; unbound binding; non-repository cwd; a failed build |
| **2** | `cannot diff crates/pistol-solver between …` | **RUN VOID** — the guard did not answer, which is not the same as answering that the diff is empty | step 5, a `BASE_REV` or `LANDING` git cannot resolve (revision 9) |
| **2** | `build-reaching or instrument paths moved between LANDING and HEAD: …` | **RUN VOID** — the tree is no longer the landing tree where it counts | step 5, any commit above `LANDING` touching `Cargo.toml`, `Cargo.lock`, `crates`, `configs` or `tools` (revision 9) |
| **2** | `untracked files outside the build-reaching set: …` | **RUN VOID** | step 4, now that a clean tree is reachable (revision 9, D-266) |
| — | `toolchain matches baseline: no` | **advisory.** H1-a rebuilds both sides on one toolchain and is immune; it bears only on H1-b | — |
| — | kind token `baseline_snapshot_incomplete` | **RUN VOID** (D-160) | the first-line comparison at step 9 |

**THE STEP NUMBERS IN THIS TABLE ARE NOW THE BLOCK'S OWN (revision 9).** They were not: the
block labelled its sections `0`-`8` while this table and §0.1 cited a `1`-`9` scheme, so a
reviewer chasing `step 6, n = 0` landed in H1-a and a reader chasing "§3.1 step 3" for the
rebuild attestation landed in the cleanliness check. Nothing about the block's behaviour was
wrong; every reference into it was off by one. The block is renumbered to match.

**AND THE EXIT CODES ARE ONLY DISJOINT IF THE TRAP LETS THEM BE (revision 9).** This table
assigns three meanings to three statuses, and revision 8's EXIT trap could return `1` for all
three: `SHELL_CHECKLIST` item 7, measured at `requested 0 → got 1, requested 1 → got 1,
requested 2 → got 1` on the registered construction with an unremovable path. A verdict that
housekeeping can rewrite is not a verdict, and this is the row that would have been silently
false — a CONFIRMED run reported as ABORT, which §6's abort protocol then attaches to. Repaired,
the same three requests return `0`, `1` and `2`.

**WHAT CATCHES AN ACCIDENTAL DEPENDENCY EDGE IS STEP 6, NOT H1 (M-2).** Revision 7's §9 said
the edge *"is the failure it [H1] exists to catch"*. Run: a bare `path` dependency in
`pistol-cli` with no call site is refused at **step 6, exit 2** (the `p = 0` step) —
`2 Cargo.toml files mention pistol-solver, expected exactly 1 … 2 or more means an edge` —
**before H1-a builds anything.** Exit 2 is RUN VOID, the same bucket as a missing baseline, and
§6's abort protocol attaches only to ABORT. The behaviour is right and loud, so nothing ships
silently; the sentence was wrong. **H1-a catches solver content that reaches codegen by any
route** — verified with an `include_str!` of solver source referenced from a called path in
`pistol.rs`, which reached **exit 1, `H1-a FAILED`**, digests `570dc5d8…` against `1ed322ea…`.
An edge with no call site is caught earlier and more cheaply, which is a property of the
ordering rather than a gap.

**`binary_sha256` is excluded from H1-b and printed by H1-a instead.** It is the one line a
linked-crate change is guaranteed to move, so gating on it makes H1-b unreachable; the other 54
lines carry the behaviour — nodes, depths, scores, bestmoves, pv and the three ladders.

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
single combined test gives one wrong answer (`SHELL_CHECKLIST` item 8), so step 4 refuses on a
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
where it was invoked is EXIT-0-WRONG-ANSWER with no bad input at all, and `tools/` at step 9 is a
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
| `tools/bench_delta.sh` on the governed pair | **6 min 18 s** — it does **not** refuse here, because the digests differ | 1 | ≈ 6 min |
| disk, per block run | **61 MB** (`PRISTINE` + `WORK`) | 3 | 183 MB, **now reclaimed by an `EXIT` trap** |

**Revision 7's cost table was wrong in both directions and is corrected (m-4).** It billed
`baseline_snapshot.sh` at 33 s × 4 for a run that took **zero** snapshots, and billed
`bench_delta.sh` at its 9 s refusal path when on the governed pair it does not refuse and runs
**6 m 18 s**. The headline survives — still well under an hour, one workstation — so T4 is
unaffected; but proportionality asks the face of the document to be right about which
instrument costs what. **Also previously unbilled: 61 MB of disk per run, never cleaned.** Ten
accumulated runs had left 605 MB; §3.1 now sets `trap cleanup EXIT` and leaks nothing.

**Machine hours: under 10 minutes, one workstation, single thread. Operator attention: one
invocation, no judgement call during the run. Wall time: under 15 minutes.**

**Revision 9 adds no instrument and no rep, so the cost table stands unchanged.** The four
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

**Second instrument.**

```sh
# Runnable: the same bound-or-refuse form §3.1 uses. Revision 4 printed literal
# a bare `rev:` placeholder in angle brackets, which dies "No such file or directory" — and passed
# `bash -n` only because `<...>` parses as a redirection, so "all four blocks pass
# bash -n" was vacuous for the one block that was not runnable.
BASE_REV=${BASE_REV:-}; [ -n "$BASE_REV" ] || { echo "prereg: BASE_REV unset" >&2; exit 2; }
LANDING=${LANDING:-};   [ -n "$LANDING" ]   || { echo "prereg: LANDING unset" >&2; exit 2; }
tools/bench_delta.sh "rev:$BASE_REV" "rev:$LANDING" 5
```

The baseline side is **the W3 commit**, so both instruments ask the same question across the
W3 digest change. It is genuinely independent and reviewers confirmed why by running it: the
snapshot never builds, while `bench_delta` builds both sides itself in throwaway worktrees, so
instrument 2 **re-derives from source** what instrument 1 **reads from a file** — which is
§3.2's failure mode, covered by construction.

**Agreement criterion, RESTATED for the H1-a / H1-b split (revision 6).** The criterion binds
**H1-b and instrument 2**, because those two compare the same pair — `BASE_REV` against
`LANDING`. H1-a compares a different pair (the landing tree with and without the solver), and
has no second instrument in this configuration; what makes H1-a attributable is its own
construction, two builds differing only in the suspect content.

> **H1-b reports the invariant blocks identical if and only if instrument 2 refuses** with
> `the two sides resolve to the same binary (… digest …)`. **When H1-b is `N/A`, instrument 2
> is expected NOT to refuse**, and its measurement adjudicates nothing about WP-1.5a.

**AND IT PRINTS `VERDICT ABORT` TWICE, MEANING SOMETHING ELSE (M-6).** On the governed pair
`bench_delta.sh` emits, for each band:

> `band early: VERDICT ABORT — nps ratio 1.000 is below the pre-registered 1.15 abort threshold;
> the change is reverted`

That is `bench_delta`'s verdict against **D-215's `Eval::delta` bracket — a different
pre-registration** — and a ratio of 1.000 is exactly what `p = 0` predicts. **Registered here
because §3.2's own vocabulary is `ABORT`, and two ABORTs meaning contradictory things in one
governed transcript is the hazard these rounds have been spent removing.** The rule: **nothing
`bench_delta.sh` prints is a verdict of this document.** Its role here is exhausted by the
refuse-or-not-refuse reading of the agreement criterion; its bands, ratios and the word ABORT
belong to D-215 and are quoted by this document never.

**Run on the real pair, and this is the case that obtains.** Instrument 2 built both sides
itself in throwaway worktrees and reported `cdbcbf0 -> ff018398…`, `7b9e904 -> a7f519fa…` —
**independently reproducing both digests this session obtained from a pristine clone** — and
therefore did **not** refuse. H1-b is `N/A`. Both sides of the biconditional are false, so the
criterion is **satisfied**. Instrument 2's timing numbers at this pair measure the
`pistol-core` `#[inline]` change and **are not evidence about the threat generator**; that is
registered here so nobody quotes them as such. Read from the **message**, never the exit status — that refusal exits 1, and 1
here is the gate working.

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
| step 6, `n = 0` (nothing matches) | **exit 1, stdout empty, stderr empty** → adjudicated ABORT | **exit 2**, `0 Cargo.toml files mention pistol-solver, expected exactly 1` |
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
| H1 | the solver-diff guard, `LANDING` a revision git cannot resolve | **exit 2**, `cannot diff crates/pistol-solver between … — the guard did not answer, which is not the same as answering that the diff is empty` |
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

- **Attempted at the largest `--stat` this repository can produce**: the whole-history diff, 287
  files, 17 879 bytes, well inside a 64 KiB pipe buffer. `git diff --stat … | grep -q .` under
  `set -euo pipefail`, **20 trials, 20 × exit 0.** No SIGPIPE.
- **Control, to show the construction IS defective in kind rather than the probe being wrong**:
  the same construction with a producer that must write past the buffer (`git log -p`, 4 654 354
  bytes), **20 trials, 20 × exit 141** — which under `|| refuse` is a refusal on a non-empty
  result, the EXIT-nonzero-WRONG-ANSWER shape.

So the guard was bounded by an accident of output size and not by anything the document
registered, which is a reason to remove the construction and not a reason to claim it bit. The
exit-128 conflation (§7.3 arm H1) is the half that reproduces, and it is what the repair is
justified by.

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
