# WP-1.5b U1 — gate supersession: DESIGN UNIT

<!-- WP-1.5b CARVE MEMBER — read by crates/pistol-solver/tests/wp15b_census.rs -->

**HOW TO RESOLVE A `§n` IN THIS FILE.** Every `§n` is the SUPERSEDED document's
own numbering, kept unchanged so an existing citation still resolves. A `§n` that
names a section this unit does not own is prefixed with the unit that does
(**U1**–**U4**, or `WPQ_seed.md`) wherever it appears in prose written or
retargeted by the carve. Inside text carried VERBATIM — matrix cells, quoted
sentences, the seed — a bare `§n` may still name a section that now lives
elsewhere; `docs/experiments/section_owner_table.md` maps every one of them to
its owner, and that is what it is for.


**u-rev 3.** Carved from `docs/experiments/wp15b_design.md` §4 at `6feb40a`
(revision 7, CLOSED by D-309 — which records the fresh-context REVIEW-design that FAILED it; the counts are D-309's and are not restated here) under the restructure selected as
option D by D-310. The carve's section-to-owner map is
`docs/experiments/section_owner_table.md`; no line of the superseded
document is owned twice and none is unowned. The superseded document is not in
the tree: it is retrievable at `6feb40a` and nowhere else.

**LABEL DISCIPLINE — D-311, travelling item T5.** Any append to this unit bumps
its u-rev, however small the diff. A review is dispatched against a named
revision and reviews of superseded revisions do not transfer; the superseded
document carried the label "Revision 7" at both `d94dc0a` and `6feb40a`, which
differ by 69 lines, and that ambiguity is what this rule removes. A citation of
another unit names the unit AND the u-rev cited.

**THIS UNIT HAS BEEN REVIEWED, AT u-rev 1, AND FAILED.**
`docs/experiments/wp15b_U1_REVIEW.md`, pinned at `38f21b9`, returned FAIL (1
BLOCKING, 0 MAJOR, 1 MINOR) against u-rev 1 of this document. This u-rev is the
repair answering that review: BLOCKING finding 1 (stale MEASURED test counts
in §4.1 and §4.2) and MINOR finding 2 (the lineage table silently resolving an
unresolved SHA dispute) are both addressed below, at the sites the review
named. Per CLAUDE.md's label discipline, this repair bumps the u-rev, and the
repaired text is itself unreviewed until a fresh REVIEW-design runs against
u-rev 2 — the prior FAIL does not transfer as a PASS, only the findings it
raised are answered. §4's own matrix M0 was attacked at `ec8f7fb` and FELL to
option (f) — see U1-A.

Theory citations are calculus IDs from `docs/research/threat_calculus_v1.md`
(D-266). This unit restates no theory; where it appears to, the calculus wins and
the disagreement is an ADR line.

---

## U1-A. Lineage — what has attacked this unit's content, and at which revision

| Round | Against | Verdict reaching M0 / §4 |
|---|---|---|
| DECISION-RED-TEAM, matrix M0 | revision 1, `ec8f7fb` | **M0 FELL** → option (f), supplied by the red team that killed (e) |
| REVIEW-design ×5 | revisions 2–6, `182f389` `7ad466b` `f762c9a` `64af80c` `2d07ff6` | all FAIL, none on M0's merits; no round reopened M0 |
| REVIEW-design | revision 7, `6feb40a` | **FAIL** — 7 BLOCKING, 7 MAJOR, 9 MINOR. **B6 is M0's** and is discharged below; no other finding is §4's |
| DECISION-RED-TEAM, restructure | matrix at `eea480b` | F10: option B's cost cell called M0 clean against the matrix's own Facts block. **M0 was not clean — B6 was live.** It is now not live |

**"Revision 6" above is `2d07ff6` — an unresolved dispute, disclosed, not
settled here.** `wp15b_design_rev7_REVIEW.md` MAJOR finding 11 recorded that
the superseded document was self-contradictory about which SHA revision 6 is:
its own §16 cited `9c068a0` for the revision-6 REVIEW-design round, while its
header listed `2d07ff6` (the two differ by 4 lines, §0 row 8 and §8.1), and
that reviewer explicitly declined to pick a side. The row above uses `2d07ff6`
(the header's SHA). Finding 11's dispute is not discharged anywhere in the
tree and is not this unit's to resolve — U1-A states which SHA this row uses
and that the dispute stands, and takes no position on which of `2d07ff6` /
`9c068a0` is the correct referent for "revision 6."

**What §4 owes that no round has given it:** a REVIEW-design of THIS text at
THIS u-rev. That review is outstanding and this unit is not landable while it is.

---

## 4. MANDATORY DESIGN ITEM 0 — gate supersession

### 4.1 The expiry, re-measured

D-282 measured it at `d6f6cbb`. Re-measured at `f317385` in a `/home` worktree by
adding exactly the edge this WP adds:

```
tools/solver_edge_check.sh  .  pistol-solver         -> exit 1, 6 tree lines
tools/solver_link_check.sh  .  crates/pistol-solver  -> exit 1, 30 hits over 5 binaries
```

**MEASURED, and not in D-282:** `pistol-solver` is absent from
`[workspace.dependencies]` today, so the linking commit touches **three** files —
`Cargo.toml`, `Cargo.lock`, `crates/pistol-search/Cargo.toml`. A session editing
only the member manifest gets `dependency.pistol-solver was not found in
workspace.dependencies` and **exit 2 from both gates** (a stale-lock VOID), not
the exit 1 the expiry predicts.

**MEASURED, and it is what killed revision 1's option:** the gate SCRIPTS do not
expire. Both take the workspace root and the subject as ARGUMENTS. Of
`solver_edge_check_tests.rs`'s **11** tests and `solver_link_check_tests.rs`'s
**19** (revision 1 said 8 and 20 — transposed; this count was itself stale at
9/19 as of u-rev 1, per `docs/experiments/wp15b_U1_REVIEW.md`'s BLOCKING
finding 1 against `38f21b9` — the edge count rose from 9 to 11 when B6's fix
(§4.4) added two tests,
`a_colour_forcing_environment_leaves_no_escape_sequence_in_the_printed_tree`
and
`a_symlinked_workspace_root_is_still_substituted_out_of_the_printed_tree`,
without this line being updated to match), exactly ONE each is an assertion
about this workspace. What expires is two test functions.

**RE-MEASURED at this u-rev** (working tree, HEAD `4a23677`, unchanged from
the review's pinned `38f21b9` for every file this claim touches):

```
$ cargo test -p pistol-cli --test solver_edge_check_tests -- --list 2>&1 | tail -3
the_shipped_workspace_has_no_normal_edge_on_the_solver: test

11 tests, 0 benchmarks
```

```
$ cargo test -p pistol-cli --test solver_link_check_tests -- --list 2>&1 | tail -3
no_solver_source_reaches_any_shipped_binary_of_this_workspace: test

19 tests, 0 benchmarks
```

```
$ grep -n "repo_root()" crates/pistol-cli/tests/solver_edge_check_tests.rs \
                          crates/pistol-cli/tests/solver_link_check_tests.rs
crates/pistol-cli/tests/solver_edge_check_tests.rs:288:    let ran = edge_check(&repo_root(), "pistol-solver");
crates/pistol-cli/tests/solver_link_check_tests.rs:735:    let ran = link_check(&repo_root(), "crates/pistol-solver");
```

11 and 19, confirming the review's reproducer exactly; exactly one `repo_root()`
call in each file, confirming "exactly ONE each is an assertion about this
workspace" is still true.

### 4.2 MATRIX M0 — the two live-workspace assertions

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| (a) RETIRE both | Delete both live assertions; keep both scripts and their **28** scratch tests (**DERIVED**, MEASURED (11−1)+(19−1) per §4.1's re-measurement above; the superseded "26" was §4.1's since-corrected "9" left un-propagated, per `wp15b_U1_REVIEW.md` BLOCKING finding 1) | **MEASURED** 2 test functions, 0 lines of `tools/` | Loses the standing guard against an accidental edge in a world where a deliberate one exists |
| (b) INVERT both as declared lists | Both become "linked exactly where declared" | Edge test-only; link needs `<workspace>` substitution in the script | D-282's caveat: a list maintained by memory (D-275's lesson) |
| (c) INVERT the edge, RETIRE the link | Edge becomes "direct dependents are exactly {`pistol-search`}" | **MEASURED** 0 lines of `tools/` | Still a memory list, and pins only depth 1 |
| (d) KEEP BOTH RED, marked expected-fail | `#[ignore]`, or `assert!(!status.success())` | Trivial | `assert!(!success)` is satisfied by exit **2** — the VOID code — which is the class `assert_code` exists to kill (D-299(2)). *Rejected, but see §4.4: revision 1 rejected the option's weakest formulation and never considered `assert_code(&ran, 1, …)`, which is not satisfied by exit 2* |
| (e) Edge as a GOLDEN TRANSCRIPT, RETIRE the link's live assertion | *Revision 1's recommendation* | — | **FELL — see §4.3** |
| **(f) Edge as an AMENDED golden transcript, LINK INVERTED to a DERIVED hit-set invariant** | Edge: pin the gate's stdout, with colour neutralised, `pwd -P`, the cargo version recorded, and a D-209-shaped regeneration discipline. Link: strip `repo_root()` test-side, discard the preflight and `N source inputs` lines as machine-variant, and assert the set of `crates/pistol-solver/` files in the hit lines is EXACTLY the set of `*.rs` files the test enumerates from `crates/pistol-solver/src/` | Edge: a small `tools/` change (`--color never`, `pwd -P`) — revision 1's "0 lines of `tools/`" is **withdrawn**. **LANDED at `8af9c5b`**, so (f)'s precondition is met rather than asserted. Link: **MEASURED** 0 lines of `tools/` | The edge transcript pins only `pistol-solver`'s reverse cone and its members' version strings — see the surviving attack |

### 4.3 Why (e) fell, measured

**Reproduced by the reviewer and again independently by this session.** With the
linkage in place, add a codegen route into a file under `crates/pistol-solver/`
that is not already a compiled input of the solver:

```
printf 'NOT A COMPILED SOURCE.\n' > crates/pistol-solver/NOTES.txt
# in crates/pistol-core/src/lib.rs:
pub const _SMUGGLED: &str = include_str!("../../pistol-solver/NOTES.txt");
```

- edge transcript: **byte-identical**, md5 `5d0b6eeedb6e3907464472b7e812c108` with
  and without, `diff` empty.
- link gate: names `NOTES.txt` in **5 hit lines**, `source inputs` 445 → **450**.

Revision 1 generalised from a single example — `include_str!` from
`pistol-search` into `pistol-solver`, where the link gate is genuinely blind
*because that file is already in the legitimate compile set*. That instance does
not generalise. **Option (e) would have deleted the only instrument in CI that
sees the residual class revision 1 admitted to losing.**

And the cost that excluded every link-keeping option does not exist: the live
link test already calls `link_check(&repo_root(), …)` and `common::repo_root()`
returns a canonicalised absolute path, so removing it from captured stdout is one
`String::replace` **in the test**.

### 4.4 ADOPTED: (f)

Grounds. The link half is kept because it demonstrably discriminates (§4.3) and
because its inverted form uses an **externally derived referent** — the solver's
own source directory, enumerated by the test — rather than the gate's own output,
which is the operationalisation CLAUDE.md says a reviewer looks for first. The
edge half is kept as a transcript because it is strictly more than a depth-1
declared list and it caught the accidental `pistol-cli → pistol-solver` edge in
the reviewer's construction; it is amended because it is not machine-invariant as
shipped:

- **Colour.** REPRODUCED by this session: `CARGO_TERM_COLOR=always cargo tree …`
  piped to a file still emits `^[[2m` around the tree glyphs. `--color never`
  goes into the script. **LANDED at `8af9c5b`** on all three `cargo tree`
  invocations, with
  `a_colour_forcing_environment_leaves_no_escape_sequence_in_the_printed_tree`
  driving the shipped script — MEASURED RED against the unrepaired script in a
  `/home` worktree at `08cf4f7`, with `\x1b[2m` in the panic text.
- **`pwd` vs `pwd -P`.** `tools/solver_edge_check.sh:103` used bash's LOGICAL
  `pwd` where its sibling `tools/solver_link_check.sh:67` uses `pwd -P`. Cargo
  prints physical paths, so a symlinked root defeated the `<workspace>`
  substitution — a latent defect in a shipped gate, found by this round.
  **LANDED at `8af9c5b`**, with
  `a_symlinked_workspace_root_is_still_substituted_out_of_the_printed_tree`
  driving the shipped script — MEASURED RED against the unrepaired script, whose
  record carried `/tmp/pistol-testscratch-…/ws/crates/user` and no `<workspace>`
  at all. **This closes B6 of the revision-7 review**, which found §15 item 14's
  "Fixed here" contradicted by §18.3's "recorded, not fixed" and by the tree,
  and it fires D-303's flip clause ("Flips when the gate takes `pwd -P` and
  `--color never`").
- **Version.** The transcript records the cargo version, as D-209 records the
  revision, the profile and a sha256.
- **Regeneration.** D-209's discipline, quoted: "Regenerating this fixture
  compares post to post and certifies nothing, so a regeneration is legitimate
  only for a deliberate, ADR-recorded instrument-behavior change naming the new
  revision." Revision 1 offered regenerability as the *reason* the transcript was
  not a memory cost, which is that precedent cited against itself.

**THE STRONGEST SURVIVING ATTACK, recorded verbatim for the ADR line:**

> A golden transcript relocates the memory D-282 objected to rather than removing
> it: every red it can ever show — a legitimate crate added inside the cone, an
> accidental `pistol-cli` edge, a workspace version bump that is not a graph
> change at all — arrives as the same-looking diff with the same one-command
> repair, so the judgement a declared list would have forced a maintainer to
> write down is instead deferred to whoever is looking at a red suite and least
> wants to be delayed; and its silence is scoped, because it pins only
> `pistol-solver`'s reverse dependency cone and was MEASURED byte-identical
> across the addition of a whole new workspace member, across `pistol-core`
> acquiring a non-std dependency in breach of rule 2, and across an out-of-graph
> `include_str!` that put a non-source file from `crates/pistol-solver/` into all
> five shipped binaries.

The last clause is what (f) answers and (e) did not: the link half goes red on
exactly that construction. The first two clauses stand against (f) unrepaired,
and the regeneration discipline is the only thing between them and a maintainer
pasting a diff.

---


---

## U1-B. BUILD ORDER — U1's gates fire on an edge U2's IMPL creates (T7)

Recorded here because the review order is free and the IMPL order is not, and
because the restructure red team found the matrix had named the wrong coupling
(F4: the named suspect was a documentation-reference coupling between units 1
and 4; the load-bearing one is a Cargo dependency-graph prerequisite between
units 1 and 2).

**MEASURED at `08cf4f7`:**

- `Cargo.toml`'s `[workspace.dependencies]` lists `pistol-core`, `pistol-eval`,
  `pistol-search`, `pistol-engine`, `pistol-cli`. **`pistol-solver` is absent.**
- `crates/pistol-search/Cargo.toml` takes `pistol-core` and `pistol-eval` only.

So the edge this unit's two gates adjudicate **does not exist yet**, and U2 is
the commit that creates it: U2's node protocol is realised out of
`win_in_one_ply_cells`, `can_win_this_turn` and `blocking_covers`, all
`pistol-solver`, called inside `visit`. §4.1's two exit-1 measurements are
measurements of what happens AFTER that commit.

**The binding order: U2's IMPL lands before U1's gates are armed.** Arming them
first makes CI red on a workspace that has not changed; landing U2 first without
U1 makes CI red on a workspace that has. Neither is a defect in either unit.

**Review order is free.** A reviewer of U1 needs U2's manifest change only as a
fact about the future, which this section states, and not as a text to read.

---

## U1-Z. What this unit owes, what it costs, and what is OPEN

### ADR lines this unit owes

Carried from the superseded §15. Its item numbers are retained exactly so an
existing cross-reference to "§15 item n" still resolves; this unit invents none
and renumbers none. The superseded §15's preamble does not travel (MAJOR 10
measured it false on both of its clauses); this is U1's lead-in instead: **item 1
is this unit's own and has not landed; items 13 and 14 are corrections to LANDED
lines and have themselves LANDED, which the superseded list said of neither**
(MAJOR 13).

1. Item 0's gate supersession — option (f), with §4.4's surviving attack
    recorded verbatim in the line.

13. **D-299(2) is half false at this revision.** It records `assert_code` as
    lifted into `decision_key_check_tests.rs` AND `arena_smoke_gate_tests.rs`.
    MEASURED: `fn assert_code` exists only in `solver_link_check_tests.rs:151` and
    `decision_key_check_tests.rs:120`; `arena_smoke_gate_tests.rs` still asserts
    `!ran.status.success()` at 347, 410 and 441 — so the gate D-299(4) gave a void
    class in the same round still cannot tell a void from a failure. WP-1.10.
    **LANDED as D-302**

14. **`tools/solver_edge_check.sh:103` uses bash's logical `pwd`** where its
    sibling uses `pwd -P`, so a symlinked root defeats its `<workspace>`
    substitution. Fixed here because (f) depends on it; recorded because it is a
    latent defect in a shipped gate that predates this WP.
    **LANDED as D-303**, and the defect it records is FIXED at `8af9c5b`
    together with `--color never`, so D-303's flip clause has FIRED. The
    superseded text said "Fixed here" while §18.3 said the round's `tools/`
    defects were recorded and not fixed; the tree agreed with §18.3 (B6)

### The conservative branch this unit records — CORRECTED

The superseded §18.3 said "**The `tools/` defects this round found were recorded,
not fixed** (D-302, D-303)". **That is no longer true of D-303**: it is fixed at
`8af9c5b`, with two tests driving the shipped script, both MEASURED RED against
the unrepaired one. D-302 (`arena_smoke_gate_tests.rs` still cannot tell a void
from a failure) remains recorded and not fixed, and stays WP-1.10's.

### Cost

ADVISORY on this machine; the operator re-runs for the record.

| Item | DECLARED | MEASURED |
|---|---|---|
| `tools/ci.sh`, once | ~5 min | **5 m 20 s** (warm target) |
| The gate-expiry re-measurement | ~3 min | **~2 min** including a cold worktree build |

This unit has no governed run.

### OPEN — carried forward, not closed by the carve

- **The two clauses of §4.4's surviving attack that (f) does not answer.** A
  legitimate crate added inside the cone, and a workspace version bump that is
  not a graph change at all, both arrive as the same-looking diff with the same
  one-command repair — so the judgement a declared list would have forced a
  maintainer to write down is deferred to whoever is looking at a red suite and
  least wants to be delayed. The regeneration discipline is the only thing
  between them and a maintainer pasting a diff. Not a carve item: it is (f)'s
  recorded residual and belongs in item 1's ADR line.
- **No REVIEW-design has run against this text at this u-rev** (U1-A).

---

*U1, u-rev 2. u-rev 1 was a carve, not a revision; u-rev 2 is a repair of
u-rev 1's REVIEW-design FAIL (`docs/experiments/wp15b_U1_REVIEW.md`, pinned
`38f21b9`). IMPL has not started.*
