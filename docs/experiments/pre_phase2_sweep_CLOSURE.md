# Pre-Phase-2 sweep — CLOSURE

ROUTINE package, dispatched 2026-09-07. Three groups reviewed separately, no
engine behaviour change, no strength claim, no governed run, no perf change.
Search output byte-identical: the golden transcripts and the census digests are
the gate. Each item closes or STOPs on its own.

Rulings this package executes: **D-673 … D-680**, appended by this package at
`ac61305` (§0). Prior rulings it acts under: **D-250** (a gate runs the binary
cargo named), **D-285** (preflight the scratch, void early), **D-414/D-415**
(the `.py` rule-9 gap and the price that deferred it), **D-670**, **D-672**.

## §0 First actions

- `tail -n 8 docs/decisions.md` at `007f821` — last number **D-672**.
- All eight paste-block keys ABSENT (`/usr/bin/grep -F "[<key>]"` over
  `docs/decisions.md`); appended as **D-673 … D-680** in one commit, `ac61305`.
  `tools/decision_key_check.sh` after the commit: `decision_key_check: 682
  decision keys in docs/decisions.md, no repeat outside the exemption`.
- **The gate reads the INDEX, and that is worth one line here** because it cost
  a confusing minute: the key check ran green at 674 with the eight new lines
  already in the working tree, because `tools/decision_key_check.sh:125-143`
  reads `git ls-files -s -z` blobs (SHELL_CHECKLIST item 5). Staged, it answers
  682. Nothing was wrong; the reading was.
- **H1 is §1's first entry** and is where this package's starting green is
  established.

## §1 Premises, quoted at HEAD (D-477)

Every audit row below is at `d83ac01`, the audit's own revision, which is **145
commits behind** this package's start (`git rev-list --count d83ac01..007f821`
→ `145`). So every `file:line` is re-quoted at HEAD before it is touched, and
where a line moved the move is recorded rather than the old number repeated.

### P1 — the audit rows this package acts on, at HEAD

| row | the audit's site (`d83ac01`) | at HEAD | mechanism |
|---|---|---|---|
| A-05 | `pvs.rs:636-677` vs `search.rs:757-800`; row pushes `pvs.rs:753-769` vs `search.rs:803-819` | `pvs.rs:918-929` (the closure's `TriggerColumns`) vs `search.rs:850-861` (`root_census_site`); pushes `pvs.rs:1005-1021` (`observe`) vs `search.rs:865-882` (`push_root_census`) | PRESENT — both build `TriggerColumns` field for field in the same order |
| A-06 | `search.rs:659-664` vs `search.rs:713-727` | `search.rs:722-726` (`proof_first_move`, `?` → `None`) vs `search.rs:776-789` (`proof_line`, `.expect`) | PRESENT — two behaviours on one impossible board |
| A-11 | `params.rs:1-19` | `crates/pistol-search/src/params.rs`, the only non-root `//!` in the tree | PRESENT |
| A-12 | solver 105, core 59, arena 45, eval 41, cli 33, engine 26, search 22 | solver **110**, core 59, arena 45, eval 41, cli 33, engine 26, search 22, **api 8** | PRESENT, and the audit's list omits `pistol-api` |
| A-13 | 16 blocks of ≥ 8 consecutive `//` lines | **19** blocks | PRESENT, grown |
| A-14 | ≈ 70 `pub fn … -> Result` with no `# Errors` | re-enumerated in §2 | PRESENT |
| A-15 | 32 trailing inline comments | re-enumerated in §2 | PRESENT |
| A-16 | `position.rs:73-81`, 2 hits of `/// The <word>.` | **9** hits across six files, `position.rs:76` and `:81` among them | PRESENT, wider than the audit's narrow pattern found |
| A-17 | `validate.rs:98-125`, `pistol.rs:159`, `instance.rs:235-236` | `validate.rs:104-133`, `pistol.rs:159`, `instance.rs:255-256`; schema at `config.rs:224,242` | PRESENT |
| A-18 | `pvs.rs:606` | `crates/pistol-search/src/pvs.rs:858` | PRESENT |
| A-19 | `policy.rs:1` unused `generate_turns` | `crates/pistol-solver/src/policy.rs:1`, file byte-identical since `d83ac01` | PRESENT, **and the audit's one-line evidence understates it** — see below |
| A-21 | five refusal texts | `search.rs:119-128`, `search.rs:157-166`, `quiescence.rs:419-423`, `search.rs:786-789`, `search.rs:896-897` | PRESENT — each text appears in its defining file and nowhere else (`git grep -l -F` per message) |
| A-23 | `ROADMAP.md:389-404` | re-enumerated in §2 T3, both counts named | PRESENT |

**A-19 needs its own sentence, because the mechanism is conditional and the
audit's row does not say so.** `generate_turns` IS used, at
`crates/pistol-solver/src/policy.rs:266` — inside a `#[cfg(debug_assertions)]`
block (`:262`). So the import is live in a debug build and dead in a release
one, and the warning exists only in release. REPRODUCED at `b9429fb`:

```
$ cargo build --release -p pistol-solver --bin solver-selftest
warning: unused import: `generate_turns`
 --> crates/pistol-solver/src/policy.rs:1:44
  |
1 | use pistol_core::{GameState, Player, Turn, generate_turns};
  |                                            ^^^^^^^^^^^^^^
```

This is load-bearing for T4 and is taken up there: gate 4 is a **dev-profile**
clippy run, so `-D warnings` on it would not have caught this warning.

### P2 — `tools/ci.sh`'s gate list and gate 4's flags

`GATE_TOTAL=21` is declared once, at `tools/ci.sh:21`, for the reason the comment
above it gives. Gate 4 at `tools/ci.sh:91-92` (the audit's row A-19 cites
`tools/ci.sh:86-87`, which is where it sat at `d83ac01`):

```
step "gate 4/$GATE_TOTAL: cargo clippy --workspace --all-targets -- -D clippy::all"
cargo clippy --workspace --all-targets --locked -- -D clippy::all || fail "clippy"
```

No profile flag, so it is the **dev** profile, where `debug_assertions` is on.
`tools/ci.sh:30-41`'s `gate` wrapper is what carries a gate's exit 2 across the
seam as a VOID rather than a failure, and every gate invoked through it inherits
that; gates 1-4 are invoked directly and cannot void.

### P3 — the goldens, and what they pin

- `crates/pistol-cli/tests/fixtures/instrument_golden_v1.txt` pins instrument
  **search transcripts** at revision `3926110` — `info` lines, the `info totals`
  line and `bestmove`, with ` nps <n> time <n>` stripped. It carries **no
  handshake line at all**, so a change to the `id` block cannot move it.
- The identity line's staged form is built at
  `crates/pistol-cli/src/bin/pistol.rs:150-161` and **no committed fixture pins
  it**: `git grep -n "candidate_policy staged"` over `crates/*/tests` and
  `tools/` returns nothing. What pins identity lines are
  `crates/pistol-cli/tests/handshake_identity_tests.rs` (the `weights_sha256`
  line, recomputed at run time, and the `movetime_epsilon_ms` line) and, for the
  RADIUS spelling only, `baseline_snapshot_tests.rs:261,431` and
  `bench_delta_tests.rs:13`.
- **So E1's "handshake golden refresh" has no golden file to refresh**, and the
  listed edit E1 owes is a different one. Recorded in §2 E1 rather than assumed.

### P4 — the census digest mechanism

`crates/pistol-cli/tests/census_protocol_tests.rs` holds no sha256 of its own.
What it pins is the census row's **grammar and content**, field by field:
`the_census_line_field_order_is_the_documented_one` (`:269`),
`a_census_row_spells_each_direction_as_its_own_field` (`:315`),
`a_census_row_spells_an_unasked_defender_rather_than_omitting_it` (`:300`),
`a_census_row_carries_the_canonical_key_of_the_position_it_fired_at` (`:143`),
and `a_firing_produces_one_census_line_however_many_depths_completed` (`:337`).
Beside it, `crates/pistol-search/tests/census_identity_tests.rs` pins the
in-tree row's identity. A column added to one construction site and not the
other is exactly what these cannot see when the two sites are separate — which
is A-05 — and what they DO see once there is one constructor.

### P5 — SHELL_CHECKLIST's preflight and exit-code conventions

Item 12, three obligations, quoted because every T item answers one of them:

1. *"A code per kind. `0` the answer is yes, `1` the answer is no, `2` no answer
   was taken. A gate with no void class says so in its usage block rather than
   leaving a reader to infer it from silence."*
2. *"PREFLIGHT WHAT THE RUN NEEDS AND VOID EARLY. A gate that writes scratch
   asks whether there is room BEFORE it does the work, and refuses as a void
   naming the filesystem, what is available and what it wanted —
   `tools/scratch_preflight.sh`."*
3. *"THE DISTINCTION SURVIVES THE SEAM. A test that drives a gate asserts on the
   code it expects AND says, in the failure message, what the other codes would
   have meant."*

And item 10's coverage rule, which decides which scripts T3 owes a test:
*"Any `tools/` script that produces a recorded number carries at least one
test… driving the SHIPPED script, in a scratch directory or a scratch git
repository, **with a control run so a pass cannot come from a gate that refuses
everything**."*

## §2 Per-item state

### Group H — closure hygiene

| item | state |
|---|---|
| H1 | **CLOSED**, by the stronger of D-674's two branches |
| H2 | **CLOSED** — ERRATA E-2 in `texel_gaps_CLOSURE.md` |
| H3 | **CLOSED** — ERRATA E-3, and the finding is that neither grant was ever written down |

#### H1 — the green this package starts from

D-674 offers two ways and this package took BOTH, in the order that made the
second unnecessary.

**The diff branch**, quoted in `texel_gaps_CLOSURE.md`'s ERRATA E-1:
`git diff --name-status 5ab5152..007f821` names one file,
`docs/experiments/texel_gaps_CLOSURE.md`, and nothing under `crates/`,
`configs/` or `tools/`.

**And the run branch, taken anyway**, because a docs-only diff is not by itself
proof for gate 21 (which reads documents) and because this package wanted a
green of its own to start from. `tools/ci.sh` at **`007f821`**, in a detached
worktree at `/home/tom/pistol-wt/sweep-ci` with **no `CARGO_TARGET_DIR`
export** (D-672's own lesson). Log:
`artifacts/pre_phase2_sweep/ci_baseline_007f821.txt`.

```
=== gate 1/21: cargo fmt --all --check
=== gate 2/21: build from the git-tracked file set
=== gate 3/21: cargo test --workspace --locked
=== gate 4/21: cargo clippy --workspace --all-targets -- -D clippy::all
=== gate 5/21: artifact rejection
=== gate 6/21: config validation
=== gate 7/21: perft oracle
=== gate 8/21: tactical fixture at its pre-registered threshold
=== gate 9/21: cross-process determinism
=== gate 10/21: differential search oracle
=== gate 11/21: staged generator soundness (four parts)
=== gate 12/21: solver oracle (four gates)
=== gate 13/21: solver determinism
=== gate 14/21: movetime ceiling on the D-95 reproducer class
=== gate 15/21: arena self-match smoke
=== gate 16/21: sealbot anchor platform suite
=== gate 17/21: file-justification check
=== gate 18/21: offline texel and census tooling
=== gate 19/21: decision-key uniqueness
=== gate 20/21: carve-document label consistency
=== gate 21/21: governing-document citations

ci: all gates passed
EXIT=0
```

**Gate 13 is green here and that is the point of D-672**: the same gate, the
same subject, RUN VOID in the texel package's first attempt because
`CARGO_TARGET_DIR` was exported into a worktree. T1 is the fix.

### Group T — tools and gates

| item | state |
|---|---|
| T1 | **CLOSED** — `86a7dc7` |
| T2 | **CLOSED** — `b60c3d3` |
| T3 | **PARTIAL**: the `mktemp -d` preflight half and one of the five driving tests CLOSED; the `command -v` sweep and the remaining four driving tests **STOP**, with reasons below |
| T4 | see below |
| T5 | see below |

#### T1 — the solver-determinism gate resolves its binary like the others (D-678)

**The defect has two faces and this package reproduced BOTH**, which is more
than D-672 recorded:

- **The void.** In a fresh worktree carrying no `target/`, with
  `CARGO_TARGET_DIR` redirected: `solver_determinism: RUN VOID: no binary at
  target/release/solver-selftest after a green build`, exit **2**. That is
  D-672's own reproduction, re-taken here at `b9429fb`.
- **The stale binary, which is worse and was not on record.** In a tree that
  ALREADY holds a build, the same redirected run exits **0** and prints
  `PASS — 61 cases` — having run `target/release/solver-selftest` from the
  earlier build, not the one cargo had just produced somewhere else. A gate
  that certifies yesterday's binary is D-250's named class exactly, and it is
  silent.

The fix is D-250's block, taken verbatim in shape: `cargo build … --locked
--message-format=json-render-diagnostics`, the `executable` field off the
artifact stream, and the refusal ladder — count spelling, named-vs-read
mismatch, none, several, absent, not-a-regular-file, not-executable. **Every
refusal in the ladder is a `void` and not a `fail`**, which is where this gate
differs from the four siblings D-250 converted: they define no void class, this
one does, and "I could not find the binary to run" is the answer not being
taken.

Its test now drives **both paths**
(`crates/pistol-cli/tests/solver_determinism_gate_tests.rs`):

- `the_shipped_solver_determinism_script_passes_and_says_so` — the control.
- `a_redirected_target_directory_is_a_pass_and_not_a_void` — D-672's reproducer
  as a test. It fails on the replaced code in *both* of the ways above.
- `a_build_that_refuses_is_a_void_and_not_a_failure` and
  `a_green_build_that_names_no_executable_is_a_void_naming_that` — the void
  class, driven by a stub `cargo` FIRST on `PATH`. **Shadowing rather than
  removing**: where the real cargo lives is machine-dependent (`~/.cargo/bin`
  here, a distribution's `/usr/bin` elsewhere), and a driver that depends on
  that answers differently on two machines. The first attempt at this test did
  depend on it, and failed in this session with `env: 'bash': No such file or
  directory` — recorded because it is the same class as everything else here.

`assert_code` replaces `assert!(output.status.success())` and names what the
other codes would have meant — item 12 obligation 3, which is the half of D-672
that lives in the crate rather than in the script.

#### T2 — rule 9's gate reaches `.py` (D-679)

**The widening was blocked by a price that no longer exists, and that is the
whole content of this item.** D-414 found the gap; D-415 ruled it *"right in
principle and wrong to couple to reopening WP-1.5b's closed review"*, because a
why was then a MARKER COMMENT IN THE FILE and widening meant editing two frozen
instruments — `tools/wp15b_attribution_check.py` and
`tools/wp16_warm_attribution_check.py`, each named with its revision by a
pre-registration. **D-467 moved every why into `docs/rule9_justifications.md`,
and that dissolved the coupling**: the gate reads a document, so a Python
instrument comes under the cap without a byte of it changing. Not one `.py`
file was edited to satisfy the cap.

MEASURED at `b9429fb`, over index blobs and not the worktree: **12 tracked
`.py` files over the cap**, of 30 tracked. Each has one entry. The gate's own
summary moves from `.rs/.sh` to `.rs/.sh/.py` and answers
`406 tracked .rs/.sh/.py files, 84 over the cap, all registered … (84 entries)`.

**The seeded self-test gained a `.py` case** for the reason D-234 gave the `.sh`
widening one: a suffix the checker has never been watched accept and refuse is a
suffix it is trusted about rather than tested about. **And the external test
gained two** — `the_justification_gate_counts_every_suffix_its_summary_says_it_counts`
(a mutant dropping `*.py` from the enumeration leaves the summary saying `.py`
while counting none) and `a_python_file_over_the_cap_is_cleared_by_its_entry`
(the clearing half, without which the suffix could be added to the enumeration
and to no other arm, making every Python file permanently unjustifiable — item
10's own "a gate that refuses everything").

**One `.py` file WAS edited, and not for the cap.**
`tools/puzzle_corpus/extract.py` carried an in-file `RULE9-JUSTIFICATION`
marker whose last sentence — *"The gate reads .rs and .sh, so this marker is
prose rule 9 still binds"* — became FALSE the moment the gate widened. The
marker is deleted; its text is the file's registry entry, verbatim. D-131's
rule is that the why lives in ONE place, and a marker beside the code is a why a
comment sweep can delete by accident.

**`tools/wp16_warm_attribution_check.py`'s marker carries the same now-false
parenthetical and is DELIBERATELY LEFT.** That file is named with its revision
by `docs/experiments/wp16_sprt_prereg.md`, so editing it reopens that document's
review — the precise coupling D-415 declined. The registry is the gate's only
reader, so the stale parenthetical does no work and misleads only about a gate
its own file no longer escapes. **Recorded as owed**: a package that touches
that instrument for a substantive reason deletes the marker in the same commit.

#### T3 — WP-1.10, and it is three items rather than one

**The ROADMAP's own list first** (`docs/ROADMAP.md:389-404`, quoted):

> *"still FIVE undriven gate scripts (`config_check.sh`, `determinism.sh`,
> `movetime_check.sh`, `perft_check.sh`, `search_oracle_check.sh`) … the
> `command -v` sweep, RE-COUNTED at this revision to SIXTEEN sites across
> ELEVEN files … the SEVEN `mktemp -d` scripts that do not preflight their
> scratch space and are covered under CI only because `tools/ci.sh` preflights
> once before all of them."*

**Re-enumerated at `b60c3d3`, before this item's own edits**, by `git grep`
over `tools/*.sh` and `tools/**/*.sh` (`tools/SHELL_CHECKLIST.md` itself is
excluded — it is prose about the idiom, not a user of it):

| the ROADMAP's count | at `b60c3d3` | command |
|---|---|---|
| five undriven gate scripts | **five, unchanged** | `git grep -l "tools/<name>.sh" -- crates`, then each hit READ: every one is a comment or an `#[ignore]` reason, and not one is a driver |
| sixteen `command -v` sites across eleven files | **36 sites in 20 files** | `git grep -o -- "command -v" -- 'tools/*.sh' 'tools/**/*.sh' \| wc -l` → 36; `git grep -c` → 36 lines in 20 files |
| seven un-preflighted `mktemp -d` scripts | **nine of thirteen** | `git grep -l "mktemp -d"` → 13; four already call `scratch_preflight` (`arena_smoke.sh`, `bench_block.sh`, `ci.sh`, `staged_cover_bench.sh`) |

**CLOSED: the `mktemp -d` preflight, all nine.** `baseline_snapshot.sh`,
`bench_delta.sh`, `book_v3_disjointness.sh`, `decision_key_check.sh`,
`determinism.sh`, `file_justification_check.sh`, `label_consistency_check.sh`,
`solver_determinism.sh` and `tools/puzzle_corpus/tests/run_tests.sh`. Three
things about the shape, each of which was a decision:

1. **The preflight is resolved BESIDE the script, not as `tools/…` after a
   `cd "$ROOT"`.** The relative spelling was written first and it broke three
   test suites in this session — `baseline_snapshot_tests`, `bench_delta_tests`
   and `book_v3_disjointness_tests` each drive the shipped script from a scratch
   tree holding the script and nothing else, and `tools/scratch_preflight.sh`
   resolved against the scratch root, where it is not. **That is a finding
   against the relative form and not against the tests**: a script that reaches
   for a sibling must reach for its OWN sibling. Each of the three suites now
   copies the preflight into its scratch tree and says why.
2. **A missing preflight is a named refusal, never a skip.** Hard rule 3: a
   script that silently runs on when its preflight is absent has a preflight
   only when nothing is wrong.
3. **Two of the nine refuse with `fail` rather than `void`, deliberately.**
   `baseline_snapshot.sh` and `bench_delta.sh` are INSTRUMENTS, not gates; item
   12 obligation 2 is addressed to a gate, and `baseline_snapshot.sh`'s usage
   block already states — by decision, with its reason — that it has no void
   class. Introducing one would also move an exit code its own suite and D-220's
   recorded verdicts read. The other seven void, and `determinism.sh`,
   `file_justification_check.sh` and `tools/puzzle_corpus/tests/run_tests.sh`
   gained a void class and a usage line to do it.

**CLOSED: one of the five driving tests, and the count of five is itself wrong.**
`crates/pistol-cli/tests/config_check_gate_tests.rs` drives
`tools/config_check.sh` — four cases, including the CONTROL item 10 demands.
The strongest is `every_committed_document_loads_and_the_summary_counts_them_all`,
which cross-checks the gate's five bucket counts against an enumeration **the
test performs itself**, rather than against a literal: a literal churns on every
config added and, worse, a gate that stopped reaching a whole directory would
still match whatever literal was last written down.

**STOPPED, with the reason MEASURED rather than asserted.** The five gates' warm
wall costs at `b60c3d3` (`artifacts/pre_phase2_sweep/t3_gate_costs.txt`):

```
config_check           rc=0 WALL 0.2 s
perft_check            rc=0 WALL 6.5 s
search_oracle_check    rc=0 WALL 16.1 s
movetime_check         rc=0 WALL 22.5 s
determinism            rc=0 WALL 72.6 s
```

So COST is not what stops them, and the reason is a better one:

- **`perft_check.sh` and `search_oracle_check.sh` produce no recorded number,
  so item 10's letter does not bind them.** Each is a sequencer: `perft_check.sh`
  is one `echo` and one `cargo test`; `search_oracle_check.sh` is four echoes
  and SEVEN `cargo test`s (`grep -c '^cargo test '` — the closure first said
  six, counted by eye). The verdict is cargo's, and a test driving these would be testing
  `cargo test`. **This is a finding against the ROADMAP's own "five"**, which
  counted scripts rather than recorded numbers. What DOES bind them is a
  different risk — a dropped `--include-ignored` leaves the expensive half
  unrun and the gate green forever — and that is named here rather than closed.
- **`determinism.sh` and `movetime_check.sh` produce recorded numbers and item
  10 binds them, and neither can be given a COMPLIANT test without a new
  interface.** Item 10's rule is not "drive it": it is *"with a control run so a
  pass cannot come from a gate that refuses everything"*, and a control needs a
  seeded violation. Both scripts root themselves at `dirname/..` and read the
  committed fixtures; `determinism.sh` accepts budget overrides only
  (`determinism.sh:192`) and `movetime_check.sh` accepts nothing. Giving either
  a seam — a root override, a fixture override, an injected engine binary, the
  `--keys-bin` shape `tools/book_v3_disjointness.sh:23-25` already uses for
  exactly this reason — is a NAMED DECISION WITH MORE THAN ONE VIABLE OPTION on
  two CI gates, which CLAUDE.md's Process settles by an OPTION MATRIX attacked
  by a fresh-context DECISION-RED-TEAM before selection. **This package has no
  grant for that round and does not take it unattacked.**
- **The `command -v` sweep is STOPPED, and the ROADMAP says why itself.** Item 8
  asks three refusals where there is now one, across 36 sites in 20 scripts —
  and the same ROADMAP paragraph schedules *"an amendment to
  `tools/SHELL_CHECKLIST.md` item 8 for the fourth case bash admits"*. Executing
  a 36-site sweep against an item whose own text is scheduled to change is
  writing the sweep twice.

**What a resume needs**: the matrix for the seam (one decision, both gates), and
item 8's fourth case settled before the sweep rather than during it.

#### T4 — gate 4 denies rustc warnings (D-677), and what that still does not reach

**The import is fixed at the right level.** `generate_turns` is used at
`crates/pistol-solver/src/policy.rs:266`, inside `#[cfg(debug_assertions)]`, so
the crate-level import was live in debug and dead in release. It moves INTO the
block. Release build after: `cargo build --release -p pistol-solver --bin
solver-selftest` emits **0** warning lines, where it emitted the A-19 warning
before.

**Gate 4 gains `-D warnings` beside `-D clippy::all`** (`tools/ci.sh:91-92`),
and `crates/pistol-cli/tests/clippy_gate_flag_tests.rs` proves a planted warning
fails it — three cases: the CONTROL (clean code passes), a planted rustc warning
(`unused import`) that must fail, and a planted clippy lint
(`clippy::needless_range_loop`) that must also fail, so a fix that REPLACED the
flags rather than adding to them goes red. **The test READS the flags out of
`tools/ci.sh`** rather than restating them, so the check and the thing checked
cannot become two documents — and dropping either flag from the gate is what
makes the test fail.

**AND THE GATE THIS ITEM HARDENS WOULD NOT HAVE CAUGHT THE WARNING IT IS NAMED
FOR.** Gate 4 passes no profile flag, so it is the DEV profile, where
`debug_assertions` is on and A-19's import is live. `-D warnings` there denies
every rustc warning the dev profile emits and none that only a release build
does. Said plainly because a reader will otherwise assume the class is closed:
**the class is half closed.** What would close it is a release-profile lint pass,
and that is priced below rather than taken — a second full `clippy` of the
workspace in a second profile, against a class of exactly one known instance,
inside a package whose scope says no perf change and whose CI already runs
twenty-one gates. **Recorded as owed**, with its own tripwire: the fix that
landed here removes the only instance, so a future one is again invisible.

**AND THERE IS A CHEAPER OPTION THAN THE ONE THIS ROW PRICED, measured by the
group-T reviewer with a control.** A `[workspace.lints.rust] unused_imports =
"deny"` entry turns A-19's restored import into a release BUILD error
(`REL_BUILD_EXIT=101`, against `0` in the same tree without it), and CI already
runs release builds inside gates 9 and 13 — so the class closes at no extra CI
time. Not a full substitute: the failure arrives as `solver_determinism: RUN
VOID: the build failed`, which is the wrong class, and that cost belongs in the
matrix a resume writes rather than being the reason not to write one. Recorded
so the resume starts from two options, not one.

#### T5 — the `info totals` consumer register (A-08)

Docs-only, as the audit's package column says, and **a shared reader is NOT
built**. `tools/SHELL_CHECKLIST.md` gains an APPENDIX — deliberately unnumbered,
on items 11 and 12's own reasoning about renumbering, and because a register is
not a rule a reviewer answers.

**The count moved and that is the register's first useful output.** A-08 named
eight sites; the register holds **ten sites in nine files**. Two are new:
`crates/pistol-arena/src/capture.rs:59-90`, the capture pass's own
normalisation, which parses the same line for a different reason, and
`tools/bench_delta.sh:379-390`, which A-08 missed and which produces this
project's OFFICIAL perf verdict (D-220). One of A-08's is gone rather than
fixed: `artifacts/wp20b_perf_guard.sh` was uncommitted, and `artifacts/` is not
tracked. One of A-08's classes is excluded on inspection:
`crates/pistol-arena/src/record.rs:113` folds already-parsed numbers and reads no
grammar.

### Group E — engine crates, byte-identity guarded

| item | state | commit |
|---|---|---|
| E1 | **CLOSED** | `1f6e388` |
| E2 | **CLOSED** | `c586837` |
| E3 | **CLOSED** | `c586837` |
| E4 | **PARTIAL** — three of five refusal texts pinned; two are unreachable, recorded below | `c586837` |
| E5 | **CLOSED** | `c586837` |

**THE GUARD, TAKEN BEFORE THE COMMITS AND NOT ASSERTED.** Two engines built in
two trees — `b60c3d3` in a detached worktree, the sweep's own tree after — driven
over the twenty positions of `crates/pistol-cli/tests/fixtures/tactical_v0.txt`
at `depth_turns 3` and `nodes 50000`, with ` nps <n> time <n>` stripped exactly
as `tools/determinism.sh` strips it, under three seats:

```
instrument_v0                SEARCH OUTPUT IDENTICAL (159 lines)
instrument_staged_v0         SEARCH OUTPUT IDENTICAL (159 lines)
play_staged_v0               SEARCH OUTPUT IDENTICAL (160 lines)
```

**Each side ran its own tree's configs, and that is the correct comparison**:
after E1 the old binary cannot load the new documents (a missing key is an
error) and the new binary cannot load the old ones (`deny_unknown_fields`). The
claim is that the SEARCH is the same, not that the document is.

**The census digest, over a firing workload.** `configs/gate_staged_solver_v0.toml`,
`go nodes 4000 census` on the fifteen-stone bench position
`census_protocol_tests.rs` uses: **5 rows, digest `31236b5632c0c1b1…` on both
sides, byte-identical**. The first attempt at this comparison produced **zero
rows on both sides and "identical"** — a criterion the defect it guards against
preserves, which `docs/process.md` calls passing vacuously. It was rebuilt with
the position line the test's own `position_line` helper produces, and the row
count and a printed row are quoted so the guard is visibly non-empty. The first
row carries `turns_from_root 0`, so the ROOT site — the one E2 rewrote — is in
the compared set.

#### E1 — the two dead keys (D-675), and the listed edit

Dropped from the schema (`config.rs`), the validator (`validate.rs`), the
composition root (`instance.rs`), the identity line (`bin/pistol.rs`), **all
eighteen committed staged configs**, and the engine test template. A config
carrying either is now refused, and the test that pins it asserts the stronger
fact the code turned out to give: not the table, but the FULL key path.

```
search.candidate_policy.quiet_top_k: unknown field `quiet_top_k`, expected one of
`quiet_radius`, `safety_net_top_k`, … `countermove`
```

**THE LISTED EDIT (D-54y), with its diff.** P3 established that no committed
golden pins the staged identity line, so what E1 moves is one line of live
handshake output and nothing on disk:

```
$ diff <(grep '^id ' before) <(grep '^id ' after)
9c9
< id candidate_policy staged quiet_radius 2 quiet_top_k 16
---
> id candidate_policy staged quiet_radius 2
```

That is the whole of the difference between the two engines, on every seat.

**FIVE CONFIG COMMENTS WERE FALSE AND ARE NOW TRUE, which is a finding the
ruling did not anticipate.** Four headers justified "THE CUT IS DISABLED" by the
VALUE of `quiet_top_k` — *"`quiet_top_k = 128` never cuts a legitimate cell"*,
*"set above the whole candidate pool"*. No code read the key, so the cut was
unarmed whatever the value said; the conclusion those headers draw survives and
now rests on the code. **The fifth is worse and contradicted itself in one
block**: `configs/instrument_staged_v0.toml` opened *"THE CUT BINDS HERE … a
seat with the quiet cut disabled would make the SPRT measure nothing about the
prune"* and then said, four lines later, that the two knobs naming that cut were
*"not yet read by the search this D-scope ships"*. Both cannot be true of one
knob. **This is the SPRT seat**, so the sentence a reader would have taken as
the reason the seat measures anything was the false half.

#### E2 — one census row, one constructor, one push (A-05)

`TriggerColumns::at(state, threats, turns_from_root)` and
`census::push(census, site, attacker, defender)` in `crates/pistol-search/src/census.rs`;
the in-tree site (`pvs.rs`) and the root site (`search.rs`) both go through
them, and `root_census_site` now varies `turns_from_root` and nothing else. The
audit's own words for the defect — *"A column added to one and not the other
splits the census silently"* — is what the shared constructor removes; the
census tests could not see it while the two sites were separate, which is why
the guard for this item is the digest above and not the suite.

#### E3 — the OnePly degeneration, one behaviour (D-676)

`one_ply_turn(state, at)` in `search.rs`, called from `proof_first_move` and
`proof_line`. The two used to answer DIFFERENTLY on the same impossible board —
`?` to `None` and a bare `.expect`. It is now one named invariant,
`NO_ONE_PLY_PARTNER`, in the crate's own idiom (`pub const NAME: &str`, panic
carrying it), which is the form thirteen sibling invariants already take.

**AND IT IS DRIVEN, WHICH LOOKED IMPOSSIBLE.** An empty board's legal region is
exactly the origin (`movegen.rs:100-104`, rule 3), so a one-ply witness AT the
origin has no partner — a reachable input for a guard a real position cannot
provoke. Both public call sites are driven through it in
`crates/pistol-search/tests/refusal_text_tests.rs`, which discharges E3's "test
pins the error at both call sites" rather than asserting it.

#### E4 — the refusal texts (A-21), three of five

| site (at HEAD) | pinned by |
|---|---|
| `search.rs` radius past `i16` | `the_radius_refusal_says_a_ball_wider_than_a_coordinate_is_not_a_ball`, and both keys under their own names |
| `search.rs` `q_depth_turns` ceiling | `the_q_depth_refusal_names_the_table_the_chain_would_run_past` |
| the OnePly degeneration (was `search.rs`'s `.expect`) | both call sites, above |
| `quiescence.rs` `NO_COMPLETION_STONE` | **NOT PINNED** |
| `search.rs` `SOLVER_PROOF WITHOUT A MOVE` | **NOT PINNED**, but now NAMED |

**The two that are not pinned cannot be, and the attempted reproducers are the
record.** `NO_COMPLETION_STONE` fires when neither the live-window-support union
nor the mover's six neighbours offer a legal cell, inside a private quiescence
path; `SOLVER_PROOF_WITHOUT_A_MOVE` fires inside the private
`solver_proof_outcome`, which is called only on OR-rooted trees, and
`proof_first_move` returns `None` only for AND-rooted ones. Neither has a
reachable driver, and D-553's rule is explicitly about *"a test that drives the
call site with reachable input"* — a test that cannot is not a weaker test, it
is no test. **What was done instead, for the second**: it was the only
`pistol-search invariant` in the crate spelled as a bare literal with spaces
rather than a named constant, so it becomes `SOLVER_PROOF_WITHOUT_A_MOVE`
beside its thirteen siblings — greppable, and one edit away from a test if a
driver is ever found.

#### E5 — the orphaned doc block (A-18)

The `#[allow(clippy::empty_line_after_doc_comments)]` is gone, and with it the
only `#[allow]` in non-test `src/`. The detached paragraph was `should_stop`'s
and has been REATTACHED to `should_stop` rather than deleted: it states why a
node budget is tested at a granularity and a deadline at every abortable node,
which the doc that was already on that method did not say.

### Group C — comments only

**CLOSED**, and three of its six rows turned out to be smaller or different than
the audit states. Nothing but comments and doc text changed; the machine checks
group C is reviewed against — `cargo fmt --all --check`, clippy at gate 4's own
flags, `cargo doc --workspace --no-deps` warnings, the goldens and the census
digest — are all clean, and `cargo doc` is clean for the first time in this
package's reading of it.

| row | the audit | at HEAD, and what was done |
|---|---|---|
| A-11 | one non-root `//!` (`params.rs`) | CONFIRMED, and it is the only one. The block is GONE, its two paragraphs MOVED onto the items they are about — the "not a rule" paragraph onto `CandidatePolicy`, the no-`Default` paragraph **with its `compile_fail` doctest** onto `SearchParams`. The doctest is why this is a move and not a deletion: it only runs from a doc comment, and CLAUDE.md's rule is that a module's *public item docs* carry its purpose |
| A-12 | 7 crate roots, 331 `//!` lines | **8 roots, 344 lines** — the audit's list omits `pistol-api`. All eight cut: **344 → 76**, the largest `pistol-solver` 110 → 15. The BINARY root `crates/pistol-solver/src/bin/solver-cost.rs` is outside both the audit's row and its command, and carried 16; it is cut to 9 in the same spirit rather than left as the one root the package's own standard does not reach |
| A-13 | 16 blocks of ≥ 8 consecutive `//` | **17 at `c586837` under the AUDIT's OWN detector, 4 now** — see the detector note below |
| A-14 | ≈ 70 `pub fn … -> Result` with no `# Errors` | **25 `# Errors` sections at `c586837`, 103 at `cb6e853` — 78 added, none removed. 0 public `Result`-returning functions now lack one** |
| A-15 | 32 trailing inline comments | **ONE in non-test code.** See below |
| A-16 | 2 name-restating docs | **9 at HEAD** by the audit's own pattern; 7 deleted, 2 replaced with something to say |

#### A-15 is the row that was wrong, and the reason is worth the paragraph

The audit reports *"32 trailing inline comments in non-test `src/`; 15 are
`crates/pistol-solver/src/zone.rs:245-266`"*. Re-derived at HEAD, the same
pattern finds 36 — and reading them:

- **31 are inside `#[cfg(test)] mod tests`** — 30 in `zone.rs` (`:176` opens
  the module and every hit is past it) and one in `tt.rs` (`:178` likewise).
  They label the rows of
  a coordinate fixture — `Coord::new(1, 2), // P2` — where an own-line comment
  per row would triple the table and make it less reviewable. CLAUDE.md's own
  tie-breaker is *"when brevity and reviewability collide, reviewability wins"*.
- **4 are not line comments at all** (`cover.rs:78-81`): they are rows of an
  ASCII table inside a `///` doc block, where `//` is doc TEXT.
- **1 is a genuine trailing comment in non-test code**, `pistol-search/src/lib.rs`,
  on `pub mod search;`. It is moved above the item.

31 + 4 + 1 = 36, which is the total.

**So the audit's "non-test" is FILE-level and the rule is MODULE-level**, and
the row's own worked example — the fifteen `zone.rs` lines it names — is inside
a test module. The class the row names is real and its instance count in the
code the rule governs is one.

#### A-12, and two claims the solver's crate root was making that were false

Cutting 344 lines of `//!` to 76 is mostly deleting hand-maintained module
lists: rustdoc builds that list itself from each module, so a second copy in the
crate root is D-424's "a claim the document makes twice" with a rot clock on it.
Two claims did not survive the reading:

- **`pistol-solver`: *"Nothing links this crate yet: no search, engine or binary
  calls it, which is deliberate for this work package (D-249)."*** FALSE at
  HEAD: `crates/pistol-engine/Cargo.toml:20` and
  `crates/pistol-search/Cargo.toml:22` both depend on it, `pvs.rs`'s
  `solver_verdict` calls it on the search path, and CI gates 12 and 13 run it.
- **`pistol-solver`: *"the TWELVE QUERIES … NONE OF THE ELEVEN HANDS OUT A
  `WindowMasks`."*** Twelve is right — nine on `ThreatState` in `query.rs` plus
  three in `cover.rs`, counted — and ELEVEN is a number the same paragraph
  contradicts two sentences after stating it.

**The privacy `compile_fail` doctests were NOT deleted with the rest.** They are
a live mechanism — three examples that make a re-publication of the store a
build failure — and a `compile_fail` block only runs from a doc comment. They
moved onto `ThreatState`, which is the item whose privacy they are about.

## §3 Mutation evidence (R2, D-55y)

**THE HARNESS'S OWN SETUP IS A FINDING, AND IT COST TWO BASELINES.** The first
mutation baseline was taken with `CARGO_TARGET_DIR` exported into the worktree,
which is what the rule for verification work says — and it came back RED at
`solver_link_check_tests`, 8 of 19, every failure `the fixture's binary is where
cargo put it`. That is not a regression: CLAUDE.md's own Environment section
names it — *"several gate-test suites build their own scratch cargo workspaces,
and a shared target directory makes one fixture read another's dep-info"*. **A
worktree already has its own `target/`; the export is what breaks it, and it
breaks the gate runner too (D-672).** The harness stopped exporting it and says
why in its own comment.

**AND THE BASELINE FOUND A DEFECT IN THIS PACKAGE'S OWN NEW TEST, which is what
a purged baseline is for.** `clippy_gate_flag_tests` builds three scratch crates
to plant a warning in; all three were named `subject` at version `0.0.1`, which
is ONE cache entry to cargo. Under a shared target directory the first case's
artifact answered for the other two, and `a_planted_clippy_lint_fails_gate_fours_flag_set`
passed on code that was never compiled. Fixed at `c105ad4`: each scratch crate
is named after its case and pins its own `CARGO_TARGET_DIR`. **The test was
green in the live tree and wrong**, which is the shape this whole package keeps
finding.

**M2 HAD NO SUBJECT UNTIL A TEST WAS ADDED, and saying so is the point.** The
registered mutant is "one preflight removed", and removing a preflight leaves
every gate PASSING — a preflight that passes is a no-op, so the CALL is
invisible. That is precisely D-553's call-removed class. Five suites now assert
that the gate they drive printed `scratch_preflight: … KiB available`, which is
the observation that gives M2 something to kill (`cb6e853`).

### The set, at `cb6e853`, purged baseline GREEN

Baseline: `/home/tom/pistol-wt/sweep-verify` at `cb6e853`, `target/debug/deps`
and `target/debug/.fingerprint` removed first, `cargo test --locked --workspace`
→ **181 suites ok, `EXIT=0`**. Every mutant is applied, the file `touch`ed so no
artifact can be cached from the clean source (D-650), the named suites run, the
file restored and `git diff --quiet` asserted on it. Log:
`artifacts/pre_phase2_sweep/mutants_cb6e853.txt`.

| mutant | what it breaks | verdict, and the test that killed it |
|---|---|---|
| M1 | `solver_determinism.sh`'s `void()` exits **1** instead of 2 | **DEAD** — `a_build_that_refuses_is_a_void_and_not_a_failure` |
| M2 | the scratch preflight CALL removed from `decision_key_check.sh` | **DEAD** — `the_key_gate_asks_for_its_scratch_before_it_seeds_any` |
| M3 | `-D warnings` removed from gate 4 | **DEAD** — `a_planted_rustc_warning_fails_gate_fours_flag_set` |
| M4 | `deny_unknown_fields` removed from `CandidatePolicy` | **DEAD** — `config_rejects_unknown_field` |
| M5 | the root census site passes `turns_from_root: 1` | **DEAD** — 4 of `census_protocol_tests` |
| M6 | `proof_first_move`'s OnePly arm returns `None` instead of the helper | **DEAD** — `a_one_ply_witness_with_no_partner_names_the_invariant_from_proof_first_move` |
| M7 | the radius refusal's text shortened to "too large" | **DEAD** — `the_radius_refusal_says_a_ball_wider_than_a_coordinate_is_not_a_ball` |

**M5 IS NOT THE MUTANT THE DISPATCH REGISTERED, AND THE SUBSTITUTION IS THE
POINT.** R2 registers *"a column added to one site only"*. After E2 there is one
constructor, so that mutation is **no longer expressible** — which is the whole
content of the item. What is expressible, and what M5 does instead, is the root
site passing the wrong `turns_from_root`: the one thing the two sites still
differ in. It kills four tests.

**M7's first attempt did not run.** Its mutation string carried a line
continuation the source spells differently, the harness refused with
`MUTATION SCRIPT FAILED`, no test ran, and nothing was restored wrongly — the
harness aborts before `cargo test` when the patch does not apply, which is the
behaviour a mutation harness needs and the reason it asserts `git diff --quiet`
after every restore. It was retaken with the literal read out of the source.

## §4 Receipts (D-469)

Everything this package measured is under `artifacts/pre_phase2_sweep/`,
gitignored (CLAUDE.md rule 8) and sha-anchored here. `tools/artifact_check.sh`
at the closing revision: gate 5 green in the run quoted in §6.

```
31236b5632c0c1b16f0b6ff2a6a62aee6017321d05f6785b0baec7dd1222bbde  census_after.txt
31236b5632c0c1b16f0b6ff2a6a62aee6017321d05f6785b0baec7dd1222bbde  census_after_fixes.txt
31236b5632c0c1b16f0b6ff2a6a62aee6017321d05f6785b0baec7dd1222bbde  census_before.txt
1d3fbcd03d8dba34c20d08ae89c50b5816f38fed5afca3d68b3893a97956409e  ci_baseline_007f821.txt
9a175b29b81d7b64055927728e779690eb35576144d92aa1bc914874c6b3f763  ci_closing_9ce9a7c.txt
1a187f8bf4509672bdf2b5062f2cdd0d622af160d7a7443820329b6d073a66a2  identity_after_fixes_instrument_staged_v0.txt
2a8d625e221f82db3a64260fb920f713b6e0dbe369ab6ca2381095d1556ab112  identity_after_fixes_instrument_v0.txt
3562d306254bad8593bce9004b70f2c448352c8c6ae2ed64a112b6ce4e9fa366  identity_after_fixes_play_staged_v0.txt
1a187f8bf4509672bdf2b5062f2cdd0d622af160d7a7443820329b6d073a66a2  identity_after_instrument_staged_v0.txt
2a8d625e221f82db3a64260fb920f713b6e0dbe369ab6ca2381095d1556ab112  identity_after_instrument_v0.txt
3562d306254bad8593bce9004b70f2c448352c8c6ae2ed64a112b6ce4e9fa366  identity_after_play_staged_v0.txt
7122245f82cc72403aec3152037706f163dcd36eea2b4c708d4c69e1168f25cc  identity_before_instrument_staged_v0.txt
c7ec54f86d3e71a88b76c7ff37008b853567ae3d08bbbc0f8a6258d932d06a06  identity_before_instrument_v0.txt
c269f81e44b1048763778fe3c8e12fe46f91e6145f5821ed428438267b30fb8a  identity_before_play_staged_v0.txt
6422835b8aebdcf84f5dac53245810561ef684abab35e653d0089d6fee630ee4  mutants_cb6e853.txt
6adb9892359bc0731d98ef7118f0ef32d393dac2aa7b539b2ac32abab4b13652  mutation_baseline_cb6e853.txt
511e000e65afb257b78379060d75c33ef370bb7a372446a1e9e87513cc2fa0e8  mutation_harness.sh
e45cd41eed50e1149d5461fbfb31f54b4cb2e0902519c059eea1b99d84ded141  mutation_set.sh
2585e8c8a597ae109478fa477f3f5a61e7dfadf61109d63187a229aac37eb2e7  t3_gate_costs.txt
bff65973a7cb480f70ab8ec461577f1cc43a049eeb5e4e970ef04cc2be64ba77  workspace_tests_f4f4a5c.txt
8a96b93fed66b695ec71ffc8b9049a9708429e37f968d14818fdd8c47ac6582f  workspace_tests_final.txt
```

**THE CENSUS DIGESTS ARE THE E-GROUP GUARD AND THERE ARE NOW THREE OF THEM.**
`census_before.txt` (built at `b60c3d3`), `census_after.txt` (after E2's
constructor merge) and `census_after_fixes.txt` (after all three fix rounds,
which touched `census.rs` and `search.rs` again) hash IDENTICALLY —
`31236b5632c0c1b1…`. The claim survives its own repair, which is the only reason
it is worth re-stating: a fix round that touched the constructor is exactly when
a byte-identity claim taken once, earlier, stops being evidence.

The `identity_*` triples differ from their `before` in one line each, and it is
the licensed `id candidate_policy staged …` line in every case.

## §5 The three reviews, and the fix round each forced

Three fresh-context REVIEW-impl subagents, dispatched at the pinned revision
`cb6e853`, one per group, reports tracked at
`docs/experiments/pre_phase2_sweep_{T,E,C}_REVIEW.md`. **All three returned
FAIL.** Every finding was real; none was rejected as unreproducible. Each group
took its one licensed fix round (D-481).

| group | verdict | BLOCKING | MAJOR | MINOR |
|---|---|---|---|---|
| T | FAIL | 1 | 6 | 7 |
| E | FAIL | 1 | 4 | 6 |
| C | FAIL | 0 | 3 | 6 |

**THE BLOCKING FINDING WAS MINE AND CI WOULD HAVE CAUGHT IT ON DAY ONE.** Gate 17
exited 1 at `cb6e853`: four tracked files crossed rule 9's cap at `42ab538` and
`f4f4a5c`, three of them pushed over by T3's own preflight blocks and one of them
`tools/file_justification_check.sh` itself. **It survived four commits because
this package cited a CI run at `007f821` — before any of its own work — and never
ran `tools/ci.sh` again**, and because gate 17 is a shell script `ci.sh` invokes
rather than anything `cargo test` reaches. The lesson is not subtle and is
recorded rather than paraphrased: a package that changes gate scripts re-runs the
gates, not the test suite.

The fix is trims and not registry entries — every one of the four was UNDER the
cap at `007f821`, so what went over was this package's own verbosity. The single
exception is `tools/determinism.sh`, which genuinely earns its length now that
item 12 has given it a void class and a two-filesystem preflight; it carries an
entry stating that.

**AND THE FINDING UNDERNEATH IT IS THE ONE WORTH KEEPING** (T MAJOR-6):
`file_justification_gate_tests` drove the gate only in scratch repositories. It
had item 10's control — the gate does not refuse everything — and lacked the
other half, that the gate ACCEPTS THE TREE IT SHIPS IN. Its two siblings carry
that control; this one did not, which is why eleven green tests sat beside a red
gate. `the_shipped_gate_accepts_this_repository` closes it.

### The findings that were against this package's own reasoning

- **E MAJOR-1 — E1 reproduced the defect its own commit message names.** The key
  lines were stripped from eighteen configs and their COMMENTS were not, so five
  configs carried a deleted key's justification orphaned onto the next key. On
  `configs/gate_staged_snk_v0.toml`, a determinism-gate seat, *"Disabled: see the
  header. 128 exceeds the measured bound…"* sat directly above **THE SAFETY-NET
  CAP ARMED** and `safety_net_top_k = 8`. On `configs/instrument_staged_snk_v0.toml`
  a duplicated line attributed `K = 16` to *"`U3_tier_t.md` §10's registered
  value"* where its real provenance is a calibration artifact. Six configs
  cleaned.
- **E MINOR-3 — a wrong-answer mutant SURVIVED, and this closure had claimed the
  opposite.** The reviewer changed `one_ply_turn` to pick the lexicographically
  GREATEST other legal cell instead of the least and twenty targets stayed green,
  every oracle suite included. §2 group E's E3 entry said the degeneration was
  "DRIVEN, WHICH LOOKED IMPOSSIBLE" — what was driven was its REFUSAL, and its
  ANSWER was pinned by nothing.
  `a_one_ply_witness_pairs_the_completing_stone_with_the_least_other_legal_cell`
  pins it at both call sites, deriving the expectation by `min()` over the
  filtered set — a property rather than a restatement of the implementation's
  `find()` over an ordered vector — and **the mutant was re-applied in the
  verification worktree and now DIES** (`5 passed; 1 failed`).
- **T MAJOR-1 — a test whose NAME asserted a property it could not see.**
  `tools/solver_determinism.sh` preflighted AFTER the build it claims to protect,
  the only one of nine with the order inverted, so on a full filesystem it voided
  in cargo's vocabulary — D-281's own reading. The test was called
  `..._before_it_writes_any` and asserted only that the preflight line APPEARED.
  It now compares OFFSETS, so the block cannot drift back below the build.
- **T MAJOR-5 — the register's stated command could not have produced its own
  table.** `awk '/ totals /{` puts the discriminator between SLASHES, and the
  cited `git grep` searched for it in quotes, so it matched neither `awk`
  consumer — including `tools/bench_delta.sh`, the row the register advertised as
  the one the audit had MISSED. An eleventh consumer
  (`tools/staged_cover_bench.sh`) was hidden by the same blind spot. This is
  `docs/process.md`'s named class — a claim checked against the wrong population
  — inside the register whose closing line demands re-derivation.
- **C MAJOR-1 — a mid-work count published as an initial one.** §2 group C's
  A-14 row said "100 such functions, 46 without", refuted by this package's own
  diff: 78 `# Errors` sections added, and 46 cannot absorb 78. The 46 was taken
  after three batches had already landed.
- **C MAJOR-3 — closing A-16 by deletion left seven public items undocumented**,
  trading one clause of CLAUDE.md's Code style for another, in the same commit
  whose justification for cutting the crate roots is that public item docs carry
  a module's purpose.

### Two findings recorded rather than fixed, with the reason

- **E MINOR-6 — `tools/bench_delta.sh` will refuse any A/B bench spanning this
  package.** `GUARDED_ID_FIELDS` includes `candidate_policy`, and across
  `b60c3d3` → here the staged seats' identity lines differ by the licensed token.
  **This is the guard working, not a defect**, and it is written down because a
  future bisect or per-side bench across this boundary will refuse and the
  refusal will look like a mis-seat rather than an intended schema change.
- **T MINOR-5's second half — `-D warnings` exposes gate 4 to the toolchain.**
  No `rust-toolchain` file is pinned, so a rustc upgrade adding a warning turns
  gate 4 red on unchanged code. Accepted, with the reason in the gate's own
  comment: `[workspace.lints.clippy] all = "deny"` already gave the tree that
  exposure for clippy's lints, this widens it rather than creating it, and a lint
  break on a toolchain bump is loud and local.

### What a fix round does NOT discharge

Each group's fix round is its ONE round under D-481. The reviews adjudicated
`cb6e853`; the fixes are a later revision and **no fresh review has read them**.
The group-C reviewer said so unprompted while its own report was being written —
*"that fix round is a different revision and owes its own review; it discharges
nothing here"* — and it is right. What this closure claims for the fix round is
what it can: every finding is addressed, the gates and the suites below are green
at the closing revision, and E MINOR-3's mutant was re-run and dies. Whether that
buys a second review is the operator's call and not this session's, because a
second failure would be D-481's STOP.

## §6 The gates, cited from their own output

`tools/ci.sh` at the closing revision **`9ce9a7c`**, in a detached worktree at
`/home/tom/pistol-wt/sweep-ci`, with **no `CARGO_TARGET_DIR` export** — the
export is what voided the texel package's gate 3 (D-672) and what turned this
package's first mutation baseline red at `solver_link_check_tests`. Log:
`artifacts/pre_phase2_sweep/ci_closing_9ce9a7c.txt`.

```
=== gate 1/21: cargo fmt --all --check
=== gate 2/21: build from the git-tracked file set
=== gate 3/21: cargo test --workspace --locked
=== gate 4/21: cargo clippy --workspace --all-targets --locked -- -D clippy::all -D warnings
=== gate 5/21: artifact rejection
=== gate 6/21: config validation
=== gate 7/21: perft oracle
=== gate 8/21: tactical fixture at its pre-registered threshold
=== gate 9/21: cross-process determinism
=== gate 10/21: differential search oracle
=== gate 11/21: staged generator soundness (four parts)
=== gate 12/21: solver oracle (four gates)
=== gate 13/21: solver determinism
=== gate 14/21: movetime ceiling on the D-95 reproducer class
=== gate 15/21: arena self-match smoke
=== gate 16/21: sealbot anchor platform suite
=== gate 17/21: file-justification check
=== gate 18/21: offline texel and census tooling
=== gate 19/21: decision-key uniqueness
=== gate 20/21: carve-document label consistency
=== gate 21/21: governing-document citations

ci: all gates passed
EXIT=0
```

The gates this package can move, in their own words:

```
config_check: 21 engine config(s), 1 weight table(s), 18 arena config(s), 3 book config(s), 2 solver config(s)
determinism: ok — 5 seat(s), no difference outside nps/time in any of them
solver_determinism: PASS — 61 cases, byte-identical transcripts
file_justification_check: 409 tracked .rs/.sh/.py files, 85 over the cap, all registered in docs/rule9_justifications.md (85 entries)
decision_key_check: 684 decision keys in docs/decisions.md, no repeat outside the exemption
governing_citation_check: 15 governing document(s), 0 proposed path(s)
```

**GATE 4's LINE IS THE ONE TO READ TWICE.** It now prints `--locked` and
`-D warnings`, which is T4's change and T MINOR-3's correction in one line — the
step used to advertise a command it did not run. **And gate 17 is green on the
tree it ships in**, which is the whole of the BLOCKING finding: it was red from
`42ab538` to `cb6e853` and nothing in this package looked, because the only CI
run it cited was taken at `007f821` before any of its own work.

**THIS IS THE GREEN THE PACKAGE NEVER HAD.** D-674 says a package's closing
revision is the one its cited CI ran at; the run above is that, and §2's H1 entry
— which quotes a run at `007f821` — is the STARTING green, not this one.

## §7 What a successor needs

- **The package is CLOSED on group H, group E and group C. T3 is the only
  PARTIAL**, and it is partial by measurement rather than by cost: `perft_check.sh`
  and `search_oracle_check.sh` produce no recorded number, so item 10's letter
  does not bind them; `determinism.sh` and `movetime_check.sh` do, and neither can
  take a COMPLIANT test without a seeded-violation seam, which is a named
  decision on two CI gates and wants an OPTION MATRIX this package had no grant
  for. The `command -v` sweep stops for a second reason the ROADMAP supplies
  itself: item 8's own text is scheduled to change, and a 36-site sweep against a
  rule about to move is the sweep written twice.

- **THREE THINGS ARE RECORDED AS OWED AND NOTHING IN THE TREE WILL REMIND
  ANYONE.** (1) The seam decision above. (2) `tools/wp16_warm_attribution_check.py`
  still carries an in-file `RULE9-JUSTIFICATION` marker whose parenthetical is now
  FALSE — it says the gate reaches only `.rs` and `.sh` — and it is left because
  deleting it edits an instrument a pre-registration names with its revision,
  which is the precise coupling D-415 declined. (3) A release-profile lint pass,
  or the cheaper `[workspace.lints.rust]` route the group-T reviewer measured,
  for the class gate 4's dev profile cannot see.

- **THE LESSON THIS PACKAGE PAID FOR, AND IT IS NOT THE ONE IT EXPECTED.** Every
  BLOCKING and MAJOR finding across three reviews was a CLAIM that had come loose
  from the code — not one was a wrong answer. Byte identity held under an
  instrument far stronger than this package's own. What failed was: a register
  whose stated command could not have produced its own table; a test whose NAME
  asserted an order it never read; a "there is no void class" written into a gate
  that builds five binaries; five configs keeping a deleted key's justification;
  two governing specs left describing a schema the engine now refuses; and a
  count taken mid-work and published as an initial one. **The package's own
  commit messages name this class three times and it reproduced it eleven.**

- **AND THE CHEAPEST GUARD WAS THE ONE MISSING.** Gate 17 was red for four
  commits while `cargo test --workspace` was green six times, because a gate
  script is not a test and this package never re-ran `tools/ci.sh` after its
  first item. One control — *does the gate accept the tree it ships in* — turns
  that from a reviewer's find into a red suite at the commit that causes it. Two
  of the three gate suites already had it. A successor changing anything under
  `tools/` runs the gates.

- **WHAT NO ONE HAS READ.** The reviews adjudicated `cb6e853`. The fix round is
  a later revision and has had no fresh review; each group has now spent its one
  round under D-481, so a second review returning FAIL is that group's STOP. The
  evidence offered in its place is in §6 (21 gates from their own output), §4
  (three census digests agreeing across the repair) and §3 (M8, the group-E
  reviewer's surviving mutant, re-applied and now dying).
