# WP-1.3 — results

Companion to `wp13_prereg.md`. Every number here comes from a run whose
parameters were fixed before it started; every departure from that document is
in §4, and everything WP-1.3 did **not** settle is in §6, which is the section
worth reading twice.

- **Runs executed:** 2026-08-18/19, AMD Ryzen 7 3700X (8c/16t), 46 GB, Linux
  7.1.8-arch1-3, `cargo build --release --locked`, engine
  `196949af…` (the plain build; rebuilt byte-identical after Run 2's
  frame-pointer detour, verified by digest).
- **Reports:** `~/Work/pistol-wp13/*.matchlog`, outside the repository (rule 8).
- **ADR lines:** D-189 (Run 3a), D-190 (Run 1 + confirmation), D-191 (Run 3b),
  D-192 (H1), D-193 (H2), D-194 (the config change). Book extension: D-187.
  Pre-registration amendment: D-188.

## 1. What ran

| run | instrument | result |
|---|---|---|
| 0 smoke | depth 1, radius 1, 4 openings | gate held; 10-opening self-match clean |
| 3a fairness + determinism gate | 2000 openings, 4000 games, 50 k nodes | first player **61.8 %**; determinism held |
| 1 r2 vs r3 | primary book, 50 k nodes, elo1 = 25 | **h1** at 37 pairs, `nelo_pair +210.87` |
| 1-confirm | corpus book (disjoint), identical otherwise | **h1** at 50 pairs, `nelo_pair +155.38` |
| 3b fairness | 1591 openings, 3182 games | first player **57.1 %** |
| 2 flamegraph | both bands, 50 k nodes, 24 810 samples | **H1 76.27 % confirmed; H2 0.29 % refuted** |

## 2. The results

**Candidate radius.** Radius 2 beats radius 3 at a fixed node budget, on two
disjoint books: 126 wins of 174 games, 51 decisive pairs, both crossings in the
same direction with overlapping intervals. The mechanism was measured *before*
any game: at 50 000 nodes radius 2 completes a second turn-iteration in 17 of 24
bench positions where radius 3 manages 6. The narrower set buys depth; depth
wins. **The win is node-matched and is not a wall-clock win** — radius 2 spent
1.90× the time for 6.1 % fewer nodes.

**Fairness.** The first player wins **61.8 %** [59.7, 63.9] on uniformly sampled
5-stone openings and **57.1 %** [54.7, 59.6] on human-reached 7-stone ones, over
2000 and 1591 distinct games. Different estimands, never averaged. Both
contradict the research report's expectation of a *defender* tilt on 3 axes, at
10.6 σ and 5.7 σ. Both are properties of a 50 000-node instrument at which most
searches see no reply, not constants of the game.

**Where the time goes.** The eval apply/undo roundtrip is on 85.6 % of stacks,
76.3 % of them under move ordering. D-76's Stage-0 suspicion, registered on
reasoning alone, is correct by a wide margin, and `Eval::delta` (D-110) is
unlocked. Per-node allocation is 0.29 % — refuted, no fix licensed.

## 3. What the pre-registered rules actually did

Recorded because a rule that never bites is not evidence that it worked.

- **The 100-pair floor fired.** Run 1 crossed at 37 pairs, below it, so the H1
  action was withheld and the confirmatory run decided. Without the floor a
  config change would have rested on 74 games.
- **The "assert nothing about wall-clock in advance" clause fired.** Revision 1
  asserted that an r2 node-win implies a time-win. The run refuted that. Had the
  original clause survived review, a false claim would now be in the log.
- **The `verdict_unit pair` rule (D-154) mattered twice.** Both Run 3 reports
  carry an `llr_game` far past its boundary while the verdict is correctly
  `inconclusive_degenerate`.
- **The determinism gate held at scale**, twice: 2000 and 1591 pairs, every pair
  1-1, identical node counts per side, zero capped, zero forfeits.
- **`turn_cap = 40` never bound.** `capped_fraction` 0.000 in every run.

## 4. Deviations from the pre-registration

- **Run 2's profiling build changed codegen.** §5 said line tables would be added
  by environment override and that this "changes no codegen". `--call-graph
  dwarf` produced no usable call graphs on this machine at either an 8 KB or a
  64 KB stack dump — every entry `Children == Self`, despite
  `HAVE_DWARF_UNWIND_SUPPORT: on` and the binary carrying `.eh_frame` and
  `.eh_frame_hdr`. The profile was retaken with `-C force-frame-pointers=yes` and
  `--call-graph fp`. Cost measured at identical node counts: **+1.0 % to +3.6 %,
  median ≈ +2.8 %** — it cannot move a figure that cleared its threshold by 56
  points. **Root cause not diagnosed** (§6).
- **The confirmatory run also stopped below the floor** (50 pairs). The
  pre-registered condition was "also crosses H1", which it met; the floor was
  satisfied in combination (87 pairs, 174 games), not by either run alone.
- **A config was created after the pre-registration**:
  `arena_wp13_r2_vs_r3_confirm.toml`, written between runs to execute a rule the
  pre-registration had already fixed. It differs from Run 1's document in the
  book and its size and in nothing else, verified by diff.

## 5. Methodological failures hit along the way

Both would have produced a wrong published number.

1. **The H2 allocator pattern over-matched by 40×.** The first pass returned
   11.46 % and would have CONFIRMED H2, because the pattern matched
   `alloc::alloc::Global` appearing as a *generic parameter* inside
   `alloc::collections::btree::node::…` symbols — eval work counted as
   allocation. Corrected: 0.29 %. Caught only because the figure sat suspiciously
   near a 10 % bar and the matched frames were listed instead of trusted.
   **A threshold fixed in advance is no protection if the quantity measured
   against it is the wrong one.**
2. **The first stack-attribution pass silently returned zero** for every
   eval-under-ordering sample, because the profile contained no call graphs at
   all. It was caught because `perf report` independently showed
   `HandcraftedV0::undo` at 27.8 % self time, which a 0 % inclusive figure cannot
   coexist with.

## 6. NOT settled by WP-1.3 — the open debt

### 6a. Process debt

- **Pre-registration revision 2 was never reviewed.** The three review rounds
  attacked revision 1. The amendments that came out of them — the 2000-opening
  book, the 100-pair floor, the retargeted H1 attribution, the run reorder, the
  operating rules — went in on the strength of those reviews plus operator
  rulings, and then ran. No fresh-context round attacked the document that
  actually governed the experiment.
- **The book extension (D-187) never got its REVIEW-impl.** D-175 recorded that
  the generator warranted one fresh-context implementation review. Extending the
  book re-ran that generator, moved three pinned digests and added a test, with
  no review round of its own. The regeneration test and `tools/ci.sh` are the
  only things that checked it.

### 6b. Tooling defects found and NOT fixed

Each is mitigated by a rule or a habit rather than by code, so each will recur.

- **The eval weight table is not identified by content anywhere.** Two engines
  differing only in `eval_v0_weights.toml` produce byte-identical
  `experiment_sha256` and every other recorded digest, while `nelo_pair` moves by
  98 points. Every WP-1.3 result carries this limitation; the mitigation is that
  every run was started from this checkout's root. **Fix is a code change**:
  digest `eval.weights_file` in `identity_of`, or emit `id weights_sha256` from
  the handshake.
- **Engine identity is captured once and never re-checked**, while engines are
  respawned from disk for every game. A config edited mid-run is silent — a swap
  18 s into a live run produced exit 0 and a report attesting the old config.
  Mitigated by an operating rule, not by code.
- **`--out` is TOCTOU.** Two runs started before either finishes both pass the
  existence check and one silently destroys the other's report.
- **`tools/artifact_check.sh` does not catch a match report committed as `.txt`
  or `.report`.** Mitigated by naming reports `.matchlog`, which `.gitignore`
  already covers.
- **`first_player_wins` counts forfeited games as decided**, so the rate is only
  readable beside a zero `forfeits` count.
- **`openings_take` has no offset knob**, so a disjoint sample of the *same* book
  is not expressible. This forced the confirmatory run onto the other book.

### 6c. Statistical limits of what was measured

- **`ci95` at a sequential stop is anti-conservative** — measured coverage 0.868
  against 0.978 at the cap. Both Run 1 intervals are optimistic.
- **A normalized-Elo bar is set by the run's own tie rate.** 494 tied pairs
  beside 6 swept ones crosses H1. The decisive-pair count is reported for exactly
  this reason.
- **Nothing is time-matched.** The radius result is node-matched only; play mode
  is unmeasured and stays at radius 3.

### 6d. Surface that remains uncleared

- **Root cause of the dwarf-unwinding failure** on this machine.
- **A third profiling hypothesis is visible and unadjudicated**: `pistol_eval`'s
  `BTreeMap<Window, Counts>` is on 23.2 % of stacks and holds two of the six
  self-time leaders. D-114 forbids adding it to that profile after the numbers
  are in; it needs its own pre-registration.
- **D-174's remaining protocol items**: NUL bytes and invalid UTF-8 in engine
  output, `pistolok` sent twice, an engine ignoring `quit`, an engine closing
  stdin but not stdout, a grandchild holding the pipe open. Worker counts above 8
  are likewise untested — though 1, 2, 4 and 8 were cleared over a full book with
  a real early stop, and 4-worker runs of 4000 and 3182 games with the real
  engine completed cleanly with zero forfeits.
- **The tactical fixture's thresholds were pre-registered at radius 3.** It
  passes 20 of 20 at radius 2, but the thresholds were not re-derived for the new
  policy.
- **`configs/instrument_r2_v0.toml` is now value-identical to the committed
  config**, kept because completed reports cite it by path and digest.

### 6e. Deferred by operator decision

- **No head-to-head against SealBot until Stage 1's threat core lands.** At an
  11-stone midgame SealBot reaches depth 3 in 38 ms where pistol needed 84 s —
  about 2200× — while raw nps differs by only ~4×, so the gap is tree size, not
  speed. Separately, a timed match today would measure D-95 rather than strength:
  pistol's `movetime` is a floor, SealBot's is a real ceiling (~1.5 ms overshoot).
- **`hexo-bridge` is now local** at `../hexo-bridge`, and an engine is wired in as
  a `SubprocessEngine` subclass implementing two methods — so the eventual match
  is an adapter in the bridge, not a harness, and pistol needs no transport code.
  The stdio-shim spec ADR is still owed, riding along with WP-1.4.

## 7. What WP-1.3 licenses next

1. **WP-1.4 — the movetime ceiling** (D-95). Unblocks any time-matched claim,
   the play-mode radius question, and platform play.
2. **`Eval::delta`** (D-110), unlocked by H1 at 76 %, as its own work package
   with D-110's oracle test and a Hard-Rule-5 bench.
3. **WP-1.5 — the threat core**, which supersedes the radius policy entirely and
   is what makes a SealBot comparison informative.
