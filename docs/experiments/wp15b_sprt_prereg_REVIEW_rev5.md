# WP-1.5b SPRT pre-registration — REVIEW-design, revision 5

**Pinned target.** `docs/experiments/wp15b_sprt_prereg.md`, revision 5, pinned at
repo SHA `cd70944` per the launch condition. HEAD at review time is `b0c9594`
(a later ADR-log-only commit). Verified `git log cd70944..HEAD --
docs/experiments/wp15b_sprt_prereg.md` is empty: the two SHAs name byte-identical
text of the reviewed file, so this review's findings hold at both.

Fresh context. This session authored no prior revision of this document, no
prior review of it, and touched none of its instrument chain before this
review began.

## VERDICT: **PASS**

The two launch-condition defects are fixed, correctly and completely, and no
new BLOCKING or MAJOR defect was found across the nine verify items. Two MINOR
observations are recorded below; neither blocks. Revision 5 governs nothing by
itself — it still requires every §9 OPERATOR-CONFIRM slot filled, `§8`'s
soundness gate green, and `configs/instrument_staged_v0.toml` /
`configs/arena_wp15b_staged_vs_r2.toml` to exist — but as a REVIEW-design of
the document's own text and its instrument chain, it PASSES.

---

## Launch condition: both named defects verified fixed

1. **The `--config`/matrix-M4 sentence.** Old text (per the dispatch) claimed
   "its §9 MATRIX M4 ADOPTS adding `--config`." Current §7A.2 (prereg lines
   381–387) instead states N-E was selected by **D-329** "at axis A's tiebreak
   rung (b) — not adopted by the matrix," and that "matrix M4's own
   recommendation was N-Q, and its own red team killed it." **Verified against
   the primary sources, not the prereg's own word:**
   - `docs/decisions.md` D-329 (line 703): "MATRIX M4'S AXIS A SELECTS N-E …
     AT TIEBREAK RUNG (b) … THE MATRIX RECOMMENDED N-Q AND ITS OWN RED TEAM
     KILLED THAT RECOMMENDATION." Matches the prereg's claim exactly.
   - `docs/experiments/matrix_M4_axisA_selection.md`: title itself is
     "SELECTED: N-E, and it is not what the matrix recommended," body states
     "The matrix recommended N-Q; the attack killed that recommendation; this
     record selects the row the attack recommends instead." Confirms.
   - `docs/experiments/matrix_M4_axisA_REDTEAM.md` §5 "RECOMMENDATION — NOT
     N-Q. N-E." with finding F10 showing N-M (not N-E or N-Q) is eliminated
     because it cannot take the invocation
     `tools/baseline_snapshot.sh --config configs/gate_v0.toml` that
     **this very prereg's §7A.2 registers** — confirming the prereg's claim
     that D-329's elimination of N-M ran through this document by name.
   - Reproduced live: `./tools/baseline_snapshot.sh --config
     configs/gate_v0.toml` → exit 1, `baseline_snapshot: FAIL: unknown
     argument \`--config\`` (verbatim match to what the prereg quotes as
     MEASURED at this revision — `tools/baseline_snapshot.sh` still hard-codes
     `CONFIG="configs/instrument_v0.toml"` at line 182 and has no `--config`
     arm; N-E is selected but not yet shipped).
   - Searched the operative document body for stray un-fixed occurrences of
     the old phrasing: none found outside the changelog paragraph (prereg
     lines 40–41) that quotes the old, superseded text specifically to explain
     what revision 5 changed.

2. **The `wp15b_design.md` dangling citation.** Current §1 (prereg lines
   64–68) reads: "carved by D-310's option D into
   `docs/experiments/U1_gate_supersession.md` through `U4_soundness_
   instrument.md`; the pre-carve document is not in the tree, retrievable at
   `6feb40a` and nowhere else." **Verified independently:**
   - `git cat-file -e HEAD:docs/experiments/wp15b_design.md` → fails, file
     absent from HEAD.
   - `git cat-file -e 6feb40a:docs/experiments/wp15b_design.md` → succeeds,
     1975-line file present at that SHA.
   - `git show --stat cf74594 -- docs/experiments/wp15b_design.md` shows the
     very next commit touching the file deletes all 1975 lines outright (the
     carve commit, "WP-1.5b's design is carved into four units … The
     superseded document is deleted rather than kept").
   - `docs/experiments/U1_gate_supersession.md`, `U2_node_protocol.md`,
     `U3_tier_t.md`, `U4_soundness_instrument.md` all exist in the tree, each
     opening with "WP-1.5b CARVE MEMBER" — confirms the carve destination
     named in the fixed citation is real.
   - MINOR (non-blocking, see below): "retrievable at `6feb40a` and nowhere
     else" is imprecise if read as "only that one SHA" — the file is equally
     retrievable at any ancestor commit before `cf74594` deleted it (e.g.
     `d94dc0a`, `2d07ff6`). The natural reading — "not in the tree; pin the
     one specific historical revision, per the D-309/D-311 citation pattern
     this WP already uses elsewhere" — is what the sentence intends, and
     `6feb40a` is the correct choice (the final, reviewed-closed revision, per
     D-309's "revision 7 is CLOSED"). Not a defect; recorded for completeness.

---

## The nine verify items

**1. Dry-run record — input same-kind, not the registered workload; input and
output recorded; dry run does not consume the first governed run.**
VERIFIED, and re-run live. `configs/arena_wp15b_dryrun.toml` exists
(sha256 `25527dba44…`, matching §8.4's recorded digest). It pairs
`configs/instrument_v0.toml` (radius 2) against `configs/gate_v0.toml`
(radius 1) — neither is the staged seat under test, exactly as §8.1 states.
I rebuilt the workspace fresh (`cargo build --release --locked --bin arena
--bin pistol`), producing binary digest `dcfa671a…` — different from the
committed `a7f519fa…` (an ordinary rebuild-goes-stale case, see item 8 below).
Running the arena against the **committed, unmodified** config refused before
any game with `EngineBinaryDigestMismatch`, exit 2, **no report file
written** — matching §5's pre-game-refusal row exactly. I then ran the
identical config from a **scratch copy** with the digest field updated to my
local build (never touching the tracked file) and reproduced the dry run
end to end:
```
n 8  distinct-n 8  (0 duplicate games)
0 W / 7 L / 1 capped for r2  (capped fraction 0.125)
pair outcomes  p0 3 p1 1 p2 0 p3 0 p4 0  (4 pairs)
first player won 3 of 7 decided non-forfeit games
LLR pair -1.665757   nelo_pair -992.88   VERDICT inconclusive_at_game_cap
```
This matches §8.4's recorded `counts`, `pentanomial`, `first_player_wins`,
`llr_pair last -1.665756788`, `nelo_pair -992.879886851`, and `verdict
inconclusive_at_game_cap` to displayed precision. §8.6's claim that the dry
run does not consume the registered run is structural (it plays a different
matchup, on a different book, against configs the registered run does not
name) and CLAUDE.md's own text agrees a dry run of this kind is never a
governed sample.

**2. Inversion guard — per-game attribution; confined-inversion residual
stated honestly; honest-twin test run.**
VERIFIED, run live, twice. First, I ran `wp15b_attribution_check.py` against
my own freshly-produced dry-run report (not the committed one) and got:
```
attribution_check: 1a: 16 turns replayed, 8 of them discriminating, 8 of 8 games attributed
attribution_check: 1b: 7 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 8 games and 4 pairs rebuilt off the score_a path
attribution_check: PASS — 0 failure(s)
```
— an exact match to §8.4's recorded Criterion-1 output. Second, I ran
`cargo test --release --locked -p pistol-cli --test
wp15b_attribution_check_tests`, driving the shipped script's full suite,
including the honest-twin case
(`an_honest_report_it_cannot_attribute_is_refused_rather_than_certified`) and
the confined-swap attack the prereg cites
(`a_seat_swap_confined_to_games_link_1a_cannot_attribute_is_rejected`): **7
passed, 0 failed**. Read the source (`tools/wp15b_attribution_check.py`
lines 126–133) to confirm the guard really is per-game (a game with zero
discriminating replayed turns is now a named failure, not folded into a
whole-run aggregate) — matches D-308's fix description exactly.

**3. Exit-code taxonomy — precondition failures cannot read as attribution
defects; a skipped build is loud.**
VERIFIED by source reading. `def die(why)` (line 62) prints `CANNOT READ:` and
raises `SystemExit(2)` for every precondition failure (unreadable report,
missing engine/id line, non-`nodes` budget, unrunnable engine, malformed
record, duplicate key). Attribution failures use a separate path that prints
`FAIL …` and exits 1. Pass exits 0. This is a clean three-way split; a
skipped or missing build (unrunnable engine binary) routes to `die()`, exit 2,
not to the FAIL/exit-1 path a reader would mistake for an attribution defect.

**4. Budget line — replay reads the report's own budget, refuses any kind but
nodes, loudly.**
VERIFIED by source reading (lines 117–127): the script regex-matches
`^budget (\S+) (\d+)$` from the report text itself, dies (exit 2) if absent,
and dies (exit 2) if the kind is not `"nodes"` — with the message stating
*why* ("a budget that is not reproducible cannot carry link 1a"). This is
exactly D-308's fix (2), which closed the prior hardcoded-`go nodes 50000`
hole that would have silently replayed a `movetime_ms`-budgeted run at the
wrong search.

**5. Registered engine scope matches shipped D scope — F+T, stage Q absent,
N-E consistent with D-329.**
VERIFIED. §1's citation of D-310's option D (F+T units, Q deferred) matches
the actual carve: `U1_gate_supersession.md`, `U2_node_protocol.md`,
`U3_tier_t.md`, `U4_soundness_instrument.md` exist, and every mention of
"stage Q" inside U2/U3 is a deferral pointer to `WPQ_seed.md` (e.g. U2 line
216: "the batched loop is stage Q, DEFERRED with the widening schedule — see
`WPQ_seed.md`"; U3 line 22–23: "MATRIX M2 … EXCISED to `WPQ_seed.md` with
stage Q, per D-310"). N-E's selection consistency with D-329 is covered under
the launch-condition check above.

**6. Cost statement on the document's face.**
PRESENT. §7's table states wall time, machine hours (core-seconds/core-hours)
and operator attention explicitly, with MEASURED beside DECLARED per D-290/
D-292's reconciliation duty. Spot-checked the MEASURED figures against their
cited ADR lines: "16.3 core-seconds per game — 3.3× the anchor … ≈18
core-hours … the second reviewer reproduced independently at 16.6" matches
D-307's text verbatim. "5.44 core-hours, ~82 min wall" for the 2000-opening
anchor matches D-292's text verbatim. My own live rebuild measured ~17.2
core-seconds/game ((75214+62739) ms / 8 games) — a fourth, consistent data
point on top of the document's own acknowledged machine-to-machine spread
(§8.3 Criterion 3: "388/235 ms … 396/240 … 474/285 … three machines, one
quantity").

**7. Verdict criteria pre-registered, GSPRT bounds/n-accounting per hard rule
6, distinct-n dedupe, no post-hoc freedom.**
VERIFIED against source. §4's report-schema listing was checked field-by-field
against `crates/pistol-arena/src/report.rs` and `conclusion.rs`: every line,
in the exact order specified, including the `bounds h0 … h1 …` line being
separate from `llr_pair`, and `distinct_n` being reported (via
`dedupe::distinct_count`) as CLAUDE.md rule 6 requires. `Verdict::token()`
(`sprt.rs` lines 244–254) confirms the exact five-token enumeration §4
states, with "inconclusive" absent as a standalone token — matching the
prereg's explicit claim. §3's corrected capped-game statement was checked
against `score.rs::tally` (lines 74–105): confirmed the entire
`decided`/`decided_clean`/`wins_a`/`losses_a` block sits inside the
non-capped match arm, and `game_sample` is built from `(wins_a, capped,
losses_a)` exactly as claimed. §5's outcome table pre-registers every branch
(including the corrected degenerate-direction and balanced-degenerate rows)
before any game is played, satisfying the no-post-hoc-threshold rule.

**8. Engine seats digest-bound at the revision the run will use.**
VERIFIED, live, twice. Running the committed, unmodified
`configs/arena_wp15b_dryrun.toml` against a rebuilt (differently-digested)
`target/release/pistol` produced: `arena: EngineBinaryDigestMismatch: engine
r2: \`target/release/pistol\` hashes to dcfa671a… and this document binds it
to binary_sha256 a7f519fa…; the file at that path is not the build this run
is written for, so no game was played` — exit 2, no report file. This is a
live, unforced demonstration of D-283/D-294's binding working exactly as the
prereg describes, and confirms §5's pre-game-refusal row is accurate in
practice and not merely asserted.

**9. N-E implementation-debt visibility.**
VERIFIED as honestly named. `tools/baseline_snapshot.sh` at HEAD still has no
`--config` argument at all (confirmed: `--config` reaches the `*) usage >&2;
fail "unknown argument …"` catch-all at line 341) — N-E is selected (D-329)
but not yet shipped. The prereg's §7A.2 and §9.7 both state this plainly as a
present-tense fact ("does not [accept --config]… the script cannot measure
the staged seat at all") and §9.7 makes "the commit at which
`tools/baseline_snapshot.sh` accepts `--config`" an explicit OPERATOR-CONFIRM
slot rather than assuming a revision. This satisfies the dispatch's "named in
the prereg as a known cost" branch.

---

## `tools/SHELL_CHECKLIST.md` — items answered by name

For the instrument scripts the prereg's own verdict reads through:
`tools/wp15b_attribution_check.py` (Criterion 1, §7A.1's instrument),
`tools/config_check.sh` (dry-run literal command 3), and
`tools/baseline_snapshot.sh` (§7A.2's Doubt-2 instrument, not yet carrying
`--config`).

- **Items 1–7, 9 (shell-specific hazards)**: `wp15b_attribution_check.py` is
  Python, not shell, and its own docstring states why ("THIS PARSES A REPORT
  FIELD BY FIELD… A missing key here raises rather than expanding to the empty
  string"). Item 9's spirit (caller-controlled free text reaching a record)
  DOES apply and is discharged: the games/moves loop uses `text.split("\n")`
  rather than `splitlines()`, with a comment at lines 130–133 naming exactly
  the item-9 hazard (an engine's verbatim refusal carrying `\r`/`\x0b`/U+2028/
  U+0085 would otherwise desynchronize the parser's line boundary from the
  format's own). Verified present in the shipped file.
- **Item 8 (one spelling per number, one refusal per reason)**: `fields()`
  (lines 67–79) refuses a duplicated key by name rather than silently
  last-winning — matches D-308's fix (4b) and is present in the shipped file.
  `tools/baseline_snapshot.sh`'s existing guards are unaffected (N-E is not
  yet shipped there).
- **Item 10 (THE COVERAGE RULE)**: `tools/wp15b_attribution_check.py` is
  driven by `crates/pistol-cli/tests/wp15b_attribution_check_tests.rs`,
  confirmed by running it: **7 passed, 0 failed**, covering the honest
  report, three single-link corruption attacks, the confined-swap attack, its
  honest-twin control, and the void-vs-fail case. `tools/config_check.sh` is
  exercised by the dry-run's own literal command and separately by its own
  existing suite (unchanged in this revision). `tools/baseline_snapshot.sh`'s
  N-E arm has no driving test yet because it has not shipped — consistent
  with item 9's "named as a known cost" finding above, not a new gap this
  revision introduces.
- **Item 11 (containment on rm/mv/write)**: `wp15b_attribution_check.py`
  performs no delete, move or write of any caller-supplied path — not
  engaged.
- **Item 12 (VOID vs FAIL, by name)**: Fully discharged. `die()` → exit 2 for
  every precondition failure; a genuine attribution finding → exit 1; a clean
  run → exit 0. Matches D-308's fix (1) exactly, and is what item 4's budget
  check and item 3's exit-code-taxonomy verification above both rest on.

---

## MINOR observations (non-blocking)

1. "Retrievable at `6feb40a` and nowhere else" (prereg line 65–66) reads most
   naturally as "this is the one SHA to cite," but is technically imprecise —
   the pre-carve file is retrievable at every ancestor commit back through its
   creation, not uniquely at `6feb40a`. `6feb40a` is nonetheless the correct
   SHA to have picked (the final, review-closed revision per D-309), so this
   does not misdirect a reader; recorded for the drafting session's awareness
   only.
2. §3 states "The document that IS the experiment is
   `configs/arena_wp15b_staged_vs_r2.toml`" with no existence caveat, whereas
   the intro paragraph explicitly flags `configs/instrument_staged_v0.toml`
   as not yet existing. Neither `configs/arena_wp15b_staged_vs_r2.toml` nor
   `configs/instrument_staged_v0.toml` exists in the tree at this revision.
   The document's own governance clause (intro, "It becomes governing when
   three things hold together: the config exists…") already covers this, so
   it is not a defect — but a future revision could usefully state the
   missing-experiment-config fact in §3 itself, next to §9's slots, rather
   than leaving it to be inferred from the intro's narrower statement about
   the engine config alone.

---

## Summary of what was independently verified with reproducers (not taken on
the document's word)

- D-329's selection chain (M4 recommends N-Q → red team kills it → D-329
  selects N-E at rung (b), eliminating N-M on a ground that names this very
  prereg) — read from `docs/decisions.md`, `matrix_M4_axisA_selection.md`,
  and `matrix_M4_axisA_REDTEAM.md` directly.
- `docs/experiments/wp15b_design.md`'s absence at HEAD and presence at
  `6feb40a`, and its deletion in the immediately following commit `cf74594`.
- `tools/baseline_snapshot.sh --config configs/gate_v0.toml` → verbatim
  `unknown argument` failure, exit 1, live.
- The full dry run, re-executed live end-to-end (config validation, arena
  run, Criterion 1), reproducing §8.4's recorded numbers on an independent
  rebuild.
- The honest-twin and confined-seat-swap attribution tests, run live via
  `cargo test`, 7/7 passing.
- A live `EngineBinaryDigestMismatch` refusal (exit 2, no report file) when
  the committed dry-run config was run against a locally rebuilt, differently
  -digested binary.
- Every line of §4's report-schema table, checked field-by-field against
  `crates/pistol-arena/src/report.rs` and `conclusion.rs`.
- §3's corrected capped-game statement, checked against
  `crates/pistol-arena/src/score.rs::tally`.
- The `Verdict` token enumeration, checked against `crates/pistol-arena/src/
  sprt.rs`.
- The two D-308 instrument fixes cited by the prereg (per-game guard,
  void/fail exit-code split, budget-kind refusal, split("\n") vs
  splitlines(), duplicate-key refusal), all read directly from
  `tools/wp15b_attribution_check.py`'s shipped source.

**Header restated: pinned at `cd70944`; HEAD (`b0c9594`) carries an identical
copy of this file, confirmed via an empty `git log cd70944..HEAD --
docs/experiments/wp15b_sprt_prereg.md`. PASS.**
