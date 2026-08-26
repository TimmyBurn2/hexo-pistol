# Sealbot anchor match — pre-registration

**What this is**: ONE anchor match, pistol (the WP-1.7-closure engine) vs
sealbot, on the local HeXO match platform (`tools/sealbot/`). Its purpose is
to record where pistol stands against the one external engine we have, with a
transcript that can be replayed. **What this is NOT**: not SPRT, not paired,
not an Elo claim, not a strength claim of any kind beyond "this is what
happened in these N games". Sealbot is UNVERIFIED (docs/research/
sealbot_notes.md, D-197): beating it licenses the claim "beats sealbot" and
nothing more, and the standing judgment — bot far below strong humans until
measured against them — is unchanged by anything here.

## 1. The registered numbers (fixed before game one)

| What | Value |
|---|---|
| Games | **N = 40** (seats alternate; engine A = pistol is p1 in odd games, p2 in even — 20 per seat) |
| Opening policy | **The platform's standard setup, every game**: the server (matchserver) auto-plays p1's turn-1 stone at the origin (0,0), exactly as the HeXO server's htttx `setup` packet delivers; engines are first asked at turn 2 |
| pistol engine | binary built at `e2280ca` in a dedicated worktree, sha256 `665d2815ddba28e7889ebea661a10b15352036ab46bfc6f1758d72813cad4184` (= D-433's pinned digest, reproduced twice in two directories; see tools/sealbot/README.md "The engine pin" for why HEAD's build differs) |
| pistol config | `configs/instrument_staged_v0.toml` (mode instrument, 1 thread, lexicographic tie-break), run with cwd = the engine worktree |
| pistol budget | `go nodes 50000` — the same instrument budget WP-1.6/1.7 ran (D-186, D-432) |
| pistol process | one per game; wall cap 120 s per answer |
| sealbot engine | `current/` at the recorded local path, through `tools/sealbot/sealbot_shim.py` |
| sealbot budget | `time_limit = 0.3` s per turn (the value hexo-bridge's own sealbot example runs); wall cap 5 s per answer |
| Turn cap | **60** (evaluation horizon; a game with no decision is "capped", never a win) |

## 2. What counts as what

- **Decided**: a game ending in six-or-more-in-a-line. These are the ONLY
  games in the interval's sample.
- **Capped**: no decision within 60 turns. Reported separately, excluded from
  the interval.
- **Forfeit**: illegal move (a stone the rules refuse, or a turn that stops
  short of the stones owed), engine failure, or timeout. Reported separately,
  excluded from the interval; a win that arrives by the opponent's forfeit is
  tallied as `win_by_opponent_forfeit`, never as a decided win.
- **Interval**: Wilson 95% over pistol's share of DECIDED games. No paired
  statistic, no Elo conversion, no SPRT — the seats are reported separately
  precisely because nothing here pairs them.
- **Compute**: pistol's node total (from the engine's own `totals` lines) and
  both sides' wall time, per the transcripts.

## 3. The instruments, at their revisions

| Instrument | Revision | Digest/identity |
|---|---|---|
| The match platform (`tools/sealbot/`, whole tree) | `37cdf81` on branch `sealbot-anchor` (the review fix rounds: F1–F7/N4/N5 at `f254e33`, then R1 — replay_check's illegal branch mirrors the referee's first-failing-index walk — with the mixed-class suite cases) | the adapter + matchserver + shim + tests + this document's tree |
| `run_match.sh` | same commit | drives the match; builds with `--locked` |
| `replay_check` (second instrument) | same commit | replays transcripts against pistol-core |
| The run config | `local/sealbot_anchor.toml` | sha256 `773787cf7f1bde2f2677ffcdba151f8eb5acc5fc3ba2b27e8de068f8591f1dfa` |
| pistol binary | `e2280ca` | `665d2815…` (§1) |
| sealbot | local tree, unversioned | recorded as the shim's argv in the config; UNVERIFIED by design |

A change to any of these before the run reopens this pre-registration,
however small the diff.

## 4. The registered commands

Run from the repository root (the branch carrying this document):

```
tools/sealbot/run_match.sh local/sealbot_anchor.toml
tools/sealbot/matchserver/target/release/replay_check artifacts/sealbot_anchor_v1
sha256sum artifacts/sealbot_anchor_v1/report.json \
         artifacts/sealbot_anchor_v1/report.txt \
         artifacts/sealbot_anchor_v1/g*.jsonl
```

## 5. The dry run (taken before this registration; same kind, not the sample)

Input: `local/sealbot_dryrun.toml` (sha256 `4938e6849cb43eeac0222087d59547458535a6f39654844490df4a0a731f3d6a`) — the same two real engines, the same
seats rule, at reduced budgets (pistol `nodes 5000`, sealbot `0.05 s`,
cap 40, **2 games**). Output: `artifacts/sealbot_dryrun_v1/`
(report `fc5a23d345b7fab86a6d46585c2b31bde3b46c1c2d0f68fe2dfc393e2ea4f67f`,
transcripts `759fb084…` and `a5ac12e9…`). This is the instance the FIXED
instrument tree (`37cdf81`, the R1 round) produced. Its observations, from
the digested bytes and nowhere else: **game 1 — win, winner p2 at turn 28;
game 2 — win, winner p1 at turn 15; zero forfeits; nodes_total 89829, equal
to the transcript sum 89829.** (The same numbers as the two prior instances
— pistol is node-budgeted and deterministic, and sealbot at 0.05 s has
repeated — which is a comfort and not a claim: the digests pin THIS
instance.)

**Criteria, with the defect class each exists to exclude:**

- **A. Both games ran without a forfeit** (game 1: win, p2 at turn 28;
  game 2: win, p1 at turn 15 — the digested instance's own outcomes, read
  from its bytes).
  *Defect class: a driver-protocol break masquerading as a game
  outcome — a seat that cannot speak its protocol loses by forfeit and the
  anchor would silently measure plumbing.* A forfeit here fails this
  criterion.
- **B. `replay_check` exits 0 on the dry-run transcripts.** *Defect class:
  the written record is not the game that was played — stone order, turn
  boundaries, or win bookkeeping corrupted in the transcript path.* Any
  disagreement fails this criterion.
- **C. The report's `nodes_total` equals the sum of the per-turn `nodes` in
  the transcripts** (verified: 89829 == 89829). *Defect class: compute
  misattribution — per-side compute is a reporting requirement (CLAUDE.md
  rule 6) and a driver that bills the wrong seat's nodes misreports it.*
- **D. The stub suite passes**, including all three tampered-record negative
  controls (winner flip, extra legal stone, mover relabel). *Defect class:
  an instrument that cannot say no.*

## 6. The governed run's agreement criterion, and its consequence

`replay_check` over all 40 transcripts must print
`40 transcript(s) replayed to their recorded outcomes` and exit 0. The two
instruments share pistol-core by design (the rules are not the stage under
doubt); the stage under doubt is the RECORD, which only the re-read
exercises. **Registered consequence of disagreement**: the run is NOT a
measurement. The transcripts stand as raw material, the anchor verdict is
withheld, and the platform is fixed and the match re-run as a NEW
pre-registration on a fresh output directory. No post-hoc repair of the
record, no partial reading.

## 7. Cost

MEASURED dry-run scale-up: 2 games at 1/10 budgets took 16 s wall. The
governed run at full budget: games run longer (deeper answers, more turns);
ESTIMATE 10–20 min wall on this machine, operator attention ~5 min to launch
and read. **Abort bound**: if the run exceeds 60 min wall it is stopped and
recorded as such (a cost anomaly, not a verdict).

## 8. What flips or reopens this

- Any change to an instrument revision (§3) before the run.
- The operator's overrule of N, the cap, or a budget — each is an amendment
  and a fresh review of THIS document at its new revision.
- Nothing about the outcome reopens it: a 40–0 sealbot sweep and a 40–0
  pistol sweep are equally valid anchors, and both leave the standing
  judgment (below strong humans) exactly where it is.
