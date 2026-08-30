# WP-1.9b — CLOSED. O-3 measured and rejected; the store landed inline, bit-identical, inside its bracket.

> **Audience: the next agent.** Every claim here names the artifact or command
> that establishes it, so a successor verifies rather than trusts. §8 is the
> environment hazards this session actually hit — three of them are new.

## 0. ONE LINE FOR THE MORNING

**Both of WP-1.9's open debts are closed by measurement: O-3, the hand-rolled
probing table, was built, reviewed and benched at 1.518/1.594 against the
registered 1.783/1.909 comparand — below it in BOTH bands, so NO FLIP and D-501
is discharged rather than left to silence — and the store therefore landed as O-2
moved INLINE into `handcrafted.rs`, measured at 1.186/1.214 inside its registered
[1.10, 1.30] bracket and 0.001/0.006 from what the bracket predicted, with search
output byte-identical to the SAME digest WP-1.9 recorded (`0b1cb805…`, three
storage revisions, one output). Stage 1's residue is now WP-1.4 and WP-1.10
alone.**

---

## 1. What is on `dev`, and what is not

| Revision | What it is |
|---|---|
| `wp19b/o3-fixed` (`6ea88b2`) | **O-3, complete and reviewed — NOT on `dev`.** Tagged so a successor reproduces rather than re-implements, on the `wp19/mx-*` precedent. |
| `wp19b/landing` (`13abe40`) | The landed shape: O-2's store moved inline, `window_map.rs` deleted. Both the governing landing bench and the governing identity run name THIS revision. |

**Engine diff vs `3c9e28b`:** `crates/pistol-eval/src/handcrafted.rs` (the store
moves in), `crates/pistol-eval/src/lib.rs` (`mod window_map;` goes), and
`crates/pistol-eval/src/window_map.rs` deleted. `crates/pistol-core/src/window.rs`
is untouched at closure — it was edited and reverted when O-3 lost, and the
revert is real: `git diff 3c9e28b -- crates/pistol-core/` is empty.

**The store's semantics did not change.** Three call sites that went through
`WindowMap::{entry_or_default, update, get}` now spell the same operations
directly on the map: `entry().or_default()`, an `Entry::Occupied` slot edited and
removed when it empties, and `get().copied().unwrap_or_default()`. `undo` still
resolves its slot in ONE probe — **do not reintroduce a `set`**, which is
WP-1.9's own instruction and still holds.

---

## 2. The three runs, and the one that decided the package

All on `tools/bench_delta.sh` at `ab369b0`, `configs/instrument_v0.toml`, 24
positions, 5 reps, node identity holding per position in every rep, exit 0.
Registered in `docs/experiments/wp19b_bench_prereg.md` (committed `b60094e`,
BEFORE any run) and read in `docs/experiments/wp19b_bench_results.md`.

| run | sides | early | late | reading |
|---|---|---|---|---|
| dry | `mx-base` → `mx-O1` | 1.195 | 1.242 | **PASSES** its ±0.08 criterion against O-1's recorded 1.198/1.242 |
| **flip** | `a5c5661` → `6ea88b2` | **1.518** | **1.594** | **NO FLIP** — below the 1.783/1.909 comparand in BOTH bands |
| landing | `3c9e28b` → `13abe40` | **1.186** | **1.214** | **INSIDE** the registered [1.10, 1.30]; replicated at 1.171/1.205 |

### Why the dry run mattered more than usual

Its referent is another session's artifact, so it is external to everything this
session did — and it came back 0.003 and 0.000 from the recorded pair. That is
what licenses quoting the O-2 comparand today instead of re-measuring it, which
would have replaced a registered comparand with a fresh one (the bracket move
D-374 forbids).

**Dry-run discipline also caught a defect in the registration itself.** The
command was first registered at ONE rep; `tools/bench_delta.sh:136` refuses
`REPS < 5`, so that spelling could never have run. Amended at `3e004e1`, before
any run, stricter rather than weaker, and recorded rather than edited away.

### What O-3's number means, stated narrowly

O-2 inline 1.783/1.909, O-2 in a module 1.508/1.579, **O-3 inline 1.518/1.594**.
O-3 lands on top of O-2-BEHIND-A-MODULE: a hand-rolled linear-probing table is
about a seventh slower than the `HashMap` it would replace at equal placement.
`hashbrown`'s SIMD group probe and unchecked indexing beat a bounds-checked
linear walk here. **This says nothing about a probing table in general, on other
hardware, or at other occupancies, and it measured no memory figure at all** — a
second axis measured after the fact is the ground D-500 struck.

---

## 3. Track E — byte-identity, and the digest that did not move

`artifacts/wp19b_byte_identity_v2.txt`: baseline `3c9e28b` (binary `ddbae8f3…`)
against the shipped landing `13abe40` (`e0eb1b19…`) — **the binaries differ, so
the comparison cannot pass vacuously**. 44 positions (`tactical_staged_v0.txt` 20 +
`bench_positions_v1.txt` 24), both determinism budgets, 88 searches, 422 output
lines, 88 bestmoves, 0 error lines each side, `nps` and `time` elided.

**IDENTICAL**, digest `0b1cb8054857e8a4a877297733d284b23efaeaad8ccd76f0a6a65d34b5512edf`
— which is the digest `artifacts/wp19_byte_identity_v2.txt` already carried. The
pre-WP-1.9 baseline, WP-1.9's hashed module, and this inline landing are ONE
output. No SPRT is owed (D-495).

---

## 4. The reviews, and what they cost

Two fresh-context REVIEW-impl rounds, one per landing candidate.

### O-3's review — FAIL, 0 BLOCKING, 3 MAJOR, 4 MINOR

`docs/experiments/wp19b_o3_impl_REVIEW.md`. **The store was proven sound by
instruments the reviewer built**, not by inspection: a 1.2M-operation fuzz against
a `HashMap` reference, and an EXHAUSTIVE model check of backward-shift removal
over all `8^4` home assignments × 4! build orders × 4! deletion orders —
9,437,184 removal states, every one sound.

Every finding was in the evidence layer, and each fix was verified to kill the
reviewer's own reproducer (`artifacts/wp19b_mutations_v1.txt`, fix-round section):

| finding | the defect | after the fix |
|---|---|---|
| MAJOR 1 | a ONE-BIT hash-constant flip passed all 39 tests while relocating 6.75 % of windows; retiring WP-1.9's digest golden dropped a pin nothing else carried | digest golden restored — kills it as a **singleton** |
| MAJOR 2 | the rule-9 entry asserted a measured cause the cited run never separated, for an arrangement that run never contained | entry states the measurement narrowly, cites D-502 |
| MAJOR 3 | growth off-by-one passed at load factor 0.5156; a real break **hung** `probe` rather than failing, and CI has no per-test timeout | bound asserted after every insert + a `debug_assert!` in `probe` — now fails by name |
| MINOR 4 | the placement golden was reproduced byte-for-byte by triangular probing | golden set extended so its deepest key sits four slots from home |

**The lesson worth carrying: two of the four survivors were invisible to every
oracle in the workspace.** A moved placement moves no search output, so
`tools/determinism.sh` cannot see it and an agreement test passes. Goldens are
the only pin for that class — WP-1.9 learned it for a reseeded hasher, and this
package learned that ONE golden was not enough.

### The landing's review — PASS, 0 BLOCKING, 1 MAJOR, 8 MINOR

`docs/experiments/wp19b_landing_impl_REVIEW.md`. **Its strongest finding is in
the change's favour and is better than the claim this package was making**: the
landed non-test code is not merely equivalent to WP-1.9's module version, it is
byte-for-byte the revision the 1.783/1.909 comparand was measured on
(`wp19/mx-O2`), differing outside doc text and the appended tests in four items
only. The reviewer broke seven mutants to prove both rewritten container tests
pin what they name, and confirmed all three of D-502's singletons survive.

MAJOR 1 was a false qualifier — "independent runs a day apart" when `stat` shows
3h13m on one day. Both copies were uncommitted and both are corrected; the
independence that does the work is separately built binaries at different
governing revisions, not elapsed time. Six of the eight minors are fixed
(including `cargo doc -p pistol-eval` going from five warnings to **zero**); two
are recorded as residue: the ONE-PROBE shape of `undo` is guarded by no test —
only the bench catches a two-probe rewrite, and the code comment now says so —
and a pre-existing release-only warning in `pistol-solver` that no gate sees,
which is WP-1.10's material and not this landing's.

---

## 5. Rule 9

`handcrafted.rs` is over the soft cap and carries ONE entry in
`docs/rule9_justifications.md`, which is the remedy the rule itself names. The
entry's first draft was a review finding and its replacement says only what was
measured: WP-1.9's head-to-head compared this store inline against the same store
behind a module boundary, its two sides differ in more than that boundary, so it
measured the CHANGE and did not isolate the boundary as the cause. The numbers
live in D-502 and in this package's own landing bench — both resolvable without a
path under the gitignored `artifacts/`.

**The selection record's condition 1 is superseded BY NAME** (D-507):
`matrix_wp19_storage_selection.md` §4.1 required a separate module *because*
inline went over the cap with no entry. The cap is soft, the entry is the remedy,
and the entry now exists — the condition's own ground is spent.

---

## 6. Gates

`tools/ci.sh` at the base revision `e299b0e` before any work:
`artifacts/wp19b_ci_base_v1.txt`, all nineteen `gate N/19:` lines, final line
`ci: all gates passed`, and `/usr/bin/grep -cE "^ci: FAIL|^ci: RUN VOID|test
result: FAILED"` returns 0.

At closure: `artifacts/wp19b_ci_closure_v2.txt`, read the same way — nineteen
`gate N/19:` lines, `ci: all gates passed`, and the FAIL/VOID/FAILED count 0.
`_v1.txt` is the same result one commit earlier and is kept as the replication;
`_v2.txt` is the run that adjudicates the closure HEAD, because two documentation
commits landed after `_v1.txt` and a gate claim names the tree it read.

Gate 9 is `tools/determinism.sh` and it covers every seat — `radius`, `staged`,
`staged-heuristics`, `staged-solver`, `staged-safety-net-cap` — each printing
`ok — 40 searches, 20 positions, no difference outside nps/time`, and the gate's
own closing line `determinism: ok — 5 seat(s)`.

**One CI run in this session is VOID and is not a failure.**
`artifacts/wp19b_ci_start_VOID_v1.txt` was launched against `e299b0e` and reached
gate 12 before this session edited the working tree it reads. No gate after that
edit adjudicated the revision the run was launched to confirm, so the run is void
(`tools/SHELL_CHECKLIST.md` item 12's distinction, applied to a run this session
spoiled rather than one a gate declined). It was re-taken in an isolated worktree.

---

## 7. What is NOT closed

- **WP-1.4** — D-95 movetime ceiling. On HeXO the server owns the clock, so this
  is a FORFEIT RISK, licensed-not-scheduled (D-478).
- **WP-1.10** — `tools/` gate hardening: five undriven gate scripts, the
  `command -v` sweep, the `mktemp -d` preflight debt.
- **Next per D-471/D-494:** the Stage-3 scoped detector, whose §0 carries
  `book_v2` (D-505) and the WP-1.8 nps-jump limb (D-504). **It now measures
  against a post-WP-1.9b engine**, which is what D-504 anticipated.

**Observed and NOT introduced here**, recorded so it is not attributed to this
package: `crates/pistol-solver/src/policy.rs:1` imports `generate_turns` unused
in the `--release --bin pistol` build (byte-identical to `3c9e28b`), and
`cargo doc -p pistol-eval` emits rustdoc warnings that were present at `e299b0e`
in the same shape. Neither is gated.

---

## 8. Environment hazards — three of these are NEW

1. **`tools/ci.sh` reads the WORKING TREE, so a CI run is only a confirmation of
   the revision it started on if you do not touch the tree while it runs.** This
   session voided one run learning that. Run the green-confirmation in a separate
   worktree (`git worktree add --detach`, its own `target/`) and edit freely in
   the live tree.
2. **`local a="$1" b="$prefix/$a"` dies under `set -u`.** Bash expands the whole
   assignment list before binding any of it, so `$a` is unset when `b` is
   computed. It cost one byte-identity run. Two statements, not one.
3. **ADDING A COMMENT CHANGES THE RELEASE BINARY.** `panic!`, `expect` and every
   bounds check embed `file!`/`line!`, so two lines of comment above a function
   body move every panic location below it and the `--bin pistol` digest moves
   with them. Behaviour cannot change; the digest does. Do not conclude from a
   changed binary that code changed, and do NOT conclude from "I only touched a
   comment" that a registered run still names the shipped revision — re-take it.
   The same-path rebuild is what rules out the build directory as the cause
   (a revision rebuilds to its own digest at any path).
4. Everything WP-1.9's §9 lists still holds: no concurrent `cargo` on a shared
   target dir; worktrees and build dirs under `/home`, never the 24 GiB `/tmp`
   tmpfs; `/usr/bin/grep` or `git grep` for anything recorded; check `ps` before
   trusting a poller; tag any measured revision before removing its worktree;
   `git add` before the rule-9 checker, which reads tracked bytes; `cargo fmt`
   before quoting a line count.
5. **A bench must not share the machine with a build.** Every run above was taken
   with `ps -eo cmd | grep -c '[c]argo'` returning 0, and the reviews were
   dispatched only between runs for that reason.

---

## 9. Where everything is

| Document | What it is |
|---|---|
| `docs/experiments/wp19b_o3_design.md` | O-3's design, with an outcome banner recording that it did not flip |
| `docs/experiments/wp19b_bench_prereg.md` | all three runs registered before any of them |
| `docs/experiments/wp19b_bench_results.md` | the runs read against what was registered |
| `docs/experiments/wp19b_o3_impl_REVIEW.md` | REVIEW-impl of O-3 |
| `docs/experiments/wp19b_landing_impl_REVIEW.md` | REVIEW-impl of the landing |
| `artifacts/wp19b_*` | the receipts, digest-indexed in `artifacts/wp19b_export_receipt_v1.txt`, whose own sha256 is `d2d80d67529c978148ca1eb3558507c0dbcbf9b90624a6848fa5639c0836ebfb` — D-469 wants that list committed or SHA-ANCHORED, and `artifacts/` is gitignored, so this line is the anchor |
| `sessions/WP-1.9b/` | a POINTER to this file only — `/sessions/` is gitignored, so a closure written there is one a successor cannot resolve (`6812ddc`'s lesson, applied ahead of the mistake) |

ADR lines: **D-506** (O-3 measured and rejected, the trigger discharged) and
**D-507** (the landing, its bracket, and the superseded selection condition).
