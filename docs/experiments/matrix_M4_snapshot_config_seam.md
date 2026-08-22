# MATRIX M4 — the snapshot's config seam — **STOPPED AFTER TWO ROUNDS, NOTHING SELECTED**

**Status: TWO REVISIONS, TWO FRESH-CONTEXT DECISION-RED-TEAMS, NO SELECTION.**
Revision 1 authored `77f7397`, attacked in
`docs/experiments/matrix_M4_REDTEAM.md`. Revision 2 authored `cb16f7c`, attacked
in `docs/experiments/matrix_M4_REDTEAM_round2.md`. Owning unit:
`docs/experiments/U4_soundness_instrument.md` §9. **D-318 records the stop.**

> **REVISION 2 IS BELOW, VERBATIM AS ATTACKED, AND IS NOT EDITED** — and revision 1
> is quoted verbatim inside it, likewise unedited (round 2 verified 165/165 lines
> identical to `git show 77f7397:`). Everything in this STOP RECORD is above the
> body. A matrix corrected after its attack is a matrix that was never attacked.

## WHY THIS STOPS RATHER THAN SELECTING, when options DID survive

Round 2 did **not** kill every option — N-J survives, and N-E, N-K, N-L and N-F
survive wounded — so the architect's "kills every option" stop rule is **not**
what fires here. Three other things do, and each alone would be enough:

**1. REVISION 2'S OWN REASON FOR EXISTING IS FALSE, and the tree contains a
landed ADR line warning against precisely the error it made.** Revision 2
declined to select from revision 1's field because that field omitted "the seam
this project's own D-252 option matrix SELECTED … after its own red team, and
which D-283 landed". **Neither conjunct is true, and both are checkable in
seconds:**

```
$ grep -o "^D-288:.\{0,260\}" docs/decisions.md
D-288: D-252's OPTION (c) IS RELABELLED — IT WAS A DEFERRAL AND IT SAID "ADOPTED",
AND THE LABEL HAD TEETH. The line reads «(c) ADOPTED: leave the literal, register
the exposure, and schedule the binding», and what (c) actually did was decline to
bind … That is a DEFERRAL.
```

D-252's option (c) is **N-L's shape** — leave the literal, register the exposure,
schedule the binding — not a document seam. **D-288 exists in this tree for the
sole purpose of relabelling it "DEFERRED … and NO OPTION SELECTED"**, and it
states the harm in terms that describe revision 2 exactly: *"a successor picking
up WP-1.10 and reading (c) as adopted would have found the decision already
taken, the matrix already spent, and no red-team owed, on a choice that had never
been attacked."* Revision 2 is that successor. And D-283 says of its own
sub-choices, in its own text, that **they have not been attacked by a
fresh-context DECISION-RED-TEAM**.

**2. EVERY PRECEDENT GROUND IS VOID, so no ground discriminates.** The tree holds
**no attacked selection** for how an instrument binds a per-run input: D-252
selected nothing (D-288), D-283's own review is outstanding by its own text, and
U4-Z's B3 shape comparison says of itself that it was never attacked (D-316's
recorded residual). Grounds 1, 3 and 4 of the recommendation fall with the
precedents they read. **What survives is ground 2 — refuse the silent-wrong-config
record — and ground 2 argues equally for N-E, N-J, N-K, N-L and N-F, none of
which has a default.** Selecting one of five on a ground that does not
discriminate between them is forcing a survivor by another name, which is the
thing the stop rule exists to prevent.

**3. THE FIELD IS STILL INCOMPLETE AT THE SECOND REVISION, and one missing row
dominates the recommended option on the matrix's own trigger.** Rule 1's
mechanism, which ground 1 invokes, is a **closed enum** — `Budget` is
`{depth_turns, nodes, movetime_ms}`. A required `--config {instrument|staged}`
**selector over the committed set** satisfies grounds 1 and 2 identically to N-E
while owing far fewer guards, because guards (i) caller-relative resolution and
(iii) the three path refusals exist only because a caller can name an ARBITRARY
path. The matrix's framing — "any option that **lets a caller name the path**", "an
argv string" — excludes it by presupposition, **for the second revision running**.
A sixth row is also missing: binding the config through the corpus fixture, which
is already caller-named by an existing flag and already digested above the marker,
and which reaches N-J's "strictly stronger property" with no new document and no
new flag.

## THE AUTHOR'S OWN DEFECTS, recorded because they are the pattern and not the incident

- **F7 REPEATED ONE LEVEL UP.** Round 1 found that revision 1's self-disclosure
  pointed the red team away from where its own error was. Revision 2 spent a
  paragraph on that finding — and then committed **F5's and F7's class fresh in
  the two new facts carrying grounds 1 and 3**: fact 12's command reads only D-283
  and cannot see D-252 or D-288, and fact 10's command reads only `pistol.rs`
  while the fact asserts something about `arena.rs`. **Candour about a past error
  is not a control on the present one**, and this is the second round in which
  candour did the work of concealment.
- **A THIRD MEASURED NUMBER THAT DOES NOT REPRODUCE.** Fact 6d's five-site
  enumeration contains a non-site: MEASURED by round 2, with `go()` and test 1205
  patched and 1036 left alone, the suite is **29/29 green**. Revision 2 corrected
  revision 1's false "three" and put a different wrong number in its place, still
  driving flip clause 1. With M3's fact 5 and M4's fact 6b, that is **three
  MEASURED cells from this author in one session that did not reproduce, all in
  cells supporting a recommendation.**
- **N-L's "Zero code change" is MEASURED FALSE**, and N-L is the remedy of two
  flip clauses on the stated ground that it "changes no caller at all".
  Re-pinning the literal alone turns
  `snapshot_deterministic_across_a_clean_and_a_dirty_working_tree` red, because
  the fixture at `crates/pistol-cli/tests/baseline_snapshot_tests.rs:385` copies
  `configs/instrument_v0.toml` **by name**. Verified by reading that file.
- **"Revision 2 marks every number" is false** in the exact cell round 1 named
  (N-D′'s "Zero"), plus N-B′'s "Zero", N-L's "Zero code change", and an unmarked
  "15".
- **D-283's condition is read BACKWARDS by grounds 3 and 4.** They turn on D-283
  choosing the document *"because the arena had no flag surface at all"*. The
  arena takes `--config` and `--out` (`crates/pistol-arena/src/bin/arena.rs:77`),
  and D-283's condition is about the **INPUT BEING BOUND** — *"the arena has no
  `--binary` flag"*. Applied as written it puts the snapshot, whose config has no
  flag, in the **same** case as the arena, which is the opposite of what the
  taxonomy concluded.

## WHAT THE THIRD ROUND IS OWED — the architect's, not this session's

Both rounds warned about the same thing and D-305 measured it: **the red team
supplied the surviving option in four of six matrices in this work package.** A
third field authored by the session that wrote the first two, now populated with
rows both red teams supplied, is that pattern completed rather than broken. The
round is owed, and it owes:

1. The two missing rows named above — the **closed-enum selector** and the
   **corpus-fixture binding** — the first of which dominates N-E on the matrix's
   own trigger.
2. A field framed without the "caller names a path" presupposition that excluded
   them twice.
3. Costs that are MEASURED where a command settles them in seconds, and marks on
   every number including the zeros.
4. **No precedent ground that has not itself been attacked.** D-252 selected
   nothing, D-283 is unattacked by its own admission, and B3's shape comparison is
   unattacked by D-316's own residual. Any third-round ground resting on one of
   those must say so on its face.

## WHAT THIS BLOCKS

`tools/baseline_snapshot.sh` gains no config seam, so **no Staged snapshot can be
taken** and the registered above-marker quantity has a BEFORE and no AFTER. U4-Z
item 15 stays blocked and **B2 — M4 has no ADR line — stays open**, now for a
measured reason. The pre-registration contradiction below is unchanged.

## OPEN — escalated, NOT repaired

`docs/experiments/wp15b_sprt_prereg.md` asserts *"MATRIX M4 **ADOPTS** adding
`--config`"*. This session is barred from editing the pre-registration. Round 2
completes the record revision 2 left incomplete: that document is **Revision 4, a
DRAFT, "GOVERNS NOTHING YET", never reviewed, with an OPERATOR-CONFIRM slot** — so
the contradiction is a draft's forward reference and not a governing claim. It
still must be re-read against whatever the third round selects, and it currently
names a mechanism no matrix supports.

---
---

# REVISION 2 — VERBATIM AS ATTACKED at `cb16f7c`

*Everything below this line is the text the round-2 DECISION-RED-TEAM was
dispatched against, unedited, and it contains revision 1 quoted verbatim inside
it. Its "Status: AUTHORED, NOT SELECTED" is the state at attack time and remains
true. The corrections are above.*

# MATRIX M4 — the snapshot's config seam — **REVISION 2**
Status: **AUTHORED, NOT SELECTED.** Awaits a SECOND fresh-context
DECISION-RED-TEAM. Subject: `tools/baseline_snapshot.sh`. Owning unit:
`docs/experiments/U4_soundness_instrument.md` §9, **u-rev 3** (§9's bytes are
unchanged since u-rev 2; the label moved under D-311 when §8.4 was repaired).
Revision 1 authored at `77f7397`; revision 2 authored at `dab170b`.

> **REVISION 1 IS BELOW, VERBATIM AS ATTACKED, AND IS NOT EDITED.** A matrix
> corrected after its attack is a matrix that was never attacked. Revision 2 is a
> NEW TEXT that supersedes it, and under this project's own rule it is not
> attacked until a fresh-context DECISION-RED-TEAM has attacked THIS revision.
> **No ADR line may cite revision 1's field as the field a selection was made
> from**, because revision 1's field was measurably incomplete.

## WHY THERE IS A REVISION 2 RATHER THAN A SELECTION

The round-1 DECISION-RED-TEAM (`docs/experiments/matrix_M4_REDTEAM.md`, against
`77f7397`) returned **three kills**, and its verdict was that *"the matrix does
not survive as a matrix"*. Two of the kills are repairable in place. **The third
is not**: the option space omitted four rows, one of which is the seam this
project's own D-252 option matrix SELECTED for the structurally identical
question after its own red team, and which D-283 landed.

**Selecting from revision 1's field would have been selecting from a field known
to omit a precedent-selected rival.** That is the defect F11 named in the
restructure round, arriving one level up: not a matrix nobody attacked, but a
matrix whose attack succeeded and whose field was then used anyway. So the
architect's stop rule is applied in the spirit it was written: revision 1 is not
selected from, revision 2 states the full field, and revision 2 is attacked
before anything is selected.

## ROUND-1 DISPOSITION — every finding, and what revision 2 does with it

| # | Finding | Disposition in revision 2 |
|---|---|---|
| **F1** | KILL. Fact 6b's "there is one invocation site" is false; the test file holds THREE `Command::new("bash")` sites, and a required-config mutation patched only into `go()` leaves `a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root` RED (control 29/29 green). | **ACCEPTED, reproduced independently.** Fact 6b is withdrawn and replaced; N-E's cost cell is re-stated at **at least five sites**; and **ground 4 is DELETED as a ground** rather than repaired — a ground whose number was false in the direction that favoured its own recommendation does not get to stay on as a smaller number. |
| **F2** | KILL. Flip clauses 1 and 2 name N-A′ as the remedy while ground 1 declares N-A′ a hard-rule-1 breach. F5's incoherence, reproduced in the section that claims to have avoided it. | **ACCEPTED.** The flip section is rewritten. No clause names a remedy another part of the matrix forbids, and the one clause that CAN reach N-A′ says exactly what must change first. |
| **F3** | KILL. Four rows missing: the document/manifest seam (D-252 selected it, D-283 landed it), a config-PAIR mode, re-pinning the literal by ADR, and N-B never re-adjudicated after its rejection ground was measured false. | **ACCEPTED IN FULL.** All four are entered: **N-J**, **N-K**, **N-L**, **N-B′**. |
| **F4** | MAJOR. N-G is rejected on a ground fact 4 falsifies — the record attests `config <path> <sha>` above the marker whatever the value's origin. | **ACCEPTED.** N-G's rejection is re-grounded on caller-side reproducibility, which is the true defect. |
| **F5** | MAJOR. Fact 7 marks MEASURED a "four" its command cannot produce, and restates a count `U3` §10 owns exclusively — a fifth site for B5's defect, inside a U4 artefact. | **ACCEPTED.** Fact 7 now states only what `ls configs/` measures, and cites **U3** §10 without restating the count. |
| **F6** | MAJOR. Flip clause 3's "not a selection trigger" is true only between N-A′ and N-E; **N-F owes none of the four guards**, so the guard count IS the flag-versus-no-flag differential. | **ACCEPTED.** It is a selection trigger, and revision 2 states it as one. |
| **F7** | MAJOR. Revision 1's self-disclosure pointed the red team outside `crates/` while the defect was inside it. | **ACCEPTED, and it is the sharpest finding in the round.** See the note under the recommendation. |
| **F8** | MAJOR. Ground 3's precedent slate is not blank. | **ACCEPTED.** Ground 3 is rewritten around the precedent that exists. |
| **F9** | MINOR. Fact 3's `tools/*.sh` glob does not cover `tools/`. | **ACCEPTED.** Re-measured over all of `tools/`; the conclusion survives, the instrument did not. |
| **F10** | MINOR, sharpening F1. | **ACCEPTED**, folded into the corrected audit. |
| **F11** | MAJOR. `docs/experiments/wp15b_sprt_prereg.md` already asserts "MATRIX M4 **ADOPTS** adding `--config`" at the same commit the matrix says nothing is selected. | **RECORDED, NOT REPAIRED — and it is escalated to the architect.** This session is barred from editing the pre-registration. The inconsistency is real and it is live. See OPEN, below. |
| **F12** | MINOR. Unmarked numerics. | **ACCEPTED**; revision 2 marks every number. |
| **F13** | REJECTED by the red team itself: ground 1's transfer of hard rule 1 to a shell script's default is defensible and **better supported than revision 1 knew**. | **CARRIED, and strengthened with the evidence the red team supplied.** |

---

## FACTS THE MATRIX STANDS ON — MEASURED at `dab170b`, with the command

| # | Fact | Command |
|---|---|---|
| 1 | `CONFIG="configs/instrument_v0.toml"` is a **literal at line 170** with no flag, read at **five** further sites (271, 321, 425, 464, 531) and emitted at 440 | `grep -n CONFIG tools/baseline_snapshot.sh` |
| 2 | The script has **six** flag arms: `--out`, `--nodes`, `--corpus`, `--ladder-depth`, `--ladder-cap-s`, `--binary` | `grep -cE '^\t--' tools/baseline_snapshot.sh` → 6 |
| 3 | **No artefact under `tools/` takes `--config`** — re-measured over the WHOLE directory, not the `*.sh` glob revision 1 used, which missed `tools/wp15b_attribution_check.py` (F9). `tools/` holds 18 entries: 16 `.sh`, 1 `.md`, 1 `.py` | `ls tools/`; `grep -rln -- '--config)' tools/` → no output |
| 4 | `config <path> <sha>` is written into `$INVARIANT` (line 440) and the marker is echoed at line 633, so **the record attests its config ABOVE the marker whatever the value's origin** — this is a property of the record, not a property of any option, and revision 1 granted it to one option as mitigation while withholding it from another (F4) | read lines 435–445 and 628–640 |
| 5 | Per-position `depth_turns`/`nodes` → `$INVARIANT` (494); `timing position …` → `$TIMING` (496); `timing depth_at_500ms` → `$TIMING` (605). The moved-to registered quantity is above the marker and the demoted one below, in the shipped script | read lines 490–498, 600–610 |
| 6 | The shipped script is driven by **29** tests in `crates/pistol-cli/tests/baseline_snapshot_tests.rs` | `grep -c '^#\[test\]' …` → 29 |
| **6b** | **WITHDRAWN AND REPLACED (F1).** Revision 1 said all 29 tests invoke through ONE helper. **They do not: the file invokes the shipped script at THREE places** — `go()` (172), and two tests that build their own `Command` (1036, 1205), neither routed through `go()` and neither passing a config | `grep -n 'Command::new("bash")' crates/pistol-cli/tests/baseline_snapshot_tests.rs` → 172, 1036, 1205 |
| **6d** | **The caller audit for a REQUIRED config is at least FIVE sites** — the default literal (170), the usage block (109 ff.), `go()`, test 1036 and test 1205. MEASURED by the round-1 red team in a mutation worktree on `/home`: unmutated control 29/29 green; the mutation with no test-side change 28 failed; the mutation with `--config` patched into ONLY `go()` still leaves `a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root` RED. **Not re-measured by this revision** — cited to that report, which is landed and retrievable | `docs/experiments/matrix_M4_REDTEAM.md` F1 |
| 6c | `tools/ci.sh` does **NOT** invoke the snapshot | `grep -n baseline_snapshot tools/ci.sh` → no output |
| 7 | **CORRECTED (F5).** `ls configs/` measures that `configs/` holds **12** documents and that **no staged config exists on disk yet**. It measures nothing about how many staged configs there will be; **U3** §10 is the one place that count is stated and this matrix cites it without restating it. The point this fact carries needs no count: nothing staged exists yet, so a breaking change to the invocation spelling breaks no staged record | `ls configs/ \| wc -l` → 12; `ls configs/ \| grep -c staged` → 0 |
| 8 | `configs/instrument_r2_v0.toml` exists and is referenced by no script and no crate — no precedent in this tree for pointing an instrument at a second committed config | `grep -rn instrument_r2_v0 tools/ crates/` → no output |
| 9 | The script already passes `--config "$CONFIG"` **to the engine** at three call sites (425, 464, 531): the engine's own flag is spelled identically | `grep -n 'BINARY" --config' tools/baseline_snapshot.sh` |
| **10** | **NEW (F8, F13).** Required-with-no-default `--config` is **shipped twice** in this tree, and one of them cites hard rule 1 by name: `crates/pistol-cli/src/bin/pistol.rs:34` — *"an engine config. Always explicit: there is no default path"* — and the arena binary matches only `["--config", c, "--out", o]` | `grep -n -- '--config' crates/pistol-cli/src/bin/pistol.rs` |
| **11** | **NEW (F3c).** **Five** `tools/` scripts pin a config literal with no flag: `arena_smoke.sh:59`, `baseline_snapshot.sh:170`, `bench_delta.sh:99`, `movetime_check.sh:32`, `determinism.sh:39` | `grep -rn '^CONFIG=' tools/` |
| **12** | **NEW (F3a).** **D-252's option matrix chose the DOCUMENT seam for the structurally identical question and D-283 landed it**, with the reasoning *"a required document key cannot be forgotten the way a flag can"* and *"an optional binding is a binding nobody has"*. **AND D-283 states the CONDITION under which the document was the seam**: *"the arena has no `--binary` flag … so the only binding available is the one the arena reads paths out of"* | `grep -o "^D-283:.\{0,1500\}" docs/decisions.md` |

**The four guards owed by any option that lets a caller name the path** — MEASURED
enumeration, carried from the design's amendment 2: (i) caller-relative
resolution, as `--out` and `--binary` each got; (ii) the printable allow-list
extended to the whole `$CONFIG` path, because unlike `--corpus` it reaches the
record as a WHOLE PATH on two invariant lines rather than through `$(basename …)`;
(iii) three named refusals — directory, missing, not a regular file — against line
271's bare `[ -f ]`; (iv) an assertion that the script's `config` line and the
engine's `engine_id config` line name the same document. **N-F, N-L and N-B′ owe
NONE of these, because none lets a caller name a path; N-J owes (ii) and (iv) only,
against a parsed document key rather than an argv string. THE GUARD COUNT IS
THEREFORE A SELECTION TRIGGER AND NOT A COMMON COST** (F6) — revision 1 called it
common by narrowing the field to the two options that shared it.

## WHAT THE OPTIONS ARE OPTIONS ABOUT — reframed (F3b)

Revision 1 asked *"by what seam does the snapshot produce **a** record taken under
a staged config"*. The singular was doing unnoticed work: the deliverable is a
**comparison of two records**, and asking for one record excluded the pair-shaped
options by presupposition. Revision 2 asks:

**By what seam does the snapshot instrument produce the BEFORE and AFTER records
that WP-1.5b's registered quantity is a comparison of — given that the quantity
now lives above the `# timing` marker, that no staged config exists yet, and that
the instrument's own revision is named by whatever pre-registration consumes it?**

## Options

| Option | What it does | Cost | Failure modes |
|---|---|---|---|
| **N-A′ — OPTIONAL `--config PATH`, default kept** | A seventh flag arm; `CONFIG` keeps its literal as the default. | Four guards; **ESTIMATED 5–8 new tests**; no caller audit. | **FELL in round 1, and not to this matrix's argument — to three in-tree authorities.** Fact 10: two shipped binaries take `--config` REQUIRED with no default, one citing rule 1 by name. Fact 12: D-283 landed *"an optional binding is a binding nobody has"* for this class after a red team. A default is the failure mode: a caller who omits the flag gets a record that names the radius config and looks like every other record. **Entered so revision 2's field is complete; it is not a live candidate unless flip clause 4 fires.** |
| **N-E — REQUIRED `--config PATH`, no default** | Same arm; `CONFIG` starts unset and an invocation without it is refused by name. | Four guards; **ESTIMATED 5–8 new tests plus one refusal test**; caller audit **at least FIVE sites** (fact 6d) — **NOT three, as revision 1 claimed**. | Breaking change to a recently reviewed instrument before IMPL starts. Its shape matches both shipped binaries (fact 10) but its seam is a command line, and the seam D-252 selected for a per-run binding was a document (fact 12) — **revision 1 never made that comparison**, which is F3's kill. |
| **N-J — a REQUIRED SNAPSHOT RUN DOCUMENT** *(NEW, F3a)* | The per-run inputs move into a committed document carrying `config` and a required `config_sha256`; the script takes that document, required and with no default, validates the key's SPELLING (64 lowercase hex, as `ArenaConfig::validate` does) and refuses a digest mismatch before the engine is launched. | A document schema, its validation, and the coverage rule's tests — **ESTIMATED the largest change of the surviving options.** Owes guards (ii) and (iv) only. | **The precedent's own condition does not hold here** (fact 12): D-283 chose the document *because the arena had no flag surface at all*, and the snapshot has six arms and already spells the engine's flag `--config` (fact 9). It adds a document to bind a document. **But it is the only option that makes the record's config binding CHECKABLE BY DIGEST before the run**, which is a strictly stronger property than any flag option offers, and it is the shape this project selected once for this class. |
| **N-K — a CONFIG-PAIR / two-record mode** *(NEW, F3b)* | One invocation, two configs in, two records out — one instrument revision, one binary digest, one machine, one schedule. | Four guards ×1 (one parse site, two values); **ESTIMATED** larger than N-E and smaller than N-J; changes the record's top-level shape, so the 29-test suite's record assertions move. | **Makes the comparison's confound structurally impossible rather than merely refused** — the two records cannot come from different instrument revisions or machines, which is the variance the `# timing` marker discipline exists to fence off. Against it: it changes what a snapshot record IS, and D-209's golden transcripts and `tools/artifact_check.sh` both read the current shape. It also answers a question wider than M4's. |
| **N-L — RE-PIN THE LITERAL BY ADR, no flag** *(NEW, F3c)* | Leave the seam alone. Take BEFORE at instrument revision R1, change the literal by ADR, take AFTER at R2. | **Zero code change.** Two ADR lines and two runs. | **MEASURED as the `tools/`-local norm** (fact 11: five scripts pin a literal). The instrument clause already forces the pre-registration to name the script with its revision, and the record's own `config <path> <sha>` line says which document ran, so it is auditable. Against it: the BEFORE and AFTER come from two different instrument revisions BY CONSTRUCTION — the confound N-K removes, made mandatory — and every future staged snapshot needs another ADR line and another commit. |
| **N-F — a second committed script** | `tools/baseline_snapshot_staged.sh`, its `CONFIG` literal the staged document. | **MEASURED 646 lines** duplicated, or a shared body extracted and both scripts rewritten around it. Owes none of the four guards. | Two instruments that must stay in step while the whole point is comparing their outputs. Rule 9's cap bites on a duplicated 646-line body. **Revision 1 costed it "ESTIMATED the larger `tools/` change of any option" with no command, in the direction disfavouring the recommendation's most distinct rival (F8's shape)** — that estimate is withdrawn; what is MEASURED is the 646 lines, and whether extraction is larger than N-J's schema is **not measured and is not claimed**. |
| **N-B′ — flip `configs/instrument_v0.toml` to staged** *(RESTORED, F3d)* | No tools change; the standing instrument measures Staged by construction. | Zero. | **Restored because revision 1 used the falsification of one of its rejection grounds to void the old table and then dropped the rival — banking the benefit without paying it.** The false ground is gone: `grep -c instrument_v0` on the D-209 golden is 0. **Its three surviving grounds still reject it**, and they are sufficient: it lands the strength claim before its judge (rule 6), it contradicts the D-190/D-194 precedent, and it fires D-204's flip clause on this session's authority rather than the operator's. The real exposure the false ground concealed is `tactical_v0.txt`'s **15** `instrument_v0`-bound cases under D-204. **Rejected, on grounds that hold.** |
| **N-G — an environment variable** | `CONFIG="${PISTOL_SNAPSHOT_CONFIG:-…}"`. | **MEASURED one line**; **ESTIMATED 2–3 tests**. | **Rejected, RE-GROUNDED (F4).** Revision 1 said the record could not attest its config; fact 4 falsifies that — the record attests `config <path> <sha>` above the marker whatever the value's origin. The true defect is **caller-side**: the invocation that produced a record is not reproducible from the record, because the command line does not contain the input. An operator re-running "the same command" in a different shell gets a different record, and `set -euo pipefail` will not notice. |
| **N-H — a different instrument** (`tools/bench_delta.sh`) | Register the Staged quantity from the delta bench. | **ESTIMATED** comparable to N-E. | **Rejected. The void reproduces on the corrected instrument** (fact 3, now over all of `tools/`): no `tools/` artefact takes `--config`, so this relocates the change rather than avoiding it — to a script whose output is a RATIO, not the per-position record the registered quantity is made of. |
| **N-D′ — take no Staged snapshot** (the null row) | Report the radius numbers only. | Zero. | The WP's depth claim goes unmeasured, and D-310 already shrank this WP's SPRT delta by deferring stage Q. Rejected, and stated so the rejection is checkable. |

## Recommendation

**N-E — required `--config PATH`, no default.**

Grounds, in the order they bind. **Revision 1's ground 4 is deleted, not
repaired** (F1): a cost ground whose one number was false toward its own
recommendation has forfeited its place, and N-E must win without it.

1. **Hard rule 1 reaches here, and that is not this matrix's inference** (fact
   10, F13). Rule 1: *"no code-side default for any tunable — a default lives in
   exactly one schema place"*, with the stated reason that *"an absent budget is
   an error, never a fallback"*. `crates/pistol-cli/src/bin/pistol.rs:34` applies
   it to a command-line config path in the shipped engine — *"Always explicit:
   there is no default path"* — and the arena binary does the same. The round-1
   red team attacked this transfer and REJECTED its own attack. An absent config
   in a two-record comparison is an error, and a fallback is how it becomes a
   wrong answer.
2. **The failure mode a default carries is the one class this WP keeps
   producing.** A record silently taken at the wrong config is an
   EXIT-0-WRONG-ANSWER, which `tools/SHELL_CHECKLIST.md` names three consecutive
   rounds of. Fact 4 makes it detectable *after the fact*; it does not make it
   *refused*. N-E refuses it. D-283 reached the same conclusion in different
   words: *"an optional binding is a binding nobody has"*.
3. **The precedent slate is not blank, and read whole it points here** (F8, fact
   12). There are two precedents, not one. Where a per-run input had **no flag
   surface**, D-252's matrix chose the document key and D-283 landed it — and
   D-283 states that condition explicitly. Where a config path is named on a
   **command line**, this tree ships it required with no default, twice. The
   snapshot has six flag arms and already spells the engine's flag `--config`
   (fact 9), so it is in the second case, not the first. **This is the comparison
   revision 1 never made and it is why N-J is entered as a genuine rival rather
   than dismissed.**
4. **N-J's strictly stronger property is real and is not free.** N-J alone makes
   the config binding checkable by digest before the run. What it costs is a
   document that exists to bind a document, on an instrument that already has the
   argument surface — and D-283's own reasoning does not transfer to an instrument
   that has one. If a later WP needs the pre-run digest check, N-J is the shape,
   and flip clause 3 reaches it.

**ON THE ROUND-1 DISCLOSURE (F7), because it is the finding I most want to be
wrong about.** Revision 1 disclosed that it had re-measured a cost cell in the
direction favouring its own recommendation, and directed the red team to check
"that no fourth caller exists outside `tools/` and `crates/`". The defect was
INSIDE `crates/`, in the very file the disclosure named. **A disclosure that
points away from where its own error is does the work of concealment whatever its
intent, and it buys credit for candour on the way.** Revision 2 states its
corrected audit as a number it did NOT measure — fact 6d is cited to the red
team's mutation run, not re-derived here — precisely so that the number
differentiating the recommended option came from an instrument that was trying to
break it.

## What flips it

No clause below names a remedy another part of this matrix forbids (F2).

- **The caller audit is materially larger than five sites.** Trigger: an audit at
  IMPL finds callers beyond the five of fact 6d — a runbook, a manifest, a docs
  command block. Remedy: **flip to N-L**, which changes no caller at all because
  it changes no interface. Reachable: N-L is the current shape plus two ADR lines,
  and nothing of N-E need be built first. *This clause does NOT name N-A′*, which
  ground 1 and fact 10 exclude; revision 1's clause did, and that was F2.
- **The guard count is not four.** Trigger: implementation shows the guards are
  more or fewer. Remedy: **this IS a selection trigger and the direction is
  stated** (F6) — every guard added moves N-A′, N-E and N-J and does not move
  N-F, N-L or N-B′ at all, because those let no caller name a path. If the guards
  grow past roughly double, **flip to N-L**. Revision 1 declared this common;
  it is not.
- **A pre-run digest check on the config becomes required.** Trigger: a
  pre-registration or an ADR requires that a record's config be verified by
  content before the engine launches, as D-283 requires of the arena's binaries.
  Remedy: **flip to N-J.** Reachable, and N-E is not wasted: the required
  argument becomes the document path and the four guards move to the document.
- **Rule 1 is ruled not to reach a `tools/` script.** Trigger: the architect
  rules hard rule 1 governs engine config loading only. Remedy: **ground 1 falls;
  re-decide between N-E, N-L and N-A′ on grounds 2 and 3**, which do not depend on
  rule 1. **This is the only clause under which N-A′ becomes live again**, and it
  is named because ground 1 does the most work and fact 10 is evidence about
  practice, not about the rule's text.
- **The BEFORE/AFTER confound is judged unacceptable.** Trigger: a reviewer holds
  that two records taken by two invocations cannot be compared at the registered
  quantity's precision. Remedy: **flip to N-K**, the only option that makes one
  invocation produce both. Reachable and it subsumes N-E's argument surface.

## COST OF THE DECISION THIS MATRIX FEEDS

Selecting costs one further DECISION-RED-TEAM dispatch against THIS revision.
IMPLEMENTING N-E costs: one `tools/` commit with a SHELL_CHECKLIST review
answered item by item; **ESTIMATED 5–8 new tests** driving the shipped script plus
one refusal test; the five-site caller audit of fact 6d; and **two snapshot runs
at MEASURED 34.5 s each**, replicated three times per the design's amendment 5 —
**MEASURED 34.5 s × 6 ≈ 3.5 min of machine time**, plus the release build. The
34.5 s is cited to the design at `6feb40a`. **No number in revision 2 was
produced by a run this session took**; every MEASURED value is either a
structural fact of the tree with its command beside it, or is cited to the
session or the report that measured it.

## OPEN — escalated to the architect, NOT repaired here

**F11: a downstream document already treats this selection as made.**
`docs/experiments/wp15b_sprt_prereg.md` asserts *"MATRIX M4 **ADOPTS** adding
`--config`"* at the same commit at which U4-Z says *"B2 / M4 — no ADR line, and
the selection is OPEN"*. This session is barred from editing the
pre-registration, so the contradiction is recorded and not resolved. It is not
merely tidiness: the pre-registration is the document that will govern a run, and
it currently rests on a selection no matrix has ever supported. **Whichever
option the architect selects, that sentence must be re-read against it before the
pre-registration's review** — and if N-E is selected it happens to name the right
mechanism for the wrong reason, which is the harder case to notice later.

---
---

# REVISION 1 — VERBATIM AS ATTACKED at `77f7397`

*Everything below this line is the text the round-1 DECISION-RED-TEAM was
dispatched against, unedited. Its "Status: AUTHORED, NOT SELECTED" is the state
at attack time. Nothing in it is corrected here: the corrections are above, and a
matrix edited after its attack is a matrix that was never attacked.*

> # MATRIX M4 — the snapshot's config seam
> Status: **AUTHORED, NOT SELECTED.** Awaits fresh-context DECISION-RED-TEAM.
> Subject: `tools/baseline_snapshot.sh` at `9421d19`. Owning unit:
> `docs/experiments/U4_soundness_instrument.md` §9, u-rev 2.
>
> ## WHY THIS IS A FRESH MATRIX AND NOT THE `ec8f7fb` RECOVERY
>
> The `ec8f7fb` matrix is recovered verbatim at the head of U4 §9 and is **history,
> not a candidate.** T1' says identical = attack stands, differs = fresh round, and
> U4 §9's DIFF 2 measured it DIFFERS on four cells, three of them MEASURED
> falsifications. **The subject itself moved**, which is the difference that makes
> recovery impossible rather than merely stale:
>
> | What moved | From | To | Mark |
> |---|---|---|---|
> | The registered quantity | `timing depth_at_500ms`, 32 lines BELOW the `# timing` marker whose own emitted text reads *excluded from every comparison* | per-position `depth_turns` and `nodes` at the registered 50 000-node budget, plus the `ladder … nodes` counts — ABOVE the marker | **MEASURED** |
> | The BEFORE cost | 34.0 s | **34.5 s**, re-taken under the amended script because N-A *is* a change to the instrument | **MEASURED** (by the design session at `6feb40a`; not re-taken here — see COST below) |
> | N-A's mitigation | "the flag is the fourth of its exact kind"; "the `argument` helper already refuses an empty value" | WITHDRAWN. Four guards are owed and the helper is none of them | **MEASURED** |
> | N-B's rejection ground | "breaks the D-209 instrument golden transcripts" | FALSE — `grep -c instrument_v0` on that fixture is 0 | **MEASURED** |
>
> A matrix whose subject is a different quantity, measured at a different cost,
> with its recommended option's mitigation withdrawn and one rival's rejection
> ground void, is not the same matrix. The options below are authored from the
> subject as it stands at `9421d19`.
>
> ## FACTS THE MATRIX STANDS ON — every one MEASURED at `9421d19`, with its command
>
> | # | Fact | Command |
> |---|---|---|
> | 1 | `CONFIG="configs/instrument_v0.toml"` is a **literal at line 170** with no flag, read at **five** further sites (271, 321, 425, 464, 531) and emitted at 440 | `grep -n CONFIG tools/baseline_snapshot.sh` |
> | 2 | The script has **six** flag arms: `--out`, `--nodes`, `--corpus`, `--ladder-depth`, `--ladder-cap-s`, `--binary` | `grep -cE '^\t--' tools/baseline_snapshot.sh` → 6 |
> | 3 | **NO `tools/` script takes `--config`.** It would be the FIRST config-scope flag in the whole tree, not the fourth flag of an existing kind | `grep -ln 'argument --config\|--config)' tools/*.sh` → no output |
> | 4 | `config <path> <sha>` is written into `$INVARIANT` (line 440); the marker is echoed at line 633; so the config provenance is **ABOVE** the marker already | read lines 435–445 and 628–640 |
> | 5 | Per-position `depth_turns`/`nodes` → `$INVARIANT` (494); `timing position … time_ms … nps` → `$TIMING` (496); `timing depth_at_500ms` → `$TIMING` (605). **The moved-to quantity is above the marker and the demoted one below, in the shipped script and not only in the document** | read lines 490–498, 600–610 |
> | 6 | The shipped script is driven by **29** tests in `crates/pistol-cli/tests/baseline_snapshot_tests.rs` (1234 lines) — the SHELL_CHECKLIST coverage rule is already satisfied for the script as it stands | `grep -c '^#\[test\]' crates/pistol-cli/tests/baseline_snapshot_tests.rs` |
> | 6b | **All 29 tests invoke the script through ONE helper**, `go()` at lines 170–201, which assembles `--corpus`, `--ladder-depth`, `--binary` and the optional flags in a single place. There are not 29 invocation sites; there is one | read lines 165–201 of that file |
> | 6c | `tools/ci.sh` does **NOT** invoke the snapshot | `grep -n baseline_snapshot tools/ci.sh` → no output |
> | 7 | **NONE of the four staged configs exists on disk.** `configs/` holds 12 files and no `*staged*` | `ls configs/` |
> | 8 | `configs/instrument_r2_v0.toml` exists and is referenced by **no** script and **no** crate — there is NO precedent in this tree for pointing an instrument at a second committed config | `grep -rn instrument_r2_v0 tools/ crates/` → no output |
> | 9 | The script already passes `--config "$CONFIG"` **to the engine** at three call sites (425, 464, 531) — the engine's own flag is spelled the same | `grep -n 'BINARY" --config' tools/baseline_snapshot.sh` |
>
> **The four guards owed by any option that lets a caller name the path** —
> MEASURED enumeration, carried from the design's amendment 2 and re-read against
> the shipped script at this revision: (i) caller-relative resolution, as `--out`
> (line 242 ff.) and `--binary` each got; (ii) the printable allow-list extended to
> the whole `$CONFIG` path, because unlike `--corpus` it reaches the record as a
> WHOLE PATH on two invariant lines rather than through `$(basename …)`;
> (iii) three named refusals — directory, missing, not a regular file — against
> line 271's bare `[ -f ]`; (iv) an assertion that the script's `config` line and
> the engine's `engine_id config` line name the same document.
>
> ## WHAT THE OPTIONS ARE OPTIONS ABOUT
>
> By what seam does the snapshot instrument produce a record taken under a STAGED
> config, given that the number it must produce now lives above the marker, that
> the staged configs do not yet exist, and that the instrument's own revision is
> named in whatever pre-registration consumes the record?
>
> ## Options
>
> | Option | What it does | Cost | Failure modes |
> |---|---|---|---|
> | **N-A′ — OPTIONAL `--config PATH`, default unchanged** | A seventh flag arm; `CONFIG` keeps `configs/instrument_v0.toml` as its default. The recovered N-A, re-costed at the moved subject. | A `tools/` change: SHELL_CHECKLIST answered item by item; **four** guards owed (MEASURED, above); **ESTIMATED 5–8 new tests** driving the shipped script, one per guard plus a same-document assertion and a default-unchanged test, against the 29 that exist. BEFORE re-taken: **MEASURED 34.5 s** one run. | **The default is the failure mode.** A caller who omits the flag gets a record that looks like every other record and silently names the radius config — and the WP's whole point is comparing two records. Mitigated only by fact 4: the path is already on an invariant line, so two records under two configs cannot be diffed as one. It reopens a script whose review round closed recently. It is the FIRST config-scope flag in `tools/` (fact 3), so it sets the precedent every later script inherits, and it does so with a default. |
> | **N-E — REQUIRED `--config PATH`, NO default** | Same arm, but `CONFIG` starts unset and an invocation without it is refused by name. | Everything N-A′ costs, plus the caller audit — **MEASURED, and it is small: THREE sites.** The default literal is at exactly one site (fact 1); all 29 tests invoke through the single `go()` helper (fact 6b), so they are one line; `tools/ci.sh` does not invoke the script at all (fact 6c); the third site is the script's own usage block (line 109 ff.). Plus **ESTIMATED 1 further test** that the refusal fires by name. | Breaks callers loudly rather than quietly, which is the direction rule 3 asks for, but it is a breaking change to a recently reviewed instrument for a WP that has not started IMPL. If any historical record was taken by an invocation reproduced somewhere as a literal command, the comparison across that boundary needs the old spelling documented, not merely the new one enforced. |
> | **N-F — a second committed script**, `tools/baseline_snapshot_staged.sh` | No flag surface at all; a second script whose `CONFIG` literal is the staged document. | **MEASURED 646 lines** duplicated, or a shared body extracted and both scripts rewritten around it. Either way the 29-test suite must be parameterised or duplicated. **ESTIMATED** the extraction is the larger `tools/` change of any option here. | Two instruments that must stay in step, and rule 9's single-responsibility cap bites on a 646-line body duplicated. Worse for THIS subject: the two records must be COMPARED, and two scripts diverging by one line each revision is exactly how a comparison stops comparing. The `# timing` marker discipline would have to hold identically in both. |
> | **N-G — an environment variable**, `PISTOL_SNAPSHOT_CONFIG` | `CONFIG="${PISTOL_SNAPSHOT_CONFIG:-configs/instrument_v0.toml}"`. One line. | **MEASURED one line changed**; **ESTIMATED 2–3 tests**. Cheapest diff of any option. | **Rejected on the checklist's own named class.** The config then comes from ambient state that never appears in the caller's command line, so a record's provenance depends on an environment the record cannot attest — and `set -euo pipefail` will not notice, which is EXIT-0-WRONG-ANSWER by construction. It also defeats fact 4's protection in the one case that matters: an operator re-running "the same command" in a different shell gets a different record with no signal. |
> | **N-H — take the Staged number from a different instrument** (`tools/bench_delta.sh`) | Leave the snapshot alone; register the Staged quantity from the delta bench. | **ESTIMATED** comparable to N-A′, because it relocates the same change. | **MEASURED void:** `tools/bench_delta.sh` has no config flag either (fact 3 covers all of `tools/`), so this does not avoid a `tools/` change, it moves it to a script whose output is a RATIO and not the per-position record the registered quantity is made of. Rejected. |
> | **N-D′ — take no Staged snapshot** (the null row, recorded because the restructure red team's F11 found a matrix missing one) | Report the radius numbers; the Staged depth evidence is not taken. | Zero. | The WP's depth claim goes unmeasured, and D-310 already shrank this WP's SPRT delta by deferring stage Q. Rejected — but recorded, because a null row a matrix never states is a rejection nobody can check. |
>
> ## Recommendation
>
> **N-E — required `--config PATH`, no default.**
>
> Grounds, in the order they bind:
>
> 1. **Hard rule 1 is the closest binding text and it points one way.** "No
>    code-side default for any tunable — a default lives in exactly one schema
>    place." `CONFIG` is a tunable and line 170 is a code-side default. N-A′ keeps
>    it and adds a flag beside it; N-E removes it. The rule was written for
>    `Budget`, and its stated reason — "an absent budget is an error, never a
>    fallback" — transfers exactly: an absent config in a two-config comparison is
>    an error, and a fallback is how it becomes a wrong answer.
> 2. **The failure mode N-A′ carries is the one this WP keeps producing.** A record
>    silently taken at the wrong config is an EXIT-0-WRONG-ANSWER, the single class
>    `tools/SHELL_CHECKLIST.md` names three consecutive rounds of. Fact 4 makes it
>    *detectable after the fact* by a reader who compares the two `config` lines; it
>    does not make it *refused*. N-E refuses it.
> 3. **The precedent cost is asymmetric and it is paid once.** Fact 3: this is the
>    first config-scope flag anywhere in `tools/`. Whatever shape it takes is what
>    every later script copies. A required flag copied forward costs each later
>    script one refusal; an optional flag with a default copied forward costs each
>    later script a silent wrong-config path.
> 4. **The extra cost over N-A′ is bounded, and it was MEASURED rather than
>    estimated.** Facts 1, 6b and 6c: three caller sites, one of them a usage
>    comment. Fact 7: no staged config exists yet, so there is no historical staged
>    record whose invocation spelling would be broken — the breaking window is at
>    its narrowest it will ever be.
>
> **A DISCLOSURE ABOUT GROUND 4, because the correction ran toward the
> recommendation.** This cell first read *"ESTIMATED 29 test invocations to audit"*.
> Measuring it — one `sed -n '165,201p'` on the test file — collapsed it to three
> sites and made the recommended option cheaper than the matrix had costed it.
> D-291's clause is that an estimate measurable in seconds is a finding, and this
> one was; it is recorded here rather than silently replaced, because a matrix that
> re-measures a cell only when the estimate hurts its recommendation is doing
> something other than measuring. **The red team should check both that the three
> sites are three and that no fourth caller exists outside `tools/` and
> `crates/`.**
>
> **What the recommendation does NOT claim.** It does not claim N-E is free. It is
> strictly more expensive than N-A′ — by three edits and one test — and that
> difference is small enough that grounds 1–3, which are about failure modes and
> precedent rather than cost, are what the selection actually rests on. A red team
> that kills grounds 1–3 kills the recommendation regardless of ground 4.
>
> ## What flips it
>
> Each clause names a remedy the trigger can reach — the incoherence F5 found in
> the restructure matrix, where the trigger was about units 1 and 4 and the remedy
> deferred unit 3.
>
> - **The caller audit is larger than MEASURED.** Trigger: a caller outside
>   `tools/` and `crates/` — an operator runbook, a manifest, a docs command block —
>   reproduces a snapshot invocation as a literal command, taking the three sites
>   above ten. Remedy: **flip to N-A′** — the same flag, a default retained, and the
>   audit does not happen. Reachable because N-A′ and N-E differ in exactly the
>   default and nothing else; the four guards, the tests for them and the
>   SHELL_CHECKLIST answers are shared and are not re-done.
> - **A historical record exists that a required flag would orphan.** Trigger: any
>   committed or manifest-indexed snapshot record whose invocation is reproduced
>   anywhere as a literal command without a config. Remedy: **flip to N-A′.** Same
>   reachability. (MEASURED at this revision: fact 7 says no staged record can
>   exist yet; this clause is about the radius records.)
> - **The four guards turn out not to be four.** Trigger: implementing the guards
>   shows one is already discharged by an existing line, or a fifth is needed.
>   Remedy: this does not move the selection between N-A′ and N-E, since both owe
>   the same guards — it moves the COST cells of both by the same amount, and the
>   matrix records that it is not a selection trigger. **Stated so it is not
>   mistaken for one.**
> - **Rule 1 is read as not reaching a `tools/` script.** Trigger: the architect
>   rules that hard rule 1's "no code-side default" governs engine config loading
>   only and not shell instruments. Remedy: **ground 1 falls, and with it the
>   recommendation — re-decide between N-A′ and N-E on grounds 2 and 3 alone**,
>   which do not depend on rule 1. This is reachable because grounds 2 and 3 are
>   stated independently; it is named because ground 1 is doing the most work and
>   its scope is the matrix's most arguable premise.
>
> ## COST OF THE DECISION THIS MATRIX FEEDS
>
> Stated so the proportion is visible on the matrix's own face. Selecting costs one
> DECISION-RED-TEAM dispatch. **IMPLEMENTING** the selected option costs, MEASURED
> where the number exists and ESTIMATED where it does not: one `tools/` commit with
> a SHELL_CHECKLIST review answered item by item; ESTIMATED 5–8 new tests driving
> the shipped script (N-A′) or those plus the caller audit (N-E); and **two
> snapshot runs at MEASURED 34.5 s each** — a BEFORE re-taken under the amended
> script and the Staged AFTER — replicated three times per the design's amendment
> 5, so **MEASURED 34.5 s × 6 ≈ 3.5 min of machine time**, plus the release build.
> **No number in this matrix was re-measured by a run costing more than a second**;
> the 34.5 s is cited from the design at `6feb40a`, with its provenance, and is not
> this matrix's own measurement.
>
> ---
>
> *Matrix M4, authored fresh at `9421d19`. Not selected. Awaits
> DECISION-RED-TEAM.*

*Matrix M4 revision 2, authored at `dab170b`. Revision 1 above is verbatim at `77f7397`. Not selected. Awaits a second DECISION-RED-TEAM.*
