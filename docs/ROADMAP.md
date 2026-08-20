# pistol — Stage Roadmap

Authored in session 1 per CLAUDE.md. Changed only by ADR (one D-line in
docs/decisions.md per change). Stage gates are exit criteria; a stage is not
done until its gate holds. The research report
(docs/research/minimax_report.md) is the technique prior; SPRT is the judge.

## Stage 0 — Foundations (complete; WP-01..WP-07 landed)

Legal, correct, reproducible engine: board + rules + win detection in
pistol-core, perft-verified pair movegen, lazy 128-bit zobrist with phase bit,
handcrafted eval v0, PVS + iterative deepening + TT, line-protocol CLI,
deterministic instrument mode with a two-process CI gate.

Work packages WP-01..WP-07; gates named per WP. Exit: the engine plays legal
games via the line protocol under all three budget kinds, deterministically in
instrument mode, with perft-verified movegen and sane play at depth 4-6 turns.

Exit gate held at WP-07 (docs/decisions.md D-82..D-93). What it rests on: the
perft oracle (`tools/perft_check.sh`), the cross-process determinism gate
(`tools/determinism.sh`), the sha-pinned tactical fixture at its pre-registered
threshold (`tools/tactical_check.sh`), and self-play through the line protocol
with pistol-core as referee. The one exit criterion that is a *measured* answer
rather than a clean one is the depth: D-81's floor puts depth 4-6 turns inside
the 0.5 s deployment budget only at a narrower candidate radius than the one
that ships, and depth 3 at the shipping radius (docs/decisions.md D-89 records
the numbers). That gap is Stage 1's whole argument, and it is a finding rather
than a failure (CLAUDE.md rule 5).

## Stage 1 — Tactical core

Work-package cut of record, in this order (docs/decisions.md D-117, amended by
D-133; a cut that moves is an amendment on those lines, never a silent reorder
here). Each WP gets its own prompt and names its own gates there.

**WP-1.1 — differential search oracle** (D-106). A full-width negamax reference
in the pistol-search test tree: the search's equivalent of the brute-force
movegen oracle (rule 7) and the from-scratch eval (D-68). It goes first,
*ahead of the arena*, because it makes no strength claim and so needs no judge,
while everything after it is a pruning change — and SPRT judges strength, not
soundness, so a pruning commit that quietly changed the value of the tree would
pass one. Two reviewers each built such a reference ad hoc and each found zero
divergences, so this is coverage that persists, not a suspected defect.

**WP-1.2a — opening corpus extraction and fixture curation.** The positions
the arena plays from, and the curation that makes them a corpus rather than a
list: paired-opening selection, balance, and the sha-pinned fixture form the
runner reads. Cut from WP-1.2 because it is data work with no runner in it,
and a work package that is half corpus and half harness lands neither under
review (docs/decisions.md D-133).

**WP-1.2b — minimal pistol-arena.** Paired openings, GSPRT, distinct-game
dedupe, per-side compute accounting. A deviation from the report, which parks
the harness in Stage 5: Hard Rule 6 makes SPRT the judge of every Stage-1
change, so the judge must exist before the first defendant.

**WP-1.3 — calibration experiments.** Every verdict pre-registered before the
first run; no post-hoc threshold moves. (a) Candidate radius r=2 vs r=3,
fixed-node, paired openings — doubles as the arena's shakedown. (b) The
**decided-window floor** (D-105): whether a won position should evaluate at the
band top rather than summing freely. It changes move ordering, so it is a
strength claim and needed the judge. (c) The **flamegraph session** (D-114):
two pre-registered hypotheses, H1 = D-76's eval apply/undo per candidate per
node, H2 = per-node allocation in `candidates.rs`/`ordering.rs`; operator
hardware, release, instrument mode, fixed-node runs at two stone counts. The
profile adjudicates before any fix, and the confirmed hotspot then earns its
own rule-5 bench. `Eval::delta` (D-110) is pre-approved and lands only if H1
survives.

**WP-1.4 — movetime-ceiling fix** (D-95). An interruptible or root-staged first
iteration that can always answer with the best completed root move. On HeXO the
server owns the clock and hard-clamps the call, so this is a forfeit risk rather
than a known limitation. Play mode only; instrument mode is untouched.

**WP-1.5 — threat infrastructure.** Window-count and hot-window tables, the
must-block filter, the unblockable-double-threat check — the mechanisms
docs/research/sealbot_notes.md measures working in another engine for this exact
game. Threat-first staged pair generation with dominance pruning SUPERSEDES the
radius policy as the primary candidate source (radius stays as a
config-selectable fallback). Rules truth stays in pistol-core; threat semantics
live in search/solver (rule 2).

WP-1.5 is cut in two, and the cut is named here because two work packages once
carried one designation (docs/decisions.md D-249): **WP-1.5a** is the
`pistol-solver` threat generator — the crate seam, `ThreatState` and its
queries — and **WP-1.5b** is the threat-first staged pair generation that
consumes it in search, with its own sha-pinned `tactical_staged_v0.txt` fixture.
Neither includes any `pistol-eval` storage refactor; that is WP-1.9.
WP-1.5b's PRE-REGISTERED HOTSPOT is already named and it is not the eval: it is
the cover arithmetic it will call per node, whose growth shape and unmeasured
allocation-per-call are registered in docs/decisions.md D-263 together with what
the measurement owes.

**WP-1.6 — threat-only zone-bounded quiescence**, under D-111's invariant: the
static eval answers at turn boundaries only, so quiescence stands pat and
extends in TURNS, never in plies.

**WP-1.7 — killers/history/countermove on pair moves.** Ordering changes worth
measuring only once the candidate set they order is the threat-first one. Keyed
on the completing stone and on the pair, per the report's move-ordering stack.

**WP-1.8 — AND-OR solver**, upgraded to relevance-zone Deep df-pn (+1+epsilon,
GHI).

**WP-1.9 — eval window-map storage** (docs/decisions.md D-225, renumbered by
D-249). Replace `pistol-eval`'s `BTreeMap<Window, Counts>` with the storage
shape WP-1.5a's matrix selects. LICENSED, NOT SCHEDULED: it owes its own option
matrix, its own pre-registration and its own `tools/bench_delta.sh` run, and it
does not sit on the threat core's critical path. Unlike WP-1.5a its bracket IS a
whole-engine one, because `pistol-eval` is linked by the shipped binary and
`pistol-solver` is not.

**WP-1.10 — `tools/` gate hardening** (docs/decisions.md D-251, D-252). What
the `tools/`-scoped reviews that produced D-250 and D-252 found and neither
closed: test coverage for the rest of what the gate scripts record, the
`command -v` sweep at eight sites across seven files under the reviewer's
fix-across-`tools/` ruling, an amendment to `tools/SHELL_CHECKLIST.md` item 8
for the fourth case bash admits, and two MINORs — a header claiming a sha pin
the script does not enforce, and arguments silently ignored. Added by D-252: an
explicit engine binding for the four operator-run SPRT configs whose `binary` is
still a path literal (`configs/arena_wp13_*.toml`), which no gate runs, which
rule 6 makes the judge of every search and eval change, and whose exposure a
report discloses only to a reader who compares the `binary_sha256` it already
records. LICENSED, NOT SCHEDULED: it carries no engine change and does not sit
on the threat core's critical path. It is a numbered package rather than a note
because these scripts are the instrument every Stage-1 strength claim is read
through, and the defect that opened it was a gate reporting a pass for a binary
that was never built.

Exit: engine refutes the tactical fixture class at pre-registered thresholds;
every landed change SPRT-positive.

## Stage 2 — Cheap learned eval

Rapfi-style incremental pattern-codebook net: 3 directional maps, length-11
axial windows, integer-quantized + SIMD, incremental under 2-stone moves,
distilled from mantis self-play + human corpora. Acceptance bar
(pre-registered): node-matched SPRT >= +150 Elo vs handcrafted_v0; otherwise
handcrafted stays and the eval budget moves to search.

## Stage 3 — Forcing search

Full TSS/DBS with independent-region decomposition; CTSS conservative defense;
RZOP relevance zones wired into online search for VCDT/VCST detection. Every
threat count, zone radius, and win-density number RE-DERIVED for 3 axes —
never imported from square-board Connect6.

Exit: decisive forcing lines of 15-30 turns in sharp positions at negligible
node cost, SPRT-confirmed strength gain.

## Stage 4 — Parallelism + tuning

Lazy SMP (shared TT, staggered depths), ABDADA fallback if efficiency < 0.4 at
16 cores; SPSA/Texel tuning of eval weights and search margins; PROTOTYPE
gauntlet per the report's verdict table (guarded/verified null-move, LMR,
futility/razoring) — each kept only if SPRT-positive. Deterministic instrument
mode stays single-threaded and untouched.

## Stage 5 — Opening book + full harness

Offline JL-PN/SPDFPN solving of symmetry-distinct openings; 12-fold
canonicalized book; balanced-opening generator; pentanomial paired-game
manager; full reporting fields per the report's Deliverable 4 (instrument,
protocol, n, distinct-n, per-side compute, first-player win rate).
