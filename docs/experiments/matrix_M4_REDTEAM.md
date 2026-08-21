# MATRIX M4 — DECISION-RED-TEAM report

**Pinned revision: `77f7397`** (`docs/experiments/matrix_M4_snapshot_config_seam.md`,
owning unit `docs/experiments/U4_soundness_instrument.md` §9 u-rev 2).

**Does it still match HEAD?** It did at dispatch. **HEAD advanced twice during this
round** — to `1b645ac`, then to `dab170b` — so `77f7397` is no longer HEAD. `77f7397`
remains an ancestor, and of everything this report attacks only
`docs/experiments/U4_soundness_instrument.md` changed at all: it bumped to **u-rev 3**,
rebuilding §8.4's M4 and M6 *mutation* witnesses (a name collision with matrix M4 — those
are mutations, not options) and closing MAJOR 8's reachability half. **§9, the block this
matrix is owned by, is byte-identical**, and U4-Z still carries *"B2 / M4 — no ADR line,
and the selection is OPEN"*. MEASURED — the diff's hunks are at lines 15, 379, 381,
773–787 and 797–800, while §9 spans 435–580:

```
$ git diff -U0 77f7397 HEAD -- docs/experiments/U4_soundness_instrument.md | grep '^@@'
@@ -15 +15 @@
@@ -379 +379 @@
@@ -381 +381 @@
@@ -773,12 +773,15 @@
@@ -797 +800 @@
```

One housekeeping consequence, not a finding against the matrix: the matrix's subject line
cites *"`U4_soundness_instrument.md` §9, u-rev 2"*, and under D-311 that label no longer
names a text in the tree. The §9 bytes are unchanged, so this report's target is
unambiguous, but the citation will need re-pointing at u-rev 3 when the ADR line is
written.

The earlier verification, at `1b645ac`, over every other file this report attacks — all
still byte-identical at `dab170b`:

```
$ git merge-base --is-ancestor 77f7397 HEAD && echo "77f7397 IS an ancestor of HEAD"
77f7397 IS an ancestor of HEAD
$ git diff --stat 77f7397 HEAD -- docs/experiments/matrix_M4_snapshot_config_seam.md \
    tools/baseline_snapshot.sh crates/pistol-cli/tests/baseline_snapshot_tests.rs \
    docs/experiments/section_owner_table.md docs/experiments/U4_soundness_instrument.md \
    docs/experiments/wp15b_sprt_prereg.md docs/decisions.md
(no output)
```

Every file this report attacks is **byte-identical at `77f7397` and at `1b645ac`**, so
every finding below holds at HEAD.

**Context was fresh.** I did not author the matrix, the design it belongs to, the
`ec8f7fb` original, or any review behind them. I read `CLAUDE.md` first and hold the
matrix to its Process section's option-matrix clauses.

**Every numeric claim I make below is marked MEASURED or ESTIMATED.** Mutation testing
ran in a separate `git worktree` on `/home` (never the live tree), which was removed
before this report was written; the live tree was clean at start and I modified no file
but this one.

---

## 1. VERDICT

**The matrix does not survive as a matrix, and the recommendation does not survive as
argued — but the recommended option itself survives on repaired grounds.** Three
independent kills land. First, the single MEASURED number that differentiates N-E from
its nearest rival is **false**: fact 6b's "all 29 tests invoke the script through ONE
helper… there are not 29 invocation sites; there is one" is contradicted by two further
`Command::new("bash")` sites in the same file the matrix says it measured, and a
mutation worktree shows that patching only the site the matrix names leaves a test
failing. That the matrix's own DISCLOSURE certifies this number, and points the red team
*outside* `crates/` where the defect is *inside* `crates/`, converts an integrity gesture
into misdirection in effect. Second, flip clauses 1 and 2 name **N-A′ as the remedy**
while ground 1 declares N-A′ a breach of hard rule 1 — the trigger is a cost overrun and
the remedy is a rule violation, which is F5's incoherence reproduced in the document that
opens its flip section by claiming to have avoided it. Third, the option space is
incomplete in the way F11 named: the **document/manifest seam** is missing, and it is not
a novel option — `D-252`'s option matrix chose it for this exact class after a red team
and `D-283` landed it, reasoning in almost the matrix's own words that "a required
document key cannot be forgotten the way a flag can". A matrix that argues a precedent
cost while omitting the precedent that already exists is arguing from a slate it has not
checked. Against that, my sharpest planned attack **failed**: ground 1's transfer of
rule 1 from engine config to a shell default is *defensible*, and better supported than
the matrix knew — `crates/pistol-cli/src/bin/pistol.rs:34` and
`crates/pistol-arena/src/bin/arena.rs:32` both implement "there is no default path",
the first citing CLAUDE.md rule 1 by name. I do **not** conclude that every option fails.
N-E survives wounded and is the strongest survivor; but it has never been compared
against the option the tree already selected once for this class, and that comparison has
to happen before a selection is honest.

---

## 2. PER-OPTION SURVIVAL

| Option | Survival | The one attack that did it |
|---|---|---|
| **N-A′** — optional `--config`, default kept | **FALLS** | Rule 1's reach to a command-line config path is not the matrix's inference — it is shipped twice in this tree (`pistol.rs:34`, `arena.rs:32`, the first citing rule 1 by name) and landed once as an ADR (`D-283`: "an optional binding is a binding nobody has… 'absent means unchecked' is that default wearing a different word"). N-A′ is the shape three in-tree authorities reject. **Recoverable only if the architect fires flip clause 4** and rules rule 1 `tools/`-exempt. |
| **N-E** — required `--config`, no default | **SURVIVES WOUNDED** | Its differentiating cost cell is a false MEASURED number (F1): the caller audit is not three sites. Its substance is untouched and its shape matches both in-tree `--config` implementations — but it has never been compared against the document seam (F3). |
| **N-F** — a second committed script | **SURVIVES WOUNDED** | Not killed on substance; its "two instruments drifting apart" failure mode is sound and unrebutted. Wounded because it was costed against an unmeasured comparative ("ESTIMATED the extraction is the larger `tools/` change of any option here") and because flip clause 3 wrongly exempts the guard count from the selection (F6) — the four guards are the **entire** cost differential between the flag options and N-F, which owes none of them. |
| **N-G** — `PISTOL_SNAPSHOT_CONFIG` env var | **SURVIVES AS A CORRECT REJECTION, ON A FALSE GROUND** | The stated ground — "a record's provenance depends on an environment the record cannot attest" — is falsified by the matrix's own fact 4 (F4). The record attests the resolved path *and* its sha256 above the marker whatever the value's origin. N-G is still rightly rejected, but on **caller-side reproducibility**, not on unattestability. This is the same defect (a rival rejected on a ground that measures false) the matrix convicts the `ec8f7fb` table of. |
| **N-H** — take the number from `tools/bench_delta.sh` | **SURVIVES AS A CORRECT REJECTION** | Its MEASURED void reproduces (`tools/bench_delta.sh:99` pins `CONFIG` with no flag). Minor wound only: its parenthetical "fact 3 covers all of `tools/`" is false as stated — fact 3's glob is `tools/*.sh` and `tools/wp15b_attribution_check.py` is not covered (F9). Measured, the conclusion holds anyway. |
| **N-D′** — the null row | **SURVIVES AS A CORRECTLY RECORDED REJECTION** | Properly stated and properly rejected; recording it answers F11 for the null case. Trivial wound: its "Zero" cost is an unmarked numeric (F12). |

---

## 3. THE STRONGEST SURVIVING ATTACK, PER SURVIVING OPTION

Quotable into an ADR line for whichever option is selected.

**Against N-E (the strongest survivor):**

> N-E was recommended on a caller-audit MEASURED at three sites, and the audit is not
> three: `crates/pistol-cli/tests/baseline_snapshot_tests.rs` invokes the shipped script
> at three places, not one, and a required `--config` patched only into the `go()` helper
> the matrix names still leaves
> `a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root` failing —
> so the one number differentiating N-E from N-A′ was false in the direction that
> favoured it, and was certified by the matrix's own disclosure. **N-E survives because
> its substance is independent of that number** — `pistol --config` and `arena --config`
> are both required with no default, the second landed by D-283's "an optional binding is
> a binding nobody has" — but it has never been compared against the seam D-252/D-283
> selected for this same class after a red team, namely binding the input in a DOCUMENT
> rather than a flag, and that option is absent from the matrix.

**Against N-F:**

> N-F was costed against an unmeasured comparative — "ESTIMATED the extraction is the
> larger `tools/` change of any option here" — with no command, in the direction that
> disfavours the recommendation's most structurally distinct rival; and the matrix's flip
> clause 3 declares the four-guard count "not a selection trigger" by narrowing the option
> set to N-A′ and N-E, when N-F owes **none** of the four guards and the guard count is
> therefore the whole flag-versus-no-flag cost differential.

**Against N-G:**

> N-G is rightly rejected but not for the reason given: the matrix's own fact 4 puts
> `config <path> <sha256>` above the `# timing` marker regardless of where the value came
> from, so the record CAN attest its provenance under an environment variable. The matrix
> grants fact 4 to N-A′ as a mitigation and withholds it from N-G — the rejection must be
> restated on caller-side reproducibility, which is the defect that actually survives.

**Against N-H and N-D′:**

> Both rejections stand. N-H's supporting fact 3 is instrumented by a glob (`tools/*.sh`)
> that does not cover all of `tools/`; re-measured over the whole directory the conclusion
> holds, so the defect is in the instrument's stated scope and not in the answer.

**Binding on any selection:**

> No option in this matrix has been compared against the DOCUMENT seam that D-252's
> option matrix selected and D-283 landed for the structurally identical question — how
> an instrument binds a per-run input — where an override flag was considered and
> explicitly not adopted because "a required document key cannot be forgotten the way a
> flag can". The precedent ground 3 argues is not blank: two binaries already implement
> required-no-default `--config`, and five `tools/` scripts pin a config literal with no
> flag at all.

---

## 4. FINDINGS

### F1 — KILL. Fact 6b and ground 4's "THREE sites" is a MEASURED number that does not reproduce.

**Claim attacked.** Fact 6b: *"**All 29 tests invoke the script through ONE helper**,
`go()` at lines 170–201… There are not 29 invocation sites; there is one."* And ground 4:
*"Facts 1, 6b and 6c: three caller sites, one of them a usage comment."* And N-E's cost
cell: *"the caller audit — **MEASURED, and it is small: THREE sites.**"*

**Contradicting evidence.** The test file contains **three** invocations of the shipped
script, not one.

```
$ grep -n 'Command::new\|baseline_snapshot.sh' crates/pistol-cli/tests/baseline_snapshot_tests.rs
170:    fn go(&self) -> Output {
171:        let script = self.root.join("tools/baseline_snapshot.sh");
172:        let mut command = Command::new("bash");
...
1036:    let ran = Command::new("bash")
1037:        .arg(repo("tools/baseline_snapshot.sh"))
...
1205:    let ran = Command::new("bash")
1206:        .arg(repo("tools/baseline_snapshot.sh"))
```

Lines 1036 and 1205 are inside
`an_empty_flag_value_is_refused_rather_than_silently_defaulted` and
`a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root`. Neither
passes `--config`; neither routes through `go()`.

**Reproducer — mutation worktree, with a control run.** A separate `git worktree` at
`77f7397` on `/home` (never the live tree). Minimal N-E: `CONFIG=""`, a `--config` arm,
and `[ -n "$CONFIG" ] || fail "--config is required and has no default"`.

*Control, unmutated tree:*

```
$ git -C $WT checkout -- . && git -C $WT status --porcelain   # clean at 77f7397
$ cargo test --release -p pistol-cli --test baseline_snapshot_tests
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.24s
```

*Mutation applied, no test-side change:*

```
test result: FAILED. 1 passed; 28 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

*Mutation applied, and `--config` patched into ONLY `go()` — the single site the matrix
names:*

```
---- a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root stdout ----
thread '...' panicked at crates/pistol-cli/tests/baseline_snapshot_tests.rs:1215:5:
the run must succeed:
stdout:
stderr: baseline_snapshot: FAIL: --config is required and has no default

test result: FAILED. 28 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.17s
```

**MEASURED.** Doing exactly what the matrix says suffices leaves the suite red. The audit
is at least **five** sites — the script's default literal (170), the usage block (109 ff.),
`go()`, test line 1036 and test line 1205 — and the 1036 site passes under the mutation
only by accident: it asserts a refusal that fires during argument parsing, *before* the
config check. Move the required-config check into the parse loop (a natural
implementation) and it breaks too.

**KILL.** This is the worst finding available under D-291's clause: a number marked
MEASURED that does not reproduce, in the cell that differentiates the recommended option
from its nearest rival, in the direction that favours the recommendation.

---

### F2 — KILL. Flip clauses 1 and 2 name a remedy their own ground 1 forbids. F5, reproduced.

**Claim attacked.** The flip section opens: *"Each clause names a remedy the trigger can
reach — the incoherence F5 found in the restructure matrix."* Clauses 1 and 2 then both
say: **"Remedy: flip to N-A′."**

**Contradicting evidence.** Ground 1 — the ground the matrix says "binds first" — holds
that N-A′ **violates hard rule 1**: *"`CONFIG` is a tunable and line 170 is a code-side
default. N-A′ keeps it and adds a flag beside it; N-E removes it."* Clauses 1 and 2 fire
on **cost and compatibility** triggers (the audit exceeds ten sites; a historical record
would be orphaned). Neither trigger touches rule 1. So as written, both clauses move the
selection to an option the matrix's own primary ground calls a hard-rule breach, purely
because the audit got expensive.

**A cost overrun cannot be a remedy for a rule violation.** The remedy is not reachable
from the trigger. That is precisely F5's shape — trigger about one thing, remedy about
another — in the document that claims in its own first flip-section sentence to have
avoided it.

**Reproducer.** Textual, and it is the document's own text read against itself:

```
$ sed -n '76,82p;122,133p' docs/experiments/matrix_M4_snapshot_config_seam.md
1. **Hard rule 1 is the closest binding text and it points one way.** "No
   code-side default for any tunable — a default lives in exactly one schema
   place." `CONFIG` is a tunable and line 170 is a code-side default. N-A′ keeps
   it and adds a flag beside it; N-E removes it. ...
- **The caller audit is larger than MEASURED.** Trigger: a caller outside
  `tools/` and `crates/` ... taking the three sites above ten. Remedy: **flip to N-A′** ...
- **A historical record exists that a required flag would orphan.** Trigger: any
  committed or manifest-indexed snapshot record ... Remedy: **flip to N-A′.** Same
  reachability.
```

**KILL.** Only flip clause 4 (rule 1 ruled `tools/`-exempt) can make N-A′ reachable, and
clauses 1 and 2 do not depend on clause 4 having fired. Either clauses 1 and 2 must be
conditioned on clause 4, or ground 1 must be downgraded from "binding" to "one
consideration among three" — and the matrix explicitly declines the second, saying grounds
1–3 "are what the selection actually rests on".

---

### F3 — KILL. The option space is incomplete, and the missing option is one this project already selected once for this class.

**Claim attacked.** The framing: *"By what seam does the snapshot instrument produce a
record taken under a STAGED config…"* — six options offered as the space.

**Contradicting evidence — four omissions, the first decisive.**

**(a) The DOCUMENT / manifest seam is missing, and it has an attacked precedent.**
`D-252` wrote an option matrix over the structurally identical question — how an
instrument binds a per-run input that would otherwise be a literal — offering (a) an
override flag, (b) a validity check, (c) a document key. The operator selected **(c)**,
and `D-283` landed it with reasoning that reads on M4's ground 2 almost verbatim:

```
$ grep -o "D-252's option (a), an override flag, is not adopted and is not needed: a required document key cannot be forgotten the way a flag can, which was that option's own recorded defect." docs/decisions.md
D-252's option (a), an override flag, is not adopted and is not needed: a required document key cannot be forgotten the way a flag can, which was that option's own recorded defect.
```

M4 offers only flag-shaped and script-shaped seams. It never states the document seam,
never rejects it, and never cites D-252 or D-283 — while ground 3 argues at length about
a precedent that D-252/D-283 already set, in the opposite direction, after a red team.

**(b) A config-PAIR / two-record mode is missing, and the framing sentence excludes it by
presupposition.** The matrix asks how to produce **a** record. But it twice names the
deliverable as a **comparison**: N-A′'s failure mode says *"the WP's whole point is
comparing two records"*, and N-F's says *"the two records must be COMPARED"*. A
single-invocation pair mode — two configs in, two records out, one instrument revision,
one binary digest, one machine, one schedule — makes ground 2's failure mode
*structurally impossible* rather than merely *refused*, and removes the cross-run variance
the `# timing` marker discipline exists to fence off. It is not obviously the best option;
it is obviously a viable one, and it is absent because the question was framed in the
singular. That is the framing presupposing an answer.

**(c) Re-pin the literal by ADR is missing, and it is the `tools/`-local precedent.**
MEASURED — five `tools/` scripts pin a config literal with no flag:

```
$ grep -n '^CONFIG=' tools/*.sh
tools/arena_smoke.sh:59:CONFIG="configs/arena_smoke_v0.toml"
tools/baseline_snapshot.sh:170:CONFIG="configs/instrument_v0.toml"
tools/bench_delta.sh:99:CONFIG="configs/instrument_v0.toml"
tools/movetime_check.sh:32:CONFIG="configs/play_v0.toml"
tools/determinism.sh:39:CONFIG="configs/gate_v0.toml"
```

Since CLAUDE.md's instrument clause already forces the pre-registration to name the
script **with its revision**, taking BEFORE at revision R1 and AFTER at R2 is fully
auditable — the record's own `config <path> <sha>` line says which document ran. The cost
(two records from two instrument revisions) is real, but it is *the same cost N-F carries*,
and N-F is stated while this is not.

**(d) N-B and N-C are dropped without re-adjudication, and the matrix banks the benefit of
dropping N-B without paying it.** The preamble lists, as one of four reasons the `ec8f7fb`
table cannot be recovered: *"N-B's rejection ground | 'breaks the D-209 instrument golden
transcripts' | FALSE… | MEASURED"*. When a rival's rejection ground is measured false, the
honest move is to re-state that rival with its surviving grounds. Instead N-B vanishes from
the option set entirely. The matrix uses the falsification to void the old table and then
declines to re-try the rival the falsification helped. A reader of the matrix alone sees
N-B only as "rejection ground void, no row" — F11's defect exactly.

**KILL.** F11 established that a missing row is a finding even when it would not have been
selected. Here four rows are missing, one of them the shape this project already chose for
this class after a DECISION-RED-TEAM.

---

### F4 — MAJOR (KILL against the stated ground). N-G is rejected on a ground the matrix's own fact 4 falsifies.

**Claim attacked.** N-G's failure mode: *"The config then comes from ambient state that
never appears in the caller's command line, so **a record's provenance depends on an
environment the record cannot attest** — and `set -euo pipefail` will not notice, which is
EXIT-0-WRONG-ANSWER by construction."*

**Contradicting evidence.** The record **can** attest it, and the matrix says so two rows
earlier. Fact 4: *"`config <path> <sha>` is written into `$INVARIANT` (line 440)… so the
config provenance is **ABOVE** the marker already."* MEASURED:

```
$ sed -n '436,441p' tools/baseline_snapshot.sh
{
	echo "schema $SCHEMA"
	echo "revision $REVISION"
	echo "binary_sha256 $BINARY_SHA256"
	echo "config $CONFIG $CONFIG_SHA256"
	sed 's/^id /engine_id /' "$WORK/id"
$ sed -n '321p' tools/baseline_snapshot.sh
digest "$CONFIG" "the config"; CONFIG_SHA256="$DIGEST"
```

`$CONFIG` is digested at 321 and both the resolved path and its sha256 land above the
marker at 440 — **whatever the value's origin**. Under N-G the record states exactly which
document ran, and the sha discriminates even if the path spelling does not. So the
provenance is attested and this is *not* EXIT-0-WRONG-ANSWER "by construction": a reader
comparing two records sees the difference, which is precisely the property the matrix
grants N-A′ as its mitigation.

**The inconsistency is directional.** Fact 4 is applied as a *mitigation* to the
recommendation's nearest rival (N-A′: *"Mitigated only by fact 4"*) and withheld from N-G,
whose rejection depends on fact 4 being false.

**What actually survives against N-G** is narrower and the matrix states it in its last
sentence: *"an operator re-running 'the same command' in a different shell gets a different
record with no signal."* That is a **caller-side reproducibility** defect — the command
line is not a complete description of the run — and it is a sound reason to reject N-G. It
is not the reason given.

**Not a kill against N-G's disposition** (it stays rejected), but a kill against the stated
ground, and it is the same species of defect the matrix convicts the `ec8f7fb` table of in
its own preamble.

---

### F5 — MAJOR. Fact 7 breaches the carve's own gate and marks MEASURED a number its command cannot produce.

**Claim attacked.** Fact 7: *"**NONE of the four staged configs exists on disk.** `configs/`
holds 12 files and no `*staged*`"*, under the Facts table's blanket header *"every one
MEASURED at `9421d19`, with its command"*, with command `ls configs/`.

**Contradicting evidence — two defects in one cell.**

**(a) The command cannot yield the number.** `ls configs/` establishes 12 files and no
`*staged*`. It cannot establish that there are **four** staged configs, because none of
them exists. The "four" is carried, unattributed, from another unit and marked MEASURED
under a command blind to it.

```
$ ls configs/ | wc -l
12
$ ls configs/ | grep -c staged
0
```

**(b) It restates a count the carve gives to another unit, and gives it exclusively.**
`docs/experiments/section_owner_table.md` §10 is explicit — the count of staged config
documents was B5's finding precisely because it was stated four different ways, and the
carve's remedy was one statement site:

```
$ grep -n 'THE ONE PLACE THE COUNT IS STATED' docs/experiments/section_owner_table.md
| §10 lead-in | 1349 | "**Three complete**, ... | **U3** §10 — **THE ONE PLACE THE COUNT IS STATED: FOUR** ...
$ sed -n '253,256p' docs/experiments/U3_tier_t.md
**FOUR** complete documents, `deny_unknown_fields`, no code-side default for
any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
and do not restate it** (B5, which found it stated three different ways across
four sites).
```

M4 is a U4 artefact. Fact 7 is a **fifth site** stating a count U3 owns exclusively, and it
neither cites U3 §10 nor derives the number. The owner table calls itself "the carve's own
gate" and calls a double-owned line "a FAILED carve, not a finding".

**The repair is one word:** state what `ls configs/` measures — that no staged config
exists on disk yet — and cite U3 §10 for the count if the count is needed at all. Note that
the *point* fact 7 makes (nothing staged exists yet, so N-E's breaking window is at its
narrowest) does not need the number.

---

### F6 — MAJOR. Flip clause 3's "this is not a selection trigger" smuggles a real trigger out of sight by narrowing the option set to two.

**Claim attacked.** *"**The four guards turn out not to be four.** … Remedy: this does not
move the selection between N-A′ and N-E, since both owe the same guards — it moves the COST
cells of both by the same amount, and the matrix records that it is not a selection trigger.
**Stated so it is not mistaken for one.**"*

**Contradicting evidence.** The claim is true *between N-A′ and N-E* and false *across the
option set*. There are six options. **N-F owes none of the four guards** — its defining
property, in the matrix's own words, is *"**No flag surface at all**"*. Guards (i)
caller-relative resolution, (ii) the printable allow-list over the whole `$CONFIG` path, and
(iii) three named refusals are all consequences of *letting a caller name the path*, which
N-F does not do; the matrix's own guard preamble scopes them exactly that way: *"The four
guards owed by **any option that lets a caller name the path**"*.

So the guard count **is** the flag-versus-no-flag cost differential in its entirety. If the
guards turn out to be eight rather than four, every flag option gets more expensive and N-F
does not move at all. The clause reaches its "not a selection trigger" conclusion only by
quietly restricting "the selection" to the two options that share the property.

**Reproducer.** The matrix's own text, read against its own option table:

```
$ sed -n '42,44p;65p' docs/experiments/matrix_M4_snapshot_config_seam.md
**The four guards owed by any option that lets a caller name the path** —
MEASURED enumeration, carried from the design's amendment 2 and re-read against
the shipped script at this revision: (i) caller-relative resolution, ...
| **N-F — a second committed script**, ... | **No flag surface at all**; a second script whose `CONFIG` literal is the staged document. | ...
```

**Aggravating.** This clause is the one the matrix flags most loudly as honest
("**Stated so it is not mistaken for one**"). A clause that announces its own candour while
narrowing the field to make its claim true is worse than a clause that says nothing.

---

### F7 — MAJOR. The disclosure is misdirection in effect, whatever its intent.

**Claim attacked.** *"**A DISCLOSURE ABOUT GROUND 4, because the correction ran toward the
recommendation.** This cell first read *'ESTIMATED 29 test invocations to audit'*. Measuring
it — one `sed -n '165,201p'` on the test file — collapsed it to three sites… it is recorded
here rather than silently replaced, because a matrix that re-measures a cell only when the
estimate hurts its recommendation is doing something other than measuring. **The red team
should check both that the three sites are three and that no fourth caller exists outside
`tools/` and `crates/`.**"*

**Contradicting evidence — three ways this fails as a safeguard.**

**(a) It certifies a false number.** Per F1, the three sites are not three.

**(b) The instrument it names is the defect.** `sed -n '165,201p'` reads exactly the range
that contains `go()` and nothing else. A range that stops at line 201 cannot see the
invocations at 1036 and 1205 in the same 1234-line file. This is the dry-run defect class
CLAUDE.md names: *"a command that counted the wrong symbols passed a synthetic dry run and
still shipped."* The correct instrument costs the same second:

```
$ grep -c 'Command::new("bash")' crates/pistol-cli/tests/baseline_snapshot_tests.rs
3
```

**(c) It points the red team away from the defect.** The disclosure directs attention to
"no fourth caller **outside** `tools/` and `crates/`". The fourth and fifth callers are
**inside** `crates/` — in the very file the disclosure says was measured. An estimate
replaced by a worse measurement, certified by a disclosure that steers the check away from
where it would have failed, is not integrity; it is a wrong number wearing integrity's
clothes. The original ESTIMATE ("29 test invocations to audit") was, in *kind*, closer to
the truth than its replacement.

**Directional audit of the remaining ESTIMATED cells, as the brief asks.** MEASURED by
reading the option table: the two cells that *differentiate* N-E from N-A′ both run in
N-E's favour — the audit ("MEASURED… THREE sites", false) and the extra test count
("ESTIMATED 1 further test"). The one cell that inflates the most structurally distinct
rival — N-F's "ESTIMATED the extraction is the larger `tools/` change of any option here" —
carries no command and no magnitude. Cells that favour rivals (N-G's "MEASURED one line
changed; ESTIMATED 2–3 tests… Cheapest diff of any option") are correctly and generously
marked, so this is not a global bias; it is a bias concentrated exactly on the
N-E-versus-alternatives margin.

---

### F8 — MAJOR. Ground 3's precedent cost is asserted, not measured, and its premise misdescribes the tree.

**Claim attacked.** *"**The precedent cost is asymmetric and it is paid once.** Fact 3:
this is the first config-scope flag anywhere in `tools/`. Whatever shape it takes is what
every later script copies. A required flag copied forward costs each later script one
refusal; an optional flag with a default copied forward costs each later script a silent
wrong-config path."*

**Contradicting evidence.**

**(a) It is unmarked and unmeasurable.** No number in ground 3 is marked MEASURED or
ESTIMATED, and the quantity it turns on — how many later `tools/` scripts will need a
config flag — is not measurable in principle. The clause CLAUDE.md enforces says *every*
numeric claim is marked; "costs each later script one refusal" is a cost claim over an
unstated count.

**(b) The base rate cuts against it.** MEASURED: 16 `tools/` scripts exist; five pin a
config literal (F3(c)); **zero** have ever needed a config flag in the project's history
(fact 3, which reproduces). A precedent whose population is five scripts that have all
managed without it for the project's whole life is a small stake, not an asymmetric one.

**(c) The slate is not blank — and the existing precedent is N-E's own shape.** The
matrix presents itself as *setting* the precedent. MEASURED, the shape is already set one
seam over, twice, and stated in the usage text:

```
$ sed -n '34,35p' crates/pistol-cli/src/bin/pistol.rs
  --config      an engine config. Always explicit: there is no default path and
                no built-in configuration (CLAUDE.md rule 1). Paths *inside* a
$ sed -n '32,33p' crates/pistol-arena/src/bin/arena.rs
  --config  an arena config. Always explicit: there is no default path and no
$ grep -n '"--config", config, "--out", out' crates/pistol-arena/src/bin/arena.rs
77:        ["--config", config, "--out", out] | ["--out", out, "--config", config] => {
80:        _ => return Err(format!("--config and --out are both required\n\n{USAGE}")),
```

This is good news for N-E's *substance* and bad news for ground 3's *framing*: the cost is
not "setting a precedent every later script inherits", it is "conforming a shell script to
a precedent two binaries already implement" — a much easier and much smaller argument,
which the matrix does not make because it did not look.

---

### F9 — MINOR. Fact 3's instrument does not cover the scope its conclusion claims.

**Claim attacked.** Fact 3: *"**NO `tools/` script takes `--config`.**"* Command:
`grep -ln 'argument --config\|--config)' tools/*.sh`. And N-H: *"fact 3 covers all of
`tools/`"*.

**Contradicting evidence.** The glob is `tools/*.sh`; `tools/` contains a non-`.sh`
executable script.

```
$ ls -1 tools/ | grep -v '\.sh$'
SHELL_CHECKLIST.md
wp15b_attribution_check.py
```

So the stated command cannot support "all of `tools/`". **Re-measured over the whole
directory, the conclusion holds** — the Python tool takes positional arguments only and no
`--config` of its own:

```
$ grep -n 'add_argument\|sys.argv' tools/wp15b_attribution_check.py
85:    if len(sys.argv) != 3:
87:    report, engine = sys.argv[1], sys.argv[2]
```

(Its `--config` occurrences at lines 168/178 are passed *to the engine*, like
`baseline_snapshot.sh`'s own three.) Defect is in the instrument's declared scope, not in
the answer — but it is the D-221/D-223 class the checklist's item 3 names ("a substring is
not a token… Anchor, or match the field"), applied to a glob instead of a pattern.

---

### F10 — MINOR, and it sharpens F1. A caller outside `tools/` and `crates/` already exists at the matrix's own revision.

**Claim attacked.** Flip clause 1's trigger: *"a caller outside `tools/` and `crates/` — an
operator runbook, a manifest, a docs command block — reproduces a snapshot invocation as a
literal command."* Presented as a hypothetical.

**Contradicting evidence.** It is not hypothetical; it is instantiated at `77f7397`:

```
$ git grep -nE '\S*tools/baseline_snapshot\.sh[^`]*--' -- docs/ tools/
docs/experiments/wp15b_sprt_prereg.md:362:`tools/baseline_snapshot.sh --config configs/gate_v0.toml` answers
tools/baseline_snapshot.sh:109:# Usage: tools/baseline_snapshot.sh [--out PATH] [--nodes N] [--corpus PATH]
```

**Not a KILL on its own**, because the clause's threshold is "taking the three sites above
ten" and six is not eleven. But the threshold was set against a false baseline (F1), and
the clause's rhetorical force — that no such caller is known — is wrong at its own revision.

---

### F11 — MAJOR. A downstream document at the same commit already treats the selection as made.

**Claim attacked.** The matrix's status line: *"**AUTHORED, NOT SELECTED.** Awaits
fresh-context DECISION-RED-TEAM."* And U4 §9's verdict: *"until it runs no ADR line may cite
N-A as adopted."*

**Contradicting evidence.** `docs/experiments/wp15b_sprt_prereg.md` — revision 4, in the
tree at `77f7397` — names the instrument as the snapshot *"AT THE REVISION THAT LANDS ITS
`--config` FLAG"* and states:

```
$ sed -n '359,366p' docs/experiments/wp15b_sprt_prereg.md
**THE INSTRUMENT: `tools/baseline_snapshot.sh` AT THE REVISION THAT LANDS ITS
`--config` FLAG** — **OPERATOR-CONFIRM** (§9.7). Revision 2 pinned `e889b5b`, which
**cannot measure the staged seat at all**: MEASURED at this document's revision,
`tools/baseline_snapshot.sh --config configs/gate_v0.toml` answers
`baseline_snapshot: FAIL: unknown argument --config`, the script hard-coding
`CONFIG="configs/instrument_v0.toml"` with no flag to move it. The design already knows — its §9 MATRIX M4
ADOPTS adding `--config`, and its amendment 4 re-takes the BEFORE run under the
amended script
```

**"its §9 MATRIX M4 ADOPTS adding `--config`."** The pre-registration asserts the
selection, by flag spelling, at the same commit at which the matrix says nothing is
selected. This forecloses N-F, N-G, N-H and N-D′ before the red team runs — the outcome is
written into a downstream document's registered instrument slot.

**Verified that the ADR log is clean** — no landed ADR adopts `--config`, so U4 §9's B2 is
genuinely open and the breach is in the pre-registration, not in `docs/decisions.md`:

```
$ grep -c 'M4' docs/decisions.md   # matches are D-252/D-283/D-309/D-310 on other subjects
```

(No ADR line adopts the flag; `grep -n -- '--config' docs/decisions.md` returns only
D-252 and D-283, both about the arena's binary binding.)

**This is the defect CLAUDE.md names as judged, not checked:** *"neither catches a run
whose answer is already known before it is taken."* Applied to a decision rather than a
run. It does not by itself invalidate any option, but the architect should know that
selecting anything other than N-A′/N-E requires amending `wp15b_sprt_prereg.md` §7A.2 and
reopening its review — and that the pre-registration's confidence is not evidence for the
matrix.

---

### F12 — MINOR. Unmarked numerics.

**Claim attacked.** *"EVERY NUMERIC CLAIM IN THE MATRIX IS MARKED **MEASURED** OR
**ESTIMATED**"* (CLAUDE.md, D-291, recorded as recurring for the third time by the prior
round's F1).

**Contradicting evidence.** Two cells carry bare numbers:

- N-D′'s cost cell: **"Zero."** Unmarked. (Analytically true rather than measured, but the
  clause admits no exemption, and the `ec8f7fb` table this matrix supersedes had the same
  unmarked "Zero" in three rows.)
- The recommendation's closing: *"It is strictly more expensive than N-A′ — **by three
  edits and one test**"*. Unmarked, derived from the false MEASURED cell, and false in
  consequence (F1).

Ground 3's cost comparison is also unmarked (F8(a)). Minor individually; noted because it
is the fourth consecutive round in which this clause is the finding.

---

### F13 — REJECTED, with the attempted reproducer. Ground 1's transfer of hard rule 1 to a shell script's default.

**What I attacked.** The brief's fifth surface: rule 1 sits under a heading about engine
config (`serde(deny_unknown_fields)`, "a default lives in exactly one schema place",
`Budget`), and the matrix applies it to a shell variable. My intended attack was that
rule 1's *remedy* clause is unreachable for a config **path** — there is no schema in which
a config path's default could live — so the matrix reads the prohibition and discards the
half that makes it a rule about schemas.

**Why it fails.** This tree already reads rule 1 the matrix's way, in shipped code, citing
the rule by name for exactly the no-default-config-path proposition:

```
$ sed -n '34,35p' crates/pistol-cli/src/bin/pistol.rs
  --config      an engine config. Always explicit: there is no default path and
                no built-in configuration (CLAUDE.md rule 1). Paths *inside* a
```

and a landed ADR extends rule 1 to a non-schema binding in almost the matrix's terms:

```
$ grep -o "WHY REQUIRED AND NOT OPTIONAL[^.]*\.[^.]*\." docs/decisions.md
WHY REQUIRED AND NOT OPTIONAL. An optional binding is a binding nobody has, and the
documents that would have gone without it are exactly the four this line exists for; CLAUDE.
```

(continuing: *"…md rule 1 also forbids a code-side default for a tunable, and 'absent means
unchecked' is that default wearing a different word."*)

**RECORDED AS REJECTED.** The transfer is defensible and better supported than the matrix
argued. The residual gap — `crates/` binary to `tools/` shell script — is narrow, and
flip clause 4's trigger is correspondingly less likely to fire than the matrix's framing
("ground 1 is doing the most work and its scope is the matrix's most arguable premise")
suggests. **The finding that survives here is a citation gap, not a reasoning gap:** the
matrix argues rule 1 by analogy from `Budget` while two shipped usage strings and one
landed ADR make the argument directly, and it cites none of them. Folded into F8(c).

---

## 5. WHAT I DID NOT FIND

Recorded so the architect knows the attack surface was covered and came back empty.

- **Facts 1, 2, 4, 5, 6, 6c, 8, 9 all reproduce exactly**, including every line number
  (170; 271/321/425/464/531 and 440; 440 and 633; 494/496/605; 29 tests / 1234 lines;
  no `ci.sh` invocation; no `instrument_r2_v0` reference; three engine call sites). The
  N-F cost cell's **MEASURED 646 lines** reproduces (`wc -l tools/baseline_snapshot.sh`
  → 646). Fact 3's conclusion reproduces despite F9's glob gap.
- **The subject revision is correctly pinned.** `git diff 9421d19 77f7397 --
  tools/baseline_snapshot.sh crates/pistol-cli/tests/baseline_snapshot_tests.rs configs/`
  is empty, so naming the subject at `9421d19` while the matrix commits at `77f7397` is
  accurate, and the instrument-revision clause is satisfied.
- **The COST section is honest.** `34.5 s × 6 ≈ 3.5 min` checks out (207 s), its
  provenance is disclosed as the design's at `6feb40a` rather than claimed as the matrix's
  own, and *"No number in this matrix was re-measured by a run costing more than a second"*
  is true of everything I re-ran — every Facts-table command returned sub-second.
- **N-H's rejection is sound**, and its "MEASURED void" reproduces.
- **The cost cells respect SHELL_CHECKLIST item 10** (the coverage rule): N-A′ and N-E both
  budget new tests driving the shipped script, N-F budgets parameterising or duplicating
  the suite, and N-G is rejected on the checklist's own named class. No option proposes a
  recorded number with no test behind it. (U4 §9.1 amendment 2's enumeration —
  *"items ENGAGED: 1, 3, 4, 8, 9, 10, 11, 12 — eight of twelve"* — is not carried into the
  matrix's cost cells, which say only "answered item by item"; noted, not a finding, since
  the owning unit holds it.)
- **The `ec8f7fb`-is-history claim holds.** All four preamble rows reproduce, including
  `grep -c instrument_v0` on the D-209 golden being 0 and the registered quantity sitting
  above the marker in the shipped script (fact 5). The matrix is right that recovery was
  impossible and right to author fresh.

---

*DECISION-RED-TEAM report on matrix M4, pinned `77f7397` (HEAD~2 at time of writing;
subject files byte-identical at HEAD `1b645ac`). Fresh context. Not the author. Nothing
selected here — selection is the architect's.*
