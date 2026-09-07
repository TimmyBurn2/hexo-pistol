# Fast wins [ROUTINE] — closure

**Dispatch** 2026-09-07, three independent items, per-item STOP, no strength
claim, no SPRT, no sealbot match played.

**Opening revision** `235b6db` (`dev`), tree clean, no worktree, no engine
process (`ps aux | grep -Ei 'pistol|sealbot|matchserver'` empty at open).

**Closing revision, CI and per-item state** are in §5. Every `file:line` below
was read at `235b6db` unless a line says otherwise (D-692).

---

## §0 First actions

`tail -n 8 docs/decisions.md` put the log at **D-690**. The six paste-block keys
were absent (`grep -c '\[<key>\]' docs/decisions.md` = 0 for each), so they were
appended at the next free numbers, **D-691 … D-696**, unmodified:

| D | key |
|---|---|
| D-691 | `confirm-by-behaviour` |
| D-692 | `counts-name-revision` |
| D-693 | `prose-verdict-base-rate` |
| D-694 | `seed-tool-reviewed` |
| D-695 | `anchor-v7-protocol` |
| D-696 | `depth-diagnostics-first` |

---

## §1 Premises (D-477) — quoted, before anything is ruled

### P1 — `common::seed_tool`, its guard test, the harness list, the closure doc's paragraph

**Two definitions, not one.** `crates/pistol-cli/tests/common/mod.rs:199` and
`crates/pistol-arena/tests/common/mod.rs:287` each declare
`pub fn seed_tool(root: &Path, script: &str)`. The cli version delegates the
scan to a helper `tools_referenced_by` (`common/mod.rs:228-252`); the arena
version inlines the same scan in the body of its own loop
(`common/mod.rs:312-329`). **The closure logic is written twice**, and the arena
doc comment states the relationship rather than the duplication:

> "The same computed closure `pistol-cli`'s `common::seed_tool` provides, and
> it exists here for the same reason"

**The refusal each takes** is one assertion, identical in both:

```rust
assert!(
    script.starts_with("tools/") && script.ends_with(".sh"),
    "seed_tool copies `tools/` scripts; got `{script}`"
);
```

**The scan, verbatim** (cli, `common/mod.rs:240-246`):

```rust
if word.ends_with(".sh") {
    for candidate in [word.clone(), format!("tools/{word}")] {
        let candidate = candidate.trim_start_matches("./").to_owned();
        if candidate.starts_with("tools/") && repo(&candidate).is_file() {
            found.push(candidate);
            break;
        }
    }
}
```

**The harness list it computes — the COUNT, at `235b6db`.**
`git grep -n "seed_tool(" -- 'crates/*'` returns **19 lines**: 2 are the
definitions, 17 are calls. Of the 17 calls, **3 are the guard suite's own**
(`tool_seeding_tests.rs:13,41,59`), leaving **14 calls in 11 harness files**:

```
crates/pistol-arena/tests/artifact_gate_tests.rs        3
crates/pistol-cli/tests/arena_smoke_gate_tests.rs       1
crates/pistol-cli/tests/baseline_snapshot_tests.rs      1
crates/pistol-cli/tests/bench_delta_tests.rs            1
crates/pistol-cli/tests/book_v3_disjointness_tests.rs   1
crates/pistol-cli/tests/decision_key_check_tests.rs     1
crates/pistol-cli/tests/file_justification_gate_tests.rs 1
crates/pistol-cli/tests/label_consistency_check_tests.rs 2
crates/pistol-cli/tests/staged_cover_bench_gate_tests.rs 1
crates/pistol-cli/tests/staged_soundness_check_gate_tests.rs 1
crates/pistol-cli/tests/tactical_check_gate_tests.rs    1
```

**The guard test's claim, verbatim** — `tool_seeding_tests.rs`, the doc comment
of `no_test_copies_a_tools_script_outside_the_seeder`:

> "**This is the guard, and it is a PROPERTY rather than a list.** … a
> hand-written detector missed two of them because it matched
> `repo("tools/x.sh")` and one harness passed `repo(file)` out of an array."

and what it actually does (`tool_seeding_tests.rs:81-146`): walks
`repo_root().join("crates")` for `*.rs` under a `tests` component, skips
`common/mod.rs` and itself, finds each `fs::copy(` and takes the **first
argument's source text**, and reports the site when that text
`contains("tools/")`.

**The closure doc's "unscoped" paragraph, verbatim**
(`docs/experiments/pre_phase2_sweep_CLOSURE.md:1120-1126`):

> "**THE UNSCOPED MECHANISM, NAMED AS THE DISPATCH ASKS.** `common::seed_tool`
> (D-685) is UNSCOPED AND LANDED: it was not in any group's scope, no review or
> confirmation adjudicated it, and it ships. It is a computed closure with a
> property guard rather than a list, which is why it is recorded as landed
> rather than held — but a successor should know that the one mechanism in this
> package nobody reviewed is the one every gate harness now depends on."

### P2 — the three SPRT artifacts: paths, digests, fields. **FINDING: two of the three dispatch paths do not exist.**

The dispatch names `artifacts/opt_arc_combo_sprt_v1.txt`, `s2_sprt_v1.txt`,
`s3_sprt_v1.txt`. **Only the first exists.** `s2_sprt_v1.txt` and
`s3_sprt_v1.txt` are absent from `artifacts/` at `235b6db`. What exists is the
optimization arc's worktree export:

| what | path | sha256 |
|---|---|---|
| combo summary (the digest §1.4 quotes) | `artifacts/opt_arc_combo_sprt_v1.txt` | `8c64e8fdb6ec5857395fef93ebafbaa02d98206a03565082735a9f3aac54d9ec` |
| combo run log | `artifacts/scratch_export_20260906/pistol-wt/combo_sprt.log` | — |
| **combo report (per-game)** | `artifacts/scratch_export_20260906/pistol-wt/combo_sprt_report.json` | `8607fa262f80aa533baac6226176dc221d262332c6f1017ca6973ca36d70e0d3` |
| **S2 report (per-game)** | `artifacts/scratch_export_20260906/pistol-wt/s2_sprt_report.json` | `eefbb48e2138ef9c83fd1f31beb983cb8152686566e10b3dc8cd95952661702b` |
| **S3 report (per-game)** | `artifacts/scratch_export_20260906/pistol-wt/s3_sprt_report.json` | `512756128d41059a5fd90cbf8d65606d8e994fe7ed3fe7a6821414494c102ef6` |

The `.json` extension is a misnomer: the files are `arena_report 4` records —
whitespace-separated key/value lines — and `json.load` refuses them at byte 0.
They are the reports the run logs name (`arena: report written to
/home/tom/pistol-wt/<name>_report.json`).

**That the report and the summary are the same run is checked rather than
assumed**: the reader of §3 derives combo `n 910`, capped `204`
(fraction `0.224`), pentanomial `82/77/180/67/49`, and 315 / 391 wins by arm —
every one of which the summary `.txt` states independently. S3 derives
`73/102/273/92/60` over 600 pairs, which is the pentanomial
`docs/research/search_next_2026-09.md` §1.4 publishes.

**The fields each `game` record carries**, verbatim from the first line of the
combo report:

```
game 0 opening 0 p1 combo p2 committed result capped end normal forfeit_by none
reason none turns 40 dup_of none nodes_a 3612672 nodes_b 3813376 depth_a 6
depth_b 5 llr_game none llr_pair none
```

a `pair` record: `pair 0 opening 0 bucket p2 score_a 0.500000000` — `score_a`
here is the GAME score (0 / 0.5 / 1), not an evaluation — and a `moves` record
that is the opening's stone plus one `q,r/q,r` per turn and nothing else.

### P3 — is a per-game root score in those logs? **NO, and the gap is not in `SearchInfo`.**

**The score exists at every point up to the report, and stops there.**

- `crates/pistol-search/src/info.rs:229` — `pub score: i32`, documented as "the
  score of the position from the point of view of the side to move at the root".
- `crates/pistol-cli/src/report.rs:45,94` — it reaches the wire: the `info`
  grammar is `… hashfull score pv`, and "`score` is one of `cp <n>`,
  `mate <turns>` or `-mate`".
- `crates/pistol-arena/src/labels.rs:156-206` — the arena can and does parse it,
  into `score_kind` / `score_value` — **in the `--capture` label pass**.
- **The SPRT report does not persist it.** No `game` record field is an
  evaluation (see P2); `crates/pistol-arena/src/conclusion.rs:81` writes the
  pair's `score_a` game score; the `moves` record is coordinates.

**So the dispatch's parenthetical is corrected**: the gap is not a missing
`SearchInfo` field. `SearchInfo.score` exists and reaches the wire. What is
missing is the arena's PER-MOVE PERSISTENCE of it in the SPRT report.

**And the one existing mode that could recompute it refuses these reports by
name.** `arena --capture` re-asks every position of a report at a label budget
and records the score, but `crates/pistol-arena/src/capture.rs:135,160` refuse a
report "whose two seats do not attest the same engine" — "its two seats attest
different engines: they differ at `{differing}`, so a capture over …". All three
runs are A-versus-B. The recovery path is closed by the harness's own refusal,
not by an argument.

### P4 — anchor v5/v6 configs and logs: paths and per-side movetime

`local/sealbot_anchor_v5_seat1.toml` and `local/sealbot_anchor_v6_seatswap.toml`
(untracked; `git ls-files local/` returns only `local/sealbot.example.toml`).
Receipts `artifacts/sealbot_anchor_v5_seat1/` and
`artifacts/sealbot_anchor_v6_seatswap/`.

**Per-side movetime, quoted**: pistol `movetime_ms = 500`; sealbot
`time_limit_seconds = 0.3`. **UNEQUAL, in both runs.** Also unequal:
`turn_timeout_seconds` 120.0 (pistol) against 5.0 (sealbot). Ruled in
`docs/experiments/sealbot_anchor_v7_protocol.md` §A1.

### P5 — sealbot `moves_left` and the matchserver's cap. **The anticipated asymmetry does not exist.**

`moves_left` is the INTRA-TURN PHASE, values 1 and 2, not a turn-cap counter:
`current/types.h:21` `int8_t moves_left;   // 1 or 2`; `current/engine/board.h:90-93`
decrements and resets it to 2; `current/minimax_bot.cpp:32` reads it from
`moves_left_in_turn`; `current/engine/bot.h:216` mixes it into the TT key.
Verified from source at sealbot master `c94749c21c16c3b072fff6da49762dd5f92f3986`,
so the dispatch's default fork (quote the deep dive, mark UNVERIFIED) did NOT
have to be taken.

The matchserver caps at `tools/sealbot/matchserver/src/referee.rs:203-204`
(`GameResult::Capped { turn: turn_cap }`) and scores capped games as their own
cell, excluded from the decided denominator
(`tools/sealbot/matchserver/src/report.rs:5,37,204`).
`git grep -n turn_cap -- tools/sealbot/matchserver/src/` reaches `config.rs`,
`main.rs`, `openings.rs`, `referee.rs`, `report.rs` and **neither engine
client**: no engine is told the cap. Ruled in §A3 of the v7 protocol.

---

## §2 Item S — `common::seed_tool`: REVIEWED, FIXED, MUTANTS DEAD

**REVIEW-impl** at `d099be7` (subject byte-identical to `235b6db`): **FAIL**,
4 MAJOR and 6 MINOR. The findings were verified with reproducers here before any
fix landed, and the fix is one commit, `e5f4364` (D-688).

| # | finding | verified how | fixed by |
|---|---|---|---|
| MAJOR-1 | the guard built `Vec<char>` and indexed it with a BYTE offset from `str::find`, so it parsed from the wrong place on any file with non-ASCII prose | **MEASURED**: skew of **26** at `decision_key_check_tests.rs`'s own copy site; the scan extracted `ion_key_check.sh")` where the source argument is `repo("tools/decision_key_check.sh")` | iterate `text[open..].chars()` |
| MAJOR-2 | a LIVE offender sat behind it: `a_directory_that_is_not_a_repository_is_a_run_void_and_not_a_refusal` hand-copied the gate, so it voided on the MISSING RESOLVER seven lines before the git check it is named for — asserting exit 2 and `RUN VOID`, both satisfied by the wrong void | ran the seeded tree by hand | the test seeds through `seed_tool` and now asserts `not a git repository` by name |
| MAJOR-3 | the "control" asserted a string literal contained substrings of itself; it never called the parser | it was green throughout MAJOR-1 | the scan is extracted as `hand_written_copies` and the control drives it |
| MAJOR-4a | the closure followed only `*.sh`, so `.py` siblings a gate resolves by path literal never landed | **REPRODUCED**: a scratch tree holding exactly the 27-script closure of `tools/determinism.sh`, then `bash tools/governing_citation_check.sh` in it → `RUN VOID: tools/design_citation_check.py is missing`, `EXIT=2` | the alphabet is `.sh` + `.py` |
| MAJOR-4b | a subdirectory script reaching a flat sibling as `../require_tool.sh` seeded alone | the closure of `tools/sealbot/run_match.sh` was **1** file | the token is also tried relative to the referring script's directory, and normalized |
| MINOR-1,2 | an unreadable sibling silently truncated the closure; the execute bit was dropped silently | read | both panic by name (hard rule 3) |
| MINOR-3 | the destination tail came from file text and was joined unguarded | `lexically_normal` returns `None` above the root | containment guard, with its own test and control |
| MINOR-5 | the arena copy had no closure test | `git grep` over `crates/pistol-arena/tests` | `crates/pistol-arena/tests/tool_seeding_tests.rs`, 4 tests |
| MINOR-6 | the dedupe key was the raw string, so `tools//x.sh` counted twice | the `determinism.sh` closure was 27 names / 25 files | normalization |

**THE ALPHABET WAS CHOSEN BY MEASUREMENT, NOT BY TASTE.** Following `.md` as well
makes every closure **42 files from any starting point** — against **1** for
`require_tool.sh`, 25 for `determinism.sh` under `.sh` and 38 under `.sh`+`.py` —
because `tools/SHELL_CHECKLIST.md` names a dozen scripts it does not depend on.
A closure that sweeps is what the control test exists to forbid, so `.md` is out
and the reason is in the code comment.

**THE MUTANTS, all six DEAD** — receipts `artifacts/fast_wins/item_s_mutants.txt`.
M1 closure drops `require_tool.sh` → 10 harness tests fail. M2 the hand-written
copy returns → the guard names the file and line. M3 `.sh`-only alphabet → the
python-sibling test fails. M4 no referring-directory spelling → the subdirectory
test fails. M5 the containment guard stops refusing → its test fails. M6 the
byte-indexed scan returns → the control fails.

**AND ONE MUTANT DID NOT DIE FIRST TIME, WHICH IS THE FINDING THIS PACKAGE OWES
ITSELF.** The first replacement control asserted only the COUNT of hand-written
copies, and under M6 it stayed **GREEN**: the byte/char skew in that sample is
six characters, which is exactly `repo("`, so the misread source still began with
`tools/`. **A control written to close D-690's class reproduced it**, and only
running the mutant found that. The control now pins the PARSED ARGUMENT — `left:
[(3, "tools/determinism.sh\"")]` against `right: [(3, "repo(\"tools/determinism.sh\")")]`
— and M6 kills it.

## §3 Item D — depth diagnostics: **M7 NOT SUPPORTED, M9 STOPPED**

Output `docs/experiments/depth_diagnostics_2026-09.md`, instrument
`tools/depth_diagnostics.py` with `crates/pistol-arena/tests/depth_diagnostics_tests.rs`
under gate 3, receipt `artifacts/fast_wins/depth_diagnostics_v1.txt`.

**D1 (M7)** — the table is rendered by machine from the three reports and
reproduces every number the runs' own summaries state independently, including
the two pentanomials `search_next_2026-09.md` §1.4 publishes. Under the verdict
vocabulary fixed in §0 of that document, **M7 is NOT SUPPORTED**: `p2` is the
largest pentanomial cell in two runs of three but `capped` is the largest result
class in none of them.

**D2 (M9) — STOPPED, and the dispatch's parenthetical is corrected.** The gap is
**not** a missing `SearchInfo` field: `SearchInfo.score` exists and reaches the
wire. What is missing is the arena's PER-MOVE PERSISTENCE of it in the SPRT
report, and the one mode that could recompute it — `arena --capture` — refuses an
A-versus-B report by name. All three runs are A-versus-B. No engine change was
made, as the dispatch's default fork directs.

**The instrument's attribution guard discriminates**: a mutant swapping the two
report slots flips `deep` 9.00→3.00 and `shallow` 3.00→9.00, which
`reached_depth_is_credited_to_the_label_that_occupied_the_slot` catches
(`artifacts/fast_wins/depth_diag_attr_mutant.txt`).

### §2.1 The scoped confirmation — **PASS on all ten, and one new MAJOR**

Confirmed at `e5f4364` by a fresh strongest context, mutants re-run against a
**green scoped baseline** (arena `tool_seeding` 4/4, cli `tool_seeding` 8/8,
`decision_key_check` 10/10 = 22/22), tree verified clean between each. Evidence
exported to `artifacts/fast_wins_confirm/` (23 files, gitignored).

**All six claimed mutants died under a second hand, and no mutant survived.** The
confirmer added three of its own: `.md` into the arena alphabet (kills the new
control, `left: 34, right: 1`), dropping the empty-segment arm (kills the
normalization test, `left: Some("tools//require_tool.sh")`), and — the sharpest —
**M6 and M2 STACKED**, which reproduces the original defect end to end: with the
byte-indexed scan restored AND a live hand-written copy in the tree, the guard
reports **ok**.

**MAJOR-A, NEW AND GENUINE: THE FIX TURNED CI GATE 17 RED, AND IT WAS STILL RED
AT HEAD WHEN THE CONFIRMATION LANDED.** Taking the closure's three new guards
pushed `crates/pistol-cli/tests/common/mod.rs` over rule 9's cap with no
`docs/rule9_justifications.md` entry. Bisected with the shipped gate rather than
read: at `7381715` it exits 0, at `e5f4364` it prints `crates/pistol-cli/tests/common/mod.rs:
over the cap with no entry` and exits 1. The arena twin was registered; the cli
twin was not. **Fixed at `ed89e6f`**, one commit past the confirmed revision, and
the gate now reads `417 tracked .rs/.sh/.py files, 89 over the cap, all
registered`. This is a wrong-answer path in the sense D-424 protects — a red gate
a reader cannot distinguish from a real regression — so it was fixed and not
overruled.

**AND CARGO'S FAIL-FAST HID THE SUITE THE FIX ADDS**: gate 17's failure meant the
cli `tool_seeding_tests` binary never ran in the confirmer's first baseline. A
green-looking scoped run and a red workspace run were the same tree.

**Two MINORs the confirmation raised, both fixed at `ed89e6f`:**

- **MINOR-B — MINOR-1's remedy is unreachable through `seed_tool`.** Every file
  `tools_referenced_by` reads was `fs::copy`'d immediately before, so the copy's
  panic always fires first; driven at both revisions, identical message and site.
  The panic is kept — it is one call away from a caller that has not copied — but
  the doc comment no longer claims to prevent a truncation that cannot occur.
  Prose that constrains nothing is D-424's subject, and it was deleted rather
  than refined.
- **MINOR-C — "42 files from any starting point" overstated, and cited the wrong
  revision.** Re-derived at the shipping revision: of **61** `tools/` scripts,
  **44** reach ≥42 under `.md` and the rest do not; `require_tool.sh` goes 1→42
  and `determinism.sh` 38→42. The substance holds — those two are what the
  control turns on — but the comment now says what is true and says the numbers
  are the kind a reader re-derives.

**MINOR-D, recorded and NOT acted on**: the widened alphabet raises every seeded
scratch tree from 25 files / 277 KB to 38 / 467 KB, and the "does not sweep"
control holds because `require_tool.sh` is the one leaf that references nothing.
That is a real narrowing of the control's margin and it is a fact about
`tools/`'s shape, not about this fix; a successor that adds a second leaf-less
script strengthens it for free.

**A METHODOLOGY NOTE THE NEXT CONFIRMATION SHOULD HAVE.** Exporting
`CARGO_TARGET_DIR` reproduces CLAUDE.md's fixture hazard **even in a separate
worktree** — it leaks into the fixtures' own child `cargo` invocations and cost
this confirmation 6 spurious `solver_link_check` failures, which are a RUN VOID
and not a FAIL. The variable must be unset for the test run itself. The dispatch
that briefed this confirmation told it to export the variable, so the defect is
in the brief.
