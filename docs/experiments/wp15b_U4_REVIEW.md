# REVIEW-design — `docs/experiments/U4_soundness_instrument.md`, u-rev 5

**Pinned revision reviewed: `35aab95`.** Reachable and confirmed:

```
$ git rev-parse HEAD
35aab9507d0c0e425da02ed7d9fac763398fa52c
$ git status --porcelain
(no output)
```

**Does it still match HEAD? YES** — `35aab95` is HEAD and the tree is clean; every
command below was run against that state.

**Subject:** `docs/experiments/U4_soundness_instrument.md`, **u-rev 5**, 855 lines
(`wc -l`).

**Context was fresh.** I did not author this unit, the superseded design, the carve,
either matrix, either red-team round, or any earlier review. I read `CLAUDE.md`
first and hold this unit to its Hard rules and Process section.

**Reproducer discipline.** Every finding below carries a command I ran and its real
output. The one probe that needed a new source file ran in a separate `git worktree`
at `/home/tom/u4-review-wt` (never `/tmp`), removed at the end; the live tree was
verified clean afterwards. My own numeric claims are marked **MEASURED** or
**ESTIMATED**. I modified no repository file but this one, and did not commit.

---

## VERDICT: **FAIL**

**3 BLOCKING, 3 MAJOR, 5 MINOR.**

The unit's *substance* is in good order. Both stopped selections are folded
accurately against the reports they summarise — I checked the M3 fold and the M4
fold clause by clause against `matrix_M3_REDTEAM.md`, `matrix_M4_REDTEAM.md`,
`matrix_M4_REDTEAM_round2.md`, D-317 and D-318, and found no overstatement and no
softening of a single attack. MAJOR 8's repair is real: the rebuilt witnesses are
the ones the test pins, the test replays through `GameState` and does not
re-implement rules, and the parity reasoning is right against pinned rule 3. B4's
corrected sentence is true. B3's four named gates are coherent as a set and the
legacy lookup resolves every surviving `§8.3(a)`-style citation in the tree. Every
MEASURED number I re-ran reproduced — including three I re-took on the machine.

The failures are all of one class, and it is the class this WP keeps failing on: **a
change landed in one place with the claims resting on it left un-re-read elsewhere.**
The u-rev 4 and u-rev 5 folds were appended without re-reading the head block, the
lineage table, U4-T, U4-M or the U4-Z lead-in, so the document's most-read surfaces
still describe the u-rev 1 state, and its two registered *test rows* and its *cost
row* still carry S-E as adopted with no caveat at all. And B4's repair, which exists
to stop a count being restated, restates the count in its own closing clause.

---

# PER SCOPE ITEM

## SCOPE 1 — the named-gate wiring (D-316, B3's repair). **PASS, with one MINOR.**

**What I did.** Read §8.2, §8.3 and §8.7 as a set. Checked each of the four gates has
exactly one definition site. Diffed §8.7's enumeration against §8.3's table. Grepped
the whole repository for surviving letter citations. Reconstructed the retarget count
from the superseded document in both directions.

**Evidence — the four gates are each defined once.** §8.3's table:

| The four gates | Where it is specified |
|---|---|
| THE TACTICAL SUITE UNDER STAGED | §8.3 below, first bullet |
| THE DIFFERENTIAL GATE | §8.2 |
| THE COLONY FAMILY | §8.3 below |
| THE PATTERN FIXTURES UNDER STAGED | §8.3 below |

§8.7's wiring names exactly those four and no fifth. The tactical suite is in fact the
first bullet after the table. The config bullet is explicitly labelled *"A CONFIG
STATEMENT, NOT A FIFTH GATE"*. No gate is defined twice.

**Evidence — the lookup table is correct against the source it claims.** §8.3 maps
`(a)`→tactical suite, `(b)`→differential gate, `(c)`→colony family, `(d)`→pattern
fixtures. Against `ec8f7fb`:

```
$ git show ec8f7fb:docs/experiments/wp15b_design.md | sed -n '501,505p'
The minimum bar is fixed by the brief: (a) the tactical suite at 100 % of its
pre-registered thresholds under Staged; (b) a differential gate against
full-width r2 at depths 1..=3 for mates and forced blocks; (c) a colony fixture
family of ≥ 6 cases; (d) the INTEG §5 pattern fixtures under Staged. The matrix
is about the INSTRUMENT for (b), which is the only part with real options.
```

All four map correctly.

**Evidence — every surviving legacy citation resolves.** MEASURED:

```
$ grep -rn "gate (\(a\|b\|c\|d\))\|(a)–(d)\|(a)-(d)\|§8\.3(" docs/ tools/ crates/ configs/
```

returns hits only in `restructure_matrix_15b.md` (2), `restructure_selection_15b.md`
(1), `wp15b_design_rev7_REVIEW.md` (8), `wp15b_U3_REVIEW.md` (1), `docs/decisions.md`
D-316 (1) and U4 itself. Every one is `(a)` or `(b)`, and both resolve through §8.3's
table. **No orphan.** No `(c)`/`(d)` citation exists outside U4.

**Evidence — no live letter-addressing in the other units.** MEASURED, U3 §10's two
sites are retargeted (`U3_tier_t.md:261`, `:266` now read *"**U4** §8.3's TACTICAL
SUITE gate"*), and U1, U2 and `WPQ_seed.md` contain none. One live letter-address does
survive inside U4 — see MINOR 7.

**Evidence — the retarget count, verified in BOTH directions.** The disclosure says
the cost cell predicted three and execution found six. Enumerating every gate-letter
occurrence in the superseded document:

```
$ git show 6feb40a:docs/experiments/wp15b_design.md | grep -n "8\.3(\|gate (\|(a)\|(b)\|(c)\|(d)"
113,127,132,142   §0 (DROPPED by the owner table)
324–327           §4.4's own matrix rows (U1's, a different matrix)
478               §5.2's "D-257 (a)/(b)" (U2's, not a gate)
1031              `Minimal([One(c)])` — code, not a letter
1151,1231,1234    §8.3's own bullets (inside §8)
1279              §8.7's wiring (inside §8)
1354,1359         §10 (U3's) — 2 sites
1764              §15 item 15 (U4's) — 1 site
```

Outside §8, in text a unit owns, that is **three** superseded sites (U3 §10 ×2, item
15 ×1). The other three are carve-authored prose (the head's B4 sentence, U4-A's
lineage row, U4-Z's lead-in). **Total six. I found no seventh.** In particular I
checked U4-T (the five test rows are byte-verbatim from the superseded §11 and carry
no letter), U4-M/§12 item 1 and §11.6 — none needed a retarget. **The count SIX is
correct.** Its stated *diagnosis* is not — MINOR 8.

**Result: PASS.**

---

## SCOPE 2 — B4. **PASS, but the sentence carries the B5 defect — see BLOCKING 3.**

**What I did.** Read the corrected sentence at U4:307–312 against `U3_tier_t.md` §10's
config table, row by row.

**Evidence.** U4 now says:

> **BOTH staged TACTICAL configs disable the quiet cut** (`quiet_top_k` above the
> whole pool): `tactical_staged_v0.toml` for the fifteen `instrument_v0` cases and
> `gate_staged_v0.toml` for the five gate cases. The other two staged documents —
> the SPRT seat and the play config — keep the cut

U3 §10's table (`U3_tier_t.md:259–263`):

| document | `quiet_top_k` | cut |
|---|---|---|
| `instrument_staged_v0.toml` | 16 | binds — SPRT seat |
| `tactical_staged_v0.toml` | **1024** | DISABLED — the 15 `instrument_v0` cases |
| `gate_staged_v0.toml` | **128** | disabled — the five `depth_turns 3` cases |
| `play_staged_v0.toml` | 16 | binds — movetime |

Every clause matches. The underlying case split also reproduces on the shipped
fixture — MEASURED:

```
$ grep -c "^config configs/instrument_v0.toml" crates/pistol-cli/tests/fixtures/tactical_v0.txt
15
$ grep -c "^config configs/gate_v0.toml" crates/pistol-cli/tests/fixtures/tactical_v0.txt
5
$ grep -c "^case " crates/pistol-cli/tests/fixtures/tactical_v0.txt
20
```

so `require 20` = 15 + 5, both under cut-disabled configs. **The B4 repair is
correct and the derivation it feeds survives.**

**But the same sentence restates the count U3 §10 owns exclusively** —
*"which is why there are four and not three"* — which is BLOCKING 3.

**Result: PASS on truth, FAIL on the no-restatement condition.**

---

## SCOPE 3 — THE TWO STOPPED SELECTIONS

### (a) Does the fold say what the reports say? **YES. No overstatement, no softening.**

**M3 (§8 head, D-317).** Checked clause by clause against `matrix_M3_REDTEAM.md`:

| U4's claim | The report |
|---|---|
| nine options: "the five above plus S-F, S-G, S-H and a null row" | §2 lists S-A…S-I; S-I is *"no differential gate"*. **Matches.** |
| "VERDICT: every stated option falls" | §1: *"EVERY STATED OPTION FALLS."* **Matches.** |
| "the matrix is STOPPED and no survivor is forced" | §1: *"The architect has pre-committed to stopping rather than forcing a survivor; that is the honest outcome here."* **Matches.** |
| the decisive attack, `{(-1,0),(-1,5),(4,0)}` vs `{(-1,0)}` | §1 and F2, same two sets. **Matches.** |
| the four missing rows, "two of those four are immune" | §6: *"two of them (F6, F8) are immune to the attack that kills S-E."* **Matches, and names the same four.** |

The fold's MEASURED cells all reproduce:

```
$ git show 6feb40a:docs/experiments/wp15b_design.md | grep -c "^| Option |"
3
$ git show ec8f7fb:docs/experiments/wp15b_design.md | grep -o "S-E" | wc -l
0
$ grep -c inclusion docs/research/threat_calculus_v1.md
0
$ grep -n "DEF-T" docs/research/threat_calculus_v1.md
30:| DEF-T | threat number t(F) | exact **minimum hitting set** over plan family F …
$ sed -n '223,226p' crates/pistol-solver/tests/common/reference.rs
    /// The inclusion-minimal covers of the attacker's hot windows, by the
    /// definition: every subset within budget that covers, minus every one with
    /// a proper subset that also covers.
    pub fn blocking_covers(&self, defender: Player, budget: HitBudget) -> Cover {
```

The two sets in the decisive attack are moreover *derivable from the pinned witness
test's own output* — `blocking_covers(P2,Two) = Minimal([One((-1,0)), Two{(-1,5),(4,0)}])`
gives the inclusion-minimal union `{(-1,0),(-1,5),(4,0)}` and the minimum hitting set
`{(-1,0)}`. **Independently confirmed.**

The fold omits four of the red team's kills (F1's non-reproducing "8 plain `assert!`",
F4's D-115 breach, F9, F10, and F3's 8× dilution). That is a summary's licence and
D-317 carries them — **except that F4 bears directly on a mechanism U4 still
registers as a test, which is BLOCKING 2.**

**M4 (§9 head, D-318).** Checked against `matrix_M4_REDTEAM_round2.md`:

| U4's claim | The report |
|---|---|
| "Options SURVIVED both rounds, so this is not M3's stop" | Round 1 §1: *"the recommended option itself survives on repaired grounds"*; round 2 §1: *"N-E, N-J, N-K, N-L and N-F all survive as options."* **Matches.** |
| (i) revision 2's reason for existing is FALSE; D-288 relabels D-252 (c) as "NO OPTION SELECTED" | R1, verbatim. **Matches.** |
| (ii) one ground argues equally for five options | Round 2 §1: *"ground 2 does not discriminate N-E from N-J, N-K, N-L or N-F."* **Matches** (five options total). |
| (iii) the closed-enum selector dominates on the matrix's own guard trigger, excluded for two revisions | R9(a), verbatim. **Matches.** |
| the third round must carry "the closed-enum selector and the corpus-fixture binding" | R9(a) and R9(b). **Both named. Matches.** |

One clause in (ii) is the author's own addition and is not in either report — MINOR 10.

**Result: PASS.**

### (b) Does U4 still read as though S-E / N-A is ADOPTED? **YES, at four sites.**

This is **BLOCKING 2**. Summarised here: §8.3's four-gate table and §8.7's wiring
paragraph (both carve-authored at u-rev 2, *not* verbatim record, so the head's
"§8.1 through §8.7 … carried verbatim as the RECORD" disclaimer mis-describes them)
name the differential gate as *"S-E, with the reduced S-C beside it"*; **U4-T
registers S-E's two halves as this unit's tests with no caveat**; U4-M's cost row
prices the gate on S-E's traversal; and U4-M item 1 registers the snapshot *"both
under the amended script"*. §8.1–§8.7's *record* voice is adequately disclaimed at the
head; **U4-T, U4-M and the U4-Z lead-in are outside that disclaimer's stated scope.**
U4-Z items 4 and 15 *are* caveated by the lead-in — but the lead-in's own reason is
stale (MAJOR 4).

### (c) Are B1 and B2 honestly recorded as still open? **B2 yes. B1 yes, but the head contradicts it.**

**B2** is recorded three times — §9's fold, U4-Z's *"AND B2 IS NOT DISCHARGED HERE"*
paragraph, and the OPEN list — and each says the same thing: no ADR line, selection
open after two authored revisions and two red teams, third round owed and the
architect's. Round 2's own §6 independently confirms *"U4 §9's B2 is still open at
HEAD."* **Honest and complete.**

**B1** is recorded as OPEN in U4-Z with an accurate account of the stop and the four
missing rows. But the *head block* still says the matrix was *"never authored, never
attacked"* — BLOCKING 1.

---

## SCOPE 4 — MAJOR 8's repair. **PASS, with one MINOR on the residual and one MAJOR on a neighbouring row.**

**The test, run, with its own output:**

```
$ cargo test -p pistol-solver --test wp15b_mutation_witnesses -- --nocapture
     Running tests/wp15b_mutation_witnesses.rs (target/debug/deps/wp15b_mutation_witnesses-a2c3afcc6d11be2f)

running 2 tests
test the_m4_witness_is_a_position_a_legal_game_reaches_and_separates_the_two_cover_notions ... ok
test the_m6_witness_is_a_position_a_legal_game_reaches_and_holds_win_now_beside_an_unblockable_double ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**The witnesses match.** §8.4's M4 row gives P1 `(0,0)(1,0)(2,0)(3,0)`, `(-1,1)(-1,2)(-1,3)(-1,4)`,
`(0,7)` and P2 `(-2,0)(5,0)(-1,-1)(-1,6)`, `(4,-4)(5,-4)(-4,4)(-5,5)`; the test
(`wp15b_mutation_witnesses.rs:81–89`) holds exactly those coordinates in that order.
M6's fifteen P1 and sixteen P2 stones likewise match `:121–130` exactly, including the
seven fillers. Both solver answers in the prose match the test's asserted values
verbatim.

**The parity reasoning is right against pinned rule 3.** Rule 3: turn 1 is one stone,
every later turn is two by the mover ⇒ P1 odd, P2 within one of P1. M4: 9 / 8 ✓.
M6: 15 / 16 ✓. `replay()` asserts all three preconditions by name (`:43–50`).

**It really goes through the referee and does not re-implement rules.**
`common::play` (`crates/pistol-solver/tests/common/mod.rs:148–158`) drives every ply
through `GameState::place`, and `place` (`crates/pistol-core/src/state.rs:185–214`)
is where legality, region and win detection live:

```rust
if let Outcome::Win { winner, turn } = self.outcome {
    return Err(CoreError::GameDecided { winner, turn });
}
self.board.check_placement(at)?;
```

An illegal ply, an out-of-region placement or a placement after a decided game is a
`CoreError` and the test panics on it. **Rule 2 is satisfied** — the test constructs a
ply *order*, it does not re-derive geometry, legality or win detection.

**The pin is not vacuous.** The superseded M4 witness at `6feb40a:1256` is *"P1 at
(1,0)(2,0)(3,0)(4,0)(0,1)(0,2)(0,3)(0,4) sealed by P2 at (-1,0)(6,0)(0,-1)(0,6)"* —
8 P1 (even, so `p1.len() % 2 == 1` fails), first stone `(1,0)` (so `p1[0] == ORIGIN`
fails), 4 P2 against 8 P1 (so the within-one assert fails). **All three refusals
reproduce**, exactly as U4-Z claims.

**The named residual is honest** — *"a legal position is not yet a position the
mutation DIES on IN THE SEARCH"* — and `crates/pistol-search/src/staged.rs` indeed
does not exist (`ls` → *No such file or directory*). It is **not quite complete**:
MINOR 11.

**And a neighbouring row in the same table was not repaired:** M3's witness is
declared BUILT and is not a position — MAJOR 5.

**Result: PASS on MAJOR 8's own scope.**

---

## SCOPE 5 — the cross-unit finding handed to me. **VERIFIED, and it is worse than reported.**

U3's reviewer reported U4 restating the staged-config count **three** times (lines
39–40, 743, 753 at u-rev 3). Re-measured at u-rev 5 with a line-break-tolerant
scan, there are **FOUR**:

```
$ python3 -c "…" # join lines, then find \b(four|FOUR|fourth)\b with 140 chars of
                 # context matching /config|document/i
line ~40   : "corrected to the four-config reality"
line ~311  : "keep the cut, which is why there are four and not three (**U3** §10, the one place that count is stated)"
line ~798  : "which stands on §10's four configs"
line ~808  : "which needs a FOURTH config, `tactical_staged_v0.toml`"
```

(The scan's other hits — "all four" `must_block` cases at 305, "a fourth letter" at
378, the `--config` flag "fourth of its exact kind" at 493/546/593, "four revisions"
and "four gates" at 715/717/749 — are different subjects and are not restatements.)

The owner table §10 rules: *"The count is FOUR, and `docs/experiments/U3_tier_t.md`
§10 is the only place in the carve that says so."* **The finding is VERIFIED and
extended by one site — the one at line 311, which U3's reviewer did not see and which
is the worst of the four.** BLOCKING 3.

---

## THE TWO MISASSIGNED SCOPE ITEMS — absence verified

**MAJOR 12 (the unmarked `23.2` in matrix M1, U3 §6.3).** MEASURED:

```
$ grep -n "23\.2" docs/experiments/U4_soundness_instrument.md
(no output)
```

U4 carries no claim resting on it. Its only contact with M1 is §8.4's M7 row
(*"Tier T qualifies at ≥3 for the mover (option A)"*), which cites U3 §6.5 for the
disposition and does not restate a figure. **No finding.**

**MAJOR 9 (rule 5 / D-263 at §12 item 4, U3's).** MEASURED, `grep -n "D-263\|rule 5\|
hotspot\|abort threshold\|bracket"` over U4 returns exactly one line — §8.4's M6 row,
*"8 apart keeps every placement legal under rule 5"*, which is CLAUDE.md's **game**
rule 5 (the radius-8 region), not hard rule 5. U4 carries no claim resting on the
D-263 registration. **No finding, and per the brief its non-discharge is an IMPL gate
and is not reported as a design defect anywhere in this report.**

---

## MEASURED NUMBERS I RE-RAN RATHER THAN READ

Per the warning about this session's three non-reproducing MEASURED cells, I re-took
every number the u-rev 2–5 work introduced, plus every cheap carried one.

| Claim | Site | Result |
|---|---|---|
| `grep -c "^\| Option \|"` at `6feb40a` → 3 | §8 fold | **REPRODUCES** |
| `S-E` occurs 0 times at `ec8f7fb` | §8 fold | **REPRODUCES** |
| `inclusion` 0 times in the calculus; `DEF-T` = "exact **minimum hitting set**" | §8 fold | **REPRODUCES** |
| `reference.rs:223` implements inclusion-minimal covers "by the definition" | §8 fold | **REPRODUCES**, exactly at line 223 |
| `grep "(b)"` over the superseded doc → 2 hits (§4.2 matrix row; §5.2's `D-257 (a)/(b)`) | §8.7 | **REPRODUCES** |
| superseded §8.3 titled "The other three parts" | §8.7 | **REPRODUCES** (`6feb40a:1149`) |
| `tactical_v0.txt` = 15 at `instrument_v0` + 5 at `gate_v0` | §8.3 | **REPRODUCES** |
| 11 `mate_in_1` cases, 9 single-stone / 2 two-stone | §8.4 M1/M2 | **REPRODUCES** |
| `gate_v0.toml`'s table: r2 d4 "> 100 s", r2 d3 "9.7 s" | §8.1 | **REPRODUCES** verbatim |
| `PAT-GAP` plans `{-1,0 1,0} {1,0} {1,0 6,0} {6,0 7,0}` | §8.3 | **REPRODUCES** verbatim from `pattern_v0.txt:97` |
| `70.8 %` = census BATCHED nodes at corpus roots | §8.4 M8 | **REPRODUCES** (`U3_tier_t.md:117`) |
| the M4 / M6 witnesses and their solver answers | §8.4 | **REPRODUCE** — test output above |
| superseded M4 witness refused on three counts | U4-Z | **REPRODUCES** |
| six retargets, no seventh | U4-Z | **REPRODUCES** (enumeration above) |
| **`can_win_this_turn = Some(OnePly{(5,0)})` and `blocking_covers = Minimal([One((5,3))])`** on `mate_in_1_own_win_beats_blocking_the_opponent` | §8.2, the guard argument | **REPRODUCES EXACTLY** — worktree probe, output below |
| **`depth_at_500ms` 32 lines below the `# timing` marker; 97-line record** | §9.1 amd 1 | **REPRODUCES** — marker at 57, triple at 89/93/97 |
| **the pinned operator triple 2 / 2 / 1** | §9 / U4-M | **REPRODUCES** on this machine |
| **the above-marker block is byte-invariant across runs** | §9.1 amd 1 | **REPRODUCES** — `diff` of two runs' first 56 lines is empty |
| **one snapshot ≈ 34.0 / 34.5 s** | U4-M cost | **CONSISTENT** — MEASURED 33 s here |

The §8.2 probe (separate worktree, removed):

```
$ cargo test -p pistol-solver --test zz_reviewer_probe -- --nocapture
can_win_this_turn(P1,Two) = Some(OnePly { at: Coord { q: 5, r: 0 }, window: … })
blocking_covers(P1,Two)   = Minimal([One(Coord { q: 5, r: 3 })])
test probe_win_now_node_also_answers_minimal ... ok
$ git worktree remove /home/tom/u4-review-wt --force && git status --porcelain && echo "LIVE TREE CLEAN"
LIVE TREE CLEAN
```

The snapshot replication:

```
$ tools/baseline_snapshot.sh --out …/snap1.txt
baseline_snapshot: baseline_snapshot schema 1, 24 positions at nodes 50000 (registered), setup 1.63% ok
$ grep -n "depth_at_500ms" …/snap1.txt
89:timing depth_at_500ms opening 2
93:timing depth_at_500ms early_mid 2
97:timing depth_at_500ms late_mid 1
$ grep -n "^# timing" …/snap1.txt
57:# timing — machine-, schedule- and worktree-dependent; excluded from every comparison
$ diff <(head -56 …/snap1.txt) <(head -56 …/snap2.txt) && echo "ABOVE-MARKER BYTE-IDENTICAL"
ABOVE-MARKER BYTE-IDENTICAL          # second run: wall = 33 s
```

**Not re-run, with the reason:** §8.1's `cells.pop()` node counts (8794→8374 etc.),
the "28 class assertions, 0 RED", "0 of 62", "243 363 538 nodes in 554.2 s",
§8.3's "157 of 182" / "0 of 10" / "455 177 of 455 201" / "8 P2 nodes", and §8.2's
"MEASURED 17.89 s". Each needs an instrumented or mutated search run of minutes to
hours; each carries the mark the *superseded* text carried (u-rev 1 or earlier), not
a mark this session applied, and none is a cell supporting a live selection — every
selection this unit contains is stopped. **ESTIMATED cost of re-running them: several
hours.** Recorded as not re-run rather than passed over silently.

---

# FINDINGS

## BLOCKING

### 1. The head block, U4-A and §9's closing paragraph were not re-read when the u-rev 4 and u-rev 5 folds landed: the unit's own status surfaces still describe the u-rev 1 state

**Claim (U4:24–35, the first thing a reader meets):**

> - **M3 (the soundness instrument).** No matrix exists at any revision … **FRESH
>   matrix, never authored, never attacked.** The slot is stubbed at the head of §8.
> - **M4 (the snapshot's config seam).** … T1' says a diff that differs means **a
>   fresh round**.
>
> **Both fresh DECISION-RED-TEAMs are the architect's dispatch.**

**Contradicting text, in the same document:**

- §8, ~90 lines later: *"**THE MATRIX WAS AUTHORED AND ATTACKED AT u-rev 4, AND EVERY
  OPTION FELL.** `docs/experiments/matrix_M3_soundness_instrument.md` (authored
  `f8e73e4`) put nine options … to a fresh-context DECISION-RED-TEAM."*
- §9: *"**THE FRESH ROUNDS RAN, TWICE, AND STILL SELECT NOTHING (D-318).**"*
- The unit's own closing line: *"M3 attacked once, every option fell (D-317); M4
  attacked twice over two authored fields."*

**Reproducer.**

```
$ ls docs/experiments/matrix_M3_soundness_instrument.md docs/experiments/matrix_M3_REDTEAM.md \
     docs/experiments/matrix_M4_REDTEAM.md docs/experiments/matrix_M4_REDTEAM_round2.md
(all four exist)
$ sed -n '24,26p;33,34p' docs/experiments/U4_soundness_instrument.md
- **M3 (the soundness instrument).** No matrix exists at any revision, and the
  adopted option S-E occurs **zero** times at `ec8f7fb`. FRESH matrix, never
  authored, never attacked. The slot is stubbed at the head of §8.
Both fresh DECISION-RED-TEAMs are the architect's dispatch. **The carve selects
nothing.**
```

**Five further sites of the same un-re-read:**

1. **U4:37–42, the change-log sentence** — *"THE TEXT IS OTHERWISE A VERBATIM CARVE
   apart from cross-reference retargets and three named repairs"* — omits the u-rev 4
   and u-rev 5 folds entirely, which are the two largest non-verbatim additions in the
   unit.
2. **U4-A's lineage table (U4:64–71)**, whose stated purpose is *"what has attacked
   this unit's content, and at which revision"*, has **no row for any of the three
   DECISION-RED-TEAM rounds that have run since the carve** — M3 at `f8e73e4`, M4
   round 1 at `77f7397`, M4 round 2 at `cb16f7c`. It lists only the two revision-1
   rounds and the REVIEW-design history.
3. **U4:73–75** — *"What this unit owes that no round has given it: … **two fresh
   DECISION-RED-TEAMs**"*. Three have now run; what is owed is a second M3 *authoring*
   round and a third M4 round, which is what D-317 and D-318 actually say.
4. **§8's stub (U4:88–105)** — *"M3 is a FRESH matrix that has never been authored and
   has never been attacked in the form it would be selected in"*, immediately above a
   five-row table of *"to be authored"* cells, sits **above** the u-rev 4 paragraph in
   the same blockquote announcing that nine options were authored and attacked.
5. **§9:528–532** — the pre-u-rev-5 closing paragraph is left in place *beneath* the
   u-rev 5 update: *"T1' is explicit … It differs. **The fresh DECISION-RED-TEAM is the
   architect's dispatch, not the carve's**, and **until it runs** no ADR line may cite
   N-A as adopted."* It ran, twice, per the paragraph directly above it.

**Why it breaks.** This is D-305's class — *"a repair is not done until every claim
resting on the repaired thing has been re-read"* — committed at the unit's most-read
surface. A reader who stops after the head block, or who consults U4-A to learn what
has attacked this content, is told that the two red teams have not been dispatched and
that no matrix exists. Both are false at this u-rev, and the correct state is 90 to
500 lines further down. The B3 selection record shows the discipline the unit *can*
apply — *"Its 'the carve does not choose' is the state AT SELECTION TIME"* — and no
equivalent marking covers any of these six sites.

**Contrast that makes it a defect rather than a style point:** U4-Z's B3 and MAJOR 8
entries are struck through with `~~…~~` and marked CLOSED at their u-rev. The M3 and
M4 head bullets received no such treatment when their state changed.

---

### 2. U4-T registers S-E's two tests, and U4-M prices the gate on S-E, with no caveat — and the head's disclaimer does not reach them

**Claim (U4-T, "The tests this unit registers"):**

> | `staged_filtered_set_equals_the_minimal_cover_union` | **S-E, half one** … the
> public generator's forced prefix against an independently written plan-family
> referent in pistol-search's own test tree |
> | `visit_searches_every_forced_candidate` | **S-E, half two**: the always-on
> `assert!` in `visit` … |

**Claim (U4-M, Cost):**

> | The soundness gate per CI run | **ESTIMATED 40–90 s**, dominated by **S-E's one
> traversal per fixture** plus the reduced S-C's **MEASURED 17.89 s** |

**Claim (U4-M, item 1):** *"**Snapshot before / after**, both under the amended
script."* — the amended script is N-A's, and N-A is not selected.

**Contradicting text (§8, the u-rev 4 fold):** *"**They are not a selection, and after
this round they are not a recommendation either.** **No ADR line may cite S-E as
adopted**"*; and the OPEN list: *"No instrument is selected for the DIFFERENTIAL
GATE."*

**Why the head's disclaimer does not cover it.** The disclaimer is scoped by its own
words: *"**§8.1 through §8.7** below are carried verbatim as the RECORD."* U4-T, U4-M
and the cost table are outside §8. They are also *not* verbatim record in the sense
the disclaimer asserts for §8: they are this unit's own registration of what it
measures and what it tests.

**And the disclaimer mis-describes two blocks it does cover.** §8.3's four-gate table
and §8.7's wiring paragraph are **carve-authored at u-rev 2** under D-316, not
"carried verbatim" — the head says so itself three paragraphs earlier (*"apart from …
**B3**, repaired by shape 2"*). Both name the differential gate as *"S-E, with the
reduced S-C beside it"*. §8.7's is the sentence that specifies
`tools/staged_soundness_check.sh`'s enumeration for `tools/ci.sh`.

**Reproducer.** The three sites the M3 red team's F5 cites as "what the design writes
as adopted" are all still live and uncaveated:

```
$ grep -n 'reduced S-C beside it\|with a reduced S-C\|S-E with the reduced S-C' \
        docs/experiments/U4_soundness_instrument.md
158:### 8.2 THE DIFFERENTIAL GATE — S-E, with a reduced S-C beside it — **SELECTION OPEN** (see the block above)
252:| **THE DIFFERENTIAL GATE** — S-E, with the reduced S-C beside it | §8.2 |
418:(§8.2: S-E with the reduced S-C beside it), the colony family (§8.3) …
```

Only line 158 carries "SELECTION OPEN".

**Why it breaks, and why it is BLOCKING rather than MAJOR.** IMPL reads U4-T for what
to build. Both registered rows implement an instrument that a fresh-context red team
killed on grounds that are *specific to those two mechanisms*, and neither ground is
mentioned at either row:

- Half one's referent: F2 — *"a genuinely independent referent … is RED on a correct
  engine at a legally reachable FILTERED node."* The row still says "an independently
  written plan-family referent" with no note that independence and greenness are in
  tension there.
- Half two's mechanism: F4 — *"D-115 forbids S-E's primary mechanism BY NAME"*
  (widening `pistol_search::staged` to `pub` for a test). §8.2 argues D-115 *permits*
  the `assert!` half and says nothing about the widening; the U4-T row inherits that
  silence.

Neither F2 nor F4 appears anywhere outside §8's fold, and the fold does not name F4 at
all. **A stopped selection whose two implementing test rows remain registered
unqualified is the transmission failure this WP has failed on repeatedly.**

**Also under this finding:** U4-M item 1's *"both under the amended script"* — see
MAJOR 6, which adds an independent defect on top of the adopted voice.

---

### 3. B5's class survives in U4 at FOUR sites — one of them inside the clause that names U3 §10 as the only place the count may be stated

**Claim (U4:307–312, inside B4's repair):**

> The other two staged documents — the SPRT seat and the play config — keep the cut,
> **which is why there are four and not three** (**U3** §10, **the one place that count
> is stated**).

**Contradicting text (`section_owner_table.md` §10):** *"**The count is FOUR**, and
`docs/experiments/U3_tier_t.md` §10 is the only place in the carve that says so."*
And `U3_tier_t.md:253`: *"**FOUR** complete documents … **This is the one place the
count is stated.**"*

**Reproducer** (line-break tolerant, because the worst site straddles a wrap):

```
$ python3 - <<'EOF'
import re
lines=open('docs/experiments/U4_soundness_instrument.md').read().split('\n')
j=' '.join(f'@{i+1}@ {l}' for i,l in enumerate(lines))
for m in re.finditer(r'\b(four|FOUR|fourth|FOURTH)\b', j):
    ctx=j[max(0,m.start()-140):m.end()+140]
    if re.search(r'config|document', ctx, re.I):
        print(re.findall(r'@(\d+)@', j[:m.start()])[-1], re.sub(r'@\d+@ ','',ctx))
EOF
```

Four restatement sites, after discarding the hits about a different subject:

| line | text |
|---|---|
| 40 | "corrected to the **four-config** reality" |
| **311** | "keep the cut, which is why **there are four and not three** (**U3** §10, the one place that count is stated)" |
| 798 | "which stands on §10's **four** configs" |
| 808 | "which needs a **FOURTH** config, `tactical_staged_v0.toml`" |

**Why it breaks.** U3's reviewer graded the *same* defect at *one* site in U3 as
BLOCKING 1, on the owner table's ruling. Here there are four, and site 311 is a
self-contradiction within a single sentence: it states the count in the same clause
that says elsewhere is the only place the count may be stated. It is also inside B4's
own repair paragraph — **the recurring defect reproduced inside the repair that names
it, for the second consecutive revision of this sentence** (revision 7 did the same,
which is what B4 found). Sites 798 and 808 are the two U3's reviewer named; site 40 is
the third; site 311 is new and is the one no previous round has caught.

**Fix shape (not mine to apply).** Sites 40, 798 and 808 can cite U3 §10 without a
number. Site 311's clause is load-bearing only for *why* there are two tactical
configs and not one, which the preceding sentence already states by name; the "four
and not three" clause can be dropped without touching the derivation.

---

## MAJOR

### 4. U4-Z's ADR lead-in blocks item 15 on "B3's unresolved wiring", which U4-Z itself records as CLOSED ninety lines above

**Claim (U4:795–798):**

> **both items below are this unit's own, neither has landed, and BOTH are blocked on
> a selection that is OPEN** — item 4 on M3's fresh matrix, item 15 on the
> tactical-suite gate's derivation (§8.3, the superseded `(a)`), which stands on §10's
> four configs and on **B3's unresolved wiring**.

**Contradicting text, three places:**

1. U4:689 — *"### B3, gate (b) — **SETTLED. SHAPE 2 SELECTED (D-316)**."*
2. U4:827 — *"~~**B3 — gate (b)**~~ **CLOSED at u-rev 2**."*
3. U4's own OPEN list on M3: *"No instrument is selected for the DIFFERENTIAL GATE …
   and **the other three named gates are unaffected**"* — the tactical-suite gate,
   which is item 15's subject, is one of the three unaffected.

**A third, incompatible account exists in the landed ADR.** D-318: *"WHAT IS BLOCKED:
`tools/baseline_snapshot.sh` has no config seam … **U4-Z item 15 stays blocked**."*
Item 15 is about the two tactical staged configs and the tactical-suite gate; it has
no dependency on the snapshot's config seam.

**Reproducer.**

```
$ sed -n '798p;827p' docs/experiments/U4_soundness_instrument.md
…which stands on §10's four configs and on B3's unresolved wiring.
- ~~**B3 — gate (b), the two shapes above.**~~ **CLOSED at u-rev 2** …
```

**Why it breaks.** The scope of what is OPEN must be complete and honest, and here the
document gives three mutually inconsistent reasons for item 15's blockage, one of
which (B3) is measurably closed and one of which (M4's seam) is about a different
subject. On the document's own evidence item 15 may not be blocked at all. The same
sentence also calls M3's matrix *"M3's fresh matrix"*, which is the BLOCKING 1
staleness recurring.

---

### 5. §8.4's M3 row declares its witness BUILT and names no position — only an abstract shape from a doc comment — so the ledger still carries an unbuilt witness after MAJOR 8's closure says it does not

**Claim (§8.4's lead-in):** *"**Each mutation names the position it dies on, and where
the corpus cannot produce one it is BUILT** (D-260's precedent and its remedy)."*

**Claim (§8.4, M3's witness cell):** *"**BUILT**, and revision 6's witness was inert
under EQUALITY too. … The witness **must have** a phase-0 union of **three or more**
cells: `cover.rs`'s own `{a,b} {b,d} {d,e}` shape."*

**Contradicting evidence.** `{a,b} {b,d} {d,e}` is not a position. It is the abstract
window-empties example in a module doc comment:

```
$ sed -n '25,31p' crates/pistol-solver/src/cover.rs
//! # And a flat cell list is provably insufficient
//!
//! Three hot windows with empties `{a, b}`, `{b, d}`, `{d, e}` have no one-cell
//! cover and three minimal two-cell covers; `{a, e}` is drawn from the same cell
//! union and covers nothing in the middle. …
```

No coordinates, no stone counts, no parity, no legality, no pin. The cell states a
*required property* of a witness and calls it BUILT.

**Contradicting text (U4-Z's MAJOR 8 closure):** *"**Both witnesses are rebuilt** in
§8.4 as positions reached by replaying every ply through `GameState` … What is
discharged is the reachability half, which was the half MAJOR 8 raised."* A reader
takes from this that the ledger's witness problem is now confined to "does the
mutation die in the search". It is not: M3's witness has not been built at all.

**Why it breaks.** §8.4 opens by quoting D-295's finding that *"asserting an
instrument's strength rather than measuring it"* is the defect, and M3 is one of the
two mutations the ledger classes as S-E's — i.e. one of the two that would evidence
the differential gate. Recorded as MAJOR and not BLOCKING because M3 was outside
MAJOR 8's literal scope (§17 named M4 and M6 only) and because the differential gate's
instrument is stopped anyway, so nothing currently rests on it. It should be named in
the OPEN list rather than left inside a cell that says BUILT.

---

### 6. U4-M item 1 registers the snapshot under "the amended script" — an instrument that does not exist at HEAD and has never existed at any commit — while §9 in the same unit says the seam is absent

**Claim (U4-M item 1):** *"**Snapshot before / after**, **both under the amended
script**. **Registered quantity: per-position `depth_turns` and `nodes` at 50 000
nodes**."*

**Claim (§9.1 amendment 4):** *"N-A **is** a change to that instrument, so the BEFORE
run — taken under the pre-`--config` script — **is re-taken under the amended one**.
**MEASURED 34.5 s.**"* — and U4-M's cost table carries `34.0 / 34.5 s` as MEASURED.

**Contradicting text (§9's u-rev 5 fold, in the same unit):** *"`tools/baseline_snapshot.sh`
**has no config seam**, so the registered above-marker quantity has a BEFORE and no
AFTER."* And: *"No ADR line may cite N-A, N-A′ or N-E as adopted."*

**Reproducer — the amended script has never existed.**

```
$ grep -n "^CONFIG=" tools/baseline_snapshot.sh
170:CONFIG="configs/instrument_v0.toml"
$ sed -n '/^while \[ "\$#"/,/^done/p' tools/baseline_snapshot.sh
	--out) … --nodes) … --corpus) … --ladder-depth) … --ladder-cap-s) … --binary) …
	*) fail "unknown argument \`$1\`" ;;
$ git log --oneline --all -S'--config PATH' -- tools/baseline_snapshot.sh
(no output)
$ git show f317385:tools/baseline_snapshot.sh | grep -n "^CONFIG="
170:CONFIG="configs/instrument_v0.toml"
```

The only `--config` occurrences in the script are the three that pass it to the
*engine* (lines 425, 464, 531). At `f317385` — the commit U4 names as the one the
BEFORE numbers were taken at — the script is the same: a literal `CONFIG`, no flag.

**Why it breaks.** CLAUDE.md's instrument clause requires the artefact producing a
registered number to be *named with its revision*. U4-M item 1's registered
instrument is "the amended script", which has no revision because it has never
existed; and §9.1 amendment 4 attributes a MEASURED wall time to a run under it. The
unit therefore registers its one measurement against an instrument its own §9 says is
absent. (Both the amendment and the item are verbatim carries from the superseded
`6feb40a:1338` and `6feb40a:§12 item 1`, so the defect is inherited rather than
created — but the u-rev 5 fold re-read §9 and did not re-read U4-M, which is
BLOCKING 1's class again.)

For the record, everything about the snapshot that *is* checkable reproduces: I ran
`tools/baseline_snapshot.sh` twice at `35aab95` (33 s wall each), the pinned triple
came back 2 / 2 / 1, the record is 97 lines with the `# timing` marker at 57 and
`depth_at_500ms` at 89/93/97 (**32 lines below the marker**, exactly as amendment 1
says), and the above-marker block was byte-identical across the two runs. **Amendment
1 is sound; it is the AFTER half and its instrument that are not.**

---

## MINOR

### 7. The stubbed M3 matrix's column header is a live letter-address, in the one slot a future author will fill

```
$ sed -n '96p' docs/experiments/U4_soundness_instrument.md
> | Option | (b)'s instrument | Cost | Failure modes |
```

D-316 retires `(a)`–`(d)` *as an addressing scheme* and keeps them only as a lookup.
The stub table's own column head still addresses the differential gate as `(b)`. It
resolves through §8.3's table, so it is not orphaned — but it is the sentence the
second authoring round copies its frame from, and the authored matrix at `f8e73e4`
sensibly does not use it. Retitling the column *"the differential gate's instrument"*
costs nothing.

### 8. The u-rev 2 correction's diagnosis is false, and the same false diagnosis is in the landed D-316

**Claim (U4:765–767):** *"Sites 4–6 are outside §8 and inside this unit, and **the cell
counted only what was outside the unit**."* D-316 repeats it: *"the cell counted only
the sites outside the unit and missed three inside it."*

**Contradicting text — the cost cell itself (U4:722–725):** *"**three**
cross-references outside §8 retarget — `configs/tactical_staged_v0.toml`'s "why" cell
and §10's B5 paragraph, both **U3** §10, **and item 15 in this unit's list**, which
says "gate (a)"."*

The cell counted three sites, one of which (item 15 — site 3 of the correction table)
**is inside the unit**. So the cell did not count "only what was outside the unit";
it counted one inside site and missed three others. **The count SIX is correct and I
verified there is no seventh site** (enumeration under SCOPE 1). Only the explanation
is wrong — but it is the explanation in a paragraph whose subject is the accuracy of
cost cells, and it is now in an append-only ADR.

### 9. "Reinstate revision 1's four-part bar **verbatim** from `ec8f7fb:502`" is a paraphrase

U4 (twice — §8.7's defect block and U4-Z's shape 1) quotes revision 1 as *"(a) tactical
suite at pre-registered thresholds under Staged"*. The actual text is *"(a) **the**
tactical suite **at 100 % of its** pre-registered thresholds under Staged"* (see the
SCOPE 1 block for the full `sed` output). The line citation `ec8f7fb:502` is correct
and nothing substantive turns on the difference, but text presented as a verbatim
quotation should be one, in a document whose whole discipline is that a carried
quotation is unedited.

### 10. §9's fold (ii) puts D-316's residual into a list about a question D-316 is not about, and the clause is in neither red-team report

**Claim (U4:512–517):** *"the tree holds NO attacked selection **for how an instrument
binds a per-run input**, since D-252 selected nothing, D-283 states its own choices
were never attacked, and **D-316's residual says the same of B3's shape comparison**."*

D-316 is about whether the soundness gate's four parts are addressed by letters or by
names. It is not a selection about how an instrument binds a per-run input, so it does
not support the claim it is placed inside.

```
$ grep -c "D-316" docs/experiments/matrix_M4_REDTEAM.md docs/experiments/matrix_M4_REDTEAM_round2.md
docs/experiments/matrix_M4_REDTEAM.md:0
docs/experiments/matrix_M4_REDTEAM_round2.md:0
```

The clause is the author's own addition (it also appears in D-318). The other two
conjuncts are R1's and are correct, and the stop stands on them without this one.

### 11. MAJOR 8's residual attributes the whole remaining gap to "the search is not built", but M6 owes a second construction that is independent of that

**Claim (U4-Z):** *"**THE RESIDUAL IS NAMED AND IS NOT CLOSED:** a legal position is
not yet a position the mutation DIES on in the SEARCH — **that needs the search, and
the search is not built** (`crates/pistol-search/src/staged.rs` does not exist)."*

**Contradicting text (§8.4's own M6 row):** *"**The witness is driven as a NON-PV
DESCENDANT, never as a root**: the overload return is `!is_pv`-gated and ply 0 is
always a PV node, so **as a root the mutant does not fire at all and survives**."*

The pinned witness is a *position*, and a position handed to a search is a root. So
M6 additionally owes a **parent** position from which the pinned witness is reached as
a non-PV descendant at a null window — a construction that is not built, not pinned,
and not gated on `staged.rs` existing. The residual's single stated cause is
therefore incomplete for one of the two witnesses it covers. (`ls
crates/pistol-search/src/staged.rs` → *No such file or directory*; the residual's
factual claim is true, just not the whole of what is owed.)

---

## WHAT I COULD NOT BREAK — recorded so it is not re-attacked

- **Both folds are faithful.** I checked every clause of the M3 fold and every clause
  of the M4 fold against the three red-team reports and the two ADR lines. No
  overstatement, no softening, no attack downgraded, no survivor invented. The M3
  fold's "two of those four are immune" is verbatim the red team's own §6 wording, and
  it names the same four rows. The M4 fold's "not M3's stop" distinction is exactly
  what round 2's §1 and §2 support.
- **The four named gates are coherent as a set** and no part of the gate has lost a
  definition — D-316's flip clause is not triggered.
- **Every legacy `§8.3(a)`-style citation in the tree resolves** through §8.3's lookup.
  I grepped `docs/`, `tools/`, `crates/` and `configs/`.
- **The retarget count is SIX and there is no seventh site.** I reconstructed it from
  the superseded document's own letter occurrences rather than from the correction
  table.
- **B4's corrected sentence is true** against U3 §10's table and against the shipped
  fixture's own case split (15 + 5 = 20).
- **MAJOR 8's repair is genuine.** The witnesses are pinned, the pin is not vacuous,
  the replay goes through `GameState`, no rule is re-implemented, and the parity
  reasoning is right.
- **§9.1 amendment 1 is sound and I re-took it.** The marker distance, the record
  length, the byte-invariance of the above-marker block and the pinned triple all
  reproduce on this machine.
- **§8.2's guard argument reproduces exactly.** `mate_in_1_own_win_beats_blocking_the_opponent`
  really is a node where `can_win_this_turn` is `Some(OnePly{(5,0)})` while
  `blocking_covers` answers `Minimal([One((5,3))])`, so revision 6's S-E really would
  have failed a correct implementation there. That is one of the strongest cells in
  the unit and it holds.
- **The two misassigned scope items are genuinely absent.** No claim in U4 rests on
  MAJOR 12's `23.2` or on MAJOR 9's rule-5/D-263 registration.
- **B2's record is complete and honest** at all three sites that state it.
- **Size is not a finding here**, per the brief; it is recorded for the architect in
  the owner table and is not mine.

**One observation for the architect, outside this unit and therefore not a finding:**
`section_owner_table.md` §11 (u-rev 2) records U4 at **800 lines / u-rev 3**; MEASURED
at `35aab95`, `wc -l` → **855** and the unit is at **u-rev 5**. The table's `now`
column has gone stale again, for the same reason and by the same mechanism its own
u-rev 2 note describes. D-311 binds the table; the size question it hands the
architect is posed on a number that has moved twice.

---

*REVIEW-design of `docs/experiments/U4_soundness_instrument.md` u-rev 5, at pinned
revision `35aab95` (matches HEAD). Fresh context; not the author. Every finding
reproduced before reporting; every numeric claim marked MEASURED or ESTIMATED.
Verification worktree on `/home`, removed, live tree verified clean. This report is
not committed and modifies no other repository file.*
