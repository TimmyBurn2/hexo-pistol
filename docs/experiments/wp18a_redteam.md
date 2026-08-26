# WP-1.8a RED-TEAM — the four oracle gates and the solver they adjudicate

**Verdict: YES — a false proof could pass.** Every named route is closed in
code or recorded as a debt below; none is blocking. The verdict and every
finding here are the ones design §9a's RED-TEAM closure section records.

**Provenance (read this first).** This file is a RECONSTRUCTION from §9a of
`docs/experiments/wp18a_design.md`, which is the surviving record of the
red-team dispatch: the dispatching session's history (the verbatim report)
was lost to the session crash, and the closure handoff directed that the
report be written from §9a if unavailable. §9a records the two BLOCKING
routes (B-1 with its second half, B-2 with its construction route) and two
MAJORs (M-1, M-2) with the verdict. The dispatch's own minor findings
(labelled m-1..m-4) and SPECULATIVE items are NOT reconstructable from any
surviving artifact and are deliberately NOT invented here; if one of them
mattered to a conclusion it would be named in §9a, and none is. The
reviewed revision is likewise not recoverable; the tree the review attacked
is the REVIEW-impl fix-round state (post `e668dfa`, at or before `f6fd9d2`).

**Method (as recorded by §9a's receipts).** Fresh context; attacks grounded
in constructed positions run against the solver (the B-1 reproducer below),
in measurement (the R3' overload probe), and in reading the gate code paths
(the laundering mapping, the leaf-only execution argument from the mutant
receipts).

---

## B-1 — the zone tripwire and the laundering path. **REACHABLE; FIXED**

Two halves, one finding class: a legitimate win could be refused, and a
false win could be laundered into a pass.

1. **Order-dependent tripwire.** The zone-containment tripwire ran during
   the witness DFS, checking each node's zone cells against the stones
   accumulated SO FAR in the walk. A node near the root carries zone cells
   propagated from deep descendants that the DFS has not visited yet, so a
   legitimate tree can be refused. Reproducer (a legal position whose
   9,917-node win the solver refused as `NoWinUnderZone`):

   ```
   plies 0,0 1,1 2,1 -1,1 -1,2 0,3 0,4 -2,3 -2,4 2,2 3,2 -2,5 1,3 5,1 5,2 -1,4 -3,3 -6,-1 -6,0 0,1 0,2 -14,-1 -14,0 7,-1 7,1
   ```

   **FIX:** the tripwire now runs AFTER the emission walk completes,
   against the full stone union of the whole tree — order-independent by
   construction — and `emit_node` no longer refuses mid-walk.

2. **`NoWinUnderZone` laundering.** Gate (a) mapped `NoWinUnderZone` to
   `"nowin"`, so a FALSE win whose zone overflowed would print "nowin",
   agree with R3' and the registered expectation on a nowin case, and
   pass. **FIX:** the mapping is the registered §7(a) semantics —
   `NoWinUnderZone` is a MISMATCH that fails the gate.

## B-2 — the bounded differential is leaf-only. **MEASURED; STRENGTHENED**

All 61 bounded cases solve in ONE node, so gates (a), (c) and (d) never
executed a search. The red team's own M-A mutant receipt is the evidence:
under a known zone defect, gates (a), (c) and (d) all PASSED and only
gate (b) died (on the decoys). Strengthened after the review by
measurement: R3' is intractable on ANY position whose solution contains an
AND node — the minimal 15-ply overload shape ran 12+ minutes without
answering. The instrument split is therefore not "bounded vs deep" but
"leaf-only vs everything": gate (a)'s differential can never exercise the
df-pn loop on this game's board size.

**What ships as a result:** gate (b) is the ONLY multi-node instrument (a
genuine full-width re-proof, and the only gate that has ever killed a
mutant); gate (d)'s deep extension was attempted and WITHDRAWN, MEASURED at
closure — no deep case returns at a 32-entry table in bounded time (the 8
original decoys had no verdict in 300 s; decoy-m0 none in 120 s at every
size up to 512 entries, against 0.1 s and ~1 s at the full table), so the
50x node cap was an unreachable detector over a hang; (d) stays bounded
with a named wall watchdog (TT-CROSS-OVERRUN), and its bounded vacuity is
recorded alongside (c)'s; gate (c) remains bounded-only and is TAUTOLOGOUS
on one-node trees (σ cannot touch witness cells that AT-1 puts in every
order) — its live check is the pre-placement defender-six tripwire alone.

**Construction route (deep diversity):** the deep set was one base
geometry with different anchors, so a geometry-specific defect had a free
pass. **FIX:** the set now carries the 180-degree mirror of decoy-0
(634 nodes, 624 seesaw events — a genuinely different search shape because
canonical-first ordering differs under mirroring). Deep NoWin adjudication
remains beyond the v0 instrument complex (R3' intractable; the verifier
re-proves claimed wins only) — licensed-not-scheduled with the M4 widening.

## M-1 — the filler construction. **RECORDED (binds future work)**

Leg (c2) of gate (c) measures P+σ+fillers, and the red team demonstrated
filler-sensitivity on the class gate (c) was excluded from (fillers
rescuing a σ-broken proof). Unobservable on the shipped leaf-only set. The
finding binds the LICENSED deep re-inclusion of gate (c): it must re-argue
the filler policy before its first non-trivial run.

## M-2 — the zone certificate's semantic backstop. **RECORDED (debt)**

With gate (c) tautological on the shipped set, no gate can currently
falsify a consistent-but-wrong zone (the §9a m-2 shared-arithmetic risk:
the verifier shares `ZoneP::add_graded`/`union_with` with the solver). The
value path never consults zones (verified by the red team: `Solver::new`
takes only ε and tt_entries), so no false VALUE proof is reachable through
it — but the zone is the artifact future zone-pruning WPs consume, and its
certificate is unfalsifiable by the v0 gates. That is the concrete debt the
M4/1.8c line inherits.

---

## Closure state

Every REACHABLE route is closed in code (B-1 both halves, B-2's diversity
route) and every measurement-backed limitation is recorded with its owner
(gate (b) carries the multi-node burden; gate (c)'s deep re-inclusion owes
a filler argument; the zone certificate owes an instrument). The mutant
receipts stand: M-A dies at gate (b) on the decoys, M-B/M-D in the lib
suite, M-C at compile (`artifacts/wp18a_mutant_{a,b,c,d}.log`).
