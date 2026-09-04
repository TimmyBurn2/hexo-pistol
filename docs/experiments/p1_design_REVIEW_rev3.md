# P1 DESIGN — REVIEW-design, round 3 (remedies-only, of the diff)

**Target.** `docs/experiments/p1_design.md` (revision 3) at the named revision
`e0ddc14b5bd74a176fd8a28a0ca3b7102de40d2f` — a `git stash create` commit on
`dev` = `ffc5c10f4d16356f574e3221a023f78399d2e3bb`. Revision 2 was stash
`8a2333bbebfa48eb40c718e0115cad73d2c0810a`; round 2's report is
`p1_design_REVIEW_rev2.md` (*"VERDICT: FAIL (N1)"*, N2–N9, three observations).

**Does that revision still match the working tree's bytes?** Yes — sha256
`b39cc4f76706291e0c1e25882436711a3d15cb0335542a5c0178a06946f631f0` for
`git show e0ddc14…:docs/experiments/p1_design.md` and for the untracked file.
The stash also carries `p1_design_REVIEW_rev2.md` (byte-identical to the tree)
and `opt_arc_ledger.md`, which differs from the tree in ONE row: the mutation
row reads *"15 of 15 DEAD at `724f050`"* (run 7, with M15) in the tree and
*"14 of 14 DEAD at `ddb8a7b`"* in the stash — read as evidence of what IMPL ran
after the stash was cut, not adjudicated. **HEAD** at review time:
`ffc5c10f4d16356f574e3221a023f78399d2e3bb`. **Date:** 2026-09-04. **Round:**
remedies-only round 3 of the diff — the last the loop grant holds (D-597's
second limb: the reading list is the diff, round 2's report, and the files the
remedies touch). **Model:** `claude-fable-5-1` (D-597 records it).

**Verdict, stated first.** VERDICT: PASS. N1's remedy holds in both halves on
execution: §1.3 describes `apply` taking every window from
`pistol_core::window::windows_through_indexed(at)` with no loop of its own, and
`state.rs` at `724f050` does exactly that — `Window::new`, `checked_step`,
`continue` and `break` do not occur in the file, so round 2's X9 is no longer
expressible; I5 and I6 as revised read all five class sets per side against
the reference, and the test does; M15 is a real, non-equivalent mutant of X9's
class and dies at I1 as registered, and two X9-shaped edge-only mutants that
pass I1 and I5 die at I6 on the set comparison, on the very window round 2's
probe found. N2–N9 and the observations hold on re-derivation with my own
commands, with two remainders: N5's off-by-one survives in a second site the
diff did not touch (R2), and N6's replacement clause cites one source for a
claim that needs two (R3). The one thing execution found that reading would
not: I5's new set comparison is empty-against-empty on I5's own boards, so
its *"AND in every class set"* pins nothing there and M15's *"(and I5/I6)"* is
half false (R1). All three are MINOR — R1 is fixture-level under D-590, and
the defect class it concerns (a straddling window in the wrong set) has a
non-vacuous killer in I1 at the one boundary the playouts cross.

**Environment.** Everything adjudicated was run with `git grep` or `git show`
pinned to `ffc5c10`, `ddb8a7b` or `724f050`, `/usr/bin/grep`, `sed`, `diff`,
or arithmetic. Executions ran in `/home/tom/pistol-wt/review-p1-design-r3`
(`git worktree add --detach 724f050`, the head of `p1/impl`; own
`CARGO_TARGET_DIR` `/home/tom/pistol-wt/review-p1-design-r3-target`, created
fresh at 01:49 UTC), never in `p1-measure` or `p1-mutants` and never in the
live tree. `pgrep -af 'cargo|rustc|bench_delta|determinism'` was empty at
01:47:37 UTC; builds and test runs occupied 01:49–01:54 UTC, and per D-592 a
timing receipt taken beside them is void. The worktree and its target
directory were removed at the end with no branch left; `git status` in the
worktree was empty before removal and the live tree's tracked files are
untouched. The mutant run's transcript is kept as a receipt at
`/home/tom/pistol-wt/review-p1-design-r3-mutants.log` (outside the tree). This
report is the only file written in the live tree.

---

## 1. Round-2 remedies, each executed or re-derived (D-591)

| finding | remedy in the diff | how I executed / re-derived it | holds? |
|---|---|---|---|
| **N1** (MAJOR) §1.3 copied the enumeration; I6 never read the sets; no mutant for the class | §1.3: the per-axis loop keeps *"each axis's before-run and after-run"*; then *"for every `(window, index)` that `pistol_core::window::windows_through_indexed(at)` yields — THE enumeration, called, not copied (N1) … shifting them by `5 - index`"*; the *"iff"* sentence and `checked_step` are gone. I5/I6: *"in the store AND in every class set"*, *"all five class sets per side equal the reference's"*. §4: M15 registered, dying at I1 | **Design half.** `/usr/bin/grep -n -w 'iff'` and `-n 'checked_step'` on the design → no hit; §1.3:112–118 read as quoted. **Code half.** `git show 724f050:…state.rs \| grep -n -E 'Window::new\|checked_step\|\bcontinue\b\|\bbreak\b\|windows_through'` → `:3` (the `use`), `:122` (`for (window, index) in windows_through_indexed(at)`) and nothing else — X9's `continue`→`break` has no site to land on. `git diff ddb8a7b 724f050 -- crates` → `state.rs`, `lib.rs`, `threat_oracle_tests.rs` only; the runs are captured per axis into `runs[axis_slot(axis)]` and read back by `window.axis`, `shift = REACH - index` with `REACH = WINDOW_LEN - 1 = 5`, `touched` counts yielded windows. **Tests.** `assert_matches_board` at `724f050:371–414` compares `hot`, `win_in_one_ply`, `completed`, `live` at Two and Three per side against `Reference::from_board` before the per-window and snapshot checks; I5 (`fn` at `:418`) calls it after every stone (`:440`) and every undo (`:445`), I6 (`fn` at `:458`) once over the 7×7 corner-and-edge grid (`:489`). **Mutants, §3:** X0 `11 passed`; M15 in both spellings dies at I1 (`threat_oracle_tests.rs:133`, the live-count set) and I6 (`:390`); X9a/X9b (edge-only skips) pass I1 and I5 and die at I6 `:390` — *"left: ConstS@-32768,32767 / right: ConstS@-32768,32767 ConstS@32762,-32763"*, round 2's window | **yes** — both halves, executed. M15's parenthetical *"(and I5/I6)"* is half false: I5 does not kill it (R1) |
| N2 M13 in two classes | §1.1:74 *"§4's M14 is the mutant for that class (N2)"* | `/usr/bin/grep -n 'M13' docs/experiments/p1_design.md` → `248` only | yes |
| N3 *"(§1.4)"* for the snapshot | §1.1:85 *"the only enumeration is the snapshot (§1.6)"* | `/usr/bin/grep -n '§1.4'` → `37`, `78` (both `undo`/`reset_to`, correctly); §1.6 is the snapshot section (`:162`) | yes |
| N4 a fifth root-doc passage | §2:187 *"five passages"*, the fifth named as *"`pub use table::unpack` here at the root"* | `git show ffc5c10:…lib.rs \| grep -n -i -E 'table\|per-window'` → 10, 39, 45, 47, 72, 76, 86, 93, 95, 120, 135; of these the old-store passages are 10, 45–47 (one paragraph), 76, 86, 93–95 (one paragraph) = five; 39/72/120/135 are live names. `git grep -n -E 'WindowTable\|table::unpack\|table::pack\|window TABLE\|the table type\|per-window record' 724f050 -- crates/pistol-solver/src crates/pistol-search/src` → none; `724f050:lib.rs:89` reads *"`pub use line::unpack` here at the root"* (`ddb8a7b:89` still said `table::unpack`) | yes |
| N5 *"the two logs"* | §1.5:155–157 *"exclude the three stacks `apply` pushes onto — the window log, the chunk log and the frames (N5)"* | `state.rs` at `724f050`: fields `log`, `chunks`, `applied` (`:48`, `:50`, `:52`); `PartialEq` over `table` and `sets` (`:76–79`). **But** `/usr/bin/grep -n 'two logs'` → `226` (I7's row, *"under a derived `PartialEq` the two logs differ"*) — the twin the diff did not touch | yes at §1.5; the second site remains (R2) |
| N6 *"the negative pairs"* | I5:224 *"−1 / 0 (the one every game crosses from the origin, and the only one the random playouts reach: the matrix's round-1 red team, m5)"* | `sed -n 183,191p …REDTEAM.md`: m5 says the playouts *"exercise −1/0 constantly and 63/64 never"* and *"the negative pair is the one with a live falsifier today"* — nothing about −64 / −65's reach. That the playouts never reach −64 / −65 is round 1's experiment 3 (`p1_design_REVIEW.md:345`: *"straddling window instances: 63/64 0, -65/-64 0, -1/0 6041"*). The claim is true and measured; the citation covers two of its three parts | yes as a claim; the citation is short by one source (R3) |
| N7 M14's mechanism | §4:249 *"I6, by `THREAT_DESYNC`: line `-32768` aliases line `0` on the lattice-edge grid, so the aliased line already holds a bit at the fourth stone's position and `apply` refuses it (N7 — the refusal kills it, not a misread window)"* | arithmetic on `724f050:line.rs:83` (bias `1 << 17`): `(−32768 + 131072) & 0xFF = 0 = (0 + 131072) & 0xFF`; the grid's fourth stone is `(MIN, 0)`, P2 (`(0+3) % 2`), whose ConstR line 0 at position −32768 aliases stone 1's ConstR line −32768 at position −32768. **Executed** (§3, M14): I6 FAILED at `state.rs:108` *"THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one"*; I1, I5, I8 also die at `line.rs:66` (the inverse's panic) under IMPL's spelling, as round 2 said | yes |
| N8 *"committed as a receipt"* | §4:253 *"sha-anchored beside its receipt (`artifacts/p1_mutation_driver.py`; `artifacts/` is never committed, rule 8)"* | `/usr/bin/grep -c 'committed as a receipt'` → 0; `git show ffc5c10:.gitignore \| grep -n artifacts` → `:19 /artifacts/` | yes |
| N9 Scope's *"`WindowMasks` type"* unchanged | Scope:41–43 *"PUBLIC surface — two fields, five accessors — is unchanged; its two `pub(crate)` helpers … (`is_vacant`, `with`) go with it (N9)"* | `git diff ffc5c10 724f050 -- crates/pistol-solver/src/table.rs` removes exactly `is_vacant` and `with` (plus imports and the old store); `git show 724f050:…table.rs \| grep -n -E 'pub(\(crate\))? (fn\|struct)\|^\s+pub [a-z0-9_]+:'` → fields `p1`, `p2`; `own`, `opp`, `own_count`, `opp_count`, `empties` | yes |
| obs. M9 *"after the flip"* | §4:244 *"M9 chunk log records the record AFTER the stone"* | `/usr/bin/grep -c 'after the flip'` → 0 | yes |
| obs. M13's loose universal | §4:248 *"(ConstS windows with a stone behind them on the line are misread on the playouts)"* | under `pos: r` the six positions read from a start `(q, r)` are the cells `(q−k, r+k)`, k = 0..5 — the cells BEHIND the start along the axis — so a window with a stone in those cells reads a stone it does not hold; the sentence is a true sufficient condition (a window holding a stone at index ≥ 1 is also misread, which the sentence does not claim) | yes |
| obs. the I5 comment | branch only: `724f050:threat_oracle_tests.rs:422–424` *"on the stones' own axis every window through the middle stones straddles the boundary and the outer windows reach across it from one side"* | a window `s..s+5` straddles `b / b+1` iff `s ∈ [b−4, b]`; a stone at `p` has own-axis starts `[p−5, p]`. Executed (python, §3): for −1 / 0 the stones at −3..2 have 3, 4, 5, 5, 4, 3 straddling windows of six; the innermost pair's non-straddling windows start at −6 and 0. Same shape at the other two boundaries. No stone has all six | **no** — still false for one window of each innermost stone; harmless, IMPL's (§5) |

---

## 2. New findings

### BLOCKING

None.

### MAJOR

None.

### MINOR

#### R1 — I5's new set comparison is empty-against-empty on I5's own boards, and M15's *"(and I5/I6)"* is half false

**Claim attacked.** §3 I5: *"Windows crossing a chunk boundary read correctly,
in the store AND in every class set … all five class sets per side equal the
reference's"*; §4 M15: *"dies at I1 (and I5/I6, which now read the sets)"*.

**Reproducer, executed.** Both spellings of M15 (§3) pass
`windows_straddling_a_chunk_boundary_read_as_the_reference_does` while dying
at I1 and I6. Why: I5's stones alternate sides at consecutive positions along
ONE axis, so every own-axis window holding two or more stones holds both
sides, and every other-axis window holds one — and `ClassSet::of` (`sets.rs:
101–122`) puts a window in no class when `opp != 0` or `own < 2`. A throwaway
probe over I5's exact loop (three axes × three boundaries, after every apply
and every undo, 108 states): *"max class-set members across all ten sets =
0"*. The same probe on I6's grid: 2 members (both P1 `LiveTwo`, the two
windows every edge mutant's failure names). So I5's set half compares two
empty lists 108 times and cannot see any set defect, at any boundary; the pin
the row now advertises for the sets is I1's at −1 / 0 (6 041 straddling
instances with same-side stones, round 1's experiment 3), and nothing pins the
sets at −64 / −65 or 63 / 64 — where the straddle arithmetic is the same
`rem_euclid` as at −1 / 0, so no distinct defect class is left without a
killer.

**Why MINOR.** Which fixture populates which set is fixture-level (D-590), the
invariant is real, and the class M15 stands for dies at I1 and I6. Fix, one of:
delete *"AND in every class set"* and *"all five class sets per side equal the
reference's"* from I5 (the test may keep the comparison; the design should not
claim it) and drop *"I5/"* from M15's parenthetical; or have I5's fixture put
two same-side stones inside one window across each boundary (e.g. sides by
`pos.div_euclid(2) % 2`), which makes the phrase true and lets I5 kill M15 —
then the ledger's *"15 of 15"* is re-run at the revision carrying it (D-591).

#### R2 — N5's off-by-one survives in §3 I7

§3 I7:226: *"under a derived `PartialEq` the two logs differ and it FAILS"*.
`/usr/bin/grep -n 'two logs' docs/experiments/p1_design.md` → `226` — the
diff corrected §1.5 (three stacks: `log`, `chunks`, `applied` at
`724f050:state.rs:48`, `:50`, `:52`) and left this twin. D-423's shape: a claim made
twice, one copy fixed. Fix: *"the three stacks differ"* or *"the logs differ"*.

#### R3 — I5's *"the only one the random playouts reach"* cites m5 for a fact m5 does not state

§3 I5:224 attributes to *"the matrix's round-1 red team, m5"* that −1 / 0 is
*"the only one the random playouts reach"*. m5 (`REDTEAM.md:183–191`) says the
playouts *"exercise −1/0 constantly and 63/64 never"* and does not mention
whether −64 / −65 is reached; that it is not (0 instances in twelve playouts)
is round 1's experiment 3 (`p1_design_REVIEW.md:345`). The clause is true and
measured; its one citation covers two of its three parts. Fix: add *"and
round 1's experiment 3"* beside m5.

**Observations, not rated.** (a) The header's *"N2–N9 are one-line
corrections"* — N4's and N9's remedies span several lines; the phrase
constrains nothing. (b) A release build of `pistol-solver` at `724f050` warns
*"unused import: `generate_turns`"* at `policy.rs:1` because its only use is
under `#[cfg(debug_assertions)]` (`policy.rs:262–266`); `policy.rs` is
byte-identical between `ffc5c10` and `724f050` (`git diff --stat` empty), so
this is `dev`'s and not the remedy's — out of this round's scope, for
REVIEW-impl or the operator. `cargo fmt --check -p pistol-solver` is clean at
`724f050` and `cargo clippy --release --all-targets -- -D clippy::all` reports
only that warning. (c) Under M15 the tests other than I1 and I6 die by
`THREAT_DESYNC` at `sets.rs:194` (*"is not in LiveTwo to remove"*) — the sets'
own refusal, rule 3 doing its work — while I1 dies by a wrong answer at the
set comparison, which is the death the row registers.

---

## 3. Executions — receipts

All in the review worktree at `724f050`, `cargo test --release --locked -p
pistol-solver --test threat_oracle_tests`, whole suite each time; the mutation
applied by a one-anchor replacement asserting exactly one occurrence, `git
diff --stat` showing `1 file changed, 1 insertion(+), 1 deletion(-)`, and the
tree restored by `git checkout -- crates` with `git status --porcelain --
crates` empty after each (`0 changed` in the log).

| # | what | result |
|---|---|---|
| X0 | unmutated | `ok. 11 passed; 0 failed` |
| M15a | the registered class, window-skip spelling: `windows_through_indexed(at).filter(\|&(_, index)\| index != 5)` | **FAILED 6**: I1 at `threat_oracle_tests.rs:133` (*"live at"*: left `ConstS@-2,2 ConstS@-1,1 ConstS@0,0`, right adds `ConstS@-3,3`); I6 at `:390` (left `ConstS@-32768,32767`, right adds `ConstS@32762,-32763`); I2, I7, I8 and the legality test by `THREAT_DESYNC … is not in LiveTwo to remove` (`sets.rs:194`). **I5 passed.** The four `should_panic` tests passed |
| M15b | the registered wording literally: `if was[s] != now[s] && index != 5` (log still pushed) | identical failure set and lines to M15a; **I5 passed** |
| X9a | X9-shaped, high edge: `.filter(\|&(window, _)\| window.start.checked_step(window.axis, 6).is_some())` — every window ending at the last addressable cell skipped | **FAILED 1**: I6 at `:390`, left `ConstS@-32768,32767`, right `ConstS@-32768,32767 ConstS@32762,-32763` — round 2's probe window. I1, I5, I8 passed (`10 passed; 1 failed`) |
| X9b | X9-shaped, low edge: `… checked_step(window.axis, -1).is_some()` — every window starting at the first addressable cell skipped | **FAILED 1**: I6 at `:390`, left `ConstS@32762,-32763`, right both. `10 passed; 1 failed` |
| M14 | IMPL's spelling: `pack`'s biased line `& 0xFF`, `unpack` unchanged | **FAILED 4**: I6 at `state.rs:108` *"THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one"*; I1, I5, I8 at `line.rs:66` (the inverse's `try_from` panic). I2, I7, legality passed |
| probe | throwaway `tests/r3_probe.rs`: I5's loop with all ten set sizes summed after every apply/undo; I6's grid once | I5: *"108 states checked, max class-set members across all ten sets = 0"*; I6: *"2"*. File removed, tree clean |
| python | own-axis straddling windows per I5 stone, `s ∈ [b−4, b]` | 3, 4, 5, 5, 4, 3 of six at each of the three boundaries; non-straddling starts for the innermost pair: `[b−5]` and `[b+1]` |
| fmt / clippy | `cargo fmt --check -p pistol-solver`; `cargo clippy --release --locked -p pistol-solver --all-targets -- -D clippy::all` | fmt rc 0; clippy: the pre-existing `generate_turns` warning only (observation b) |

---

## 4. Re-derivation table — my commands, none the document's

| claim (§) | my command, with scope | my result | document's | agree? |
|---|---|---|---|---|
| stash = tree (header) | `git show e0ddc14…:docs/experiments/{p1_design,p1_design_REVIEW_rev2,opt_arc_ledger}.md` vs the tree, `sha256sum` / `diff` | design SAME (`b39cc4f7…`); rev2 report SAME; ledger differs in the mutation row only | (implicit) | yes |
| `apply` calls the enumeration (§1.3) | `git show 724f050:…state.rs \| grep -n -E 'Window::new\|checked_step\|continue\|break\|windows_through'` | `:3`, `:122` only | *"called, not copied"* | yes |
| shift `5 − index` (§1.3) | `state.rs:56` `REACH = WINDOW_LEN - 1`; `:124` `shift = REACH - u32::from(index)`; `window.rs:6` `WINDOW_LEN = WIN_LEN` | 5 − index | same | yes |
| three chunk entries, one window entry per yielded window (§1.3) | `state.rs:114` (one push per `Axis::ALL`), `:141–142` (`touched += 1` per yielded window) | same | same | yes |
| I5/I6 read five sets per side (§3) | `git show 724f050:…threat_oracle_tests.rs \| sed -n 371,414p`; call sites `:440`, `:445`, `:489` | hot, win1, completed, live Two/Three, both sides, then per-window and snapshot | *"all five class sets per side"* | yes — but empty on I5's boards (R1) |
| three stacks excluded from equality (§1.5) | `state.rs:48`, `:50`, `:52`; `:76–79` | `log`, `chunks`, `applied`; `eq` over `table`, `sets` | same | yes; I7 still says two (R2) |
| M13 sites (§1.1, §4) | `/usr/bin/grep -n 'M13' docs/experiments/p1_design.md` | `248` | one site | yes |
| snapshot section (§1.1) | `/usr/bin/grep -n '§1.4\|§1.6'` and the section headers | `§1.6` at `:85`; `§1.4` at `:37`, `:78` are `undo` | same | yes |
| five root-doc passages (§2) | `git show ffc5c10:…lib.rs \| grep -n -i -E 'table\|per-window'`, read by content | five old-store passages; at `724f050` none remain in either crate's `src` | five | yes |
| `pub use line::unpack` (§2) | `git show {ddb8a7b,724f050}:…lib.rs \| grep -n unpack` | `ddb8a7b:89 table::unpack`; `724f050:89 line::unpack`; the doctest `:79` `line::unpack` at both | same | yes |
| m5's words (§3 I5) | `sed -n 183,191p …REDTEAM.md` | *"exercise −1/0 constantly and 63/64 never"*; *"the negative pair is the one"* | *"the only one the random playouts reach"* | claim yes; citation short (R3) |
| M14's fourth stone (§4) | `edge = [MIN, MIN+1, MIN+7, 0, …]`, `(i+j) % 2`; `line.rs:83` bias `1 << 17` | `(MIN, 0)`, P2; lines −32768 and 0 both `& 0xFF = 0`; executed, the refusal names it | same | yes |
| M15 dies at I1 (and I5/I6) (§4) | M15a, M15b | I1 and I6 yes; I5 no | *"I1 (and I5/I6 …)"* | I1, I6 yes; **I5 no — R1** |
| `is_vacant`, `with` removed; five accessors (Scope) | `git diff ffc5c10 724f050 -- …table.rs`; `grep 'pub … fn'` at `724f050` | exactly those two removed; `own`, `opp`, `own_count`, `opp_count`, `empties`; fields `p1`, `p2` | same | yes |
| `artifacts/` uncommitted (§4) | `git show ffc5c10:.gitignore \| grep -n artifacts` | `:19 /artifacts/` | *"never committed, rule 8"* | yes |
| I5's comment (branch) | arithmetic, executed in python | 5 of 6 for the innermost stones | *"every window through the middle stones straddles"* | **no** (§5) |
| `pack`'s doc widths (§1.1, carried) | `line.rs:76`, `:85`, `:102–103` | *"24 bits of line above 16 of chunk"*; `axis << 40 \| line << 16 \| chunk`, `unpack` masks `0xFF_FFFF` and `0xFFFF` | same | yes |

---

## 5. "IMPL verifies" rows (D-590), carried from round 2 §5 and updated

| row | status at `724f050` |
|---|---|
| I4's new test name | still open: the design says `taking_back_out_of_order_is_a_desync`; the branch has `taking_back_a_stone_that_is_not_the_last_applied_is_a_desync` (`threat_oracle_tests.rs:349`). The body applies the origin then `(1,0)` and undoes the origin, as I4 requires. One of the two changes |
| I5 | the comment at `:422–424` was rewritten and is still false for one own-axis window of each innermost stone (§1, last row; §3 python) — harmless, but a comment that is false. **New:** the set comparison it now runs is empty-against-empty on its boards (R1). Verify M2 dies at I5 alone (the receipt says it does; the store half is what kills it) |
| I6 | **discharged**: the sets are compared against the reference at the edge; X9-shaped mutants at both edges and M15 die there at `:390`; the grid populates two `LiveTwo` members and both appear in the failure |
| I8 | discharged in round 2 (`:493–504`, X1 at `:500`); the file is unchanged there |
| M8 | unchanged: the row names two shapes; the receipt's wording should match the mutant applied |
| M10, M12 | discharged in round 2 (X1, X8); untouched by the diff |
| M14 | **executed here at `724f050`**: dies at I6 by the refusal the row now names, and at I1/I5/I8 by the inverse's panic under IMPL's spelling — the receipt must count the I6 kill as the class's and name the other three as the inverse's |
| M15 | **executed here**: dies at I1 (`:133`) and I6 (`:390`) in both spellings; not at I5 (R1). The tree's ledger row says *"15 of 15 DEAD at `724f050`"* — verify the receipt names I1 as the death and does not credit I5 |
| `reset_to`'s comment | present at `724f050:position.rs:52–53` (*"REPLACED rather than unwound, and that is the only / correct spelling"*) |
| the docs of m5, plus N4 | **discharged**: `lib.rs:89` retargeted; no old-store name remains in `pistol-solver/src` or `pistol-search/src` at `724f050` (§4) |
| `pack`'s doc | verified: 24 bits of line above 16 of chunk match `line << 16`, `0xFF_FFFF`, `0xFFFF` |
| the ADR line | at the landing, as before |
| `policy.rs:1` release warning | new, outside scope (observation b): pre-existing on `dev`, a release-only `unused_imports` for a `cfg(debug_assertions)` use |

---

## 6. What this round did not review

Per D-597 this round read the diff, round 2's report, and the files the
remedies touch. Nothing in §1.1, §1.2, §1.4, §1.6, §5 or §6 outside the diff's
hunks was re-derived; round 1's premise attacks and round 2's X1–X8 were not
re-run except M14, which the N7 remedy restates and which cost one build. The
bench pre-registration, the matrix and its selection were not opened.

---

VERDICT: PASS
