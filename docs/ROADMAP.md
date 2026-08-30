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

**WP-1.5d — the stage-Q safety-net candidate cap. CLOSED,
`inconclusive_at_game_cap`, THE GATE STAYS OFF (D-491, D-492).** The cap
(`safety_net_top_k`) truncates the delta-ranked quiet ball on a BATCHED row where
Tier F and Tier T are both empty, at every node except the ROOT TURN's — the
exemption spelled in TURNS, because rule 3 gives turn 1 one stone and every later
turn two, so no ply threshold names the played turn (D-484). The mechanism is
LANDED, gated off in all twelve committed configs, ten mutants dead at named
cases. The governed SPRT at K = 16 over `random_openings_v1` openings
`1500..1999` at 50 000 nodes IS a measurement under Criterion 1'' (**0
unattributable** of 500 pairs, both attribution instruments PASS at exit 0) and
returns `verdict inconclusive_at_game_cap` at the full 1000-game cap:
`llr_pair 1.167` of a required `±2.944`, `nelo_pair +16.9 ci95 21.5`. **NOT
ACCEPTED AND NOT REFUTED** — the point estimate favours the capped seat and the
interval spans zero. **The cap is not inert**: its own registered sensitivity
receipt puts the class on ~20 % of governed-shape searches and a CHANGED PLAYED
TURN on 4.9–5.8 % of all searches, so the finding is that the effect is smaller
than 500 paired openings resolve at `elo1 = 15.0`, not that nothing happened.
`safety_net_top_k` stays `0`. **THE BOOK IS NOW SPENT** — every slice of
`random_openings_v1.txt` is consumed — so a confirmatory run at larger n needs a
regenerated or extended book and its own pre-registration. The D-95 root-turn
package stays LICENSED, NOT SCHEDULED.

**`book_v2` is scheduled into the Stage-3 detector's §0 (D-505):** seeded and
re-executable with the command and seed committed, fresh ranges by construction,
and a size registered with grounds covering the SPRT's worst-case n. The
2000-opening `random_openings_v1.txt` is **RETIRED FOR GOVERNED USE** — still
readable as the artifact that governed the closed runs, but no new
pre-registration may draw a slice from it. **The ±21.5 resolution run stays
licensed-not-scheduled** and, when it runs, runs on `book_v2` under a new
pre-registration, never a re-read.

**STAGE 1 IS CLOSED WITH NAMED RESIDUE (D-503)** — a closure, never a
completeness claim. Carried open and licensed, by name: **WP-1.4** (D-95, the
HeXO forfeit risk), **WP-1.10** (`tools/` gate hardening, whose WP-1.9
precedence D-496 displaced by name). **TWO OF THAT LIST ARE NOW CLOSED BY
WP-1.9b (D-506, D-507)**: **O-3** was implemented, reviewed and measured at
1.518 / 1.594 against the 1.783 / 1.909 comparand — below it in both bands, so
NO FLIP, and D-501's trigger is discharged by measurement rather than left to
silence; and **the module-split cost** is recovered, the storage now living
inline in `handcrafted.rs` at a measured 1.171 / 1.205 inside its registered
[1.10, 1.30] bracket, output byte-identical to the same digest WP-1.9 recorded.
The residue that remains open is **WP-1.4** and **WP-1.10**.

**NEXT SCHEDULED STEP: the Stage-3 scoped detector** (D-471 as amended by
D-494): cheap VCDT/TSS detection gating solver calls, targeting the measured ~6x
call-count cut the WP-1.8c bracket demands, with DBS decomposition only after
the detector earns its own SPRT. The `WP-1.9` designation collision D-493 flagged
is spent: D-473 settled the names and D-494 settled the order.

**Its §0 carries two obligations already scheduled into it, and they are listed
here so the package does not re-derive them:**

1. **`book_v2`** (D-505) — seeded and re-executable with the command and seed
   committed, fresh ranges by construction, size registered with grounds covering
   the SPRT's worst-case n. `random_openings_v1.txt` is spent and retired for
   governed use, so the detector's own SPRT cannot draw from it.
2. **The WP-1.8 nps re-test, discharged into this package** (D-504) — the
   detector's registered bench re-measures the SOLVER SEAT under post-WP-1.9
   (and post-WP-1.9b, which HAS landed — D-507) nps as part of its own bracket. **The
   prereg must state which limb of the WP-1.8 clause it answers** — the nps-jump
   limb is discharged here, the Stage-2-exit limb is not.

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

**WP-1.8b — CLOSED, IMPLEMENTED AND GATED OFF, NO STRENGTH CLAIM (D-441).**
The solver is wired into `pistol-search` behind `[solver] on_search_path`
(false in every committed config, schema v3), the M4 one-free-stone
widening applied first (the anchor probe's branch B: v0 proved the
winners' conversions only two turns after sealbot's own collapse; M4
moves game 1's winner-proof TO the collapse turn itself — a measured
value flip, v0 refutes g001-t42 at 955 nodes where M4 proves it at
10,726, independently re-verified full-width in 37 s). The wiring: root
+ turn-boundary trigger calls (any open four-or-better, by calculus
ID), both directions (the defender entry is a thin wrapper over df-pn's
own to-move dispatch — zero df-pn changes), solver nodes counted against
the shared per-side budget with `search_nodes`/`solver_nodes` as two
independent printed counters, the per-call cap with
spent-means-store-nothing semantics, the root defender proof's Z2 zone
restriction (fail-open), and `SolveOutcome::Unknown` never laundering as
`NoWin`. **The rule-5 bench ABORTED its registered bracket: at cap 16384
the ON seat's nps ratio measured ≤ 0.02 against the ≥ 0.5 bound (OFF
223,668 nps; ON searches 9–240+ s, receipts
`artifacts/wp18b_bench_v1.txt`) — the ON seat is not a candidate for h1
regardless of any SPRT, per the pre-registered clause, and the governed
SPRT was FORECLOSED BY OPERATOR OVERRULE (D-441): no verdict branch
could move the committed config.** The wiring stays exercised in CI by
the determinism gate's fourth seat (its own budgets, cap 512). **The
cost findings are WP-1.8c's inputs**: the AND-side enumeration was
50–100× too slow and got its minimal-cover fast path (with the rule-5
opened-ball subtlety the inline equivalence check caught live); the
OR-side arm-B enumeration (`|R|·|L|` constructions per visit, MEASURED
|R| mean 5.2 / |L| mean 480) is the remaining hotspot; the M4 probe
artifacts (`wp18b_probe_v{1,2}_results.txt`) and the TT knee probe
(1.8a) are the rest of the file. The deep-shape-family debt (a second
family for gate (b)) is unscheduled as before.

**WP-1.8c — CLOSED, BRACKET ABORT, AND THE WP-1.8 ARC CLOSES WITH IT
(docs/decisions.md D-461..D-465).** The registered hotspot was real and the
commissioning dispatch located it one layer too early: arm B's own construction
is 0.15 % of the solve wall, while the `|R|·|L|` count it generates is paid in
`child_keys` — MEASURED 79.6 % of the committed wall — exactly as
`wp18b_m4_design.md` §2 said it would be. Four OUTPUT-IDENTICAL legs (child keys
by zobrist delta; `legal_placements` hoisted out of the cover loop; arm A's
DEF-PLAN filter as a predicate over raisers and live-two windows;
`region_cells` by row sweep) take one df-pn visit from **2,904 to 91.80 µs on
the WP-1.3 corpus and 2,201 to 85.02 on the 85 anchor positions — 31.63× and
25.89×** at IDENTICAL node counts on every rung of a five-revision ladder.
**AND THE BRACKET STILL ABORTS**: corpus bands 0.0809 and 0.0458 against ≥ 0.5,
trigger-rich 0.0488 against ≥ 0.25, IQR gate clean, so no SPRT (D-465). The cap
is not the lever and the WP measured that rather than argued it — 64× less cap
buys 2.3× of ratio. **Three defects inherited from 1.8a/1.8b were found by this
WP's own instruments and fixed here**: the TT replacement law was not
epoch-aware, so a solve's answer depended on the solves before it (D-462); the
node budget was not enforced on the gate-on seat, which spent a measured mean
3× its budget (D-463); and the trigger-rich bench fixture was neither sha-pinned
nor loadable nor as trigger-rich as registered, so half the bracket had never
been runnable (D-464). The gate stays `false` in every committed config; the
solver stays implemented, oracle-gated and determinism-seated. **RE-TEST
SCHEDULED** alongside WP-1.6's quiescence re-test at the Stage-2 exit, and
whenever a measured nps jump lands — the bench re-runs first, new
pre-registration, fresh slice, never a re-read.

**The nps-jump limb of that clause is DISCHARGED into the Stage-3 detector
(D-504).** WP-1.9 landed the jump (1.508 / 1.579, D-502), and the detector's own
registered bench re-measures the solver seat under post-WP-1.9 nps as part of its
bracket — honoured by measurement inside the detector, not skipped, because the
measured path to the 1.8c bracket is call-count reduction, the cap knob is
measured dead, and per-visit cost already took its 30x (D-461). **The Stage-2-exit
limb is untouched**, and so is WP-1.6's own quiescence re-test.

**WP-1.9 — eval window-map storage** (docs/decisions.md D-225, renumbered by
D-249). Replace `pistol-eval`'s `BTreeMap<Window, Counts>` with a selected
storage shape. Its WP-1.10 precedence was displaced BY NAME by D-496.

**WP-1.9 — CLOSED, LANDED, BIT-IDENTICAL, BELOW ITS BRACKET (D-502).** The
window map is a `HashMap<u64, Counts>` over an order-preserving packed key with
a seedless in-crate hasher, selected by MATRIX WP-19-S after two failed
DECISION-RED-TEAM rounds and an architect ruling (D-500, D-501). **Track E, not
SPRT**: byte-identity of search output against the pre-implementation binary
over 44 positions at both determinism budgets, 422 lines and 88 bestmoves
identical, verified twice; identity is the stronger oracle and D-495 says a
strength run adds nothing. The rule-5 bench, registered before the run, lands
**below** its [1.60, 2.10] bracket at **1.508 / 1.579** — above the abort line,
so a FINDING, and **the bracket does not move**. The cause was measured, not
guessed, after two hypotheses failed: the module split costs 0.844 / 0.828, and
inlining it back is a follow-up worth 1.18x-1.21x at the price of one rule 9
justification entry. **BOTH OF THOSE ARE NOW SPENT BY WP-1.9b**: the inline
landing measured 1.171 / 1.205 against the shipped module (D-507), and **O-3 —
the hand-rolled probing table D-225 named — was implemented and MEASURED AT
1.518 / 1.594, which does not flip the selection in either band (D-506).**

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

## Corpus grade (D-453, D-454, D-457)

The human corpus `b2fe61eb…` **cannot be re-acquired**: the scraper that built
it is gone and no endpoint, script or token for it survives anywhere. It is
ARTIFACT-GRADE. That licenses statements *about the artifact* and existence
proofs needing only authenticity — so D-463's zero-duplicates census, D-464's
turn-14 sharing horizon and D-218's radius-6 refutation all stand. It does not
license anything generalising to the platform's players, so **D-434's Stage-2
Texel-style calibration and independent holdout are BLOCKED** until a
POPULATION-GRADE corpus supersedes it. The corpus's own `source_filter` was
audited at the byte level and its two checkable conjuncts hold (D-457); what
that cannot establish is that the stated filter was the *only* filter applied
(D-456), and the platform reporting 101257 finished games against this corpus's
8698 makes that residual larger, not smaller.
