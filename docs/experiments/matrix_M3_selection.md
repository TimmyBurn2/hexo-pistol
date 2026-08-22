# MATRIX M3 — SELECTION RECORD: **S-M is selected**, on a field its own red team found incomplete

**Selected at `809b5db`** by the architect-delegated session, from
`docs/experiments/matrix_M3_soundness_instrument_rev2.md` (revision 2, authored
`d48824f`) after the fresh-context DECISION-RED-TEAM at
`docs/experiments/matrix_M3_REDTEAM_round2.md`. Owning unit:
`docs/experiments/U4_soundness_instrument.md` §8. ADR line: **D-323**.

> **NEITHER THE MATRIX NOR THE RED TEAM IS EDITED BY THIS RECORD.** The matrix's
> recommendation — S-K — is DEAD, killed by its own attack, and this record
> selects a different row. That is what a red team is for and it is stated here
> rather than smoothed away.

## WHY THIS SELECTS RATHER THAN STOPS, when the field is incomplete

D-318 stopped M4 partly because *"the field is still incomplete at the second
revision, and one missing row DOMINATES the recommended option on the matrix's
own trigger."* The red team finds this field incomplete too (F5, the rules-derived
row **S-N**), so the question is whether the same disposition follows. **It does
not, and the difference is dominance.**

- In M4 the missing closed-enum row satisfied the recommendation's own grounds
  *identically while owing fewer guards* — it dominated N-E outright.
- Here **S-N dominates the row that is already dead** (S-K: S-N kills M4, S-K does
  not) and does **not** dominate the survivor. Against S-M it is not costed, its
  criterion is **not stated** — the red team says so in its own words, *"I state
  the criterion's exact wording as owed to the authoring round and not supplied
  here"* — and its naive form is **RED on a correct engine**, because a non-losing
  pair may carry a free second stone outside any cover. Its measured cost is
  **1.76 M legal turns per FILTERED node, mean; 2.61 M max**, which makes it a
  sampled or fixture-bound row at best.

A row that is unstated, unpriced against the survivor, and red in its only written
form does not block a selection; it is **registered as owed**, and condition 4
below makes it a flip trigger rather than a footnote. The alternative — a third
authoring round — buys a comparison against a criterion nobody has yet been able
to write, while the differential gate stays unspecified for a third round and
U4-Z item 4 and B1 stay blocked.

## WHAT IS SELECTED

**S-M — per-node EQUALITY of the emitted set against the LANDED referent R1**
(`crates/pistol-solver/tests/common/reference.rs`), at every FILTERED node of the
gate corpus, with the referent REUSED rather than rewritten.

## THE STRONGEST ATTACK SURVIVING AGAINST S-M — the red team's own words, which D-323 quotes

> S-M asserts the convention **D-321** records as OPEN: if the project settles
> toward `DEF-T`'s minimum-cardinality reading, the gate turns RED on a correct
> engine, and its referent cannot warn of it — R1 is independent of `cover.rs`'s
> CODE and shares its CONVENTION, so fact 7's `0 of 3406` agreement is invariant
> under precisely the defect in question, which is CLAUDE.md's
> two-instruments-blind-to-the-same-stage clause read from inside. What survives
> the attack is that its advantage over the immune-marked S-K is **not** confined
> to that open question: MEASURED, S-M kills both registered S-E-class mutations —
> M4 on its own pinned witness, and M3 on 20 of 20 differing nodes of the
> registered playout regime — where S-K kills neither, so R8 does not decide
> between them.

## WHY S-M AND NOT THE MATRIX'S OWN RECOMMENDATION

Three of the red team's findings, each independently sufficient, and none of them
a non-reproducing cell — **all eight facts reproduced**:

1. **F2, the decisive one.** S-K fires on NEITHER registered mutation of the class
   it instruments. U4 §8.4 registers exactly two S-E-class mutations: M3 (phase
   regeneration) and M4 (minimum-cardinality covers). MEASURED by the red team:
   M4 survives S-K on M4's own pinned witness and dies to S-M; M3 survives S-K on
   **0 of 20** differing nodes and dies to S-M on **20 of 20**. An instrument for
   the differential gate that fires on zero of the differential gate's registered
   mutations is the vacuity that killed S-C, one round later.
2. **F1.** The matrix scored both criteria on D-124's `pop()` mutant, which U4
   §8.4 registers as **M8**, class *the `assert!`* — and U4 §8.2 states that a drop
   after generation *"no test of the generator can see"*. S-K's own cost ground
   (no seam inside `pvs`) is what makes it blind to that mutant, so its measured
   72 % against M8 is 0 % in the shape the ground pays for.
3. **F4.** S-K's IMMUNE mark rests on a step its own failure-mode cell concedes is
   not entailed — `DEF-T` fixes a NUMBER, S-K asserts a SET — and the scenario the
   cell calls "smaller" is MEASURED live on **116 of 174 (66.7 %)** of the
   protocol's own FILTERED population. S-K is DEPENDS-OPEN-THEORY on a second
   convention question, not immune.

Under R8 an option depending on the open theory cannot beat an immune option **on
that ground**. S-M's advantage over S-K is measured to lie on a *different*
ground — M3's phase-regeneration class — so R8 does not exclude it, and with S-K
and S-L fallen S-M is the only row standing.

## FIVE REGISTERED CONDITIONS, each a red-team finding that would otherwise ride free

1. **THE REFERENT IS REUSED, NOT REWRITTEN.** The IMPL takes R1 by `#[path]`
   include (MEASURED, matrix fact 6: it compiles and answers from
   `pistol-search`'s test tree for one include line plus a dev-dependency the WP
   lands anyway, at the cost of three `dead_code` warnings). **A second,
   freshly-written referent for this criterion is FORBIDDEN without a registered
   agreement criterion and a registered consequence for disagreement**, per
   CLAUDE.md's second-instrument clause — writing one would create two instruments
   for arithmetic one already covers.
2. **`0 of 3406` MAY NOT BE CITED AS EVIDENCE ABOUT THE CONVENTION.** R1 and
   `cover.rs` are blind to it together; their agreement is invariant under the
   defect in question. It is evidence about the arithmetic and about nothing else,
   and any document that cites it otherwise is repeating the error this condition
   exists to name.
3. **THE GATE SHIPS MARKED DEPENDS-OPEN-THEORY (D-321).** U4 §8.2's text for this
   gate carries the mark and the flip clause below; a reader must not find a gate
   whose criterion looks settled.
4. **S-N IS OWED AND IS A FLIP TRIGGER, not a footnote.** If the rules-derived
   survival criterion is ever stated in a form that is GREEN on a correct engine
   and affordable at a sampled population, M3 reopens as a two-row comparison
   between it and S-M — because it would be immune to R8 *by construction* rather
   than by taking a side, and it kills M4.
5. **THE REGISTERED NUMBERS GET AN INSTRUMENT WITH A REVISION.** Red-team F7 is
   accepted: the census probe existed only as prose. Its phase-correct successor is
   reproduced in full below, so every figure D-322 and D-323 carry has a named,
   retrievable source at a named SHA.

## WHAT THIS SELECTION DOES NOT DECIDE

- **The seam by which a test observes the emitted set.** Every row needed one;
  D-115's constraint on widening `pistol_search::staged` (round-1 F4) applies to
  S-M as it applied to S-E, and it is a separate named decision.
- **The gate's corpus and its per-CI cost.** The 40-90 s figure is CARRIED from
  U4-M's cost table and is not this round's measurement.
- **The convention.** D-321 stands; `docs/research/threat_calculus_v1.md` is not
  amended by this selection.

## THE INSTRUMENT, VERBATIM — the probe that produces D-322's figures

Run in a throwaway worktree at `/home/tom/.cache/m3-author` over the tree at
`d48824f` (crate sources byte-identical to `809b5db`:
`git diff --stat d48824f 809b5db -- crates/` is empty), as
`crates/pistol-solver/tests/zz_m3_phase.rs`. It is not committed to the tree
because it is not a gate; it is committed HERE, which is what gives it a
revision.

```rust
//! Throwaway: the U2 §5.3 FILTERED row, exactly as the protocol states it.
mod common;
use common::playouts::{Rng, random_ply};
use pistol_core::{Coord, GameState};
use pistol_solver::{Cover, HitBudget, MinimalCover, StonesLeft, ThreatState};

fn union_all(c: &[MinimalCover]) -> Vec<Coord> {
    let mut v: Vec<_> = c.iter().flat_map(|k| k.cells()).collect();
    v.sort_unstable(); v.dedup(); v
}
fn union_min(c: &[MinimalCover]) -> Vec<Coord> {
    let m = c.iter().map(|k| k.cells().len()).min().unwrap();
    let mut v: Vec<_> = c.iter().filter(|k| k.cells().len() == m).flat_map(|k| k.cells()).collect();
    v.sort_unstable(); v.dedup(); v
}

#[test]
fn the_protocol_row_as_u2_states_it() {
    let (mut mover_positions, mut filtered, mut differ) = (0usize, 0usize, 0usize);
    let (mut sk, mut sm, mut n) = (0usize, 0usize, 0usize);
    for seed in 1..=12u64 {
        let mut rng = Rng::new(seed);
        let mut game = GameState::new_game();
        let mut threats = ThreatState::new();
        while game.board().stone_count() < 150 && !game.outcome().is_decided() {
            let next = random_ply(game.board(), &mut rng);
            let mover = game.to_move();
            game.place(next).expect("legal");
            threats.apply(next, mover);
            if game.outcome().is_decided() { break; }
            // PROTO-NODE, at the node the search would visit next.
            let us = game.to_move();
            let Some(left) = StonesLeft::from_state(&game) else { continue };
            mover_positions += 1;
            if threats.can_win_this_turn(us, left).is_some() { continue }
            let Cover::Minimal(covers) = threats.blocking_covers(us, HitBudget::from(left)) else { continue };
            filtered += 1;
            let incl = union_all(&covers);
            let min = union_min(&covers);
            if incl != min { differ += 1; }
            if incl.len() > 1 {
                n += 1;
                let mut mutant = incl.clone();
                let dropped = mutant.pop().unwrap();
                if min.contains(&dropped) { sk += 1; }
                if mutant != incl { sm += 1; }
            }
        }
    }
    println!("mover positions (PROTO-NODE evaluated) = {mover_positions}");
    println!("FILTERED (U2 5.3, mover only, phase-derived left/budget) = {filtered}");
    println!("conventions DIFFER = {differ}  ({:.1} % of FILTERED)", 100.0*differ as f64/filtered as f64);
    println!("pop-mutant applicable = {n}; S-K fires {sk} ({:.1} %), S-M fires {sm} ({:.1} %)",
        100.0*sk as f64/n as f64, 100.0*sm as f64/n as f64);
}
```

Its output, verbatim:

```
$ cargo test -p pistol-solver --test zz_m3_phase -- --nocapture
mover positions (PROTO-NODE evaluated) = 1700
FILTERED (U2 5.3, mover only, phase-derived left/budget) = 174
conventions DIFFER = 22  (12.6 % of FILTERED)
pop-mutant applicable = 154; S-K fires 132 (85.7 %), S-M fires 154 (100.0 %)
test the_protocol_row_as_u2_states_it ... ok
```

**These figures were produced by the selecting session and they agree with the
red team's independently written probe to the digit** — 174, 22, 12.6 %, 132 of
154, 85.7 %, 154 of 154. That agreement is between two instruments that do NOT
share a stage: two probes written from the protocol text by sessions that did not
see each other's source. It is not the `0 of 3406` kind of agreement, and
condition 2 does not apply to it.

---

*Selection record for MATRIX M3, at `809b5db`. The matrix's recommendation fell;
the surviving row is selected with five registered conditions and one flip
trigger. D-323 carries the line; D-322 carries the correction the round owed to
D-321.*
