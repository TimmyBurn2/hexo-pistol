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

WP-1.5's SUPERSESSION SENTENCE ABOVE IS NOT DISCHARGED BY WP-1.5b, AND THE TWO
HALVES IT UNDERSTATES ARE RECORDED HERE RATHER THAN LEFT TO BE REDISCOVERED.
Both are ADR changes to this file, not editorial notes.

- **The dominance-pruning half is DEFERRED, NOT DROPPED** (docs/decisions.md
  D-313). "Staged pair generation **with dominance pruning** SUPERSEDES the radius
  policy" is one sentence carrying two commitments; WP-1.5b's design ships the
  staged scheme and states that it adds **no dominance pruning beyond** it. The
  pruning half therefore has no owner in WP-1.5a or WP-1.5b and is scheduled with
  WP-1.5c below.
- **The supersession itself is COMPLETED BY THE OPERATOR'S SPRT, NOT BY WP-1.5b**
  (docs/decisions.md D-314). WP-1.5b performs the PARENTHETICAL only — it makes
  radius a config-selectable fallback and makes staged generation selectable
  beside it. What makes staged the PRIMARY candidate source is the operator's
  SPRT moving the committed config, and until that run clears its registered
  bound the committed engine is still the radius one. D-204's flip clause fires
  there, not here.

**WP-1.5b IS CLOSED, SPRT-POSITIVE — docs/decisions.md D-386.** The operator's
SPRT (two disjoint-book runs, both node-matched, both crossing `verdict h1`
below the pre-registered 100-pair floor individually, confirmed together per
prereg §5's own D-190-precedented procedure) fired the supersession this
section names above: `configs/instrument_v0.toml`'s committed candidate
policy moved from radius 2 to staged in the same commit. D-204's flip has
fired; its tactical-threshold re-derivation remains its own unscheduled
work, per D-204's own text — not inherited as settled by this closure.
**WP-1.6 is UNBLOCKED**: the stage-share counter seam it reads already
exists (`crates/pistol-search/src/info.rs`, `pvs.rs`, `staged.rs`), landed
with WP-1.5b's own IMPL and untouched by this closure.

**WP-1.5c — the quiet stage, its widening schedule, and dominance pruning**
(docs/decisions.md D-315; the designation is the one §15 item 8 already cites for
the Tier-Q ball scan). D-310 cut WP-1.5b to stages F and T; the QUIET stage
(Tier Q) and the widening schedule that batches it were excised, not cancelled,
and `docs/experiments/WPQ_seed.md` is the verbatim excised text kept as this
package's input. What it owes, none of it inherited as settled: its own design,
its own option matrix over the widening schedule — matrix **M2** has never been
authored in the form its adopted candidate W-E takes, so it is a fresh matrix and
not a recovery — a fresh-context DECISION-RED-TEAM before any option is selected,
and its own SPRT. It also carries the dominance-pruning half of WP-1.5's
supersession and the Tier-Q ball scan. THE STRENGTH DEBT IS THE REASON THIS IS
SCHEDULED AND NOT MERELY LICENSED: WP-1.5b's SPRT delta shrinks by exactly the
axis D-310 removed, and this is the only package where that delta can be
recovered — D-310 flips if this WP is never scheduled. PRIORITY: after WP-1.5b,
and it does not block WP-1.6.

**WP-1.6 — threat-only zone-bounded quiescence**, under D-111's invariant: the
static eval answers at turn boundaries only, so quiescence stands pat and
extends in TURNS, never in plies.

**WP-1.6 — CLOSED, `h0`, PREMATURE AT EVAL v0 (D-428).** The governed SPRT
run was taken at `a4d5fbb` and IS a measurement under Criterion 1'' (0
unattributable pairs of 225). `defensive_only` quiescence is NOT accepted
against the committed staged policy at 50 000 nodes on `random_openings_v1`:
`verdict h0`, 225 pairs, `llr_pair -2.9787`. The extension stays GATED OFF
(`q_depth_turns = 0`) and the committed config did not move. **This is a
planning finding about EVAL v0, not about quiescence as a class** — the
design's claim is that quiescence corrects a horizon the STATIC eval
misreads, so **the Stage-2 re-test STAYS SCHEDULED** below and
`defensive_and_offensive` remains licensed-not-scheduled (D-396).

**WP-1.7 — killers/history/countermove on pair moves.** Ordering changes worth
measuring only once the candidate set they order is the threat-first one. Keyed
on the completing stone and on the pair, per the report's move-ordering stack.

**WP-1.7 — CLOSED, `h0` (D-433).** The three heuristics are implemented,
gated OFF by default in every committed config, and measured: the governed
SPRT run (fresh slice `1000..1499`, 341 pairs, `llr_pair -2.9911` against
the `-2.9444` `h0` boundary, Criterion 1'' exit 0 with 0 unattributable
pairs, the second instrument's agreement criterion holding on both clauses)
did not accept them at 50 000 nodes on `random_openings_v1`. A measured
finding, not a threshold move: the chess-lineage prior did not survive a
threat-first ordering that already captures most of these heuristics'
value — which the design and the prereg both registered as the likely
outcome before game one. The rule-5 bench's ordering-QUALITY signal (nps
ratio 1.06/1.06, one position gaining a completed depth) stands as recorded
context (D-431); the licensed-not-scheduled relaxations (top-K promotions,
depth-scaled bonuses, play-order pair keying) are future pre-registrations,
not inherited business.

**WP-1.8 — AND-OR solver**, upgraded to relevance-zone Deep df-pn (+1+epsilon,
GHI).

**WP-1.8a — CLOSED, CORRECTNESS LANDED, NO STRENGTH CLAIM (D-437).** The
policy-game df-pn solver lives in `pistol-solver` (Pawlewicz-Lew thresholds
plus 1+epsilon, paper-faithful EP-1 zone sequence, 128-bit-keyed
epoch-stamped TT with the proven-retention law, proof-DAG witness emission)
and is adjudicated by four oracle gates through `tools/solver_oracle_check.sh`
(CI gate 12/18), ALL GREEN: (a) 61 bounded cases agree with R3'; (b) 38
proof trees re-verified full-width, the ONLY multi-node instrument, R3' being
MEASURED intractable on any position whose solution contains an AND node;
(c) 29 wins, 118,135 sigma placements revalued, 26,865 refused on collision;
(d) TT values agree at both table sizes, bounded-only after the deep extension was MEASURED infeasible (no deep case returns at a 32-entry table in bounded time; §9a records the withdrawal and the knee probe, WP-1.8c's input). The determinism
seat is byte-identical (CI gate 13/18). Four mutants dead: M-A at gate (b)
on the decoys, M-B/M-D in the lib suite, M-C at compile. The RED-TEAM
verdict was YES a false proof could pass, and every named route is closed
in code or recorded in design §9a: the order-dependent zone tripwire
(FIXED, now runs after the walk against the full stone union), the
NoWinUnderZone laundering path (FIXED, gate-(a) mismatch semantics), the
leaf-only bounded set (STRENGTHENED FINDING: gates (a), (c) and (d) never
executed a search on it; only (b) has ever killed a mutant), deep diversity
(the mirrored decoy, 634 nodes / 624 seesaw). GHI machinery was skipped
outright (D-436: the game is monotone, the state graph a DAG). The solver
is NOT on the search path; WP-1.8b wires it. Open debts are named in §9a
(the zone certificate unfalsifiable by v0 gates; the filler policy binds
gate (c)'s licensed deep re-inclusion; deep NoWin adjudication beyond the
v0 instruments).

**WP-1.8b — wire the solver into pistol-search.** The WP-1.8a solver is not
on the search path; this package wires it. Its first debt is a second shape
family for gate (b)'s deep set (one base geometry plus its mirror is not
diversity, the red team's finding).

**WP-1.9 — eval window-map storage** (docs/decisions.md D-225, renumbered by
D-249). Replace `pistol-eval`'s `BTreeMap<Window, Counts>` with the storage
shape WP-1.5a's matrix selects. LICENSED, NOT SCHEDULED: it owes its own option
matrix, its own pre-registration and its own `tools/bench_delta.sh` run, and it
does not sit on the threat core's critical path. PRIORITY: after WP-1.5b and
after WP-1.10 (D-289), because its recorded stake is a table-only reading that
D-258 forbids quoting as a whole-engine bracket. Unlike WP-1.5a its bracket IS a
whole-engine one, because `pistol-eval` is linked by the shipped binary and
`pistol-solver` is not.

**WP-1.10 — `tools/` gate hardening** (docs/decisions.md D-251, D-252, and
D-289 for each item's owner). What the `tools/`-scoped reviews that produced
D-250 and D-252 found and neither closed: test coverage for the rest of what the
gate scripts record — still FIVE undriven gate scripts (`config_check.sh`,
`determinism.sh`, `movetime_check.sh`, `perft_check.sh`,
`search_oracle_check.sh`), since the two landing with D-284 and D-285 are new
scripts that carry their own suites and remove nothing from that list; the `command -v` sweep, RE-COUNTED at
this revision to SIXTEEN sites across ELEVEN files (D-251's eight-across-seven
predates two scripts that landed with D-276, and four more sites land now,
knowingly carrying the terse idiom so the sweep's own enumeration stays right); an amendment to `tools/SHELL_CHECKLIST.md`
item 8 for the fourth case bash admits; two MINORs — a header claiming a sha pin
the script does not enforce, and arguments silently ignored by four scripts (not
by the two landing now, which refuse them by name with a test); and, added by
D-285, the SEVEN `mktemp -d` scripts that do not preflight their scratch space
and are covered under CI only because `tools/ci.sh` preflights once before all
of them.
D-252's fifth item is SPENT: the explicit engine binding for the four
operator-run SPRT configs landed at D-283, and it is struck from this list
rather than carried, because a WP that ships an item already fixed is a list
that outlives its subject. LICENSED, NOT SCHEDULED: it carries no engine change
and does not sit on the threat core's critical path. PRIORITY: after WP-1.5b,
and BEFORE WP-1.9 — these scripts are the instrument every Stage-1 strength
claim is read through, and the defect that opened the package was a gate
reporting a pass for a binary that was never built.

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
