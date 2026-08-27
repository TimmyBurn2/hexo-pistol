# REVIEW prompt — sealbot anchor: local match platform + adapter

You are a fresh-context reviewer. You did not write this code. Attack the
premise, verify the claims, and report findings with severities. You have ONE
fix round budget: findings you raise should be ones worth fixing once, not
style (CLAUDE.md: reviewers flag correctness and requirement gaps, not style).

**Revisions under review** (branch `sealbot-anchor`, worktree
`/home/tom/Projects/pistol-sealbot-match`):
- instrument tree: `450d7aa5cebc0ed2e51ef27327faf55c637776ef`
- pre-registration: `7394743` (its own governing revision)
State both in your report header, and whether HEAD still matches `7394743`.

## What is under review

1. `tools/sealbot/` — the local HeXO match platform: `matchserver/` (Rust
   crate, NOT a workspace member), `sealbot_shim.py`, `pistol_hexo_adapter.py`
   (interim bridge adapter, not exercised by the match), `run_match.sh`,
   `tests/` (stub suite). Design claims live in `tools/sealbot/README.md`.
2. `docs/experiments/sealbot_anchor_prereg.md` — the pre-registration the
   governed anchor run will stand on.
3. `local/sealbot.example.toml` (committed shape) and the `.gitignore` change
   (`/local/*` except the example).

NOT under review: anything under `crates/` — the branch's OWN commits (`450d7aa^..HEAD`, i.e. the three commits after dev HEAD `923475f`) touch no crate; note the branch CARRIES WP-1.8a's already-committed solver work between `e2280ca` and `923475f`, which is not yours to review — verify the split yourself with both `git diff --stat 923475f..HEAD` (this work) and `git diff --stat e2280ca..923475f` (WP-1.8a's). The pistol-solver unjustified-over-cap gate failures are WP-1.8a's, pre-existing at dev HEAD.

## The commissioning dispatch's constraints (verify each)

- Adapter code in the repo under `tools/sealbot/`; platform-specific paths in
  gitignored `local/`; example committed; transcripts/reports to `artifacts/`,
  content-named, digested in the ADR (the ADR line comes after the run).
- "Thin. No game logic outside pistol-core (rule 2)."
- Proportionality: NO red team; THIS review is the one review; one fix round.

## Rules this work must satisfy (check, don't assume)

- CLAUDE.md rules 2 (rules truth in pistol-core only), 3 (fail loud), 4
  (determinism: the pistol side runs instrument mode, node budget registered),
  8 (no artifacts committed), 9 (file cap), 11 (pistol-api untouched).
- The pre-registration rules in CLAUDE.md's Process section: literal commands
  exercised on the same KIND of input before review; criteria that name a
  DEFECT CLASS and could actually fail; a second instrument that does not
  share the stage under doubt; a registered consequence for disagreement.

## tools/SHELL_CHECKLIST.md — answer these items BY NAME for `run_match.sh`
and `tests/run_tests.sh` (both are `tools/` scripts producing recorded
numbers): items 1, 2, 3, 6, 8, 9, 10, 11, 12. For item 10 name the test that
drives each shipped script and run it.

## Concrete questions to attack (not exhaustive)

1. **Rules fidelity.** Does the referee implement the official platform
   behaviour as claimed? Check: the server-played origin opening (p1 turn 1);
   engines first asked at turn 2; strict submitted-order stone application;
   rule-4 truncation (a completing stone ends the game, a submitted second
   stone is not applied); illegal move = forfeit; incomplete turn = forfeit;
   cap = horizon, never a win. Anything pistol-core refuses but the platform
   would accept, or vice versa?
2. **The pair-order recovery.** `pistol_client.rs` recovers play order by
   shadow replay + `make_turn` + undo. Is the undo count right for Single vs
   Pair? What happens when `make_turn` refuses?
3. **`play`-vs-`place` semantics in the referee.** The transcript's per-turn
   record for an illegal SECOND stone: is the record turn number right?
4. **The Wilson interval.** Hand-check the formula against a known table
   value (e.g. 1/1 and 7/10).
5. **`replay_check`.** Does its replay semantics actually mirror the
   referee's for every outcome kind? Could a tampered record pass it (try
   more tamperings than the suite's one: a moved stone, a swapped turn, a
   dropped turn, an extra stone)?
6. **The prereg's dry-run criteria.** Could each criterion pass while its
   named defect class is present? Is any criterion vacuous (a property the
   defect preserves)?
7. **The engine pin.** README claims the `e2280ca` binary reproduces D-433's
   digest and explains the HEAD difference. Re-derive or re-run the two-build
   comparison; is the reasoning sound?
8. **Config discipline.** `deny_unknown_fields`, no defaults, kind-specific
   budget fields refused when mismatched. Anything settable that should be
   fixed, or fixed that should be settable?

## Commands you may run (all from the worktree root)

```
git diff --stat 923475f..HEAD        # this work only
git diff --stat e2280ca..923475f     # WP-1.8a's carried commits, not under review
tools/sealbot/tests/run_tests.sh
tools/sealbot/run_match.sh local/sealbot_dryrun.toml      # after removing artifacts/sealbot_dryrun_v1 first
tools/sealbot/matchserver/target/release/replay_check artifacts/sealbot_dryrun_v1
cd tools/sealbot/matchserver && cargo clippy --release
```

## Report format

Header (revisions, HEAD match), then findings each with severity
(BLOCKING / MAJOR / MINOR / NOTE), a minimal reproducer where you claim a
defect, and your per-item SHELL_CHECKLIST answers. End with a verdict:
APPROVE (as-is or with named non-blocking notes) or BLOCK (named findings).
You get the code and the documents; the governed run does not launch until
your verdict lands and any fix round closes.

---

## ADDENDUM — the fix-round re-check (scoped)

The first review returned **BLOCK** (F1) with F2–F7 required in the same
round. The fix round is committed and the re-check is a DIFF REVIEW of
exactly these changes, per the review's own closing instruction:

- instrument tree: `f254e33` (fix round) vs `450d7aa` (as first reviewed)
- pre-registration: this document's new HEAD vs `3a53624` (as first
  reviewed)

Re-check obligations, by finding:

- **F1** — re-run the zero-stone reproducer against the fixed binary; it must
  forfeit `incomplete 0 of 2` and the run must COMPLETE (the suite's `m3`
  drives it under `timeout`, but re-derive it yourself, not through the
  suite).
- **F2** — re-run the three-stone reproducer; the referee's record and
  `replay_check` must AGREE (exit 0), and the record's turn must be the
  asked turn (F6).
- **F4** — re-attempt the two tamper classes that passed (extra legal stone
  on a continue turn; `first_stone_win` flip) plus a mover relabel; all
  three must now fail `replay_check`.
- **F5** — `tools/ci.sh` gate 16/19 must invoke the suite; confirm the
  numbering is coherent end to end.
- **F3** — §5's recorded observations must match the digested instance's own
  bytes (winner, turn, nodes) at every site; grep the document for the
  superseded numbers (`124747`, `game 1 capped`, the old digests) and expect
  zero hits.
- **F6/F7, N4/N5** — spot-check by reading; each is small.

Anything OUTSIDE this diff is out of scope unless it is a NEW defect the
fix introduced. Verdict format as above; APPROVE releases the governed run.

---

## ADDENDUM 2 — the R1 re-check (mechanical, per round 1's own closing)

Round 1's re-check found R1 (MAJOR): the F2 fix's length-dispatch made
`replay_check` disagree with CORRECT referee records on mixed
over-submissions — a submission of three stones whose first illegal thing is
a place-refusal (mix1: occupied second cell; mix2: out-of-region first cell)
got a correct by-place record the checker rejected. The fix, at `37cdf81`:
the illegal branch mirrors the referee's walk (the count boundary is checked
before each placement; a refused cell fires wherever it sits), the suite
gains the mixed-class cases m5/m6, and the dry run was re-taken.

The re-check obligations, all mechanical:

- `replay_check` on the two round-1 reproducers (rebuild them or take the
  referee's own record: `[[1,1],[0,0],[2,2]]` → illegal by place at `0,0`;
  `[[50,50],[1,1],[-1,1]]` → illegal by place at `50,50`) must AGREE (exit
  0) — and the pure by-count class (`m4`) must still agree too.
- The suite must be green including m5/m6 (which pin the referee's own
  by-place classification on the mixed class).
- The prereg's §3 pin must read `37cdf81` and §5's digests must match the
  bytes on disk (`fc5a23d3…` report; `759fb084…`, `a5ac12e9…` transcripts);
  grep for the superseded strings (`b4d8fecf`, `ee8647df`, `daf61837`,
  `f254e33`) and expect zero hits in §5.
- No other surface re-opens (round 1's verdict, own words).
