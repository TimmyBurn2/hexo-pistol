# MATRIX M4 — DECISION-RED-TEAM report, ROUND 2

**Pinned revision: `cb16f7c`** — `docs/experiments/matrix_M4_snapshot_config_seam.md`,
**REVISION 2** (the text above the fold). Owning unit
`docs/experiments/U4_soundness_instrument.md` §9.

**Does it still match HEAD?** It did at dispatch. **HEAD advanced during this round**,
to `a35503f` (matrix M3's red team and D-317). `cb16f7c` remains an ancestor, and
**every file this report attacks is byte-identical at `cb16f7c` and at HEAD** except
two documents that changed outside the blocks I rely on:

```
$ git merge-base --is-ancestor cb16f7c HEAD && echo "cb16f7c IS an ancestor of HEAD ($(git rev-parse --short HEAD))"
cb16f7c IS an ancestor of HEAD (a35503f)
$ git diff --stat cb16f7c HEAD -- docs/experiments/matrix_M4_snapshot_config_seam.md \
    docs/experiments/matrix_M4_REDTEAM.md docs/experiments/wp15b_sprt_prereg.md tools/ crates/ configs/
(no output)
$ git diff --stat cb16f7c HEAD -- docs/decisions.md docs/experiments/U4_soundness_instrument.md \
    docs/experiments/section_owner_table.md
 docs/decisions.md                           |  2 ++
 docs/experiments/U4_soundness_instrument.md | 43 +++++++++++++++++++++++-----
 docs/experiments/section_owner_table.md     | 41 +++++++++++++++++--------
```

The `docs/decisions.md` change is **D-317 appended** (M3 stopped with no survivor).
**D-252, D-283 and D-288 — the three ADRs every decisive finding below rests on — are
byte-identical**, and so is **U4 §9**, the owning block:

```
$ for d in D-252 D-283 D-288; do
    a=$(git show cb16f7c:docs/decisions.md | grep "^$d:" | sha256sum | cut -c1-16)
    b=$(grep "^$d:" docs/decisions.md | sha256sum | cut -c1-16)
    echo "$d  cb16f7c=$a  HEAD=$b"; done
D-252  cb16f7c=e5fa9bc474564229  HEAD=e5fa9bc474564229
D-283  cb16f7c=c8cb4b5b0e36eb12  HEAD=c8cb4b5b0e36eb12
D-288  cb16f7c=4c1bd7329b160e83  HEAD=4c1bd7329b160e83
$ diff <(git show cb16f7c:docs/experiments/U4_soundness_instrument.md | sed -n '435,581p') \
       <(sed -n '464,610p' docs/experiments/U4_soundness_instrument.md) && echo "§9 BODY BYTE-IDENTICAL"
§9 BODY BYTE-IDENTICAL
```

One housekeeping consequence, not a finding: the matrix's header pins the owning unit
at **u-rev 3**, and at HEAD U4 is at **u-rev 4** (§8's M3 block moved; §9's bytes did
not). The citation will need re-pointing when the ADR line is written — the second
time in two rounds this label has drifted under the matrix.

**I attacked REVISION 2 ONLY.** Revision 1 is treated as history. I verified that
revision 1's quoted body is genuinely unedited — MEASURED, all 165 lines identical to
`git show 77f7397:`:

```
$ python3 - <<'EOF'   # strip the "> " quote prefix from the fold and diff against the original
...
orig lines 165 quoted lines 165
IDENTICAL
EOF
```

**Context was fresh.** I did not author the matrix, revision 1, the design, or the
round-1 red team. I read `CLAUDE.md` first and hold this matrix to its Process
section's option-matrix, numeric-mark and criterion clauses.

**Every numeric claim I make is marked MEASURED or ESTIMATED.** Mutation testing ran
in a separate `git worktree` at `cb16f7c` on `/home` (`/home/tom/m4_rt2_wt`), never
`/tmp` and never the live tree; it was removed before this report was written and the
live tree was verified clean:

```
$ git worktree remove /home/tom/m4_rt2_wt --force && git status --porcelain && echo "LIVE TREE CLEAN"
LIVE TREE CLEAN
```

I modified no repository file but this one. I did not select.

---

## 1. VERDICT

**The matrix does not survive as a matrix for the second consecutive round, and this
time the failure is worse than round 1's: revision 2's own reason for existing is
false.** Revision 2 declined to select from revision 1's field because that field
omitted *"the seam this project's own D-252 option matrix SELECTED for the
structurally identical question **after its own red team**, and which D-283 landed."*
**Neither conjunct is true, and the tree contains an ADR whose entire purpose is to
say so.** D-252's option matrix selected **nothing** — its option (c) was *"leave the
literal, register the exposure, and schedule the binding"*, and **D-288 exists solely
to relabel that option as a DEFERRAL with "NO OPTION SELECTED"**, warning in terms
that read on this matrix verbatim that a successor would otherwise *"have found the
decision already taken, the matrix already spent, and no red-team owed, on a choice
that had never been attacked."* That is precisely what revision 2 did. And D-283's
own text states that its choices **"HAVE NOT BEEN ATTACKED BY A FRESH-CONTEXT
DECISION-RED-TEAM"** and that *"that review is outstanding"*. So the precedent whose
absence voided revision 1's field, which entered N-J, which rebuilt ground 3, and
which is quoted to strike N-A′ off the live list, is an unattacked selection standing
on a matrix that selected nothing. Compounding it, the two facts that carry grounds 1
and 3 — **facts 10 and 12, both NEW in revision 2** — are MEASURED claims whose stated
commands cannot produce them, which is F5's and F7's defect class committed *fresh* in
the very revision whose recommendation spends a paragraph on candour about F7. A third
kill is independent of all that: ground 3 and ground 4 both turn on the claim that
D-283 chose the document *"because the arena had no flag surface at all"* — the arena
has `--config` and `--out`, and D-283's actual condition is about the **input's** flag
(*"the arena has no `--binary` flag"*), which, applied as written, puts the snapshot in
the **first** case, not the second. I do **not** conclude that every option fails: N-E,
N-J, N-K, N-L and N-F all survive as options. What has fallen is the recommendation.
After these kills, N-E retains exactly one intact ground — ground 2, that a default is
an EXIT-0-WRONG-ANSWER — and **ground 2 does not discriminate N-E from N-J, N-K, N-L
or N-F, none of which has a default either.** N-E survives as a member of the
no-default field with no surviving ground that picks it out of that field.

---

## 2. PER-OPTION SURVIVAL

| Option | Survival | The attack that did it |
|---|---|---|
| **N-A′** — optional `--config`, default kept | **FALLS** — but one of its three executioners is void | It still falls: D-252's own recorded defect of the flag option is N-A′'s exactly (*"an operator who forgets gets the old behaviour out of the very flag added to prevent it"*), and rule 3 forbids skip-with-default. But the matrix strikes it partly on *"Fact 12: D-283 landed … for this class **after a red team**"*, and **D-283 says in its own text it was never attacked** (R1). One of the "three in-tree authorities" is not an authority. |
| **N-E** — required `--config`, no default | **SURVIVES WOUNDED, AND ITS RECOMMENDATION FALLS** | Three of its four grounds fall: ground 1 is contradicted by its own flip clause 4 (R8), ground 3 inverts the precedent's stated condition (R2) and rests on a false precedent (R1), ground 4 declines N-J on the same false condition (R2) plus an unmeasured comparative the matrix withdrew one row earlier (R5). Ground 2 survives and does not discriminate N-E from N-J/N-K/N-L/N-F. Its cost cell's "at least FIVE sites" mis-transcribes the measurement it cites (R4). |
| **N-J** — a required snapshot run document | **SURVIVES, AND ITS REJECTION FALLS** | Not a strawman — but it is declined on two grounds I falsified. Its cost is an unmeasured comparative (*"ESTIMATED the largest change of the surviving options"*) that N-F's own cell, four rows down, says **"is not measured and is not claimed"** (R5); and its guard count is understated, since its own "what it does" cell has a caller-named required document path, which owes guards (i) and (iii) (R7). |
| **N-K** — a config-pair / two-record mode | **SURVIVES WOUNDED** | Untouched on substance and it is the only option that removes the confound structurally. Wounded because its cost is an unmeasured comparative chain (*"larger than N-E and smaller than N-J"*, no command), because it is **omitted from the guard enumeration and from flip clause 2's direction statement** while its own cost cell says it owes four guards (R7), and because the option it is compared against is itself uncosted. |
| **N-L** — re-pin the literal by ADR | **SURVIVES WOUNDED, AND IS UNDER-ARGUED IN BOTH DIRECTIONS** | Its "**Zero code change**" is **MEASURED FALSE** by me: performing exactly the N-L operation turns the suite red (R6). Against that, its precedent standing is *understated*: D-252's actually-recorded disposition for this class — before D-283 — was *"leave the literal, register the exposure, and schedule the binding"*, N-L's shape, and the matrix cites D-252 without ever reading it (R1). Two flip clauses remedy to N-L on a reachability claim (*"changes no caller at all"*) that R6 falsifies. |
| **N-F** — a second committed script | **SURVIVES WOUNDED** | Its own cost cell is now the most honest in the table — and it directly contradicts N-J's (R5). Substance unrebutted and unattacked by me. |
| **N-B′** — flip the committed config to staged | **SURVIVES AS A CORRECT REJECTION** | Its three surviving grounds hold against the tree (rule 6; D-190/D-194's H1 action deliberately landing *after* all four runs; D-204's flip clause being the operator's). Wound: its cost is an unmarked "Zero" and its `tactical_v0.txt` exposure is an unmarked, uncommanded **15** (R10) — correct, MEASURED by me, and still a D-291 breach. |
| **N-G** — an environment variable | **SURVIVES AS A CORRECT REJECTION, NOW CORRECTLY GROUNDED** | F4's repair holds: caller-side reproducibility is the true defect and fact 4 is no longer withheld from it. Wound: N-G is **absent from the guard enumeration** although an env var also lets a caller name a path (R7). |
| **N-H** — a different instrument | **SURVIVES AS A CORRECT REJECTION** | Its void reproduces at HEAD. Wound: F9's repair widened the *scope* and narrowed the *pattern*, so the instrument is blind by construction over exactly the file F9 named (R11). Conclusion independently verified true by me. |
| **N-D′** — the null row | **SURVIVES AS A CORRECTLY RECORDED REJECTION** | Properly stated. Its "**Zero**" is the exact cell round-1 F12 named by name, and it is **still unmarked** (R10). |

**Does every option fail?** No. The architect is not in D-317's position here. But
**the field is still incomplete** (R9) and **the recommendation cannot be selected on
this text**: three of its four grounds are falsified above, and the fourth does not
distinguish the option it recommends.

---

## 3. DISPOSITION AUDIT — did the repairs repair?

| # | Revision 2's claim | Holds? |
|---|---|---|
| **F1** | "ACCEPTED, reproduced independently… re-stated at **at least five sites**… **ground 4 is DELETED as a ground**" | **PARTLY. The ground really is deleted. The number is still wrong** (R4). Round 1's mutation reproduces exactly for me, but round 1 recorded that site 1036 *"passes under the mutation only by accident"* and needs no change under the implementation it tested. Revision 2 dropped that caveat and enumerated 1036 as one of the five. **MEASURED: with `go()` and test 1205 patched and 1036 untouched, the suite is 29/29 green.** One false precise number has been replaced by an unfalsifiable lower bound whose enumeration contains a non-site — and the number still does selection work in flip clause 1's trigger. |
| **F1 (deferral)** | fact 6d "**Not re-measured by this revision** — cited to that report" | **NOT LEGITIMATE AS DONE.** The deferral itself is defensible: the report is landed and the code is byte-identical (`git diff dab170b cb16f7c -- crates/ tools/ configs/` → empty, MEASURED). What is not defensible is that the transcription changed the cited claim (R4), that the row sits in a table whose header reads *"MEASURED at `dab170b`, with the command"* with a citation in the command column, and that re-measuring costs one `grep` plus **MEASURED 4.33 s** of test time. D-291's clause bites. |
| **F2** | "The flip section is rewritten. No clause names a remedy another part of the matrix forbids" | **HOLDS structurally.** Clauses 1 and 2 now remedy to N-L, and clause 4 is the only route to N-A′ and is conditioned on rule 1 falling. **But clause 1's stated reason for reachability — N-L "changes no caller at all because it changes no interface" — is MEASURED FALSE (R6).** The incoherence is gone; the remedy's premise is not sound. |
| **F3** | "ACCEPTED IN FULL. All four are entered: N-J, N-K, N-L, N-B′" | **THE ROWS ARE ENTERED AND THEY ARE REAL. THE REASON FOR ENTERING THEM IS FALSE** (R1), and **the field is still incomplete** (R9). N-J is not a strawman — but the precedent that admits it does not exist as described, and the two grounds that decline it are both falsified (R2, R5). |
| **F4** | "N-G's rejection is re-grounded on caller-side reproducibility" | **HOLDS.** Clean repair. Fact 4 is now stated as a property of the record and is no longer granted to one option and withheld from another. |
| **F5** | "Fact 7 now states only what `ls configs/` measures, and cites U3 §10 without restating the count" | **HOLDS.** MEASURED: `ls configs/ \| wc -l` → 12, `ls configs/ \| grep -c staged` → 0, and no staged-config count appears anywhere in revision 2's body. **But the defect class F5 named was re-committed in two NEW facts** (R3) — facts 10 and 12 are MEASURED claims whose commands cannot produce them. F5 was repaired at the cell and reproduced at the class. |
| **F6** | "It is a selection trigger, and revision 2 states it as one" | **PARTLY. The narrowing is reduced from two options to six, not removed** (R7). The guard paragraph and flip clause 2 both enumerate N-A′/N-E/N-J against N-F/N-L/N-B′ and **omit N-K, N-G, N-H, N-D′** — and **N-K's own cost cell says it owes "Four guards ×1"**, so the table contradicts the trigger. N-J is assigned "(ii) and (iv) only" while its own description hands a caller a required document path. |
| **F7** | "ACCEPTED, and it is the sharpest finding in the round" | **THE DIAGNOSIS IS ACCEPTED AND THE BEHAVIOUR IS REPEATED ONE LEVEL UP** (R3). The note buys its candour on a number that runs *against* the recommendation, while two rows above it in the same table, facts 10 and 12 — both new, both carrying a ground — commit F7(b)'s exact defect: a command that cannot see what the claim asserts. Fact 12's claim is additionally false at its source. Credit for the old error; the same error, fresh, uncorrected. |
| **F8** | "Ground 3 is rewritten around the precedent that exists" | **FAILS.** The precedent as described does not exist (R1), and the taxonomy that replaces it inverts the precedent's own stated condition (R2). Ground 3 is worse than revision 1's, because revision 1's ground 3 merely asserted a blank slate; revision 2's asserts a specific, checkable, false one. |
| **F9** | "Re-measured over all of `tools/`; the conclusion survives, the instrument did not" | **HALF-REPAIRED** (R11). The **scope** widened (`tools/*.sh` → `tools/`, 18 entries enumerated — MEASURED, reproduces). The **pattern narrowed**, from `'argument --config\|--config)'` to `'--config)'`, a shell `case`-arm idiom that a Python tool cannot match even if it required the flag. The instrument is blind by construction over exactly the one file F9 was about. The conclusion is true — I verified it independently. |
| **F10** | "ACCEPTED, folded into the corrected audit" | **NOT FOLDED.** The caller round 1 instantiated (`docs/experiments/wp15b_sprt_prereg.md:362`) appears in none of fact 6d's five sites, and flip clause 1 still frames "a docs command block" as hypothetical. Harmless in substance — MEASURED, that invocation already names `--config`, and repo-wide it is the only docs invocation — but the disposition claims a fold that did not happen. |
| **F11** | "RECORDED, NOT REPAIRED — and it is escalated" | **RIGHT DISPOSITION, INCOMPLETE RECORD** (R12). A matrix barred from editing the pre-registration cannot do better than record and escalate, and the OPEN section is accurate as far as it goes. It omits the three facts an architect needs to size the breach: the pre-registration is **Revision 4, DRAFT, "THIS DOCUMENT GOVERNS NOTHING YET"**, it **"has never passed a review"**, and the slot in question is an **OPERATOR-CONFIRM** slot. Recording an escalation without the status of what is escalated overstates it. |
| **F12** | "ACCEPTED; revision 2 marks every number" | **FALSE, and falsifiable by reading the same cell round 1 named** (R10). N-D′'s "**Zero.**" is still unmarked. So are N-B′'s "Zero.", N-L's "**Zero code change.** Two ADR lines and two runs", and N-B′'s "**15**" — which is additionally uncommanded and absent from the Facts table. Fifth consecutive round. |
| **F13** | "CARRIED, and strengthened with the evidence the red team supplied" | **CARRIED, AND OVERSTATED** (R8). The evidence is two usage strings; **MEASURED, no ADR adopts required-no-default `--config` for `pistol.rs`** (`grep -o "no default path.\{0,300\}" docs/decisions.md` → no output). Ground 1 says the transfer "is not this matrix's inference"; flip clause 4, in the same document, says "fact 10 is evidence about practice, not about the rule's text". Both cannot be right. |

---

## 4. THE STRONGEST SURVIVING ATTACK, PER SURVIVING OPTION

Quotable into an ADR line for whichever option the architect selects.

**Against N-E (still the recommendation, no longer the recommended-on-grounds):**

> N-E was recommended on four grounds and three of them fall to the tree. Ground 3's
> "two precedents" rests on the claim that D-252's option matrix chose the document
> seam after a red team; **D-288 exists in this repository to record that D-252's
> option (c) was a DEFERRAL with "NO OPTION SELECTED"**, and D-283 states in its own
> text that its choices "HAVE NOT BEEN ATTACKED BY A FRESH-CONTEXT DECISION-RED-TEAM".
> Ground 3 and ground 4 both then turn on D-283 having chosen the document "because
> the arena had no flag surface at all" — the arena takes `--config` and `--out`, and
> D-283's condition is that the arena had **no flag for that input**, which is exactly
> the snapshot's position with respect to its config. Ground 1 asserts that rule 1's
> reach "is not this matrix's inference" while the matrix's own flip clause 4 concedes
> that fact 10 "is evidence about practice, not about the rule's text", and no ADR
> adopts required-no-default `--config` for `pistol.rs`. What survives is ground 2 —
> a default is an EXIT-0-WRONG-ANSWER and N-E refuses it — **and ground 2 is equally
> an argument for N-J, N-K, N-L and N-F, none of which has a default.**

**Against N-J:**

> N-J is declined on cost and on a precedent condition, and both are unsound. Its cost
> is "ESTIMATED the largest change of the surviving options" with no command and no
> magnitude — the identical unmeasured comparative revision 2 explicitly **withdrew**
> from N-F four rows down, where the same table now states that "whether extraction is
> larger than N-J's schema is **not measured and is not claimed**". A table cannot
> both decline to claim a comparison and rest a recommendation on it. Its guard count
> is understated: N-J hands a caller a required document path, so it owes guards (i)
> and (iii) as much as any flag option, not "(ii) and (iv) only".

**Against N-K:**

> N-K is the only option that makes the BEFORE/AFTER confound structurally impossible,
> and the matrix costs it entirely by comparison to two options whose own costs are
> unmeasured ("larger than N-E and smaller than N-J"). It is then **omitted from the
> guard enumeration and from the flip clause that F6 forced the matrix to state as a
> selection trigger**, while its own cost cell says it owes four guards — so the
> trigger the matrix repaired is stated over a subset of the field for the second
> revision running.

**Against N-L:**

> N-L is costed at "**Zero code change**" and that is MEASURED FALSE: performing
> exactly the N-L operation — re-pinning the literal, and nothing else — turns
> `snapshot_deterministic_across_a_clean_and_a_dirty_working_tree` red, because
> `ScratchRepo::new` copies `configs/instrument_v0.toml` into the scratch tree **by
> name**. Two flip clauses remedy to N-L on the stated reason that it "changes no
> caller at all". Against that, N-L is also the option the matrix under-reads in its
> own favour: D-252's recorded disposition for this class, before D-283, was "leave
> the literal, register the exposure, and schedule the binding" — N-L's shape — and
> the matrix cites D-252 in a fact whose command never opens it.

**Against N-F:**

> N-F's cost is now the only comparative in the table that admits it is not measured,
> and the table contradicts it one row up. Its substance — two instruments that must
> stay in step while the whole point is comparing their outputs — is unrebutted and
> was not attacked in either round.

**Binding on any selection:**

> No option in this matrix has been compared against a precedent that exists. The
> tree contains **no attacked selection** for how an instrument binds a per-run input:
> D-252's matrix selected nothing (D-288), D-283's selection records that its review
> is outstanding, and U4-Z's shape-1/shape-2 comparison says of itself that "it was
> never put to a fresh-context DECISION-RED-TEAM". Ground 3's "the precedent slate is
> not blank" is true only in the sense that the slate holds two unattacked decisions
> and two usage strings. **And the field still excludes the mechanism rule 1 itself
> uses**: `Budget` is a CLOSED ENUM with no default, and no option in this matrix
> offers a closed set of committed configs — the shape the matrix's own binding text
> would suggest.

---

## 5. FINDINGS

### R1 — KILL. Revision 2's reason for existing is false, and an ADR exists whose sole purpose is to say so.

**Claim attacked.** Preamble: *"the option space omitted four rows, one of which is the
seam this project's own **D-252 option matrix SELECTED** for the structurally identical
question **after its own red team**, and which D-283 landed."* And fact 12, marked
**MEASURED**: *"**D-252's option matrix chose the DOCUMENT seam for the structurally
identical question and D-283 landed it**"*. And N-J's cell: *"it is the shape this
project selected once for this class."* And N-A′'s cell: *"D-283 landed … for this
class **after a red team**."*

**Contradicting evidence — three separate falsifications, all in `docs/decisions.md`.**

**(a) D-252's option (c) was NOT the document key. It was N-L's shape.**

```
$ grep -o "(c) ADOPTED[^.]*\." docs/decisions.md
(c) ADOPTED: leave the literal, register the exposure, and schedule the binding.
(c) ADOPTED: leave the literal, register the exposure, and schedule the binding», and what (c) actually did was decline to bind the four operator-run SPRT documents and record the exposure.
```

**(b) D-252's matrix selected nothing, and D-288 exists to say so — in words that
describe this matrix's error in advance.**

```
$ grep -o "^D-288:.\{0,900\}" docs/decisions.md
D-288: D-252's OPTION (c) IS RELABELLED — IT WAS A DEFERRAL AND IT SAID "ADOPTED", AND THE
LABEL HAD TEETH. … an option recorded as ADOPTED in an option matrix is one that has been
SELECTED, and CLAUDE.md settles a named design decision with more than one viable option by
a matrix attacked by a fresh-context DECISION-RED-TEAM BEFORE selection — so a successor
picking up WP-1.10 and reading (c) as adopted would have found the decision already taken,
the matrix already spent, and no red-team owed, on a choice that had never been attacked.
… The correct labels: (a) RECOMMENDED, NOT ADOPTED, as written; (b) REJECTED ON THE MERITS,
as written; **(c) DEFERRED — the exposure registered, the binding scheduled, and NO OPTION
SELECTED.**
```

**(c) Neither D-252's matrix nor D-283's selection was ever attacked.**

```
$ grep -o "THE OPTION MATRIX, WRITTEN HERE AND NOT YET ATTACKED\." docs/decisions.md
THE OPTION MATRIX, WRITTEN HERE AND NOT YET ATTACKED.
$ grep -o "THE OPTION MATRIX AND ITS STATUS\..\{0,600\}" docs/decisions.md
THE OPTION MATRIX AND ITS STATUS. The choice between (a) an arena flag, (b) an
existence-and-executability check, and (c) a document key was written in D-252 and the
operator's amendment sheet SELECTED (c); the sub-choices settled here — required rather than
optional, run-start rather than validation-time, and a gate that rewrites rather than a
document that is exempt — are recorded above with their reasons and HAVE NOT BEEN ATTACKED
BY A FRESH-CONTEXT DECISION-RED-TEAM, because this session was directed not to dispatch
subagents. That review is outstanding against the revision this lands at and this line is
what it reads.
```

**And fact 12's own command cannot see any of it.** Its command column is
`grep -o "^D-283:.\{0,1500\}" docs/decisions.md` — a command that reads D-283 and
nothing else, offered as the MEASURED support for a claim about **D-252's option
matrix**. Round 1 made the same error first (its F3(a) called D-252's option (c) "a
document key" and said the matrix chose it "after a red team"); revision 2 accepted
F3 **"IN FULL"**, rebuilt its whole field and ground 3 on that characterisation, and
**never opened D-252**. D-283 alone would have half-warned it: D-283 relabels D-252's
(c) as "a document key" too, and the tree's answer to that discrepancy — D-288 — sits
five ADR lines later and is reachable from any `grep D-252 docs/decisions.md`.

**Why this is the decisive kill.** Revision 2 exists *because* revision 1's field
omitted a precedent-selected rival. There was no precedent-selected rival. The
document that unseated revision 1 is itself resting on the failure mode
`CLAUDE.md` names — *"An option adopted without a matrix, or a matrix never attacked,
is the same breach as silent architecture drift"* — cited as authority. **KILL.**

*(N-J is still a real option and this finding does not remove it. What it removes is
the authority the matrix borrows to admit it, to strike N-A′ with, and to build
ground 3 on — and it hands N-L a precedent the matrix never noticed it had.)*

---

### R2 — KILL. "The arena had no flag surface at all" is false, and it is the hinge of grounds 3 and 4.

**Claim attacked.** N-J's failure-modes cell: *"**The precedent's own condition does
not hold here** (fact 12): D-283 chose the document *because the arena had no flag
surface at all*, and the snapshot has six arms."* And ground 3: *"Where a per-run
input had **no flag surface**, D-252's matrix chose the document key… The snapshot has
six flag arms … so it is in the second case, not the first."*

**Contradicting evidence.** The arena has a flag surface. It has exactly two arms, and
one of them is spelled `--config`:

```
$ sed -n '29,33p' crates/pistol-arena/src/bin/arena.rs
usage:
  arena --config <path> --out <path>

  --config  an arena config. Always explicit: there is no default path and no
            built-in configuration (CLAUDE.md rule 1). It states the openings,
$ grep -n '"--config", config, "--out", out' crates/pistol-arena/src/bin/arena.rs
77:        ["--config", config, "--out", out] | ["--out", out, "--config", config] => {
```

**The matrix quotes the real condition correctly one table earlier and then
paraphrases it into its opposite.** Fact 12 quotes D-283 accurately: *"the arena has
no `--binary` flag … so the only binding available is the one the arena reads paths
out of"*. The condition is about **the input**, not the instrument. The arena had two
flag arms and still fell in the document case, because the *binary* had no flag.

**Apply D-283's own test to the snapshot.** The input is the config. MEASURED, it has
no flag:

```
$ grep -n CONFIG tools/baseline_snapshot.sh | head -1
170:CONFIG="configs/instrument_v0.toml"
$ grep -rln -- '--config)' tools/ ; echo "exit $?"
exit 1
```

By the precedent's stated condition the snapshot is in the **first** case. Ground 3
reaches the second only by substituting "the instrument has flag arms" for "the input
has a flag", and ground 4 declines N-J — the option ground 4 concedes has "a strictly
stronger property" — on the same substitution.

**Stated so this is not a strawman:** a *sound* version of ground 3 exists and the
matrix does not make it. D-252's recorded objection to the flag option was that it
*"adds an argument to the PUBLIC GRAMMAR of the instrument rule 6 makes the judge"* —
a cost argument that genuinely distinguishes the arena (whose grammar is two arms and
which is the SPRT judge) from the snapshot (six arms, not the judge). That argument is
about cost and public grammar. It is not the argument ground 3 makes, and it is not
the argument that admits or declines N-J. **KILL against grounds 3 and 4.**

---

### R3 — KILL. F5's and F7's defect class, committed fresh in the two facts that carry grounds 1 and 3.

**Claim attacked.** The Facts table header: *"FACTS THE MATRIX STANDS ON — MEASURED at
`dab170b`, **with the command**"*. And the COST section's closing: *"every MEASURED
value is either a structural fact of the tree **with its command beside it**, or is
cited to the session or the report that measured it."*

**Contradicting evidence — two of the three NEW facts.**

**Fact 10** asserts two things and commands one: *"Required-with-no-default `--config`
is **shipped twice** … `crates/pistol-cli/src/bin/pistol.rs:34` … **and the arena
binary matches only `["--config", c, "--out", o]`**"* — command:
`grep -n -- '--config' crates/pistol-cli/src/bin/pistol.rs`. That command never opens
`arena.rs`.

```
$ grep -n -- '--config' crates/pistol-cli/src/bin/pistol.rs
29:  pistol --config <path>                      speak the line protocol on stdin
34:  --config      an engine config. Always explicit: there is no default path and
42:                pre-registered against, so `selftest` takes no --config.
98:    let path = PathBuf::from(one(&flags, "--config")?);
99:    only(&flags, &["--config"])?;
```

The arena half is **true** — I verified it (R2) — and it is **unattested by the
command the matrix prints**. Fact 10 carries ground 1.

**Fact 12** asserts a claim about D-252 and commands a read of D-283 only (R1). Its
claim is additionally **false**. Fact 12 carries ground 3.

**Why this is a kill and not bookkeeping.** These are the two facts revision 2 added,
in the revision whose recommendation devotes a paragraph to F7 — *"a disclosure that
points away from where its own error is does the work of concealment whatever its
intent"* — and F7's operative sub-finding was **F7(b): "the instrument it names is the
defect… a range that stops at line 201 cannot see the invocations at 1036 and 1205"**.
A command that reads `pistol.rs` cannot see `arena.rs`; a command that reads `D-283`
cannot see `D-252`. The same instrument defect, twice, in the same table, in the
revision that names it. And the closing sentence quoted above is false for both rows.
**KILL.**

---

### R4 — KILL against the F1 disposition. "At least FIVE sites" is not what the cited instrument measured.

**Claim attacked.** Fact 6d: *"**The caller audit for a REQUIRED config is at least
FIVE sites** — the default literal (170), the usage block (109 ff.), `go()`, **test
1036** and test 1205. MEASURED by the round-1 red team…"* And N-E's cost cell: *"caller
audit **at least FIVE sites** (fact 6d) — **NOT three, as revision 1 claimed**."*

**Contradicting evidence.** The cited report attached a caveat to 1036 that revision 2
dropped: *"the 1036 site passes under the mutation only by accident: it asserts a
refusal that fires during argument parsing, before the config check."*

**Reproducer — mutation worktree at `cb16f7c` on `/home`, with a control.** Minimal
N-E applied to the shipped script: `CONFIG=""`, a `--config` case arm beside
`--binary`, and `[ -n "$CONFIG" ] || fail "--config is required and has no default"`
before line 271's `[ -f ]`.

*Control, unmutated at `cb16f7c`:*
```
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.33s
EXIT=0
```

*Mutation applied, no test-side change — round 1 reproduces exactly:*
```
test result: FAILED. 1 passed; 28 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

*Mutation applied, `--config` patched into ONLY `go()` — round 1 reproduces exactly:*
```
thread 'a_relative_out_lands_in_the_callers_directory_and_not_the_repository_root' panicked at
crates/pistol-cli/tests/baseline_snapshot_tests.rs:1215:5:
the run must succeed:
stderr: baseline_snapshot: FAIL: --config is required and has no default
test result: FAILED. 28 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.14s
```

*Mutation applied, `go()` **and 1205** patched, **1036 deliberately NOT patched** — the
measurement revision 2 did not take:*
```
1205 patched; 1036 deliberately NOT patched
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.25s
```

**MEASURED.** Two test-side sites suffice. Test 1036 is enumerated in fact 6d as a
member of the audit and **requires no change** under the implementation the cited
measurement used. The enumeration that replaced revision 1's false "three" contains a
non-site; its "at least" makes it unfalsifiable upward; and it is still load-bearing,
because flip clause 1's trigger is *"an audit at IMPL finds callers beyond the five of
fact 6d"*. Deleting ground 4 removed the ground and left the number doing selection
work.

**Also MEASURED, and it strengthens the point rather than the number:** site 1205 runs
with `.current_dir(&caller)` outside the repository, so under guard (i) — caller-relative
resolution, which minimal N-E does not implement — that site needs an *absolute* config
path, not the same edit as `go()`. The audit's sites are not uniform, which no cell in
the matrix says. **KILL against the disposition; the direction runs against the
recommendation, and I state that so the finding is not read as bias-hunting.**

---

### R5 — KILL. The withdrawn comparative was relocated onto the recommendation's strongest rival, and the table now contradicts itself.

**Claim attacked.** N-F's cost cell, repairing F8's shape: *"**Revision 1 costed it
'ESTIMATED the larger `tools/` change of any option' with no command, in the direction
disfavouring the recommendation's most distinct rival (F8's shape)** — that estimate is
withdrawn; what is MEASURED is the 646 lines, and **whether extraction is larger than
N-J's schema is not measured and is not claimed**."*

**Contradicting evidence.** N-J's cost cell, four rows above: *"A document schema, its
validation, and the coverage rule's tests — **ESTIMATED the largest change of the
surviving options.**"*

```
$ grep -c 'ESTIMATED the largest change of the surviving options' docs/experiments/matrix_M4_snapshot_config_seam.md
1
$ grep -c 'whether extraction is larger than N-J' docs/experiments/matrix_M4_snapshot_config_seam.md
1
```

N-F is a surviving option — round 1 rated it SURVIVES WOUNDED and revision 2 does not
reject it. So "the largest change of the surviving options" **is** the claim that N-J
> N-F, which N-F's own cell says is "not measured and is not claimed". The matrix
asserts and disclaims the identical comparison in one table.

**And the direction is the one F7 flagged.** Revision 1's unmeasured comparative
inflated N-F, "the recommendation's most distinct rival". Revision 2 withdrew it and
wrote the same shape — no command, no magnitude — onto **N-J**, the option ground 4
concedes has "a strictly stronger property than any flag option offers". The repair
moved the defect from the old strongest rival to the new one. **KILL.**

---

### R6 — MAJOR. N-L's "Zero code change" is measurably false, and two flip clauses rest on it.

**Claim attacked.** N-L's cost cell: *"**Zero code change.** Two ADR lines and two
runs."* And flip clause 1: *"Remedy: **flip to N-L**, which **changes no caller at all
because it changes no interface**. Reachable: N-L is the current shape plus two ADR
lines, and nothing of N-E need be built first."*

**Contradicting evidence.** The shipped script's literal is named by the test suite:

```
$ grep -rn 'instrument_v0' crates/pistol-cli/tests/baseline_snapshot_tests.rs
283:	printf 'id budgets depth_turns nodes\nid config configs/instrument_v0.toml\n'
385:            "configs/instrument_v0.toml",
```

Line 385 is inside `ScratchRepo::new`'s copy list — the scratch tree materialises
`configs/instrument_v0.toml` **by name** and nothing else.

**Reproducer — the N-L operation, and nothing else, in the same worktree.** Re-pin the
literal to the tree's other committed instrument config (`configs/instrument_r2_v0.toml`,
which fact 8 says nothing references):

```
$ python3 -c '...'   # CONFIG="configs/instrument_v0.toml" -> CONFIG="configs/instrument_r2_v0.toml"
N-L operation applied: literal re-pinned to the second committed config, NO other change
$ cargo test --release -p pistol-cli --test baseline_snapshot_tests
stderr: baseline_snapshot: FAIL: no config at configs/instrument_r2_v0.toml
failures:
    snapshot_deterministic_across_a_clean_and_a_dirty_working_tree
test result: FAILED. 28 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.40s
```

**MEASURED.** N-L changes a caller. Its real cost is two ADR lines, the staged config
document, and at least the one test-fixture site at 385 — small, but not zero, and not
"no caller at all". **The cheapest option in the matrix, the one two flip clauses
remedy to, carries an unmarked cost claim that one 3.4-second test run falsifies**, in
a document whose COST section states *"No number in revision 2 was produced by a run
this session took"*. D-291's clause is exactly this: an estimate measurable in seconds.

*Aptly, the test that catches it is the one D-232 records as having been vacuous in the
tree it runs in until that round fixed it.*

---

### R7 — MAJOR. F6's narrowing is reduced, not removed, and the guard assignment contradicts the option table.

**Claim attacked.** The guards paragraph: *"**N-F, N-L and N-B′ owe NONE of these,
because none lets a caller name a path; N-J owes (ii) and (iv) only** … **THE GUARD
COUNT IS THEREFORE A SELECTION TRIGGER AND NOT A COMMON COST** (F6)"*. And flip clause
2: *"every guard added moves N-A′, N-E and N-J and does not move N-F, N-L or N-B′ at
all, because those let no caller name a path."*

**Contradicting evidence — two defects.**

**(a) Four of the ten options are absent from both lists, and one of them contradicts
the table.** The enumerated six are N-A′, N-E, N-J | N-F, N-L, N-B′. **N-K, N-G, N-H
and N-D′ appear in neither.** N-K's own cost cell says:

```
$ grep -o 'Four guards ×1 (one parse site, two values)' docs/experiments/matrix_M4_snapshot_config_seam.md
Four guards ×1 (one parse site, two values)
```

So the matrix's own table puts N-K on the guard-bearing side while the trigger F6
forced omits it — and N-K is the option flip clause 5 remedies to. N-G also lets a
caller name a path (through the environment) and is likewise unlisted. **F6's exact
defect — "reaches its conclusion only by quietly restricting the selection to the
options that share the property" — is narrowed from two options to six and survives.**

**(b) N-J's assignment contradicts N-J's own description.** N-J: *"the script takes
that document, **required and with no default**"* — a caller-named path on argv.
Guard (i) is caller-relative resolution and guard (iii) is three named refusals
against a bare `[ -f ]`; both are properties of *letting a caller name a path*, which
N-J does. N-J owes all four with respect to the document path, plus (ii) and (iv) for
the config path parsed out of it — strictly more than any flag option, not fewer.

**Reproducer.** The matrix's own text against itself:

```
$ grep -o 'N-J owes (ii) and (iv) only[^.]*\.' docs/experiments/matrix_M4_snapshot_config_seam.md
N-J owes (ii) and (iv) only, against a parsed document key rather than an argv string.
$ grep -o 'the script takes that document, required and with no default' docs/experiments/matrix_M4_snapshot_config_seam.md
the script takes that document, required and with no default
```

**Direction, stated:** (b) understates N-J's cost, which runs *against* the
recommendation, so this is not a bias finding. It is a finding that the quantity the
matrix elevated to a **selection trigger** is mis-assigned across the field it is meant
to select over.

---

### R8 — MAJOR. Ground 1 asserts what its own flip clause 4 concedes is not so, and no ADR backs it.

**Claim attacked.** Ground 1: *"**Hard rule 1 reaches here, and that is not this
matrix's inference** (fact 10, F13)."* Flip clause 4, in the same document: *"it is
named because ground 1 does the most work and **fact 10 is evidence about practice, not
about the rule's text**."*

**Contradicting evidence.** MEASURED — the tree carries no ADR adopting
required-no-default `--config` for the engine binary; the evidence is two usage strings:

```
$ grep -o "no default path.\{0,300\}" docs/decisions.md ; echo "exit $?"
exit 1
$ grep -n -- '--config' docs/decisions.md | wc -l
```
(the only `--config` occurrences in the ADR log are D-252's and D-283's, both about the
arena's *binary* binding — verified by reading both lines in full.)

The one ADR-level extension of rule 1 beyond a schema is D-283's *"CLAUDE.md rule 1
also forbids a code-side default for a tunable, and 'absent means unchecked' is that
default wearing a different word"* — **and D-283 records that this exact sub-choice
("required rather than optional") has not been attacked** (R1c). So ground 1's
supporting stack is: two usage strings (practice), plus one unattacked ADR. The
residual inference — from a `crates/` binary's argv to a `tools/` shell script's shell
variable — is narrow, as round 1 said, and it is **still the matrix's inference**. The
matrix says it is not, and then says it is, forty lines apart. Not a kill on the
ground's substance; a kill on its stated standing, in the ground the recommendation
says "binds first".

---

### R9 — MAJOR. The option space is incomplete for the second round, and one of the missing rows is the mechanism ground 1's own binding text uses.

**Claim attacked.** Ten options offered as the space, after a round that found four
missing rows.

**Contradicting evidence — two rows, the first stronger than any of round 1's four.**

**(a) A CLOSED ENUM of committed configs, required, with no default.** Ground 1 rests
on rule 1, quoting it: *"no code-side default for any tunable — a default lives in
exactly one schema place"*, with *"`Budget` is a closed enum {depth_turns, nodes,
movetime_ms}; an absent budget is an error, never a fallback"* as its stated reason.
**Rule 1's own mechanism for the thing ground 1 invokes it for is a CLOSED ENUM.** An
option `--config {instrument|staged}` — a required selector over the committed set,
not a free path — is: required with no default (satisfies ground 1 and ground 2
identically to N-E), and **owes far fewer guards than N-E**, because guards (i)
caller-relative resolution and (iii) directory/missing/not-a-regular-file all exist
only because a caller can name an arbitrary path, which a closed enum forbids by
construction. On the matrix's own selection trigger — the guard count (R7) — this row
would dominate N-E. It is absent, and the field's framing (*"lets a caller name the
path"*, *"an argv string"*) excludes it by presupposition for the second revision
running: revision 2 reframed the question from "a record" to "the BEFORE and AFTER
records", and left "the caller names a **path**" untouched.

**(b) Bind the config through the existing workload fixture.** The corpus is already a
caller-named document, named by an existing flag, already digested into the record
**above the marker**, and already carrying keyed header lines in the script's own
idiom:

```
$ sed -n '20p' crates/pistol-cli/tests/fixtures/bench_positions_v1.txt
# param band centre 15 width 2
$ sed -n '442p' tools/baseline_snapshot.sh
	echo "corpus $(basename "$CORPUS") sha256 $CORPUS_SHA256 positions $COUNT"
```

This is D-283's document seam **without a new document and without a new flag** — the
two costs ground 4 charges N-J with. It reaches N-J's "strictly stronger property"
(a content binding checkable before the run) for the price of a header key, because
the corpus digest is already computed and already in the invariant block. It has a
real failure mode the matrix could state and reject on — the script's own usage text
scopes `--corpus` as *"WORKLOAD SCOPE: it shrinks the run without touching the
registered budget"*, and putting a config binding in it conflates the two scopes — and
MEASURED, the script does not currently parse `# param` at all (`grep -n 'param'
tools/baseline_snapshot.sh` → no output), so it is not free. **A viable option with a
statable rejection is exactly what F11 and D-317 say must appear as a row.**

*(I considered and reject as non-viable, recorded so the space is covered: a `--binary`
wrapper that execs the engine with a staged `--config`. MEASURED, `pistol.rs:98-99`
uses `one(&flags, "--config")` then `only(&flags, &["--config"])`, and the script
already passes `--config` at three sites, so the duplicate would be refused. Not a
missing row.)*

---

### R10 — MINOR, fifth consecutive round. F12's disposition is false in the cell round 1 named by name.

**Claim attacked.** *"**F12** … **ACCEPTED**; revision 2 marks every number."*

**Contradicting evidence.** Four unmarked numerics in revision 2's body:

```
$ sed -n '/^| \*\*N-D′/p;/^| \*\*N-B′/p;/^| \*\*N-L/p' docs/experiments/matrix_M4_snapshot_config_seam.md | grep -o '| Zero[^|]*|'
| Zero code change.** Two ADR lines and two runs. |
| Zero. |
| Zero. |
```

- **N-D′'s "Zero."** — the exact cell round-1 F12 named.
- **N-B′'s "Zero."**
- **N-L's "Zero code change." and "Two ADR lines and two runs"** — and R6 measures the first of these false.
- **N-B′'s "15"** — *"`tactical_v0.txt`'s **15** `instrument_v0`-bound cases"* — unmarked, no command, and not in the Facts table. **MEASURED by me, the number is correct:**

```
$ grep -c '^config configs/instrument_v0.toml' crates/pistol-cli/tests/fixtures/tactical_v0.txt
15
$ grep -c instrument_v0 crates/pistol-cli/tests/fixtures/tactical_v0.txt
16
```
(the sixteenth is a comment at line 26 — so "15 cases" is right and "16 occurrences" is
what a naive command would have returned, which is why the missing command matters.)

Also unmarked: flip clause 2's *"if the guards grow past roughly double"*, the threshold
that fires a flip to N-L.

---

### R11 — MINOR. F9's repair widened the scope and narrowed the pattern, leaving the instrument blind over exactly the file F9 was about.

**Claim attacked.** Fact 3: *"re-measured over the WHOLE directory, not the `*.sh` glob
revision 1 used, which missed `tools/wp15b_attribution_check.py` (F9)"* — command
`ls tools/`; `grep -rln -- '--config)' tools/`.

**Contradicting evidence.** Revision 1's pattern was
`'argument --config\|--config)'`; revision 2's is `'--config)'` — a shell `case`-arm
idiom. The one file whose omission caused F9 is a Python script, which cannot express
`--config)` even if it required the flag:

```
$ printf 'import argparse\np = argparse.ArgumentParser()\np.add_argument("--config", required=True)\n' > probe/t.py
$ grep -rln -- '--config)' probe/ ; echo "exit $?"
exit 1
```

A tool that **requires** `--config` returns "no output" from the widened instrument.
The scope grew; the pattern's coverage over the newly in-scope file is nil. This is
`SHELL_CHECKLIST` item 3's class ("a substring is not a token… Anchor, or match the
field") applied to a pattern instead of a glob — F9's own diagnosis, one layer along.

**The conclusion is nevertheless true**, verified independently: every `--config`
under `tools/` is a flag passed *to* the engine or arena, and the Python tool takes
positional arguments only:

```
$ grep -rn -- '--config' tools/ | wc -l
13
$ grep -n 'sys.argv' tools/wp15b_attribution_check.py
85:    if len(sys.argv) != 3:
86:
87:    report, engine = sys.argv[1], sys.argv[2]
$ ls tools/ | wc -l
18
$ ls tools/ | sed 's/.*\.//' | sort | uniq -c
      1 md
      1 py
     16 sh
```

Fact 3's counts reproduce exactly. **Not a kill; the instrument, again, is narrower
than its stated scope.**

---

### R12 — MINOR. The F11 escalation omits the status of the thing it escalates.

**Claim attacked.** OPEN: *"the pre-registration is the document that will govern a
run, and it currently rests on a selection no matrix has ever supported."*

**Contradicting evidence.** True, and incomplete in the direction that overstates:

```
$ sed -n '3p' docs/experiments/wp15b_sprt_prereg.md
**Revision 4. DRAFT. THIS DOCUMENT GOVERNS NOTHING YET.**
$ grep -n 'has never passed a review' docs/experiments/wp15b_sprt_prereg.md
757:**This document has never passed a review, and it does not claim to.**
$ sed -n '358,359p' docs/experiments/wp15b_sprt_prereg.md
**THE INSTRUMENT: `tools/baseline_snapshot.sh` AT THE REVISION THAT LANDS ITS
`--config` FLAG** — **OPERATOR-CONFIRM** (§9.7).
```

**Recording-and-escalating is the right disposition** — a matrix barred from editing
another document cannot do more, and the false sentence (*"its §9 MATRIX M4 ADOPTS
adding `--config`"*) is real and does foreclose seven of the ten options by flag
spelling. But an escalation that does not state that the document is a never-reviewed
draft with an OPERATOR-CONFIRM slot gives the architect a live breach where the tree
holds a queued draft. **Not a kill. It is not a reason to stop the matrix.**

---

### R13 — NOTE. Cut-boundary fit holds, with one unstated exposure.

The matrix decides nothing U1, U2, U3 or `WPQ_seed.md` owns. Fact 7's repair is
correct — the count U3 §10 owns exclusively is cited and not restated, verified:

```
$ grep -n 'one place the count is stated' -i docs/experiments/U3_tier_t.md
254:any value. **This is the one place the count is stated; U2 §2.2 and U3-Q cite it
$ sed -n '1,215p' docs/experiments/matrix_M4_snapshot_config_seam.md | grep -c 'four staged\|FOUR'
0
```

N-B′ is the one row whose **adoption** would decide U3 §10's object
(`configs/instrument_v0.toml`) and D-194's; it is rejected, so the boundary holds.
**The unstated exposure:** N-J's cost cell does not say where its run document lives.
If it lands in `configs/`, it moves the count U3 §10 owns exclusively — the count B5
was about. A cost cell for a new committed document should say which directory it
lands in.

On `tools/SHELL_CHECKLIST.md`'s coverage rule (item 10, *"Any `tools/` script that
produces a recorded number carries at least one test"*), the cost cells are compliant
for N-A′, N-E, N-J, N-K and N-F. **N-L's is not**: it budgets no test work at all
(*"Zero code change. Two ADR lines and two runs"*) for an operation that R6 measures as
turning the suite red.

---

### R14 — REJECTED, with the attempted reproducer.

**What I attacked.** Fact 3's *conclusion* — that no `tools/` artefact takes
`--config` — on the theory that R11's narrowed pattern was hiding a real instance
under another spelling (`--config=*`, `"--config")`, argparse, `getopts`).

**Why it fails.** There is none. Every `--config` under `tools/` is passed *to*
`$ENGINE`, `$BINARY` or `$ARENA`, and no artefact parses one for itself:

```
$ grep -rn -- '--config' tools/
tools/determinism.sh:153,169,182:	"$ENGINE" --config "$CONFIG" …
tools/movetime_check.sh:111:			"$ENGINE" --config "$CONFIG")" …
tools/baseline_snapshot.sh:425,464,531:	… "$BINARY" --config "$CONFIG" …
tools/bench_delta.sh:291,366:	… --config "$CONFIG" …
tools/arena_smoke.sh:170:# The arena has no `--binary` flag — `--config` and `--out` are its only
tools/arena_smoke.sh:261:	"$ARENA" --config "$config" --out "$out" …
tools/wp15b_attribution_check.py:168,178:  [engine, "--config", config] …
$ grep -rn 'getopts\|--config=' tools/ ; echo "exit $?"
exit 1
```

**RECORDED AS REJECTED.** Fact 3's answer is right; only its instrument is narrow
(R11). Worth noting for the architect that `tools/arena_smoke.sh:170` carries D-283's
condition in a comment — *"The arena has no `--binary` flag — `--config` and `--out`
are its only …"* — which is the sentence R2 shows the matrix paraphrased into its
opposite, sitting in `tools/` where the matrix was already grepping.

---

## 6. WHAT I DID NOT FIND

Recorded so the architect knows the surface was covered and came back empty.

- **Every command in revision 2's Facts table reproduces at HEAD** — facts 1, 2, 3,
  4, 5, 6, 6b, 6c, 7, 8, 9, 10, 11 all return exactly what the matrix says, including
  every line number (170; 271/321/425/464/531; 440 and 633; 494/496/605; 29 tests;
  no `ci.sh` invocation; 172/1036/1205; 12 configs and 0 staged; no `instrument_r2_v0`
  reference; three engine call sites; 18 `tools/` entries; five pinned literals;
  `wc -l tools/baseline_snapshot.sh` → 646). **Fact 12 is the only Facts-table row
  whose claim is false** (R1); facts 10 and 12 are the only two whose commands cannot
  produce them (R3).
- **Revision 1's body below the fold is genuinely unedited** — 165 lines, byte-identical
  to `git show 77f7397:`, verified by stripping the quote prefix and diffing. Revision
  2's account of what revision 1 said is accurate in every row of the disposition table
  I checked; where the disposition is wrong, it is wrong about the *repair*, never
  about the *finding*.
- **The COST section is honest and its provenance checks out.** `34.5 s × 6 ≈ 3.5 min`
  is 207 s, and 34.5 s is traceable to the design at `6feb40a`
  (`git show 6feb40a:docs/experiments/wp15b_design.md | grep -n '34\.5'` → lines 1305,
  1338, 1341, 1667). The disclosure *"No number in revision 2 was produced by a run
  this session took"* is true of the runs; it is false of the second clause about
  commands (R3), and R6 shows the discipline it expresses cost the matrix a number it
  could have had for 3.4 seconds.
- **N-B′'s three surviving grounds hold against the ADR log.** D-190/D-194 land the H1
  action *after* all four runs precisely so a config document does not move under a
  live run; D-204 binds the tactical thresholds at the committed config and reserves
  the flip to the operator; rule 6 makes the SPRT the judge. Correct rejection.
- **`grep -c instrument_v0` on the D-209 golden is 0**, so the preamble row revision 1
  used to void the `ec8f7fb` table, and which revision 2 restates in N-B′'s cell, is
  true.
- **U4 §9's B2 is still open at HEAD** — *"B2 / M4 — no ADR line, and the selection is
  OPEN"* — so nothing has been landed under this matrix while it was being attacked.

---

*DECISION-RED-TEAM round 2 on matrix M4, pinned `cb16f7c` (an ancestor of HEAD
`a35503f`; every attacked file byte-identical, D-252/D-283/D-288 and U4 §9 verified
unchanged). Fresh context. Not the author of the matrix, of revision 1, of the design,
or of round 1. Mutation worktree on `/home`, removed; live tree clean. Nothing selected
here — selection is the architect's, and on this text three of the recommendation's
four grounds are not available to it.*
