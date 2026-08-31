# REVIEW-design — `docs/experiments/wp20m_design.md` revision 3

## Header

- **Named revision adjudicated:** `406ace98d7d75a1fb55c0eda04a140e348da3998`
  (`docs(wp20m): the design is lifted rather than rewritten — the true label
  sentence returns, the seat check moves one type left, and the deleted coldness
  section comes back`).
- **Matches HEAD:** **yes.** `git rev-parse HEAD` →
  `406ace98d7d75a1fb55c0eda04a140e348da3998`, branch `dev`.
- **Tree state:** `git status --porcelain` → **empty**. Clean.
- **Prior revisions compared against:** `5064b05` and `7af62e7`, read via
  `git show <sha>:docs/experiments/wp20m_design.md` and word-diffed against the
  subject (see Part 1).
- **What I ran:** `git`, `git grep`, `/usr/bin/grep`, `sed`, `awk`, `diff`,
  `wc`, `cat`, `ls`, `sort`, `tr`, and
  `python3 tools/design_citation_check.py` with the four `--proposes` paths the
  dispatch names. Every recorded grep is `/usr/bin/grep` or `git grep` (D-265).
- **What I refused to run:** `cargo` in any form, `tools/ci.sh`,
  `tools/determinism.sh`, `tools/arena_smoke.sh` — a CI run is in flight in this
  tree. Where a claim needs a run I name the run rather than assert the answer.
- **Read as binding:** CLAUDE.md (whole), `docs/process.md`, D-539 … D-547 in
  full plus D-80/200/252/291/413/423/424/483/500/518/527/537/542/543/544,
  `docs/experiments/wp20m_design_REVIEW.md` (full),
  `docs/experiments/wp20m_design_REVIEW_rev2.md` (full),
  `docs/experiments/wp20m_DESIGN_STOP.md`,
  `docs/experiments/wp20_DESIGN_STOP_SPLIT.md`,
  `docs/experiments/wp20_dispatches.md`,
  `docs/experiments/matrix_wp20_shape_selection.md` row (g),
  `docs/book_v2_ledger.md`.
- **Code read at this revision:** `pistol-arena` — `identity.rs`,
  `handshake.rs`, `transcript.rs`, `validate.rs`, `config.rs`, `report.rs`,
  `replay.rs`, `exchange.rs`, `channel.rs`, `seats.rs`, `lib.rs`,
  `bin/arena.rs`, `bin/stub_engine.rs`, `Cargo.toml`, `tests/common/mod.rs`,
  `tests/sprt_power_tests.rs`, `tests/replay_tests.rs`,
  `tests/seat_setup_identity_tests.rs`; `pistol-cli` — `report.rs`,
  `protocol.rs`, `corpus/emit.rs`, `corpus/mod.rs`, `lib.rs`, `bin/pistol.rs`,
  `tests/workspace_shape_tests.rs`; `pistol-engine` — `position.rs`,
  `position_token.rs`, `instance.rs`; `pistol-core` — `turn.rs`;
  `pistol-search` — `search.rs`, `position.rs`, `pvs.rs`, `tt/mod.rs`;
  `pistol-solver` — `solver.rs`; `tools/ci.sh`, `tools/determinism.sh`,
  `tools/arena_smoke.sh`, `tools/config_check.sh`,
  `tools/file_justification_check.sh`; `configs/arena_smoke_v0.toml`,
  `configs/gate_v0.toml`, root `Cargo.toml`.

---

## VERDICT: **FAIL**

**1 BLOCKING, 8 MAJOR, 14 MINOR.**

This is a much better document than either predecessor, and the method D-546
imposed did the job it was given. **The freeze audit is nearly clean, the lift
audit is clean, all 88 citations resolve, and the expensive half — does the cited
line SAY what the document says it says — came back true on every one of the
thirty-odd I checked by hand, with two off-by-one exceptions and one
mischaracterisation.** Zero findings are AUTHOR DEBT: the checker could have
caught none of them. That is the D-543 remedy working exactly as advertised, and
it is worth saying plainly before the findings.

It fails on one thing, and it is the answer to the previous review's MAJOR C:

> **§14 sends the real-binary re-run receipt to `tools/arena_smoke.sh`, and
> `arena --capture` cannot read the report that script writes.**
> `configs/arena_smoke_v0.toml:57` is `kind = "depth_turns"`, and
> `crates/pistol-arena/src/transcript.rs:164-170` refuses **every** source report
> whose budget kind is not `nodes` — not only a movetime one, which is how the
> design describes it at `:481`. The one named site for the package's headline
> receipt cannot execute, and no other is given.

Everything else is MAJOR or below. Several of the MAJORs are the arc's own
recurring vacuity class in new clothes (M3, M4), and one is a mechanism whose
protection does not exist (M2).

---

# PART 1 — THE FREEZE AUDIT (check 1) AND THE LIFT AUDIT (check 2)

## 1.1 The freeze audit — what §0.2 claims against what the diff shows

Method: every `> `-prefixed block in revision 3 was stripped of its quote
prefix, reduced to a whitespace-normalised word stream, and `diff`ed against the
same reduction of the named source span in `5064b05` / `7af62e7`. Then every
revision-2 line over 25 characters was swept for a distinctive four-word
fragment against a flattened revision 3, and each miss was adjudicated by hand.

| §0.2 row | claim | verified? | verdict |
|---|---|---|---|
| rev 1 §4 **COLDNESS** → §12 | RESTORED, quoted, citations added | rev3 `689-705` vs rev1 `80-96`: **word-for-word IDENTICAL** | **TRUE.** The citation paragraph at `:707-720` is new material beside the quote, not inside it, exactly as the ground says |
| rev 2 §1 **WHERE THE CODE LIVES** | EDITED: count → the run; flag grammar completed | rev3 `138-143` vs rev2 `63-69`: only `crosses that.` → `crosses [the cap].`, bracketed | **TRUE.** `283`/`300` are gone from the numeral sweep (check 11). Flag grammar `--go <go line>` → `--label-nodes <n>` at `:124` is the MINOR I edit and is disclosed |
| rev 2 §2 **WHICH POSITIONS ARE ASKED** | EDITED: range enumerated, legality + decidedness restored | rev3 `170-190` vs rev2 `73-94`: the three claimed edits are present **AND one sentence is silently dropped** | **PARTLY FALSE** — see **m1** |
| rev 2 §3 second ¶ | **LIFTED VERBATIM** | rev3 `272-275` vs rev2 `110-113` | **TRUE — byte-for-byte** |
| rev 2 §4 **THE ONE NORMALISATION** | **LIFTED VERBATIM** | rev3 `300-310` vs rev2 `121-131` | **TRUE — byte-for-byte** |
| rev 2 §4 **THE SOURCE IS NAMED** | **LIFTED VERBATIM** | rev3 `382-386` vs rev2 `139-143` | **TRUE — byte-for-byte** |
| rev 2 §6 **the failure table** | EDITED: **two rows added** | two rows added ✓, **and row 1 was reworded** | **INCOMPLETE** — see **m2** |
| rev 2 §7 **the budget's kind** | EDITED: builder named, grammar fixed | rev3 `472-477` vs rev2 `178-183`: quote identical; edits are new prose below it | **TRUE** |
| rev 2 §8 **the removed mutant** | KEPT removed, replaced | quote at rev3 `516-523` drops rev2's `Pass 2 does not call it: …`, which the rev-2 review graded **NOT APPLIED** (MAJOR 7) and so is not frozen; and §8's own text discloses the reversal at `:526-537` | **TRUE** |
| rev 2 header **capture decisions table** | LIFTED, extended by one row | quote at rev3 `64-68` drops the word **four**; the added row is *"WHAT is written, and in what grammar"*, not the slot | **GROUND MISDESCRIBES THE EDIT** — see **m3** |

**One frozen section is unlisted in the table and is a lift, not an edit:**
revision 2's §4 *"THE FILE'S SHAPE"* (rev-2 review MAJOR 4, *applied half*) is
carried at rev3 `355-359` and is **byte-for-byte identical** to rev2 `133-137`.
Because it is a verbatim lift it is not an edit, so D-547 does not require a row;
but three other verbatim lifts *are* rowed, so the table is inconsistent rather
than wrong. Recorded, not graded.

**Converse check — does §0.2 claim an edit it did not make?** No. Every claimed
edit is present in the diff.

## 1.2 The lift audit — L1 … L7, and new losses

| loss (rev-2 review Part 1) | back? | back correctly? |
|---|---|---|
| **L1** the true `validate_engines` sentence | **YES**, quoted at `:104-107` from rev1 `25-28` | **YES, and better.** `:109-115` grounds it at `validate.rs:243-250` and `config.rs:170-193`, both verified exact, and anchors it on `configs/arena_smoke_v0.toml`, which I confirmed is a committed self-match: one binary, one engine config (`:82`, `:88`), two labels (`:70`, `:85`) |
| **L2** §4 COLDNESS entire, with D-540's pilot obligation | **YES**, as §12 `:683-720` | **YES.** Word-identical to rev1's §4, and D-540's fresh-process criterion is back verbatim at `:701-705`, defect class named. Every one of the eight added citation sites verified exact (see check 12) |
| **L3** the legality precondition | **YES**, `:210-213` | **YES**, with `transcript.rs:359-379` added, which I confirmed is the `make_turn` replay refusing turns after a win |
| **L4** "WHY VERBATIM" | **YES**, `:131-133` | **YES.** Partial quote starting at *"A consumer that disagrees…"*; the preceding sentence is legitimately outside the quote |
| **L5** rev-1 INVARIANT 3 (bytes) | **YES**, as INVARIANT 6 `:574-576` | **YES**, and it now covers the `bestmove` line, which was MAJOR B's second limb, with `a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote` registered |
| **L6** the unreadable-report test | **YES**, `a_report_pass_two_cannot_read_is_refused_by_name` | **YES**, mapped to INVARIANT 9 |
| **L7** the label-budget digest input's test + mutant | **YES**, `a_capture_identity_moves_when_the_label_budget_moves` + its mutant | **YES** |

**New losses — things revision 1 or 2 had that revision 3 dropped:** the sweep
found none that were verified or passed. The revision-2 text absent from
revision 3 is, without exception: header prose about revision 1 (replaced under
BLOCKING 3, not frozen), §3's defective mechanism (correctly deleted), §5 (rev-2
MAJOR 5 NOT APPLIED, not frozen), the flat test list (rev-2 MAJOR D, not frozen),
rev2 §11's requirement-5 claim (rev-2 MAJOR 9, applied-but-defective, not
frozen), the `283`/`300` count (MINOR J, deliberately), `--go <go line>` (MINOR
I, deliberately), and *"It is driven by the real `pistol` binary"* (rev-2 MAJOR
C, replaced by §14). **The lift audit is clean.**

---

# PART 2 — THE FOURTEEN CHECKS

### 1. Freeze audit — **PASS with two findings (m1, m2) and one misdescription (m3).**
See Part 1.1. No substantive change to a passed section escaped the list except
the two named; both are correctness-neutral, and one is required by the BLOCKING
A remedy.

### 2. Lift audit — **PASS.** See Part 1.2. L1–L7 all restored, all correctly, no new losses.

### 3. §3, the BLOCKING A remedy — **(a) TRUE, (b) HOLDS ONLY UNDER AN UNSTATED PRECONDITION, (c) TRUE, (d) TRUE.**

**(a) `EngineIdentity` carries no label and derives the equality used.**
`crates/pistol-arena/src/identity.rs:13-22` — four fields, `id_lines`,
`binary_sha256`, `config_sha256`, `weights_sha256`, and **no label**. `:12` is
`#[derive(Debug, Clone, PartialEq, Eq)]`. `Transcript::identities` at
`crates/pistol-arena/src/transcript.rs:34`, doc at `:30` says *"Per slot, `0` is
engine A"*, which is exactly the design's slot claim. **The comparison
`identities[0] == identities[1]` exists and needs no new predicate.** The one
defect is a one-line citation offset (**m5**).

**(b) Do two seats of one self-match attest equal identities?** Traced
`identity::capture` (`identity.rs:56-87`) end to end. `binary_sha256` is
`digest_of(&engine.binary)` — content, equal. `config_sha256` is
`digest_of(&engine.config)` — content, equal. `weights_sha256` is off the
handshake — equal. `id_lines` is `spoken.lines` from `handshake::shake`, and the
engine builds them at `crates/pistol-cli/src/protocol.rs:136-149` (name, version,
protocol, mode, budgets) plus `crates/pistol-cli/src/bin/pistol.rs:142-175`,
whose **first line is `format!("config {}", path.display())` — the config path
AS SPELLED on the command line, not its content.** Everything else on that list
(`eval`, `tt_bytes`, `candidate_policy`, `movetime_epsilon_ms`,
`weights_sha256`) is a value read out of the config file and is therefore equal
whenever `config_sha256` is.

So: **two seats naming the same config FILE by two different path STRINGS attest
different identities and are refused.** This is **MAJOR M8**, not BLOCKING. It is
not the same defect class wearing its fix: it fails **safe** (a false refusal,
loud, by name — the direction hard rule 3 wants), it does not refuse 100% of
inputs, and it does not hold for `configs/arena_smoke_v0.toml`, which spells
`configs/gate_v0.toml` identically in both sections. What earns MAJOR is that
this package **authors the config where the precondition must hold**
(`configs/arena_wp20_label_pilot.toml`), states the equality flatly at `:255-257`
without naming the condition, and registers a test
(`a_self_play_report_whose_seats_carry_distinct_labels_is_accepted`) that
**cannot see it**: the stub engine identifies with `vec![weights_line]` only
(`crates/pistol-arena/src/bin/stub_engine.rs:266`) and emits no `config` id line
at all, so every in-crate fixture is blind to the one field that varies.

**(c) `configs/arena_smoke_v0.toml` is the self-play shape the design says.**
Confirmed: `:8-9` — *"It is a SELF-MATCH: both sides are the same binary with the
same engine config"*; `:71`/`:86` one binary, `:82`/`:88` one config, `:70`/`:85`
two labels. **TRUE.**

**(d) "spawns slot zero" is unambiguous.** `:258-266` says which slot, why the
choice is a choice of spelling once identities are attested equal, and says it is
written down anyway. `:284-289` explains why `verify_engines` still checks both.
**TRUE** — though the "spawns slot zero" limb of INVARIANT 3 is pinned by no test
(**m10**).

### 4. §4.2, the TAB grammar — **the claim is true for `pistol`, over-stated as written, and the refusal is sufficient; the loader does distinguish empty from missing.**

Field by field:

| # | field | source | TAB reachable? |
|---|---|---|---|
| 1 | game index | pass 2, from `RecordedGame::index` (`usize`) | no |
| 2 | prefix `k` | pass 2, a `usize` | no |
| 3 | `position` line as sent | `exchange::position_line` (`exchange.rs:154-161`), literal + `Turn::to_string()` joined by single spaces | no |
| 4 | normalised `info totals` line | **the engine's own bytes off the channel** | not from `pistol` — `render_info` (`crates/pistol-cli/src/report.rs:82-97`) formats `u32`/`u64`, `score_token`, and `pv` turn tokens, joined with `' '`. Reachable from a non-`pistol` engine |
| 5 | `bestmove` line | **the engine's own bytes** — from `pistol`, `bestmove_line` (`report.rs:106-108`) over `Turn` | same |

The design's sentence at `:341-350` — *"every field is built by a `pistol-cli`
formatter"* — is true of fields 3–5 **only because the engine is `pistol`**, and
nothing in §3's refusal requires that: it requires the two seats to attest **one**
engine, not `pistol`. The handshake gate (`protocol`, `mode instrument`, one
`weights_sha256`) is satisfiable by a third-party speaker. `Channel` does not
sanitise: `from_utf8_lossy` then `trim_end_matches(['\n','\r'])` leaves an
interior TAB intact.

**The design's own refusal is therefore load-bearing rather than belt-and-braces,
and it is sufficient**: `:346-350` refuses a field carrying a TAB and a record
whose TAB count is not four, by name (hard rule 3). Graded **MINOR (m6)** only
for the invariant/test placement, not for the mechanism.

**Empty vs missing:** with a fixed four-TAB arity, `a\t\tb\tc\td` (field 2 empty)
carries four TABs and `a\tb\tc\td` minus a field carries three, so the arity check
distinguishes them mechanically. No field of this record can legitimately be
empty (fields 1–2 are decimals, 3 is at minimum `position start`, 4–5 are
non-empty engine lines), so an empty field is always an error — but the design
does not say the loader refuses one. One sentence, folded into **m6**.

### 5. §5, the digest — **(a) NOT SAID and NOT REACHABLE as cited (M1), (b) SAFE ON THE FACT THAT MATTERS but the design's ground is not exactly true, (c) one collision route is open and the design names it (M2).**

**(a) Is `arena_version` reachable from pass 2?** `report::instrument` writes it
at `crates/pistol-arena/src/report.rs:130` from `env!("CARGO_PKG_VERSION")`, in
the verdict block. **`Transcript` does not carry it** — the struct is
`transcript.rs:28-52` and has no such field, and `transcript::read`
(`:135-209`) never parses an `arena_version` line. So pass 2 has exactly two
ways to obtain it and they are different values: `env!("CARGO_PKG_VERSION")`
compiled into the pass-2 binary (trivially available, and the one §5's stated
purpose requires), or a **new** parse of the source report's line (the version
that PLAYED, which is invariant under a change to pass 2 and so defeats the
purpose). **The design says neither**, and its evidence sentence points at the
report field while its purpose sentence points at pass 2's own. **M1.**

**(b) Is the engine identity genuinely already inside `experiment_sha256`?**
Not exactly. `experiment_digest` (`report.rs:41-76`) closes over, per engine,
`engine.label` and the identity's `binary_sha256`, `config_sha256`,
`weights_sha256` (`:61-74`). It does **not** close over `id_lines`. As
established in check 3(b), the only content of `id_lines` not implied by those
three digests is the config **path spelling** — which changes no answer, and
`replay::verify_engines` (`replay.rs:216-241`) pins the whole identity at run
time anyway. **So the decision to drop it is right; the design's ground —
*"already closes over … so the second spelling was the same fact twice"* — is
true of three fields of four and the document does not say so.** Recorded as
**m5's** companion, not graded separately: it changes no conclusion.

**(c) What can collide?** I enumerated everything pass 2's output depends on:
the asked set (a function of the report's games, and the rev-1 reviewer closed
the three routes by which two reports of one experiment could hold different
games — I re-read that argument and did not reopen it), the slot (identities
attested equal ⇒ no effect), the label `go` (in), `hang_timeout_ms` (a firing
watchdog refuses the run, so no successful capture varies with it), the engine
(pinned by `verify_engines` and inside `experiment_sha256`), and **pass 2's own
code**. That last one is the hole, the design names it, and then does not close
it — see **M2**.

### 6. §13, requirement 5 — **(a) SATISFIES hard rule 8; (b) MISQUOTES NOTHING but MISAPPLIES the rule and is internally contradictory (M5); (c) the dispatch's registered mutant is left homeless without being named (M6).**

**(a)** `docs/label_corpus_manifest.md` committed, one row per capture with
digests, the program printing the row and never writing the file
(`crates/pistol-arena/src/lib.rs:41-45` — *"Nothing here writes inside the
repository"* — verified exact). That is precisely hard rule 8's *"a committed
manifest may sha-index them"*, and the printing rather than retyping is D-543's
remedy correctly generalised. **Satisfied.**

**(b)** The quotation is exact — `docs/book_v2_ledger.md:16` is indeed
*"**The rule.** A new pre-registration takes the next unconsumed range, adds its
row here in the same commit that adds its arena config…"*. The **application**
is wrong twice, and the ledger's own closing text is the witness:
*"Neither has a row above, because neither has a **committed pre-registration**
drawing from this book."* **M5.**

**(c)** The dispatch's Development-round item 2 registers the mutant *"ledger
overwrite -> append test dies"*
(`docs/experiments/wp20_dispatches.md`, Development round). §13's design — the
program prints, a human commits — **removes the append behaviour and therefore
the mutant's site**. That may well be the right call; the design does not say it
is making it. Compare `:494-500`, where the design names a departure from a
review's own FIX and argues it. Here the same kind of departure from *governing
dispatch text* is silent. **M6 — and it is an evasion by omission rather than by
argument, which is what makes it a finding.**

### 7. §14, the re-run receipt site — **(a) TRUE, (b) TRUE, (c) FALSE IN THE WAY THAT MATTERS, (d) TRUE BUT OVER-CLAIMED.**

**(a)** `crates/pistol-arena/tests/common/mod.rs:9,11` offers exactly `ARENA` and
`STUB`. `git grep -n "CARGO_BIN_EXE" -- crates/` puts `CARGO_BIN_EXE_pistol`
**only** under `crates/pistol-cli/tests/` — zero hits in `crates/pistol-arena/`.
`crates/pistol-arena/Cargo.toml` declares exactly the two `[[bin]]` targets.
**TRUE.**

**(b)** `crates/pistol-arena/Cargo.toml` depends on `pistol-cli`, so the reverse
edge is a cycle. **TRUE.**

**(c)** *"there is exactly one such place: `tools/arena_smoke.sh`"* — the
existence claim is true (it is the only script that builds and runs both). **But
the capture pass cannot run there**, because the report it writes carries a
`depth_turns` budget and `transcript::read` refuses it. **BLOCKING B1.**
Separately, the absolute *"Neither test crate can run both programs"* at `:779`
is overstated: this crate's own `tests/sprt_power_tests.rs:19-37` establishes the
`current_exe()`-relative route to a target in the same directory that is not a
`[[bin]]`, and the real `pistol` is built into that directory by
`cargo test --workspace`. That route is bad (it breaks under
`cargo test -p pistol-arena`, which rule 3 forbids answering with a skip), so the
design's **conclusion** survives; its **claim** does not. **m4.**

**(d) Does `a_captured_totals_line_keeps_every_field_but_nps_and_time` die under
both normalisation mutants against the stub?** For the fields the stub emits,
yes: `crates/pistol-arena/src/bin/stub_engine.rs:118-132` builds a real
`SearchInfo` (`nps: 1` at `:126`, `time_ms: 0` at `:127`, `hashfull_permille: 0`,
`score: 0`, one-turn `pv`) and hands it to `pistol_cli::Session`
(`:266`), so the line goes through `render_info` and carries
`depth_turns seldepth nodes nps time hashfull score pv`. Removing the
normalisation leaves `nps 1 time 0` in the record — the test fails. Widening it
to strip `hashfull` or `seldepth` — the test fails. **So the design's claim holds
for the non-solver spelling.**

It does **not** hold for the solver spelling, and that is **M4**: the stub sets
`solver_nodes: 0` (`:123`), so `render_info`'s `solver_field` block
(`crates/pistol-cli/src/report.rs:62-81`) is empty; and every committed engine
config has the solver off — `configs/gate_v0.toml:94` is
`on_search_path = false` with the comment *"gate OFF in every committed config"*
— so §14's own arena-smoke receipt could not see it either. A normalisation
widened to strip `solver_root_nodes` dies nowhere in this package. *"The vacuity
is confined to one test"* (`:789`) is therefore false.

**Is there a route to the real `pistol` from a `pistol-arena` test the design
missed?** Yes — the `current_exe()` route above. It is worse than what the design
chose. Recorded as **m4** so nobody re-derives it.

### 8. §7's deliberate departure — **RIGHT, and §7's rule is pinned; two residuals.**

`crates/pistol-arena/src/bin/arena.rs:82-100` is a slice-pattern `match` with a
`_ =>` arm returning an error (`:94-99`). **Any unrecognised flag is refused,
exit 2.** So with `--label-nodes <n>` as the only spelling, there is no movetime
label budget to refuse and `a_movetime_label_budget_is_refused_by_name` would
indeed have no site. **The departure is correct and correctly named.**

§7's rule is not left unpinned: `the_label_go_line_is_the_one_budget_section_spells`
plus the mutant *"the `go` line formatted by hand instead of through
`BudgetSection::go_line`"* pin that the label `go` is `go nodes <n>` and nothing
else. `BudgetSection::go_line` (`config.rs:120-134`) is real and returns `None`
only for `MovetimeMs`. **Consistent with how the binary parses.**

Two residuals: the substitute test
`a_capture_over_a_movetime_report_is_refused_by_name` is named for a refusal
that is in fact *any non-`nodes` budget* (this is the BLOCKING's root, **B1**),
and the `_` arm's refusal message at `:96-97` enumerates the two existing modes
and would need `--capture` added — one sentence the design does not write
(**m14**).

### 9. INVARIANT 10's exemption — **HONEST, but its second leg is unsound (M7).**

The exemption is not an evasion of rev-2 MAJOR D. MAJOR D's finding was that the
invariant had *no test and no acknowledgement*; revision 3 acknowledges it at
`:590-599`, argues it from `docs/process.md`'s own criterion rule, and names
three pieces of evidence. Two of the three are sound: **the diff** (the only SPRT-path
file this package touches is `exchange.rs`, and only one visibility keyword —
verified: `totals_of` is `fn totals_of` at `exchange.rs:169`, private today, with
its single consumer at `:76`), and **CI gate 15** (`tools/ci.sh:157-158`, which
does compare two runs' verdict blocks byte for byte, `arena_smoke.sh:22-27`).

The third leg does not hold: **INVARIANT 7's test is itself unspecified, and
read literally it is the very comparison INVARIANT 10 says is impossible.** See
**M7**.

### 10. Closure of the sets — done exhaustively.

12 invariants, 29 tests, 21 mutants. **This is a real and large improvement over
revision 2** and I want it recorded: every test maps to an invariant or to a named
section rule, no two invariants are in tension, and rev-2 MAJOR D's two
untested invariants (5 and 9-as-was) are both now pinned.

**(a) Invariants with no test — 1 whole and 2 limbs:**

| invariant | status |
|---|---|
| **10** (pass 1 unmodified) | none, **declared** at `:590-599` — legitimate, see check 9 |
| **3**, limb *"spawns slot zero"* | none, **not declared**. **m10** |
| **12**, input *"the capture format version"* | none. The other three inputs each have a test and a mutant; this one has neither. **m7** |

**(b) Tests pinning no invariant and no named section rule — 0.** All three
non-invariant tests point at a section rule the document owns (§7 ×2, §13 ×1).
Clean.

**(c) Rules with neither test nor mutant — 4:**

| rule | where |
|---|---|
| *"the totals line carries no score at all → capture it as written"* | §6 table row 7, `:447` |
| *"a malformed totals line … the run is refused by name"* | §8 `:538-544` — and §6, which owns failure modes, carries no such row. **m9** |
| §13(b)'s ledger-row obligation | `:753-762`. **M5**, **M6** |
| the dispatch's throughput-shape obligation | nowhere. **m12** |

**(d) Mutants that cannot die, or may not:**

- *"the `newgame` removed from pass 2's loop"* → **cannot die with any instrument
  in the tree.** **M3**, and it is the most serious of this group.
- *"the normalisation widened to strip another field"* → **survives if the field
  is a solver field.** **M4.**
- *"a capture record's fields reordered on write"* → **may not die.** Fields 1 and
  2 are both decimals; swapping them round-trips through any loader that reads by
  position, and the registered killer is
  `a_capture_file_round_trips_through_its_own_loader`, which a swap satisfies.
  **m8.**
- *"a fourth load-bearing lookup added to `totals_of`"* → **can die**, contra the
  shape of rev-1's dead mutant. `value("solver_nodes")?` returns `None` for every
  committed (solver-off) config, so `compute.add` (`exchange.rs:76-79`) never
  fires and the report's node counts zero. It mutates a body this package does
  not change, which the design states and grounds in D-542. Accepted.
- Every other mutant traces to a test that can observe it.

**(e) Tests that would pass vacuously:**

- `a_rerun_over_one_report_is_byte_identical` — **declared** a shape test against
  the stub at `:784-786`. Honest. Its non-vacuous discharge is §14's, which is
  **B1**.
- `a_captured_totals_line_keeps_every_field_but_nps_and_time` — **partly vacuous**
  (**M4**).
- `every_label_go_is_preceded_by_a_newgame` — **fully vacuous as the tree stands**
  (**M3**).
- `a_captured_field_containing_a_tab_refuses_the_run_by_name` — no engine in this
  tree can emit a TAB, and the design does not say how the test is driven
  (**m11**).
- `a_capture_over_a_movetime_report_is_refused_by_name` — pins inherited
  behaviour, cannot fail from this package's diff. The design says so
  (*"the refusal pass 2 inherits"*), so it is honest; but it is mis-named
  (**B1**).

### 11. D-483 — **PASS, and the argument in §0.1 is right.**

```
$ /usr/bin/grep -oE "[0-9]+" docs/experiments/wp20m_design.md | LC_ALL=C sort -u
0 05 1 10 100 102 103 104 105 106 108 11 111 112 114 118 12 120 124 13 130 131
134 135 14 143 15 153 154 16 161 164 169 17 170 173 174 18 188 189 19 193 194
195 2 20 200 203 208 209 216 22 226 229 239 24 241 243 250 252 253 256 27 289
29 3 300 32 33 34 359 36 379 39 4 41 423 43 44 45 47 483 5 500 5064 527 539 540
542 543 544 545 546 547 55 57 59 6 61 62 68 69 7 70 73 74 76 79 8 80 82 84 89 9
97 98 99
```

117 distinct numerals, and I classified every one. **Every single one is a
citation line number, a section number, an invariant number, a D-key, a CI gate
number, `256` of `sha256`, or the `5064`/`05` of a revision SHA.** `283` — rev-2
MINOR J's finding — is **gone**. `300` and `100` survive only inside citation
ranges (`arena.rs:82-100`, `transcript.rs:289-300`, `emit.rs:12-100`); I checked
each by grep. There is **no budget value, no node count, no threshold, no range,
no bracket, no line count.**

The §0.1 argument stands. A citation line is a *pointer* whose failure mode is
being stale, and a stale pointer is caught mechanically by
`design_citation_check.py` — which is precisely the property a *measured* number
does not have. **No number in this document is one a pre-registration, gate or
criterion would consume.**

### 12. The citation checker, and the half it cannot do.

```
$ python3 tools/design_citation_check.py --proposes crates/pistol-arena/src/capture.rs \
    --proposes crates/pistol-arena/src/usage.rs \
    --proposes configs/arena_wp20_label_pilot.toml \
    --proposes docs/label_corpus_manifest.md docs/experiments/wp20m_design.md
docs/experiments/wp20m_design.md: 88 citation(s) checked, 0 unreproduced
```
**Green, 88 citations, exit 0.** All four proposed files are correctly absent
from the tree (`docs/label_corpus_manifest.md` verified missing).

**Then the half it cannot do.** I read the cited lines for thirty-one citations
and compared them against the claim built on them:

| citation | claim | content supports it? |
|---|---|---|
| `validate.rs:243-250` | refuses equal labels, *"the two sides must be told apart in the report"* | **YES, exact** |
| `validate.rs:39-45` | *"the one refusal this crate exists to make loudly"* | **YES, exact** |
| `transcript.rs:189-194` | refuses a report whose two seats carry one label, with the quoted reason | **YES, exact** |
| `transcript.rs:289-300` | attributes each game's seats by label and refuses a mismatch | **YES, exact** |
| `transcript.rs:359-379` | `make_turn` replay refusing a turn after a win | **YES** |
| `transcript.rs:32-34` / `:29-34` | one identity per slot; slot 0 is engine A | **YES, exact** (doc at `:30`) |
| `transcript.rs:124-131` | *"the format is whitespace-delimited and does not quote"* | **YES** (comment at `:125-127`) |
| `transcript.rs:135-209` | `transcript::read`'s shape | **YES** (`pub fn read` at `:135`) |
| `transcript.rs:43-45` | `Transcript` carries `hang_timeout_ms` for a replay's own watchdog | **YES, exact** |
| **`transcript.rs:164-170`** | **"a movetime source report is refused by name"** | **NO — the code refuses EVERY non-`nodes` kind, `depth_turns` included. B1** |
| `config.rs:170-193` | `label` is a field of `EngineSection` | **YES, exact** |
| `config.rs:114-134` / `:120-134` | `go_line` returns `None` for `MovetimeMs` | **YES, exact** |
| **`identity.rs:11`** | **"It derives `PartialEq` at :11"** | **NO — the derive is at `:12`; `:11` is the doc line. m5** |
| `identity.rs:11-22` | four fields, no label | **YES** |
| `replay.rs:216-241` | `verify_engines` re-captures and compares, with the D-252 quote | **YES, exact** (`:211-212` carries the quoted words) |
| `replay.rs:16-19` | *"a criterion over SOME of a report's games…"* | **YES, exact** |
| `report.rs (arena):130` | `arena_version` from `env!("CARGO_PKG_VERSION")` | **YES, exact** |
| `report.rs (arena):41-76` / `:61-74` | `experiment_digest`; per-engine digests | **YES** (also closes over `label`, unmentioned) |
| `exchange.rs:154-161` | `position_line` emits the refused empty form | **YES, exact** |
| `exchange.rs:169-188` | `totals_of`, three `?` lookups | **YES, exact** |
| `exchange.rs:76-79` | the `compute.add` the `?`-chain would suppress | **YES, exact** |
| `report.rs (cli):15-18` | *"a measurement of the machine, not of the search"* | **YES, exact** |
| `report.rs (cli):20-29` | the totals marker's reason, D-80 | **YES, exact** |
| `report.rs (cli):62-84` | the solver block interpolated between `nodes` and `nps` | **YES, exact** |
| `report.rs (cli):82-97` / `:106-108` | the single-space joiners | **YES, exact** |
| `determinism.sh:153-154` | *"`nps` and `time` are the only fields two runs may disagree about"* + the `sed` | **YES, exact** |
| `arena_smoke.sh:22-27` / `:29-33` | the verdict-block comparison; the pre-registered cost | **YES, exact** |
| `emit.rs:12-100`, `:36-44`, `:102-118` | the `Fixture` type; `param`/`derived`; `body_of` + `claimed_body_digest` | **YES, exact** |
| `instance.rs:73-76`, `search.rs:229-239`, `tt/mod.rs:105-112` and `:114-118`, `solver.rs:195-203`, `position.rs:55-70`, `search.rs:57-79`, `:200-208`, `:253`, `pvs.rs:104-111` | the whole coldness chain, eight sites | **YES — every one exact.** `tt/mod.rs:109` really is `buckets.fill([EMPTY; BUCKET_ENTRIES])` and `:117` really is the epoch bump; `search.rs:205` really says *"the only callers are this crate's own tests and the `trigger_census` example"*; `Run::pv` really is at `pvs.rs:110` |
| `seats.rs:22-59` and `:47` | `with_seats` spawns per seat and sends `NEW_GAME` | **YES — `:47` is exactly the `NEW_GAME` send** (fn closes at `:60`, trivial) |
| `book_v2_ledger.md:16` | the rule, quoted | **YES, exact — and the ledger's own table contradicts the design's application of it. M5** |
| `lib.rs (arena):41-45` / `:47-69` | writes nothing in the repo; a `pub mod` list | **YES** (`:69` is `mod validate;`, private — trivial) |
| `bin/arena.rs:82-100`, `:103`, `:124-143`, `:173-174`, `:226-229`, `:10-13` | the mode match; `outpath::claim` before it; `workers_of`; `verify_engines` after `read`; the `unreachable!`; the `use` | **YES — all five exact** |
| `stub_engine.rs:120-131` | hardcoded `nps`/`time` | **YES** (`:126-127`) |
| `tests/common/mod.rs:8-11`, `determinism_tests.rs:99` | the two binaries; `CARGO_BIN_EXE_pistol` | **YES, exact** |

**AUTHOR DEBT: none.** Every finding in this review is in the half the checker
explicitly disclaims. That is the strongest positive result of this round and it
should be recorded as such: the instrument foreclosed its own half completely.

### 13. Requirement coverage.

| req | disposition |
|---|---|
| **1** self-play games; GAME and LABEL budgets separate | **DELIVERED** as mechanism (§1, §7). Value correctly deferred. ⚠ The GAME budget's value lives in the config §13(b) says this package lands — **M5** |
| **2** one record per position; documented versioned schema + loader test | **DELIVERED for the capture record** (§4.2 grammar, §4.3 `capture::read`, INVARIANT 11, four loader tests). Meaning columns correctly WP-2.0-S's; game outcome recoverable via §4.4. This closes rev-1 MAJOR 4/10 and rev-2 MAJOR E cleanly |
| **3** census | correctly WP-2.0b's (D-539). §13(c) signposts it |
| **4** deterministic end to end; a re-run receipt | **INVARIANT 8 delivered; the receipt is BLOCKED (B1)**; the pilot's own is correctly deferred at `:806-811` |
| **5** ledgers | **(a) delivered; (b) M5; (c) deferred correctly** |

**"Design decides and records":** storage format + schema version ✓; label policy
(all positions) ✓; transposition dedup → WP-2.0-S ✓ (D-544); census-minimum rule →
WP-2.0-S/operator ✓ (D-544); **throughput expectation stated as a shape —
FALLS BETWEEN.** §12 states the `newgame` memset cost and gives it to the pilot
(D-500), which is not throughput; §11's deferral list does not name throughput;
§14 does not either. **m12** — the smallest instance of the exact defect that
split this package.

### 14. Row (g) / branch B premise — **PRESERVED.**

- A third arm in `bin/arena.rs`'s mode match, two passes, over a written report:
  `:100-161`. ✓ D-542's mechanism.
- Branch B: `totals_of` to `pub(crate)`, fields deferred to WP-2.0-S, §8. ✓
- **No engine diff.** Every file the design proposes to touch is in
  `pistol-arena`, `configs/`, `docs/`, `tools/` and one `pistol-cli` *test*.
  Nothing in `pistol-core`/`-eval`/`-search`/`-solver`/`-engine`, and nothing new
  on the wire. ✓ D-539/D-540's licence intact.
- **No new crate, no manifest change.** `capture.rs` and `usage.rs` are `pub mod`
  additions to `crates/pistol-arena/src/lib.rs`; no `[[bin]]`, no dependency.
  `crates/pistol-cli/tests/workspace_shape_tests.rs` pins *dependency names per
  manifest* (`:78`, `:119`, `:142`, `:182`) and a member list, none of which
  moves. ✓ **All four `workspace_shape_tests` claims verified untouched.**
- **One departure:** §14 leaves `pistol-arena` for `tools/arena_smoke.sh` and
  `crates/pistol-cli/tests/arena_smoke_gate_tests.rs`, which falsifies
  `matrix_wp20_shape_selection.md:61`'s *"only `pistol-arena` is touched"*. The
  design names the departure (`:806`); hard rule 10 wants the ADR amendment.
  **m13.**

---

# PART 3 — FINDINGS

## BLOCKING

### B1 — §14's receipt site cannot execute: the smoke report's budget is `depth_turns`, and pass 2 refuses every non-`nodes` report

**What the design says** (`:791-796`):

> **THE RECEIPT ITSELF IS TAKEN WHERE THE REAL `pistol` MEETS THE REAL `arena`,
> AND THERE IS EXACTLY ONE SUCH PLACE**: `tools/arena_smoke.sh`, CI gate 15 …
> It is extended by one step: **a capture pass over the report it already
> writes**, run twice, with the two capture files compared byte for byte

and (`:480-482`):

> the crate refuses a movetime budget in three further places:
> `crates/pistol-arena/src/transcript.rs:164-170` (a movetime **source report**
> is refused by name)

**Evidence.**

```
crates/pistol-arena/src/transcript.rs:164-170
    if kind != "nodes" {
        return Err(refuse(format!(
            "the run used a `{kind}` budget and only a `nodes` budget replays: the whole premise \
             is that a re-driven engine answers what it answered, which wall-clock does not \
             promise (CLAUDE.md rule 4)"
        )));
    }
```

The predicate is `kind != "nodes"`, not `kind == "movetime_ms"`. And:

```
configs/arena_smoke_v0.toml:54-58
[budget]
# An instrument budget. A movetime budget is refused by name: …
kind = "depth_turns"
value = 1
```

`tools/arena_smoke.sh:49` sets `CONFIG="configs/arena_smoke_v0.toml"`, and the
only rewrite the script performs on it is `binary` / `binary_sha256`
(`:228-246`); the budget is untouched, and the script's own pre-registered cost
line says *"three arena runs at **depth_turns 1**"* (`:29`).

**Why it is wrong.** Pass 2 reads its source report through `transcript::read` —
the design says so three times (`:206-213` grounds INVARIANT 1 on it, `:441` makes
an unreadable report a refusal, `:498-500` registers a test for *"the refusal
pass 2 inherits from `transcript::read`"*). So `arena --capture <the smoke
report>` is refused at `transcript.rs:164` before a single position is asked.
**The design's single named site for the package's headline receipt — the remedy
for rev-2 MAJOR C, the discharge of dispatch requirement 4, and the only place
INVARIANT 6 and INVARIANT 8 are non-vacuously observable — cannot run.**

An implementer hits this on their first attempt and must then choose between
three things the design should have chosen between, each with consequences the
design has not priced: change `configs/arena_smoke_v0.toml`'s budget kind (a
committed config on gate 6's path, and gate 15 asserts `depth_a`/`depth_b` per
game at `arena_smoke.sh:316`, and the gate's pre-registered cost at `:29-33` is
stated in depth terms); add a second arena run at a `nodes` budget inside the
gate (a new committed config, a new cost, and the "one step" claim is false); or
relax `transcript::read` (a change to the SPRT/replay path, which INVARIANT 10
forbids).

The mischaracterisation at `:481` is the root: had the design described
`transcript.rs:164-170` as *"any non-`nodes` source report is refused"* — which
is what it says — §14 could not have been written. This is the arc's own defect
class, a claim about the code the code does not make, in the one section written
to answer the previous review's hardest finding.

**FIX.** Either extend `tools/arena_smoke.sh` with a second, `nodes`-budget arena
run over a new committed config and take the capture receipt from *that* report
(pricing the added cost on the gate's own pre-registration face), or move the
receipt to `crates/pistol-cli/tests/arena_smoke_gate_tests.rs` driving a scratch
workspace at a `nodes` budget; and correct `:481` and the test name
`a_capture_over_a_movetime_report_is_refused_by_name` to say what the refusal
actually is.

## MAJOR

### M1 — §5 does not say WHICH `arena_version`, and the one it cites is not reachable from pass 2

**Quoted** (`:402-409`): *"`arena_version` is written into every report from
`env!("CARGO_PKG_VERSION")` (`crates/pistol-arena/src/report.rs:130`) … Without
it, a change in **pass 2's own behaviour** produces a colliding capture
identity"*.

**Evidence.** `crates/pistol-arena/src/report.rs:130` is inside `fn instrument`
and writes the version of the binary that PLAYED. `Transcript`
(`crates/pistol-arena/src/transcript.rs:28-52`) has no `arena_version` field and
`transcript::read` (`:135-209`) never parses that line. Pass 2's own version is
`env!("CARGO_PKG_VERSION")`, available for free in `capture.rs`.

**Why it is wrong.** The two values are different whenever pass 1 and pass 2 are
different builds — the only case the input exists for. The design's *evidence*
sentence points at the report's field; its *purpose* sentence requires pass 2's
own. An implementer taking the evidence sentence literally must add a parse to
`transcript::read` (a change to pass 1's reader, brushing INVARIANT 10) and gets
a digest that is invariant under exactly the change §5 says it must move for.

**FIX.** One clause: *"`arena_version` is pass 2's own
`env!("CARGO_PKG_VERSION")`, not the source report's line"* — or the converse,
with the parse named.

### M2 — the instrument input provides the protection §5 says it provides only if someone bumps a number nobody has bumped since the first commit

**Quoted** (`:406-409`): *"Without it, a change in pass 2's own behaviour
produces a colliding capture identity, and the only thing standing between them
is a hand-maintained format version. `docs/process.md`'s "Instrument governing
revision" is the rule that requires it."*

**Evidence.** `crates/pistol-arena/Cargo.toml:3` is `version.workspace = true`;
`Cargo.toml:6` is `version = "0.0.1"`; `git log -p --follow -- Cargo.toml |
/usr/bin/grep "^[+-]version"` returns exactly one line, the `+version = "0.0.1"`
of the first commit `b0dc2cc` (2026-08-17). **The workspace version has never
moved.**

**Why it is wrong.** `arena_version` is a second hand-maintained constant, so the
digest gains no protection the format version did not already give — the
sentence's own diagnosis applied to its own remedy. And `docs/process.md`'s
"Instrument governing revision" asks that an instrument be *"named … WITH ITS
REVISION"*, whose remedy in this project is a commit SHA in a pre-registration,
not a package version that is not a revision of anything. The rule is cited for
something it does not say. This is the one open collision route from check 5(c):
a change to pass 2's normalisation or record grammar, without a hand bump,
produces a **different capture under the same `capture_sha256`**.

**FIX.** Either say plainly that both `arena_version` and the capture format
version are hand-maintained and that the pilot's pre-registration names pass 2's
**governing revision** as `docs/process.md` requires, or drop `arena_version` and
keep one hand-maintained number instead of two.

### M3 — INVARIANT 4's mutant cannot die: the only instrument that observes a `newgame` latches after the first one

**Registered** (`:644`): *"the `newgame` removed from pass 2's loop →
`every_label_go_is_preceded_by_a_newgame`"*.

**Evidence.** `crates/pistol-arena/src/bin/stub_engine.rs:268-289`:

```
let mut told_new_game = false;
…
    if asked.starts_with(pistol_cli::protocol::NEW_GAME) {
        told_new_game = true;
    } else if asked.starts_with(pistol_cli::protocol::POSITION) && !told_new_game {
```

`told_new_game` is **set once and never cleared**. The two existing witnesses are
`crates/pistol-arena/tests/seat_setup_identity_tests.rs:189`
(`every_fresh_spawn_is_sent_newgame_before_it_is_given_a_position`) and
`crates/pistol-arena/tests/replay_tests.rs:278`, both of which pin *per spawn*,
not *per go*. No other stub behaviour observes `newgame`, and the stub's own doc
at `:46-52` records why an honest engine cannot (D-413: *"deleting the send left
the whole workspace green"*).

**Why it is wrong.** Pass 2 sends `newgame` before **every** label `go` on **one**
long-lived channel. Delete every one of those sends after the first, and — if
pass 2 spawns through `seats::with_seats`, which sends one `NEW_GAME` at
`seats.rs:47`, or through any spawn path that greets once — the `demands_newgame`
stub is satisfied by the first send and every registered test stays green. **The
package's headline coldness mechanism, the thing §12 exists to justify, is
registered against a mutant that survives.** This is D-413's finding restated at
a different granularity, and the design cites `seats.rs:47` without noticing that
what that line witnesses is not what INVARIANT 4 asserts.

**FIX.** One sentence naming the witness: either a new stub behaviour that
refuses a second `position` after a `go` unless a fresh `newgame` intervened (the
`demands_newgame` shape with the latch cleared on `go`), or a seam in
`capture.rs` that makes the sent line sequence assertable in-process.

### M4 — "the vacuity is confined to one test" is false: the solver spelling is observable nowhere in this package

**Quoted** (`:786-789`): *"the stub emits those two fields, the capture must not
carry them, and every other field must survive — so both normalisation mutants
die in-crate. The vacuity is confined to one test, and it is named."*

**Evidence.** `crates/pistol-cli/src/report.rs:62-81` emits the solver block only
`if info.solver_nodes > 0`. `crates/pistol-arena/src/bin/stub_engine.rs:123` sets
`solver_nodes: 0`. `configs/gate_v0.toml:90-94` sets `on_search_path = false`,
with the comment *"the solver on the search path, gate OFF **in every committed
config** until an SPRT says otherwise"* — and §1 requires pass 1's engine sections
to name a **committed** config.

**Why it is wrong.** §4.1's whole normalisation argument turns on the solver
spelling (*"the reason the expression matches whether or not the solver ran is
that the solver block is interpolated between `nodes` and `nps`"*, `:317-320`),
and INVARIANT 6 asserts *"the solver fields appear exactly when the engine
printed them"*. Neither the registered stub test nor §14's arena-smoke receipt
can ever produce a solver-bearing totals line, so the registered mutant *"the
normalisation widened to strip another field"* survives if the field is a solver
field, and the invariant's solver clause is pinned by nothing anywhere. The
design's bound on its own vacuity is therefore wrong, and the section that
states the bound is the section written to answer the previous review's vacuity
finding.

**FIX.** State the true bound — *"no registered test can observe the solver
spelling, because every committed config has the solver off; the normalisation's
solver-spelling behaviour rests on the reading at `report.rs:62-84` and on
nothing executable"* — or register a unit test over the normalisation function
with a synthetic solver-bearing totals line, which needs no engine at all.

### M5 — §13(b) is internally contradictory and misapplies the rule it quotes

**Quoted** (`:757-762`): *"Pass 1's arena experiment config is
`configs/arena_wp20_label_pilot.toml`, **added by this package** and validated by
CI gate 6 … its row is added in the same commit. **The RANGE is a number and is
the pilot's pre-registration** (D-483)"*.

**Evidence.** `tools/config_check.sh:43,60,112-114` — every `configs/**/*.toml`
is parsed and `arena_*.toml` goes to `validate_arena_config`, so the file must be
**complete and valid** the moment it lands: `openings_file`, `openings_skip`,
`openings_take`, `turn_cap`, `n_workers`, `hang_timeout_ms`, `[budget] kind` and
`value`, four `[sprt]` floats and two engine sections with `binary_sha256`
(hard rule 1: `deny_unknown_fields`, no code-side default). And
`docs/book_v2_ledger.md` closes its own table with *"Neither has a row above,
because neither has a **committed pre-registration** drawing from this book."*

**Why it is wrong.** Two ways. (i) This package cannot author the file: its
`openings_skip`/`openings_take` are the range the same paragraph gives to the
pilot's pre-registration, and its `budget.value` is the GAME budget the dispatch
also makes a registered value. (ii) The ledger's rule assigns the row to *a
pre-registration*, and the arena config travels **with** it — so the row and the
config are the pilot's, not this package's. The design's own D-483 clause is the
proof of its own contradiction, and this is precisely the "falls between" shape
that split WP-2.0.

**FIX.** One clause: *"`configs/arena_wp20_label_pilot.toml` and its
`docs/book_v2_ledger.md` row are landed together by the pilot's
pre-registration, which is where their numbers are chosen; this package lands no
arena experiment config."* Then drop it from the `--proposes` list.

### M6 — the dispatch's registered mutant "ledger overwrite → append test dies" is left without a home, silently

**Quoted** (`:745-747`): *"**The capture mode PRINTS that row on stdout and never
writes the file**"*.

**Evidence.** `docs/experiments/wp20_dispatches.md`, Development round item 2:
*"Mutants: schema field dropped -> loader test dies; seed ignored ->
determinism receipt dies; **ledger overwrite -> append test dies**; census
direction collapsed -> its test dies."* §13 takes requirement 5 and designs a
program that appends nothing, so that mutant has no site — and §10's mutant table
carries no successor for it.

**Why it is wrong.** The decision (print, don't write) is defensible and probably
right; leaving a governing dispatch's registered mutant homeless without saying
so is not. §7 shows the design knows how to do this properly: *"This is a
departure from a review's own FIX and is named as one, because a test that cannot
fail is worse than an absent one"* (`:500-502`). The same paragraph is owed here,
against text with more authority than a review's FIX.

**FIX.** One sentence in §13(a): the dispatch's ledger-append mutant has no site
because no program in this package writes a ledger, and the property it guarded —
that a row is never lost — is carried instead by the committed manifest being a
file under review in the commit that adds it.

### M7 — INVARIANT 7's test names a comparison the same document calls impossible, and INVARIANT 10's exemption rests on it

**Quoted** (`:590-599`): *"**INVARIANT 10 IS THE ONE NO TEST PINS** … No unit
test can compare the SPRT path's output against a build that no longer exists.
Its evidence is the diff … **plus INVARIANT 7's test, which pins that keyword's
neutrality**, plus CI gate 15"*. The test is
`raising_totals_of_leaves_the_sprt_report_byte_identical` (`:623`).

**Evidence.** The only established in-crate precedent for a report-equality
assertion is `two_worker_run_report_identical_to_single_worker`
(`crates/pistol-arena/tests/run_tests.rs:130`), which compares two runs of the
**same** build. There is no golden arena report in the tree
(`git grep` finds no committed expected-report fixture for `pistol-arena`).

**Why it is wrong.** Read literally, the test's name is the cross-build
comparison INVARIANT 10 says cannot be written. Read charitably, it is a
golden-report or a node-counts-non-zero assertion — but the design never says
which, and the difference decides whether the mutant *"a fourth load-bearing
lookup added to `totals_of`"* dies (it does, against a golden report or a node
count; it does not, against a same-build self-comparison, which is invariant
under any mutation applied to both sides). So INVARIANT 10's exemption — which is
otherwise honest and well argued — leans on a leg the design has not built.

**FIX.** One clause saying what the test compares against: *"a committed expected
report over a stub fixture, so the comparison needs one build and not two"*, or
the node-count assertion that the `?`-chain mutant actually falsifies.

### M8 — §3's identity equality holds only when both seats spell the config path identically, and no in-crate test can see it

**Quoted** (`:253-257`): *"**MECHANISM.** Pass 2 refuses, by name, a report whose
two seats do not attest the same engine — the same `id_lines`, `binary_sha256`,
`config_sha256` and `weights_sha256`. A self-play report has two labels and one
identity"*.

**Evidence.** `crates/pistol-cli/src/bin/pistol.rs:142-166` —
`identity_lines`'s first element is `format!("config {}", path.display())`, the
**path as spelled**, and `crates/pistol-arena/src/identity.rs:74,82` puts
`spoken.lines` into `EngineIdentity::id_lines` verbatim. `config_sha256` is the
file's **content** (`identity.rs:84`). So two `EngineSection`s naming one file by
two path strings (the same file with and without a leading dot-slash, say) produce equal digests
and **unequal** `id_lines`, and §3 refuses the report.

**Why it is wrong.** The design asserts the equality without its precondition, in
the same package that authors the config where the precondition must hold. It
fails safe, which is why this is MAJOR and not BLOCKING — but the registered test
`a_self_play_report_whose_seats_carry_distinct_labels_is_accepted` cannot detect
the gap, because the stub identifies with `vec![weights_line]` only
(`crates/pistol-arena/src/bin/stub_engine.rs:262-266`) and emits no `config` id
line; and §14's real-`pistol` receipt, which is the only place it could be seen,
is **B1**.

**FIX.** One sentence: the two engine sections must name the engine config by the
identical path string, because `id config <path>` is an id line and the identity
comparison includes it — with the refusal's message saying so, since the failure
is otherwise unreadable.

## MINOR

- **m1 (D-547).** §2's quoted block (`:184-188`) drops rev2's closing sentence
  *"Revision 1's INVARIANT 4 forbade "exclusion by outcome" and would have forced
  the hang."* (`7af62e7:91-92`), inside a block introduced as a lift of a passed
  section. §0.2's row for §2 (`:46`) names three edits and not this one.
  Correctness-neutral, but D-547 makes an unlisted edit a finding by itself.
  **FIX:** add it to §0.2's row, or restore the sentence.
- **m2 (D-547).** §6's table row 1 was reworded from *"its two engine sections
  differ"* to *"its two seats attest different engines"* (`:441` vs
  `7af62e7:166`). §0.2's row for §6 (`:50`) lists only *"two rows added"*. The
  edit is required by BLOCKING A's remedy and is correct; it is still unlisted.
  **FIX:** one clause in the §0.2 ground.
- **m3.** §0.2's row for the header table (`:53`) grounds the extension on
  *"which slot answers is a fifth capture decision"*, but the added row is
  *"WHAT is written, and in what grammar | §4"* (`:74`); the slot was folded into
  the existing engine row. The quoted block also silently drops rev2's word
  *"four"* (`:66` vs `7af62e7:28`). **FIX:** name the row that was actually
  added.
- **m4.** `:779` — *"**Neither test crate can run both programs.**"* is
  overstated: `crates/pistol-arena/tests/sprt_power_tests.rs:24-37` establishes
  the `current_exe()`-relative route to a same-directory target that is not a
  `[[bin]]`, and `cargo test --workspace` builds `pistol` into that directory.
  The route is worse than the design's choice (it breaks under
  `cargo test -p pistol-arena`), so the conclusion survives. **FIX:** *"no route
  that survives `cargo test -p pistol-arena`"*.
- **m5.** `:251` — *"It derives `PartialEq` at
  `crates/pistol-arena/src/identity.rs:11`"*. The derive is at `:12`; `:11` is
  the doc line. The checker cannot catch this. **FIX:** `:12`.
- **m6.** INVARIANT 11 (`:583-585`) says a capture file *"whose fields carry a
  TAB is refused by name"* — but on read, a field carrying a TAB is
  indistinguishable from an extra field, so that clause collapses into the arity
  clause and the loader cannot make it independently. The registered test
  `a_captured_field_containing_a_tab_refuses_the_run_by_name` is a **write**-side
  behaviour pinned to a **loader** invariant. And neither the invariant nor §4.3
  says the loader refuses an *empty* field, which is always an error here.
  **FIX:** move the TAB clause to an invariant about writing, and add
  "or is empty" to the loader's arity refusal.
- **m7.** INVARIANT 12 names four digest inputs; three have a test and a mutant
  and **the capture format version has neither**. **FIX:** register
  `a_capture_identity_moves_when_the_format_version_moves` and its mutant.
- **m8.** The mutant *"a capture record's fields reordered on write"* (`:657`)
  may not die: fields 1 and 2 are both decimal, and swapping them round-trips
  through a positional loader, satisfying
  `a_capture_file_round_trips_through_its_own_loader`. **FIX:** make the
  round-trip test assert field values, not merely that parsing succeeds.
- **m9.** §8 (`:539-544`) states that a malformed totals line makes *"the run …
  refused by name"*, in a section that does not own failure modes, while §6's
  table — which does — carries no such row, no test and no mutant. D-423's
  "state it once, in the section that owns it". **FIX:** one row in §6's table.
- **m10.** INVARIANT 3's limb *"spawns slot zero"* is pinned by no test. Unlike
  INVARIANT 10, the design does not say so. **FIX:** either register a test
  asserting one asking channel bound to slot zero's section, or add the limb to
  the INVARIANT 10 paragraph's list of what no test pins and why.
- **m11.** `a_captured_field_containing_a_tab_refuses_the_run_by_name` has no
  engine in this tree that can emit a TAB (`pistol` cannot; the stub goes through
  `pistol_cli::Session` and cannot), so the test needs either a new stub
  behaviour or a unit test over the record-writing function with a synthetic
  field. The design says which for §14's obligation and not for this one.
  **FIX:** one clause naming the driver.
- **m12.** The dispatch's *"throughput expectation stated as a shape, measured in
  the pilot, never guessed (D-500's class)"* is in neither §11's deferral list
  nor delivered anywhere; §12's memset cost is the cost, not the throughput.
  **FIX:** one line in §11 assigning it to the pilot's pre-registration.
- **m13 (hard rule 10).** Two ADR amendments this design's shape makes owed and
  does not request: `matrix_wp20_shape_selection.md:61`'s *"only `pistol-arena`
  is touched"* is falsified by §14's `tools/` and `pistol-cli/tests/` extensions
  (the design names the departure at `:806` but asks for no amendment); and
  `/usr/bin/grep -c "capture_sha256" docs/decisions.md` → **0** while
  `label_sha256` → **1** (D-544), so the rename is unrecorded for a third
  revision (rev-1 MINOR 3, rev-2 MINOR 3, both NOT APPLIED). **FIX:** name both
  as ADR acts owed at landing.
- **m14.** `crates/pistol-arena/src/bin/arena.rs:94-99`'s fallback refusal
  enumerates the two existing modes; a third arm needs it extended, and the
  `USAGE` text at `:16-59` likewise. The design specifies the `USAGE` extraction
  but not its content. **FIX:** one clause.

---

## Could an implementer build from this without deciding something the design should have decided?

## **No.**

Seven decisions, before a line is written — and the list is **much shorter and
much better than either predecessor's**:

1. **Where the real-binary re-run receipt is actually taken**, since the named
   site cannot read the report it is pointed at (**B1**). This is the same
   decision rev-2 MAJOR C raised, unresolved by a different route.
2. **Which `arena_version` enters `capture_sha256`** — pass 2's own, or the
   source report's, which is not on `Transcript` (**M1**).
3. **How `every_label_go_is_preceded_by_a_newgame` is made able to fail** — a new
   stub behaviour, or a seam (**M3**).
4. **Who lands `configs/arena_wp20_label_pilot.toml` and with what numbers**
   (**M5**).
5. **What `raising_totals_of_leaves_the_sprt_report_byte_identical` compares
   against** (**M7**).
6. **Whether the two engine sections must spell the config path identically**,
   which decides whether a legitimate pilot report is accepted (**M8**).
7. **How the TAB test is driven**, and what the loader does with an empty field
   (**m6**, **m11**).

Decisions 1, 3 and 5 decide whether the package's three headline invariants (6,
4 and 7) are pinned or vacuous — the D-527 class, which this arc has now paid for
four times. Decisions 2, 4 and 6 change what the corpus *is* or whether it can be
produced at all.

**What is genuinely retired since revision 2:** the seat comparison and its
type (BLOCKING A — the remedy is correct and I could not break the direction that
matters), which slot answers, the capture record's delimiter/order/arity, the
loader and its test, the coldness ground and the pilot's owed criterion, the
`bestmove` line's invariant, and the CLI grammar. Seven of rev-2's twelve
open items are closed, and closed against the tree rather than by assertion.

---

## The strongest attack that did not land

**I attacked §3's rebuilt seat check on the hypothesis that it is BLOCKING A
wearing its fix — that two seats of one self-match could attest different
identities, or that two different engines could attest the same one.** The
dispatch pre-declared that reading BLOCKING. It came back MAJOR, in one
direction only, and here is why the harder halves closed:

1. **Can two DIFFERENT engines attest EQUAL identities?** No.
   `EngineIdentity::binary_sha256` is `digest_of(&engine.binary)` computed
   **before the spawn** (`identity.rs:65`) and re-checked against the config's
   declared digest (`:66-73`), and `config_sha256` is the config file's content
   (`:84`). Two different binaries or two different config documents cannot
   compare equal. The refusal is sound in the direction that protects the corpus.
2. **Does the equality reach the report round-trip intact?** Yes.
   `report::instrument` writes `engine_id {slot} {line}` per id line
   (`report.rs:179`) and the six-field `engine` record (`:170-178`);
   `transcript::read_engines` (`transcript.rs:215-266`) reads them back
   field-for-field with a twelve-word arity refusal. The identity pass 2 compares
   is the identity pass 1 captured.
3. **Does `verify_engines` close the gap between "the report attests" and "the
   binary is"?** Yes, and it is stricter than a same-run check —
   `replay.rs:216-243` re-captures each slot through `identity::capture` and
   compares the **whole** `EngineIdentity`, `id_lines` included, which
   `experiment_digest` does not. D-252's own reproducer is covered.
4. **Only the config PATH SPELLING varies**, and that is M8 — a false refusal,
   loud and by name, not a false acceptance.

**Two more attacks failed and are recorded so nobody re-runs them:**

- **D-540's *"`book_v2` … never by committed configs"* against §13(b)'s
  `configs/arena_wp20_label_pilot.toml`.** I expected a clean ADR breach: every
  file under `configs/` is tracked and gate 6 validates it, so a pilot arena
  config drawing from book_v2 would be a committed config referencing it. It is
  not a breach. D-540's own reason clause — *"rather than of a file **a shipped
  seat** happens to read"* — scopes "committed configs" to the engine configs a
  seat reads, and `docs/book_v2_ledger.md:16` explicitly contemplates a
  pre-registration **adding its arena config**. `git grep -ln
  "random_openings_v2" -- configs/` returns only the two generator configs, which
  is consistent with either reading and decides neither. The finding against
  §13(b) is **M5**, its internal contradiction — not D-540.
- **§4.1's normalisation, re-attacked on the solver spelling.** The rev-2
  reviewer closed four routes; I tried a fifth — that `render_info`'s solver block
  might separate `nps` from `time` in some configuration. It cannot:
  `crates/pistol-cli/src/report.rs:82-84` interpolates `{solver_field}` strictly
  between `nodes {}` and `{NPS_FIELD} {}`, and `{NPS_FIELD} {} {TIME_FIELD} {}`
  is one literal run. `determinism.sh:154`'s `sed -E 's/ nps [0-9]+ time
  [0-9]+//'` matches in both spellings. **§4.1 remains the best-verified
  paragraph in the document across three reviews.** Its residual is M4 — that
  nothing executable in this package exercises the spelling — not the expression.

---

## What I could not settle by reading, and the run that would

- **B1's exact failure text.** I established from `transcript.rs:164` and
  `configs/arena_smoke_v0.toml:57` that the capture pass is refused; I did not
  observe the refusal. **The run:** `tools/arena_smoke.sh` to produce a report,
  then `arena --capture <that report> --out C --label-nodes <n>` — which should
  print `arena: replay report: the run used a `depth_turns` budget and only a
  `nodes` budget replays…`. Refused here per the dispatch.
- **M3's mutant, end to end.** That `told_new_game` latches is read off
  `stub_engine.rs:268-289`; that a pass 2 with per-position `newgame` deleted
  goes green is inferred. **The run:** in a worktree, write `capture.rs`, delete
  the in-loop `newgame`, and run `cargo test -p pistol-arena` — the mutation
  should not be killed.
- **M7's baseline.** Whether
  `raising_totals_of_leaves_the_sprt_report_byte_identical` is writable at all
  without a golden report is a design question the document must answer, not a
  run — but **the run that would show the mutant's fate** is: add
  `value("solver_nodes")?` to `totals_of`, run `cargo test -p pistol-arena`, and
  see whether anything dies.
- **The headroom after the edit** (carried from both prior reviews, still open).
  283 and `SOFT_CAP=300` are measured; the third arm's own size is not. **The
  run:** write the arm, extract `USAGE` to `usage.rs`, run
  `tools/file_justification_check.sh`. The design is now correctly silent about
  the number and names this run instead, which is the right shape.
