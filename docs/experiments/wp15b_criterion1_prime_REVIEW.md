# WP-1.5b — REVIEW-design (scoped amendment review), Criterion 1' / revision 6

**Pinned target.** Repo HEAD `a292897f989770e99fc0038e4c6335a510e95555` on `dev`. Verified live: `git rev-parse HEAD` → `a292897f989770e99fc0038e4c6335a510e95555`, `git status --porcelain` → empty (clean tree). HEAD matches the dispatch's pinned target exactly; no drift.

Fresh context. This session authored none of the three commits under review (`217affe`, `a80a864`, `a292897`), no prior revision of the prereg, and no prior review of it.

## VERDICT: **PASS**

No BLOCKING or MAJOR finding. Two MINOR findings recorded below, neither blocks. All live checks (tests, clippy, fmt, ported-arithmetic spot-check, D-381 non-reread verification) confirm the amendment does what D-384 says it does, and does only that.

---

## Scope item 1 — §7A.1 against D-384

Read `docs/decisions.md` D-384 (line 813) and `docs/experiments/wp15b_sprt_prereg.md` §7A.1 (lines 360–433) side by side.

- **Criterion 1' text**: §7A.1 line 380–384 quotes D-384 **verbatim, byte-for-byte**: "A report is a measurement iff (a) zero confirmed inversions under links 1b/1c applied to all games, and (b) the verdict is invariant under adversarial reassignment of every link-1a-vacuous pair, recomputed from the report's own pentanomial and LLR machinery." Matches D-384's quoted sentence exactly.
- **Provenance chain** (D-383 killed widening on the cold-replay/warm-TT confound; TOLERATE-WITH-ROBUSTNESS selected; warm-replay licensed-not-scheduled; the flip clause) is all present and accurately restated in §7A.1 lines 397–408, 421–427.
- **Per-game (a) vs per-pair (b)** distinction, and the D-308 provenance of "per game," are both correctly attributed (§7A.1 lines 410–419).
- **MINOR 1 — clause-(a) attribution of link 1a is an interpretive addition, not literal text.** D-384's literal clause (a) names only "links 1b/1c." §7A.1 (line 384–387) states link 1a's own direct discriminating-turn mismatch "is a confirmed inversion under clause (a) exactly as it always was" — extending clause (a) to cover 1a as well, which the ADR's quoted sentence does not literally say (1a appears only inside clause (b)'s "link-1a-vacuous pair" phrase). Checked this is not a functional gap: the code (`tools/wp15b_attribution_check.py` lines 284–288) still unconditionally appends a 1a mismatch to `failures` regardless of any clause label, and the driving test `a_seat_label_attached_to_the_wrong_engine_is_rejected` (non-vacuous, both turns discriminate) exercises exactly this path and passes live (see Scope item 3). So the *behavior* is correct and unchanged; the *label* "under clause (a)" is the prereg's own gloss on a D-384 shorthand that plausibly just meant "the links whose per-game behavior is unaffected by this ADR," not "clause (a) excludes 1a." Recorded as a wording-precision nit against D-384/§7A.1, not a defect — a future ADR line could clarify the enumeration.
- **MINOR 2 — the "Run 1 not re-read, prospective-only" sentence is not restated in the document itself.** D-384's final sentence — "Run 1 (D-381) is NOT re-read under Criterion 1' and stands exactly as recorded — 1' governs runs taken after its own registration" — is nowhere in `wp15b_sprt_prereg.md` (searched for `D-381`, `prospectiv`, `re-read`, `stands exactly`: the only `D-381` hit is the historical narrative at line 11, and the only "not re-read" text is at line 846, in §11's description of *this review's own scope*, not a claim the document makes about itself). §7A.1 relies entirely on the D-384 citation for this ruling. This is not just theoretical: this review's own live run of the amended checker against the preserved D-381 report (see item 4 below) returns `PASS`/exit 0 — a result a reader relying on §7A.1 alone (without pulling D-384's full text) could misread as "D-381 is a measurement after all." In the actual tree, nothing does misread it that way: D-381's `decisions.md` entry is byte-unmodified since `217affe` (confirmed by the D-384 commit's diff being a pure 2-line insertion), and no new governed report or ADR line reopens it. Recommend inlining D-384's prospective-only sentence into §7A.1 on the next revision touching this section; not blocking now.

## Scope item 2 — checker diff (`a80a864`) against Criterion 1' and the real SPRT machinery

Read the full diff (`git show a80a864 -- tools/wp15b_attribution_check.py`) and the current file, and compared line-by-line against `crates/pistol-arena/src/sprt.rs` and `score.rs`.

- **Clause (a)** (1b/1c unchanged, universal, per game): confirmed unchanged in the diff — 1b's game-rule-3 check and 1c's `score_a`/pentanomial rebuild are untouched except for hoisting `scores`/`buckets` earlier (reused by clause (b), not recomputed — comment at lines 388–389 makes this explicit, closing off a drift risk).
- **Clause (b)** (adversarial reassignment of every vacuous pair, recomputed off the report's own pentanomial/LLR machinery): implemented at lines 296–368. The pair-simultaneous-vacuity invariant is **asserted, not assumed** (`die()` at lines 310–315 if a pair has exactly one vacuous game) — matches the diff's stated intent exactly.
- **Ported arithmetic vs. the real machinery**, checked constant-by-constant and line-by-line:
  - `NELO_TO_T = math.log(10.0) / 800.0` ↔ `sprt.rs`'s `LN_10 / 800.0` — identical.
  - `PAIR_SCORES = (0.0, 0.25, 0.5, 0.75, 1.0)` ↔ `sprt.rs::PAIR_SCORES` — identical.
  - `pair_sample()` (n, first/second moments, mu, var) ↔ `Sample::of()` — identical formula and summation order (`second/n - mu*mu`, the "raw-moment form" `sprt.rs` deliberately pins).
  - `t0/t1 = elo*NELO_TO_T*sqrt(2.0)` ↔ `Unit::Pair.t()`'s `normalized_elo * NELO_TO_T * SQRT_2` — identical.
  - `llr = n * ((t1-t0)*t_hat - (t1²-t0²)/2)` ↔ `Sample::llr()` — identical expression, same order.
  - `h0 = ln(beta/(1-alpha))`, `h1 = ln((1-beta)/alpha)` ↔ `Bounds::of()` — identical.
  - Crossing order (`llr>=h1 → h1`, `llr<=h0 → h0`, else `inconclusive_at_game_cap`) ↔ `crossing()` + `verdict()`'s fallback — identical, and `verdict()`'s forfeit-first branch is correctly and explicitly left OUT of the Python port (forfeits are caught by a separate, pre-existing prereg §5 rule; the code's own comment at lines 104–107 names this rather than silently omitting it).
  - **Self-check before trust** (lines 330–348): the ported arithmetic is checked against the report's own printed `verdict` token on the *unmodified* pentanomial and `die()`s (exit 2, a VOID not a finding, per §7A.1's own text) on any mismatch, before ever being used on the flipped pentanomial. This is exactly the "externally derived referent" pattern CLAUDE.md's own text asks for, applied to a reimplementation-drift risk this project has been burned by before.
  - The "flip" operation (`bucket → 4-bucket` per vacuous pair, all vacuous pairs flipped **simultaneously in one recomputation**) is the literal reading of D-384/§7A.1's own elaboration ("as though every such vacuous pair had been the opposite of what the report says") — a single global-flip scenario, not a search over all 2^k subsets. This is not the statistically strongest possible adversarial test (a per-pair worst-case search could in principle diverge from it), but it is what D-384 and §7A.1's own prose literally specify, so the code is faithful to the *stated* rule — not a stricter or looser rule than written. Whether that stated rule is the best possible one is TOLERATE-WITH-ROBUSTNESS's own design, already ruled by the architect and out of this review's scope.
  - `pentanomial_counts` (histogram, index=bucket) vs. `buckets` (per-pair, index=pair) are kept as distinct names with an explicit comment (lines 335–339) flagging exactly the "a bare rename would have hidden this" risk — good practice, no conflation found.

## Scope item 3 — tests, clippy, fmt (run live)

```
$ cargo test --test wp15b_attribution_check_tests -p pistol-cli -- --test-threads=1
running 7 tests
test a_result_credited_to_the_seat_that_did_not_move_last_is_rejected ... ok
test a_seat_label_attached_to_the_wrong_engine_is_rejected ... ok
test a_seat_swap_confined_to_a_vacuous_pair_fails_robustness ... ok
test a_vacuous_pair_that_does_not_move_the_verdict_is_certified ... ok
test a_verdict_inverted_downstream_of_the_game_lines_is_rejected ... ok
test an_answer_that_could_not_be_taken_is_exit_two_and_not_a_finding ... ok
test an_honest_report_passes_all_three_links ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.65s
```
All 7 pass, including the two new/rewritten ones: `a_seat_swap_confined_to_a_vacuous_pair_fails_robustness` (D-308's kept attack, now caught via clause (b) since clause (a) no longer sees it — MEASURED against the ported arithmetic in the test's own comment, verified by the passing test) and `a_vacuous_pair_that_does_not_move_the_verdict_is_certified` (the new TOLERATE behavior — a vacuous-but-robust 20-pair report is certified exit 0, not refused).

```
$ cargo fmt --all -- --check
(exit 0, no output)

$ cargo clippy --workspace --all-targets -- -D clippy::all
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
(exit 0, no warnings/errors)
```

## Scope item 4 — D-381 is not re-read

- `git log a292897..HEAD` — confirms `a292897` **is** HEAD (nothing landed after it).
- `docs/decisions.md`: D-384 is the last entry in the file (`tail -5`); D-381's own entry (line 807) is untouched — the `217affe` diff is a pure 2-line insertion appending D-384, no edit to D-381.
- `artifacts/` contains exactly one file, `wp15b_governed_run.txt` — the same D-381 report (its `experiment_sha256` field, `228fa48f…`, matches D-381's citation). No second/newer report exists anywhere in the repo (filesystem-wide search for `*wp15b*governed*` found only this file and its scratchpad copy).
- **Live spot-check, as the dispatch invited**: built the release binary (digest `9acd23544d…`, matching D-379's confirmed value — the correct revision for this report), then ran the amended checker against the preserved D-381 report:
```
$ python3 tools/wp15b_attribution_check.py artifacts/wp15b_governed_run.txt target/release/pistol
attribution_check: 1a: 232 turns replayed, 162 of them discriminating, 106 of 116 games directly attributed by replay
attribution_check: 1a robustness: 5 vacuous pair(s) — 8 (opening 8), 22 (opening 22), 34 (opening 34), 44 (opening 44), 57 (opening 57) — adversarially reassigned; verdict `h1` unchanged (`h1`)
attribution_check: 1b: 116 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 116 games and 58 pairs rebuilt off the score_a path
attribution_check: PASS — 0 failure(s)
EXIT=0
```
This exactly matches the commit message's own claim ("exit 0, all 5 vacuous pairs named, verdict h1 unchanged under reassignment") and the 5 openings (8, 22, 34, 44, 57) `wp15b_vacuity_diagnostics.md` and D-381 both name. **This confirms the mechanism works correctly** — but nothing in the tree treats this as reopening D-381: no new ADR line, no commit, no report file records this as a verdict change. D-381 stands exactly as recorded, per D-384's own text. (See MINOR 2 above for the one gap: the document itself doesn't say this explicitly, only the ADR does — verified true in practice regardless.)

## Scope item 5 — criterion registered before the run it governs

Confirmed no confirmatory or new SPRT run has been taken under Criterion 1': `a292897` is HEAD (empty forward log), `artifacts/` holds only the pre-existing D-381 report, and `decisions.md` has no entry after D-384. The amendment is landed and awaiting exactly this review before any run it would govern — consistent with the pre-registration/review-before-run rule.

---

## Findings summary

| # | Severity | Finding |
|---|---|---|
| 1 | MINOR | §7A.1 attributes link 1a's direct discriminating-turn mismatch to "clause (a)," while D-384's literal quoted clause (a) names only links 1b/1c. Verified no functional gap (code and a driving test both confirm 1a mismatches still fail unconditionally); recommend an ADR precision fix on a future revision, not blocking. |
| 2 | MINOR | Neither §7A.1 nor any other part of `wp15b_sprt_prereg.md` restates D-384's explicit "Run 1 (D-381) is NOT re-read… governs prospectively" sentence — it exists only in the ADR. Verified true in practice (D-381 untouched, no new report/ADR line), and this review's own live run against D-381's report demonstrates exactly the ambiguity a future reader could hit (checker says PASS on the old report). Recommend inlining the sentence in §7A.1; not blocking now. |

No BLOCKING or MAJOR finding.

**Header restated: pinned at `a292897f989770e99fc0038e4c6335a510e95555`; HEAD unchanged and clean at review's end (`git rev-parse HEAD` reconfirmed). PASS.**
