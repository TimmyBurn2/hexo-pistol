# WP-1.8b — anchor-position probe registration (diagnostic; NOT a strength claim)

**Status: REGISTERED BEFORE THE RUN. This document governs a diagnostic
probe of the WP-1.8a v0 solver on the sealbot anchor's own positions. It
produces no strength claim, moves no config, and judges nothing by SPRT.**
Its single decision output is the BRANCH (A or B) below, which selects
whether section 2 of WP-1.8b proceeds on the v0 solver or takes the M4
one-free-stone widening round first.

## 1. The positions

The two distinct stone sequences the anchor measured (D-438: 40 games, two
sequences, each replayed 20 times), read from the governed transcripts
`artifacts/sealbot_anchor_v1/g001.jsonl` and `g002.jsonl` (sha-indexed in
`MANIFEST.sha256`, `e2821749…`, verified 123 files / 0 failures at the
merge, receipt in the merge review). Game 1: pistol = p1, sealbot = p2,
sealbot wins at turn 46. Game 2: pistol = p2, sealbot = p1, sealbot wins at
turn 41.

**The position set: EVERY to-move position of both games from turn 2
onward** — the prefix of applied stones before that turn's own placement,
the server-played origin included, in submitted order. Expected count: 45
positions from game 1 (turns 2-46) and 40 from game 2 (turns 2-41), 85
total; the extractor's count must equal the number of `event: "turn"`
records (dry-run criterion D-1). For each position the solver is asked the
same question the v0 policy game asks: does the MOVER force a win. A `win`
on a pistol-to-move position would be a proof pistol missed; a `win` on a
sealbot-to-move position is a proof of the conversion sealbot executed.
Both are recorded; only the latter can serve branch A.

## 2. The registered branch, written before the probe runs

Verbatim from the WP-1.8b commissioning dispatch:

> - A: the solver proves at least one win for the side that actually won,
>   at or before the turn on which sealbot's own tree collapsed (its last
>   three answers, 1559 -> 581 -> 1 nodes). Proceed to section 2 on v0.
> - B: it proves none. The M4 one-free-stone widening (licensed in 1.8a
>   §9a) is applied: ONE design round, ONE impl round, both reviewed, gates
>   (a) (b) (d) re-run green, then the probe re-runs once. Whatever it says
>   the second time, proceed to section 2. No second widening.
>
> Default on any ambiguity in reading the branch: A is only A if the proof
> is on a position from the transcript, unedited. Anything else is B.

**The collapse turn, fixed from the run's own bytes before the probe
runs.** The dispatch's parenthetical ("1559 -> 581 -> 1") is a gloss that
byte-matches NEITHER game's tail; the measured tails
(`artifacts/sealbot_anchor_v1_analysis_sealbot_depths.txt`, diagnostic
replay, every move reproduced) are: game 1 turns 42/44/46 = 14,685 / 581 /
1 nodes; game 2 turns 37/39/41 = 62,563 / 629 / 1 nodes. The registered
collapse turns are the FIRST turn of each tail — **game 1: turn 42; game
2: turn 37** — the strictest defensible reading (an earlier bound makes A
harder, never easier). **Branch A holds iff the solver returns `win` for
the eventual winner TO MOVE (sealbot) at turn ≤ 42 in game 1 or turn ≤ 37
in game 2, on an unedited transcript prefix.** A win proven only at later
turns, only for pistol, on any edited position, or under any config other
than `configs/solver_v0.toml`, is NOT A. Everything else is B, and B's
consequence is the dispatch's own: the M4 widening round, then ONE re-run
of this probe, then section 2 regardless.

## 3. Caps, stated before running

| Cap | Value |
|---|---|
| Wall cap per position | **60 s**, `timeout(1)`; exit 124 records `wall-cap` as the position's answer — a bounded non-answer, never a verdict |
| Node cap | **None exists in the v0 API**: `Solver::new` takes `epsilon` and `tt_entries` only (MEASURED, `crates/pistol-solver/src/solver.rs`); nodes are RECORDED per returning position, not capped |
| Whole probe | bounded by construction: 85 positions × 60 s = 85 min worst case; expected minutes (most positions are quiet midgame) |

## 4. The instrument, with the revisions that govern it

| Artefact | Role | Revision |
|---|---|---|
| `tools/wp18b_probe_extract.py` | transcripts -> per-position fixture files + `positions.tsv` index; driving test `crates/pistol-cli/tests/wp18b_probe_extract_test.rs` | `e669603` |
| `crates/pistol-solver/examples/wp18b_probe.rs` | solves each fixture, prints `case <name> value <v> nodes <n> seesaw <s> depth_turns <d>`, asserts NO expectation | `e669603` |
| `ProofTree::win_depth_turns` | the depth column: attacker-turn depth of the witness DAG | `e669603` |
| `configs/solver_v0.toml` | the solver config, unchanged since WP-1.8a | unchanged through the merge |
| `target/release/examples/wp18b_probe` | the binary, built `--release --locked` at `e669603` | digest recorded with the dry run below |

A change to any of these reopens this registration, exactly as an
amendment would (CLAUDE.md instrument rule).

## 5. The literal commands

**The governed block** (only `$SCRATCH` differs from the dry-run form):

```bash
set -euo pipefail
SCRATCH=<scratch>
python3 tools/wp18b_probe_extract.py "$SCRATCH/probe" \
  artifacts/sealbot_anchor_v1/g001.jsonl artifacts/sealbot_anchor_v1/g002.jsonl
: > "$SCRATCH/probe_results.txt"
for f in "$SCRATCH"/probe/*.txt; do
  name=$(basename "$f" .txt)
  if timeout 60 target/release/examples/wp18b_probe "$f" configs/solver_v0.toml \
      > "$SCRATCH/one.txt" 2> "$SCRATCH/one.err"; then
    grep '^case ' "$SCRATCH/one.txt" >> "$SCRATCH/probe_results.txt"
  else
    rc=$?
    if [ "$rc" -eq 124 ]; then
      echo "case $name value wall-cap nodes - seesaw - depth_turns -" >> "$SCRATCH/probe_results.txt"
    else
      echo "PROBE VOID: $name exited $rc: $(cat "$SCRATCH/one.err")" >&2
      exit 1
    fi
  fi
done
wc -l < "$SCRATCH/probe_results.txt"
```

**The dry-run form** is this block with the two transcript arguments
replaced by the stub pair (§6) and `timeout 60` replaced by `timeout 60`
for three legs and `timeout 0.05` for the dedicated wall-cap leg D-4.

## 6. The dry run — input, criteria, record

**The input**: two stub transcripts, same JSONL schema, shorter games —
the exact bytes embedded in the driving test
`crates/pistol-cli/tests/wp18b_probe_extract_test.rs` (`STUB_G1`,
`STUB_G2`), differing from the governed input only in identity.

**The criteria, each tied to the defect class it excludes**:

- **D-1 — count**: the extractor prints `extracted 5 positions` and
  `positions.tsv` holds 5 case rows. The referent is EXTERNAL: the stubs
  contain 3 and 2 `event: "turn"` records, countable from the bytes by
  hand. *Excludes: an extractor that drops, duplicates or invents turn
  records* — a count the extractor computed itself cannot catch its own
  drift.
- **D-2 — prefixes**: the `stones_before` column reads `1 3 5 1 3` and the
  turn-4 fixture's `plies` line is `0,0 2,-2 1,0 0,1 0,5`, both derived BY
  HAND from the stub bytes, not by the extractor. *Excludes: a
  mis-accumulated prefix* (the current turn's own stones leaking in, the
  origin dropped, a first-stone-win second stone applied).
- **D-3 — the probe**: exactly one `case …` line per fixture, `summary 1
  cases`, exit 0, `depth_turns` a nonnegative integer on every line.
  *Excludes: a probe that skips or mangles a case and still exits 0.*
- **D-4 — the wall-cap leg**: `timeout 0.05` on the same command records
  `value wall-cap` for the case and the block CONTINUES to exit 0.
  *Excludes: a wall-cap treated as a verdict, or one that aborts the whole
  probe.*

**The record** (taken before the governed probe, at instrument revision
`e669603`):

| Criterion | Observed | Verdict |
|---|---|---|
| D-1 | `extracted 5 positions -> …positions.tsv`; 5 case rows in transcript order | **MET** |
| D-2 | `stones_before` column `1 3 5 1 3`; turn-4 fixture `plies 0,0 2,-2 1,0 0,1 0,5` (byte-pinned by the driving test) | **MET** |
| D-3 | one `case …` line per fixture, `summary 1 cases` each, block exit 0; all five stubs `value nowin nodes 1` (quiet stub games — expected; the governed set is what carries the live threats) | **MET** |
| D-4 | `timeout 0.05` legs: every case recorded `value wall-cap`, the block continued and exited 0 | **MET** |

Binary digest at the dry run: `target/release/examples/wp18b_probe` sha256
`ad9755474a0dfcc881eb9d372a6084ecb0eda645ba6b1c3af6e3d4da01f56752`.

**AND THE DRY RUN CAUGHT A REAL DEFECT, fixed before the governed run**:
the block as first registered globbed `"$SCRATCH"/probe/g*.txt` — coupled
to the GOVERNED transcripts' filename pattern, so on the same-kind stub
input the glob matched nothing and the block refused (`PROBE VOID: g*
… No such file or directory`, exit 1). The registered glob is now
`"$SCRATCH"/probe/*.txt` — the block iterates whatever the extractor
produced, by kind and not by name. Recorded here because it is the
dry-run rule working, and because the fix is visible in the diff of this
document's own first commit.

## 7. Cost

Extraction: seconds. The probe: minutes expected, ≤ 85 min bounded,
detached (`setsid nohup`) and polled. Operator attention: one read of
`probe_results.txt` against §2.

## 8. THE RESULT (governed probe, taken under this registration at `194d02d`)

Artifacts (gitignored, rule 8): `artifacts/wp18b_probe_v1_results.txt`
(sha256 `fb43e534…`), `artifacts/wp18b_probe_v1_positions.tsv`,
`artifacts/wp18b_probe_v1_run.log` (`0f74c0c3…`). 85/85 positions answered:
44 `nowin`, 35 `wall-cap`, **6 `win`**. Node costs of the returning solves:
1 to 33,856 (the big NoWin searches), the wins 1-1,599 nodes.

**The six wins:**

| position | mover | nodes | depth_turns | branch-relevant? |
|---|---|---|---|---|
| g001-t44-p2 | sealbot | 86 | 2 | winner, but t44 > collapse bound 42 — NOT A |
| g001-t46-p2 | sealbot | 1 | 1 | winner, but t46 > 42 — NOT A |
| g002-t10-p2 | **pistol** | 1,599 | 5 | NOT the winner — a win the LOSER missed |
| g002-t12-p2 | **pistol** | 397 | 4 | NOT the winner — a win the LOSER missed |
| g002-t39-p1 | sealbot | 714 | 2 | winner, but t39 > collapse bound 37 — NOT A |
| g002-t41-p1 | sealbot | 1 | 1 | winner, but t41 > 37 — NOT A |

**THE BRANCH IS B.** The solver DOES prove the eventual winner's
conversion in both games — but every winner-proof lands exactly two turns
AFTER the registered collapse turn (game 1: proofs at 44/46 vs bound 42;
game 2: proofs at 39/41 vs bound 37). §2's registered rule ("a win proven
only at later turns … is NOT A") reads this as B, and the strict
first-turn-of-tail bound was registered precisely so doubt would break
toward B. **B's consequence is §2's own: the M4 one-free-stone widening —
ONE design round, ONE impl round, both reviewed, gates (a) (b) (d)
re-run green, then this probe re-runs ONCE. Whatever it says the second
time, section 2 proceeds. No second widening.**

**Two diagnostics the branch does not consume, recorded because they are
the WP's motivation measured**: (1) the v0 solver proves PISTOL — the
loser of game 2 — a forced win at turns 10 and 12 (depths 5 and 4), wins
pistol's own 50,176-node search did not find and did not refute; the
solver sees tactical truth the search misses, which is h1's case in one
row. (2) 35 of 85 positions wall-capped at 60 s — the v0 policy's
unbounded NoWin searches are the cost shape the per-call node cap in
section 2's design exists for.

## 9. What this probe is NOT

Not an SPRT, not a strength claim, not a config move, not a re-reading of
D-438. Its output selects the branch; the branch's consequences are the
dispatch's own words in §2.
