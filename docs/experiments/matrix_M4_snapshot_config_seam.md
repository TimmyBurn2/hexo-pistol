# MATRIX M4 — the snapshot's config seam
Status: **AUTHORED, NOT SELECTED.** Awaits fresh-context DECISION-RED-TEAM.
Subject: `tools/baseline_snapshot.sh` at `9421d19`. Owning unit:
`docs/experiments/U4_soundness_instrument.md` §9, u-rev 2.

## WHY THIS IS A FRESH MATRIX AND NOT THE `ec8f7fb` RECOVERY

The `ec8f7fb` matrix is recovered verbatim at the head of U4 §9 and is **history,
not a candidate.** T1' says identical = attack stands, differs = fresh round, and
U4 §9's DIFF 2 measured it DIFFERS on four cells, three of them MEASURED
falsifications. **The subject itself moved**, which is the difference that makes
recovery impossible rather than merely stale:

| What moved | From | To | Mark |
|---|---|---|---|
| The registered quantity | `timing depth_at_500ms`, 32 lines BELOW the `# timing` marker whose own emitted text reads *excluded from every comparison* | per-position `depth_turns` and `nodes` at the registered 50 000-node budget, plus the `ladder … nodes` counts — ABOVE the marker | **MEASURED** |
| The BEFORE cost | 34.0 s | **34.5 s**, re-taken under the amended script because N-A *is* a change to the instrument | **MEASURED** (by the design session at `6feb40a`; not re-taken here — see COST below) |
| N-A's mitigation | "the flag is the fourth of its exact kind"; "the `argument` helper already refuses an empty value" | WITHDRAWN. Four guards are owed and the helper is none of them | **MEASURED** |
| N-B's rejection ground | "breaks the D-209 instrument golden transcripts" | FALSE — `grep -c instrument_v0` on that fixture is 0 | **MEASURED** |

A matrix whose subject is a different quantity, measured at a different cost,
with its recommended option's mitigation withdrawn and one rival's rejection
ground void, is not the same matrix. The options below are authored from the
subject as it stands at `9421d19`.

## FACTS THE MATRIX STANDS ON — every one MEASURED at `9421d19`, with its command

| # | Fact | Command |
|---|---|---|
| 1 | `CONFIG="configs/instrument_v0.toml"` is a **literal at line 170** with no flag, read at **five** further sites (271, 321, 425, 464, 531) and emitted at 440 | `grep -n CONFIG tools/baseline_snapshot.sh` |
| 2 | The script has **six** flag arms: `--out`, `--nodes`, `--corpus`, `--ladder-depth`, `--ladder-cap-s`, `--binary` | `grep -cE '^\t--' tools/baseline_snapshot.sh` → 6 |
| 3 | **NO `tools/` script takes `--config`.** It would be the FIRST config-scope flag in the whole tree, not the fourth flag of an existing kind | `grep -ln 'argument --config\|--config)' tools/*.sh` → no output |
| 4 | `config <path> <sha>` is written into `$INVARIANT` (line 440); the marker is echoed at line 633; so the config provenance is **ABOVE** the marker already | read lines 435–445 and 628–640 |
| 5 | Per-position `depth_turns`/`nodes` → `$INVARIANT` (494); `timing position … time_ms … nps` → `$TIMING` (496); `timing depth_at_500ms` → `$TIMING` (605). **The moved-to quantity is above the marker and the demoted one below, in the shipped script and not only in the document** | read lines 490–498, 600–610 |
| 6 | The shipped script is driven by **29** tests in `crates/pistol-cli/tests/baseline_snapshot_tests.rs` (1234 lines) — the SHELL_CHECKLIST coverage rule is already satisfied for the script as it stands | `grep -c '^#\[test\]' crates/pistol-cli/tests/baseline_snapshot_tests.rs` |
| 6b | **All 29 tests invoke the script through ONE helper**, `go()` at lines 170–201, which assembles `--corpus`, `--ladder-depth`, `--binary` and the optional flags in a single place. There are not 29 invocation sites; there is one | read lines 165–201 of that file |
| 6c | `tools/ci.sh` does **NOT** invoke the snapshot | `grep -n baseline_snapshot tools/ci.sh` → no output |
| 7 | **NONE of the four staged configs exists on disk.** `configs/` holds 12 files and no `*staged*` | `ls configs/` |
| 8 | `configs/instrument_r2_v0.toml` exists and is referenced by **no** script and **no** crate — there is NO precedent in this tree for pointing an instrument at a second committed config | `grep -rn instrument_r2_v0 tools/ crates/` → no output |
| 9 | The script already passes `--config "$CONFIG"` **to the engine** at three call sites (425, 464, 531) — the engine's own flag is spelled the same | `grep -n 'BINARY" --config' tools/baseline_snapshot.sh` |

**The four guards owed by any option that lets a caller name the path** —
MEASURED enumeration, carried from the design's amendment 2 and re-read against
the shipped script at this revision: (i) caller-relative resolution, as `--out`
(line 242 ff.) and `--binary` each got; (ii) the printable allow-list extended to
the whole `$CONFIG` path, because unlike `--corpus` it reaches the record as a
WHOLE PATH on two invariant lines rather than through `$(basename …)`;
(iii) three named refusals — directory, missing, not a regular file — against
line 271's bare `[ -f ]`; (iv) an assertion that the script's `config` line and
the engine's `engine_id config` line name the same document.

## WHAT THE OPTIONS ARE OPTIONS ABOUT

By what seam does the snapshot instrument produce a record taken under a STAGED
config, given that the number it must produce now lives above the marker, that
the staged configs do not yet exist, and that the instrument's own revision is
named in whatever pre-registration consumes the record?

## Options

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| **N-A′ — OPTIONAL `--config PATH`, default unchanged** | A seventh flag arm; `CONFIG` keeps `configs/instrument_v0.toml` as its default. The recovered N-A, re-costed at the moved subject. | A `tools/` change: SHELL_CHECKLIST answered item by item; **four** guards owed (MEASURED, above); **ESTIMATED 5–8 new tests** driving the shipped script, one per guard plus a same-document assertion and a default-unchanged test, against the 29 that exist. BEFORE re-taken: **MEASURED 34.5 s** one run. | **The default is the failure mode.** A caller who omits the flag gets a record that looks like every other record and silently names the radius config — and the WP's whole point is comparing two records. Mitigated only by fact 4: the path is already on an invariant line, so two records under two configs cannot be diffed as one. It reopens a script whose review round closed recently. It is the FIRST config-scope flag in `tools/` (fact 3), so it sets the precedent every later script inherits, and it does so with a default. |
| **N-E — REQUIRED `--config PATH`, NO default** | Same arm, but `CONFIG` starts unset and an invocation without it is refused by name. | Everything N-A′ costs, plus the caller audit — **MEASURED, and it is small: THREE sites.** The default literal is at exactly one site (fact 1); all 29 tests invoke through the single `go()` helper (fact 6b), so they are one line; `tools/ci.sh` does not invoke the script at all (fact 6c); the third site is the script's own usage block (line 109 ff.). Plus **ESTIMATED 1 further test** that the refusal fires by name. | Breaks callers loudly rather than quietly, which is the direction rule 3 asks for, but it is a breaking change to a recently reviewed instrument for a WP that has not started IMPL. If any historical record was taken by an invocation reproduced somewhere as a literal command, the comparison across that boundary needs the old spelling documented, not merely the new one enforced. |
| **N-F — a second committed script**, `tools/baseline_snapshot_staged.sh` | No flag surface at all; a second script whose `CONFIG` literal is the staged document. | **MEASURED 646 lines** duplicated, or a shared body extracted and both scripts rewritten around it. Either way the 29-test suite must be parameterised or duplicated. **ESTIMATED** the extraction is the larger `tools/` change of any option here. | Two instruments that must stay in step, and rule 9's single-responsibility cap bites on a 646-line body duplicated. Worse for THIS subject: the two records must be COMPARED, and two scripts diverging by one line each revision is exactly how a comparison stops comparing. The `# timing` marker discipline would have to hold identically in both. |
| **N-G — an environment variable**, `PISTOL_SNAPSHOT_CONFIG` | `CONFIG="${PISTOL_SNAPSHOT_CONFIG:-configs/instrument_v0.toml}"`. One line. | **MEASURED one line changed**; **ESTIMATED 2–3 tests**. Cheapest diff of any option. | **Rejected on the checklist's own named class.** The config then comes from ambient state that never appears in the caller's command line, so a record's provenance depends on an environment the record cannot attest — and `set -euo pipefail` will not notice, which is EXIT-0-WRONG-ANSWER by construction. It also defeats fact 4's protection in the one case that matters: an operator re-running "the same command" in a different shell gets a different record with no signal. |
| **N-H — take the Staged number from a different instrument** (`tools/bench_delta.sh`) | Leave the snapshot alone; register the Staged quantity from the delta bench. | **ESTIMATED** comparable to N-A′, because it relocates the same change. | **MEASURED void:** `tools/bench_delta.sh` has no config flag either (fact 3 covers all of `tools/`), so this does not avoid a `tools/` change, it moves it to a script whose output is a RATIO and not the per-position record the registered quantity is made of. Rejected. |
| **N-D′ — take no Staged snapshot** (the null row, recorded because the restructure red team's F11 found a matrix missing one) | Report the radius numbers; the Staged depth evidence is not taken. | Zero. | The WP's depth claim goes unmeasured, and D-310 already shrank this WP's SPRT delta by deferring stage Q. Rejected — but recorded, because a null row a matrix never states is a rejection nobody can check. |

## Recommendation

**N-E — required `--config PATH`, no default.**

Grounds, in the order they bind:

1. **Hard rule 1 is the closest binding text and it points one way.** "No
   code-side default for any tunable — a default lives in exactly one schema
   place." `CONFIG` is a tunable and line 170 is a code-side default. N-A′ keeps
   it and adds a flag beside it; N-E removes it. The rule was written for
   `Budget`, and its stated reason — "an absent budget is an error, never a
   fallback" — transfers exactly: an absent config in a two-config comparison is
   an error, and a fallback is how it becomes a wrong answer.
2. **The failure mode N-A′ carries is the one this WP keeps producing.** A record
   silently taken at the wrong config is an EXIT-0-WRONG-ANSWER, the single class
   `tools/SHELL_CHECKLIST.md` names three consecutive rounds of. Fact 4 makes it
   *detectable after the fact* by a reader who compares the two `config` lines; it
   does not make it *refused*. N-E refuses it.
3. **The precedent cost is asymmetric and it is paid once.** Fact 3: this is the
   first config-scope flag anywhere in `tools/`. Whatever shape it takes is what
   every later script copies. A required flag copied forward costs each later
   script one refusal; an optional flag with a default copied forward costs each
   later script a silent wrong-config path.
4. **The extra cost over N-A′ is bounded, and it was MEASURED rather than
   estimated.** Facts 1, 6b and 6c: three caller sites, one of them a usage
   comment. Fact 7: no staged config exists yet, so there is no historical staged
   record whose invocation spelling would be broken — the breaking window is at
   its narrowest it will ever be.

**A DISCLOSURE ABOUT GROUND 4, because the correction ran toward the
recommendation.** This cell first read *"ESTIMATED 29 test invocations to audit"*.
Measuring it — one `sed -n '165,201p'` on the test file — collapsed it to three
sites and made the recommended option cheaper than the matrix had costed it.
D-291's clause is that an estimate measurable in seconds is a finding, and this
one was; it is recorded here rather than silently replaced, because a matrix that
re-measures a cell only when the estimate hurts its recommendation is doing
something other than measuring. **The red team should check both that the three
sites are three and that no fourth caller exists outside `tools/` and
`crates/`.**

**What the recommendation does NOT claim.** It does not claim N-E is free. It is
strictly more expensive than N-A′ — by three edits and one test — and that
difference is small enough that grounds 1–3, which are about failure modes and
precedent rather than cost, are what the selection actually rests on. A red team
that kills grounds 1–3 kills the recommendation regardless of ground 4.

## What flips it

Each clause names a remedy the trigger can reach — the incoherence F5 found in
the restructure matrix, where the trigger was about units 1 and 4 and the remedy
deferred unit 3.

- **The caller audit is larger than MEASURED.** Trigger: a caller outside
  `tools/` and `crates/` — an operator runbook, a manifest, a docs command block —
  reproduces a snapshot invocation as a literal command, taking the three sites
  above ten. Remedy: **flip to N-A′** — the same flag, a default retained, and the
  audit does not happen. Reachable because N-A′ and N-E differ in exactly the
  default and nothing else; the four guards, the tests for them and the
  SHELL_CHECKLIST answers are shared and are not re-done.
- **A historical record exists that a required flag would orphan.** Trigger: any
  committed or manifest-indexed snapshot record whose invocation is reproduced
  anywhere as a literal command without a config. Remedy: **flip to N-A′.** Same
  reachability. (MEASURED at this revision: fact 7 says no staged record can
  exist yet; this clause is about the radius records.)
- **The four guards turn out not to be four.** Trigger: implementing the guards
  shows one is already discharged by an existing line, or a fifth is needed.
  Remedy: this does not move the selection between N-A′ and N-E, since both owe
  the same guards — it moves the COST cells of both by the same amount, and the
  matrix records that it is not a selection trigger. **Stated so it is not
  mistaken for one.**
- **Rule 1 is read as not reaching a `tools/` script.** Trigger: the architect
  rules that hard rule 1's "no code-side default" governs engine config loading
  only and not shell instruments. Remedy: **ground 1 falls, and with it the
  recommendation — re-decide between N-A′ and N-E on grounds 2 and 3 alone**,
  which do not depend on rule 1. This is reachable because grounds 2 and 3 are
  stated independently; it is named because ground 1 is doing the most work and
  its scope is the matrix's most arguable premise.

## COST OF THE DECISION THIS MATRIX FEEDS

Stated so the proportion is visible on the matrix's own face. Selecting costs one
DECISION-RED-TEAM dispatch. **IMPLEMENTING** the selected option costs, MEASURED
where the number exists and ESTIMATED where it does not: one `tools/` commit with
a SHELL_CHECKLIST review answered item by item; ESTIMATED 5–8 new tests driving
the shipped script (N-A′) or those plus the caller audit (N-E); and **two
snapshot runs at MEASURED 34.5 s each** — a BEFORE re-taken under the amended
script and the Staged AFTER — replicated three times per the design's amendment
5, so **MEASURED 34.5 s × 6 ≈ 3.5 min of machine time**, plus the release build.
**No number in this matrix was re-measured by a run costing more than a second**;
the 34.5 s is cited from the design at `6feb40a`, with its provenance, and is not
this matrix's own measurement.

---

*Matrix M4, authored fresh at `9421d19`. Not selected. Awaits
DECISION-RED-TEAM.*
