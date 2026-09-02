# REVIEW-design ROUND 4 — `wp21_label_cache_design.md` revision 5. **VERDICT: FAIL.**

**REVISION READ**: `docs/experiments/wp21_label_cache_design.md` revision 5, at the
dispatched commit **`b07d774`** (`b07d7744eefebfe81de71e13341ba9746ad8bc21`, branch
`dev`). Every code `file:line` below is read at **`adb2012`**, the revision the design
names for its citations; `git diff --stat adb2012 b07d774` touches four files, none of
them code, so `adb2012` and `b07d774` are the same tree for every code claim.

**DOES IT STILL MATCH HEAD? YES** — `git rev-parse HEAD` at the end of this review
returns `b07d7744eefebfe81de71e13341ba9746ad8bc21`.

**COUNTS: 1 BLOCKING, 4 MAJOR, 6 minor.** Round 4 of five under D-585; round 5 is
remedies-only. The mechanism is sound and every citation reproduces; what fails is
the one test that pins the one guard this package adds (BLOCKING 1), plus three
kill/fixture claims that do not reproduce against the code.

**READ**: `CLAUDE.md`; `docs/process.md` in full; the design; the three prior reports'
headers; `arc3_ledger.md` §1c, F-1.16, §1d; `wp21_throughput_prereg.md` §2, §2.0,
§2.1, §4.2–§4.5, §5, §7, §7.1; `wp21_prereg.md` §1, §4, §5, §6.1;
`matrix_label_cache_key.md` §4; D-562, D-568, D-572, D-576, D-581, D-584, D-585,
D-586; `capture.rs`, `bin/arena.rs`, `passes.rs`, `usage.rs`, `channel.rs`,
`exchange.rs`, `seats.rs`, `bin/stub_engine.rs`, `openings.rs`, `handshake.rs`,
`schedule.rs:122-150`, `tools/cold_label_check.py`, `tests/cold_label_check_tests.rs`,
`tests/common/mod.rs`, `tests/capture_tests.rs:600-760`,
`tests/protocol_abuse_tests.rs:150-240`, `tools/ci.sh`, `tools/determinism.sh`,
`tools/governing_citation_check.sh`, `docs/rule9_justifications.md:82-88`,
`artifacts/arc3_opening_prefix_fold.txt`, `artifacts/arc3_leverB_41_count_v3.txt`.
No cargo was run; nothing in the tree was edited.

---

## RE-DERIVATION (my commands, their scope, their numbers)

| claim | my command (scope) | mine | document's |
|---|---|---|---|
| sixteen per-tranche `k = 2` floors, 792 | own Python (`scratchpad/fold.py`): parse the 4 500-line book myself, own 12-element group (`(q,r)→(−r,q+r)` ×6, `(q,r)→(r,q)` reflect), canonical = min over images of the sorted `(q,r,player)` list; windows `skip 13 + 218·i`, 15×218 + 217 | `42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46`, **sum 792**; whole range 13..3499: exact 2327, symmetry 368 | identical (D-584, D-586, §2.4) |
| `k = 2` stone-set classes = exact classes | same script, third column | 213 209 … 210, equal to exact in all 16 | matches the receipt's reading (so `key_pos`'s in-book floor is 0) |
| D-586's 792 / 93 076 | `python3 -c 'print(792/93076)'` | 0.008509 → **0.85 %** | 0.85 % |
| 93 076 | `3487 × 347/13` | 93 076.05 | `wp21_prereg.md:212` |
| 742 / 347 / 0.5323 / 395 hits | `artifacts/arc3_leverB_41_count_v3.txt`; `395/742` | 0.532345 | §1 one-liner, D-576 |
| word counts 2 680 / 4 747 | `git show b07d774:…design.md \| wc -w`; `git show adb2012:…design.md \| wc -w` | 2680 / 4747 (56.5 %) | §1d of the ledger |
| gate 9 at `tools/ci.sh:109-110`, total at `:21` | `sed -n '15,25p;100,115p' tools/ci.sh` | `readonly GATE_TOTAL=20` at 21; `step "gate 9/…"`/`gate "determinism"` at 109-110 | §1 one-liner ✓ |
| gate 9's seats/budgets | `sed -n '60,95p' tools/determinism.sh \| grep SEATS…` | five `gate_*` seats; `depth_turns 4`, `nodes 200000`, one override `nodes-10000` | §2.1 ✓ |
| `run :326`, `go :333`, `with_seats :338`, loops `:340/:341`, `ask` call `:343`, `normalise :356`, `no_tab :359`, stray guard `:241-246`, `NEW_GAME` per ask `:247` | `cat -n crates/pistol-arena/src/capture.rs` | all at the cited lines | ✓ |
| `arena.rs` `match words :39-86`, `--label-nodes` arm `:51`, `--census` arm `:59-74`, catch-all `:79-85`, `outpath::claim :89` | `cat -n bin/arena.rs \| sed -n 1,120p` | all at the cited lines | ✓ |
| `passes.rs :82-96` prints the summary | `cat -n passes.rs` | `println!` block 82-96 | ✓ |
| `exchange.rs:34` play-pass stray guard | `cat -n exchange.rs` | `channel.unsolicited()` at 34 | ✓ |
| `newgame` senders in the arena | `git grep -n NEW_GAME adb2012 -- crates/pistol-arena` | **`seats.rs:47` (once per spawn, play AND capture) and `capture.rs:247` (once per ask)**; `handshake.rs` sends only `HANDSHAKE` | design: *"Play sends one per spawn and the capture one per ask"* — **the capture sends both**, see BLOCKING 1 |
| "no engine in the tree writes an unsolicited line" | `git grep -n 'nothing had been asked' adb2012 -- crates/pistol-arena/tests` | `protocol_abuse_tests.rs:215`; its `doubled.sh` (`:180-199`) writes a second `bestmove` after the first `go` | design row 6 says none — see MAJOR 2 |
| `Behave` variants all end on `bestmove` | `cat -n bin/stub_engine.rs` (18 variants, `:13-127`) | true of the stub; not of the tree's test engines | — |
| `--partition` landed at `f1acc57` | `git show --stat f1acc57` | `tools/cold_label_check.py`, its tests, the tranche generator | ✓ |
| `canonical_form(&[(Coord, Player)])`, `Turn::pair` canonical | `git grep -n 'pub fn canonical_form\|pub fn pair(' adb2012 -- crates/pistol-core/src` | `symmetry.rs:166`, `turn.rs:119` (doc `:90-96`: `first < second`) | ✓ |
| turn cap 40 → ≤ 79 stones | `wp21_prereg.md:141-157` | cap 40; 1 + 2·39 = 79 | §2.4 ✓ |
| capture goes through `with_seats` | `git grep -n with_seats adb2012 -- crates` | `capture.rs:338`, `replay.rs:106`, `schedule.rs:146` (play: one spawn per game) | row 6's *"one per spawn"* is play's; the capture's spawn is unmentioned |
| D-581 says *"one sort the arena already does"* | `grep -o 'Two counters[^.]*\.' docs/decisions.md` | D-581: *"Two counters, no extra search, in the run log beside the hit rate"* — **no sort** | D-586 attributes the sort phrase to *"§4 and D-581"* — minor 1 |
| rule-9 entry for `capture.rs` | `grep -n -A12 capture.rs docs/rule9_justifications.md` | `:82`, *"Every line here is the capture ask"* | row 9 amends it ✓ |
| gate 20's lists | `sed -n 40,62p tools/governing_citation_check.sh`; `git show b07d774 -- tools/governing_citation_check.sh` | design on `GOVERNING`; `label_cache.rs` added to `PROPOSES` at b07d774 | row 9 ✓ |

---

## FINDINGS

### BLOCKING 1 — T4 places the stray at a MISS, so the guard X3 exists for is untestable as registered

**Quoted** (row 6): *"Play sends one `newgame` per spawn and the capture one per ask"*;
(T4): *"row 6's stub with `n` = the asked-prefix count of game 0 … so the stray
follows game 0's last answer and game 1's first prefix, a hit, finds it"*.

**Evidence.** The capture runs inside `seats::with_seats` (`capture.rs:338`), which
sends `newgame` once per spawn (`seats.rs:47`) before `drive` runs; `ask` then sends
one per ask (`capture.rs:247`). So the stub's `newgame` count in a capture is
`1 + (asks so far)`. Let game 0 have P0 asked prefixes. The `n`-th `newgame` with
`n = P0` is the one sent by ask number `P0 − 1`; *"the `go` that follows it"* is game
0's **second-to-last** ask; the stray therefore sits in the pipe when game 0's **last**
prefix is processed — a MISS in the cached run as in the uncached one. At a miss the
hoisted guard (row 2b) and a guard left inside `ask` (today's `:241-246`) fire at the
same prefix with the same message. **The defect class X3 names — "a guard that stops
running on the prefixes that hit" — preserves T4's criterion**, which is
`docs/process.md`'s vacuous criterion: a mutant that leaves the guard on the miss path
only (i.e. does not hoist it) passes T4 as registered. The §6 row *"X3, the hoisted
guard removed"* is a different, cruder mutant (no guard anywhere), and it dies for a
reason unrelated to hits (see MAJOR 1). The ledger §1d repeats the same off-by-one.

**Fix.** `n = P0 + 1`, stated with the reason: the stub sees `with_seats`' `newgame`
before the first ask, so the `newgame` before game 0's last `go` is the
`(P0 + 1)`-th. Replace row 6's sentence with the true count: *"in play the stub sees
one `newgame` per spawn; in a capture one per spawn plus one per ask"*. Add the
mutant the test is actually for to §6: *"X3 — the guard left on the miss path (inside
`ask`) and not hoisted — dies at T4: the cached run finishes at exit 0, every prefix
of game 1 being a hit"*. T4 should read `P0` off `capture::asked_prefixes(&game0)`
(pub, `capture.rs:28`) rather than counting by hand.

### MAJOR 1 — the X3 mutant's stated observable is false for an `info`-shaped stray, and §3's own defect description needs a `bestmove`-shaped one

**Quoted** (row 6): *"it writes one unsolicited `info` line"*; (§6): *"X3, the hoisted
guard removed — T4: the cached run exits 0 where the uncached refuses"*; (§3 X3):
*"a stray in the pipe at a hit is read as the answer to a LATER miss, or never"*.

**Evidence.** With the guard removed (from `ask` by the hoist, then from the loop by
the mutant) nothing checks the pipe; the next miss's `receive` loop reads the stray
first, `classify` returns `Step::Ignore` for any line starting `info ` that is not a
totals or census row (`capture.rs:197-199`), and the ask continues to the real answer.
**Both runs exit 0.** T4 still fails (it expects a refusal), so the mutant dies, but
not by the observable the row states, and an `info` line is never *"read as the
answer"* to anything — §3's description of X3's damage is only true of a stray that
is a `bestmove` (read as the answer: `totals` is `None` at `:267`, refused as *"closed
with no totals line"*) or a totals line.

**Fix.** The stray is a second `bestmove` line, written in the SAME write as the
answer (the stub's `StdoutLock` is line-buffered, so two `writeln!`s are two
syscalls). Then the uncached run refuses deterministically under every guard mutant,
the cached run under the miss-path-only mutant finishes at exit 0, and the §6 row's
observable becomes true. Keep §3's sentence.

### MAJOR 2 — "No engine in the tree writes an unsolicited line" does not reproduce

**Quoted** (row 6): *"No engine in the tree writes an unsolicited line, so X3 has no
test without this"*.

**Evidence.** `crates/pistol-arena/tests/protocol_abuse_tests.rs:180-199` builds
`doubled.sh` through `script_engine`, which prints `bestmove 5,0/6,0\nbestmove
8,0/9,0` on the first `go` — the D-172 reproducer — and `:215` asserts the forfeit.
Round 3's `git grep -n unsolicited -- crates` and the design's sentence both counted
the wrong population (the word, not the behaviour). The stub row is still the better
instrument (a script cannot share the stub's identity line); the claim that there is
no other is false, and the true statement is the narrower one.

**Fix.** *"No `Behave` variant writes a line after its `bestmove`; the one test engine
that does (`protocol_abuse_tests.rs:180-199`) is a shell script whose identity no
report this pipeline captures can attest, so X3's test needs a stub behaviour."*

### MAJOR 3 — the `asks`-derived mutant does not die where §6 says

**Quoted** (§6): *"`asks` — derived from the records or the memo — T2, cached arm"*.

**Evidence.** With a live cache, `asks` derived from the records (distinct `position`
count) or from the memo (`memo.len()`) EQUALS the counter; T2's cached arm
(`asks < records`) passes. An unconditional derivation dies at T2's **uncached** arm
(`asks == records` fails, since distinct < records or `memo.len() == 0`). A
mode-conditional derivation (`records.len()` when `Off`, distinct when `On`) survives
both arms and every other row: it is observationally equivalent to the counter for
as long as the cache is live, which is exactly §2.5's point — the derivation is only
distinguishable on a DEAD cache, and no registered test has one.

**Fix.** Name the uncached arm for the unconditional derivation; record the
mode-conditional derivation as an equivalent mutant (D-568's receipt must list it as
such rather than claim a kill), and let §2.5's argument carry the prohibition — it is
a REVIEW-impl item, not a mutant.

### MAJOR 4 — T6's fixture is not producible by the shipped `arena` and stub as registered

**Quoted** (T6): *"T1 over a report holding a forfeit and a rule-4 win"*; (§5):
*"driving the shipped `arena` and stub unless a row says otherwise"*.

**Evidence.** `capture::run` requires both seats to attest ONE engine
(`one_engine`, `capture.rs:144-162`) and the stub's `weights_sha256` is the digest of
its behaviour file (`stub_engine.rs:405-408`), so both seats share one `behave`
word. No behaviour both forfeits in one game and wins in another. The suite's own
comment at `capture_tests.rs:633-637` refuses to rely on a stub self-match ending in a
win, and its decided-game cases are built as `RecordedGame` values at the library
level (`:640-668`, `:725-751`), not as reports.

**Fix.** Two reports: (a) `illegal` on both seats — every game forfeits at the first
engine move and the report still writes (`capture_tests.rs:529`); (b) `honest` on both
seats over a crafted five-turn opening in which P1's five stones lie on one axis at
`(−4..0, 0)` and P2's are far away — the stub's lexicographically smallest neighbour
of its cluster is `(−5, 0)`, `probe.place` reports a win, and it plays the single stone
(`stub_engine.rs:319-335`): a rule-4 win from the shipped stub, deterministic, and
five turns like T5's fixture so one loader call takes both. Or say T6 is a
`RecordedGame`-level test and drop *"shipped"* for that row.

### minor 1 — D-586 misquotes D-581, and §2.4 misnames what D-586 corrects

D-586: *"`matrix_label_cache_key.md` §4 and D-581 say 'two counters, no extra search,
one sort the arena already does'"*. D-581 (`decisions.md:1230`) says *"Two counters,
no extra search, in the run log beside the hit rate"* — the sort phrase is the
matrix's alone. And §2.4 says D-586 *"corrects the matrix's 'no extra search'"*; D-586
(1) corrects the SORT claim, and "no extra search" is true — a sort and twelve images
are not a search. Fix both sentences to name the sort.

### minor 2 — §7 is revision history inside the document

Lines 14-15: *"Its revision history is in `arc3_ledger.md`, not here."* §7 (*"Round
1: …"*) is a round-1 disposition. Its one load-bearing fact — a hit skips `ask`, not
record construction, so `no_tab` still runs (`:359`) — is a §2.3 sentence; the rest
is ledger.

### minor 3 — D-423 duplicates

X1, X1b and X3 are each described in §1 (rows 4, 2a, 2b) AND §3; *"asks is
incremented at the call to `ask` and nowhere else"* is in row 2c AND §2.5; *"~53 %"*
is in §3 AND §9.5; row 6 AND T4 each narrate the `newgame`-count trick (and both carry
BLOCKING 1). One owner per claim, the other a pointer.

### minor 4 — rule 9 for the new test file

Nine test rows plus a fixture builder that checks three geometric properties with
`pistol-core` will not fit `tests/label_cache_tests.rs` under the soft cap; row 9
lists only `capture.rs`'s entry. Pre-register the entry (the pattern is
`rule9_justifications.md:87`) or split the fixture into `tests/common`.

### minor 5 — T4's "at that prefix" is a race the design should name

`unsolicited()` is `try_recv` (`channel.rs:198-205`); the stray is enqueued by the
reader thread after the main thread has already received the `bestmove`. At the game
boundary the main thread replays game 1 through `asked_prefixes` (`:341`, a full
`GameState` replay) before the guard runs, which is why the design's boundary
construction is the right one and why the tree's existing stray test
(`protocol_abuse_tests.rs:167`) has passed CI; but *"refused at THAT prefix"* is
scheduling-dependent in principle. State it, and assert in the cached run *"refused
within game 1"* (every prefix of which is a hit) rather than at turn 0. MAJOR 1's
single write narrows the window further.

### minor 6 — wording in row 4

*"five reachable spellings of a capture line ending in `--label-cache`"* — two of the
five (`… --label-nodes n`, `… --census`) do not end in it. *"five reachable spellings
of a capture line"* is the sentence.

---

## SIZE AND D-424

2 680 words is 56 % of revision 4, reproduced. Prose that constrains nothing and
could go, ESTIMATED ~350 words: §2.1's second half from *"The strongest attack"*
onward (D-581 and matrix §4 own it; one pointer); §2.2's first two sentences (the
invariant is `wp21_throughput_prereg.md` §2.0's, already cited); §7 whole (minor 2);
§9 items 2 and 4 (each a restatement of §2.4 with a pointer to it). Everything else
either names a site, a refusal, a test, a mutant, or a divergence, and constrains a
reading. The tables are load-bearing and should stay.

---

## WHAT SURVIVED ATTACK

- **Every `file:line` citation** reproduces at `adb2012` (table above), including the
  two the earlier rounds got wrong (`ci.sh:109-110`, `:21`).
- **The mechanism against the code.** `go` is computed once at `:333` outside both
  loops; `position_line` is the only varying argument; `run` is `pub` so the
  scope invariant must be structural — the mode-in/counts-out shape is right. The
  hit path cannot reach `normalise` and cannot skip `no_tab` (`:359` is on the
  record, after `ask`). X1 sits in the `match` before `outpath::claim` (`:89`), so
  no file is left; X1b at `run`'s first statement is before `one_engine`,
  `verify_engines` and `with_seats`, so before any spawn — T3b is writable through
  `passes::read_report` (pub) and `capture::run`.
- **D-586.** All sixteen floors, 792, 0.85 % and 93 076 re-derived with my own group
  implementation and my own parse of the book; the reading "each tranche against its
  own floor" is exactly what a miss-only, earlier-miss-only counter measures at
  `k = 2` (`exact − symmetry` per tranche, duplicates being hits) and zero at `k = 1`
  and `k = 3`; `key_pos`'s in-book floor is 0. The one-percent line is a stated
  judgement anchored to a measured 0.85 %, not a derivation — it is a registered
  number with a ground, which is what D-581 lacked, and it is sound as a flip clause.
  (3)'s withdrawal of *"settles D-562(2)"* is correct: the counters compare a key with
  itself.
- **T2's fixture.** `schedule.rs:132-134`: even index A is P1, odd B is P1; one stub
  and one config on both seats make the two games one move list, so every prefix of
  game 1 is a hit (and `position start` is a hit for every game after the first in
  any case).
- **T5's fixture** passes `openings.rs`: `refuse_symmetry_duplicates` (`:201-222`)
  keys on the WHOLE opening's `canonical_form`, `uniform_turn_count` (`:225-245`)
  wants five turns each — both stated. The stone-list mutant makes Y's `k = 4` prefix
  a hit and `key_pos` reads 0; the canonical mutant makes Z's a hit and `key_full`
  reads 1; neither is visible to T1 on the honest stub (play-order-independent
  greedy) for the transposition, and the counter, not T1, is the reliable kill for
  the symmetry (T1's *"bestmove in the wrong frame"* is fixture-dependent). Correct.
- **T8**: `partitioned()` (`cold_label_check.py:172-192`) derives hits as later
  occurrences; the `rebuild` helper (`tests:106`) re-digests a cut body; nine hits =
  game 1 whole (TURN_CAP 8 → ≤ 9 prefixes), the tenth = game 2's `position start`.
  Writable beside the existing partition cases.
- **§8's limb-4 finding** is right: `label_cache_count.py` reads a file identical
  cached or not, so `wp21_throughput_prereg.md` §7.1 limb 4 cannot fail.
- **§2.3's divergence** from §4.2's *"the pair the engine returned"* is recorded, and
  the normalised side is the right one (one record-construction site).
- **§3's residual** (nothing checks the pipe after the last prefix) is true of both
  passes and correctly left open.
- **Row 7's floor** is a real criterion: `records_of` voids only an EMPTY class; a
  filter over nine hits prints `9 of 9 … agree` and exits 0 today.

---

## ATTACKS I ATTEMPTED AND REJECTED

- **Rule 1, "absent means off" is a code-side default.** Registered in
  `wp21_throughput_prereg.md` §4.2 as the `--census` precedent, one place
  (`usage.rs`), and the default is §4.4's mechanism. Not a finding.
- **Rule 4, `fold_ms` is wall-clock.** Printed only; no branch reads it; the capture
  file carries no trace. Not on a choice path.
- **Rule 3, a hit is a silent path.** It is the registered behaviour with an external
  referent (§4.4, T1); a self-check that re-asked would cost what it saves. Rejected.
- **`BTreeMap` iteration order.** Point lookups only; no iteration on a choice path.
- **The floor breaks an existing cold-check case.** `the_sample_is_every_stride_th…`
  samples stride 3 over ~36 records (2 openings × 2 games × ≤ 9 prefixes) ≈ 12 ≥ 10,
  and the one-record void case stays a void. Not reproducible without running; the
  implementer should confirm the record count once, since a stub game that wins
  early shortens it.
- **The `with_seats` closure hides the memo from the signature but not from a
  second `run`.** Two `run`s are two closures and two memos; a caller cannot pass one
  in. The structural claim holds.
- **T3's "names neither" for the catch-all.** `arena.rs:79-85` lists the four legal
  shapes and mentions neither `--census` nor `--label-cache`. The mutant "arm removed"
  is distinguishable by message. Holds.
- **`key_full >= key_pos` as a criterion a defect preserves.** It is an invariant,
  not the criterion; T5's `>= 1` / `>= 2` are.
- **A transposition inside ONE game at different stone counts.** Impossible; every
  prefix of a game has a distinct stone count, so the one-opening zero in T5 is real.
- **D-586's denominator: "those tranches' misses" vs 93 076.** The 0.85 % is the
  whole-sweep ratio and the trigger is per-cached-tranche in the same unit; the unit
  is consistent and stated.
- **The 12-image group.** My own rotation/reflection pair reproduced every class
  count, so `canonical_form`'s group is the full dihedral group of the lattice as the
  design assumes.

---

**VERDICT: FAIL — 1 BLOCKING, 4 MAJOR, 6 minor.** The BLOCKING finding is an
off-by-one that makes the package's one new guard untestable as registered; its fix
is one integer and one sentence. Reviewed at `b07d774`; HEAD still `b07d774`.
