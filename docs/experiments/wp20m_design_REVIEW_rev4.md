# SCOPED RE-REVIEW — `docs/experiments/wp20m_design.md` revision 4

## Header

- **Named revision adjudicated:** `a9a4a3a14d6dd4b5f22bc4c82368b0215b693198`
  (`docs(wp20m): the fix round — the receipt site that could not read its own
  report is replaced by a unit test that needs no engine, and the instrument
  version that never moved comes out of the digest`).
- **Matched HEAD when this review opened:** **yes.** `git rev-parse HEAD` →
  `a9a4a3a…`, branch `dev`, `git status --porcelain` showing one untracked file
  (`docs/experiments/wp20s_design_REVIEW.md`).
- **HEAD MOVED DURING THIS REVIEW and I re-anchored rather than assume.** HEAD is
  now `f96593b94e0da78711071367e6b8f5e214e1f321`
  (`docs(wp20s): the fix round — the parser that could not carry a two-token
  score…`), tree clean. **The subject is unchanged:**
  `git show a9a4a3a:docs/experiments/wp20m_design.md | diff - docs/experiments/wp20m_design.md`
  → **empty**. Everything below adjudicates a file that is byte-identical at
  `a9a4a3a` and at HEAD.
- **ONE GOVERNING DOCUMENT DID MOVE, and it changed a check.**
  `f96593b` added 167 lines to `docs/experiments/wp20_dispatches.md`,
  transcribing a THIRD dispatch and adding a new rule — *"A document quoting
  "the dispatch" must say which"*. That rule **post-dates the subject** and I do
  not grade the design against it. Every dispatch quotation below was
  re-verified against `git show a9a4a3a:docs/experiments/wp20_dispatches.md`,
  not against the working tree, which I had first read after the commit landed.
- **Prior revisions compared against:** `406ace9` (the reviewed revision),
  `7af62e7` (rev 2) and `5064b05` (rev 1), all read via `git show
  <sha>:docs/experiments/wp20m_design.md`.
- **What I ran:** `git`, `git grep`, `/usr/bin/grep`, `sed`, `awk`, `diff`,
  `comm`, `wc`, `cat`, `ls`, `sort`, `tr`, and `python3
  tools/design_citation_check.py` with the four `--proposes` paths. Every
  recorded grep is `/usr/bin/grep` or `git grep` (D-265).
- **What I refused to run, per the dispatch:** `cargo` in any form,
  `tools/ci.sh`, `tools/determinism.sh`, `tools/arena_smoke.sh`. Where a claim
  needs a run I name the run.
- **Read as binding:** `docs/experiments/wp20m_design_REVIEW_rev3.md` in full
  (all 23 findings), CLAUDE.md, `docs/process.md`, D-539 … D-547 in full,
  `wp20m_design_REVIEW.md`, `wp20m_design_REVIEW_rev2.md`, `wp20_dispatches.md`
  at `a9a4a3a`, `docs/book_v2_ledger.md`,
  `docs/experiments/matrix_wp20_shape_selection.md`.

---

## VERDICT: **FAIL**

**0 BLOCKING, 2 MAJOR, 6 MINOR.**

**Say the good part first, because it is most of the document and it is true.**
The BLOCKING is fixed and fixed well: §14's three-site replacement is cheaper,
more permanent and more honest than the `tools/` extension it replaces, and the
mischaracterisation that made the old §14 writable is corrected everywhere it
appeared. **Seven of eight MAJORs are APPLIED and I verified each against the
tree rather than against the document's word.** **All fourteen MINORs are
APPLIED.** The freeze audit is **clean** — I diffed `406ace9` against `a9a4a3a`
hunk by hunk, and every one of the sixteen hunks lands either on §0.2a's table or
in a section no reviewer passed. All six lifted blocks are word-identical to
their sources, §12 is word-identical to revision 1's §4, and the restored §2
sentence matches `7af62e7` exactly. `design_citation_check.py` is green over
**113** citations, and of the **forty-five** I read by hand — **including all
sixteen that are new in revision 4** — every one says what the document says it
says. **AUTHOR DEBT: none.** Not one finding below is in the half the checker
could have caught.

It fails on two things, and both are the thing this dispatch told me to hunt
for: **a fix round that introduced a new defect while applying a finding.**

> **N1 (MAJOR).** §14's answer to the BLOCKING moved
> `a_captured_totals_line_keeps_every_field_but_nps_and_time` from a stub-driven
> INTEGRATION test (revision 3, `406ace9:786-789`) to a unit test over a
> synthetic line (`:768`, `:1001-1002`). **Nothing now pins that pass 2 CALLS the
> normalisation.** Delete the call — leave the function and its unit tests intact
> — and every registered test in this package stays green. The registered mutant
> *"the normalisation removed"* (`:812`) survives under the reading that matters,
> and §9's "two things no test pins" does not declare it.
>
> **N2 (MAJOR).** §5 removed `arena_version` from the digest and argues over ten
> lines that it is a trap. **§4.3 still writes "the arena version" into the
> capture file's header as a `param` (`:454`), and still does not say which one.**
> `Transcript` has no such field (`crates/pistol-arena/src/transcript.rs:29-53`),
> so M1's decision — pass 2's own `env!`, or a new parse in `transcript::read`
> that brushes INVARIANT 10 — is exactly where rev-3 left it, moved one section.

Neither is a correctness defect in the mechanism. Both leave an implementer a
decision the design should have taken, and N1 is a regression: revision 3 had the
coverage and revision 4 spent it.

---

# PART 1 — DISPOSITION OF EVERY PRIOR FINDING

## BLOCKING

### B1 — §14's receipt site cannot execute — **APPLIED, BUT THE FIX INTRODUCED A NEW DEFECT (N1)**

§14 is replaced entirely (`:985-1060`) and the root mischaracterisation is
corrected in §7 (`:599-615`) and restated on the document's own face
(`:10-20`). I checked the four sub-questions the dispatch named.

**(a) Is the normalisation genuinely a pure function over a line, such that a
synthetic-line unit test is possible and non-vacuous? — YES.** The normalisation
is the removal of ` nps <n> time <n>` (§4.1 `:378-383`), and
`crates/pistol-cli/src/report.rs:82-84` is one format literal —
`nodes {}{solver_field} {NPS_FIELD} {} {TIME_FIELD} {}` — so `nps` and `time` are
adjacent in **both** spellings, the solver block being interpolated strictly
before them (`report.rs:62-81`, verified: `let solver_field = if
info.solver_nodes > 0 {` at `:62`, closing at `:81`). A `fn(&str) -> String` is
implementable, and a synthetic line built to `render_info`'s shape exercises it
without any engine. **Non-vacuous.**

**(b) Does §14.1 kill BOTH normalisation mutants, in BOTH spellings? — YES, all
four.** The mutant table registers four normalisation mutations (`:812-814` plus
INVARIANT 8's at `:773`) and each has a function-level killer:

| mutation | killer | dies? |
|---|---|---|
| the normalisation removed *(function body)* | `a_captured_totals_line_keeps_every_field_but_nps_and_time` | **yes** |
| widened to strip another NON-solver field | same | **yes** |
| widened to strip a SOLVER field | `the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line` | **yes** — and this is the one that died nowhere in revision 3 |
| *(INVARIANT 8's content)* | `two_totals_lines_differing_only_in_nps_and_time_normalise_equal`, both spellings | **yes** |

**The fifth mutation — the normalisation removed from the CALL SITE — dies
nowhere.** That is **N1**, below.

**(c) Is §14.4's refusal to touch `tools/` sound, or is the design declining the
only permanent guard for its headline invariant? — SOUND.** I attacked this hard
because it is the shape of an evasion. It is not one:

- INVARIANT 6's normalisation clause gets a **permanent, in-CI** guard from
  §14.1's unit tests — which the `tools/` route would not have improved on.
- INVARIANT 6's `bestmove` clause is pinned by
  `a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote`, stub-driven
  through the real `pistol_cli::Session` (`stub_engine.rs:266`), so those are real
  formatter bytes.
- INVARIANT 8's non-normalisation content (record order, header, digest) is
  pinned permanently by `a_rerun_over_one_report_is_byte_identical` (§14.3).
- The one coupling nothing here pins — that `render_info`'s field ORDER keeps
  `nps` and `time` adjacent — is pinned by an **inherited standing gate**, gate
  9's own `sed` at `tools/determinism.sh:153-154`, which the design cites at
  `:390`.
- The cost §14.4 declines to pay is stated accurately (`:1053-1057`): a second
  committed `nodes`-budget arena config on gate 6's path, and a second arena run
  inside gate 15 whose cost is pre-registered on its own face
  (`tools/arena_smoke.sh:29-33`).

One over-claim, graded **N8 (MINOR)**: *"to buy what §14.1 buys for free and what
§14.2 buys anyway"* (`:1057`) counts the pilot's **one-time** receipt as
equivalent to a standing gate. It is not, and the document does not say that the
standing guard on the real engine's spelling is gate 9's, not this package's.

**(d) Is the `depth_turns`/`nodes` fact now stated correctly EVERYWHERE it
appears? — YES. The sweep is clean.**

```
$ /usr/bin/grep -n "movetime\|MovetimeMs\|164-170\|depth_turns" docs/experiments/wp20m_design.md
```
returns 16 lines. **Not one claims `transcript.rs:164-170` is about movetime.**
`:601-602` says it *"tests `kind != "nodes"` and refuses every budget kind but
`nodes` — `depth_turns` included"*, which is exactly
`crates/pistol-arena/src/transcript.rs:164` (`if kind != "nodes" {`, verified).
`:594` correctly narrows the remaining movetime claim to **two** places, both
verified: `config.rs:132` is the `MovetimeMs { .. } => None` arm and
`arena.rs:229` is `unreachable!("validate refuses a movetime budget before this
point")`. The test is renamed to
`a_capture_over_a_report_whose_budget_is_not_nodes_is_refused_by_name` (`:787`),
and §7 says so as a correction rather than silently (`:629-632`). **The
consequence is recorded as a requirement on pass 1** (`:608-615`), and
`configs/arena_smoke_v0.toml:54-58` is quoted as the report that therefore cannot
be an input — verified: `:57` is `kind = "depth_turns"`.

**Disposition: APPLIED BUT INTRODUCED A NEW DEFECT (N1).**

## MAJOR

### M1 — §5 does not say WHICH `arena_version` — **PARTIALLY APPLIED. This is N2.**

The digest input is gone (`:487-494`, three inputs), and INVARIANT 12 follows
(`:720-722`). **But §4.3's header `param` list was not touched**, and it is the
one place the value must actually be produced:

```
docs/experiments/wp20m_design.md:453-455
`crates/pistol-cli/src/corpus/mod.rs:6`). The header carries, as `param`, the
capture format version, the arena version, the source report's
`experiment_sha256` and `source_sha256`, and the label `go` line; and as
```

**Verified against the tree, not the document:**
`git grep -n "arena_version" -- crates/` puts it at exactly two writers,
`crates/pistol-arena/src/report.rs:130` and
`crates/pistol-arena/src/replay_report.rs:99`, both `env!("CARGO_PKG_VERSION")`,
and at **no reader**. `Transcript` (`crates/pistol-arena/src/transcript.rs:29-53`)
carries ten fields and none is `arena_version`. Every other `param` on §4.3's
list has a source pass 2 holds — the format version is its own constant,
`experiment_sha256` and `source_sha256` are `Transcript` fields (`:47`, `:50`),
the label `go` line is its own flag. **`arena_version` alone has none, and §4.3
does not say where it comes from.**

Worse than a leftover: §5 now argues at length that *"a reader takes the second
[hand-maintained number] for a mechanism"* (`:504-505`), and §4.3 puts that exact
number in front of that exact reader — under `param`, which
`crates/pistol-cli/src/corpus/emit.rs:36` defines as *"An input the extraction was
run with"* and `:40-41` distinguishes from a measurement precisely so *"a reader
has to be able to tell a choice from a measurement"*.

**Disposition: PARTIALLY APPLIED — graded MAJOR as N2.**

### M2 — the instrument input buys no protection — **APPLIED, and its evidence is MEASURED and true**

Every limb verified:

- *"`git log -p --follow -- Cargo.toml` shows one `version = "0.0.1"`, from the
  first commit"* (`:501-502`).
  `git log -p --follow -- Cargo.toml | /usr/bin/grep "^[+-]version"` →
  **exactly one line**, `+version = "0.0.1"`. `Cargo.toml:6` is `version =
  "0.0.1"`; `crates/pistol-arena/Cargo.toml:3` is `version.workspace = true`.
  **The claim is exact.**
- *"The rule asks that an artefact producing a registered number be "named in the
  pre-registration WITH ITS REVISION""* (`:508-510`). `docs/process.md:13-16`:
  *"An artefact that produces a registered number … is named in the
  pre-registration WITH ITS REVISION"*. **Quoted exactly, and the design's reading
  is the rule's own** — a commit SHA is a revision; a package version is not.
  Revision 3 cited it for something it does not say, and this revision says so.
- **The FIX asked for one of two things and the design takes BOTH**: `:515-521`
  states plainly what the digest does not protect against, and puts the governing
  revision in the pilot's pre-registration. That paragraph is the honest half of
  M2's remedy and I could not break it.

**Disposition: APPLIED.**

### M3 — INVARIANT 4's mutant cannot die — **APPLIED. Verified implementable, non-disturbing, and lethal.**

The remedy is a new stub behaviour `demands_newgame_per_ask` (`:791-804`).

**Is the latch real?** Yes. `crates/pistol-arena/src/bin/stub_engine.rs:268` is
`let mut told_new_game = false;`, set at `:274` inside
`if asked.starts_with(NEW_GAME)`, and **never cleared anywhere in the file**. The
design's range `:267-289` contains it (`:267` is `config_edited`, one line early
— it contains the fact, it does not point at the wrong one).

**Is it implementable as described?** Yes, and additively. `Behave` is a
`#[derive(Clone, Copy)]` enum at `:13-54` with a `parse` at `:56-71` keyed on a
word; the `DemandsNewGame` block is one `if behave == Behave::DemandsNewGame`
guard at `:271-290`. A second variant plus a parse arm plus a clear-on-`go` is a
strictly additive edit.

**Does it disturb `demands_newgame` or its two witnesses?** No. The existing
guard tests one variant by equality, so a sibling variant cannot reach it, and
`crates/pistol-arena/tests/seat_setup_identity_tests.rs:189` and
`crates/pistol-arena/tests/replay_tests.rs:278` both drive the string
`demands_newgame`.

**Does the mutant really die?** Yes, and I traced it. Under
`demands_newgame_per_ask` the flag clears on `go`. Pass 2's loop is
`newgame → position → go` per asked position (`:166-168`). Delete every in-loop
`newgame`: the spawn's own `NEW_GAME` (`crates/pistol-arena/src/seats.rs:47`)
satisfies ask 1; ask 1's `go` clears the flag; **ask 2's `position` earns the
named `error` line** (`stub_engine.rs:279-286`), which §6 (`:556`) maps to a
refusal of the run, and `every_label_go_is_preceded_by_a_newgame` fails. Any game
of one or more turns yields two or more asked prefixes (§2's range, `:256-258`),
so the second ask always exists.

**Is a stub behaviour within this package's licence?** Yes.
`crates/pistol-arena/src/bin/stub_engine.rs` is in `pistol-arena`; no engine crate
is touched; and the file already carries its rule-9 entry
(`docs/rule9_justifications.md:22`), whose ground — *"every deviation this
instrument makes lives in one file on purpose"* — is an argument **for** putting
the new behaviour there rather than against it. The design's citation of the
existing doc's own reasoning (`stub_engine.rs:43-53`, *"deleting the send left the
whole workspace green"*, D-413) is exact.

**Disposition: APPLIED.** This is the strongest single fix in the round.

### M4 — "the vacuity is confined to one test" is false — **APPLIED**

`the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line` is
registered (`:769`), the SOLVER mutant gets its own row (`:814`), and the true
bound is stated at `:1009-1020`. Evidence verified: `report.rs:62` is
`if info.solver_nodes > 0`, `stub_engine.rs:123` is `solver_nodes: 0`,
`configs/gate_v0.toml:94` is `on_search_path = false` with `:92`'s comment *"gate
OFF in every committed config"*. **Disposition: APPLIED.**

### M5 — §13(b) is internally contradictory — **APPLIED, and it opens no new gap**

§13(b) is reversed (`:958-977`): the pilot's pre-registration lands
`configs/arena_wp20_label_pilot.toml` and its ledger row together; this package
lands no arena experiment config and owes only the constraint.

**Is anything now owed by no package?** No — I walked every limb.
`configs/arena_wp20_label_pilot.toml` → the pilot, declared as such in §0.1's
`--proposes` block (`:44-46`). The `book_v2` row → the pilot. Requirement 5(a),
the corpus manifest → this package (§13a). Requirement 5(c) → WP-2.0b (D-539).
Requirement 4's real-binary receipt → the pilot (§14.2), which is where the
governing dispatch already put it (`a9a4a3a:docs/experiments/wp20_dispatches.md`
§4 of the third dispatch: *"the determinism re-run receipt on a sub-range"*).

**Is the constraint the design keeps correct and sufficient?** Correct: pass 2
reads through `transcript::read`, which refuses `kind != "nodes"`
(`transcript.rs:164`). Sufficient for the purpose stated. §7, §1 and §11 are all
consistent with the reversal — §7 (`:608-615`) states the constraint, §1
(`:157-162`) constrains only the engine sections, §11 (`:859`) keeps the numbers
with the pilot.

**Ledger citations verified exactly.** `docs/book_v2_ledger.md:16` is *"**The
rule.** A new pre-registration takes the next unconsumed range, adds its row here
in the same commit that adds its arena config…"*, and the table closes at `:51`
with *"because neither has a committed pre-registration drawing from this book."*
**Disposition: APPLIED.**

### M6 — the dispatch's ledger mutant left homeless — **APPLIED**

Named at `:946-956`, with the property it guarded and the substitute. The quote
*"ledger overwrite → append test dies"* is verified at the governing revision
(`a9a4a3a:docs/experiments/wp20_dispatches.md:106`, `->` rendered as `→`).
**Disposition: APPLIED.**

### M7 — INVARIANT 7's test names an impossible comparison — **APPLIED. The causal chain is true in every link.**

The invariant is restated (`:710-711`) and the test renamed
(`the_sprt_reports_per_game_node_counts_are_billed_from_the_totals_line`,
`:772`). **I traced the chain the design asserts at `:836-846` end to end:**

1. `crates/pistol-arena/src/exchange.rs:169-188` is `fn totals_of`, three `?`
   lookups at `:185-187` (`nodes`, `TIME_FIELD`, `depth_turns`). A fourth
   `value("solver_nodes")?` returns `None` for a solver-off config
   (`configs/gate_v0.toml:94`), so `totals_of` returns `None`. **TRUE.**
2. `exchange.rs:76-79` is `if let Some(totals) = totals_of(&line) { compute.add(…)
   ; continue; }`. With `None`, control falls to `:80`'s `INFO_PREFIX` arm and
   **`continue`s** — so the run completes rather than forfeiting, and
   `compute.add` never fires. **TRUE, and the "run completes" limb matters: a
   forfeit would have been observable by other means.**
3. `crates/pistol-arena/src/conclusion.rs:40,46-47` writes `nodes_a {} nodes_b {}`
   per game from `record.compute[0].nodes` / `[1].nodes`. **So the per-game counts
   go to zero. TRUE.**
4. The renamed test asserts they are the stub's own counts and not zero. **It
   dies.**

**A bonus the design does not claim and I record so nobody re-derives it:** the
mutant already dies in the tree. `crates/pistol-arena/tests/run_tests.rs:98`
(`report_contains_per_side_compute_fields`) asserts `nodes_a > 0` per game, and
`stub_engine.rs:116-117` comments that its non-zero nodes exist *"so a test that
asserts per-side compute was recorded must be able to fail"*. The registered test
is a strengthening, not a duplicate. **Disposition: APPLIED.**

### M8 — the config-path precondition — **APPLIED**

Stated at `:314-326`, and every limb verified against the tree:
`crates/pistol-cli/src/bin/pistol.rs:142` is `fn identity_lines`, and `:163` is
`format!("config {}", path.display())` — the path **as spelled**;
`crates/pistol-arena/src/identity.rs:82` is `id_lines: spoken.lines` and `:84` is
`config_sha256: digest_of(&engine.config)?` — the file's **content**. The
fails-safe argument at `:328-332` is right: `binary_sha256` is computed before the
spawn (`identity.rs:65`) and re-checked against the declared digest
(`:66-72`), so two different engines cannot compare equal. `configs/arena_smoke_v0.toml:82`
and `:88` are both `config = "configs/gate_v0.toml"`, spelled identically —
exactly as the design says.

**Is the new unit test writable?** Yes. `EngineIdentity` is a `pub struct` with
four `pub` fields (`identity.rs:13-22`); two values differing only in `id_lines`
are constructible in-crate and passed to pass 2's own refusal function. One
residual tension, graded **N5 (MINOR)**. **Disposition: APPLIED.**

## MINOR — all fourteen APPLIED

| # | disposition | evidence |
|---|---|---|
| **m1** | **APPLIED** | The sentence *"Revision 1's INVARIANT 4 forbade "exclusion by outcome" and would have forced the hang."* is restored inside the quoted block (`:241-242`). I word-diffed `:223-244` against `7af62e7:73-94`: **identical**. §0.2's row (`:70`) lists the restoration with its ground |
| **m2** | **APPLIED** | §0.2's §6 row (`:74`) now names the rewording *"its two engine sections differ"* → *"its two seats attest different engines"* and grounds it on §3's BLOCKING A remedy |
| **m3** | **APPLIED** | §0.2's last row (`:78`) names the row actually added — *"WHAT is written, and in what grammar"* — discloses the dropped word *"four"*, and says the slot went into §3's existing row. I confirmed the drop is the only text change besides a trailing `:` → `.` |
| **m4** | **APPLIED** | §14.4 (`:1042`) is now *"by a route that survives `cargo test -p pistol-arena`"* and names the rejected route with its citation. `crates/pistol-arena/tests/sprt_power_tests.rs:19-37` verified: a `current_exe()`-relative path to an **example**, with `:20-24`'s own doc explaining why `CARGO_BIN_EXE_` does not apply |
| **m5** | **APPLIED** | `:305` now cites `identity.rs:12`. Verified: `:11` is the doc line, `:12` is `#[derive(Debug, Clone, PartialEq, Eq)]` |
| **m6** | **APPLIED** | The TAB clause moved to INVARIANT 6 as a **write**-side refusal (`:706-709`, argued at `:435-441`), and the loader gains *"or any of whose five fields is empty"* (`:466-471`) with `a_capture_record_with_an_empty_field_is_refused_by_name` and its mutant |
| **m7** | **APPLIED** | `a_capture_identity_moves_when_the_format_version_moves` (`:785`) and *"the format version dropped from the identity"* (`:831`) both registered. Residual on a **different** input → **N6** |
| **m8** | **APPLIED** | Mutant sharpened to *"a capture record's **first two fields swapped** on write"* (`:826`), test renamed `…_field_by_field` (`:780`), invariant restated (`:717`). Residual: a field-by-field assertion kills the swap only if the fixture's game index and `k` differ, which the design does not say — thin, and I do not grade it |
| **m9** | **APPLIED** | §6 gains *"the closing totals line is not recognised as one"* (`:560`), with `an_unrecognised_totals_line_refuses_the_run_and_names_the_game_and_turn` (`:779`) and its mutant (`:824`) |
| **m10** | **APPLIED** | INVARIANT 3's slot-zero limb is declared unpinned at `:738-743`, **with a reason rather than an apology** — once the identities are attested equal no observation can distinguish the slots |
| **m11** | **APPLIED** | The driver is named: *"unit, over the record writer"* (`:771`), argued at `:438-441` |
| **m12** | **APPLIED** | §11 (`:864-872`) assigns the throughput obligation to the pilot and separates it from §12's memset cost. The quote is exact at the governing revision (`a9a4a3a:wp20_dispatches.md:95-96`) |
| **m13** | **APPLIED — one limb MOOT** | The matrix limb is MOOT: §14.4's decision keeps the claim true, and I verified `docs/experiments/matrix_wp20_shape_selection.md` §2, line 61 — *"only `pistol-arena` is touched"* — is not falsified by anything the design now proposes. The `capture_sha256` limb is applied as §15 (`:1064-1071`), and its grep claim is exact: `/usr/bin/grep -c "capture_sha256" docs/decisions.md` → **0**, `label_sha256` → **1** |
| **m14** | **APPLIED** | §1 gains the bullet at `:209-214` naming both the `USAGE` paragraph and the fallback refusal's mode list. `crates/pistol-arena/src/bin/arena.rs:94-99` verified: the `_ =>` arm's message enumerates exactly the two existing modes |

**Totals: B1 applied-with-new-defect; MAJORs 7 APPLIED / 1 PARTIALLY; MINORs 14
APPLIED / 0 outstanding.**

---

# PART 2 — THE FREEZE AUDIT (D-547)

**Method.** `git diff 406ace9 a9a4a3a -- docs/experiments/wp20m_design.md`, all
**723** lines, **sixteen hunks**, each mapped by hand to a section and then to
§0.2/§0.2a. Then every `> `-quoted block reduced to a whitespace-normalised word
stream and `diff`ed against the same reduction of its named source.

## (a) Is every change on §0.2a's table or in a section no reviewer passed?

**YES. No silent edit to a frozen section.**

| hunk | § | frozen? | listed? |
|---|---|---|---|
| `-1,8` | header | no (rev-3 material; *"Nothing in revision 3 is frozen"*, `:83`) | — |
| `-19,7` | §0.1 `--proposes` block | no | — |
| `-43,19` | §0.2 + new §0.2a | no | — |
| `-159,6` | §1 `USAGE` bullet | **YES** | §0.2a row 1 ✓ |
| `-185,7` | §2 quoted block | **YES** | §0.2 row (`:70`) **and** §0.2a row 3 ✓ |
| `-248,14` | §3.1 | no (rev-2 BLOCKING A) | §0.2a ✓ |
| `-345,10` | §4.2 | no (new in rev 3) | §0.2a ✓ |
| `-373,7` | §4.3 loader prose | no (prose below the lift) | §0.2a ✓ |
| `-389,24` / `-426,8` | §5 | no (rev-2 MAJOR 5 NOT APPLIED) | §0.2a ✓ |
| `-444,6` | §6 table row | **YES** | §0.2 row (`:74`) **and** §0.2a row 6 ✓ |
| `-477,12` / `-496,10` | §7 prose below the lift | **YES** | §0.2a row 2 ✓ |
| `-573,98` | §9 + §10 | no (rev-2 MAJOR D) | §0.2a ✓ |
| `-679,6` | §11 | no (rev-2 MAJOR 9) | §0.2a ✓ |
| `-751,15` / `-767,45` | §13, §14, §15 | no | §0.2a ✓ |

**§12 is not in any hunk. It was not touched.**

## (b) Are the LIFTED VERBATIM blocks still byte-for-byte their sources?

**YES — all six, including the fourth that revision 4 newly rowed.** Word-stream
comparison after stripping the `> ` prefix:

| block | rev 4 | source | result |
|---|---|---|---|
| §3.2 engine verification | `:350-353` | `7af62e7:110-113` | **IDENTICAL** |
| §4.1 THE ONE NORMALISATION | `:378-388` | `7af62e7:121-131` | **IDENTICAL** |
| §4.3 THE FILE'S SHAPE | `:445-449` | `7af62e7:133-137` | **IDENTICAL** |
| §4.4 THE SOURCE IS NAMED | `:477-481` | `7af62e7:139-143` | **IDENTICAL** |
| §7 the budget's kind | `:586-591` | `7af62e7:178-183` | **IDENTICAL** |
| §2 which positions are asked | `:223-244` | `7af62e7:73-94` | **IDENTICAL** (with m1's sentence back) |

The §0.3 capture-decisions lead-in differs from `7af62e7:27-31` by exactly two
things: the word *"four"* (disclosed at `:78`) and a trailing `:` → `.` (not
disclosed, correctness-neutral, not graded). The D-544 premise lift is
**IDENTICAL** to `7af62e7:39-43`.

## (c) Is §12 still word-identical to revision 1's §4?

**YES.** `:881-897` against `5064b05:80-97`: **IDENTICAL**, D-540's fresh-process
criterion and its defect-class clause included.

## (d) Was anything revision 3 had that a reviewer credited lost?

**No.** L1–L7 all survive; §4.1, §12, INVARIANT 6's `bestmove` clause, the loader
tests and the §14.2 pilot-receipt sentence all carried forward. Everything
removed was graded defective by the rev-3 review: §5's `arena_version` argument
(M1/M2), §14's `tools/` extension (B1), `raising_totals_of_leaves_the_sprt_report_byte_identical`
(M7), `a_capture_over_a_movetime_report_is_refused_by_name` (B1),
*"the vacuity is confined to one test"* (M4), §13(b)'s config assignment (M5).

**One thing revision 3 had that revision 4 spent, and it was not graded
defective:** the stub-driven integration coverage of the normalisation
(`406ace9:786-789`, *"`a_captured_totals_line_keeps_every_field_but_nps_and_time`
is **not vacuous against the stub**"* — a sentence the rev-3 review verified as
TRUE in its check 7(d)). **That is N1.** It is the one true loss in this round
and it is the reason for the verdict.

**One error on the face of the disclosure itself, graded N3:** §0.2a's lead-in
(`:87-89`) says *"**Two** of this round's edits land in sections frozen by an
earlier review"*. **Four do** — §1, §2, §6 and §7. All four are listed with
grounds, so D-547's substance is met; the count and the "the rest" that follows it
are wrong.

---

# PART 3 — NEW DEFECTS

### N1 (MAJOR) — nothing pins that pass 2 CALLS the normalisation; §14's fix spent the test that did

**What the document says.**

```
docs/experiments/wp20m_design.md:768
| `a_captured_totals_line_keeps_every_field_but_nps_and_time` | 6 | **unit, synthetic totals line** |

docs/experiments/wp20m_design.md:998-1002
normalisation is a pure function from one line to one line. Registered over
SYNTHETIC totals lines:
- `a_captured_totals_line_keeps_every_field_but_nps_and_time` — the non-solver
  spelling: every field of `render_info`'s output survives but those two.

docs/experiments/wp20m_design.md:812
| the normalisation removed | `a_captured_totals_line_keeps_every_field_but_nps_and_time` |
```

**What revision 3 had.**

```
406ace9:786-789
**`a_captured_totals_line_keeps_every_field_but_nps_and_time` is not vacuous
against the stub**: the stub emits those two fields, the capture must not carry
them, and every other field must survive — so both normalisation mutants die
in-crate.
```

**Why it is wrong.** *"The normalisation removed"* has two mutation sites and the
design registers one killer:

- **Site A — the function's body** (`normalise(line) → line`). The unit test
  dies. ✓
- **Site B — the call** (pass 2 writes the raw totals line into field 4). The
  unit test **passes**: the function is intact and its synthetic input is
  unaffected.

**I walked every other registered test for site B and none of them fails.**
`a_rerun_over_one_report_is_byte_identical` cannot — §14.3 says so in terms
(`:1033-1036`): the stub's `nps`/`time` are constants
(`stub_engine.rs:126-127`), so two runs agree with or without the normalisation.
`the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line` and
`two_totals_lines_differing_only_in_nps_and_time_normalise_equal` are the same
unit shape. `a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote`
is the **bestmove** line. `a_capture_file_round_trips_through_its_own_loader_field_by_field`
round-trips whatever was written. `a_capture_whose_body_digest_is_wrong…`,
`…wrong_field_count…`, `…empty_field…` are loader refusals.
`a_captured_field_containing_a_tab_refuses_the_run_by_name` is over the record
**writer** — and note that the design describes THAT one as *"unit, **over the
record writer**"* (`:771`) while describing this one as *"unit, **synthetic totals
line**"* (`:768`), so the distinction is the document's own and my reading is not
a strained one.

**Site B breaks INVARIANT 6 and INVARIANT 8 against any real engine**, and it is
the mutation an implementer is most likely to introduce, because the call is one
line in a loop and the function is a tidy tested unit. §9 promises *"TWO THINGS
NO TEST PINS, BOTH DECLARED HERE RATHER THAN LEFT FOR A REVIEWER TO FIND"*
(`:724-725`). **This is a third, and it is not declared.** That is the D-527 class
the design invokes against itself at `:750-752`.

**Every other unit test in this round has an integration companion; this one
alone does not.** `two_identities_differing_only_in_an_id_line_are_refused_naming_that_line`
(unit) is companioned by `a_report_whose_seats_attest_different_engines_is_refused_by_name`
with its own mutant *"the seat identity check removed"* (`:819`). The
normalisation has no such pair.

**FIX (one clause).** Either restore the integration limb — *"and
`a_captured_totals_line_keeps_every_field_but_nps_and_time` is ALSO asserted over
a capture file produced against the stub, whose ` nps 1 time 0` the record must
not carry, so the mutant "the normalisation not called" dies too"* — or register
`a_captured_record_carries_the_normalised_totals_line` beside it and give the
mutant table a *"the normalisation not applied on the write path"* row.

### N2 (MAJOR) — §4.3 still writes `arena_version` into the header, and still does not say which one

Stated in full under **M1** above. Evidence: `:454` against
`crates/pistol-arena/src/transcript.rs:29-53` (no such field) and
`git grep -n "arena_version" -- crates/` (two writers, both `env!`, no reader).

**FIX (one clause), either direction.** *"`arena_version` is pass 2's own
`env!("CARGO_PKG_VERSION")` and is a `param` for provenance only — it is not a
digest input, for the reason §5 gives"*; or drop it from the header too, which
is what §5's own argument points at.

### N3 (MINOR) — §0.2a's frozen-edit count is wrong on the face of the freeze disclosure

`:87-89` says two; §1, §2, §6 and §7 are four. All four carry grounds, so nothing
is hidden — but the sentence tells a reader of the block D-547 exists to make
trustworthy that two sections need re-derivation when four do.
**FIX:** *"Four of this round's edits land in sections frozen by an earlier
review: §1, §2, §6 and §7."*

### N4 (MINOR) — INVARIANT 10's evidence sentence names one touched SPRT-path file when this package touches two

`:728-730`: *"the only file on the SPRT path this package touches is
`crates/pistol-arena/src/exchange.rs`, and only its one visibility keyword"*.
**§1 of the same document** says this package also edits
`crates/pistol-arena/src/bin/arena.rs` — a `Mode` variant, a dispatch pattern and
a call (`:186-187`), a `USAGE` extraction (`:198-203`), and now the fallback
refusal's mode list (`:209-214`, m14's own remedy). `bin/arena.rs` is the binary
that runs the SPRT.

**The conclusion survives** — §1 shows the existing arms and `outpath::claim`
(`arena.rs:103`, before the match) are untouched, so pass 1's behaviour is
unchanged — but the sentence carrying INVARIANT 10's whole evidentiary weight is
false as written, and revision 4's own m14 fix widened the gap. Carried from
revision 3, where the rev-3 reviewer read "the SPRT path" narrowly and passed it.
**FIX:** *"the only file whose SPRT-path BEHAVIOUR this package changes is
`exchange.rs`, and only its one visibility keyword; `bin/arena.rs` gains a match
arm beside the existing two, which §1 shows leaves both untouched."*

### N5 (MINOR) — §3.1 says the check needs no new predicate and then requires one

`:305-306`: *"the check is `identities[0] == identities[1]` and **needs no new
predicate**"*. `:324-325`: *"**the refusal names the field that differed**"* — and
the mutant table registers *"`id_lines` dropped from **the identity comparison**"*
(`:820`), which presupposes a comparison with separable fields. A derived
`PartialEq` over the whole struct cannot name a field, and cannot have one
dropped from it without being rewritten. Both readings produce the same code
(`==` to detect, a field walk to report), so under D-424 this is thin — but the
document states the rule twice and differently, which is D-544's second recorded
shape. **FIX:** *"the detection is `identities[0] == identities[1]`; the refusal
message then walks the four fields to name the one that differed."*

### N6 (MINOR) — INVARIANT 12's `experiment_sha256` has no test that varies it and no mutant that drops it

`:541-542` promises *"a pure function over them so that a test can vary each one
in turn"*. Three digest tests are registered (`:784-786`). **Drop
`experiment_sha256` from the digest and all three pass:**
`two_reports_of_one_experiment_share_a_capture_identity` passes more easily, and
the other two vary different inputs. The registered mutant *"`source_sha256` used
as the identity"* (`:830`) covers substitution, not omission.
**FIX:** one mutant row, *"`experiment_sha256` dropped from the identity →
`two_captures_of_different_experiments_do_not_share_an_identity"*.

### N7 (MINOR) — §6 owns the failure modes and carries no TAB row

m9's remedy is grounded on §6 owning failure modes (D-423), and revision 4 added
the malformed-totals row (`:560`) on exactly that reasoning. The TAB refusal is a
refusal of the whole run, naming the game and turn (`:432-433`), and it appears in
§4.2 and in INVARIANT 6 (`:708-709`) and **not** in §6's table (`:552-561`).
Defensible — a write-side check is not "an ask failing" — but the asymmetry with
the row this same round added is now visible. **FIX:** one row, or one clause in
§4.2 saying why the write side is not §6's.

### N8 (MINOR) — §14.4 counts a one-time pilot receipt as a standing gate

`:1057`: *"to buy what §14.1 buys for free and what §14.2 buys **anyway**"*. §14.2
buys a receipt taken once, in the pilot; the declined `tools/` route would have
bought a gate that runs on every CI. The refusal is still right (see B1(c)), but
the sentence prices the alternative wrongly. **FIX:** name gate 9's own `sed`
(`tools/determinism.sh:153-154`) as the standing guard on the coupling, which is
true and is the complete answer.

## Sweeps run and clean

- **Does the removal of `arena_version` leave any other section referring to
  it?** **YES, one — §4.3.** That is **N2**. `:98` (the §0.2a row), `:497`, `:499`
  are the removal's own argument and are correct.
- **Does the §13(b) reversal leave §7, §1 or §11 inconsistent?** **No.** All three
  checked line by line; §0.1's `--proposes` declaration was updated to match
  (`:44-46`).
- **Do the renamed tests leave any mutant pointing at a name that no longer
  exists?** **No.** I extracted all 34 registered test names and all 25 mutant
  targets and `comm`'d them: every mutant target resolves (the one non-name row is
  *"forfeited or book positions skipped | their two tests"*). The two test-shaped
  names appearing outside the table are
  `a_movetime_label_budget_is_refused_by_name` (deliberately not registered,
  argued at `:626`) and `raising_totals_of_leaves_the_sprt_report_byte_identical`
  (revision 3's, explicitly retired at `:844-846`).
- **Closure of the sets.** 12 invariants, 34 tests, 25 mutants. Every invariant
  has at least one test except **10** and INVARIANT **3**'s slot-zero limb, both
  **declared** (`:727-743`). No test pins nothing. No two invariants are in
  tension. Mutants that cannot die: **one** — *"the normalisation removed"* under
  site B (**N1**). Mutants unregistered where one is owed: **one** —
  `experiment_sha256` dropped (**N6**). Rules with neither test nor mutant: **one**
  — *"the totals line carries no score at all → capture it as written"* (`:561`),
  which the rev-3 review recorded without a finding ID and which revision 4 did not
  address; **carried, not newly graded**.
- **D-483 numeral sweep — PASS.** 134 distinct numerals; I classified every one
  after stripping citations, section numbers, invariant numbers, D-keys, gate
  numbers, revision SHA prefixes and `sha256`. What remains is field indices (1-5),
  rule numbers, pass numbers, `identities[0]`, `p1`/`p2`, the review's own finding
  counts (*"1 BLOCKING, 8 MAJOR, 14 MINOR"* — facts about a document, not
  measurements a prereg consumes), and the quoted evidence string `version =
  "0.0.1"`. **No budget value, no node count, no threshold, no range, no line
  count.**
- **The citation checker.**
  ```
  $ python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/capture.rs \
      --proposes crates/pistol-arena/src/usage.rs --proposes docs/label_corpus_manifest.md \
      --proposes configs/arena_wp20_label_pilot.toml docs/experiments/wp20m_design.md
  docs/experiments/wp20m_design.md: 113 citation(s) checked, 0 unreproduced
  ```
  Exit 0. All four proposed paths confirmed absent from the tree.
- **The half the checker cannot do.** I read the cited lines for **forty-five**
  citations, **including all sixteen new in revision 4** — `identity.rs:12`,
  `:56-87`, `:74-82`, `:84`; `pistol.rs:142-166`; `stub_engine.rs:43-53`,
  `:120-131`, `:262-266`, `:267-289`; `channel.rs:96-106`; `report.rs (cli):62-81`;
  `arena.rs:94-99`; `sprt_power_tests.rs:19-37`; `configs/arena_smoke_v0.toml:54-58`
  and `:82`/`:88`; `configs/gate_v0.toml:94`; `transcript.rs:164-170` — plus
  `transcript.rs:29-34`/`:43-45`/`:135-209`, `exchange.rs:76-79`/`:169-188`,
  `report.rs (arena):130`/`:61-74`, `report.rs (cli):82-97`, `conclusion.rs:40,46-47`,
  `config.rs:114-134`, `validate.rs:39-45`, `arena.rs:82-100`/`:103`/`:226-229`,
  `emit.rs:36-44`, `lib.rs (cli):36`, `corpus/mod.rs:6`, `lib.rs (arena):41-45`,
  `common/mod.rs:8-11`, `determinism_tests.rs:99`, `book_v2_ledger.md:16` and its
  closing text, `docs/process.md`'s Instrument rule, `matrix…md` §2, four
  `wp20_dispatches.md` quotes at the governing revision, and §15's own grep.
  **Every one supports the claim built on it. Not one off-by-one.** The single
  imprecision is `stub_engine.rs:267-289`, whose range begins one line before the
  flag it points at and contains it.
- **AUTHOR DEBT: none.** Both MAJORs and all six MINORs are in the half the
  instrument explicitly disclaims. That is the second consecutive round with zero
  author debt and it should be recorded as the D-543 remedy working.

---

# PART 4 — THE VERDICT

## **FAIL — 0 BLOCKING, 2 MAJOR, 6 MINOR.**

## Could an implementer build from this without deciding something the design should have decided?

## **No.** Two decisions — and the list is by far the shortest this arc has produced.

1. **How `a_captured_totals_line_keeps_every_field_but_nps_and_time` is driven,
   and therefore whether "the normalisation not called" is a mutation anything can
   kill** (**N1**). This decides whether the package's headline invariant is pinned
   or vacuous — the D-527 class, which this arc has now paid for five times.
2. **Which `arena_version` goes in the capture header** — pass 2's own `env!`,
   which is free, or the source report's line, which is not on `Transcript` and
   whose parse would touch pass 1's reader against INVARIANT 10 (**N2**).

Revision 3's list was seven. **Five of those seven are genuinely retired, against
the tree and not by assertion:** the re-run receipt's site, the `newgame` mutant's
witness, what INVARIANT 7's test compares, who lands the pilot config, and the
config-path precondition. The sixth (**N2**) survived the fix that was supposed to
retire it, and the seventh — the TAB test's driver and the empty field — is
closed. What is new is N1, which revision 3 did not have.

## The strongest attack that did not land

**I attacked §14's replacement on the hypothesis that it is B1 wearing its fix —
that the design, having been caught naming a site that cannot execute, has
retreated to unit tests that cannot fail either, and has quietly sent its hardest
obligation to a document that does not exist.** The dispatch pre-declared that
reading as the thing to check hardest. **It came back sound in every limb but one,
and the surviving limb is N1, which is narrower than the attack.** Here is why the
harder halves closed:

1. **Is §14.1's unit test a retreat?** No — it is strictly stronger than what it
   replaces for the mutants it owns. The solver spelling was observable **nowhere**
   in revision 3 (`report.rs:62`'s `if info.solver_nodes > 0`,
   `stub_engine.rs:123`'s `solver_nodes: 0`, `gate_v0.toml:94`'s `on_search_path =
   false` — all verified), so a normalisation widened to strip `solver_root_nodes`
   died nowhere. Against a synthetic line it dies immediately. **§4.1's central
   argument — that the expression matches in both spellings — becomes executable
   for the first time in four revisions.**
2. **Is sending the real-binary receipt to the pilot an evasion?** **No, and this
   is the finding I most expected and could not make.** The governing dispatch
   already puts it there: `a9a4a3a:docs/experiments/wp20_dispatches.md`, the third
   dispatch's §4, registers *"the determinism re-run receipt on a sub-range"* as a
   pilot criterion. §14.2 returns an obligation to its own home rather than
   discarding it, and §14.4 prices the alternative honestly.
3. **Does the refusal to touch `tools/` leave the headline invariant unguarded?**
   No — see B1(c). INVARIANT 6 and INVARIANT 8 each keep permanent, in-CI guards,
   and the one coupling this package cannot pin is pinned by gate 9's own `sed`.
4. **Is the matrix's licence claim now false?** No. `matrix…md` §2 line 61's
   *"only `pistol-arena` is touched"* survives, which is why m13's first limb is
   MOOT and §15 correctly carries only the rename.

**Three more attacks failed and are recorded so nobody re-runs them:**

- **`demands_newgame_per_ask` as an out-of-licence instrument change.** I expected
  either a licence breach or a rule-9 problem. Neither: the stub is in
  `pistol-arena`, the enum is additive, the two existing witnesses key on a
  different string, and `docs/rule9_justifications.md:22` already justifies the
  file on grounds that argue **for** putting the behaviour there. And the mutant
  really does die, at the second ask, on any game of one or more turns.
- **M7's causal chain as a plausible story rather than a fact.** I traced all four
  links (`exchange.rs:169-188` → `:76-79` → `conclusion.rs:46-47` → the report's
  `nodes_a`) and every one holds, including the non-obvious one: a `None` from
  `totals_of` falls through to `:80`'s `INFO_PREFIX` arm and **continues**, so the
  run completes with zeroed counts rather than forfeiting. The design got this
  right, and the tree already carries a test that kills the mutant
  (`run_tests.rs:98`).
- **§13(b)'s reversal as a way of shedding an obligation.** I looked for something
  now owed by no package and found nothing: every limb of requirement 5, plus
  requirement 4's receipt, has a named owner, and §0.1's `--proposes` block was
  updated to say which package creates each of the four declared files.

## What I could not settle by reading, and the run that would

- **N1's exact fate.** That a call-site deletion survives every registered test is
  read off the design's own driver column and §14.3's own admission; I did not
  observe it. **The run:** in a worktree, write `capture.rs` with the normalisation
  applied, then delete the call while leaving the function; `cargo test -p
  pistol-arena` should stay green. Refused here per the dispatch.
- **N2's consequence.** Whether an implementer taking §4.3 literally would add a
  parse to `transcript::read` is a judgement about a reader, not a run. **The run
  that would settle the adjacent fact** — that `arena_version` is unreachable from
  `Transcript` — I did make: `git grep -n "arena_version" -- crates/` returns two
  writers and no reader.
- **m8's residual.** Whether a field-by-field round-trip kills the first-two-fields
  swap depends on the fixture using distinct values for the game index and `k`,
  which the design does not state. **The run:** write the fixture with `game 0, k
  0` and the mutant survives; with `game 3, k 7` it dies. A one-clause fix, not a
  finding.
- **The headroom after the edit** (carried from all three prior reviews, still
  open, and the design is correctly silent about the number). **The run:** write
  the arm, extract `USAGE` to `usage.rs`, run `tools/file_justification_check.sh`
  (`SOFT_CAP=300`; `bin/arena.rs` is not in
  `docs/rule9_justifications.md` today).

---

## A closing note for the architect, since this verdict stops the package

**The document is close, and the two findings are one clause each.** Everything
this fix round was granted for, it did: the BLOCKING is gone, its root
mischaracterisation is corrected everywhere, seven MAJORs and all fourteen MINORs
are applied, the freeze held, the lifts are byte-exact, and 113 citations resolve
with zero author debt across forty-five hand checks. **The method D-546 imposed is
demonstrably working** — the failures are no longer claims the code does not make,
because the checker and the freeze have closed that class. What remains is the
class D-544 named and nothing yet guards: **a fix that spends a true thing while
applying a finding.** N1 is exactly that, and I could not pass it, because a
surviving mutant on this package's headline normalisation is the defect the last
five reviews were about.
