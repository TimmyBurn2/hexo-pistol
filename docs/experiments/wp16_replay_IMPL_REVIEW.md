# WP-1.6 warm replay — REVIEW-impl

**Revision reviewed**: `1d1322d` ("feat(arena,tools): warm replay -- a second mode on
arena, and Criterion 1''"), together with `63a51bc` ("feat(arena): extract the seat
spawn/setup/teardown sequence, proved pure"), whose purity claim `1d1322d`'s central
claim rests on.

**Does it still match HEAD?** `git rev-parse HEAD` = `1d1322d45d40096e1924df7f958658c5e9c5d4ae`
at the moment this report was finished. **HEAD has not moved.**

**One thing the reviewer did not cause and must record**: the working tree was clean at
`11:12` when this review opened and is *not* clean now — `docs/experiments/wp16_sprt_prereg.md`
carries an uncommitted revision-3 rewrite (`+277/-35`), written at `11:18:20`, i.e. **during
this review**, by a session other than this one. It is not part of either reviewed revision
and is out of scope; it is named here only so that a reader is not surprised by a dirty tree,
and because a pre-registration that names these artefacts as its instruments is being amended
while the artefacts are under review.

**Reviewer**: fresh context, did not implement this work. Model: Claude Opus 5 (1M context).
**Date**: 2026-08-24.

---

## VERDICT: **FAIL**

1 BLOCKING, 5 MAJOR, 7 MINOR.

The extraction is genuinely pure and genuinely non-circular, and code identity between the
replay path and the live path holds at the diff level — the two things this work package
was created to establish are established. What fails is the layer above them: the Python
criterion that turns a replay into a verdict has a demonstrated **EXIT-0-WRONG-ANSWER**
(`tools/SHELL_CHECKLIST.md`'s opening sentence), and the mutation sweep found five behaviours
that the ADR log names by name and that no test in the workspace pins.

---

## What was verified clean, so the findings below are read in proportion

These are stated first because three prior rounds failed on grounding, and a reviewer who
reports only defects gives no evidence that the load-bearing claims were opened.

**The purity proof is NOT circular.** `crates/pistol-arena/tests/seat_setup_identity_tests.rs`
copied from `1d1322d` into a worktree pinned at the pre-extraction revision `7649ba0` passes
there, and passes at `1d1322d`:

    $ git worktree add /home/tom/pistol-verify/preextract-rev 7649ba0
    $ cp crates/pistol-arena/tests/seat_setup_identity_tests.rs \
         /home/tom/pistol-verify/preextract-rev/crates/pistol-arena/tests/
    $ (cd /home/tom/pistol-verify/preextract-rev && cargo test -p pistol-arena \
         --test seat_setup_identity_tests)
    test result: ok. 3 passed; 0 failed; ... finished in 32.42s

    $ cargo test -p pistol-arena --test seat_setup_identity_tests      # at 1d1322d
    test result: ok. 3 passed; 0 failed; ... finished in 32.36s

Six digests, pinned before the extraction, reproduced after it. D-408's flip clause is not
triggered.

**The extraction is behaviour-preserving on a line-by-line reading.** `git show
7649ba0:crates/pistol-arena/src/schedule.rs` lines 170-195 against `seats.rs`:
every channel is still started before any is shaken (the old array literal evaluates
left-to-right; the new `Vec` push loop is the same order); the same `?` points drop the same
channels to `Channel::drop`; `NEW_GAME` still returns `ArenaError::Handshake` on a closed
input; `shutdown` still runs only on a driver that RETURNED. D-408's "ONE ASYMMETRY KEPT"
claim is true of the code.

**Code identity between the replay path and the live path holds at the diff level.**

    $ grep -rn "Channel::start" crates/*/src        # identity.rs:110, seats.rs:62 — nothing else
    $ grep -rn "handshake::shake" crates/pistol-arena/src   # identity.rs:111, seats.rs:73
    $ grep -rn "NEW_GAME" crates/pistol-arena/src   # seats.rs:80 — one sender
    $ grep -rn "ask(" crates/pistol-arena/src       # game.rs:86, replay.rs:193 — two callers

There is no second spawn sequence and no re-implemented `position`/`go`/`bestmove` exchange.
`replay.rs` reaches engines only through `seats::with_seats` and asks only through
`exchange::ask`. (One two-line copy remains — MINOR 8.)

**D-409's three reasons for not reusing `schedule::run` are all true of `schedule.rs`.**
It takes `&ArenaConfig` + `&Openings` (line 59-64), hardcodes its call to `one_game`
(line 97), and runs `score::first_crossing_pairs` early-stop logic (line 110).

**D-411 verified.** `git diff 8ca4063..1d1322d -- tools/wp15b_attribution_check.py` is empty
(0 lines). Run unchanged against a freshly generated dry-run report it still answers
`1a: 16 turns replayed, 10 of them discriminating, 8 of 8 games directly attributed by
replay ... PASS — 0 failure(s)`, exit 0.

**D-412's measured numbers reproduce.** Independently re-run with the pinned binary
(`sha256sum target/release/pistol` = `b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673`,
the digest D-412 names):

| claim | D-412 | this reviewer |
|---|---|---|
| games / divergences | 8 of 8, 0 | 8 of 8, 0 |
| compared turns | 201 | **201** |
| old cold instrument's window | 16 | **16** (12.6x) |
| per-seat node counts equal to the run's | all | **all 16 of 16** |
| checker | PASS, exit 0 | PASS, exit 0, `4 pair(s) directly attributed`, `0 unattributable` |
| replay / run wall | 14.319 / 14.356 = 1.003x | 14.593 / 14.674 = **0.994x** |

The pinned artifact digests also still hold:
`sha256sum artifacts/wp16_replay_drivingtest_{run,replay}.txt` = `83c4db10…`, `9993c671…`.
A strong corroboration the ADR does not itself cite: the run's own report says
`timing_engine a … searches 100` and `timing_engine b … searches 101` — 201, exactly the
replay's `compared_turns` sum, so on this report every searched turn was in fact compared.

**Criterion W-1's non-vacuity argument is not merely argued — I broke it and it fired.**
Mutation R5 (below) makes the replay clear each engine's table before every ask, i.e. a
genuinely COLD replay. It still reports `0 divergence(s)` and exit 0 from `arena --replay`;
the node-count referent catches it:

    warm_attribution_check: DETERMINISM VIOLATION: game 0: seat b spent 853205 node(s) in the
    run and 862238 replaying the identical sequence of positions at the identical budget
    exit 3

`0 divergence(s)` alone would not have been a criterion, exactly as `wp16_sprt_prereg.md`
§8.6 registers. This is the single best thing in the work package.

---

## Findings

### BLOCKING 1 — `clause_b` does not enforce the premise of the proof D-412 says it implements: a pair whose games differ inside the BOOK is still "directly attributed"

D-412 and the checker's own header state the covering argument as:

> If their move lists DIFFER, let t be the first turn they differ at. **Both games agree up
> to t, so the board at t is the same in both**, and the mover at t is the same PLAYER INDEX
> in both …

The code (`tools/wp16_warm_attribution_check.py:535-542`) does not look for the first turn
they differ at. It looks for the first turn they differ at **that is also at or past the
book**:

```python
witness = next(
    (
        at
        for at in range(min(len(one), len(two)))
        if one[at] != two[at] and at >= report["opening_turns"]
    ),
    None,
)
```

A difference *below* `opening_turns` is **skipped**, and the search runs on to a later index.
At that later index the two boards are not identical, so the proof's first step is false of
the pair the checker has just attributed. Nothing else closes the gap: `clause_b` never
compares the pair's two `opening` fields, never compares their book prefixes, and never
checks that the two games swap seats — all three of which the header's own sentence ("Its two
games share a book prefix and swap which label sits in which seat") assumes.

**Minimal reproducer.** A four-game report whose pair 0 declares `opening 0` on both games
while their book second moves differ, and which differs again at a searched turn. Build script
at `/home/tom/pistol-verify/repro/mk2.py`; the two move lists are:

    moves 0 0,0 1,1/1,2 3,3/3,4 4,4/4,5 5,5/5,6
    moves 1 0,0 2,2/2,3 3,3/3,4 4,4/4,5 9,9/9,6
    game 0 opening 0
    game 1 opening 0

    $ python3 tools/wp16_warm_attribution_check.py report.txt replay.txt ./shim
    warm_attribution_check: W coverage: 4 game(s) replayed in full, every clean game's node counts equal to the run's
    warm_attribution_check: W classification: 0 divergence(s), 0 confirmed inversion(s), 0 unexplained
    warm_attribution_check: (b): 0 inert pair(s) excluded by theorem, 2 pair(s) directly attributed at their first differing searched turn, 0 unattributable
    warm_attribution_check: 1b: 4 decided non-forfeit game(s) adjudicated against the move list
    warm_attribution_check: 1c: 4 game(s) and 2 pair(s) rebuilt off the score_a path
    warm_attribution_check: PASS — 0 failure(s)
    exit 0

**And no test exercises the bound at all.** Mutation M8 — delete `and at >= report["opening_turns"]`
outright — passes the whole suite, because every fixture's pair-mates share a book by
construction:

    $ (in worktree /home/tom/pistol-verify/mut, bound removed)
    $ cargo test -p pistol-cli --test wp16_warm_attribution_check_tests
    test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s

**Reachability, stated honestly.** The arena itself cannot write such a report: `one_game`
takes `openings.taken[index / 2]`, so pair-mates share the opening by construction. The defect
bites only on a report that has been corrupted or mis-written — which is precisely the threat
model this criterion exists for (Criterion W-2 is literally a label-transposed report, and
D-406's own history is "two revisions of a pre-registration registered dry-run criteria that
PASSED on an arena mutated to invert the entire verdict"). An unchecked premise inside the
one instrument that decides whether a governed run is a measurement is BLOCKING on that
ground, not on likelihood.

**The fix is small**: refuse when the first difference is below `opening_turns` (or require
the two lists' first `opening_turns` entries to be equal, and the two games' `a_is_p1` to be
opposite), and add a case that seeds it.

---

### MAJOR 2 — `check_coverage` trusts the replay document's own `status` word, so a record that claims `status divergence` with no `divergence` record skips every coverage check and is reported as "replayed in full"

`tools/wp16_warm_attribution_check.py:431-432`:

```python
if record["status"] != "clean":
    continue
```

Everything that makes coverage non-vacuous — `replayed_turns == len(played)` and the node
equality that is the externally derived referent — lives *after* that `continue`. Nothing
cross-checks `status` against the divergence-record set that `read_replay` collected, and the
summary note printed afterwards claims full coverage unconditionally.

**Minimal reproducer** (same fixture as BLOCKING 1, one record edited):

    replay 0 recorded_turns 5 replayed_turns 2 compared_turns 0 nodes_a 999999 nodes_b 999999 status divergence

    $ python3 tools/wp16_warm_attribution_check.py report.txt replay_hole2.txt ./shim
    warm_attribution_check: W coverage: 4 game(s) replayed in full, every clean game's node counts equal to the run's
    warm_attribution_check: W classification: 0 divergence(s), 0 confirmed inversion(s), 0 unexplained
    warm_attribution_check: (b): 0 inert pair(s) excluded by theorem, 2 pair(s) directly attributed at their first differing searched turn, 0 unattributable
    warm_attribution_check: PASS — 0 failure(s)
    exit 0

Game 0 was fed 2 of its 5 turns, compared 0 of them, and reported node counts two orders of
magnitude off the report's — and the instrument printed "replayed in full … node counts equal
to the run's" and exited 0. The shipped `replay_report::render` never writes that combination,
so this is a crafted-document hole rather than a live one; it is nonetheless a false statement
printed by the instrument on an input it accepted.

**Fix**: refuse a `status` that is neither `clean` nor `divergence`, and require
`status divergence` ⟺ exactly one `divergence` record for that index.

---

### MAJOR 3 — driving test (iii)'s forfeit sibling is VACUOUS for the conjunct it names

`a_forfeit_sibling_of_an_inert_pair_is_not_excluded` is the test D-406's punch list and the
commit message both cite for "forfeits are always non-inert". Its fixture does
`games[1].free.pop()` before setting `end = "forfeit"`, so the two move lists differ in
LENGTH — `one == two` is already `False` and `and not forfeited` is never the deciding
conjunct.

**Mutation M6**, deleting the conjunct:

```python
-        if one == two and not forfeited:
+        if one == two:
```

    $ cargo test -p pistol-cli --test wp16_warm_attribution_check_tests
    test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.25s

**And the mutant is not equivalent — it flips a verdict.** On a report whose pair 0 has two
IDENTICAL move lists with one game forfeited (`/home/tom/pistol-verify/repro/mk.py`):

    $ python3 tools/wp16_warm_attribution_check.py report.txt replay.txt ./shim
    warm_attribution_check: FAIL (b) pair 0 (opening 0) is not inert (a forfeit ended one of its games) and its two games never differ at a turn either engine searched, so no replayed turn tells the two seats apart in it
    warm_attribution_check: FAIL — 1 failure(s)
    exit 1

    $ python3 mutant.py report.txt replay.txt ./shim        # only `and not forfeited` removed
    warm_attribution_check: (b): 1 inert pair(s) excluded by theorem, 1 pair(s) directly attributed, 0 unattributable
    warm_attribution_check: PASS — 0 failure(s)
    exit 0

The shipped code is RIGHT. The test that certifies it is vacuous. A one-line fixture change
(drop the `free.pop()`, keep the forfeit) makes it bite.

---

### MAJOR 4 — no test drives the shipped checker against a document `replay_report::render` actually wrote

`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs` builds every report and every
replay document by hand, in Rust, from string literals replicating `report.rs` and
`replay_report.rs`'s spellings. `crates/pistol-arena/tests/replay_tests.rs` produces real
documents but never feeds one to the checker. Nothing in `tools/ci.sh` runs the checker.

    $ grep -rn "wp16_warm_attribution_check" crates/ tools/ci.sh
    crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs:315: .arg(repo("tools/wp16_warm_attribution_check.py"))
    (and doc-comment mentions only)

So a field rename in `replay_report::render` — `compared_turns`, `mover_slot`, `status`,
`divergences` — would break Criterion 1'' in production with `cargo test --workspace` fully
green, because the checker's regexes and its fixtures are two copies of the same stale
description. That is the same defect class this work package exists to remove; it has moved
out of prose and into a pair of agreeing replicas.

The test file argues for hand-written fixtures ("a fixture copied from a run would agree with
the instrument by provenance"), which is right for the CORRUPTION cases and wrong for the
control: one additional case that runs `arena --replay` and pipes both real documents through
the shipped checker would close it. The operator-run chain in D-412 does exactly this by hand;
it is not a gate.

---

### MAJOR 5 — the HALT RULE is pinned by no test

D-409 names it ("THE HALT RULE: the first disagreement ends that game's replay, because
feeding an engine past a divergence feeds it a move it did not choose and desynchronises its
table"), `replay.rs`'s module doc devotes a section to it, and `GameReplay::replayed_turns`
documents the invariant "Short of `recorded_turns` exactly when the replay halted on a
divergence."

**Mutation R3b** — record the first divergence, then keep walking to the end of the move list
(so `replayed_turns == recorded_turns` on a divergent game, and the engines are fed past the
divergence):

    $ cargo test -p pistol-arena --test replay_tests --test replay_refusal_tests
    test result: ok. 4 passed; 0 failed; ... (replay_refusal_tests)
    test result: ok. 3 passed; 0 failed; ... (replay_tests)

Nothing downstream catches it either: `check_coverage` `continue`s on `status != "clean"`
(MAJOR 2's `continue`), so the truncation invariant is never read. The blast radius is bounded
— the FIRST divergence, which is the only one reported, is unchanged — but a named correctness
rule with a written rationale and zero tests is the shape D-406 was written about.

A minimal test: assert `replayed_turns < recorded_turns` on the swapped-label report in
`a_swapped_seat_label_diverges_at_the_first_differing_turn`, which already produces eight
divergent games.

---

### MAJOR 6 — the `NEW_GAME` step of the extracted sequence is pinned by no test, and D-408's branch-coverage claim is false for it

D-408: "The rows are chosen to reach every branch of the extracted sequence deliberately."
The commit message: "spawns BOTH seats of every game through `seats::with_seats` … **so
protocol behaviour including NEW_GAME is inherited and not described**."

**Mutation R2** — `seats.rs:80`, `if channel.send(NEW_GAME).is_err()` → `if false &&
channel.send(NEW_GAME).is_err()`, i.e. the send is never made and its error branch is dead:

    $ cargo test --workspace
    (27 test binaries) test result: ok. ... 0 failed  — every one, including
    seat_setup_identity_tests (3 passed, 32.20s), run_tests (4 passed),
    replay_tests (3 passed), replay_refusal_tests (4 passed)

The whole workspace is green with `newgame` deleted from the one place it is sent.

This is D-406's own MAJOR 2, recorded verbatim in the ADR log — "test (i) cannot pin the
NEW_GAME-inheritance question at all, since a functional no-op on a fresh process passes it
identically whether the setup is right or wrong, and revision 3 deleted revision 2's own
protocol-drift check without replacing it" — and the implementation did not close it. The
*structural* claim (one function, two callers) is true and is the part D-407 asked for; the
claim that the six pinned scenarios reach every branch is not.

`seats.rs`'s own doc concedes the send is a no-op on a fresh process, so this is not a live
correctness defect today. It is an uncovered branch under a coverage claim, and the stub
engine already logs the lines it receives — asserting that each spawn is sent `newgame` is
cheap.

---

### MINOR 7 — a broken INSTRUMENT is reported as the ENGINE's determinism failing (checklist item 12)

MEASURED. Mutation R5 makes the replay cold (`channels[engine].send(NEW_GAME)` before every
`ask`). The engine is fine; the replay driver is not. The checker says:

    warm_attribution_check: DETERMINISM VIOLATION: game 0: seat b spent 853205 node(s) in the run
    and 862238 replaying the identical sequence of positions at the identical budget
    warm_attribution_check: this is the engine's own instrument-mode guarantee failing
    (CLAUDE.md rule 4), not an attribution question.
    exit 3

Node inequality on a clean game has two causes — engine non-determinism, or the replay not
reproducing the sequence — and the message asserts the first by name. Item 12's whole subject
is a reader who "goes looking for a defect in the subject" because the gate spelled an
instrument problem as a subject problem. The exit code is distinct and the registered
consequence (prereg §5: "WP-1.6 does not proceed until it is understood") forces investigation
either way, which is why this is MINOR rather than MAJOR — but the sentence should name both
causes.

### MINOR 8 — the mover-index arithmetic is copied, not shared

`replay.rs:191-192` duplicates `game.rs:82-83`:

```rust
let mover_is_p1 = state.to_move() == pistol_core::Player::P1;
let engine = usize::from(mover_is_p1 != game.a_is_p1);
```

The comment says so ("the same arithmetic `game::play` does"). Two lines, and it is
self-correcting in practice — a divergence between the two would make every replayed game
diverge and the driving test would fail — but it is literally the pattern D-406's BLOCKING 1
named ("a copy that makes the same calls today is not an inheritance"), left in the one file
whose reason for existing is to not do that. A `fn seat_of(mover_is_p1: bool, a_is_p1: bool)`
in `game.rs` called from both would cost nothing.

### MINOR 9 — rule 9 unenforced on the largest file in the workspace

`tools/wp16_warm_attribution_check.py` is 721 lines with no `RULE9-JUSTIFICATION:` comment.
`tools/file_justification_check.sh` enumerates `git ls-files -s -z '*.rs' '*.sh'`, so `.py`
is outside the gate entirely. CLAUDE.md rule 9 says "Files", not ".rs and .sh files".
Precedent exists (`wp15b_attribution_check.py`, 450 lines, also unmarked), so this is a
standing gap the commit widens rather than one it creates. Either add the justification
comment or extend the gate's pathspec.

### MINOR 10 — checklist item 12 obligation 3 partially unmet in the new suite

"A test that drives a gate asserts on the code it expects AND says, in the failure message,
what the other codes would have meant." Met by
`the_dual_engine_probe_tells_a_determinism_violation_from_an_inversion` and by
`documents_that_are_not_about_each_other_are_a_void_and_not_a_finding` (both name the wrong
reading explicitly). Not met by `an_inert_pair_is_excluded_by_the_theorem_and_its_cross_check_is_a_no_op`
(`assert_eq!(code, Some(0), "{}", said(&out))`), `a_clean_game_that_spent_different_nodes_replaying_is_a_determinism_violation`
(`Some(3)`, same bare message) and `an_inert_pair_the_report_did_not_score_one_all_is_a_finding`
(`Some(1)`, same). `replay_refusal_tests.rs` meets it in full.

### MINOR 11 — three shipped paths with no test

- `replay_report::render`'s `warm_replay_aborted` branch (`covered N of M`, `aborted <name>`)
  is never produced by any Rust test; only simulated by a string edit in the Python suite.
- The `engine <slot>` whitespace-path refusal (`transcript.rs:258`, the twelve-word check) has
  no case in `replay_refusal_tests.rs`, though D-409 names "a path with whitespace this format
  cannot round-trip" among the refusals.
- `check_coverage`'s forfeiter over-spend arm (`if again > ran: violation(...)`) is reached by
  no fixture.

### MINOR 12 — the checker re-spells the wire protocol rather than deriving it

`cold_answer` builds `"position start moves " + …` (line 216) and `read_report` builds
`f"go nodes {budget_value}"` (line 125). `exchange::position_line` and
`transcript.rs:231`'s `format!("{} nodes …", pistol_cli::protocol::GO)` are the sources of
truth. A change to either leaves the cold probe silently probing with a stale spelling. The
seam is a language boundary and D-411 owns the probe deliberately, so this is not fixable by
sharing — but it is one more place where protocol behaviour is *described*, and it should be
named in the ADR as such rather than only as "the same shape wp15b already sends".

### MINOR 13 — "asks each seat at every turn it searched" is false for a forfeited game

The commit message's headline sentence. A forfeiting engine's last, refused ask has no
recorded move, so `walk` never replays it. The checker's own comment at line 439-442
acknowledges this precisely and handles it ("only the OTHER seat's count is comparable and the
forfeiter's may only be lower"); the commit message and D-409 do not carry the qualification.

---

## Mutations run, and what caught them

Run in `/home/tom/pistol-verify/mut` and `/home/tom/pistol-verify/mut2`, git worktrees at
`1d1322d`, never in the live tree.

| # | mutation | caught by | diagnostic? |
|---|---|---|---|
| R1 | `replay::walk` accepts any answer (comparison deleted) | `replay_tests::a_swapped_seat_label_diverges_at_the_first_differing_turn` | yes — prints the whole clean document |
| R2 | `seats::with_seats` never sends `NEW_GAME` | **nothing** (`cargo test --workspace` fully green) | — **MAJOR 6** |
| R3 | divergence neither recorded nor halted | `replay_tests` (ii) | yes |
| R3b | divergence recorded, walk does NOT halt | **nothing** | — **MAJOR 5** |
| R4 | `replay::verify_engines` is a no-op | `replay_refusal_tests::a_report_this_mode_cannot_answer_about_is_refused_by_name` (`binary_digest`) | yes — names the void/finding distinction |
| R5 | replay clears each table before every ask (COLD) | stub tests: **nothing**; real engine + checker: **exit 3** | yes, and it validates Criterion W-1 |
| M6 | `clause_b` inert test drops the forfeit exclusion | **nothing** | — **MAJOR 3** |
| M7 | `classify` always reports a CONFIRMED INVERSION | driving test (v) | yes — quotes both exit codes' meanings |
| M8 | `clause_b` witness ignores the `>= opening_turns` bound | **nothing** | — **BLOCKING 1** |
| M9 | node cross-check compares the report's value to itself | `a_clean_game_that_spent_different_nodes_replaying_is_a_determinism_violation` | yes |

"Advance by the ENGINE's answer instead of the recorded move" was considered and **not run**:
under the halt rule the two are equal at every turn that is reached, so the mutant is
semantically equivalent to the shipped code and is not a valid mutation. R5 (cold replay) was
substituted as the warmth-breaking mutation, and it is the sharper test.

---

## `tools/SHELL_CHECKLIST.md`, answered by name for `tools/wp16_warm_attribution_check.py`

The script is Python, not shell under `set -euo pipefail`, so items 1-7 transfer only as far
as their *reasons* do. Stated individually rather than dismissed as a block:

1. **Command substitution whose status is DISCARDED** — TRANSFERS IN SPIRIT, MET.
   `cold_answer` does not read `said.returncode`, so the child's status is effectively
   discarded — but it checks the *shape* of the value (`if len(best) != 1: die(...)`), which is
   what item 1 actually asks for, and refuses by name into the void code.
2. **Pipeline in a `then` body** — DOES NOT TRANSFER. Python has no `set -e`; every failure is
   an exception, and `__main__` funnels `KeyError/ValueError/IndexError` into `die` (exit 2).
3. **`grep` under `pipefail`** — DOES NOT TRANSFER to the Python. It DOES transfer to the bash
   shim the test suite writes, and that shim gets it right:
   `LINE="$(grep '^position start moves ' || true)"` with a comment saying an empty result is
   legitimate, and a named refusal on the next line.
4. **`LC_ALL` and guard direction** — DOES NOT TRANSFER (no character classes). The related
   concern is met elsewhere: `read_report`/`read_replay` deliberately use `text.split("\n")`
   and not `splitlines()`, with a comment naming `\r \x0b \x0c U+2028 U+0085` — the
   allow-list-shaped reasoning item 4 asks for.
5. **Index vs working tree** — DOES NOT TRANSFER; the script reads two operator-named files and
   touches no git object.
6. **A sweep by prefix must own the prefix** — DOES NOT TRANSFER; the script deletes nothing.
7. **Traps** — DOES NOT TRANSFER; no trap, no cleanup.
8. **One spelling per number, one refusal per reason** — TRANSFERS, MET. `fields()` refuses a
   repeated key by name; `only()` refuses "more than one … so there is no one answer to read";
   every numeric read is an anchored `(\d+)`. On the Rust side `arena.rs::workers_of` validates
   the SPELLING (`parsed.to_string() != word`) with its own test over `04 +4 0 four 4.0`.
   Three reasons, three refusals: an unreadable file, a non-UTF-8 file and a wrong first token
   are three separate `die`s.
9. **What reaches a record is caller-controlled** — TRANSFERS, MET on the read side.
   `divergence_line <index> <free text>` is the only unquoted free-text record and it is last
   on its own record kind; `"divergence_line 0 x".startswith("divergence ")` is `False`, so it
   cannot be misread as a `divergence` record. An engine label containing whitespace is
   refused upstream by `transcript.rs`'s twelve-word check before it can reach a record.
10. **THE COVERAGE RULE** — **MET LITERALLY, WEAK IN SUBSTANCE.**
    `crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs` drives the SHIPPED script
    (`repo("tools/wp16_warm_attribution_check.py")`, not a copy) in a scratch directory, and
    it has **two controls**: `a_clean_replay_of_an_honest_report_is_attributable`, and the
    unedited re-run placed LAST in `documents_that_are_not_about_each_other_are_a_void_and_not_a_finding`
    with the comment "so the cases above cannot all be passing against a checker that refuses
    whatever it is handed". A pass cannot come from a checker that refuses everything.
    What the coverage does not reach is MAJOR 3 (the named conjunct is never the deciding one),
    BLOCKING 1 (the book bound is never exercised) and MAJOR 4 (no real document ever reaches
    the script).
11. **Containment guard on caller paths feeding a delete or overwrite** — NOT APPLICABLE, and
    the enumeration is the evidence: the script has **no** destructive site. `open(..., "rb")`
    twice, `subprocess.run` once, `print` otherwise. `grep -n "os.remove\|shutil\|open(.*[\"']w"`
    over the file returns nothing. The Rust side's one write is `outpath::claim`, an `O_EXCL`
    create tested by `an_existing_out_path_is_refused_before_any_engine_is_spawned`, which also
    asserts the pre-existing file's bytes are untouched.
12. **VOID vs FAIL, by name** — **MET IN THE CODE, PARTIALLY MET IN THE TESTS.**
    - *Obligation 1, a code per kind*: four kinds, four codes, all reachable and all distinct,
      and I reached each of them: `0` (control, above), `1` (MAJOR 3's reproducer),
      `2` (`documents_that_are_not_about_each_other…`, seven seeded cases), `3` (R5's cold
      replay and mutation M9's test). The usage block in the docstring states all four.
      `violation()` prints "It is bigger than this work package and it is NOT counted as an
      attribution failure" before exiting 3.
    - *Obligation 2, preflight and void early*: the script writes no scratch, so there is
      nothing to preflight for space. What it does preflight is right: the engine binary is
      only run at `cold_answer`, and an `OSError` there is a void, not a finding. On the Rust
      side `arena.rs::replay_pass` stats the source before reading it and refuses a non-regular
      file by name, so a FIFO cannot hang a read that has no watchdog yet.
    - *Obligation 3, the distinction survives the seam*: partially — see MINOR 10.
    - Related and worth citing: D-410's design decision to replay the recorded move lists
      through pistol-core on the DOCUMENT, before any engine is spawned, is item 12 applied
      correctly — an illegal recorded move is a corrupt document (exit 2, no document), never a
      divergence some engine is blamed for.

---

## Things this implementation claims that I could NOT verify

1. **~~`tools/ci.sh`: all 16 gates passed~~ — VERIFIED after all.** `bash tools/ci.sh` in a
   clean worktree at `1d1322d` finished while this report was being written:
   `ci: all gates passed`, exit 0, gate 16/16 reached. Two lines worth quoting because they
   bear on findings above: gate 13 `arena_smoke: ok — 8 games over 4 openings, distinct-n 4,
   verdict inconclusive_degenerate, three runs agree on the verdict block at 1 and 2 workers`;
   gate 14 `file_justification_check: 279 tracked .rs/.sh files, 37 over the cap, all
   justified` — 279 tracked files, none of them `.py`, which is MINOR 9 in the gate's own
   words.
2. **D-412's wall times to three digits** (`14.356 s` / `14.319 s`). Not reproducible as a
   number — my replication of the same chain on the same machine gave `14.674 s` / `14.593 s`,
   ratio `0.994x` against their `1.003x`. Both support the design's DECLARED "~1x"; neither is
   a stable third digit, and the ADR states them as if they were. The claim's SUBSTANCE
   ("~1x the original run") is verified; its precision is not.
3. **D-408's digests as *recorded at* `7649ba0`.** I verified the stronger property that
   matters — the same test file passes at both revisions, so the pinned constants are not
   fitted to the post-extraction code. I cannot verify the historical claim about *when* the
   recording run happened, and no artefact could establish it.
4. **That the six scenarios reach every branch of the extracted sequence.** Disproved for
   `NEW_GAME` (MAJOR 6). For the others — spawn, handshake, `verify_respawn`, drive,
   `shutdown`, and the failed-driver/`Drop` asymmetry — I read the code and the scenarios
   against each other and believe the claim, but I did not mutate each one individually.
5. **`crates/pistol-engine/src/instance.rs`: `set_position` never touches the searcher, only
   `new_game` clears it** — the premise of the whole warm/cold distinction, cited by
   `replay.rs`, the checker's header and D-409. Out of my reviewed scope; I did not open it.
   R5's measurement is consistent with it (clearing the table before every ask moved every
   node count), which is indirect corroboration and not a check.

---

## What a fix round would need to do

1. **BLOCKING 1** — make `clause_b` refuse a pair whose first difference is inside the book
   (or whose two games disagree on `opening`, or whose two games do not swap seats), and seed
   a case for it.
2. **MAJOR 2** — validate `status` against the divergence-record set before `check_coverage`
   is allowed to skip a record.
3. **MAJOR 3** — drop the `free.pop()` from `a_forfeit_sibling_of_an_inert_pair_is_not_excluded`
   so the forfeit conjunct is the deciding one, or add a second sibling that keeps the lists
   equal.
4. **MAJOR 4** — one case that runs `arena --replay` and pipes both REAL documents through the
   shipped checker.
5. **MAJOR 5** — assert `replayed_turns < recorded_turns` on the already-existing swapped-label
   divergent games.
6. **MAJOR 6** — have the stub record the verbs it receives and assert `newgame` per spawn, or
   record in the ADR that the branch is deliberately unpinned and why the six-scenario
   "every branch" sentence is withdrawn.

Findings 7-13 are MINOR and can travel with the above or be recorded as accepted.
