# WP-1.5b IMPL — REVIEW-impl + RED-TEAM (combined dispatch, D-364)

## Header

**Reviewed revision.** `38d9493654e8098743e618af1bafd9afa130c56d`
(`38d9493`, `docs(decisions): D-364 — proportionality line for combining
REVIEW-impl and RED-TEAM into WP-1.5b's Phase 4 dispatch`).

```
$ git log --oneline -1            # at entry
38d9493 docs(decisions): D-364 — proportionality line for combining REVIEW-impl and RED-TEAM into WP-1.5b's Phase 4 dispatch
$ git status --porcelain | wc -l  # at entry
0
```

At exit:

```
$ git rev-parse HEAD
38d9493654e8098743e618af1bafd9afa130c56d
$ git status --porcelain | wc -l
0
```

**Match with HEAD:** the reviewed revision still matches HEAD at exit and the
tree is clean at both ends. This session made no commits. It applied three
deliberate mutations and one instrumentation edit (all in §RED-TEAM 4 below),
each reverted with `git checkout --` and each cleanliness confirmed by
`git status --porcelain`; this worktree
(`/home/tom/Projects/HeXO-AlphaBeta/.claude/worktrees/agent-a383215a2f97832b8`)
is the separate worktree CLAUDE.md's mutation-testing rule requires, not the
live tree. The only file this session wrote to keep is this one.

**Subject.** `git log --oneline f02a4ba..HEAD` — 14 commits:

```
38d9493 docs(decisions): D-364 …
23373e3 perf(solver): D-263 remedy 1 — three-pairwise-disjoint-families early-out …
8c13251 tools+docs(solver): measure the D-263 BASELINE …
8ce13ff test+tools(search): land THE COLONY FAMILY and THE PATTERN FIXTURES UNDER STAGED …
538b3e5 test(pistol-cli): tactical_staged_v0.txt — THE TACTICAL SUITE UNDER STAGED
271cdf1 feat(tools): N-E — baseline_snapshot.sh gains a required --config seam
4355500 test(pistol-search): S-M differential gate (D-323), closing D-323's SEAM
ccc7681 docs(configs): land the four staged config documents (U3_tier_t.md §10)
ca635cf feat(pistol-engine): CandidatePolicy::Staged config schema and handshake
02333ae feat(pistol-search): stage-share counters on SearchInfo (U2-M item 2)
5f48948 test(pistol-cli): U1's gate-supersession commit — option (f) …
d6a24fd feat(pistol-search): threat-first staged pair generation for stages F and T
28efd8f build(pistol-search): take a normal dependency on pistol-solver
5aea027 feat(pistol-solver): add ThreatState::live_cells_at_count, WP-1.5b's Tier-T query
```

**Governing spec.** `docs/wp15b_impl_prompt.md` at HEAD (read in full),
`docs/decisions.md` D-341 → D-364 (read in full), `CLAUDE.md`,
`tools/SHELL_CHECKLIST.md`, `docs/experiments/wp15b_impl_prompt_REVIEW2.md`
(format and verdict convention), `docs/experiments/U4_soundness_instrument.md`
§8.4 (the mutation ledger, for M7's registered text).

**Scope note.** Both halves were run in one context under D-364. The RED-TEAM
half was executed first as adversarial probes against the shipped artefacts and
only then reconciled against the REVIEW-impl reading, so the RED-TEAM findings
below are not derived from the spec-conformance frame.

---

## VERDICT

# **FAIL**

**1 BLOCKING, 5 MAJOR, 8 MINOR, 6 REJECTED.**

The code itself is in good shape. Every gate I ran is green from my own logs,
the config layer is genuinely airtight against the 22 malformed documents I
threw at it, the rules layer refuses every edge position I built by name, the
cover-arithmetic early-out is exhaustively sound over the space I could
enumerate and is genuinely covered by an independent referent, and N-E's four
registered conditions are all paid and all reproducible. What fails is not the
implementation of the staged generator. What fails is that

1. a clause the governing prompt marks **binding** — the node protocol's own
   rule-5 registration, which §2 says IMPL may not begin without — never
   landed, and IMPL began and completed anyway (BLOCKING 1); and
2. the pre-registered consequence §3.4 fixed *before any gate ran* has **fired**
   — I measured mutation M7 SURVIVING against a GREEN four-part instrument —
   and nothing in the tree records it (MAJOR 1); and
3. the two `tier_t_*_count` keys, which are the whole of §3.4's selected shape,
   have **no behavioural test coverage whatsoever** (MAJOR 2), which is the
   mechanism behind (2).

None of the three requires reverting code. All three are answered by
measurement and by ADR lines.

---

# PART A — REVIEW-impl

## A.1 Gates I ran myself, with my own log output

Every claim in this section is my own run in this worktree at `38d9493`, not a
restatement of the implementing session's report.

**`cargo test --workspace --locked` — PASS.**

```
$ grep -c "test result: ok" test.log
130
$ grep -E "^EXIT=|test result: FAILED" test.log
EXIT=0
```

130 `test result: ok` blocks, 0 FAILED, exit 0. (This matches D-363's "130 of
130 `test result: ok` blocks, workspace-wide" independently.)

**`cargo fmt --all --check` — PASS.** Exit 0, no output.

**`cargo clippy --workspace --all-targets --locked -- -D clippy::all` — PASS.**
Completed with `Finished dev profile ... in 4.00s` and not one warning line
across all nine crates.

**`tools/staged_soundness_check.sh` — PASS, all four parts, exit 0.** My own log:

```
=== staged_soundness_check: 1/4: THE TACTICAL SUITE UNDER STAGED (tactical_staged_v0.txt, release)
selftest: 20 cases from crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt (required 20), configs: configs/tactical_staged_v0.toml, configs/gate_staged_v0.toml
… 20 × `ok` …
selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce

=== staged_soundness_check: 2/4: THE DIFFERENTIAL GATE — S-M, marked DEPENDS-OPEN-THEORY (D-321)
test the_filtered_row_matches_r1_at_every_filtered_node_of_the_corpus ... ok
test result: ok. 1 passed; 0 failed; …

=== staged_soundness_check: 3/4: THE COLONY FAMILY (six built distant-cluster cases)
test result: ok. 6 passed; 0 failed; …

=== staged_soundness_check: 4/4: THE PATTERN FIXTURES UNDER STAGED (the calculus's own named patterns)
test result: ok. 1 passed; 0 failed; …

staged_soundness_check: all four parts passed
EXIT=0
```

The four §3.3 names are all present, none stubbed, and the preflight VOID guard
fired correctly on both filesystems before any work
(`scratch_preflight: /tmp (device 38) has 7210884 KiB available, floor 1048576 KiB`).

**`tools/determinism.sh` — PASS.**

```
determinism: 20 positions, 2 budgets, config configs/gate_v0.toml
determinism: runs A and B agree (164 lines)
determinism: per-position processes agree with the session
determinism: ok — 40 searches, 20 positions, no difference outside nps/time
```

(See MINOR 5: this gate runs only at a `kind = "radius"` config.)

**`tools/config_check.sh` — PASS**, and it validates all eight engine configs
including the four new ones, with the `Staged { .. }` rendering visible:

```
config_check: 8 engine config(s), 1 weight table(s), 6 arena config(s), 1 book config(s)
ok   configs/gate_staged_v0.toml  … candidates=Staged { quiet_radius: 1, quiet_top_k: 128, widen_schedule: [256], tier_t_own_count: 2, tier_t_opponent_count: 3 } …
ok   configs/instrument_staged_v0.toml  … candidates=Staged { quiet_radius: 2, quiet_top_k: 16, widen_schedule: [32], … } …
ok   configs/play_staged_v0.toml  … candidates=Staged { quiet_radius: 3, quiet_top_k: 16, widen_schedule: [32], … } …
ok   configs/tactical_staged_v0.toml  … candidates=Staged { quiet_radius: 2, quiet_top_k: 1024, widen_schedule: [2048], … } …
```

**`tools/file_justification_check.sh` — PASS**:
`267 tracked .rs/.sh files, 32 over the cap, all justified`.
**`tools/artifact_check.sh` — PASS**: `ok (389 tracked files, none of them artifacts)`.
**`tools/decision_key_check.sh` — PASS**: `366 decision keys … no repeat outside the exemption`.
**`tools/label_consistency_check.sh` — PASS**: `6 documents … every document agrees with itself`.

## A.2 §1 (scope) — CONFORMS

Nothing outside the D-scope landed. No stage Q, no widening schedule armed
(`StagedParams` carries three fields, not five, and `instance.rs::search_policy`
drops `quiet_top_k`/`widen_schedule` at the seam **by name**, not silently). No
dominance pruning. No `pistol-eval` refactor. No committed default moved —
`instrument_v0.toml` and `play_v0.toml` are still `kind = "radius"`, verified in
the `config_check` output above.

## A.3 §2 (build order) — FIRST CLAUSE CONFORMS, SECOND CLAUSE BREACHED

**First clause — CONFORMS.** U2's IMPL lands before U1's gates are armed. Commit
order in `f02a4ba..HEAD` is `5aea027` (solver query) → `28efd8f` (the dependency
edge) → `d6a24fd` (the staged generator) → `5f48948` (U1's gate supersession).
U1's commit is strictly after the edge it adjudicates. ✅

**Second clause — BREACHED.** See BLOCKING 1.

## A.4 §3.1 N-E — CONFORMS, ALL FOUR CONDITIONS PAID AND REPRODUCED

I ran the shipped script directly rather than trusting D-359.

*Condition 1 (the digest is `$3`, not `$4`).* PAID.

```
$ tools/baseline_snapshot.sh --config configs/instrument_staged_v0.toml --ladder-depth 1 --nodes 200 --out /tmp/ne_snap.txt
rc=0
$ awk '/^config /{print "fields="NF}' /tmp/ne_snap.txt
fields=3
config /…/configs/instrument_staged_v0.toml 1f6b75df482ef2fce6e786a41b8a3de1efb8545b01d3c11e471b725ac632a21a
```

*Condition 2 (the guard is NOT a reuse of the basename loop).* PAID, and the gap
D-329 named is closed:

```
$ tools/baseline_snapshot.sh --config "/tmp/spaced dir/instrument_v0.toml" …
rc=1
baseline_snapshot: FAIL: the config path `/tmp/spaced dir/instrument_v0.toml` has a SPACE, and the whole path is written into a whitespace-delimited field of the record's invariant block, where it would shift every field after it
```

I also confirmed the guard is not narrower than the record needs — an injected
newline is refused by the printable-ASCII allow-list, on the whole path:

```
$ tools/baseline_snapshot.sh --config "$(printf 'configs/instrument_v0.toml\ninjected line')" …
rc=1
baseline_snapshot: FAIL: the config path `…/configs/instrument_v0.toml
injected line` has a character outside printable ASCII, and the whole path is written into the record's invariant `config` line
```

*Condition 3 (an item-10 driving test, two halves with a control).* PAID.
`crates/pistol-cli/tests/baseline_snapshot_tests.rs`, 37 tests, all green in my
own workspace run, including `a_missing_config_flag_is_refused_as_required`,
`a_config_path_with_a_space_in_a_directory_component_is_refused`, and the control
`the_same_config_path_shape_without_a_space_is_accepted`.

*Condition 4 (an item-12 sentence).* PAID — the usage block now names the config
refusal as a FAIL, and `the_usage_text_states_the_resolution_base_and_the_exit_status_classes`
is green.

Missing-flag refusal, by name, no default:

```
$ tools/baseline_snapshot.sh --ladder-depth 1 --nodes 200
rc=1
baseline_snapshot: FAIL: --config is required (no default): pass the document this snapshot measures, e.g. --config configs/instrument_v0.toml
$ tools/baseline_snapshot.sh --config "" …
rc=1
baseline_snapshot: FAIL: --config was given an empty value, and an empty value is not one
```

**The N-E debt (D-329, D-359) is genuinely CLOSED.** ✅

## A.5 §3.2 / §3.3 the four gates and the SEAM — CONFORM, with MAJOR 5 on the amendment mechanism

All four §3.3 names exist, wire into one script, and are green (A.1). The S-M
gate reaches `staged_candidates` through its ordinary public signature with a
bare `#[path]` include of R1 and no change to either `Cargo.toml`, exactly as
D-358 claims; I verified the include compiles and answers from
`pistol-search`'s own test tree. The gate carries a non-vacuity floor
(`filtered_nodes >= 10`) so it cannot pass by never firing — that is the right
shape and it is what CLAUDE.md's "a criterion nothing can fail" clause asks
for.

Two observations on this gate's *scope*, recorded as MINOR 8 rather than as
defects: it inspects the FILTERED row only, and it `continue`s past every other
row, so a row-classification error in the direction *shipped says Impossible,
R1 says Minimal* is skipped rather than caught. That direction is covered
elsewhere (`threat_oracle_tests.rs`, see RED-TEAM 3), but not here.

The SEAM closure itself is sound. The mechanism by which the governing prompt
was updated to record it is MAJOR 5.

## A.6 §3.4 C at the threshold reading — SHIPPED, BUT ITS PRE-REGISTERED CONSEQUENCE HAS FIRED AND IS UNRECORDED

`tier_t_own_count = 2`, `tier_t_opponent_count = 3` in all four documents; the
threshold reading is implemented at `staged.rs::tier_t_side` exactly as §3.4
states (`threshold <= 2 → live_cells_at_count(Two) ∪ live_cells_at_count(Three)
∪ threat_cells`; `3 → live_cells_at_count(Three) ∪ threat_cells`). The shape is
right.

§3.4's item 2 is a pre-registered consequence: *"If the instrument is GREEN
while mutation M7 … also SURVIVES, the instrument has demonstrated it cannot
tell A from C, C's entire ground is unmeasured, and that is recorded as such in
the results rather than read as a confirmation of C."*

The instrument is GREEN (Part A.1). I ran M7. **It SURVIVES.** See MAJOR 1 and
RED-TEAM 4.

## A.7 §4 the D-263 hotspot — THE BENCH CHAIN HAPPENED, WITH ONE MEASUREMENT GAP

I verified the chain from the commits themselves rather than from the ADR text.

- `8c13251` adds **only** `tools/staged_cover_bench.sh` (200 lines) and the
  D-362 ADR line. It touches no engine source. So the BASELINE commit is a
  measurement commit and nothing else. ✅
- `23373e3` adds **only** `crates/pistol-solver/src/cover.rs` (+122) and the
  D-363 ADR line. One change, one commit. ✅
- The subject tree for the BASELINE (`cover.rs` at `8ce13ff`) is byte-identical
  to `cover.rs` at `8c13251`, since `8c13251` does not touch it — so D-362's
  "no remedy applied yet" is true of the measured subject. ✅
- The bench reports **nps AND time-to-depth**, at **both** stone-count bands,
  IQR-gated at 10%, over `bench_positions_v1.txt`, at
  `configs/instrument_staged_v0.toml`. That is §4's registered instrument. ✅
- Remedy 1 is D-263's stated first remedy, taken first. ✅ Remedies 2 and 3 are
  named OWED in D-363, disclosed rather than dropped — not a finding.
- The bracket/threshold **timing** deviation is disclosed and reasoned in D-363.
  D-363's own flip clause invites a reviewer's judgement on it, so I record
  mine: **I do not judge the timing deviation disqualifying, and I do not vacate
  the PASS.** The rule's form is baseline-only, the 1.02 margin is stated as a
  round number above the ~1% IQR tolerance rather than fitted to the observed
  1.044–1.048, and the abort arm ("either band's nps ratio < 1.0, or a ratio the
  two runs' IQRs cannot distinguish from 1.0") is one the observed numbers could
  have failed. That is what makes it a rule and not a post-hoc justification.

The gap is MINOR 7: §4 asks IMPL to *"measure `blocking_covers` and
`min_hitting_set_exceeds` at the candidate counts its own generator produces"*,
and nothing in the tree measures those two functions' own share of a node. The
whole-search nps/ttd win is a lower bound on the arithmetic's cost, not a
measurement of it.

## A.8 §5 fixtures and configs — CONFORM

The four documents match §5's table to the digit:

| document | mode | quiet_radius | quiet_top_k | widen_schedule |
|---|---|---|---|---|
| `instrument_staged_v0.toml` | instrument | 2 | 16 | `[32]` |
| `tactical_staged_v0.toml` | instrument | 2 | 1024 | `[2048]` |
| `gate_staged_v0.toml` | instrument | 1 | 128 | `[256]` |
| `play_staged_v0.toml` | play | 3 | 16 | `[32]` |

`tactical_staged_v0.txt`: twenty cases, `require 20`, sha matches the pin
(`fbd9be4cf7fa845e0ee65894c333db63e7fbb5de0a54088857c6e5401da9f53e`, verified by
`sha256sum`), fifteen at `tactical_staged_v0.toml` and five at
`gate_staged_v0.toml`, all twenty solved through the real release binary in my
own run. ✅

`SearchInfo.stages: StageCounters` exists with the five named quantities plus
the documented sixth (`batched_quiet_safety_net`), is written on the whole-search
path (`outcome.info.stages = run.stages` at the end of `Searcher::search`, after
both salvage constructions, so no path silently reads zero), and is read by a
committed harness
(`staged_tests.rs::stage_counters_are_reported_in_search_info_and_zero_under_radius`).
The line protocol is unchanged. ✅

## A.9 §6 / §8 finish policy

| item | status |
|---|---|
| 1. every §U2-T/§U3-T/§U4-T test row passes | ✅ from my own run (but see MAJOR 2: the rows do not cover the `tier_t_*_count` semantics) |
| 2. `staged_soundness_check.sh` exists, wires all four, is SHELL_CHECKLIST-reviewed with every item answered by name, and is green | **PARTIAL** — exists ✅, wires all four ✅, green ✅ (my log); the SHELL_CHECKLIST review is a *self*-review by the implementing session (D-361), which is not the review CLAUDE.md's tools/ rule contemplates and which §6 still lists as OPEN. This review answers the items by name in §A.10 below, which discharges the item for this script but yields MAJOR 3 and MAJOR 4 |
| 3. §4's hotspot ran its registration | ✅ with MINOR 7 and the disclosed timing deviation |
| 4. `wp15b_sprt_prereg.md` passes its review + the operator's governed SPRT completes | **OPEN, correctly not claimed.** The prereg's revision 5 PASSED (`wp15b_sprt_prereg_REVIEW_rev5.md`), but that review names three preconditions for revision 5 to govern, one of which is that `configs/arena_wp15b_staged_vs_r2.toml` exists — it does not (`config_check` lists six arena configs, not including it). §5 does not list it among what IMPL produces, so this is not an IMPL gap; it is the next blocker on item 4 and is named here so it is not re-discovered |
| 5. U1's gate-supersession commit lands after U2's IMPL | ✅ (A.3) |

## A.10 `tools/SHELL_CHECKLIST.md` — items answered by name

The dispatch cites the checklist, so I answer its items for the three changed
`tools/` artefacts. `tools/ci.sh`'s change is renumbering only and raises none
of the classes.

### `tools/staged_soundness_check.sh` (new)

- **1 — discarded command-substitution status.** MET. `BUILD_LOG="$(cargo build …)" || fail`
  takes the value into a variable and refuses by name. No substitution sits
  inside a `printf` argument on a path that could fail.
- **2 — pipeline in statement position.** MET. The only pipelines are
  `mapfile -t BUILT < <(sed …)` (process substitution, cross-checked against
  `NAMED` immediately after) and the `grep -c` below.
- **3 — grep under pipefail.** MET. `NAMED="$(grep -c '"executable":"' <<<"$BUILD_LOG" || true)"`
  carries `|| true` and its *spelling* is then validated
  (`case "$NAMED" in *[!0-9]* | "") fail …`), which is item 8 as well.
- **4 — LC_ALL and guard direction.** N/A. No character class is used as a
  correctness guard here.
- **5 — index vs worktree.** N/A. This script reads no tracked-file set.
- **6 — sweep by prefix.** N/A. Nothing is deleted.
- **7 — traps.** N/A. No EXIT trap.
- **8 — one spelling per number, one refusal per reason.** MET, and well: five
  distinct refusals for the artifact path (`-e`, `-f`, `-x`, none, several),
  each with its own message, plus the count-spelling check.
- **9 — caller-controlled values reaching a record.** N/A. This script takes no
  arguments.
- **10 — THE COVERAGE RULE.** **NOT MET, and D-361's exemption argument does not
  hold.** See MAJOR 4.
- **11 — caller path feeding a delete or overwrite.** N/A. No `rm`, `mv` or
  caller-supplied write target.
- **12 — VOID vs FAIL.** Obligations 1 and 2 MET (exit 0/1/2 declared in the
  usage block; `scratch_preflight.sh` runs on both filesystems and voids early —
  I saw it fire). Obligation 3 ("the distinction survives the seam") NOT MET:
  nothing drives this gate, so no test asserts on the code it expects or says
  what the other codes would have meant. This is item 10's consequence, folded
  into MAJOR 4.

### `tools/staged_cover_bench.sh` (new — produced D-362's and D-363's numbers)

- **1 — discarded command-substitution status.** ONE INSTANCE, cosmetic:
  line 123's `echo "… ($(grep -c early "$WORK/bands") early, …)"` discards
  status inside an `echo` argument. MINOR 3. The load-bearing substitutions
  (`nn=$(band_metric … | stats)`) do propagate: `band_metric`'s `fail` exits the
  pipeline's left subshell, `pipefail` carries it to the assignment, and `set -e`
  kills the script *after* the named message is printed. Correct.
- **2 — pipeline in statement position.** ONE VIOLATION, line 107–108. MINOR 1.
- **3 — grep under pipefail.** ONE VIOLATION, the same line — an unguarded
  `grep '^id '` in statement position. MINOR 1. The other two greps are guarded
  (`|| true`) or validated.
- **4 — LC_ALL.** N/A.
- **5 — index vs worktree.** N/A.
- **6 — sweep by prefix.** MET. `WORK="$(mktemp -d)"`, script-created.
- **7 — traps.** MET. One `trap 'rm -rf "$WORK"' EXIT`, single command, no
  status-clobbering housekeeping after it.
- **8 — one spelling per number.** **NOT MET** for `REPS`. MINOR 2.
- **9 — caller-controlled values reaching a record.** MET in effect: the only
  caller-supplied value is `REPS`, and it is echoed into the record line — which
  is exactly why item 8's spelling gap matters (MINOR 2).
- **10 — THE COVERAGE RULE.** **NOT MET. This is the binding one.** MAJOR 3.
- **11 — caller path feeding a delete or overwrite.** MET. The one destructive
  site is `rm -rf "$WORK"`, and `$WORK` originates from `mktemp -d`. Traced, not
  recalled.
- **12 — VOID vs FAIL.** Obligations 1 and 2 MET. Obligation 3 NOT MET — no test
  drives it (MAJOR 3).

### `tools/baseline_snapshot.sh` (reopened by N-E)

- **1** MET (the guard reads `$CONFIG` as a value, refuses by name).
- **2, 3** N/A to the diff.
- **4 — LC_ALL and guard direction.** MET, and this is the item that matters
  here: the guard is written as an ALLOW-LIST (`*[![:print:]]*`), which is the
  direction the checklist prescribes, so a locale pin makes the refusal wider,
  not narrower. Reproduced live against a newline (§A.4).
- **5, 6, 7** N/A to the diff.
- **8 — one refusal per reason.** MET: missing flag, empty value, non-printable,
  and space are four distinct refusals with four distinct messages. All four
  reproduced.
- **9 — caller-controlled values reaching a record.** MET, and this is N-E's
  whole point: the WHOLE path reaches the `config` line and the WHOLE path is
  guarded, checked before `caller_path` rewrites the variable.
- **10 — coverage.** MET. Three new driving tests plus a control, in a suite CI
  runs.
- **11 — caller path feeding a delete or overwrite.** MET. `--config` feeds a
  `[ -f ]`, a `sha256sum` and an `echo`; no destructive site.
- **12 — VOID vs FAIL.** MET, and condition 4 is exactly this item: the usage
  block now states that a missing or malformed `--config` is a FAIL and that the
  script declares no VOID class.

## A.11 Config law (CLAUDE.md rule 1) — CONFORMS, verified adversarially

`CandidatePolicy` is `#[serde(tag = "kind", deny_unknown_fields)]`; the `Staged`
variant carries all five keys §10 states; there is no `#[serde(default)]`
anywhere on the new schema; and validation lives in `pistol-engine/validate.rs`
with `check_radius` shared so `Radius`'s and `Staged`'s radius bounds cannot
drift. The cross-field rule (`widen_schedule` entries strictly increasing AND
strictly greater than `quiet_top_k`) is implemented and is the one revision 3
lacked. `Searcher::new` independently re-refuses a `tier_t_*_count` outside
`2..=3`, so the search layer does not rely on the config layer having checked.
See RED-TEAM 1 for the 22-document adversarial sweep. ✅

## A.12 Determinism law (CLAUDE.md rule 4) — the property HOLDS; the CI gate does not cover it

I read every choice path the staged dispatch touches for order-dependence:

- `tier_f`: `win_in_one_ply_cells` → `fill_empties`, which ends
  `out.sort_unstable(); out.dedup()`. At `StonesLeft::Two` the union is
  re-sorted and deduped. Deterministic on both arms.
- `filtered`: `sort_unstable` + `dedup` over the cover union.
- `tier_t_union` / `tier_t_side`: `sort_unstable` + `dedup` on both sides.
- `within_radius` (the safety net): `BTreeSet`, ascending by construction.
- `delta_rank`: `sort_by_key(Reverse(score))`, a **stable** sort over an
  ascending-coordinate input — same tie-break `ordering::order` uses.
- `blocking_covers`: `minimal.sort_unstable()` before return, and
  `three_pairwise_disjoint_families` returns a `bool`, so the `HashMap`-backed
  window table's iteration order is unobservable (and
  `window_map_ordering_is_unobservable` already pins that).
- No clock is read on any staged choice path.

Empirically, my own cross-process probe at `configs/instrument_staged_v0.toml`
over the whole `bench_positions_v1.txt` corpus at two budgets (46 searches per
run, two separate processes, plus one process per position):

```
lines A=230 B=230 C=135
A==B OK
```

**The property holds.** The gap is that no CI gate checks it — MINOR 5.

---

# PART B — RED-TEAM

## B.1 Config parsing — 22 documents, ALL REFUSED BY NAME. No finding.

I generated 21 malformed/boundary staged documents plus a control and ran each
through the real release binary. Every one of the 21 exits 2 with a named,
key-attributed refusal; the control exits 0.

```
control                rc=0
unknown_field          rc=2   `search.candidate_policy.nonsense_key`: unknown field `nonsense_key`, expected one of `quiet_radius`, `quiet_top_k`, `widen_schedule`, `tier_t_own_count`, `tier_t_opponent_count`
kind_typo              rc=2   `search.candidate_policy.kind`: unknown variant `stagged`, expected `radius` or `staged`
missing_opp            rc=2   `search.candidate_policy.tier_t_opponent_count`: missing field `tier_t_opponent_count`
missing_schedule       rc=2   (missing field `widen_schedule`)
radius_zero            rc=2   `search.candidate_policy.quiet_radius`: must be in 1..=64, got 0
radius_huge            rc=2   (4294967295, same key)
own_count_0/1/4        rc=2   `search.candidate_policy.tier_t_own_count`: must be 2 or 3 — LAW-SUPPORT's threshold reading admits no other count (U3_tier_t.md §6.1), got N
own_count_256          rc=2   `search.candidate_policy`: invalid value: integer `256`, expected u8
own_count_neg          rc=2   (invalid type for u8)
own_count_float        rc=2   `search.candidate_policy`: invalid type: floating point `2.0`, expected u8
opp_count_1            rc=2   (same rule, opponent key)
top_k_zero             rc=2   `search.candidate_policy.quiet_top_k`: must be at least 1, got 0
top_k_u64max           rc=2   `<document>`: TOML parse error … u64 value was too large
schedule_empty         rc=2   `search.candidate_policy.widen_schedule`: must be non-empty
schedule_decreasing    rc=2   … got 32 after 64
schedule_dup           rc=2   … got 32 after 32
schedule_equal_topk    rc=2   … strictly greater than quiet_top_k (16), got 16 after 16
schedule_below_topk    rc=2   … strictly greater than quiet_top_k (64), got 32 after 64
radius_extra_staged    rc=2   `search.candidate_policy.quiet_top_k`: unknown field `quiet_top_k`, expected `radius`
```

`deny_unknown_fields` really works through the internally tagged enum (the
`unknown_field` and `radius_extra_staged` rows prove it in both directions).
Missing keys are errors, not defaults. The cross-field widening rule catches the
exact `quiet_top_k = 64` / `[32]` case D-356 names, and also the equal case. No
finding, and this is the strongest part of the change.

Reproducer:
`/tmp/claude-1000/…/scratchpad/cfg_redteam.sh` and `cfg_msgs.sh`.

## B.2 Staged generator edge positions — no finding

**The quiet-ball safety net at game start.** I tried to make it emit nothing.
`staged_candidates` on a new game: `StonesLeft::One`, `can_win_this_turn` →
`None`, `blocking_covers` → `NothingToBlock` (the opponent owns no window),
`tier_t_union` → empty, so the net fires and calls `within_radius(board,
quiet_radius)`, whose empty-board arm is `legal_placements(board)` — the origin.
Non-empty by construction, at every radius, for every `quiet_radius >= 1` the
validator admits. Live, through all four staged documents:

```
instrument_staged_v0  newgame + go depth_turns 2 → bestmove 0,0
instrument_staged_v0  newgame + go nodes 1       → bestmove 0,0
tactical_staged_v0    …                          → bestmove 0,0
gate_staged_v0        …                          → bestmove 0,0
play_staged_v0        …                          → bestmove 0,0
```

No panic, no `NO_MOVE_FROM_A_COMPLETED_ITERATION`, no
`NO_CANDIDATES_MID_TURN`. REJECTED as a finding.

**`HitBudget::Zero` interacting with the early-out.** Unreachable from the
staged path: `HitBudget::from(StonesLeft)` only ever yields `One` or `Two`, and
`blocking_covers` returns `Impossible` on `Zero` *before* reaching
`empty_families`, so the early-out is never consulted at that budget. And even
if it were, `Impossible` is the correct answer at `Zero` whenever a hot window
stands. REJECTED.

**Radius-8 legal-region boundary and lattice edges.** Every construction I built
is refused by `pistol-core`'s own rules, identically under staged and radius —
CLAUDE.md rule 2 holds, no staged-specific weakness:

```
colonies 16 apart  → IllegalPosition: cell 16,0 is outside the legal region: a stone must be placed within hex-distance 8 of some stone already on the board   (staged AND radius, identical)
32000,0 first stone→ IllegalPosition: the first stone of the game goes on 0,0, not on 32000,0 …
phase-1 root       → IllegalPosition: those stones leave p1 to move at phase 0, and the document says p1 at phase 1
decided root       → IllegalPosition: p1 completed a line on turn 7: a won position is terminal … (rule 4)
bad parity         → IllegalPosition: … which is not a turn structure any game reaches (rule 3: one stone on turn 1, two on every turn after)
```

REJECTED as findings. Rule 3 (fail loud) is upheld throughout.

## B.3 The `three_pairwise_disjoint_families` early-out — the highest-value target, and it survives everything I could throw

This was the assignment's named top priority. Three independent attacks:

**(a) Is the necessary condition actually sound?** The claim is: three pairwise
disjoint families force a hitting set of size ≥ 3, which exceeds every
`HitBudget` (closed at two). I tested this exhaustively rather than by argument.
I added a temporary `#[cfg(test)]` sweep over **every** collection of four
families drawn from the 15 possible 1- and 2-cell families over a 5-cell
universe — 50 625 collections — asserting that whenever the early-out fires, no
1- or 2-cell set covers all families:

```
REVIEWER: checked 50625 collections, early-out fired 13140
test cover::tests::reviewer_exhaustive_early_out_never_claims_impossible_when_a_two_cell_cover_exists ... ok
```

13 140 firings, zero false positives. Reverted before finishing. The edge cases
I specifically hunted for — an empty family (disjoint from everything, but an
empty family means an unhittable window, so `Impossible` is right anyway),
duplicate families, and three families of which only two are pairwise disjoint —
are all inside that sweep. **REJECTED.**

**(b) Does the regression D-363 cites as its correctness proof actually exercise
the early-out, or is that claim vacuous?** This was my strongest prior. It is
**not** vacuous. I instrumented the helper with an `eprintln!` on the firing
branch and counted:

```
cargo test -p pistol-solver -- --nocapture              → 2154 firings
cargo test -p pistol-solver --test threat_oracle_tests  → 2049 firings
cargo test -p pistol-search --test staged_differential_gate_tests → 6 firings
```

`threat_oracle_tests::threat_incremental_matches_reference_on_random_playouts`
asserts equality of the shipped `blocking_covers` **and**
`min_hitting_set_exceeds` against R1 — an independently written brute-force
referent — at every ply of seeded playouts, for both sides and both budgets. It
crosses the early-out 2 049 times. D-363's claim stands. Instrumentation
reverted. **REJECTED.**

**(c) Mutation.** See B.4 mutation 1: the R1 oracle kills it on a real position.

## B.4 Mutation testing — three mutations applied in this worktree, all reverted

### Mutation 1 — weaken the pairwise requirement in `three_pairwise_disjoint_families`

`crates/pistol-solver/src/cover.rs`: `if disjoint(&families[i], family_k) && disjoint(&families[j], family_k)`
→ `if disjoint(&families[i], family_k)`. This makes the early-out claim
`Impossible` for triples that are not pairwise disjoint — a false-positive
unblockable-threat claim, i.e. a mate score for the wrong side.

**CAUGHT, twice, and by an independent referent:**

```
test cover::tests::a_third_family_sharing_a_cell_with_one_of_two_disjoint_families_is_not_three_pairwise_disjoint ... FAILED

test threat_incremental_matches_reference_on_random_playouts ... FAILED
assertion `left == right` failed: seed 4 ply 548 p1: min_hitting_set_exceeds at Two
  left: true
 right: false
```

The second is the one that matters: R1, a separate implementation, disagrees on
a position a seeded playout actually reaches. **Reverted** (`git checkout --`,
`git status --porcelain` empty).

### Mutation 2 — delete the opponent half of `tier_t_union`

`crates/pistol-search/src/staged.rs`: `cells.extend(opponent)` → dropped. This
removes every opponent-side Tier-T cell from every BATCHED and BATCHED-lost node
— i.e. `tier_t_opponent_count` is rendered inert.

**SURVIVES.** `cargo test --workspace --locked`: zero failures. All four parts of
`tools/staged_soundness_check.sh`: green, `all four parts passed`. **Reverted.**

This is MAJOR 2.

### Mutation M7 (registered, `U4_soundness_instrument.md` §8.4) — Tier T at ≥3 for the mover, option A

`crates/pistol-search/src/staged.rs::tier_t_side`: `if threshold <= 2` →
`if false && threshold <= 2`, so own windows qualify at ≥ 3 regardless of the
configured `tier_t_own_count = 2`. This is exactly option A against the selected
option C.

**SURVIVES.** `cargo test --workspace --locked`: zero failures.
`tools/staged_soundness_check.sh`: `all four parts passed`,
`selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce`.

The mutation is *not* inert — it changes the search materially, visible in the
node counts of the very same fixture:

| case | shipped | under M7 |
|---|---|---|
| `must_block_p2_five_in_a_row` | `bestmove -1,4/5,1` nodes 134 | `bestmove -2,4/5,1` nodes 57 |
| `must_block_p2_gap_fill` | `bestmove 2,2/3,1` nodes 749 | `bestmove -2,4/3,1` nodes 60 |
| `mate_in_3_double_three_becomes_double_four` | nodes 287007 | nodes 27025 |
| `quiet_two_short_clusters` | `bestmove -1,1/5,1` score 180 | `bestmove -1,1/5,1` score 148 |

The generator emits different sets, chooses different moves, and returns
different scores — and the whole four-part instrument, plus 130 test-result
blocks, cannot tell. **Reverted.** This is MAJOR 1.

### Mutation 4 — edit the pinned fixture

Appended two lines to `crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt`.
**CAUGHT:**

```
test tactical_staged_v0_fixture_is_pinned_and_every_position_is_legal ... FAILED
assertion `left == right` failed: tactical_staged_v0.txt changed; update its pinned sha in the same commit
```

**Reverted.** CLAUDE.md rule 7's pin works.

## B.5 Fixture malformation — no finding, rule 3 upheld

Every corruption fails loudly with a named error and a non-zero exit:

```
truncated to 120 lines  rc=2  `require 20` asks for more passes than the 3 cases stated
truncated to 105 lines  rc=2  `require 20` asks for more passes than the 1 cases stated
require raised to 21    rc=2  `require 21` asks for more passes than the 20 cases stated
empty file              rc=2  no `require` line: the pass threshold is pre-registered in the fixture, not chosen by whoever runs it
absent file             rc=2  cannot read: No such file or directory (os error 2)
config line → nonexistent config  rc=2  config: `configs/nope_staged_v0.toml`: cannot read: No such file or directory (os error 2)
one case given an unmeetable `expect cell`  rc=1  selftest: 18 of 20 cases solved (required 20)
eleven cases given `expect mate 7`          rc=1  selftest: 9 of 20 cases solved (required 20)
```

No silent pass, no skip-with-default. **REJECTED as findings.**

One structural note, not a defect: `tools/staged_soundness_check.sh` checks only
`[ -f "$FIXTURE" ]` and does not verify the fixture's digest — the pin lives in
the Rust test, which runs under a different gate (`cargo test`, gate 3). That is
the same split `tools/tactical_check.sh` already uses, so it is precedent, not
drift.

## B.6 Movetime under `play_staged_v0` — attempted, REJECTED

Under `Staged`, `ordering::order` is never called (by design, §5.4), which also
means the ordering loop's wall-clock deadline check (`self.order_deadline()`)
never runs — one fewer clock check per node on the exact class D-95 names. And
`tools/movetime_check.sh` (gate 12) runs only at `configs/play_v0.toml`, so
`configs/play_staged_v0.toml` is not covered by any ceiling gate. I tried to
turn that into an overshoot:

```
                       movetime 50        movetime 100       movetime 200
configs/play_v0.toml   max 50, over 0     max 100, over 0    max 200, over 0
configs/play_staged…   max 50, over 0     max 100, over 0    max 200, over 0
```

12 positions × 3 budgets, ceiling `N + 50`. Zero overshoots under either policy.
**REJECTED** — recorded with the attempted reproducer, per §Process. The
*coverage* gap is MINOR 6.

---

# FINDINGS

## BLOCKING 1 — §2's second binding clause was never satisfied: the node protocol's own rule-5 registration does not exist, and IMPL began and completed U2's shape without it

**Category:** BLOCKING. **Status:** CONFIRMED. **Not disclosed anywhere** in
D-341 → D-364 or in any commit message.

`docs/wp15b_impl_prompt.md` §2, second binding clause, verbatim:

> **rule 5 is undischarged for the node protocol itself** — the change that puts
> `can_win_this_turn` and `blocking_covers` on every node has no expected-gain
> bracket, no abort threshold and no IQR-gated bench of its own, distinct from
> §4's cover-arithmetic hotspot. This is a rule-5 registration **the architect
> must place before U2's IMPL starts**, not a repair IMPL or a carve may write.
> IMPL does not begin U2's shape until this registration lands.

This clause is not incidental — `wp15b_impl_prompt_REVIEW2.md` records it as the
closure of that review's MAJOR 5, promoted from §6 pointer status into §2's
*binding* order specifically so it could not be read as advisory.

**No such registration landed.** U2's shape landed at `d6a24fd` and the whole
D-scope is complete at HEAD.

**Reproducer.**

```
$ python3 - <<'PY'   # every ADR key, whole log, matching a rule-5 vocabulary
… for lines matching ('node protocol'|'per-node') AND ('bracket'|'abort'|'rule 5'|'hotspot') …
PY
D-76   (WP-1.2, the eval apply/undo roundtrip)
D-114  (the flamegraph pre-registration)
D-193  (WP-1.3 run 2, H2 not confirmed)
D-263  (the cover arithmetic — §4's hotspot, explicitly "distinct from" this one)
D-362  (that hotspot's BASELINE)
```

Nothing else. `grep` for `"rule-5 registration"` / `"rule 5 registration"` across
`docs/` returns only `U2_node_protocol.md:835` (the OPEN item itself),
`wp15b_impl_prompt_REVIEW.md`, `wp15b_design_rev7_REVIEW.md` and
`wp15b_impl_prompt_REVIEW2.md:119` — i.e. only the places that say it is owed,
never a place that discharges it.

And D-362/D-363 cannot be read as discharging it. §4 says the two registrations
are **distinct**, and `tools/staged_cover_bench.sh` measures **one** tree state
absolutely, at `configs/instrument_staged_v0.toml` only. There is no measurement
anywhere in the tree of Staged against Radius — so the cost of carrying a
`ThreatState` in `Position` and calling `can_win_this_turn` + `blocking_covers`
at every node, which is a whole-search perf change to the SPRT's own challenger,
has no bracket, no abort threshold and no bench, exactly as §2 states.

**Why BLOCKING and not MAJOR.** §2 is the one section the governing prompt marks
"binding; review order was free, this is not", and its own text states a
precondition on beginning, not a deliverable. IMPL honoured the first binding
clause and not the second. A finish policy that admits an unsatisfied binding
precondition is not a finish policy.

**Fairness note, and the remedy.** This registration is explicitly *the
architect's*, "not a repair IMPL or a carve may write" — so this is not the
implementing session having done something wrong so much as having proceeded
without a precondition it was forbidden to supply itself. **No code need be
reverted.** The remedy is (a) an architect ADR line placing the registration,
and (b) the bench it registers — which is cheap, because
`tools/staged_cover_bench.sh` already exists and needs only a second
configuration (`configs/instrument_v0.toml`) to produce the Staged-vs-Radius
comparison the bracket would be stated against.

---

## MAJOR 1 — §3.4's pre-registered consequence has FIRED and is unrecorded: mutation M7 SURVIVES against a GREEN instrument

**Category:** MAJOR. **Status:** CONFIRMED by my own measurement.

`docs/wp15b_impl_prompt.md` §3.4, item 2 — "Pre-registered consequence, fixed
before any gate runs":

> If the instrument is GREEN while mutation M7 (`U4_soundness_instrument.md`
> §8.4; Tier T at ≥3 for the mover — option A) also SURVIVES, the instrument has
> demonstrated it cannot tell A from C, C's entire ground is unmeasured, and that
> is recorded as such in the results rather than read as a confirmation of C.

`U4_soundness_instrument.md` §8.4's own ledger row: *"M7 | Tier T qualifies at ≥3
for the mover (option A) | informative | survival is a recorded finding under
**U3** §6.5's second branch, with a diagnosis, per D-281."*

The instrument is GREEN (Part A.1, my own log). I ran M7. **It SURVIVES.**
Nothing in D-341 → D-364, in any commit message, or anywhere in the tree records
M7 having been run at all.

**Minimal reproducer.** In `crates/pistol-search/src/staged.rs::tier_t_side`:

```rust
-    if threshold <= 2 {
+    if false && threshold <= 2 {          // M7: option A, Tier T at >= 3 for the mover
         threats.live_cells_at_count(side, LiveCount::Two, &mut scratch);
         out.extend_from_slice(&scratch);
     }
```

Then:

```
$ cargo test --workspace --locked          # zero failures
$ tools/staged_soundness_check.sh
selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce
… parts 2/3/4 all `test result: ok` …
staged_soundness_check: all four parts passed
```

Non-inertness of the mutant is established by the node-count and bestmove table
in B.4 (e.g. `mate_in_3_double_three_becomes_double_four`: 287 007 nodes shipped
vs 27 025 under M7; `must_block_p2_gap_fill`: `bestmove 2,2/3,1` vs
`bestmove -2,4/3,1`).

**Consequence, per §3.4's own registered text, which I am not free to soften:**
the four-part instrument cannot tell option A from option C, so **C's entire
ground is unmeasured**, and that must be recorded as such rather than read as a
confirmation of C. The gates' greenness is not evidence for the threshold
reading.

This does not by itself say C is wrong. It says the instrument that shipped
alongside C is silent about C.

---

## MAJOR 2 — `tier_t_opponent_count` (and `tier_t_own_count`) have no behavioural test coverage at all; deleting Tier T's whole opponent half passes every gate

**Category:** MAJOR. **Status:** CONFIRMED by mutation. This is the mechanism
behind MAJOR 1.

**Minimal reproducer.** In `crates/pistol-search/src/staged.rs::tier_t_union`:

```rust
-    cells.extend(opponent);
+    let _ = opponent;                     // the opponent half of Tier T, deleted
     cells.sort_unstable();
```

```
$ cargo test --workspace --locked          # zero failures
$ tools/staged_soundness_check.sh          # all four parts passed
```

**Why nothing catches it.** Every behavioural test in the tree instantiates
`StagedParams` at exactly one point, `(own = 2, opponent = 3)`:

```
crates/pistol-search/tests/staged_tests.rs        params(2, 2, 3)  ×6
crates/pistol-search/tests/staged_colony_family_tests.rs      tier_t_own_count: 2, tier_t_opponent_count: 3  ×2
crates/pistol-search/tests/staged_differential_gate_tests.rs  tier_t_own_count: 2, tier_t_opponent_count: 3
crates/pistol-search/tests/staged_pattern_fixture_tests.rs    tier_t_own_count: 2, tier_t_opponent_count: 3
```

No test asserts any behavioural *difference* between threshold 2 and 3, on either
side. The only tests naming these keys elsewhere
(`crates/pistol-engine/tests/config_validate_tests.rs`) are range checks on the
config schema — they assert the validator refuses 1 and 4, never that the numbers
mean anything.

Compounding it: the S-M differential gate exercises **only the FILTERED row**
(`if row != StagedRow::Filtered { continue; }`), and Tier T is generated only on
the BATCHED and BATCHED-lost rows — so the one gate with an independent referent
is structurally blind to Tier T's contents by construction.

The result is that `tier_t_own_count` and `tier_t_opponent_count` are shipped as
config-visible tunables, are the entire content of §3.4's landed selection, and
carry zero behavioural coverage. Any future change to their semantics — including
the option-B widening §3.4 branch 1 pre-registers as the repair if C proves too
narrow — would land against a green suite that cannot see it.

**What would close it:** one test per side asserting that the emitted Tier-T set
at threshold 2 is a strict superset of the set at threshold 3 on a position where
a live-2 window exists, and one asserting the opponent half contributes at least
one cell no own-side query produces. Both are cheap and both would have killed
mutation 2 and mutation M7.

---

## MAJOR 3 — `tools/staged_cover_bench.sh` produced D-362's and D-363's recorded numbers and carries no driving test, and no SHELL_CHECKLIST review exists for it

**Category:** MAJOR. **Status:** CONFIRMED. **Not disclosed** — unlike
`staged_soundness_check.sh`, no ADR line claims or argues an exemption for this
script.

`tools/SHELL_CHECKLIST.md` item 10, which CLAUDE.md's §Process names as *"the
binding one"*:

> **Any `tools/` script that produces a recorded number carries at least one
> test.** Not a self-test inside the script — a test in a suite CI runs, driving
> the SHIPPED script … `tools/bench_delta.sh` produced this project's OFFICIAL
> perf verdict (D-220) with zero tests until D-231. … A number nothing tests is a
> number nothing defends.

`tools/staged_cover_bench.sh` produced every number in D-362 (`nps median
128054.4 (IQR 1273.2)`, `time-to-depth-2 median 164.0 ms (IQR 4.0)`, and the late
band's pair) and every number in D-363 (`133683.8`, `156.0`, `115912.7`, `238.0`,
and the four ratios `1.044 / 1.048 / 1.051 / 1.050`), and D-363's **PASS verdict
rests entirely on them**. It is the same role, and the same risk, that item 10
was written about `bench_delta.sh` for.

**Reproducer.**

```
$ grep -rn "staged_cover_bench" --include=*.rs .
(no matches)
$ ls crates/pistol-cli/tests/ | grep -i cover_bench
(nothing)
$ ls crates/pistol-cli/tests/bench_delta_tests.rs
crates/pistol-cli/tests/bench_delta_tests.rs      # the precedent, per D-231
```

The only references to the script anywhere are inside the script itself and in
D-362/D-363's own prose.

Item 12 obligation 3 fails with it: the script declares a VOID class (exit 2)
that nothing ever drives, so the FAIL/VOID distinction does not survive the seam
— which is precisely the D-281/D-285 reading failure item 12 exists to prevent.

And under CLAUDE.md's instrument clause — *"an artefact that produces a
registered number is named in the pre-registration WITH ITS REVISION, and a
change to it reopens the review exactly as an amendment to the document does"* —
this artefact has never had a SHELL_CHECKLIST review of any kind. §6's OPEN list
names `staged_soundness_check.sh` and the reopened `baseline_snapshot.sh`; it
does not name this script, because the script did not exist when §6 was written.
This review supplies that checklist reading (§A.10), which turns up MINOR 1, 2
and 3 — three genuine class-1/3/8 items in a script whose numbers a landed ADR
verdict rests on. That is the argument for item 10, made concrete on this
script's first review.

---

## MAJOR 4 — D-361's item-10 exemption for `tools/staged_soundness_check.sh` rests on the wrong precedent, and the right one is in-tree and says the opposite

**Category:** MAJOR. **Status:** CONFIRMED. This is a **specific objection to
reasoning D-361 does disclose**, not a re-discovery of it.

D-361 states the script is:

> EXEMPT from item 10, the coverage rule, on the same precedent `tools/ci.sh`
> itself already sets: neither script produces a recorded number, both are
> PASS/FAIL/VOID orchestrators over parts that are each independently tested,
> and `tools/ci.sh` carries no driving test of its own in this tree.

`tools/ci.sh` is not this script's precedent. `tools/tactical_check.sh` is —
`staged_soundness_check.sh`'s part 1 is that script's artifact-resolution ladder
and its `selftest --fixtures` invocation, copied nearly line for line, down to
the refusal messages. And `tools/tactical_check.sh` **does** carry a driving
test, added for exactly this reason. Its own test file's header states the
governing reading verbatim:

> `crates/pistol-cli/tests/tactical_check_gate_tests.rs:1-12`
> "NOTHING IN THIS REPOSITORY DROVE THIS SCRIPT. Its stdout carries `selftest: 20
> of 20 cases solved (required 20), 0 failed to reproduce` and its exit status IS
> the gate verdict, **which is a recorded number under `tools/SHELL_CHECKLIST.md`'s
> coverage rule however the number is arrived at — the rule asks what a script
> RECORDS, not whether it computed the digits itself** (docs/decisions.md D-240,
> D-250). It went untested until a `tools/`-scoped review found the class the
> checklist opens with in it: EXIT-0-WRONG-ANSWER."

`tools/staged_soundness_check.sh` records the identical line — I saw it in my own
run: `selftest: 20 of 20 cases solved (required 20), 0 failed to reproduce`. By
the reading D-240/D-250 established and this repository's own test file states,
that is a recorded number and item 10 applies.

The defect that motivated `tactical_check_gate_tests.rs` — a stale binary at a
redirected target directory letting the gate print `20 of 20` and exit 0 for an
engine that fails the suite — is reachable in `staged_soundness_check.sh` by
construction, because it is the same ladder. The ladder looks correct to me on
reading, but "looks correct on reading" is what the precedent explicitly rejects.

**Reproducer.**

```
$ ls crates/pistol-cli/tests/ | grep -E "staged_soundness|tactical_check"
tactical_check_gate_tests.rs        # exists
                                    # no staged_soundness_check_tests.rs
$ grep -rn "staged_soundness_check" --include=*.rs .
(no matches)
```

**Bounded objection.** Parts 2–4 of the script are thin `cargo test` wrappers and
the ci.sh precedent is fair for those. It is **part 1** — the binary resolution
plus the recorded selftest line — that the exemption does not reach.

---

## MAJOR 5 — the governing IMPL prompt was amended in place by the implementing session, mid-IMPL, relaxing its own finish policy, with no fresh review, no ADR line, and no revision label to bump

**Category:** MAJOR. **Status:** CONFIRMED.

`docs/wp15b_impl_prompt.md` passed REVIEW2 at `f3752c3`. It was then edited at
commit `4355500` — the fourth IMPL commit, by the implementing session — in three
places:

```
$ git log --oneline f02a4ba..HEAD -- docs/wp15b_impl_prompt.md
4355500 test(pistol-search): S-M differential gate (D-323), closing D-323's SEAM
```

The substantive change is in §8, and it is a **relaxation of the finish policy**:

```
-**Finish DOES require §3.2/§3.3's differential-gate SEAM decision to land, with
-no excusing clause:** … If the SEAM is still open, IMPL is not finished — full
-stop. The SEAM decision is the architect's to make, and it sits on IMPL's
-critical path exactly because item 2 admits no exception for it.
+**§3.2/§3.3's differential-gate SEAM decision — CLOSED, D-353/D-358.** … so item
+2 is satisfiable with all four parts wired, none stubbed.
```

Three problems, none of which touch the *substance* (I agree with D-358 that the
SEAM is genuinely closed — the entry point was made public at `d6a24fd` for
exactly this consumer, and the gate reaches it through nothing but that public
signature):

1. **CLAUDE.md:** *"an amendment reopens the review, however small the diff …
   Reviews of superseded revisions do not transfer."* The amended prompt governs
   ten subsequent IMPL commits and has never been reviewed at the revision that
   governs them. (This dispatch is the first reading of the amended text, and it
   is a REVIEW-impl of the code, not a REVIEW of the amended prompt.)
2. **D-311:** *"A REVISION LABEL BUMPS ON ANY APPEND TO A REVIEWED DOCUMENT …
   It binds every document a review is dispatched against."* This document
   carries **no revision label at all**, so the bump has no vehicle and "the
   revision REVIEW2 passed" and "the revision that governed commits 5–14" are the
   same name for two different texts — precisely the ambiguity D-311 exists to
   remove, and the same ambiguity that cost the sixth review round a header
   paragraph.
3. **CLAUDE.md rule 10 / D-342's no-silent-drift discipline:** no ADR line
   records that the governing prompt was amended. D-358 records the SEAM closure
   and says nothing about editing §3.2, §3.3, §6 and §8 of the document that
   governs the work. `4355500`'s commit message names the gate, not the
   amendment.

The commit that removed "IMPL is not finished — full stop" from the finish policy
was made by the session the finish policy binds. Even where that is the right
call on the merits, it is the shape the process exists to make visible, and here
it was not visible in the log at all.

**Reproducer:** `git diff f02a4ba..HEAD -- docs/wp15b_impl_prompt.md` (45 lines
changed); `git log --oneline f02a4ba..HEAD -- docs/wp15b_impl_prompt.md` (one
commit, `4355500`, an IMPL commit); `grep -n "u-rev\|revision" docs/wp15b_impl_prompt.md`
finds no self-label; `grep -n "wp15b_impl_prompt" docs/decisions.md` finds
citations of the document but no line recording its amendment.

---

## MINOR 1 — `tools/staged_cover_bench.sh:107-108`: unguarded `grep` in statement position under `pipefail` (SHELL_CHECKLIST item 3)

**Status:** CONFIRMED, reproduced in isolation.

```bash
printf 'pistol\nquit\n' | timeout 60 "$ENGINE" --config "$CONFIG" 2>/dev/null |
	grep '^id ' | sed 's/^/staged_cover_bench: identity /'
```

Statement position, no `|| true`, no `fail`. If the engine emits no `id ` line —
a rejected config, a handshake change, a `timeout` kill — `grep` exits 1,
`pipefail` propagates it, and `set -e` kills the bench **with no message at all**,
after it has already printed the instrument digests. That is item 3's exact class
and rule 3's "name the refusal" both.

**Reproducer:**

```
$ bash -c 'set -euo pipefail; printf "x\n" | grep "^id " | sed "s/^/y/"; echo REACHED'; echo "exit=$?"
exit=1
```

`REACHED` never prints, nothing is written to stderr.

## MINOR 2 — `tools/staged_cover_bench.sh:70-71`: `REPS`'s spelling is not validated (SHELL_CHECKLIST item 8)

**Status:** CONFIRMED. `[ "$REPS" -ge 5 ] 2>/dev/null || fail` validates the
value and not the spelling, and `$REPS` is then written into the script's own
record line (`… reps $REPS`).

**Reproducer:**

```
$ bash -c 'REPS=010; [ "$REPS" -ge 5 ] 2>/dev/null && echo "PASSES numeric test"; echo "seq gives: $(seq 1 $REPS | tr "\n" " ")"'
PASSES numeric test
seq gives: 1 2 3 4 5 6 7 8 9 10
$ bash -c 'REPS=+7; [ "$REPS" -ge 5 ] 2>/dev/null && echo "+7 PASSES numeric test"'
+7 PASSES numeric test
```

`010` passes the guard as octal **8**, runs **10** reps, and the record line says
`reps 010`. This is item 8's own `[ 010 -ge 1 ]` example, on a script whose
output a landed ADR verdict quotes.

## MINOR 3 — `tools/staged_cover_bench.sh:123`: command-substitution status discarded inside an `echo` (SHELL_CHECKLIST item 1)

**Status:** CONFIRMED, cosmetic.
`echo "… ($(grep -c early "$WORK/bands") early, $(grep -c late "$WORK/bands") late)"`.
`grep -c` prints `0` and exits 1 on no match; inside an `echo` argument the status
is discarded, so a bands file with no `early` rows prints `0 early` rather than
refusing. `COUNT` is separately validated `> 0`, so no number the ADR quotes
depends on this line — hence MINOR.

## MINOR 4 — D-362 names the instrument's tree state as a tree in which the instrument does not exist

**Status:** CONFIRMED.

D-362: *"`tools/staged_cover_bench.sh` is that bench: ONE tree state (HEAD at
commit `8ce13ff`, before this line's own commit touches `cover.rs`)"*.

```
$ git show --stat --oneline 8c13251
 docs/decisions.md           |   2 +
 tools/staged_cover_bench.sh | 200 ++++++++++++++++++++++++++++++++++++++++++++
```

`tools/staged_cover_bench.sh` does not exist at `8ce13ff`; it is created by
`8c13251`, the commit D-362 itself lands in. The **subject** revision is stated
correctly and is well-defined (`cover.rs` is byte-identical at `8ce13ff` and
`8c13251`, since `8c13251` touches no engine source). What is misstated is the
**instrument's** governing revision, which CLAUDE.md requires be named: it is
`8c13251`, and the run was taken against an uncommitted working-tree copy of it.
The correct form is "instrument at `8c13251`, subject at `8ce13ff`". (D-363's
re-run is clean on this axis: the script is byte-identical between the two
commits — `git diff 8c13251 23373e3 -- tools/staged_cover_bench.sh` is empty.)

## MINOR 5 — the determinism CI gate does not cover `CandidatePolicy::Staged`

**Status:** CONFIRMED as a coverage gap; the **property itself holds**.

`tools/determinism.sh:39` is `CONFIG="configs/gate_v0.toml"`, a `kind = "radius"`
document, and it is the only config the gate runs. CLAUDE.md rule 4 makes the
determinism self-test a CI gate, and WP-1.5b introduces an entirely new choice
path (staged generation plus a `ThreatState` carried and unwound in `Position`)
that no run of that gate touches.

I verified the property holds by hand — see A.12, `A==B OK` over 230 output lines
across two separate processes at `configs/instrument_staged_v0.toml`. So this is
an uncovered invariant, not a broken one. Closing it is a one-line change: a
second `CONFIG` pass, or a staged config alongside `gate_v0.toml`.

Reproducer: `/tmp/claude-1000/…/scratchpad/staged_determinism.sh`.

## MINOR 6 — the movetime ceiling gate does not cover `configs/play_staged_v0.toml`

**Status:** CONFIRMED as a coverage gap; compliance verified to hold (B.6).

`tools/movetime_check.sh:32` is `CONFIG="configs/play_v0.toml"`. The play seat
this WP ships is not gated, and it is the seat where `ordering::order` — and with
it the ordering loop's own deadline check — no longer runs. Zero overshoots in my
probe, so this is coverage, not a defect.

## MINOR 7 — nothing in the tree measures the registered hotspot's own share of a node

**Status:** CONFIRMED.

§4: *"measure `blocking_covers` and `min_hitting_set_exceeds` at the candidate
counts its own generator produces, not at counts chosen to make a curve."*
`tools/staged_cover_bench.sh` reports whole-search nps and time-to-depth only.
The 4.4–4.8 % whole-search improvement from remedy 1 is a **lower bound** on the
arithmetic's share of a node, not a measurement of it, so nothing confirms
D-263's own registered ceiling (10.51 % → 7.45 % of a fast node) against the
generator as built. §4's INSTRUMENT clause does register a whole-search bench,
so this is a tension inside §4 rather than a deviation from it — recorded because
it is what a future remedy-2 bracket would have to be derived against.

## MINOR 8 — the S-M differential gate is structurally blind to row-classification errors in one direction

**Status:** CONFIRMED as a scope observation; the direction is covered elsewhere.

`staged_differential_gate_tests.rs:140` is `if row != StagedRow::Filtered { continue; }`.
It `panic!`s helpfully when shipped says FILTERED and R1 does not — but the
opposite direction (shipped early-outs to `Impossible`, so the row is
`BatchedLost` or `OverloadReturn`, while R1 answers `Minimal`) is silently
skipped. That is exactly the direction a false-positive
`three_pairwise_disjoint_families` would produce, so the gate that landed
alongside the early-out cannot see the early-out's own failure mode.

It is covered — `threat_oracle_tests::threat_incremental_matches_reference_on_random_playouts`
asserts full `Cover` equality against R1 at every ply and crosses the early-out
2 049 times (B.3(b)), and it is what killed mutation 1. Recorded so the coverage
is not attributed to the wrong gate.

---

# REJECTED FINDINGS (attempted, with the reproducer, per §Process)

| # | Attempted finding | Why rejected |
|---|---|---|
| R1 | The early-out can answer `Impossible` where a ≤2-cell cover exists | Exhaustive sweep over all 50 625 four-family collections from a 15-family catalogue: 13 140 firings, zero false positives. Temporary test in `cover::tests`, reverted |
| R2 | D-363's "the regression is the proof" is vacuous because the early-out never fires in it | It fires **2 049 times** in `threat_oracle_tests` alone (R1-oracle equality at every playout ply), 2 154 across `pistol-solver`, 6 in the S-M gate. Instrumented with `eprintln!`, counted, reverted |
| R3 | The quiet-ball safety net can emit an empty set at game start | `within_radius`'s empty-board arm is `legal_placements(board)` = the origin. Live through all four staged configs at two budgets: `bestmove 0,0` every time, no panic |
| R4 | The staged config schema admits a bad document | 21 malformed/boundary documents, all refused by name with the right key; the control accepted. `deny_unknown_fields` verified in both directions |
| R5 | Fixture truncation / wrong sha / an unsolvable case passes silently | Every corruption exits 1 or 2 with a named error; the sha pin goes red on an in-tree edit (mutation 4) |
| R6 | Losing `ordering::order`'s deadline check makes `play_staged_v0` overshoot its movetime ceiling | 12 positions × 3 budgets (50/100/200 ms), ceiling `N+50`: **zero** overshoots, identical to `play_v0`. Coverage gap recorded as MINOR 6 instead |

---

# What I did NOT find, stated positively

Because a review that only lists defects misreports the artefact:

- The **cover-arithmetic early-out** is mathematically sound, exhaustively
  verified over the space I could enumerate, genuinely exercised thousands of
  times by an independent brute-force referent, and killed by that referent when
  mutated. It is the best-defended thing in this changeset.
- The **config layer** is airtight. 21 adversarial documents, 21 named refusals,
  correct key attribution on every one, including the cross-field widening rule
  and the internally-tagged-enum unknown-field case in both directions.
- The **rules layer** is untouched and un-weakened. Every illegal position I
  built is refused identically under `Staged` and `Radius`, by `pistol-core`,
  by name. CLAUDE.md rule 2 holds.
- **N-E** is fully paid, all four conditions independently reproduced against
  the shipped script.
- **Determinism** holds under `Staged` across processes, by construction and by
  measurement.
- The **D-263 bench chain** really happened in the shape §4 registers: a
  measurement-only BASELINE commit, then a one-change remedy commit, both
  IQR-gated, both bands, nps and time-to-depth.
- The **`#[ignore]` / release split**, the artifact-stream binary resolution, the
  VOID preflight, and the sha-pin discipline all follow existing in-tree
  precedent rather than inventing new shapes.

---

*Reviewed at `38d9493654e8098743e618af1bafd9afa130c56d`; HEAD unchanged and tree
clean at exit. No fix was applied — this is a review dispatch, and all four
mutations were reverted.*
