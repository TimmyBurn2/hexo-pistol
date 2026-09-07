# Sealbot clock fix — closure

**What it was for.** `docs/decisions.md` **D-699** stopped the v7 anchor's budget
decision after two DECISION-RED-TEAM rounds and named three things a resume
needs. This package delivers the second — the harness change — and the third, a
criterion. **The first, an ADR amending D-695's "equal measured movetime per
side" clause, is NOT taken here and the decision stays stopped.**

**Closing revision** `6146b49`, `dev`. **CI green there from the gate's own log**
(`artifacts/fast_wins/ci_clockfix.txt`): `ci: all gates passed`, **zero failing
tests in the whole run**. Gate 16 `sealbot-tests: PASS`. The lines this package
moved: `file_justification_check: 421 tracked .rs/.sh/.py files, 89 over the cap,
all registered`; `decision_key_check: 702 decision keys`.

**Nothing under `crates/` changed**: `git diff --name-only 1b89034 6146b49 --
crates/pistol-search crates/pistol-core crates/pistol-eval crates/pistol-solver
crates/pistol-engine configs` is EMPTY, and the golden
`instrument_behavior_byte_identical_pre_post` ran green in the closing run.

## What changed, and what it bought

`SealbotClient::new_game` reads a `sealbot_shim: ready` line from stdout before
the clock starts, and every reply carries a floored whole-millisecond
`engine_time_ms` the client REQUIRES.

**MEASURED end to end on the real shim**, first answer of a game, 300 ms budget:

| | wall | excess |
|---|---|---|
| before | 326.9 / 325.8 / 326.1 ms | **25.8–26.9 ms** |
| after | 300.9 / 300.8 / 300.4 ms | **0.4–0.9 ms** |

and the seat's non-search overhead stops being a BOUND (4.50 % of wall in v5,
forced by a null column) and becomes a per-answer number.

## The process, said plainly: three design revisions, two of which fell

| round | verdict | what killed it |
|---|---|---|
| design rev 1 | **FAIL** 7 MAJOR / 10 MINOR | its criteria passed on fully defective data at every ODD game count, because `a_is_p1 = game % 2 == 1` makes the population bimodal; one criterion was failed by the seat that already works |
| design rev 2 | **FAIL** 6 MAJOR / 8 MINOR | two start-up constants 4x apart for the same run; a looseness ratio computed on a referent the same section disowns nine lines later; a fourth criterion that **could not fire** while the third held |
| design rev 3 | — | in force |
| REVIEW-impl | **FAIL** 4 MAJOR / 8 MINOR | a wedge, a panic, a guard that could not see its own drift, and a coverage gap that was not forced |

**THE ONE THAT MATTERED MOST WAS A REAL BUG.** `await_ready` returned `Err` on
both branches **without killing the process**, where `pick_turn` already killed.
A shim hanging before its stdin loop — an import stall, or sizing the TT — then
makes the referee's `close_stdin(); wait()` block for ever, so the **whole run
wedges** rather than one game forfeiting. Measured, exit 124.

**AND THE SECOND WAS A GUARD THAT COULD NOT SEE THE DRIFT IT EXISTED FOR.** The
readiness-spelling guard was a `grep` over whole files, and `sealbot_shim.py`
holds the literal TWICE — module docstring and constant — so mutating only the
constant left the docstring and the guard matched. It is
`tools/SHELL_CHECKLIST.md` item 3's "a substring is not a token", and the
package's own mutant receipt had recorded that mutant as DEAD when it was alive,
because it died for the two producers that need no guard.

**THE COVERAGE GAP WAS NOT FORCED, AND CLAIMING IT WAS IS THE FINDING.** Two
revisions asserted CI could not drive the real shim because it needs a sealbot
checkout. It does not: the shim takes both module directories from **argv**.
`tools/sealbot/tests/fake_sealbot/` is two small modules, and the shipped shim
now runs in CI against them with a bot that sleeps a **known fraction** of its
limit — reporting **100 ms against a 400 ms budget**, which is exactly what
separates an honest seat from one echoing its configured budget. The source
guard is deleted rather than repaired.

## The mutants, all dead

Receipts `artifacts/fast_wins/clock_fix_mutants.txt` and
`clock_fix_review_round.txt`.

| | mutant | dies as |
|---|---|---|
| N1 | the client stops REQUIRING `engine_time_ms` | `notime` not refused naming "no whole-millisecond" |
| N2 | `new_game` skips the preamble read | `silent` not refused naming "engine timeout" |
| N3 | the shim writes `ready` only to stderr | the shim's first STDOUT line is the JSON reply |
| N4 | only the shim's CONSTANT drifts | the shim's first STDOUT line is `sealbot_shim: rready` |
| N6 | the shim reports its CONFIGURED budget | reported 400 for a bot that slept ~100 |
| N8 | `await_ready`'s timeout branch stops killing | the `wedge` config exited 124 |
| N9 | `await_ready`'s protocol branch stops killing | the `wedgeword` config exited 124 |
| N10 | the byte-index slice returns | the `nonascii` config exited 101 |

**N8, N9 AND N10 NEEDED NEW STUBS, and that is worth writing down.** The original
`silent` and `wrongword` stubs READ STDIN, so they exit on EOF and the referee's
`wait()` returns whether or not the client killed them — the wedge is reachable
only with a stub that never reads stdin at all. A test can be green because the
subject is correct or because the fixture cannot express the failure, and only
the mutant tells you which.

## What is still owed

1. **The ADR amending D-695.** Its "equal measured movetime per side" clause and
   the series budgets (500/300) are in unresolved conflict, and
   `matrix_anchor_v7_budget.md` carries a FELL banner after two red-team rounds.
   Until that line is written and a revision 3 of the matrix survives, the v7
   protocol may not be run.
2. **The criteria are REGISTERED, NOT RUN.** No match is played by this package,
   so D-699's precondition is not discharged by landing the fix — it is
   discharged when an anchor is played and read against
   `sealbot_clock_fix_design.md` §4.
3. **Nothing is blocked meanwhile**: v7's trigger is the first Phase 2 `h1`, and
   Phase 2 has not started.

## Two standing facts a successor should not rediscover

- **The per-game start-up charge is not a constant across runs.** It was
  22.08 ms/game in v5 and 6.44 ms/game in v6, and the difference is spawn order,
  not load: `referee.rs:156,163` starts slot A first, and in v6 sealbot IS slot
  A, so pistol's own spawn absorbs most of sealbot's.
- **The matchserver is outside CI gates 1 and 4.** It is deliberately not a
  workspace member, so `cargo fmt --check` and `cargo clippy --workspace` never
  reach it; it carries 2 clippy errors and 10 rustfmt-dirty files at HEAD, none
  of them introduced here and none of them fixed here.
