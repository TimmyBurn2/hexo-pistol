# WP-1.5b IMPL — governing prompt

Authored fresh at WP-1.5b's design closure (`docs/decisions.md` D-351), from
`docs/experiments/U1_gate_supersession.md`, `U2_node_protocol.md`, `U3_tier_t.md`
and `U4_soundness_instrument.md` — REFERENCE material under D-351, cited here and
not restated.
`docs/experiments/section_owner_table.md` is the index of which unit owns which
part of the superseded pre-carve design.

---

## 1. Scope

**D-scope is stages F and T only** (`docs/decisions.md` D-310; `docs/ROADMAP.md`
WP-1.5b). Threat-first staged pair generation, `CandidatePolicy::Staged`, the
node protocol (win-now / overload / filtered / batched), and Tier-T
qualification at the threshold reading — built and SPRT-judged against the
committed radius policy.

**NOT in this D-scope, excised rather than cancelled:**

- **Stage Q (the quiet tier) and its widening schedule** — `docs/experiments/WPQ_seed.md`
  is the verbatim excised text, kept as input to the follow-up work package
  `docs/ROADMAP.md` names **WP-1.5c** (D-315). Matrix M2 has never been authored
  in the form its own candidate takes (`WPQ_seed.md`, THE M2 DEBT NOTE) — WP-1.5c
  owes its own design, its own matrix, its own DECISION-RED-TEAM and its own
  SPRT, not a recovery of anything here.
- **Dominance pruning beyond the staged scheme** — deferred, not dropped
  (`docs/ROADMAP.md`, D-313); scheduled with WP-1.5c.
- **The radius-to-staged supersession itself** — completed by the operator's
  SPRT moving the committed config, not by this IMPL (D-314). This IMPL makes
  radius a config-selectable fallback and staged selectable beside it; nothing
  in this document authorizes moving the committed default.
- **Any `pistol-eval` storage refactor** — WP-1.9.

## 2. Build order (binding; review order was free, this is not)

`docs/experiments/U1_gate_supersession.md` §U1-B: this unit's two gates
(`tools/solver_edge_check.sh`, `tools/solver_link_check.sh`) adjudicate an edge
— `pistol-search` depending on `pistol-solver` — that **does not exist until
U2's IMPL creates it**. **U2's IMPL lands before U1's gates are armed.** Arming
first makes CI red on an unchanged workspace; landing U2 without U1 leaves CI
red on a changed one. Neither ordering is a defect in either unit, but this one
is binding.

A second binding clause, promoted here from pointer status
(`U2_node_protocol.md` §U2-Z): **rule 5 is undischarged for the node protocol
itself** — the change that puts `can_win_this_turn` and `blocking_covers` on
every node has no expected-gain bracket, no abort threshold and no IQR-gated
bench of its own, distinct from §4's cover-arithmetic hotspot. This is a rule-5
registration **the architect must place before U2's IMPL starts**, not a repair
IMPL or a carve may write. IMPL does not begin U2's shape until this
registration lands.

## 3. The selected shapes IMPL builds against

Each of the following is a **landed selection**, not a proposal — IMPL builds
the shape as selected, including the debt riding with it. Three carry a landed
ADR line (§3.1 D-329, §3.2 D-323, §3.3 D-316) with their argument at U4's own
§8/§9; §3.4's is **GATED** (`U3_tier_t.md` §U3-Z item 2, not yet written) and
its argument lives at `U3_tier_t.md` §6.1/§6.5.

### 3.1 N-E — the snapshot's `--config` seam (`docs/decisions.md` D-329)

A **required `--config PATH`, no default**, with a new whole-path guard on
`tools/baseline_snapshot.sh`. This is hard rule 1's fourth clause — *"NO
code-side default for any tunable"* — rung (a) was silent across the
three-row field because all three rows already satisfy it (MEASURED: none of
the three has a default, all refusing an absent `--config` by name at exit 1),
so rung (a) does not distinguish N-E; N-E's actual ground is rung (b), at
D-329.

**Four registered conditions ride with the selection and IMPL pays all four**
(D-329):

1. The `config` line's digest is **`$3`, not `$4`** — `config <path> <sha>` is
   three fields; the four-token reasoning belongs to the differently-shaped
   `corpus` line.
2. The whole-path guard **may not be spelled as a reuse of the
   `tools/baseline_snapshot.sh:289` basename loop** — MEASURED twice to leave
   `configs/spaced dir/instrument_v0.toml` reaching the record at exit 0.
3. An **item-10 driving test** is owed for both new refusal classes, in two
   halves with a control (precedent: 91 test lines for one guard arm at
   `b067d47`).
4. An **item-12 sentence** is owed stating a config refusal is a **FAIL**; the
   script currently declares no void class for it.

A fifth, condition-adjacent residual (the relative-`--config`-vs-relative-`--out`
base mismatch, red-team F6) is **already CLOSED, before N-E is built** — R19 at
`63eac4c` gave the script one base for every caller-supplied path
(`U4_soundness_instrument.md` §U4-Z, the N-E conditions bullet). N-E inherits
nothing here.

### 3.2 S-M — the differential gate's instrument (`docs/decisions.md` D-323)

**Per-node EQUALITY** of the emitted set against the **landed** referent **R1**
(`crates/pistol-solver/tests/common/reference.rs`), **reused by a `#[path]`
include and NOT rewritten**. Ships **MARKED `DEPENDS-OPEN-THEORY`** (D-321): the
convention question (`DEF-T`'s minimum-cardinality reading vs. `cover.rs`'s
inclusion-minimal one) is OPEN theory and the calculus is not amended by this
selection.

**Five registered conditions ride with it** (D-323), and IMPL does not
re-litigate any of them:

1. R1 is reused, not rewritten; a second, freshly-written referent for this
   criterion is **forbidden** without a registered agreement criterion and a
   registered consequence for disagreement.
2. The `0 of 3406` **cover-enumeration agreement** figure **may not be cited
   as evidence about the convention** — R1 and `cover.rs` are blind to it
   together.
3. The gate ships marked `DEPENDS-OPEN-THEORY`, at the gate's own text.
4. **S-N is OWED and is a FLIP TRIGGER, not a footnote** — the rules-derived
   survival row the red team found missing. **S-N is NOT implemented by this
   IMPL.** If it is ever stated in a form green on a correct engine and
   affordable at a sampled population, M3 reopens as a two-row comparison
   against S-M under D-323's own flip clause — a later decision, not this one.
5. The registered numbers (D-322, D-323) carry their instrument, committed in
   `docs/experiments/matrix_M3_selection.md`.

**What D-323 explicitly does not decide, and IMPL may not read as decided:**
the **SEAM** by which a test observes the emitted set (D-115's constraint on
widening `pistol_search::staged` to `pub`) is a **separate, still-OPEN named
decision** — carried as debt in §6 below, not resolved here. S-E's second half
(the always-on `assert!` in `visit` for a post-generation drop) is likewise
**neither selected nor rejected** (`U4_soundness_instrument.md` §U4-T).

### 3.3 The four soundness-gate names (`docs/decisions.md` D-316)

The gate has four parts, named rather than lettered, each specified in exactly
one place (§8.3 and §8.2; wired at §8.7).

| Gate | Specified at |
|---|---|
| **THE TACTICAL SUITE UNDER STAGED** | §8.3, first bullet |
| **THE DIFFERENTIAL GATE** — S-M (§3.2 above) | §8.2 |
| **THE COLONY FAMILY** — ≥ 6 built distant-cluster cases | §8.3 |
| **THE PATTERN FIXTURES UNDER STAGED** | §8.3 |

All four become **one script, `tools/staged_soundness_check.sh`**, added to
`tools/ci.sh`. The retired letters `(a)`–`(d)` still resolve through §8.3's
lookup table for any pre-existing citation; nothing new is addressed by letter.
**The differential gate's own script fragment cannot be written until §3.2's
SEAM decision lands** — the other three parts are unaffected and are not
blocked on it. Because §8's finish policy requires all four gates wired and
green with no excusing clause, **IMPL's finish is itself blocked on the SEAM
decision landing** — this is not a defect in either §3.3 or §8, it is the
binding consequence of requiring the fourth gate unconditionally.

### 3.4 C at the threshold reading — Tier-T qualification (`U3_tier_t.md` §6.5)

Config keys `tier_t_own_count = 2`, `tier_t_opponent_count = 3` (the
**threshold** reading — own windows qualify at count ≥ 2, opponent windows at
≥ 3; not the exact reading — `U3_tier_t.md` §6.1). **Pre-registered
consequence, fixed before any gate runs, two branches:**

1. If the soundness instrument (§3.3 above, U4 §8 in full) shows C dropping a
   cell a proven tactic needs, C is replaced by B — strictly wider under the
   threshold reading — as an amendment with its own review, never a silent
   threshold move.
2. If the instrument is GREEN while mutation M7 (`U4_soundness_instrument.md`
   §8.4; Tier T at ≥3 for the mover — option A) also SURVIVES, the instrument
   has demonstrated it cannot tell A from C, C's entire ground is unmeasured,
   and that is recorded as such in the results rather than read as a
   confirmation of C.

Item 2 of `U3_tier_t.md` §U3-Z (the ADR line recording C's strongest surviving
attack) is **gated**: it may not be written until a fresh DECISION-RED-TEAM has
attacked matrix M1 **as amended** (the threshold flip and C's selection under
it post-date the only attack M1 has ever had) — carried as debt in §6 below.

## 4. The pre-registered hotspot — bracket and abort threshold (`docs/decisions.md` D-263)

D-263 pre-registers the **cover arithmetic** (`blocking_covers`,
`min_hitting_set_exceeds`) as WP-1.5b's hotspot, named before the per-node
caller exists, and states plainly that it carries no bracket, no abort
threshold and no bench — *"NONE of those is in this line."* This registration
is unamended and, checked at this revision, unchanged in the tree: no ADR line
records a substitution, `docs/ROADMAP.md`:89–92 still names the cover
arithmetic, `pistol-solver` is not a dependency of `pistol-search`
(`grep -c pistol-solver crates/pistol-search/Cargo.toml` → `0`), and no
`staged.rs` exists in `crates/pistol-search/src/`. **IMPL's registered hotspot
is therefore the cover arithmetic, per D-263, as landed.**

One thing has already landed and IMPL writes its code from: `U2_node_protocol.md`
§5.2 (M5-E), an adopted design decision that deletes the redundant query pair
the cover arithmetic was paying twice, cutting D-263's own ceiling from
**10.51 % to 7.45 %** of a fast node by deleting work rather than accelerating
it — IMPL writes this deletion as part of building the cover arithmetic's
per-node call site; it is not a separate hotspot.

**What D-263 owes and IMPL pays:** measure `blocking_covers` and
`min_hitting_set_exceeds` at the candidate counts its own generator produces,
not at counts chosen to make a curve; try the cheap remedies in D-263's stated
order before the algorithm — the three-pairwise-disjoint-families early-out,
then lifting `empty_families`'s per-call allocation off the hot path, and only
then a different enumeration.

- **HOTSPOT:** the cover arithmetic (`blocking_covers`,
  `min_hitting_set_exceeds`) on the per-node path, per D-263.
- **EXPECTED GAIN BRACKET:** none may be derived before IMPL measures it —
  D-263 registers no bracket. BASELINE = the cover arithmetic as it lands on
  the per-node path, MEASURED first in its own commit; each remedy in the
  stated order is a further commit whose bracket is set from the prior
  measurement before it is written.
- **ABORT THRESHOLD:** D-263 registers none either; IMPL sets one from the
  BASELINE commit's own measurement before writing the first remedy, per rule
  5 — a remedy that misses its own derived bracket, or regresses whole-search
  nps, aborts. A structural floor found this way is a finding, not a failure
  (CLAUDE.md rule 5).
- **INSTRUMENT:** one IQR-gated bench reporting **nps AND time-to-depth**
  (never the snapshot, which reports `depth_turns`/`nodes` only), taken at
  **both stone counts** the arena plays from.
- **ONE CHANGE = ONE COMMIT** (CLAUDE.md rule 5).

**ADR debt, owed and not IMPL's to close silently:** `U3_tier_t.md` §U3-Z item
7 (U3:702–708) is a re-measurement, **MEASURED, OWED, and NOT LANDED** —
*"D-263 named the wrong hotspot: Tier-T cell extraction is MEASURED at about
6× both threat queries combined (533 ns reused-buffer / 662 ns fresh, against
86 ns for the pair), on BATCHED nodes only (the 29.2 % that take a forced row
never extract Tier T at all)."* This document does not decide whether that
re-measurement replaces D-263's hotspot — that is an ADR line the architect
writes, amending D-263 and `docs/ROADMAP.md`:89–92 together (CLAUDE.md rule
10: silent architecture drift is a breach). Until that ADR lands, IMPL benches
the cover arithmetic per D-263 as registered above; if the ADR lands before
IMPL reaches §4's commits, IMPL benches Tier-T cell extraction instead, using
`U3_tier_t.md` §U3-M item 4's own bracket/threshold/instrument text, and this
document is amended to match rather than silently superseded.

## 5. Fixtures and configs IMPL produces

None of the following exists in the tree at this document's revision (MEASURED
absent by `ls`, this commit).

- **Four staged config documents**, `deny_unknown_fields`, no code-side
  default, per `U3_tier_t.md` §10:

  | document | mode | `quiet_radius` | `quiet_top_k` | `widen_schedule` | cut |
  |---|---|---|---|---|---|
  | `configs/instrument_staged_v0.toml` | instrument | 2 | 16 | `[32]` | binds |
  | `configs/tactical_staged_v0.toml` | instrument | 2 | 1024 | `[2048]` | disabled |
  | `configs/gate_staged_v0.toml` | instrument | 1 | 128 | `[256]` | disabled |
  | `configs/play_staged_v0.toml` | play | 3 | 16 | `[32]` | binds |

  `quiet_top_k`/`widen_schedule` are carried in every document because a
  `deny_unknown_fields` document is complete or it is nothing; **whether the
  shipped D-scope surface keeps those two keys at all is OPEN and is the
  architect's** (`U3_tier_t.md` §U3-Z) — carried as debt in §6, not resolved by
  shipping the documents.
- **`crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt`**, sha-pinned once
  landed (CLAUDE.md rule 7, D-209's discipline): twenty cases, fifteen at
  `tactical_staged_v0.toml` and five at `gate_staged_v0.toml`
  (`U4_soundness_instrument.md` §8.3).
- **These four documents are what `docs/experiments/wp15b_sprt_prereg.md`
  §9.4's soundness-gate slot and the snapshot's AFTER measurement need** before
  that document's governed run can be taken (§3.3 above). §9.7 is a separate
  slot — the revision at which `tools/baseline_snapshot.sh` accepts `--config`
  (§3.1 above) — a REVISION, not a config-document need. `wp15b_sprt_prereg.md`
  is its own governed document with its own outstanding review; this prompt
  does not supersede or restate it.
- **`SearchInfo.stages: StageCounters`** — the stage-share counter seam
  (`U2_node_protocol.md` §U2-M item 2): F/T/Q firing rates, the filtered-node
  rate, the `Cover::Impossible` rate, the overload-return rate. Written from the
  same points `nodes`/`nps`/`time_ms` are, on **every** construction path
  including both salvage ones — a counter that silently reads zero on a
  wall-clock path makes the play-mode stage shares unreadable. Stage Q's own
  widening-rate counters DEFER with stage Q (`WPQ_seed.md` §7.2); a later WP
  (WP-1.5c, which owns the deferred quantities) reads them through this same
  seam, so the seam itself is not deferred with them — this seam carries no
  WP-1.6 dependency (`U2_node_protocol.md` §U2-M item 2; no source names one).
  The line protocol's output does not change; the seam is read by a committed
  harness in the `pistol-search` test tree, not printed.

## 6. What is OPEN and this document does not resolve

The argument for each item below is at its own unit's citation, not here.

- **U1** (`U1_gate_supersession.md` §U1-Z): the two clauses of §4.4's surviving
  attack that option (f) does not answer (a legitimate crate added inside the
  cone; a workspace version bump that is not a graph change).
- **U2** (`U2_node_protocol.md` §U2-Z): the two unreconciled M5-E-equivalence
  population figures (168 030 vs. 343 344 comparisons — D-346 restored the
  claim, it did not reconcile the figures). The node-protocol rule-5
  registration is promoted to §2's binding order, not carried here.
- **U3** (`U3_tier_t.md` §U3-Z): the self-completeness-claim architect gap
  (whether D-331 owes a binding clause for a document's universal about its
  own state); MAJOR 12, the unmarked `23.2` in §6.3's failure-mode cell
  (provenance undecided among three candidate cells); the fresh
  DECISION-RED-TEAM against M1 as amended that §3.4's ADR line is gated on;
  the D-scope of `quiet_top_k`/`widen_schedule` (§5 above).
- **U4** (`U4_soundness_instrument.md` §U4-Z): the differential gate's SEAM
  (§3.2/§3.3 above); a fresh-context attack on N-E in its own right (D-333
  rules this does not reopen the selection, but the attack itself remains
  undone); a POSITION for §8.4's M3 witness and a PARENT position for M6's
  second construction; the snapshot's second instrument (replication is
  registered, an independent instrument is not); SHELL_CHECKLIST reviews for
  both `tools/staged_soundness_check.sh` (new) and the reopened
  `tools/baseline_snapshot.sh` (N-E).
- **The hotspot substitution ADR** (§4 above): whether Tier-T cell extraction
  (`U3_tier_t.md` §U3-Z item 7) replaces D-263's cover arithmetic as WP-1.5b's
  registered hotspot is OWED as an ADR line and is not this document's to
  decide.

## 7. NOT IN SCOPE

Carried verbatim from `U2_node_protocol.md` §14: no quiescence (WP-1.6); no
killers/history/countermove (WP-1.7); no df-pn (WP-1.8); no eval terms from `t`
or `τ`; no dominance pruning beyond the staged scheme; no `LEGAL_RADIUS`
change; no ball-scan optimisation; no `pistol-eval` refactor.

## 8. Finish policy

IMPL for stages F and T is finished when:

1. Every test row named in `U2_node_protocol.md` §U2-T, `U3_tier_t.md` §U3-T and
   `U4_soundness_instrument.md` §U4-T passes, behaviour-named with calculus IDs
   in doc comments (CLAUDE.md rule 7).
2. `tools/staged_soundness_check.sh` exists, wires all four §3.3 gates into
   `tools/ci.sh`, is reviewed against `tools/SHELL_CHECKLIST.md` with every item
   answered by name, and is green — the differential gate's own part marked
   `DEPENDS-OPEN-THEORY` rather than silently green on an unresolved
   convention.
3. §4's cover-arithmetic hotspot (or, if the §4 ADR debt has landed by then,
   the substituted Tier-T-extraction hotspot) has run its registration —
   BASELINE measured first in its own commit, each remedy/accessor commit
   bracketed from the prior measurement, in the governing hotspot's own stated
   order — with one IQR-gated bench reporting nps and time-to-depth, and each
   commit stands or is reverted by that verdict alone.
4. `docs/experiments/wp15b_sprt_prereg.md` passes its own outstanding
   fresh-context review at the revision governing its run, and the operator's
   governed SPRT run completes and reports a verdict under that document's own
   §5.
5. U1's gate-supersession commit lands, in the order §2 binds: after U2's
   IMPL, never before.

**Finish does not require:** stage Q or the widening schedule (§1, WP-1.5c);
S-N (§3.2 condition 4, explicitly not this IMPL's to write); or the operator's
SPRT moving the committed config (§1 — a decision made after the run, by the
operator, never a deliverable of IMPL).

**Finish DOES require §3.2/§3.3's differential-gate SEAM decision to land, with
no excusing clause:** item 2 requires all four §3.3 gates wired and green, and
§3.3 states plainly that the differential gate's own script fragment cannot be
written until the SEAM decision lands. If the SEAM is still open, IMPL is not
finished — full stop. The SEAM decision is the architect's to make, and it sits
on IMPL's critical path exactly because item 2 admits no exception for it.

A landing that skips any of 1–5 is not finished; it is OPEN debt, named as such
in §6, never silently dropped.
