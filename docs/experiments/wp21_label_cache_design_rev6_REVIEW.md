# REVIEW-design ROUND 5 of 5 (remedies-only, D-585) — `wp21_label_cache_design.md` revision 6. **VERDICT: FAIL — 0 BLOCKING, 1 MAJOR, 2 minor.**

**REVISION READ**: `docs/experiments/wp21_label_cache_design.md` revision 6 at the dispatched
commit **`5c77a8f`** (`5c77a8fefff725a97cbec86422a7d3de381facc5`, branch `dev`). Every code
`file:line` below is read at HEAD; `git diff --stat adb2012 5c77a8f` touches no file under
`crates/*/src` (it adds `tests/wp21_assemble_tests.rs`, `tools/wp21_assemble.py`, and docs), so
the design's `adb2012` citations and HEAD are one tree for every code claim.

**DOES IT STILL MATCH HEAD? YES** — `git rev-parse HEAD` at the end of this review returns
`5c77a8fefff725a97cbec86422a7d3de381facc5`.

**COUNTS: 0 BLOCKING, 1 MAJOR, 2 minor**, plus two nits that constrain nothing. Ten of round 4's
eleven findings are CLOSED against the code; the eleventh (MAJOR 4) is PARTIAL: its rule-4-win
half is correct and re-derived, its forfeit half names a stub behaviour whose report the capture
pass REFUSES at the first ask, so the test row cannot run as written. That one row is the MAJOR.
The mechanism, every refusal, every mutant row and every citation hold.

**PROPORTION, stated because D-585 makes a round-5 FAIL a STOP-and-split**: the MAJOR is one
stub word in one test row (`illegal` → `demands_newgame_per_ask`), pins no mechanism, and was
introduced by adopting round 4's suggested fix (a) without reproducing it — the ledger §1d says
*"every finding reproduced against the code before its fix"*; the FIX was not. Under the arc's
own convention a MAJOR is a FAIL (`wp15b_U1_REVIEW_urev4.md`: *"FAIL — 0 BLOCKING, 1 MAJOR"*),
and this reviewer does not move the threshold after the fact. What the operator does with a
FAIL of this shape is the operator's ruling, not this report's.

**READ**: `CLAUDE.md`; `docs/process.md` in full; the design; round 4's report; `arc3_ledger.md`
§1d; D-576, D-581, D-584, D-585, D-586, D-587; `capture.rs`, `seats.rs`, `exchange.rs`,
`channel.rs`, `schedule.rs:115-150`, `bin/arena.rs:1-120`, `passes.rs`, `bin/stub_engine.rs`,
`openings.rs`, `pistol-core` `axis.rs:51-58`, `coord.rs:18-24,131-133,166-178`,
`turn.rs:85-135`; `tests/capture_tests.rs:236-290,500-770`, `tests/protocol_abuse_tests.rs:
92-93,140-240`; `tools/ci.sh`, `tools/governing_citation_check.sh`, `tools/design_citation_check.py`
(grep only); `docs/rule9_justifications.md`; both artifacts named in §2.4. No cargo was run;
nothing in the tree was edited; this report is the only file written.

---

## DISPOSITION OF ROUND 4's ELEVEN FINDINGS

| # | round 4 said | revision 6 | verified against the code | status |
|---|---|---|---|---|
| BLOCKING 1 | stray placed at a MISS; `n = P0` off by one | row 6: *"`with_seats` sends one `newgame` per spawn (`seats.rs:47`) and `ask` one per ask (`:247`)"*; T4: **`n = P0 + 1`**, `P0` read off `capture::asked_prefixes` | `seats.rs:47` sends `NEW_GAME` inside `with_seats` before `drive`; `capture.rs:247` sends it per ask; play's `exchange::ask` (`:37-38`) sends `position` and `go` only. Stub's count in a capture: 1 + asks-so-far; ask `j` sends `newgame` #`j+1`; game 0's last ask is `j = P0` ⇒ #`P0 + 1`. In play the stub sees exactly one, and `P0 + 1 ≥ 2` since prefix 0 is always asked | **CLOSED** |
| MAJOR 1 | `info` stray is ignored; needs a `bestmove` | row 6: a SECOND `bestmove` in the same write; *"`classify` ignores an unrecognised `info` line (`:197-199`), so only a `bestmove` is read as the answer to a later ask"* | `capture.rs:197-199` returns `Step::Ignore` for `info `; `:266-273` returns the first `bestmove` seen, refusing when `totals` is `None`. A `bestmove` stray at game 1 turn 0 (uncached) is read before the real answer's totals ⇒ refused | **CLOSED** |
| MAJOR 2 | *"no engine in the tree writes an unsolicited line"* false | row 6: *"No `Behave` variant writes a line after its `bestmove`; the one test engine that does (`protocol_abuse_tests.rs:180-199`) is a shell script whose identity no report this pipeline captures can attest"* | all 18 `Behave` arms in `serve` (`stub_engine.rs:414-548`) write at most one line and `continue`, or go through `session.line` whose answers end on `bestmove`; `:180-199` is `doubled.sh`. The REASON given is inexact (minor B below); the fact and the conclusion hold | **CLOSED** (see minor B) |
| MAJOR 3 | `asks`-derived mutant dies at the UNCACHED arm; mode-conditional form is equivalent | §6: three rows — `records` → T2 cached; derived unconditionally → *"T2, uncached arm: the derivation reads fewer than `records`"*; derived only when `On` → *"EQUIVALENT while the cache is live"*, a REVIEW-impl item | uncached run: distinct `position` count < records (game 1 duplicates game 0) and a memo that is never inserted into reads 0; both ≠ `records` ⇒ `asks == records` fails. Mode-conditional form indistinguishable by every row listed | **CLOSED** |
| MAJOR 4 | T6's fixture not producible by one stub behaviour | T6 is two reports: (a) `illegal` on both seats; (b) `honest` over a five-turn opening with P1 at `(-4..0, 0)` | **(b) holds** — re-derived below with my own greedy: turn 6 P2 plays `(-1,5)/(-1,6)`, turn 7 P1's smallest cluster neighbour is `(-5,0)`, six contiguous on `r = 0`; `probe.place` (`stub_engine.rs:332-336`) returns a single. **(a) does not**: `Behave::Illegal` answers every `go` with `bestmove 0,0` and `continue`s (`:527-539`) — NO totals line — so the capture refuses at game 0, turn 0 (`capture.rs:267-272`), exactly as the pinned `bad_bestmove` case does (`capture_tests.rs:560-569`). No capture exists for T1 to compare | **PARTIAL → MAJOR 1 below** |
| minor 1 | D-586 misquotes D-581; §2.4 misnames the corrected phrase | D-587 appended; §2.4 names the SORT phrase and says *"no extra search"* is true | `decisions.md:1230` (D-581) reads *"Two counters, no extra search, in the run log beside the hit rate"* — no sort; `:1242` (D-587) quotes it exactly and attributes the sort phrase to the matrix alone | **CLOSED** |
| minor 2 | §7 was revision history | §7 is now OBLIGATIONS; the `no_tab` fact lives in §2.3 (*"it skips `ask`, not record construction, so `no_tab` (`:359`) runs on every record"*) | `capture.rs:359` is on the record after `ask` | **CLOSED** |
| minor 3 | D-423 duplicates (X1/X1b/X3, `asks`, ~53 %, the `newgame` trick) | rows 2a/2b/4 point to §3; row 2c points to §2.5 for `asks`; *"~53 %"* appears once (§8.4); row 6 owns the count, T4 owns the derivation of `n` | `grep -c '53' design.md` → one hit; each of X1/X1b/X3 described once in §3 | **CLOSED** |
| minor 4 | rule 9 for the new test file | row 9: *"`label_cache_tests.rs` gets an entry of its own"* with the shape named | pattern is `rule9_justifications.md:87` as round 4 said; the entry is pre-registered. (*"nine rows"* is eight — T8 lives in `cold_label_check_tests.rs`; a nit) | **CLOSED** |
| minor 5 | *"at that prefix"* is a race; assert *"within game 1"* | §3: *"the guard is a time-of-check: `unsolicited()` is `try_recv` …"*; T4: *"the cached run refuses **within game 1**"* | the remedy is what round 4 asked for. It does not make the cached arm deterministic — see minor A, which is the sharper form of the same point and answers the dispatcher's question | **CLOSED as asked; minor A** |
| minor 6 | row 4 wording | *"five reachable spellings of a capture line"* | — | **CLOSED** |

---

## RE-DERIVATION (my commands, their scope, their numbers)

| claim | my command (scope) | mine | document's |
|---|---|---|---|
| `newgame` senders in the capture | `cat -n seats.rs` (whole file); `cat -n capture.rs` (whole file); `cat -n exchange.rs \| sed -n 1,80p` | `seats.rs:47` in `with_seats` before `drive` (`:55`); `capture.rs:247` per ask; `exchange.rs:37-38` sends no `newgame` | row 6 ✓ |
| `n = P0 + 1` | arithmetic over the two sites above | ask `j` sends `newgame` #`j+1`; last ask of game 0 ⇒ #`P0+1`; play sees one | T4 ✓ |
| `info` ignored; `bestmove` with no totals refused | `cat -n capture.rs` | `Step::Ignore` at `:197-199`; `totals.ok_or_else` refusal at `:267-272` | row 6, T4 ✓ |
| `doubled.sh` | `sed -n 140,240p protocol_abuse_tests.rs` | `script_engine(... "doubled.sh" ...)` spans `:180-199`; doubles on the FIRST `go` | row 6 ✓ (line range) |
| no `Behave` arm writes after its `bestmove` | `cat -n stub_engine.rs` (whole file, 576 lines) | every deviating arm writes ≤ 1 line then `continue`s (`:516-539`, `:470-480`, `:483-492`); census arms write BEFORE `session.line` (`:418-455`) | row 6 ✓ |
| `illegal` under a capture | `stub_engine.rs:527-539` + `capture.rs:266-273` | `bestmove 0,0`, no totals ⇒ refused at game 0, turn 0 | **T6(a) ✗ — MAJOR 1** |
| a forfeit report the capture CAN walk | `git grep -n demands_newgame_per_ask -- crates/pistol-arena/tests`; `stub_engine.rs:456-481` | `capture_tests.rs:250-261` captures a `demands_newgame_per_ask` self-play; in play the latch clears on `go` (`:460-465`) so the mover's SECOND `position` draws an `error` line ⇒ `exchange.rs:70-75` forfeits; under the capture's per-ask `newgame` it answers honestly | the fix for MAJOR 1 |
| T6(b) rule-4 win | own Python: `NEIGHBOUR_DIRECTIONS` order from `axis.rs:51-58`, `Coord` order `(q, r)` from `coord.rs:18-24`, P1 `(0,0),(-4,0),(-3,0),(-2,0),(-1,0)`, P2 `(0,5),(2,5),(4,5),(6,5)` | turn 6 P2: `(-1,5)/(-1,6)`; turn 7 P1 smallest neighbour `(-5,0)`; `[-5..0]` contiguous six | T6(b) ✓ |
| such an opening passes the loader | `cat -n openings.rs`; `coord.rs:169-171` | negative tokens parse (*"two signed decimals"*); `one_opening` replays (`:190`) — five in a row is not decided; `refuse_symmetry_duplicates` keys the WHOLE opening (`:201-222`); `uniform_turn_count` wants equal lengths (`:225-245`); `turn_cap > 5` (`:86`); `TURN_CAP = 8` in `capture_tests.rs:9` leaves room for the turn-7 win | T6(b) ✓ |
| reader thread sends line-by-line | `cat -n channel.rs \| sed -n 82-111p` | one `read_until(b'\n')` and one `send` per loop iteration; two lines from one chunk are two sends | minor A |
| gate 9 at `ci.sh:109-110`, total `:21` | `/usr/bin/grep -n 'GATE_TOTAL=\|gate 9/\|determinism' tools/ci.sh` | `readonly GATE_TOTAL=20` at 21; `step "gate 9/…"` 109, `gate "determinism"` 110 | §1 ✓ |
| `--partition` landed at `f1acc57` | `git show --stat f1acc57` | `tools/cold_label_check.py` +64/−, its tests, the tranche generator | row 7 ✓ |
| 742 / 347 / 395 | `/usr/bin/grep -nE '742\|347\|395' artifacts/arc3_leverB_41_count_v3.txt` | 742 asked, 347 distinct under all four keys, 395 hits | §1 ✓ |
| sixteen `k = 2` floors, 792 | `/usr/bin/grep -n merges artifacts/arc3_opening_prefix_fold.txt` (scope: the ARTIFACT — round 4 re-derived these from the book with its own group; that script is gone with its scratchpad and the numbers are unchanged text) | `42 51 48 55 47 48 50 44 53 48 51 60 45 56 48 46`, sum 792, min 42, max 60 | §2.4 ✓ |
| ten-record floor is `wp21_prereg.md` §4's | `/usr/bin/grep -nE 'fewer than' docs/experiments/wp21_prereg.md` | `:345` *"a class with fewer than **10 sampled records**"* | row 7 ✓ |
| governing sections exist | `/usr/bin/grep -nE '^#+ ' wp21_throughput_prereg.md` | §2 `:110`, §2.0 `:242`, §4.2 `:527`, §4.4 `:571`, §4.5 `:633`, §7.1 `:757` | GOVERNING line ✓ |
| D-581 vs D-587 text | `/usr/bin/grep -nE '^D-(581\|587):' docs/decisions.md` | `:1230` no sort phrase; `:1242` quotes it exactly | minor 1 ✓ |
| gate 20's lists | `sed -n 40,80p tools/governing_citation_check.sh` | design in `GOVERNING`; `label_cache.rs`, `label_cache_tests.rs` in `PROPOSES`; **`tools/wp21_assemble.py` still in `PROPOSES` though `git ls-tree HEAD` holds it** — the script only exempts, it does not refuse an existing path (`design_citation_check.py:71`), so CI is not red; a housekeeping note for the assembler's commit, not this design | row 9 ✓ |
| rule-9 entry for `capture.rs` | `/usr/bin/grep -n 'capture.rs' docs/rule9_justifications.md` | `:82` *"Every line here is the capture ask"* | row 9 ✓ |

---

## FINDINGS

### MAJOR 1 — T6's forfeit report cannot be captured: `illegal` answers a capture's `go` with no totals line, and the run is refused at game 0, turn 0

**Quoted** (T6): *"T1 over two further reports … one played by `illegal` on both seats, where
every game forfeits at the first engine move and the report still writes"*; (§5 preamble):
*"driving the shipped `arena` and stub unless a row says otherwise"*.

**Evidence.** Every word of the quoted sentence is true — and T1 needs two CAPTURES over that
report, and there are none. `Behave::Illegal` on any `go` writes `bestmove 0,0`, flushes and
`continue`s (`stub_engine.rs:527-539`), never reaching `session.line`, so no `info totals` line
is written. The capture's `ask` returns the first `bestmove` it reads and refuses when `totals`
is `None` (`capture.rs:266-273`): *"game 0, turn 0: the search closed with no totals line this
driver recognised"* — the identical control flow the suite already pins for `bad_bestmove`
(`capture_tests.rs:560-569`). The cached run refuses at the same miss. Round 4's fix (a) cited
`capture_tests.rs:529`'s comment (*"such a run forfeits every game and still writes a report,
and pass 2 walks that report like any other"*) and read "walks" as "captures"; the three tests
under that comment (`:536-569`) assert the walk is REFUSED. The ledger §1d says every finding
was reproduced before its fix; this fix was not.

**Why it is a MAJOR and not a nit.** It is a registered test row of the shipped instrument that
fails on correct code — the same class as round 4's MAJOR 4, and the arc's convention counts a
MAJOR as a FAIL. It pins no mechanism: T6(a) exists because §7 names *"a forfeit"* as a red-team
input, and the cache treats a forfeited game like any other (`asked_prefixes` ignores the flag,
`capture_tests.rs:725-753`).

**FIX** (one word, verified against the code above): `demands_newgame_per_ask` on both seats.
In play each spawn is sent one `newgame` (`seats.rs:47`); the latch clears on `go`
(`stub_engine.rs:460-465`), so the first mover's SECOND `position` draws an `error` line and
`exchange.rs:70-75` forfeits it with `protocol_error` — the report holds the opening plus two
engine turns and writes. Under the capture every ask is preceded by `newgame` (`capture.rs:247`)
and the stub answers honestly, which `capture_tests.rs:250-261` already pins end to end. Reword
T6(a): *"one played by `demands_newgame_per_ask` on both seats, where every game forfeits at
the first mover's second turn and the capture's per-ask `newgame` lets it be re-asked"*.

### minor A — T4's cached arm: *"refuses within game 1"* is a scheduling claim the code does not guarantee; the mutant arm IS deterministic

**This answers the dispatcher's question directly.** The design (§3) says the time-of-check is
*"the reason T4 asserts a refusal within the run of hits rather than at one prefix"*. For the
UNCACHED run that is enough and more: `ask` at game 1 turn 0 reads a `bestmove` stray as the
answer whether or not the guard saw it (`capture.rs:266-272`), so refusal is deterministic. For
the CACHED run under correct code, nothing follows game 1: every prefix is a hit (a `try_recv`, a
`BTreeMap` lookup, a record), the closure returns, `with_seats` calls `shutdown` (`channel.rs:
227-232`: send `quit`, kill, wait — no receive), and the process exits 0. The stray is sent by
the reader thread as a SECOND `send` after the answer's (`channel.rs:86-110`, one `read_until`
and one `send` per iteration). No happens-before exists between that second `send` and any of
game 1's checks, because the main thread receives nothing after game 0's last `bestmove` until
the run ends — and that is structural: a hit is exactly the prefix at which the main thread does
not read the pipe. So the cached arm passes unless the reader thread is descheduled between its
two consecutive sends for the whole of game 1's hits, ESTIMATED tens of microseconds; under core
saturation with wakeup preemption onto the reader's core that window is reachable, and
`cargo test`'s parallel arena spawns are that load. The same class of residual is already
accepted at `channel.rs:191-197` and in `protocol_abuse_tests.rs:167`, where the window is a
whole opponent turn; here it is the width of a hit, which is the width the cache exists to
remove.

**The miss-path-only mutant** (`X3` left inside `ask`, not hoisted): game 1 performs no channel
operation at all, the pipe is never read, `shutdown` reads nothing — **exit 0, deterministically**.
The uncached arm refuses deterministically under every guard mutant. So T4 kills the mutant with
probability 1 − ε and can flake on correct code with probability ε. A deterministic test of the
hoist does not exist without a wait, which D-159 forbids on this path; I looked for one (below)
and found none.

**Why minor.** Round 4 classed the same race as a minor; the code's guarantee is absent rather
than a claim shown false; ε is one preemption between two back-to-back sends when the stray
shares the answer's write. **FIX**: (1) in §3, replace *"the reason T4 asserts a refusal within
the run of hits"* with the true residual — *"the cached arm has no check after game 1's last hit;
T4's cached assertion holds unless the reader thread is preempted between its two sends for the
whole of game 1, a window ESTIMATED at tens of microseconds; the mutant arm has no such
residual"*; (2) register as a REVIEW-impl item that the stub's stray shares the answer's write
SYSCALL — one `write_all` of one buffer through the `StdoutLock` (a `LineWriter` given a single
write ending in `\n` issues one syscall); two `writeln!`s are two syscalls and widen the window
to include the stub process's own scheduling; (3) optionally, close X3's stated residual and
make correct-code refusal deterministic with a drain after `quit` (read to `Closed`; a line
drained before EOF is a stray) — that trades the flake for a probabilistic MUTANT kill by
message, which is the better side of the trade for CI, but it is a mechanism change and not a
remedy, so it is offered and not required.

### minor B — row 6's reason for not using `doubled.sh` is inexact; the conclusion stands

**Quoted** (row 6): *"a shell script whose identity no report this pipeline captures can
attest"*. A script on BOTH seats attests fine: `SCRIPT_HANDSHAKE` (`protocol_abuse_tests.rs:
92-93`) is a literal `weights_sha256`, `self_match` gives both seats one config path, the binary
is one file, so `one_engine` (`capture.rs:144-162`) and `verify_respawn` pass. The true reason a
stub behaviour is needed is that `doubled.sh` doubles on its FIRST `go` (`:190-191`) — game 0,
turn 0, a MISS in every run — and a script counting `newgame`s would be row 6 rewritten in bash
with no `probe.place`. **FIX**: *"the one test engine that does (…) doubles at its first `go`, a
miss in any run, so X3's test needs a behaviour that deviates after a counted `newgame`"*.

### Nits (constrain nothing; IMPL takes them up)

- Row 9: *"nine rows"* in `label_cache_tests.rs` — eight (T1, T2, T3, T3b, T4, T5, T6, T7); T8
  is in `cold_label_check_tests.rs` by row 8's own words.
- Gate 20's `PROPOSES` still lists `tools/wp21_assemble.py`, which HEAD holds; harmless today
  (`design_citation_check.py` exempts, never refuses) and the assembler's commit's to remove.

---

## WHAT SURVIVED ATTACK

- **The `newgame` arithmetic** — re-derived from `seats.rs:47`, `capture.rs:247` and
  `exchange.rs:37-38`; `n = P0 + 1` is the count, and play cannot reach it.
- **The `bestmove` stray.** Uncached: refused at game 1, turn 0 by the guard or by `ask`,
  deterministically. Mutant "guard removed" and "not hoisted": cached run exits 0. §6's X3 row
  is now true as stated.
- **The `asks` mutant trio**, including the honest EQUIVALENT label.
- **T6(b)** — the rule-4 win from the shipped stub, re-derived with my own neighbour order and
  my own `Coord` order; the opening passes `openings.rs`'s replay, symmetry and turn-count
  checks; `TURN_CAP = 8` admits the turn-7 win.
- **D-587** quotes D-581 exactly and moves nothing D-586 decided.
- **§2.3** carries the one load-bearing fact §7 used to; §7 is obligations only.
- **Every citation** in the document reproduces at HEAD (table above); `adb2012` and HEAD are
  one tree for `crates/*/src`.
- **T2's fixture reasoning** (identical games from one stub on both seats, `schedule.rs:134`),
  **T5's fixture** against the loader, **T8's floor** — settled in round 4, unchanged in text,
  not re-litigated.

---

## ATTACKS I ATTEMPTED AND REJECTED

- **A deterministic hit-path stray test exists.** Tried: stray before the totals (consumed by the
  same ask); stray between totals and `bestmove` (returned as the answer, the real one becomes
  the stray — same race); stub exits after the stray (`Closed` is queued AFTER the line, so
  `Empty` is still possible); a second opening so a miss follows game 1 (the un-hoisted guard
  refuses there too — discriminates only by message, racily); asserting on the counts line
  (a refused run prints none). Every construction needs the main thread to have received
  something sent after the stray, and at a hit it receives nothing. Rejected: structural, hence
  minor A rather than a BLOCKING "untestable guard" — the guard is testable to 1 − ε.
- **The mutant arm is racy too.** No: under the mutant the pipe is never read in game 1 and
  `shutdown` does not receive; exit 0 is scheduling-independent.
- **T6(b) is blocked by P2's turn-6 move.** P2's smallest neighbours are `(-1,5)/(-1,6)` for the
  P2 stones I chose; any "far off" P2 set keeps `(-5,0)` and `(-5,1)` empty. The design leaves
  P2's cells to the test builder, which checks the geometry with `pistol-core` before use.
- **The five-in-a-row opening is refused as decided.** `one_opening` replays through the rules
  (`:190`); five is not six. Rejected.
- **`Turn::pair` refuses the fixture's spelling.** `-4,0/-3,0` and `-2,0/-1,0` are ascending in
  `(q, r)`; `coord.rs:169` accepts signed decimals. Rejected.
- **`asks` counted before the lookup rather than at the miss.** Row 2c and §2.5 both say the
  miss increments it, immediately before the call to `ask`; consistent. Rejected.
- **The hoisted guard changes the uncached run's bytes.** It changes a refusal's timing at most;
  a stray-free run has nothing to refuse and the record is built from the same strings. Rejected.
- **MAJOR 2's fix introduced a false claim.** It introduced an inexact reason (minor B) whose
  conclusion — a stub behaviour — is the right instrument regardless. Not a MAJOR: it changes no
  test, no mutant, no criterion.
- **D-587 re-opens D-586.** It corrects one attribution and says so; the floors, the one percent
  and the withdrawal are untouched. Rejected.
- **The 792 table** — reproduced from the artifact only this round; round 4's from-the-book
  re-derivation is on record and the text is unchanged. Not re-attacked.

---

**VERDICT: FAIL — 0 BLOCKING, 1 MAJOR, 2 minor.** The MAJOR is T6's forfeit fixture: `illegal`
cannot be re-asked, so the row's two captures do not exist; the fix is the one stub behaviour
that forfeits in play and answers under the capture's per-ask `newgame`. Everything else round 4
raised is closed against the code, and the mechanism is unchanged and sound. Reviewed at
`5c77a8f`; HEAD still `5c77a8f`.
