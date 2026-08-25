# SCOPED RE-REVIEW — `docs/experiments/wp18a_design.md` revision 2 (WP-1.8a)

**Revision reviewed:** `34c6838` (HEAD matched; clean tree). Fresh-context reviewer; did not write rev 1 or rev 2.

**Verdict: FAIL** — of the 15 revision-1 findings, 12 CLOSED, 3 PARTIALLY CLOSED; new findings: 1 BLOCKING, 2 MAJOR, 4 MINOR. The paper-faithful EP-1 and the full-width R3' are real fixes, and every quote checked is verbatim; but the rewrite of gate (c) replaced an under-specified gate with a mis-specified one.

## Closure table (revision-1 findings)

| # | Rev-1 finding | Verdict |
|---|---|---|
| 1 | BLOCKING: EP-1/T3-2 narrowed to hot windows | CLOSED |
| 2 | MAJOR: R3' shared defender-side lemmas | CLOSED |
| 3 | MAJOR: threshold loop semantics unpinned | CLOSED |
| 4 | MAJOR: subtraction underflow unargued | CLOSED |
| 5 | MAJOR: gate (c) under-specified | PARTIALLY CLOSED (sub-items addressed on paper; the specified replay is itself defective — BLOCKING A) |
| 6 | MAJOR: gate (d) no termination story | CLOSED |
| 7 | MAJOR: dry run deferred | PARTIALLY CLOSED (see MAJOR C) |
| 8 | MAJOR: matrix marking law; radius 4; attack unrecorded | PARTIALLY CLOSED (residue MINOR D) |
| 9 | MAJOR: reconstruction seam | CLOSED |
| 10-15 | MINORs | CLOSED |

## BLOCKING A — Gate (c)'s P+σ replay specification is unsound as written

(a) The "subset" claim is false: σ stones ARE stones on the board of P+σ; rule 5 makes every cell within hex-distance 8 of a σ stone legal in P+σ. The defender's turn set in P+σ is a **superset**, not a subset. (b) σ-enabled blocking pairs exist and the non-edge rule cannot discharge them: at any t=1 AND node, {h, f'} with f' a σ-enabled cell is blocking but not a tree edge, and a blocking pair leaves no surviving plan, so "resolve as attacker win by plan completion" cannot fire. The gate as specified either false-fails or false-passes. (c) The defender-win check in the replay is unspecified, and σ are defender stones: σ₁ may sit on a k=3 defender active segment (outside Z₁ only), turning it k=2; the defender then completes six during replay; if the walker blindly applies the plan completion after the defender has already made six, gate (c) certifies the relevance property falsely — EXIT-0-WRONG-ANSWER. Fix direction: specify the replay's defender-node contract exactly (apply every legal P+σ turn via pistol-core; defender-win outcome fails the gate; a specified disposition for σ-enabled blocking non-edges — recursive re-verification or a narrowed tolerance class, stated on the gate's face).

## MAJOR B — Gate (b)'s verifier does not concretely establish defender-no-win at expanded AND nodes

Defender-no-win is re-derived only at t≥3 shortcut nodes; at expanded AND nodes the same fact underpins both the tree's validity and the non-edge resolution. State that the walker applies defender turns via core and treats a defender-win `PlyOutcome` as verification failure at EVERY AND node. (MAJOR not BLOCKING because gate (a) backstops on the fixture class.)

## MAJOR C — §10's dry-run discharge pins the stub revision and registers no externally-derived criterion

The stand-in fixture's values are externally known by construction (open five ⇒ Win; no live window ⇒ NoWin), yet the criteria are print-shape only — every one a property the defect class the dry-run rule exists to catch preserves. Register "position 1 prints `Win`, position 2 prints `NoWin`" (and a nonzero digest) as criteria, and run the dry run at a revision where the selftest actually solves.

## MINOR D — Matrix marking-law residue

M1's numeric claims ("distance 4", "reach 8") unmarked; two future measurements carry a present-tense MEASURED tag (§4 seesaw, §7c σ counts).

## MINOR E — The rev-1 review report is not in the tree

The rev-2 header claims it landed at `docs/experiments/wp18a_design_REVIEW.md`; no such file existed. The batch commit is owed before closure.

## MINOR F — Gate (c)'s cost is not priced

Sample sizes registered but no wall-time or operator-attention figure; the ≈2·10⁴ figure assumes tightly clustered positions.

## MINOR G — §2.4's child-filter wording is ambiguous

"Pairs covering all plan-empty cells" admits a false reading (occupying every plan-empty cell) beside the correct one (a cover of the plan family — a hitting set of the plans' empty sets). Say the hitting-set form; one of the two natural readings is the rev-1 BLOCKING defect in miniature.

## Verified sound (new revision-2 material)

- The bounded active-segment scan survives attack: the legal region is monotone along proof paths, so any window intersecting the root legal region — the entire σ sample class — intersects every proof node's region and is scanned at every node.
- v0's no-zone-pruning soundness story is complete without any zone argument (the three defender-side lemmas suffice; verified independently).
- Gate (b)'s verifier mechanism sound in P; "least surviving plan by (axis,start)" well-defined; non-tree ⟹ non-blocking holds in P; an omitted blocking pair fails loudly.
- M5 receipt reproduced: no-edge leg exits 0, control leg exits 1, exactly as §6 records.
- Node accounting, certificate split, M1 knob semantics, gate (d) cap, seesaw counter — judged sound.
