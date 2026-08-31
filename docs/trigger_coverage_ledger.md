# Trigger-coverage ledger — positions holding a proof the TRIGGER never reaches

**What this file is.** An append-only register of positions on which the solver
is known to prove something, and at which `pistol-search`'s trigger **does not
fire during a governed search** — so the proof is unreachable by the search at
any depth, whatever a detector does.

**Why it exists, and why these are not recall-fixture failures.** D-536 (ruling
2 of the detector arc's three) defines the recall fixture over rows that FIRE,
and moves the never-firing rows here. The reason is a unit distinction the arc
paid to learn: **a position the trigger never reaches is a fact about the
TRIGGER'S WIDTH, not about any detector's precision.** Filing it as a fixture
failure charges a detector for ground it was never offered — and a gate that
does that ships red on correct code, which is D-481's own defect class.

**What a row here means, stated once.** It means the incumbent trigger
(`SolverTrigger::AnyOpenFour`: fire when either side holds a live window with
four or more own stones) admits **no firing anywhere** in a governed search from
this position, while the solver called DIRECTLY on it returns a proof. It does
NOT mean the position is unwinnable, that the search plays it badly, or that the
trigger is wrong — only that these two facts are both true and that nothing in
the current architecture connects them.

**What closes a row.** A trigger-width change that makes the position fire. At
that point the row LEAVES this ledger and RETURNS to the recall fixture, where
it is a recall question again (D-536's flip clause). A row is never closed by
argument, only by a measurement showing a firing.

**Append-only.** Rows are added and their status column is updated; a row is
never deleted, because a ledger that forgets what it once held cannot be used to
tell whether coverage improved.

---

## Rows

| id | position | class | measured | status |
|---|---|---|---|---|
| TC-1 | `g001-t44-p2` — sealbot anchor v1, game 1, turn 44, P2 to move, 85 stones | was a D-512 VALUE row | **0 firings, 0 proofs** in a governed search at `nodes 50000`, cap 2048 (7,742 search nodes spent, none of them a trigger point); the uncapped probe proves `win` at **86 visits**, depth 2 turns | **OPEN** |
| TC-2 | `g002-t39-p1` — sealbot anchor v1, game 2, turn 39, P1 to move, 75 stones | was a D-512 VALUE row | **0 firings, 0 proofs** in the same governed search (6,886 search nodes); the uncapped probe proves `win` at **714 visits**, depth 2 turns | **OPEN** |

### The columns a detector would have read at each, and why they explain the row

| id | `mover_hot` | `opp_hot` | `mover_w1` | `opp_w1` | `mover_l3` | `opp_l3` | cover |
|---|---|---|---|---|---|---|---|
| TC-1 | 0 | 0 | 0 | 0 | 6 | 2 | none |
| TC-2 | 0 | 0 | 0 | 0 | 5 | 0 | none |

**Neither side is hot at either position**, which is exactly the trigger's own
negation — so the trigger declines at the root, and the measurement above says
it declines at every node below it too. Both positions carry live THREES in
quantity (6 and 5 for the mover) and no fours at all: **the proofs are reached
through threat sequences the four-stone trigger has no way to see the start
of.** That is the trigger-width finding these rows are, stated in the columns
that make it one.

### Receipts

- **The zero-firing measurement**: `artifacts/stage3c_census_value_fixture_v1.txt`,
  a governed search from each of D-512's seven rows at `nodes 50000`, cap 2048,
  on the cold-table seat D-527 corrected. TC-1 and TC-2 are its entries 0 and 3,
  both `firings 0 invocations 0 proofs 0`.
- **The columns**: `artifacts/stage3b_value_fixture_v1.txt`, whose `columns`
  lines carry them. **That artifact carries NO cap ladder for these two rows**
  — the instrument prints `NOT-A-FIRING-POINT` and declines to probe, because
  asking `solve_defender` at a position where neither side is hot violates a
  precondition and panics (D-525). Citing it for the proof figures would be
  citing a run that did not take them.
- **The proof figures `86` and `714`**: `artifacts/wp18b_probe_v1_results.txt`
  and `artifacts/wp18b_probe_v2_results.txt`, which agree — both give
  `value win nodes 86 … depth_turns 2` and `value win nodes 714 … depth_turns 2`,
  taken with NO node cap. They are the same figures D-512 states.
- The `stage3*` artifacts are digest-indexed in
  `artifacts/stage3_round2_export_receipt_v1.txt`, anchored in
  `docs/experiments/overnight_export_receipt.md`.

### What is NOT in this ledger and why

`g002-t12-p2` and `g002-t10-p2` do not fire at their own position but **do fire
in-tree and find a proof there**, so they stay in the recall fixture (D-536).
`g001-t46-p2` and `g002-t41-p1` fire at the root and prove in one node; D-536
retains them rather than striking them for being cheap. `g001-t42-p2` remains
CALL-RECALL-ONLY on D-512's own terms.
