# WP-2.0b — the five findings the design carries INTO implementation

The rev-8 review's recommendation was *"land with named surgery, do not split"*, with
the surgery applied to the design and **five findings deferred to implementation time**
rather than to another design revision. They are listed here because a finding deferred
in a review report a successor may not read is a finding lost.

Each is fixed by the implementer, and REVIEW-impl checks it as part of the diff.

| id | finding | what the implementer does |
|---|---|---|
| AG2 | **H1 is defined twice.** §9 registers H1 as the cross-binary token-OFF comparison (post-change vs `artifacts/pistol_prechange_a56449b`), and the surrounding prose still reads in places as though H1 were the single-binary ON/OFF ratio. | State H1 once, in the bullet that owns it, and have the other mentions point there (D-423). |
| AG3 | **H1 has no rejection region.** A `1.000x` hypothesis needs a band outside which it is rejected, and the review MEASURED the floor: paired sd **0.0075**, so **±2 % at REPS=5**. | Register `H1 rejected outside [0.98, 1.02]` on the CROSS-BINARY comparison — where, unlike the withdrawn single-binary bracket, the band is not common-mode and the number is measured rather than invented. |
| AG4 | **The fourth-word refusal (test 15) has no counterpart in the grammar section's own wording**, so an implementer could satisfy §8 and leave §3's sentence unpinned. | Pin the exact refusal text in test 15 and quote it in `budget_token.rs`'s `///` docs. |
| AG5 | **The governing dispatch's fourth mutant is not accounted for by name.** v2 lists *"transposition ruling inverted -> fixture dies"*; §8 covers it at test 4, but the mapping is not stated, so a closure reader cannot check the dispatch's list against the design's. | State the four dispatch mutants against §8's rows in one table at closure. |
| AF2-residual | **The calibration run is unsized.** §1.1 registers its arms, quantity, inputs and constraint but no duration or position source, deliberately. | Not the implementer's: it is the operator's or the run's own pre-registration. Listed so it is not mistaken for an oversight. |

**None of these changes what is built.** They are registration and documentation defects
in §9 and §3, and the rev-8 review's own words are that *"§§2-8 are correct and
landable"* and that it *"found no defect that produces a wrong answer at runtime"*.
