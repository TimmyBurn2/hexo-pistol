# ROUND-4 VERIFICATION — `docs/experiments/wp18a_design.md` revision 4 fix round (WP-1.8a)

**Diff reviewed:** `git diff 6dcd3ae..ec05cac -- docs/experiments/wp18a_design.md` (rev 3 → rev 4 + the cap-consistency fix at `66af539`). HEAD matched.

**Verdict: PASS.**

## Round-3 findings

- **N-1: CLOSED** — the fix is in the OPERATIVE registered sentence (§7c: "all σ cells lie in the EMPTY cells of `P`'s legal region (`legal_placements(P)`, region less occupied cells …)"), not merely a gloss.
- **N-2: CLOSED** — arithmetic verified independently. Empties of a k-empty defender active segment lie in `Z_min(k,3)` (EP-1 grading, order-3 cap); `σ_i ∉ Z_i`; `Z_1 ⊆ Z_2 ⊆ Z_3`. k=1: empties ∈ Z_1, σ_1 ∉ Z_1 → 0 of 1 ✓. k=2: empties ∈ Z_2, only σ_1 may enter → 1 of 2 ✓. k=3: empties ∈ Z_3, σ_1/σ_2 may enter, σ_3 may not → 2 of 3 ✓. k≥4: |σ| ≤ 3 < k (and in fact ≤ 2, since σ_3 ∉ Z_3) → ≤ k−1 ✓. "No k is ever filled" follows. The premise holds: any window σ could complete is attacker-free in P (hence an active segment), intersects P's legal region (it contains σ cells), so it is scanned, and EP-1 applies at every proof node including the root.
- **N-3: CLOSED** — tuple order pinned (ascending by (q,r), i-th stone = i-th element, lexicographic enumeration of ascending tuples); both caps now say "all such tuples, capped at the first 5 000 / 2 000".
- **N-4: CLOSED** — ≈7 300 = ~300 + 5 000 + 2 000 ✓; 7 300 × 20 = 146 000 ≥ ~145 000 ✓; marked ESTIMATED; the dry-run extrapolation weakness is stated (stand-in measures the near-best-case σ-solve; depth-3-4 driver measured by the gate's first run).
- **N-5: CLOSED** — per-gate PASS/FAIL lines explicitly reassigned to the gate revisions; digest criterion now "position 1 prints `Win` with a nonzero proof digest" only (correct — a `NoWin` has no proof tree).
- **N-6: CLOSED** — position 2 gains two defender stones "placed so the position stays mid-game", with the N-6 rationale quoted.
- **N-7: CLOSED** — the note now states the carrying fact (value path never consults zones at `free_stone_radius = 8`, so certificate and oracle cannot contaminate; too-small `Z_p` lands as `NoWin`), explicitly CONDITIONAL on the no-prune value.
- **N-8: CLOSED** — cost priced (10⁵-10⁶ applications per AND node, terminating by construction), detached discipline, wall cap, `VERIFIER-OVERRUN` named (distinct from (c2)'s 60-minute cap — the two legs are separate wall budgets).
- **N-9: CLOSED** — `wp18a_design_REVIEW.md` header now reads "Landed retroactively with revision 3's fix round, two fix rounds after the review itself".

## New findings in the diff

- **NEW-1 (was MAJOR, fixed within the round): self-contradicting registered cap.** Gate (b) read "runs under the same detached discipline and **30-minute** wall cap **as (c2)**" while (c2)'s registered cap is 60 minutes. Corrected to "its own 30-minute wall cap … (distinct from (c2)'s 60-minute cap — the two legs are separate wall budgets)"; the header's "cost figures match the registered caps" reworded to "carry their own registered caps". Verified consistent.
- **NEW-2 (was MINOR, fixed within the round): unmarked estimate.** Gate (b)'s "on the order of 10⁵-10⁶" now carries ", ESTIMATED —".

No other new claims, contradictions, or ambiguities introduced; the remaining hunks are reflows, the seesaw "measured at the gates" precision, and accurate history blocks.

**PASS.**
