# OPTION MATRIX — a seeded-violation seam for `determinism.sh` and `movetime_check.sh`

**Status: REVISION 1 FELL. The DECISION-RED-TEAM verdict is FALLS, 9 of 10
attacks landed, and the option selected is one revision 1 never listed —
OPTION F, a PATH-shimmed `cargo`, which changes neither gate. See
`matrix_gate_seam_REDTEAM.md` and §6 below. Revision 1's text is kept BELOW,
unedited, because a matrix that is quietly corrected to match its own red team
is a matrix nobody can audit.**
CLAUDE.md's Process: a named decision with more than one viable option is
settled by an option matrix attacked by a fresh-context DECISION-RED-TEAM
BEFORE selection, and the surviving option's ADR line records the strongest
surviving attack.

## §1 The decision, stated once

`tools/SHELL_CHECKLIST.md` item 10 binds any `tools/` script that produces a
recorded number to at least one test **driving the shipped script, with a
control run so a pass cannot come from a gate that refuses everything**.

Two gates produce recorded numbers and have no such test:

- `tools/determinism.sh` — `determinism: ok — %d seat(s), no difference outside
  nps/time in any of them`, and per seat `%d searches, %d positions`.
- `tools/movetime_check.sh` — `movetime: ok — %d seat(s), all within their own
  epsilon`, and per seat `%d searches, worst overshoot %d ms against epsilon %d ms`.

**The decision is not "should they be tested".** It is: *what seam lets a test
seed a violation these gates must refuse?* A test that only re-runs them proves
they pass when they pass — the vacuous half — and item 10 asks for the other
half by name.

**WHY THIS IS A DECISION AND NOT AN IMPLEMENTATION.** Both gates root themselves
at `dirname/..` and read committed configs and fixtures; neither can be pointed
at anything a test controls. Every option below CHANGES A CI GATE'S INTERFACE,
and the failure mode they share is that a seam wide enough to seed a violation
is wide enough to make the gate certify something that is not the subject.

## §2 The premises, quoted

**P1 — the gates' current input surface.** MEASURED at `21f8833`:

- `tools/determinism.sh:69-86` — `SEATS` is a hardcoded array of
  `"name config fixture [budget-overrides]"`; `BUDGETS=("depth_turns 4" "nodes 200000")`.
  `run_seat` at `:199-206` **already accepts trailing per-seat budget words**,
  so this script is not argument-free by design.
- `tools/movetime_check.sh:38-39` — `SEATS=("radius configs/play_v0.toml"
  "staged configs/play_staged_v0.toml")`, one hardcoded `FIXTURE`. It accepts
  nothing.
- Both resolve the engine off cargo's artifact stream (D-250's block), so
  neither reads a binary path from anywhere today.

**P2 — a protocol-speaking stub already exists, with a behaviour vocabulary.**
MEASURED: `crates/pistol-arena/src/bin/stub_engine.rs:180-184` lists nineteen
behaviours — `honest, honest_last, illegal, garbage, bad_bestmove, hang, exit,
bad_protocol, play_mode, edit_own_config, demands_newgame,
demands_newgame_per_ask, refuses_go, tab_totals, census_rows, census_none,
census_tab, census_rows_unasked, stray_after_newgame <n>`. **None of them is
"answer differently on the second run"**, which is the behaviour a determinism
seed needs, so every option that uses the stub also adds a behaviour.

**P3 — the precedent, and the reason it was set.** MEASURED,
`tools/book_v3_disjointness.sh:23-25`: *"`--keys-bin` names an already-built
`book_keys` instead of building one. It exists because this script's own test
drives it from inside `cargo test`, and a nested `cargo build` there blocks on
the target directory's lock."* So this repository has already accepted a
test-facing binary-injection flag on a gate, for a DIFFERENT reason (the lock,
not the seam). `tools/solver_link_check.sh` takes a root argument, which is the
other shape.

**P4 — CI's own invocation.** `tools/ci.sh` calls both gates with no arguments
(`gate "determinism" tools/determinism.sh`, `gate "movetime ceiling"
tools/movetime_check.sh`), so any flag added below is unused in CI by
construction. **CI time cost of every option: zero.** MEASURED, warm:
`determinism.sh` 72.6 s, `movetime_check.sh` 22.5 s
(`artifacts/pre_phase2_sweep/t3_gate_costs.txt`).

## §3 The options

### A — inject the engine (`--engine <path>`)

The gate resolves its binary off cargo's artifact stream unless `--engine` names
one. A test hands in a stub that answers differently on two runs; the gate must
refuse and name the seat.

- **Cost**: ESTIMATED ~15 lines per gate (flag parse, one branch in the
  resolution ladder), plus one `stub_engine` behaviour and one test per gate.
- **Reach**: the WHOLE shipped script runs, including the recorded number.
  Seeds the exact violation each gate exists to catch.
- **Failure mode**: an operator pointing a gate at a binary that is not the
  subject and reading the green as the subject's. Mitigable — the gate can
  print the resolved path, and CI passes no flag (P4) — but not eliminable.
- **Precedent**: P3, the same shape for a different reason.

### B — override the root (`--root <dir>`)

The gate runs against a scratch tree holding seeded configs and fixtures.

- **Cost**: ESTIMATED ~10 lines per gate, but the TEST is the expensive part: a
  scratch tree needs configs, sha-pinned fixtures and a built engine, and both
  gates build the engine from `$ROOT`'s workspace — so the scratch tree must be
  a whole workspace or the flag must be combined with A anyway.
- **Reach**: whole script.
- **Failure mode**: `$ROOT` feeds path construction throughout; item 11's
  containment rule applies to every write under it, and both gates write only
  under `mktemp -d`, so the blast radius is small but the guard surface is the
  largest of the four.
- **Precedent**: `tools/solver_link_check.sh`.

### C — override one seat (`--seat "name config fixture"`)

- **Cost**: ESTIMATED ~8 lines in `determinism.sh` (which already parses
  trailing seat words, P1), more in `movetime_check.sh` which parses nothing.
- **Reach**: whole script, but **a seeded config and fixture cannot make two
  runs of the SAME engine disagree** — determinism is a property of the engine,
  not of the documents. So C can seed "no work was done" and cannot seed the
  violation the gate is named for. **This is the option that does not do the
  job**, and it is listed because it is the one that looks cheapest.
- **Failure mode**: a test that passes while pinning nothing about the
  comparison — item 10's own vacuity.

### D — extract the comparator, test that (`tools/transcript_compare.sh`)

- **Cost**: ESTIMATED a new script plus its test; the gates become drivers.
- **Reach**: **the comparator only.** The recorded number is produced by the
  DRIVER, so item 10's binding text — *"a script that produces a recorded
  number"* — still names the untested part. **D moves the obligation rather
  than discharging it.**
- **Failure mode**: the appearance of coverage. A green comparator suite beside
  an undriven driver reads, in a closure, exactly like a discharged item.

### E — do nothing; record the gap

- **Cost**: zero.
- **Reach**: none. Two CI gates keep producing numbers nothing defends, which
  is the state `tools/SHELL_CHECKLIST.md` item 10 exists to end and which
  `tools/bench_delta.sh` already cost this project two rounds of undetected
  defects (D-231).

## §4 Recommendation

**A**, with B rejected on guard surface, C on inadequacy, D on moving the
obligation, and E on item 10.

The strongest argument for A is that it is the only option that seeds the
violation the gate is NAMED for: two runs of one engine disagreeing.
The strongest argument against A is its failure mode — a flag that lets a gate
certify a binary that is not the subject — and the mitigation is partial, not
total.

## §5 What the red team is asked to attack

1. **Is A's failure mode worse than the gap it closes?** A gate that can be
   pointed at a stub is a gate whose green means less. Argue that E beats A.
2. **Is C really inadequate?** If a seeded FIXTURE could make one engine's two
   runs differ — through a position that reaches an unseeded hash path, say —
   then C is cheaper and narrower and the recommendation is wrong.
3. **Is D's reach really the comparator only?** If the recorded number can be
   moved into the comparator, D discharges item 10 at lower risk than A.
4. **Does A actually work?** P2 says no existing stub behaviour answers
   differently on a second run. Show that such a behaviour is implementable
   without making the stub nondeterministic in a way that breaks its other
   nineteen uses.
5. **Is the premise sound that these two gates are bound by item 10 at all?**
   The closure argues `perft_check.sh` and `search_oracle_check.sh` are not,
   because they produce no number of their own. Attack the line.


---

## §6 REVISION 2 — what the red team did to this document

**VERDICT: FALLS.** Nine of ten attacks landed. The one that failed is ATTACK 9,
which tried to show item 10 does not bind these two gates at all: the closure's
line — a sequencer producing no number of its own is not bound — is drawn
correctly, and both gates ARE bound.

### The finding that ends the decision

**§2's load-bearing premise, *"neither can be pointed at anything a test
controls"*, is FALSE.** Both gates resolve their binary off cargo's own artifact
stream (D-250's block), so a `cargo` shim FIRST on PATH that prints one
`compiler-artifact` record naming a stub points either gate at anything, with no
flag and no edit. MEASURED by the red team on the UNMODIFIED scripts:

- honest stub → `determinism: ok — 5 seat(s)`, exit 0, the recorded number reached
- a stub answering differently on its SECOND process →
  `determinism: FAIL: radius: two processes disagreed on the same input`, exit 1
- a stub answering nothing → `0 bestmove lines carrying a turn token, for 40
  searches`, exit 1 — item 10's control clause, verbatim
- `movetime_check.sh`, same mechanism, a stub reporting `time 99999` →
  `movetime: FAIL: … took 99999 ms (bound 550 ms)`, exit 1

**OPTION F therefore dominates A on every axis the matrix itself named**: it
seeds the exact violation each gate is named for, costs ZERO shipped-code
change, and has no failure mode at all — there is no flag for an operator to
misuse, because there is no flag.

**AND A WOULD HAVE BEEN WORSE THAN A DRAW.** ATTACK 5: an `--engine` flag
BYPASSES the artifact-stream resolution ladder D-250 exists to defend, so A's
test would reach LESS of the gate than F's does. The recommendation was not
merely more expensive; it was narrower.

### The finding that is worst about how this document was written

**OPTION F IS THIS REPOSITORY'S OWN COMMITTED PATTERN, IN A FILE THIS PACKAGE
WROTE EARLIER IN THE SAME SESSION.** `crates/pistol-cli/tests/solver_determinism_gate_tests.rs`
shims `cargo` on PATH exactly this way, to drive a DETERMINISM gate's void
paths, and its own comment explains why shadowing beats removing. The matrix
was written without looking at it. ATTACK 8 puts the knife in: the D-291
measurement this document skipped — `wc -l` on that very file — is the one that
would have surfaced option F, **and the matrix would not have been written at
all**.

### The defects in the premises, each conceded

- **ATTACK 1**: "MEASURED at `21f8833`" is FALSE. Every §2 P1 line number is the
  UNCOMMITTED working tree's, and the offset is exactly this package's in-flight
  `require_tool.sh` change to those two files. CLAUDE.md wants a `git stash
  create` SHA where the work is uncommitted; the red team used `9e1f4ea`.
- **ATTACK 2**: P4's `72.6 s` / `22.5 s` are from `b60c3d3`, twelve commits and
  an eighteen-line change to `determinism.sh` earlier, and cite a gitignored
  artifact.
- **ATTACK 7**: option C's rejection is unsound AS WRITTEN. It argues
  determinism-only, but `movetime_check.sh` reads its bound out of the config, so
  a seeded `movetime_epsilon_ms = 1` DOES trip that gate (measured: the real
  engine at `movetime 1` reported `elapsed=3`). C is still the wrong option — it
  is flaky and works on only one of the two gates — but not for the reason given.
- **ATTACK 8**: option E's cost carries no MEASURED/ESTIMATED mark at all, and
  option D's `ESTIMATED` tags a phrase with no number in it.

### Selection

**OPTION F.** Implemented in `crates/pistol-cli/tests/determinism_gate_tests.rs`:
four cases, both gates, a seeded violation each and item 10's control, and not
one byte of either gate changed.

**THE STRONGEST SURVIVING ATTACK**, recorded because the ADR line has to carry
it: a PATH shim is a test-only mechanism with no shipped guard, so nothing stops
a future gate from resolving its binary some other way and silently leaving
these tests driving a path the gate no longer takes — F buys its zero cost by
depending on D-250's block staying the resolution, and nothing asserts that it
does.
