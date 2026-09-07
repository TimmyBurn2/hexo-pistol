# WP-2.2 Phase 2a — §0 receipt

The dispatch's six opening actions, each with the output that discharges it.
Every number here is MEASURED at the revision named beside it.

## 1. The log's tail

`tail -n 8 docs/decisions.md` at `0585e25` ends at **D-701
[stopped-branch-not-executable]**. Nothing was pre-numbered.

## 2. The paste block

Seven keys, checked before they were written —
`git grep -q "\[<key>\]" -- docs/decisions.md` returned ABSENT for every one at
`0585e25` — and appended at the next free numbers:

| key | number |
|---|---|
| `anchor-v7-two-arms` | **D-702** |
| `stop-docs-on-dev` | **D-703** |
| `w1-sprt-killed` | **D-704** |
| `two-arm-acceptance` | **D-705** |
| `hex-enum-computed` | **D-706** |
| `phase2-split` | **D-707** |
| `matrix-never-selects` | **D-708** |

## 3. The anchor v7 amendment

D-702's ruling is transcribed verbatim into `sealbot_anchor_v7_protocol.md`'s
**stopped budget section** (§A1's *"WHAT v7 DOES ABOUT THE INEQUALITY"*), and the
NOT-RUNNABLE banner is replaced by one that records the budget as settled BY
RULING rather than by a surviving matrix. `matrix_anchor_v7_budget.md` stays
FELL and is superseded, not revived. **§A2 through §A7 do not move** (D-54y):
`git diff` over the file touches the banner, the governing-revision note, §A1's
forfeit-threshold paragraph and §A1's budget paragraphs, and nothing below §A1.
D-695's *"equal measured movetime per side"* now reads *"equal nominal, measured
reported, legacy arm exempt"*, by the ADR line D-699 named as its first resume
item.

## 4. The clock fix — the verdict sequence, and the confirmation by behaviour

**THE DESIGN VERDICT SEQUENCE, quoted from `sealbot_clock_fix_CLOSURE.md`'s own
table**: `design rev 1` **FAIL** 7 MAJOR / 10 MINOR; `design rev 2` **FAIL**
6 MAJOR / 8 MINOR; `design rev 3` in force. **TWO REVIEW-design FAILs.** Its
`REVIEW-impl` then returned **FAIL** 4 MAJOR / 8 MINOR, of which the closure
records that *"THE ONE THAT MATTERED MOST WAS A REAL BUG"*.

**THE SCOPED CONFIRMATION, run by behaviour and not by form (D-691).** The
reproducer is the finding's own: a shim that hangs BEFORE its stdin loop, so the
referee's `close_stdin(); wait()` never returns. Instrument:
`confirm_wedge.sh`, run in a detached worktree at `0585e25` with its own
matchserver build.

| arm | exit | transcript |
|---|---|---|
| **the fix as shipped** | **0** | `{"detail":"p2 forfeited: engine timeout","event":"game_end","game":1,"kind":"forfeit"}` |
| **the mutant** — `await_ready`'s timeout branch stops killing | **124** | none; the run wedged |

**PASS. The fix is CLOSED.** The control is what makes it one: without the
mutant arm, exit 0 is equally explained by a harness that cannot wedge at all,
and the mutant reproduces the wedge the finding described at the exit code it
named.

**Receipt**: `artifacts/wp22_phase2a/clockfix_confirm/`, six files,
`sha256sum -c` clean, receipt digest
`7678648c5dcf2418a5a0eb320f0faac3835c21ae792723742e737fac2345bb66`.
The worktree was exported before removal (D-469) and `git worktree list` shows
only the main tree.

## 5. Green from the gate's own log

`tools/ci.sh` at `0585e25`, full log
`artifacts/wp22_phase2a/ci_head_0585e25.txt`: **`ci: all gates passed`**,
`EXIT=0`, 21 of 21 gates. The lines a successor reads:

```
decision_key_check: 703 decision keys in docs/decisions.md, no repeat outside the exemption
governing_citation_check: 16 governing document(s), 0 proposed path(s)
label_consistency_check: 6 documents ... every document agrees with itself
```

**THE 703 IS THIS RUN'S OWN AND NOT A STALE FIGURE**, and the reason is worth
one line: `tools/decision_key_check.sh` reads the TRACKED BYTES
(`tools/SHELL_CHECKLIST.md` item 5), so it adjudicated `0585e25`'s blob and not
the working tree that already held the paste block. That is why this run is a
clean baseline for `0585e25` despite being launched beside uncommitted edits,
and why CI is run again at the closing revision (D-674).

## 6. Processes and worktrees

`ps` showed no live job at the start; `git worktree list` showed the main tree
alone at the start and again after the clock-fix worktree was exported and
removed. All Stage-E compute runs under `artifacts/wp22_phase2a/`.
