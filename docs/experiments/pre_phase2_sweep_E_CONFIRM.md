# Pre-Phase-2 sweep — GROUP E, CONFIRMATION of the fix round

## Header

- **Revision confirmed**: `90a77a8` ("docs(sweep): the closure says plainly that
  it is one commit past its own cited run, and quotes the docs-only diff").
- **Does it still match HEAD?** YES at the end of this review.
  `git rev-parse HEAD` → `90a77a89b90310e8fa365fb2baf050b17a52927f`.
  `git status --porcelain` → ` M docs/decisions.md` (the dispatcher's two ADR
  lines D-686/D-687, declared OUT OF SCOPE and verified to be exactly that one
  hunk, four added lines and nothing else) plus
  `?? docs/experiments/pre_phase2_sweep_T_CONFIRM.md`, a sibling confirmation
  session's report. No tracked file under `crates/`, `tools/` or `configs/`
  differs from `90a77a8`.
- **Fix-round commits read**: `9ce9a7c` ("fix(sweep): the three reviews'
  findings") and `0fedfa8` ("test(search): the solver-proof invariant is driven
  after all"), both in full. `87305f7` was read for the question the dispatch
  asks of it — whether a later commit re-opened an E property — and it does not
  touch any E-group surface.
- **Finding list adjudicated**: `docs/experiments/pre_phase2_sweep_E_REVIEW.md`,
  11 findings (BLOCKING 1; MAJOR 1–4; MINOR 1–6).
- **Worktrees**: `/home/tom/pistol-wt/confirm-e` (detached at `90a77a8`) and
  `/home/tom/pistol-wt/confirm-e-pre` (detached at `cb6e853`, the reviewed
  revision, for the byte-identity re-derivation). Neither exported
  `CARGO_TARGET_DIR`; each used its own `target/`. Both removed after their
  cited logs were exported (§5).

---

## §0 The load-bearing guard, re-derived across the FIX ROUND

The E reviewer re-derived byte identity for `b60c3d3` → `cb6e853`. **Nobody had
re-derived it across the fix rounds**, which is where it matters here: `9ce9a7c`
and `0fedfa8` edited `census.rs`, `pvs.rs`, `search.rs`, `params.rs`,
`position.rs`, `staged.rs`, `quiescence.rs` and six configs.

**First, mechanically, what those two commits actually changed in shipped code.**
Every non-comment line in every `src/` tree and every `configs/` document between
`cb6e853` and `90a77a8`, enumerated rather than argued
(`e_noncomment_diff_cb6e853_90a77a8.txt`):

- `TriggerColumns::at` gains `site: &'static str`; the two call sites pass
  `"root"` and `"in-tree"`; the panic string interpolates it.
- a new `#[cfg(test)] mod tests` in `search.rs` (E4's driver).

**Nothing else.** `params.rs`, `position.rs`, `staged.rs`, `quiescence.rs`, every
`pistol-solver/src/*` file and all six configs are comment- and doc-only, and the
config diff filtered of `#` lines is EMPTY.

**Second, measured.** Two release builds, `cb6e853` and `90a77a8`, each in its
own worktree, driven over the E reviewer's own position corpora (40 positions
broad, 20 for the solver seats), ` nps <n> time <n>` stripped
(`e_identity_cb6e853_vs_90a77a8.txt`):

```
broad_gate_staged_snk_v0.txt        lines=345  bestmoves=80  difflines=0
broad_gate_staged_v0.txt            lines=341  bestmoves=80  difflines=0
broad_instrument_staged_v0.txt      lines=337  bestmoves=80  difflines=0
broad_instrument_v0.txt             lines=337  bestmoves=80  difflines=0
broad_play_staged_v0.txt            lines=337  bestmoves=80  difflines=0
broad_tactical_staged_v0.txt        lines=337  bestmoves=80  difflines=0
census_gate_staged_solver_v0.txt    lines=229  bestmoves=20  difflines=0
solver_gate_staged_solver_v0.txt    lines=165  bestmoves=40  difflines=0

census rows / root rows / digest:
pre  rows=158 root_rows=1 digest=73d9bc6938bb15de2466a0106a52f053a898b62b6666a2c3577693f58b407616
post rows=158 root_rows=1 digest=73d9bc6938bb15de2466a0106a52f053a898b62b6666a2c3577693f58b407616
```

540 searches per side, **zero differing lines anywhere, handshake included** —
the licensed `id candidate_policy staged …` change is already inside `cb6e853`,
so across the fix round there is nothing to license.

**And it joins up with the earlier evidence rather than replacing it.** The 158
census rows digest to `73d9bc6938bb15de2466a010…`, which is the value the E
REVIEW's own §1 table records for its `positions20` workload taken at **`b60c3d3`**,
and the sample row printed here is byte-identical to the one that report quotes.
So the census output is unchanged `b60c3d3` → `cb6e853` → `90a77a8` on a workload
the closure never used.

**Third, the closure's own receipts, re-derived rather than trusted**
(`e_receipts_90a77a8.txt`):

```
file count: 78
find . -type f | LC_ALL=C sort | xargs sha256sum | sha256sum
8b1e3d30597f7e47fccda5bde11cfa766662538fe449f3a59e2e1fef8da1aa8a  -
closure §4 claims: 8b1e3d30597f7e47fccda5bde11cfa766662538fe449f3a59e2e1fef8da1aa8a

31236b5632c0c1b16f0b6ff2a6a62aee6017321d05f6785b0baec7dd1222bbde  census_before.txt
31236b5632c0c1b16f0b6ff2a6a62aee6017321d05f6785b0baec7dd1222bbde  census_after.txt
31236b5632c0c1b16f0b6ff2a6a62aee6017321d05f6785b0baec7dd1222bbde  census_after_fixes.txt
```

§4's 78-file digest reproduces EXACTLY, so the receipt set is intact and the
three census digests are the files the closure hashed. **The byte-identity claim
holds at `90a77a8`, by an instrument the closure did not use and across the
commits the earlier evidence did not cover.**

---

## §1 Per-finding verdicts

| id | severity | verdict | reproducer re-run at `90a77a8`, and its output | one line of reasoning |
|---|---|---|---|---|
| BLOCKING 1 | BLOCKING | **CONFIRMED** | `./tools/file_justification_check.sh; echo EXIT=$?` → `file_justification_check: 413 tracked .rs/.sh/.py files, 87 over the cap, all registered in docs/rule9_justifications.md (87 entries)` / `EXIT=0`; and `artifacts/pre_phase2_sweep/ci_closing_0fedfa8.txt` ends `ci: all gates passed` / `EXIT=0` with 21 `=== gate n/21` lines | Gate 17 is green, and a full CI run now exists at a revision inside this package — `0fedfa8`, from which `git diff --stat 0fedfa8 90a77a8` is `docs/experiments/pre_phase2_sweep_CLOSURE.md \| 115 +++---`, one docs-only file, so the green run governs the shipped bytes at HEAD. |
| MAJOR 1 | MAJOR | **CONFIRMED** | `awk '/^quiet_radius/{f=1} f{print NR": "$0} /^safety_net_top_k/{if(f)exit}'` on all six staged configs → e.g. `configs/gate_staged_snk_v0.toml`: `34: quiet_radius = 1` / `35: # THE SAFETY-NET CAP ARMED — this document exists for exactly that (D-482).` … `40: safety_net_top_k = 8`; and `configs/instrument_staged_snk_v0.toml`: `33: quiet_radius = 2` / `34: # THE CAP ARMED. K = 16 is not a choice made in this file…` | The orphaned blocks are gone from all five the finding named plus a sixth (`gate_staged_heuristics_v0`), the word "Disabled" no longer leads into "THE SAFETY-NET CAP ARMED", and the `K = 16` misattribution to §10 is gone. `git diff cb6e853 90a77a8 -- configs/` filtered of `#` lines is EMPTY, so no key moved; `tools/config_check.sh` → `EXIT=0`. |
| MAJOR 2 | MAJOR | **CONFIRMED** | `sed -n '33,41p' crates/pistol-search/src/params.rs` → `/// This carries exactly what the search reads. `U3_tier_t.md` §10's table also / stated `quiet_top_k` and `widen_schedule` … nothing implemented them, so they were validated for schema completeness and read by nobody.` | Both falsified clauses are gone: the doc no longer says the table states the keys, nor that the config layer validates them. Past tense throughout. (One clause, "they are gone from the document too", is read here as the CONFIG documents, which is true; see MAJOR 4 for why the other reading would not be.) |
| MAJOR 3 | MAJOR | **CONFIRMED** | `sed -n '1,21p' crates/pistol-search/src/census.rs` → `/// **THE SECOND HALF OF THAT ARGUMENT NO LONGER HOLDS AND IS WITHDRAWN.** It / used to say "each firing site still names the two fields …" — true when the root and the in-tree site were two constructions. … so there is one site to attack and one mutant reaches it` | The falsified mutation-coverage claim is withdrawn by name and replaced with what is true (one constructor, one mutant, the referent test as the defence), which is stronger than the finding's suggested rewrite. |
| MAJOR 4 | MAJOR | **FAIL** | `git grep -n -E 'quiet_top_k\|widen_schedule' 90a77a8 -- docs/experiments/U3_tier_t.md` still returns `:24`, `:350`, `:353`, `:862`. §10's own document table at `:350-353` reads `\| document \| mode \| quiet_radius \| quiet_top_k \| widen_schedule \| why \|` / `\| configs/instrument_staged_v0.toml \| instrument \| 2 \| 16 \| [32] \| …` under the present-tense heading `**FOUR** complete documents … **This is the one place the count is stated**` | See §2. The four cited instances were fixed and `U2_node_protocol.md` was fixed, but the CLASS is open in the same document and the same section the code's citation points at. |
| MINOR 1 | MINOR | **CONFIRMED** | `sha256sum crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt` → `272c9b772aab49c7c6e1b4d9bc79d2f9b1937fc2937bc1cbe7caf09b9d991fe2`, equal to `TACTICAL_STAGED_V0_SHA256` at `tactical_staged_v0_tests.rs:25-26`; header now reads `# the quiet cut DISABLED — no code path arms it, which is why the two knobs / # that used to name it left the schema in docs/decisions.md D-675` | The derivation no longer rests on a key that does not exist; the conclusion (`require 20`) is unchanged and now rests on the code. The sha pin was moved deliberately and the `const`'s own doc records why the edit is listed rather than regenerated. |
| MINOR 2 | MINOR | **CONFIRMED** | `git grep -n "TriggerColumns::at(state" -- crates/` → `pvs.rs:861: … from_root, "in-tree"),` and `search.rs:866: … 0, "root"),`; `census.rs:176` → `"pistol-search invariant {}: the {site} trigger fired on a decided position"`; the same string searched across `crates/ tools/ configs/` outside `census.rs` returns nothing (exit 1) | The word the merge lost is restored, and both firings are now named rather than only the root one. Still no consumer reads the text, so it remains lossless. See NEW-1 and NEW-2 for what the restoration itself owes. |
| MINOR 3 | MINOR | **CONFIRMED** | See §3 — the reviewer's `.rev()` mutant re-applied in `/home/tom/pistol-wt/confirm-e` and run: `test result: FAILED. 5 passed; 1 failed`, the failure being `a_one_ply_witness_pairs_the_completing_stone_with_the_least_other_legal_cell`, `left: Some(Pair(Coord { q: 3, r: 0 }, Coord { q: 10, r: 0 }))` vs `right: Some(Pair(Coord { q: -8, r: 0 }, Coord { q: 3, r: 0 }))` | The happy path is now driven at BOTH call sites (`proof_first_move` and `proof_line`), the expectation is derived by `min()` over the filtered set rather than restating `find()`, and the surviving mutant dies. |
| MINOR 4 | MINOR | **CONFIRMED** | `git grep -n "candidate_policy staged" -- 'crates/*/tests' 'tools' 'configs'` → no output, `EXIT_A=1`; `git grep -ln "^id " -- 'crates/*/tests/fixtures'` → no output, `EXIT_B=1`; `docs/decisions.md:1428` carries `D-681 [no-handshake-golden-existed]` | Hard rule 10's remedy on an append-only log is an amending line, and D-681 is it: it states the correction, re-derives the two greps at `b60c3d3`, names what actually pins identity lines, and records that the listed edit is the diff itself. |
| MINOR 5 | MINOR | **CONFIRMED** | `staged.rs:241-242` → `// arm answers with, uncapped: stage Q's width knob left the schema with / // D-675, and this D-scope does not arm stage Q.`; `wp15b_census.rs:517-519` → `// batched nodes are the ones stage Q's width knobs would be sized against / // (they left the schema at docs/decisions.md D-675, unread by any code)` | Both stale comments amended. The class is closed inside `crates/`: the only other mentions of either key are `bin/pistol.rs:147` (explicitly historical, "rode here until D-675") and `config_validate_tests.rs` (the test that refuses them by name). |
| MINOR 6 | MINOR | **CONFIRMED** (recording accurate, reason holds) | `tools/bench_delta.sh:301` → `GUARDED_ID_FIELDS="config eval tt_bytes candidate_policy weights_sha256"`; `git show b60c3d3:crates/pistol-cli/src/bin/pistol.rs` line 159 → `format!("candidate_policy staged quiet_radius {quiet_radius} quiet_top_k {quiet_top_k}")` vs `90a77a8`'s line 155 → `format!("candidate_policy staged quiet_radius {quiet_radius}")`; the closure records it in §5 "Two findings recorded rather than fixed, with the reason" | The recording is accurate and its reason holds at HEAD: the guarded field really does differ across the boundary. **It is not the never-dischargeable class**: the outcome is a LOUD refusal at exit 1 that names `candidate_policy` (`bench_delta.sh:319-322`), i.e. an instrument declining to measure, not code producing a wrong answer. Recording is the right remedy under D-424 and it is recorded. |

---

## §2 Why MAJOR 4 is FAIL

The fix did most of the work. `U2_node_protocol.md` §U2-M item 2 now emits
`id candidate_policy staged quiet_radius <n>` and adds a paragraph saying the
`quiet_top_k <k>` token "rode this line until D-675 and is GONE" — that half is
**discharged**. In `U3_tier_t.md` §10 the config template no longer carries the
two keys, "Six keys, not five" became "Four keys", and the two deleted validation
rules are re-scoped as what WP-1.5c would owe. All four instances the finding
cited by line number are fixed.

**But the finding's property is not its four line numbers.** It is that
`crates/pistol-engine/src/config.rs:213` names `U3_tier_t.md` §10 as "this
document's schema, **the one place** the count of staged documents and their
shape is stated", so a reader following the code's own citation must not land on
a contradiction, and must not be led to write a document the engine refuses. At
`90a77a8` three present-tense statements of exactly the falsified fact survive,
one of them **inside §10 itself, above the paragraph that corrects it**
(`e_major4_u3_residual_90a77a8.txt`):

1. **§10's document table, `:350-353`** — under the heading *"**FOUR** complete
   documents, `deny_unknown_fields`, no code-side default for any value. **This
   is the one place the count is stated**"*, the table's columns are
   `quiet_radius | quiet_top_k | widen_schedule` and its rows assert
   `configs/instrument_staged_v0.toml` → `16`, `[32]`;
   `configs/tactical_staged_v0.toml` → `1024`, `[2048]`;
   `configs/gate_staged_v0.toml` → `128`, `[256]`. None of those three documents
   carries either key at `90a77a8`, and one that did would be refused by name.
   This is MAJOR 4's own stated failure mode, unmoved, two pages above the
   template that was moved.
2. **§1, `:24`** — *"`quiet_top_k` and `widen_schedule` still appear in §10's
   config documents … **whether the D-scope shipped surface keeps those two keys
   at all is OPEN**"*. §10's own amended paragraph says the opposite in bold, in
   the same file: *"§U3-Z's own OPEN question … is what that answered"*.
3. **§U3-Z, `:862-870`** — *"**§10's config documents each commit both keys**"*
   and *"**The carve does not choose.**"* D-675 chose.

Under D-424's own test — does the disputed claim change what anyone may
conclude? — each does. A reader of §10's table concludes a committed config
carries a key it does not; a reader of §1 or §U3-Z concludes the schema question
is still open when the schema has closed it and the engine now refuses the
answer they would write. That is not prose that constrains nothing, so it is a
fix and not an overrule candidate, exactly as the finding said of the instances
it named.

This is the dispatch's stated FAIL shape: **the cited instances were patched and
the class was left open**, in the same document, in the same section, behind the
same citation from the same line of source. Nothing here is a way the search can
produce a wrong answer — the schema refuses both keys by full path and
`config_validate_tests.rs:236-253` pins it — so the remedy is three more edits in
one file, not a code change.

---

## §3 The E MINOR-3 mutant, re-applied here

Not accepted from the closure. Applied, run, restored, verified.

**The mutation** (`crates/pistol-search/src/search.rs`, `one_ply_turn`, the
reviewer's own: pick the lexicographically GREATEST other legal cell instead of
the least):

```rust
    let partner = pistol_core::legal_placements(state.board())
        .into_iter()
        .rev()
        .find(|cell| cell != at)
```

**Clean baseline first** (`e_refusal_clean_90a77a8.log`):

```
running 6 tests
test a_one_ply_witness_with_no_partner_names_the_invariant_from_proof_first_move ... ok
test a_one_ply_witness_with_no_partner_names_the_same_invariant_from_proof_line ... ok
test a_one_ply_witness_pairs_the_completing_stone_with_the_least_other_legal_cell ... ok
...
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
EXIT=0
```

**Against the mutant** (`e_refusal_mutant_rev_90a77a8.log`):

```
test a_one_ply_witness_pairs_the_completing_stone_with_the_least_other_legal_cell ... FAILED

assertion `left == right` failed: proof_first_move pairs the completing stone with the LEAST other legal cell
  left: Some(Pair(Coord { q: 3, r: 0 }, Coord { q: 10, r: 0 }))
 right: Some(Pair(Coord { q: -8, r: 0 }, Coord { q: 3, r: 0 }))

test result: FAILED. 5 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
EXIT=101
```

**THE MUTANT DIES**, at `5 passed; 1 failed`, which is what the closure claimed.
The two refusal tests stay green under it, which is the control the finding
implied: the mutation is a wrong ANSWER and not a changed refusal.

**Restored**: `cp` of the pre-mutation copy back, then
`git diff --quiet -- crates/pistol-search/src/search.rs` → `EXIT=0`, and
`git status --porcelain` in the worktree → empty.

---

## §4 New findings introduced by the fix diffs

None BLOCKING, none MAJOR. Three MINOR, all in the same family: the fix round
added a mechanism and a measurement and did not carry the package's own
book-keeping with them.

### NEW-1 (MINOR) — the closure's E2 paragraph and its M5 rationale were falsified by the fix's own `site` parameter

`9ce9a7c` added `site: &'static str` to `TriggerColumns::at`. Three statements in
`docs/experiments/pre_phase2_sweep_CLOSURE.md` were not updated, in that commit or
in the two closure-only commits after it:

- `:536-537` — *"`TriggerColumns::at(state, threats, turns_from_root)` … in
  `crates/pistol-search/src/census.rs`"*. The signature has four parameters.
- `:539` — *"`root_census_site` now varies `turns_from_root` and nothing else"*.
  It also varies `site`.
- `:723` (§3, the reason M5 was substituted for R2's registered mutant) — *"the
  root site passing the wrong `turns_from_root`: **the one thing the two sites
  still differ in**"*. There are now two.

**Reproducer**: `/usr/bin/grep -n "TriggerColumns::at(state" crates/pistol-search/src/{pvs.rs,search.rs}`
→ four arguments at both sites; `/usr/bin/grep -n "site" docs/experiments/pre_phase2_sweep_CLOSURE.md`
→ no line records the parameter. This is the same class as MAJOR 3 (a document's
mutation-coverage argument coming loose from the code), one severity lower
because the closure is a package record rather than a rustdoc a caller reads.

### NEW-2 (MINOR) — `site` is provenance no test can see, and its swap-mutant survives by construction

The `site: &'static str` parameter has exactly ONE use in `TriggerColumns::at`'s
body: the format string of the `OVERLOAD_ON_A_DECIDED_POSITION` panic at
`census.rs:176` (`git grep -n "site" crates/pistol-search/src/census.rs` puts it
at `:151` in the signature, `:170` in a comment, `:176` in the panic; the `site`
at `:206` is `push`'s unrelated local). That panic fires
only when `StonesLeft::from_state(state)` returns `None` — a decided position —
and `git grep -n "trigger fired on a decided position" -- crates/ tools/ configs/`
finds the string only at its defining line. So **no test observes `site`**, and a
mutant that swaps `"root"` and `"in-tree"` at the two call sites cannot be killed
by anything in the workspace. The fix for MINOR 2 therefore added a discriminator
of precisely the A-21 class this same package exists to close, without pinning it
and without recording the attempted reproducer the way `0fedfa8` did for
`NO_COMPLETION_STONE` in the sibling commit's own comment nine lines away. Not a wrong-answer class —
its only effect is the wording of a crash report — which is why it is MINOR and
not MAJOR.

### NEW-3 (MINOR) — the "sixty-four colourings" measurement has no receipt

`0fedfa8` writes into `crates/pistol-search/src/quiescence.rs:418-426`, as a
shipped source comment, *"**MEASURED** over all sixty-four colourings of the
minimal enclosure … **none** has an empty tier 1"*, and the closure repeats it at
`:583-592`. The closure's §4 says *"Everything this package measured is under
`artifacts/pre_phase2_sweep/`"*, and it is not: `find . -type f | LC_ALL=C sort`
over those 78 files matches nothing for `probe|colour|color|enclos|completion`
(exit 1), and the 78-file digest reproduces as `8b1e3d30…` exactly, so the set is
the one the closure hashed rather than one that lost a file. A MEASURED claim
carried in shipped source with no receipt is unrepeatable by the next reader,
which is what the comment says it exists to spare them. It changes no code — the
guard stays a `panic!` under either outcome — so it is MINOR.

---

## §5 What was run, and what was exported

Logs copied to `artifacts/pre_phase2_confirm/` (gitignored) before either
worktree was removed, per D-469:

| file | what it holds |
|---|---|
| `e_gate17_90a77a8.txt` | `tools/file_justification_check.sh` at `90a77a8`, full output, `EXIT=0` |
| `e_config_check_90a77a8.txt` | `tools/config_check.sh` at `90a77a8`, `EXIT=0` |
| `e_refusal_clean_90a77a8.log` | `refusal_text_tests` clean, 6 passed |
| `e_refusal_mutant_rev_90a77a8.log` | the same suite against the `.rev()` mutant, 5 passed / 1 failed |
| `e_identity_cb6e853_vs_90a77a8.txt` | the byte-identity re-derivation across the fix rounds, 540 searches per side, census rows and digest |
| `e_noncomment_diff_cb6e853_90a77a8.txt` | every non-comment `src/` and `configs/` line the fix rounds changed |
| `e_receipts_90a77a8.txt` | §4's 78-file digest and the three census digests, re-derived |
| `e_major4_u3_residual_90a77a8.txt` | the three surviving `U3_tier_t.md` statements and the code citation that reaches them |
| `e_findings_evidence_90a77a8.txt` | per-finding quotes for MAJOR 1, MINOR 1, 2, 4, 5, 6 |
| `e_identity_transcripts/{pre,post}/` | the raw engine transcripts both sides of the identity comparison were diffed from |

Also run and reported from its own output: `cargo test -p pistol-search --lib`
(28 passed, including `search::tests::an_and_rooted_proof_reaching_the_outcome_names_its_invariant`,
E4's new driver).

**What I did NOT verify.** The sixty-four-colouring sweep itself (NEW-3) was not
re-derived; the finding is that its receipt is absent, not that its conclusion is
wrong. `tools/ci.sh` was not re-run at `90a77a8` — the run at `0fedfa8` is cited
from its own output and the diff to HEAD is one docs file, which is the closure's
own D-674 argument and it checks out. Group T's and group C's findings were out
of scope.

---

## §6 Group verdict

**FAIL — E MAJOR-4.**

Ten of eleven findings are CONFIRMED, and the code is in good order: the
byte-identity guard — group E's load-bearing claim — was re-derived across the
fix rounds by an instrument the closure did not use, at 540 searches per side and
158 census rows, with **zero differing lines anywhere**, and the only non-comment
code the fix rounds shipped is one `&'static str` panic discriminator and one
test module. Gate 17 is green, a full CI run exists inside the package, the
receipts digest reproduces exactly, and the wrong-answer mutant that survived the
review now dies.

The one FAIL is documentary and one file wide: `U3_tier_t.md` still states, in
§10's own document table and twice more in the present tense, the shape the
engine now refuses — the class MAJOR 4 named, patched at its four cited lines and
left standing at three others behind the same citation from
`crates/pistol-engine/src/config.rs:213`. Three edits in one file clear it; no
code moves and none of the byte-identity evidence needs retaking.
