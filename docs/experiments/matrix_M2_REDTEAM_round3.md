# DECISION-RED-TEAM — MATRIX M2, ROUND 3

## HEADER

**Artefact attacked:** `docs/experiments/matrix_M2.md`, **REVISION 3**, at commit
**`bb2fee2`** (`bb2fee2c6a69a75aa1bf0f98789090490ff2dd25`).

**Does it match HEAD?** **YES.** `git rev-parse HEAD` → `bb2fee2c6a69a75aa1bf0f98789090490ff2dd25`;
`git status --short` → empty. The revision attacked is HEAD and the tree is clean. **The live
tree was never edited by this review.**

**What this round is.** Revision 1 fell (`c075bcc`), revision 2 fell (`27b1c3d`) on the fact that
a turn is TWO stones. Revision 3's selection is **not the matrix's own recommendation** — it is
the operator ruling **D-478**, and revision 3 is written as the field that ruling is checked
against. This report attacks (1) the completeness and turn-coherence of the field, (2) the
honesty and completeness of §4's pricing, (3) whether D-478's own stated ground survives, (4)
the carried-forward facts F1–F12, and (5) §5's calibration premise.

**Read in full:** `docs/experiments/matrix_M2.md` (515 lines); `CLAUDE.md`; `docs/process.md`;
`docs/experiments/WPQ_seed.md` §7.1, §7.2 and the M2 debt note; `docs/decisions.md` D-9, D-22,
D-74, D-95, D-124, D-291, D-310, D-315, D-318, D-374, D-388, D-423, D-424, D-473–D-478;
`sessions/WP-1.5d/2026-08-29-WP15D-STOPPED.md`; `sessions/WP-1.9/wp19_design_REVIEW_rev2.md`
BLOCKING N1; `docs/experiments/matrix_M2_REDTEAM.md` and `_round2.md` (verdicts and strongest
attacks). **Code read at HEAD:** `crates/pistol-search/src/{pvs,search,staged,candidates,
position}.rs`, `crates/pistol-search/src/tt/mod.rs`, `crates/pistol-engine/src/validate.rs`,
`crates/pistol-core/src/{state,rules,turn}.rs`.

**Digests — ALL SIX VERIFY BYTE-EXACT**, `sha256sum` at `bb2fee2`:

```
db8a8793d6a2b4a5f2635c60139b3577a68c6a872331fc89acfa43b2fb327be5  artifacts/wp15d_m2_evidence_v1.txt
081c928a0900ae9332a2e1f2b3fe012732a2b0e56b97c4de2064e06fd34add76  artifacts/wp15d_m2_evidence_instrument_v1.txt
455aef9e235785986290a0ce43c5fe6cb835532e5ede0923027c169a0d0c3b7f  artifacts/wp15d_m2_evidence_v2.txt
f73608dd3693762e02968e6ec9a4c8078ac109fcc44bf3580ab8b1fc437d632c  artifacts/wp15d_m2_evidence_instrument_v2.txt
5a64034e3ab178beecded86359692a2076a66ae89d3e4961c83b1ac4d082a3ae  artifacts/wp15d_turn_axis_v1.txt
43fa71ce9cc6e99cf69aee40493096c4a3d30301ed97cd1c88794df5c3026c10  artifacts/wp15d_turn_axis_book_v1.txt
```

**What I re-ran.** A detached worktree of my own at `bb2fee2`,
`/home/tom/Projects/pistol-wt-rt3`, `CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt3-target`,
on `/home` and never `/tmp`. It carries revision 3's committed instrument (copied read-only out
of `/home/tom/Projects/pistol-wt-r3`, scopes 0–5, the ply counters, the membership probe) plus
**four additions of my own**: a `(turns_from_root, phase)`-resolved cap counter, a
truncated-store POISON tracker, and four test drivers. Instrument digests, so the runs below are
reproducible:

```
27c4a00fbaaf53453cf6499c4a04e03e6ca05a8c289569b13e9665bbf3defbaa  git diff (tracked files)
c838c874412b6c5dd9a58a0371a38e64fa6c5c8ac0531acdce0b220a0623f5bb  crates/pistol-search/src/scratch.rs
ff07def6104e584715337887c3c7336d3854013de3cc60cd41d435b2a54dc019  crates/pistol-search/tests/zz_rt3_axis.rs
c6657e849d188838099c7214c7d155ee93d2dd9f16ee83844099ec8e9a3c5370  crates/pistol-search/tests/zz_rt3_tt.rs
c188933e7fa750742eebb80fd38119dead35f6b2cf9d8214e4c07c94fbcb406b  crates/pistol-search/tests/zz_rt3_turn1.rs
661f3de4d8b40ae001d4933e82ae3eed21ee1ffa559e4ca4298f23ded2c4ae33  crates/pistol-search/tests/zz_rt3_book.rs
```

**The worktree is LEFT IN PLACE, not removed.** CLAUDE.md forbids removing a worktree before its
contents are exported with a digest receipt, and this review is permitted to write one file
only. The operator should export or discard it deliberately.

**A concurrent artefact, noted not attacked.** An untracked
`docs/experiments/wp15d_design.md` (revision 1, mtime `Aug 29 15:30`) appeared in the live tree
during this review. It is not this review's artefact, this review did not author or modify it,
and it is not part of the verdict below — but it implements the selected row and it restates
BLOCKING 1's false step explicitly, so it is quoted once inside BLOCKING 1.

**Runs taken (all `Stop::Nodes(50_000)`, `quiet_radius 2`, ordering heuristics off unless
stated):** the whole 2 000-opening SPRT book at five seats including the two the matrix never
receipted; 40 self-play games with the transposition table KEPT, at four seats; and two
rules-level walks of the ply↔turn correspondence. Five book seats ran concurrently on a
16-core machine at load ≈ 6, so **wall-time ratios below are same-run and same-load, and
absolute milliseconds are not comparable with the committed receipts' absolutes.**

---

## VERDICT

# **FALLS.**

Not on the operator's principle — **D-478's principle is right and survives this round intact**:
a cap must not reach the node where the move is chosen. It falls because **the row the matrix
offers as that principle's implementation does not have the property, and the matrix's own §5
asserts twice that it does.**

Two findings are independently sufficient.

**BLOCKING 1 is a CORRECTNESS finding and is NOT overrulable under D-424.** `T-BELOW`'s
defining ground — *"It prunes nothing in the move the engine returns"* — is true only inside a
single search. The engine keeps the transposition table across the searches of one game
(`crates/pistol-search/src/search.rs:53`), and across searches ply is **not** a function of the
key: the node this row truncates at ply ≥ 2 in search *N* **is** the root and root-child of
search *N+1*. MEASURED, 40 games, one searcher per game: **45 667 records written by a truncated
node, 14 365 cutoffs taken on them, 41 of those at ply 0 or ply 1 — inside the played turn — and
all 41 on a `Bound::Exact` or `Bound::Upper` that the storing node had not proved**, plus 202
played-turn table-move promotions seeded from a poisoned record. The null control (no cap) is
**0 / 0 / 0 / 0**. `T-ROOT`, the row D-478 defers, is **0 poisoned cutoffs and 0 at ply 0/1** —
its truncated nodes hold fewer stones than any later root and can never be revisited. **So §4's
fifth cell, the single cell the whole selection rests on, is inverted by measurement in the
configuration the engine actually runs in.** D-424 is explicit that a finding naming a way the
code can produce a wrong answer *"is never overruled, only fixed"*. **It is fixable**, and the
fix is already the adopted text of the document the matrix cites for the defect class:
`WPQ_seed.md` §7.2 — *"a fail-low or exact score from a set that was not exhausted is unsound in
the bound it claims, so it stores nothing."* The matrix never mentions that rule, and §5's
"THREE THINGS THIS DOES NOT SETTLE" does not list it.

**BLOCKING 2: the field is incomplete for the third round running, on its own new axis.** Rule 3
gives turn 1 ONE stone (`crates/pistol-core/src/rules.rs:30-36`), and `check_root` accepts a
turn-1 root (`search.rs:468-474` refuses only `phase() != First`). At a turn-1 root §1's axis
sentence — *"Ply 0 is the played turn's first stone, ply 1 its second, and everything from ply 2
down is the opponent's reply and beyond"* — is **false**: ply 1 is the **opponent's first
stone**. MEASURED at a turn-1 root, capped/seen by `(turns_from_root, phase)`: `T-BELOW` caps
**57 of 57** phase-1 nodes of the opponent's turn and **0 of 4** phase-0 nodes. **It prunes one
stone of a two-stone turn and not the other** — verbatim the defect for which §1 STRIKES `W-K2`
and `W-K0` as *"not a choice anyone would make on purpose"*. `T-ROOT` is incoherent at the same
root in the mirror direction (it caps 3 of 3 opponent phase-0 nodes and 0 of 20 phase-1, so it
prunes **below** the played turn, contradicting its own §1 definition).

**The row that should be selected, and it is not in the field:** the same scope expressed on the
**turn counter the engine already maintains** — `self.turns_from_root() > 0`
(`crates/pistol-search/src/pvs.rs:523`, *"How many turns this position is from the root. Both
plies of a turn share it"*) — **plus** `WPQ_seed.md` §7.2's store-truncation rule. Call it
**T-BELOW-T**. It is identical to `T-BELOW` at every turn-≥2 root, turn-coherent at the turn-1
root where `T-BELOW` is not, and with the store rule it is the only row in or out of this field
that actually has the property D-478 selected `T-BELOW` for. **It strictly dominates the
selected row and was never authored.** D-477 is the reason it was missed and says so in advance:
*"where the axis is a unit … the quotation names the line where that unit is CONSUMED"* — the
matrix quotes `pvs.rs:397-399` and `search.rs:328-329` and never quotes `rules.rs:30`, which is
where the ply↔turn correspondence is actually established and where it breaks.

**BLOCKING 2 is an axis/completeness finding of the same class D-476 and D-318 both stopped a
matrix for, and an operator ruling MAY reach it** — it is a judgement about which field is the
right field. **BLOCKING 1 may not.**

---

## FINDINGS

### BLOCKING 1 — `T-BELOW` reaches the played turn through the transposition table, on a bound the truncated node did not prove. §5 grounds 1 and 5 are both false in the configuration the engine runs in. NOT OVERRULABLE.

**Claims attacked, verbatim:**

> "1. **It prunes nothing in the move the engine returns** — MEASURED, not defined:
> `ply0 = 0` and `ply1 = 0` across 1 593 643 capped rows on the book and all four spread
> positions. `WPQ_seed.md` §7.1's rule is satisfied exactly, and D-124 is not engaged."
> — `matrix_M2.md` §5, ground 1

> "5. **It is sound in the transposition table**, measured at this scope: 0 membership
> disagreements, against W-K3's 821."
> — `matrix_M2.md` §5, ground 5

> "A capped node stores a `Record` (`pvs.rs:449-467`) ranging over the capped set, and the probe
> (`pvs.rs:245-256`) takes a cutoff on `!is_pv` alone; the bound is sound for a later visitor
> **iff the same key always emits the same set**. … The reason is structural and worth stating:
> the stone count is a function of the key and only increases down the tree, so ply 0's key can
> never recur below it, and a scope that is a function of ply alone therefore emits one set per
> key."
> — `matrix_M2.md` F11

**THE ATTACK.** F11's "iff" is the right criterion and its structural argument is **correct
within one search** — I verified it and it holds. But it is **silently scoped to one search**,
and the engine is not built that way:

```
crates/pistol-search/src/search.rs:53
/// The table is kept: successive searches in one game share what they learned,
```

`Searcher::clear()` is documented at `search.rs:192` as *"what a new game does"* — not what a
new search does. `Table::probe` (`tt/mod.rs:129-137`) validates nothing about age or generation;
`new_generation()` (`search.rs:218`) only changes replacement priority. So entries written in
search *N* are live and probeable in search *N+1*.

**Across searches, ply is not a function of the key.** Ply *is* determined by stone count within
one search (each ply places one stone), which is what makes F11's argument work there. But the
root advances between searches: search *N+1*'s root has more stones than search *N*'s root, so
**a node at ply *p* in search *N* sits at ply *p − Δ* in search *N+1***. For a single-engine
driver Δ = 2 (one turn) and for an arena seat Δ = 4 (two turns) — either way the nodes
`T-BELOW` truncates (ply ≥ 2) are **exactly** the nodes that become the next search's ply 0 and
ply 1. F11's own "iff" therefore fails for this row, in the direction that matters.

**The bound direction, stated so the harm is precise.** A truncated node searches a SUBSET of
its real moves, so for the mover `value(subset) ≤ value(full)`. `Bound::Lower` (fail-high) stays
sound — the achieving move is in both sets. `Bound::Exact` and `Bound::Upper` do **not** hold for
the full set. A later uncapped visitor probing at ply 1 takes a cutoff on `!is_pv` alone
(`pvs.rs:245-256`); ply 0 is always PV (`beta - alpha > 1` at `pvs.rs:153`) so it cannot cut,
but **ply 1 can and does** — a non-PV child opens at `(alpha, alpha + 1)` (`pvs.rs:498`). Ply 1
is the second stone of the turn the engine plays. That is the node revision 2 fell for capping,
reached indirectly.

**REPRODUCED.** Command:

```
cd /home/tom/Projects/pistol-wt-rt3 && CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt3-target \
  cargo test --release -p pistol-search --test zz_rt3_tt -- --nocapture --test-threads=1
```

40 self-play games from the SPRT book, turn cap 40, **one `Searcher` per game so the table is
kept exactly as `search.rs:53` describes**, `Stop::Nodes(50_000)`, `quiet_radius 2`, ordering
heuristics off. `poisoned_keys` counts distinct keys stored by a node whose candidate set was
actually truncated; `poisoned_cutoffs` counts cutoffs taken on such a record;
`ply01_unsound_bound` counts those at ply ≤ 1 whose bound was `Exact` or `Upper`:

```
RT3-C/KEPT-TT row="W-N nocap"      games=40 searches=930  poisoned_keys=0     poisoned_cutoffs=0     of_which_at_ply01=0  ply01_unsound_bound=0  poisoned_tablemove_ply01=0    setfn_obs=6531587 setfn_disagree=0
RT3-C/KEPT-TT row="T-BELOW ply>1"  games=40 searches=1017 poisoned_keys=45667 poisoned_cutoffs=14365 of_which_at_ply01=41 ply01_unsound_bound=41 poisoned_tablemove_ply01=202  setfn_obs=7211933 setfn_disagree=202
RT3-C/KEPT-TT row="T-ROOT ply<=1"  games=40 searches=738  poisoned_keys=77    poisoned_cutoffs=0     of_which_at_ply01=0  ply01_unsound_bound=0  poisoned_tablemove_ply01=213  setfn_obs=4366156 setfn_disagree=127
```

| | poisoned keys | poisoned cutoffs | of those, at ply 0/1 | on an unproved bound | membership disagreements |
|---|---|---|---|---|---|
| **W-N** (no cap) — control | 0 | 0 | **0** | **0** | **0** / 6 531 587 |
| **T-BELOW** `ply > 1` *(SELECTED)* | 45 667 | 14 365 | **41** | **41** | **202** / 7 211 933 |
| **T-ROOT** `ply <= 1` *(deferred)* | 77 | **0** | **0** | **0** | 127 / 4 366 156 |

**THE CONTROL THAT ISOLATES THE CAUSE.** The same 40 games, the same scope, the same K, with
**one variable changed**: a fresh `Searcher` at every turn instead of one per game — which is
precisely the shape S3 and S5 ran in.

```
RT3-D/COLD-TT  row="T-BELOW ply>1" games=40 poisoned_keys=45752 poisoned_cutoffs=14434 of_which_at_ply01=0  ply01_unsound_bound=0  poisoned_tablemove_ply01=259 setfn_obs=7171931 setfn_disagree=259
RT3-C/KEPT-TT  row="T-BELOW ply>1" games=40 poisoned_keys=45667 poisoned_cutoffs=14365 of_which_at_ply01=41 ply01_unsound_bound=41 poisoned_tablemove_ply01=202 setfn_obs=7211933 setfn_disagree=202
```

Poisoned keys and poisoned cutoffs are unchanged to within a percent (45 752 / 14 434 against
45 667 / 14 365 — truncated nodes poison entries either way, and intra-search those entries are
consumed consistently, exactly as F11 argues). **The played-turn count goes 41 → 0.** Keeping the
table across the turns of a game is the whole of the difference, and keeping it is what
`search.rs:53` says the engine does. Note also that `setfn_disagree` is **259 with a cold table**
— the membership disagreements are the ply-band overlap between successive searches, not the
table-move promotion, since the probe's map is independent of the transposition table.

**Four things this table settles.**

1. **The null control is clean.** With no cap, the emitted set is a pure function of the key
   across a whole game with a kept table — 0 disagreements in 6 531 587 observations. So the 202
   disagreements and the 41 cutoffs are the cap's doing and nothing else's.
2. **§5 ground 5 is false as stated.** "0 membership disagreements" holds only under F11/S3's
   own sampling; with the table kept across a game the selected row disagrees with itself 202
   times. It is not sound in the transposition table.
3. **§5 ground 1 is false as stated.** The row does reach the move the engine returns: 41
   cutoffs inside the played turn on a bound the storing node could not prove, and 202
   played-turn move-ordering promotions from poisoned records, in 40 games.
4. **The asymmetry is structural and it runs the opposite way to §4's table.** `T-ROOT`'s
   truncated nodes are its ply 0 and ply 1 — nodes with FEWER stones than any later search's
   root — so no later search can ever reach them: **0 poisoned cutoffs of any kind.** (Its 127
   disagreements are the benign direction, a node emitted FULL in search *N* and truncated in
   *N+1*; it produced zero cutoffs on a truncated record.) The row that prunes the played turn
   **openly**, where the exclusion is a declared forward prune over a value that is exact, leaves
   no poisoned entry behind; the row selected for **not** touching the played turn is the one
   that touches it silently.

**Why F11's measurement could not have caught this.** Not a criticism of the number — a
criticism of the sample. The committed probe resets and runs over independent fixture positions,
and **its driver builds a new `Searcher` for every position**, so no two searches in the entire
S3 experiment share a table or overlap trees. Same for S5, the "governed-shape" divergence run:
`crates/pistol-search/tests/zz_r3_book.rs:52` — `fn run(...)` calls
`common::staged_searcher(...)` on **every invocation**, and `zz_s5_governed_turn_axis` calls
`run(&state, …)` twice per turn. Every experiment in this document therefore runs a **cold table
at every turn**, which is the one configuration in which the defect is unreachable. This is
D-424's own standing lesson arriving again — *"a check that cannot fail is not a check"* — and
§3's heading *"WHAT THE GOVERNED RUN WOULD SEE"* is not what the governed run would see.

**THE FIX IS ALREADY WRITTEN, IN THE DOCUMENT THE MATRIX CITES.** `WPQ_seed.md` §7.2, the
ADOPTED option W-E:

> "**The transposition store gains a truncation rule**, removing the poisoned-entry class rather
> than living with it. A subset maximum `>= beta` is a genuine lower bound, so **fail-high stores
> `Bound::Lower` as today**; a fail-low or exact score from a set that was **not exhausted** is
> unsound in the bound it claims, so **it stores nothing**."

and §7.1(3), the passage F11 cites for the class it kills `W-K3` with:

> "A PV node that truncates stores `Bound::Exact`, which the probe consumes unconditionally at
> every later non-PV hit. **An exact score over a SUBSET is a lower bound only.**"

The seed's rule is about **exhaustion**, not about whether the scope is a function of ply. F11
narrows it to an `is_pv`-keyed defect, uses that narrowing to kill `W-K3`, and then reports the
selected row "sound" against the narrowed version. **The general rule the seed adopted is
nowhere in matrix M2**, and §5's list of three unsettled things does not include it.

**AND THE SUCCESSOR DESIGN ALREADY ASSERTS THE FALSE STEP, IN TERMS.** While this review was
running, an untracked `docs/experiments/wp15d_design.md` (revision 1, 291 lines, mtime
`Aug 29 15:30`, not authored by this review and not modified by it) appeared in the live tree,
implementing D-478's `T-BELOW`. Its §6 does not omit the cross-search question — it answers it,
and the answer is backwards:

> "2. **A key cannot recur at a different ply band**: the stone count is a function of the key
> and increases strictly down the tree, so the root turn's keys never appear below it — **within
> a search or across the searches of a game, since a later search's root holds more stones than
> an earlier one's.**"
> — `docs/experiments/wp15d_design.md:215-218`

The argument establishes that **the root turn's keys never appear BELOW**, which is true and is
not the hazard. The hazard is the reverse direction — **a deep node's key appearing AT THE ROOT
of a later search** — and the very fact the design cites as its reason ("a later search's root
holds more stones than an earlier one's") is precisely what makes that direction not merely
possible but *guaranteed along the played line*: the root advances by exactly the stones the
engine and its opponent just placed, so search *N*'s ply-2 (or ply-4) node **is** search *N+1*'s
root. And the design's registered test pins the narrower property only —
*"the same key emits the same membership **across a search** with ordering heuristics on"*
(`:237`) — so like F11/S3 it cannot falsify the class.

**One thing the design does fix, and one it does not.** It places the truncation BEFORE
`promote_table_move` (`:75-78`), which makes the emitted set *"a pure function of (position,
`quiet_radius`, K)"* and would remove the 202 / 127 membership disagreements I measured (those
were taken with the promotion AFTER, per §5's own open question 2). **That does not touch
BLOCKING 1.** My POISON counters do not read the digest at all: they mark a key at the moment a
node whose cell vector was actually truncated stores its `Record`, and count cutoffs later taken
on that key. **The 41 unproved-bound cutoffs at ply 0/1 stand whatever side of the truncation the
table move falls on**, because their cause is that the emitted set is a function of (position,
**ply**) and ply is not a function of the key across searches. The design needs `WPQ_seed.md`
§7.2's store rule as well, and a test that keeps one `Searcher` across a game.

---

### BLOCKING 2 — the axis is still not turn-coherent. At a turn-1 root the selected row prunes one stone of a two-stone turn and not the other, which is the exact defect §1 strikes two rows for. A strictly dominating row exists and is not in the field.

**Claim attacked, verbatim:**

> "A scope is named by which nodes of the PLAYED TURN it prunes … **Ply 0 is the played turn's
> first stone, ply 1 its second, and everything from ply 2 down is the opponent's reply and
> beyond.**"
> — `matrix_M2.md` §1

> "The last two are struck as options rather than killed as ideas: **a scope that prunes one
> stone of a two-stone move and not the other is not a choice anyone would make on purpose**"
> — `matrix_M2.md` §1, on `W-K2` and `W-K0`

**THE ATTACK.** The sentence is true at every root of turn ≥ 2 and **false at a turn-1 root**,
because rule 3 gives turn 1 one stone:

```rust
crates/pistol-core/src/rules.rs:30-36
pub const fn stones_in_turn(turn: u32) -> u32 {
    if turn == FIRST_TURN {
        FIRST_TURN_STONES
    } else {
        TURN_STONES
    }
}
```

and the search's own `plies_for` says so in words (`search.rs:492-496`: *"Not twice the depth:
the first turn of a game owes one stone (rule 3)"*). A turn-1 root is a **legal, accepted root**:
`check_root` (`search.rs:468-474`) refuses only a decided game and `phase() != Phase::First`, and
a fresh `GameState::new_game()` is `phase = First`. It is the root of every game's first search.

**REPRODUCED, at the rules level.** Command:

```
cd /home/tom/Projects/pistol-wt-rt3 && CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt3-target \
  cargo test --release -p pistol-search --test zz_rt3_axis -- --nocapture --test-threads=1
```

```
RT3-A/ROOT turn=1 phase=First stones_owed=1
RT3-A/PLY ply=0 mover=P1 belongs_to_turn=1 turns_from_root=0 phase_at_entry=First  outcome=TurnComplete
RT3-A/PLY ply=1 mover=P2 belongs_to_turn=2 turns_from_root=1 phase_at_entry=First  outcome=TurnContinues
RT3-A/PLY ply=2 mover=P2 belongs_to_turn=2 turns_from_root=1 phase_at_entry=Second outcome=TurnComplete
RT3-A/PLY ply=3 mover=P1 belongs_to_turn=3 turns_from_root=2 phase_at_entry=First  outcome=TurnContinues
...
RT3-A/ROOT2 turn=3 phase=First stones_owed=2
RT3-A/PLY2 ply=0 belongs_to_turn=3 turns_from_root=0 phase_at_entry=First
RT3-A/PLY2 ply=1 belongs_to_turn=3 turns_from_root=0 phase_at_entry=Second
RT3-A/PLY2 ply=2 belongs_to_turn=4 turns_from_root=1 phase_at_entry=First
RT3-A/PLY2 ply=3 belongs_to_turn=4 turns_from_root=1 phase_at_entry=Second
test result: ok. 2 passed; 0 failed
```

At a turn-1 root **ply 1 is the opponent's FIRST stone**, not the played turn's second. So:

- **`T-BELOW` (`ply > 1`)** exempts ply 0 (the whole played turn) and ply 1 (the opponent's first
  stone), and caps ply 2 (the opponent's **second** stone). **It splits the opponent's turn 2.**
- **`T-ROOT` (`ply <= 1`)** caps ply 0 (the whole played turn) **and** ply 1 (the opponent's
  first stone). It prunes **below** the played turn, contradicting its own §1 row — *"Prunes.
  Both stones of the played turn, and nothing below it"* — and splits the opponent's turn 2 in
  the mirror direction.

**REPRODUCED, with the cap running and resolved by turn and phase instead of by ply.** Command:

```
cd /home/tom/Projects/pistol-wt-rt3 && CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt3-target \
  cargo test --release -p pistol-search --test zz_rt3_turn1 -- --nocapture --test-threads=1
```

`Tn pP : capped/seen`, `n` = `turns_from_root`, `P` = `Phase::index()`. A row is turn-coherent
iff for every turn it caps BOTH phases or NEITHER:

```
RT3-E/TURN1ROOT row="T-BELOW ply>1"     depth=4 T0p0:0/5 T0p1:0/0  T1p0:0/4    T1p1:57/57  T2p0:229/229 T2p1:78/78  T3p0:27/27 T3p1:17/17 || TURN-INCOHERENT AT: turn1(from root)
RT3-E/TURN1ROOT row="T-ROOT  ply<=1"    depth=3 T0p0:4/4 T0p1:0/0  T1p0:3/3    T1p1:0/20   T2p0:0/246   T2p1:0/113  T3p0:0/18  T3p1:0/267 || TURN-INCOHERENT AT: turn1(from root)
RT3-E/TURN1ROOT row="T-ALL   every ply" depth=4 T0p0:5/5 T0p1:0/0  T1p0:4/4    T1p1:28/28  T2p0:131/131 T2p1:61/61  T3p0:24/24 T3p1:12/12 || TURN-INCOHERENT AT: none
RT3-F/TURN3ROOT row="T-BELOW ply>1"     depth=4 T0p0:0/5 T0p1:0/68 T1p0:14/14  T1p1:0/0    T2p0:0/0     T2p1:92/92  T3p0:889/889 T3p1:96/96
RT3-F/TURN3ROOT row="T-ROOT  ply<=1"    depth=4 T0p0:5/5 T0p1:18/18 T1p0:0/0   T1p1:0/0    T2p0:0/0     T2p1:0/0    T3p0:0/35  T3p1:0/20
```

**`T-BELOW` at a turn-1 root caps 57 of 57 phase-1 nodes of the opponent's first turn and 0 of 4
of its phase-0 nodes.** The turn-3 control (RT3-F) shows both rows perfectly coherent, so the
defect is confined to the turn-1 root — but that root is not degenerate: the safety net is
**maximally** live there (F1's own comment: *"at the game's earliest plies no window anywhere has
reached a live count"*), and the cap is doing real work:

```
RT3-B/TURN1ROOT scope=0 K=0 depth_turns=3 sn_rows=799 capped=0   ply0=0 ply1=0 deeper=0
RT3-B/TURN1ROOT scope=4 K=8 depth_turns=4 sn_rows=453 capped=444 ply0=0 ply1=0 deeper=444
RT3-B/TURN1ROOT scope=5 K=8 depth_turns=3 sn_rows=671 capped=7   ply0=4 ply1=3 deeper=0
```

799 safety-net rows at the incumbent, 444 rows capped by the selected row, one completed turn of
depth gained. **And the `ply0`/`ply1` counters mislabel here by their own documentation** —
`scratch.rs:110-113`: *"Ply 0 and ply 1 are the two stones of the turn the engine returns"*. At
this root `T-ROOT`'s reported `ply1=3` is three caps on the **opponent's** first stone.

**THE MISSING ROW.** `pvs.rs:523` already exposes exactly the right unit, with a doc that states
the property the field needs:

```rust
/// How many turns this position is from the root. Both plies of a turn
/// share it, because they share a turn number.
pub(crate) fn turns_from_root(&self) -> u32 {
    self.position.state().turn() - self.root_turn
}
```

**`T-BELOW-T` = the truncation under `self.turns_from_root() > 0`.** Identical to `T-BELOW` at
every turn-≥2 root (RT3-F is the proof: `T0p0` and `T0p1` both 0-capped, everything below
capped); turn-coherent at the turn-1 root where `T-BELOW` is not; costs one already-existing
accessor instead of a ply comparison. It **dominates** the selected row on the field's own
coherence criterion and is nowhere in the document. That is the third round in a row in which
the missing row is on the axis, and D-477 — written by the session that authored this revision —
names the reading rule that would have found it: quote the line where the axis's unit is
**consumed**. `rules.rs:30` is that line and it is not quoted anywhere in `matrix_M2.md`.

*(A note on scope: `T-BELOW-T` fixes the axis. It does **not** fix BLOCKING 1 — the turn-1
misalignment and the cross-search TT poisoning are independent, and `T-BELOW-T` needs
`WPQ_seed.md` §7.2's store rule as well.)*

---

### MAJOR 3 — §4 claims to price the ruling and omits the one cost the matrix's own §0 says is not an open question, in the direction that favours the selected row. Its "four of five" is really five of six, and the sixth cell is one the instrument fills in the same run.

**Claim attacked, verbatim:**

> "**T-ROOT leads on four of five cells and the fifth is the whole reason it is not selected.**"
> — `matrix_M2.md` §4

**THE ATTACK, part one — a cell favouring `T-BELOW` that §4 omits.** F3 is unambiguous that
capping at ply 0 carries a named correctness cost:

> "**An option that caps at ply 0 makes the comment at `pvs.rs:329-335` false as written** —
> 'FAILS OPEN to the unrestricted set' becomes 'fails open to the capped set'. **This is not an
> open choice, it is the correctness hole the immediately preceding package named and fixed** …
> Every ply-0-capping row below carries this as a cost, not as a question."
> — `matrix_M2.md` F3

`T-ROOT` (`ply <= 1`) caps at ply 0. **The five-cell table in §4 does not contain this row at
all.** The section whose stated purpose is *"a matrix that recorded only the reasons for a ruling
would not be a matrix"* omits the one cost its own §0 marks as settled-and-not-a-question, and
the omission runs against the row §4 is arguing for. I verified F3's site is real and unchanged
at HEAD (`pvs.rs:336-347`, the `root_restrict` block; `set.forced == 0` on a safety-net row
makes `forced_intact` vacuously true), and that the seat is armed
(`configs/gate_staged_solver_v0.toml`, `configs/bench_wp18c_solver_on.toml`). **NOT REFUTED —
this cell is real and it is missing.**

**THE ATTACK, part two — a cell favouring `T-ROOT` more strongly than stated.** §4 and §3 both
leave `T-ROOT`'s count-two-cells-cut as an em-dash:

> "| **T-ROOT**, K = 8 | **677 (33.9 %)** | 6 783 / 461 602 (**1.5 %**) | **—** | **0.770** |"
> — `matrix_M2.md` §3

The instrument reports `opp2_in_pool` and `opp2cut` on every seat; the cell needed no new run.
MEASURED by me on the same 2 000 openings (below): **`T-ROOT` cuts 20 890 of 3 513 762 in-pool
count-two cells — 0.59 % — against `T-BELOW`'s 69.3 %.** The row §4 defers excludes **118× fewer**
of the only threat-shaped class a safety-net row can hold. That is a sixth cell, it is decisive,
it favours `T-ROOT`, and it was one field-width away in the author's own output line. D-291's
clause applies exactly: a number that could have been measured in the run that was already
taken.

---

### MAJOR 4 — §5's calibration premise is refuted by extending its own grid by one point. The book column is not flat; it is monotone toward the grid floor, and so is the column §5 nominates as the one that moves.

**Claim attacked, verbatim:**

> "**F10 is the trap** — completed depth is monotone in narrowing — but §3's book column is
> **nearly FLAT in K for this row (535 / 524 / 514)**, which is what makes a non-degenerate rule
> possible here and is stated before any rule is written. **The wall-time column is the one that
> moves.**"
> — `matrix_M2.md` §5

**THE ATTACK.** The grid stops at K = 16. Its neighbour scope's own committed column collapses
between 16 and 32 (`artifacts/wp15d_m2_evidence_v2.txt:303-306`, `scope1`: K=4 → 1 280,
K=8 → 1 083, K=16 → 964, K=32 → **682**), so three points ending at 16 are not evidence of
flatness over a calibration grid. I took the fourth point.

**REPRODUCED.** Command (five seats, one per process so the global scratch statics do not
interfere):

```
cd /home/tom/Projects/pistol-wt-rt3 && CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rt3-target \
  cargo test --release -p pistol-search --test zz_rt3_book -- --nocapture --exact <seat>
```

```
RT3-BOOK/nodes50000/r2/scope0/K0  openings=2000 sn_rows=1936431 capped=0       ply0=0    ply1=0    deeper=0       depth_hist=[0,10,1869,113,8,0,0,0] ge3=121  ms=388850
RT3-BOOK/nodes50000/r2/scope5/K8  openings=2000 sn_rows=461602  capped=6783    ply0=5455 ply1=1328 deeper=0       depth_hist=[0,0,1323,570,107,0,0,0] ge3=677 ms=294436
RT3-BOOK/nodes50000/r2/scope4/K8  openings=2000 sn_rows=1724042 capped=1593643 ply0=0    ply1=0    deeper=1593643 depth_hist=[0,0,1476,305,215,4,0,0] ge3=524 ms=437223
RT3-BOOK/nodes50000/r2/scope4/K32 openings=2000 sn_rows=1785015 capped=1657681 ply0=0    ply1=0    deeper=1657681 depth_hist=[0,0,1510,422,68,0,0,0]  ge3=490 ms=437413
RT3-BOOK/nodes50000/r2/scope0/K8  openings=2000 sn_rows=263291  capped=263291  ply0=6925 ply1=1587 deeper=254779  depth_hist=[0,0,0,1205,790,5,0,0]   ge3=2000 ms=456500
```

| `T-BELOW` K | openings at depth ≥ 3 | wall ratio (this run) |
|---|---|---|
| 4 | 535 *(committed)* | 1.129 *(committed)* |
| 8 | **524** *(replicated exactly)* | 1.124 |
| 16 | 514 *(committed)* | 1.139 *(committed)* |
| **32** | **490** *(NEW)* | **1.125** |

**The column is not flat and it is not non-degenerate.** It is monotone decreasing at every
step, and the single step from 16 to 32 (**−24**) is larger than the entire K = 4→16 span
(**−21**) that §5 calls "nearly FLAT" — the slope steepens, it does not level. The argmax over
any grid is that grid's smallest K, which is F10's degeneracy verbatim: *"Completed depth on
`spread_v1` is monotonically improved by narrowing, without bound, so it ranks options by how
hard they narrow."* §5's inference is also backwards on its own terms: a maximand that is FLAT
cannot select K at all, and one that is MONOTONE selects the extreme. Flatness is not what makes
a non-degenerate rule possible; it is what makes the rule vacuous, which is exactly
`wp19_design_REVIEW_rev2.md`'s M1 finding — *"under the repository's own standing ttd convention
the new rule again selects the grid maximum"* — recurring on a different key.

**And the wall-time column does not rescue it.** §5 says *"The wall-time column is the one that
moves."* MEASURED here at K = 8 vs K = 32: **1.124 vs 1.125**, a 0.1 % difference over a 4×
change in K, against a 34-opening move on the column §5 calls flat. The column §5 nominates as
the discriminator is the one that does not move; the column it dismisses as flat is the one that
does, monotonically, toward the grid floor. **Neither registered column has an interior optimum,
so the calibration N1(a)(b)(c) is meant to constrain has nothing to constrain it.**

---

### MAJOR 5 — §5 ground 1 says D-124 "is not engaged". D-124 is precisely about a search that narrows at the tail, which is what `T-BELOW` does at 92.4 % of safety-net rows; the matrix has it exactly backwards, and assigns it to the wrong row.

**Claims attacked, verbatim:**

> "`WPQ_seed.md` §7.1's rule is satisfied exactly, and **D-124 is not engaged**." — §5, ground 1

> "**Prunes.** Both stones of the played turn, and nothing below it. **This is where D-124 says
> no oracle catches a mistake**" — §1, the `T-ROOT` row

**THE ATTACK.** D-124's own first sentence:

> "D-124: **A value-differential oracle cannot see a search that narrows its candidate set at the
> TAIL**, and that is a limit of the method rather than of this matrix. Verified with a
> reproducer before being written down (D-116): **dropping the worst-ordered candidate at every
> node of `pvs::visit`** — one `cells.pop()` after `order` — leaves every one of the oracle's
> assertions green"

D-124's reproducer drops a tail candidate at **every node**, deep nodes included, and the oracle
stays green. `T-BELOW` is a score-ranked top-K truncation — a tail drop — applied at every node
below the played turn, binding at **1 593 643 of 1 724 042 safety-net rows (92.4 %)**. That is
D-124's class in its purest form. `T-ROOT`, by contrast, binds at **6 783 of 461 602 rows
(1.5 %)** and does so at a node whose value is exact and whose exclusion is declared. **The
matrix assigns D-124 to the row that engages it 61× less and declares it not engaged for the row
that engages it most.** D-124's closing sentence is addressed to this work package by name —
*"Stage 1's very next candidate-set change is exactly this shape … the work package that lands
it inherits the fact that this oracle will not judge its dominance rule, and owes a check of its
own"* — and §5's ground 1 is the sentence that discharges that owed check by asserting it does
not apply. **NOT REFUTED.** This does not by itself change which row is better; it removes one of
the six grounds §5 rests the selection on.

---

### MAJOR 6 — `T-BELOW`'s first stated kill condition is registered on a channel the matrix's own receipt shows cannot move.

**Claim attacked, verbatim:**

> "**Kill.** A corpus bench outside its registered bracket, or an SPRT that reads `h0`."
> — `matrix_M2.md` §1, the `T-BELOW` row

**THE ATTACK.** The matrix establishes twice that the corpus channel is inert for this row:

> "**It is inert where the class is rare**: identical node counts and identical completed
> depths, position for position, on the corpus." — §5, ground 4

and the receipt behind it, `artifacts/wp15d_turn_axis_v1.txt:202-206`:

```
S2/CORPUS/nodes50000/r2/scope0/K0  sum_nodes=1104026 sum_ms=4776 sn_rows=66 capped=0
S2/CORPUS/nodes50000/r2/scope4/K4  sum_nodes=1104026 sum_ms=4828 sn_rows=17 capped=9
S2/CORPUS/nodes50000/r2/scope4/K8  sum_nodes=1104026 sum_ms=4795 sn_rows=18 capped=10
S2/CORPUS/nodes50000/r2/scope4/K16 sum_nodes=1104026 sum_ms=4790 sn_rows=21 capped=13
```

Node counts identical, completed depths identical, and total wall time spanning **4 776–4 828 ms
(1.1 %)** — noise at this granularity. A bench bracket over that channel cannot be exceeded by
anything the row does; it can only be exceeded by measurement noise. **A kill condition that
cannot fire is not a kill condition** — D-424's own lesson, stated in this repository, about
exactly this shape. The second kill condition (an SPRT that reads `h0`) is real and does the
whole job; the first should be deleted or restated as the no-regression check F9 already calls
it (*"The corpus bench is therefore a no-regression check and never a gain channel"*), which is
a third statement of the same fact and D-423's claim-made-twice pattern.

*(Secondary, recorded not pressed: `sn_rows` moves 66 → 18 between the control and the selected
row, so the trees are NOT identical and the cap binds 10 times; the identical node totals are
the budget, not the tree. The conclusion — no corpus regression — survives; the word "inert"
overstates what was measured.)*

---

### MINOR 7 — the two cells §4's headline comparison rests on have no committed receipt. I re-took them and they reproduce exactly.

The dispatch flagged this and it is correct: **`scope5` (`ply <= 1`, `T-ROOT`) appears in no
committed artifact.**

```
$ for f in artifacts/wp15d_turn_axis_v1.txt artifacts/wp15d_turn_axis_book_v1.txt \
           artifacts/wp15d_m2_evidence_v1.txt artifacts/wp15d_m2_evidence_v2.txt; do
    echo "=== $f"; /usr/bin/grep -o "scope[0-9]*" $f | LC_ALL=C sort -u | tr '\n' ' '; echo; done
=== artifacts/wp15d_turn_axis_v1.txt        scope0 scope1 scope4
=== artifacts/wp15d_turn_axis_book_v1.txt   scope0 scope1 scope4
=== artifacts/wp15d_m2_evidence_v1.txt      scope0 scope1
=== artifacts/wp15d_m2_evidence_v2.txt      scope0 scope1 scope3
```

and the committed book driver's grid never asks for it — `zz_r3_book.rs:88`:
`for (cap, scope) in [(0,0), (4,4), (8,4), (16,4), (8,1)]`. So §1's `T-ROOT` bullet and §3's and
§4's `T-ROOT` rows are MEASURED-marked cells standing on a log that is not in the record, in a
work package where D-318 records a measured pattern of this author's MEASURED cells not
reproducing.

**I re-took the seat and the cells REPRODUCE EXACTLY.** Every figure in §1's `T-ROOT` bullet:
**677** openings at depth ≥ 3 (`ge3=677`), **6 783 of 461 602** rows capped (1.47 %), ply split
**`ply0 = 5 455, ply1 = 1 328`** — all identical to the matrix, on an independently written
driver. The wall ratio is **0.757×** in my run against the matrix's 0.770× (five concurrent
seats under load ≈ 6; both are "≈ 23–24 % faster than the incumbent"). I also replicated
`T-BELOW` K = 8 (**524**, `1 593 643 / 1 724 042`) and `T-ALL` K = 8 (**2 000**, `263 291`, wall
**1.174×** — the matrix's figure, confirmed in-run) byte-for-byte.

**So this is a receipt-hygiene finding, not a number finding.** The cells are honest. But §4 is
the section revision 3 exists to add, and its headline comparison should not rest on output that
`sha256sum` cannot reach. **The remedy is to run the seat and land the log**, which costs about
five minutes — not to re-argue the cell.

---

### MINOR 8 — §0's bolded claim and its own next clause disagree, and its ply-breakdown columns are one position of four without saying so.

> "**The selected row prunes NOTHING on this fixture** — zero prune events at 21, 51 and 99
> stones" — `matrix_M2.md` §0

The second clause is exact and verified (`artifacts/wp15d_turn_axis_v1.txt`,
`S1/r3/except-root-turn/K8`: `capped=0` at p01, p02, p03 for every K on the grid). The bolded
first clause is false on the same receipt: at 11 stones `capped=66` of `sn_rows=369`. §1 states
it correctly (*"0 prune events at 21 / 51 / 99 stones"*), which makes this a claim the document
makes twice with different content — D-423/D-424's named defect, in the paragraph that
introduces the selection.

Separately, §0's table is headed *"a breakdown of which ply each scope's cap actually bound at"*
alongside a depth column covering **four** positions, but the ply columns are the **99-stone
position alone** (`ply0=4, ply1=0, deeper=157` is `S1/r3/every-ply/K8 p03`; `ply1=170` is
`S1/r3/except-ply0/K8 p03`). Nothing in the table says so.

---

### MINOR 9 — the round-3 receipt names a base revision its worktree was not at.

`artifacts/wp15d_turn_axis_book_v1.txt` header: *"Base revision
`1fa6810e55f15194fa92bba37809fe0680c6d49b`; worktree `/home/tom/Projects/pistol-wt-r3`"*. That
worktree is at `4540540` (`git rev-parse HEAD` in it). Both are docs-only commits so no code
differs and no number is affected, but a pinned revision that does not match the tree it was
taken in is the pin failing at its one job.

---

## THE STRONGEST SURVIVING ATTACK

> **`T-BELOW`'s defining property — that it prunes nothing in the move the engine returns — holds
> only inside a single search, and this engine does not run that way.** The transposition table is
> kept across the searches of one game (`crates/pistol-search/src/search.rs:53`, *"successive
> searches in one game share what they learned"*; `clear()` at `:192` is *"what a new game does"*;
> `probe` at `tt/mod.rs:129` validates no age). F11's soundness criterion is right — *"the bound
> is sound for a later visitor iff the same key always emits the same set"* — and its structural
> argument that ply is a function of the key is **correct within one search and silently scoped to
> one**: between searches the root advances, so a node truncated at ply ≥ 2 in search *N* **is**
> the root and root-child of search *N+1*, where it is emitted whole. A truncated node's value is
> a lower bound only, so its `Bound::Exact` and `Bound::Upper` do not hold for the full set.
> MEASURED, 40 self-play games with one `Searcher` per game, `Stop::Nodes(50_000)`: **45 667
> records written by a truncated node, 14 365 cutoffs taken on them, 41 of those at ply 0 or ply 1
> — inside the played turn — every one on a bound the storing node had not proved**, plus 202
> played-turn table-move promotions from poisoned records and 202 membership disagreements in
> 7 211 933 observations. **The uncapped control is 0 / 0 / 0 / 0 / 0 in 6 531 587 observations,
> and `T-ROOT` — the row D-478 defers for pruning the played turn — is 0 poisoned cutoffs and 0
> at ply 0/1**, because its truncated nodes hold fewer stones than any later root and can never be
> revisited. **The fifth cell of §4, the sole cell the selection rests on, is inverted by
> measurement: the row that prunes the played turn openly leaves nothing behind, and the row
> chosen for not touching the played turn touches it silently.** Every experiment in the document
> is blind to this because every driver builds a fresh `Searcher` per position
> (`zz_r3_book.rs:52`), including the run §3 titles *"WHAT THE GOVERNED RUN WOULD SEE"*. The fix is
> not new work: `WPQ_seed.md` §7.2 — the option the seed ADOPTED, and the section F11 cites for
> the class it kills `W-K3` with — already states it, *"a fail-low or exact score from a set that
> was not exhausted is unsound in the bound it claims, so it stores nothing"*, and matrix M2 does
> not contain that rule anywhere. Compounding it, the field is incomplete for the third round
> running on the axis it was rewritten for: rule 3 gives turn 1 one stone
> (`crates/pistol-core/src/rules.rs:30-36`), `check_root` accepts a turn-1 root
> (`search.rs:468-474`), and there ply 1 is the opponent's first stone — MEASURED, `T-BELOW` caps
> **57 of 57** phase-1 nodes of the opponent's turn and **0 of 4** phase-0 nodes, which is
> verbatim the "prunes one stone of a two-stone move and not the other" defect §1 strikes `W-K2`
> and `W-K0` for. The dominating row is one already-existing accessor away —
> `turns_from_root() > 0` (`pvs.rs:523`, *"Both plies of a turn share it"*) — and D-477, appended
> by the session that wrote this revision, names the reading rule that would have found it: quote
> the line where the axis's unit is consumed. That line is `rules.rs:30`, and it is quoted nowhere
> in `matrix_M2.md`.

---

## WHAT WOULD FLIP THIS — i.e. what makes `T-BELOW` the right call

Stated plainly, because I think the operator's **principle** is right and only its
implementation is wrong. The selection survives, as **`T-BELOW-T`**, on:

1. **The axis fix.** Guard on `self.turns_from_root() > 0` (`pvs.rs:523`), not on `ply > 1`. One
   accessor. Behaviourally identical at every turn-≥2 root (RT3-F), turn-coherent at the turn-1
   root. Re-take RT3-E against it and require `TURN-INCOHERENT AT: none`.
2. **The store fix, which is `WPQ_seed.md` §7.2's adopted rule and not a new invention.** A node
   whose candidate set was truncated stores `Bound::Lower` on a fail-high and **stores nothing**
   on a fail-low or an exact score. Re-take RT3-C and require `ply01_unsound_bound = 0` with the
   table kept across a game. That is the falsifier; the committed S3 probe is not one, because
   its driver never shares a table between two searches.
3. **A `T-ROOT` receipt.** Five minutes of machine time. §4's headline comparison should stand on
   a `sha256sum`-able log. (Its numbers are correct — I checked.)
4. **A calibration key that is not completed depth.** §5's flatness premise is refuted at K = 32
   (535 / 524 / 514 / **490**, monotone, slope steepening) and the wall-time column it nominates
   instead moves 0.1 % over a 4× change in K. Neither has an interior optimum. `wp19_design_
   REVIEW_rev2.md` BLOCKING N1(a)(b)(c) is the right checklist and it is not yet satisfied.
5. **§4 completed in both directions.** Add F3's ply-0 fail-open cost against `T-ROOT`; fill in
   `T-ROOT`'s count-two cell, which the instrument already prints (**0.59 %** against `T-BELOW`'s
   **69.3 %**). The honest headline is "five of six", not "four of five", and the row is still
   correctly deferred on the fifth — but only once the fifth is repaired, because as measured
   today `T-BELOW` loses that cell too.

**Of these, (1) and (2) are the blockers. (2) is a correctness fix and D-424 puts it beyond any
overrule; (1) is an axis judgement and an operator ruling may reach it, though it costs one line
of code to do properly and the field has now been rebuilt twice for want of it.**

---

## WHAT I CHECKED AND FOUND SOUND

Recorded so the next round does not re-derive it. All of the following I attacked and could not
break.

- **Every digest.** All six named artifacts hash byte-exact at `bb2fee2`.
- **Every `file:line` citation in the matrix resolves correctly at HEAD**, which is not true of
  every document in this work package's history and is worth saying: `pvs.rs:153` (`visit(
  depth_plies, -INFINITY, INFINITY, 0)`), `:245` (`let known = self.table.probe(...)`), `:368`
  (`let mut best_score = -INFINITY;`), `:450` (`self.table.store(`, F11's "449-467"), `:498`
  (`self.visit(depth_plies, alpha, alpha + 1, ply)`), `:336-347` (F3's `root_restrict` block),
  `:174-186` (`Run::salvage`'s doc), `:523` (`turns_from_root`), `search.rs:328-329`,
  `staged.rs:222-236` (F1, quoted whole and accurately), `validate.rs:94-97` and `:104-121`.
- **F1** — the safety net is a guard, not a construction; Tier F provably empty on the rows
  `batched` is reached from; the node-local scoping of the claim is correctly narrowed from
  revision 1's subtree version.
- **F4** — a widening trigger cannot bind on this recursion. Verified structurally at HEAD.
- **F6** — `Run::salvage`'s doc says what the matrix says it says, and the `PartialRoot` /
  `Fallback` distinction is drawn correctly; D-478's own correction of the dispatched
  "unsearched top-1 play" is right and important.
- **F10** — completed depth is a degenerate maximand on this class. Sound, and MAJOR 4 above is
  an application of it to the matrix's own §5 rather than a dispute with it.
- **F11's structural argument, within one search.** Ply is a function of stone count is a
  function of the key; a ply-uniform scope emits one set per key **inside a search**. Correct.
  BLOCKING 1 is about the boundary the argument does not cross, not about the argument.
- **F12** — verified independently: `/usr/bin/grep -rln "quiet_top_k" configs/ | LC_ALL=C sort |
  wc -l` returns **12**; `validate.rs:94-97` refuses `quiet_top_k == 0` so the schema has no
  off-value; `validate.rs:104-121` cross-validates `widen_schedule` against it. §5 ground 6's
  answer (a new key with its own name and off-value, the existing two left
  validated-and-unread) is the right shape.
- **§5 ground 4's conclusion** — no corpus regression. Node counts and completed depths are
  identical position for position (`S2/CORPUS` rows). See MAJOR 6 for the wording.
- **§3's divergence numbers as arithmetic** — 24/631 = 3.80 % and 21/670 = 3.13 %, both present
  in `artifacts/wp15d_turn_axis_book_v1.txt`. What they do not have is the governed run's shape
  (BLOCKING 1's closing paragraph).
- **Every committed book cell I re-ran reproduces exactly**: W-N 121; `T-BELOW` K=8 524 with
  `1 593 643 / 1 724 042` and 69.3 %; `T-ALL` K=8 2 000 with `263 291` at 1.174×; and the
  unreceipted `T-ROOT` K=8 at 677 with `6 783 / 461 602` and `ply0 = 5 455, ply1 = 1 328`.
- **§0's "zero prune events at 21, 51 and 99 stones"** — verified on the receipt at every K.
- **T-ALL's kill** — it is `T-ROOT ∪ T-BELOW` exactly, so neither verdict on it attributes.
  Survives the axis change as the matrix says.
- **`W-K3`'s kill** stands on its own measurement (821 disagreements in 1 254 229) regardless of
  BLOCKING 1; BLOCKING 1 says the *other* scopes are not clean either, not that `W-K3` is.
- **The struck rows are correctly struck** as turn-incoherent — at turn-≥2 roots. At a turn-1
  root the surviving rows join them, which is BLOCKING 2.

---

*Round 3. `docs/experiments/matrix_M2.md` revision 3 at `bb2fee2`, matching HEAD, tree clean.
Neither earlier red-team report is edited by this one. The live tree was not modified; all
measurement was taken in a detached worktree at `/home/tom/Projects/pistol-wt-rt3` with its own
`CARGO_TARGET_DIR`, which is LEFT IN PLACE because its instrument has not been exported.*
