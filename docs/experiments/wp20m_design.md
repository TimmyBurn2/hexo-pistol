# WP-2.0-M — DESIGN: the labelling pass

**REVISION 5**, under **D-548**'s grant of up to four further rounds, after
revision 4 (`a9a4a3a`) took a scoped re-review and returned **FAIL** — 0
BLOCKING, 2 MAJOR, 6 MINOR. **D-546's STOP had already fired**: revisions 3 and 4
are two failures, and the operator lifted the cap rather than taking the package
back. D-548 records the ground — the failure CLASS changed even though the
verdict did not, with two consecutive rounds returning zero author debt.

**WHAT REVISION 4 GOT WRONG, ON THE FACE, BECAUSE IT IS THE CLASS THIS ROUND
EXISTS TO CLOSE.** Revision 4's fix for revision 3's BLOCKING **spent a true
thing while applying a finding**: it moved
`a_captured_totals_line_keeps_every_field_but_nps_and_time` from a stub-driven
INTEGRATION test onto synthetic lines, and the sentence it replaced — verified
TRUE by the previous reviewer — was the only thing pinning that pass 2 **calls**
the normalisation at all. Delete the call, leave the function, and all
thirty-four registered tests stayed green. **That is D-544's recorded shape**, and
neither the citation checker (D-543) nor the passed-section freeze (D-547) guards
it: nothing false enters the document, and nothing a reviewer passed is edited.

**SO THIS REVISION ADDS THE GUARD THE ARC LACKS, AS A DISCIPLINE RATHER THAN AS
PROSE.** §0.2b lists, for every test this round moves, retires or re-drives, **what
pinned it before and what pins it now** — and a remedy that leaves the second
column empty is a remedy that spent something.

**WHAT REVISION 4's BLOCKING WAS**, kept because a fix round that hides what it is
fixing is how revision 2 failed. Revision 3's §14 sent the real-binary re-run
receipt to a capture pass over the report `tools/arena_smoke.sh` writes. **That
report cannot be read.** `crates/pistol-arena/src/transcript.rs:164-170` refuses
**every** source report whose budget kind is not `nodes` — not only a movetime
one, which is how revision 3 described it — and `configs/arena_smoke_v0.toml`
carries a `depth_turns` budget. §14 was replaced, not patched, and the
mischaracterisation that made it writable was corrected wherever it appeared
(§7).

This revision is not a rewrite. **D-545 named the defect as the authoring method** — five design reviews
in this arc, five failures, every one a claim about the code the code does not
make or a true claim a rewrite destroyed — so the method is fixed before the
content is.

---

## 0. THE ROUND, THE METHOD, AND THE FREEZE

### 0.1 The method, which is the grant's own condition

**LIFT, DO NOT REWRITE.** Every mechanism a reviewer has passed enters this
document as a **quotation of the surviving text**, cited to the revision it
survived in. New prose is connective tissue and NEW claims, and every new claim
about the tree carries a `path:line` citation (D-543).

`tools/design_citation_check.py` is run green over this document, with the files
it names and the tree does not hold declared, **before** the review is
dispatched. There are four, and **which package creates each is part of the
declaration**: `crates/pistol-arena/src/capture.rs` and
`crates/pistol-arena/src/usage.rs` are this package's;
`docs/label_corpus_manifest.md` is created by the commit that records this
package's first capture (§13a); and `configs/arena_wp20_label_pilot.toml` is
**named here and created by the pilot's pre-registration** (§13b), which is a
file this design constrains and does not write. Its own
output states what that is worth and this document does not oversell it: a green
run means the citations are real, not that the document is right.

**A NOTE ON D-483 AND THE CITATIONS, because the two rules meet here.** This
revision carries far more `path:line` citations than its predecessors, so a
numeral sweep over it returns many more numbers. **A citation is a pointer, not a
measurement**: D-483 forbids a design from carrying a measured number, and rev-2
MINOR J graded revision 2's line COUNT a breach on exactly that reading — *"a
line count is not a number a prereg or gate consumes, which is the harm D-483 was
written against"*. That count is gone from this revision (§1). What remains is
the citation discipline D-543 requires, and the two rules point the same way: a
claim about the code carries the citation that makes it checkable, and no number
in this document is one any pre-registration, gate or criterion reads.

### 0.2 The freeze (D-547), which is the other condition

A section a reviewer passed is frozen. Every edit to one is listed here with its
ground; an unlisted edit is a finding by itself.

| frozen section | where it was passed | what this revision does | ground for the edit |
|---|---|---|---|
| revision 1 §4 **COLDNESS** | `docs/experiments/wp20m_design_REVIEW.md` — its *"strongest attack that did not land"* re-derived the chain and closed *"INVARIANT 1 is sound and cheap"* | **RESTORED, as §12**, quoted, with a citation added at each site it names | Revision 2 deleted a passed section entire and took D-540's pilot obligation with it. That deletion is D-547's motivating instance; restoring it is the fix. The added citations are new material beside the quoted text, not edits to it |
| revision 2 §1 **WHERE THE CODE LIVES** | `docs/experiments/wp20m_design_REVIEW_rev2.md` MAJOR 11 APPLIED | **EDITED**: the measured line count is replaced by the run that produces it; the flag grammar is completed | MINOR J — a measured count is what D-483 forbids and what this document's own header banned four paragraphs above it. MINOR I — the flag's argument shape was left to the implementer |
| revision 2 §2 **WHICH POSITIONS ARE ASKED** | rev-2 review BLOCKING 2 APPLIED, all three limbs verified against the tree | **EDITED**: the prefix range is enumerated, and the legality precondition and the decidedness mechanism are restored. Revision 3 also dropped one sentence of the quoted block without listing it; **revision 4 restores that sentence** | MINOR G, MINOR K, and loss L3 — revision 2 deleted the paragraph that made its own INVARIANT 2 decidable. The dropped sentence is rev-3 review **m1**, an unlisted edit to a frozen section, which D-547 makes a finding by itself |
| revision 2 §3 second paragraph, **engine verification** | rev-2 review MAJOR 3 APPLIED | **LIFTED VERBATIM** | — |
| revision 2 §4 **THE ONE NORMALISATION** | rev-2 review's *"strongest attack that did not land"*, four routes closed | **LIFTED VERBATIM** | — |
| revision 2 §4 **THE SOURCE IS NAMED ON THE FACE OF THE FILE** | rev-2 review MAJOR 10 APPLIED | **LIFTED VERBATIM** | — |
| revision 2 §6 **the failure table** | rev-2 review MAJOR 1 APPLIED | **EDITED**: three rows added, **and row 1 is reworded** | MINOR H — hard rule 3 prefers a refusal named per reason, and the channel has two outcomes the table did not name. Row 1's rewording (*"its two engine sections differ"* → *"its two seats attest different engines"*) is **required by §3's BLOCKING A remedy** and was unlisted in revision 3 (rev-3 review **m2**). The third row is the malformed totals line (**m9**) |
| revision 2 §7 **the budget's kind** | rev-2 review MAJOR 8, substance taken | **EDITED**: the builder is named, and the grammar is fixed so no other kind is spellable | MAJOR 8's unapplied half |
| revision 2 §8 **the removed mutant** | rev-2 review MAJOR 6 APPLIED | **KEPT removed**, and replaced by a mutation this package's shipped code can actually take | MAJOR 6's own reasoning, carried one step further |
| revision 2 §4 **THE FILE'S SHAPE** | rev-2 review MAJOR 4, applied half | **LIFTED VERBATIM** | — (rowed for consistency: rev-3 review noted three verbatim lifts were rowed and this one was not) |
| revision 2's header table of **capture decisions** | rev-2 review BLOCKING 3 APPLIED | **LIFTED**, extended by **one row — *"WHAT is written, and in what grammar"***; the quoted lead-in drops revision 2's word *"four"*, which the added row makes false | The grammar was undecided across three revisions (rev-2 MAJOR E). **Revision 3's ground for this row named the slot instead, which is not the row it added** (rev-3 review **m3**); the slot is answered inside §3's existing row and the ground is corrected here |

**NOT FROZEN, because no reviewer passed them**: §3's first paragraph (rev-2
BLOCKING A), §5 (rev-2 MAJOR 5, NOT APPLIED), the invariant and test sets (rev-2
MAJOR D), and revision 2's §11 requirement-5 claim (rev-2 MAJOR 9, applied but
defective). **Nothing in revision 3 is frozen**: its review returned FAIL.

### 0.2a What REVISION 4 changed, and against which finding

D-547 asks that an edit to a frozen section be listed with its ground. **FOUR of
revision 4's edits landed in sections frozen by an earlier review — §1, §2, §6
and §7** — and revision 4's own header said two, which understated the block
D-547 exists to make trustworthy. All four carried grounds; the count did not.
The table below is revision 4's, with its count corrected; revision 5's own edits
are in §0.2b.

| § | change | finding |
|---|---|---|
| **§1** (frozen: *where the code lives*) | the `USAGE` paragraph and the fallback refusal's mode list are named as content the arm must carry | **m14** — a binary whose help text omits a mode it has |
| **§7** (frozen: *the budget's kind*) | the description of `transcript.rs:164-170` is corrected, and the constraint it forces on pass 1's config is recorded | **B1's root.** The quoted revision-2 block is untouched; the correction is in the prose beneath it |
| §2 | one sentence of the quoted block restored | m1 |
| §3.1 | the identity's config-path precondition, and a derive line number | M8, m5 |
| §4.2 / §4.3 | the TAB refusal moved to the write side with its driver named; the loader refuses an empty field | m6, m11 |
| §5 | `arena_version` **removed**; three inputs; the governing revision named where `docs/process.md` puts it | M1, M2, m7 |
| §6 | a row for an unrecognised totals line | m9 |
| §9 | INVARIANT 6, 7, 11 and 12 restated; a second declared-unpinned item | M7, m6, m10 |
| §10 | drivers named per test; the stub gains `demands_newgame_per_ask`; the solver-spelling and normalisation unit tests added; three renames | **M3**, **M4**, m7, m8 |
| §11 | the throughput obligation assigned | m12 |
| §13 | the ledger-append mutant's absence named; the arena config moved to the pilot | M5, M6 |
| §14 | **replaced entirely** | **B1**, M4, m4 |
| §15 | new: the ADR act owed at landing | m13 |

### 0.2b What REVISION 5 changed, and what each remedy PINNED BEFORE and PINS NOW

**This block is the guard D-548 names as missing.** A fix round is where this arc
loses true things: not by writing something false, which the checker catches, and
not by editing a passed section, which the freeze catches, but by **trading away
what a remedy was standing on**. So every test this round moves, retires or
re-drives is listed with both columns, and **an empty right-hand column is the
finding**.

| finding | change | what pinned it BEFORE | what pins it NOW |
|---|---|---|---|
| **N1** | the normalisation's integration limb is restored beside the unit tests | rev 3: one stub-driven integration assertion, killing both the function and the CALL | rev 5: the unit tests kill the function in both spellings, **and** `a_captured_record_carries_the_normalised_totals_line` kills the call, with its own mutant row |
| **N2** | `arena_version` leaves the header as it left the digest | nothing — it was a `param` no reader reads | nothing is owed: §5's argument covers both, and the provenance it was offering is the pilot's governing revision |
| **N4** | INVARIANT 10's evidence names SPRT-path BEHAVIOUR, not files | rev 4: a sentence naming one file where this package edits two | rev 5: the diff, INVARIANT 7's test, and gate 15 — unchanged; only the sentence is true now |
| **N5** | §3.1 separates detection from the refusal message | rev 4: one rule stated twice and differently | rev 5: `==` detects, a field walk reports, and the mutant names the walk |
| **N6** | `experiment_sha256` gains a test and a mutant | nothing — three digest tests all passed with it dropped | `two_captures_of_different_experiments_do_not_share_an_identity` |
| **N7** | §6 gains the TAB row it owns | §4.2 and INVARIANT 6 only | §6's table, which owns failure modes, pointing at §4.2 for the grammar |
| **N8** | §14.4 prices the declined route against gate 9 rather than the pilot | — | — (a pricing sentence, no test moves) |
| carried | the no-score rule gains a test | nothing, across three revisions | `a_totals_line_with_no_score_at_all_is_captured_as_written` |
| m8 residual | the round-trip fixture's first two fields must differ | — | the clause in §10 that makes the swap mutant able to die |

---

### 0.3 What this package decides, and what it does not

Lifted from revision 2 (`7af62e7`), whose BLOCKING 3 remedy the reviewer passed:

> **THE CLAIM IS REPLACED, NOT REPAIRED.** This package makes **no decision
> about what the score, the node counts or the provenance MEAN**. It **does**
> make decisions about capture, and they are on the face of this document
> because a licence that is false suppresses the attack rather than answering
> it.

| decision | § |
|---|---|
| WHICH positions are asked | §2 |
| WHICH engine answers, and from WHICH slot | §3 |
| WHAT is written, and in what grammar | §4 |
| WHAT counts as the same capture | §5 |
| WHAT happens when an ask fails | §6 |

And lifted, because it corrects a governing ADR line and must not be lost again:

> **AND THE SPLIT'S OWN PREMISE IS CORRECTED (D-544).** That line says every
> prior failure was an interpretation question. **It is overstated**: five
> capture findings from the first review — which seat answers, the budget's
> kind, the failure modes, the CLI grammar, and ply-versus-turn — crossed the
> split line unfixed. They are taken here.

**D-483 binds this document: mechanisms, invariants and tests only.** No measured
numbers. **No engine diff**; only `pistol-arena` changes.

### 0.4 Reading order

§12 (coldness) is the ground under §1's `newgame` and under §4's budget; a first
reader takes §1, §2, §12, then the rest in order. It sits at §12 rather than at
§4 so that every section number a reviewer has already adjudicated still names
the same section (D-547).

---

## 1. THE MECHANISM

**Pass 1 — PLAY, and it gets no new code.** Lifted from revision 1 (`5064b05`),
including the clause revision 2 deleted and then contradicted — the deletion
D-545 records as the arc's clearest failure:

> `arena --config <experiment> --out <report>` on the unmodified SPRT path, both
> engine sections naming the same committed config. Self-play is expressible
> today: **`validate_engines` refuses identical *labels*, not identical binaries
> or configs.**

That sentence is true at this revision and is the ground §3 is rebuilt on:
`crates/pistol-arena/src/validate.rs:243-250` compares `engine_a.label` with
`engine_b.label` and refuses only their equality, and `label` is a field of
`EngineSection` (`crates/pistol-arena/src/config.rs:170-193`) — while `binary`,
`binary_sha256` and `config` are free to be equal, as
`configs/arena_smoke_v0.toml` already demonstrates by being a committed
self-match on one binary and one engine config under two labels.

**Pass 2 — CAPTURE.** Lifted from revision 2, with the flag grammar completed:

> Read the report, and for each game, for each asked position (§2): send
> `newgame`, send the position, send the `go`, read to `bestmove`. Write one
> line per position.

```
arena --capture <report path> --out <capture path> --label-nodes <n>
```

**WHY VERBATIM**, lifted from revision 1 — the sentence that says why this
package is worth building before the schema is settled, and which revision 2
deleted without replacing (loss L4):

> A consumer that disagrees with WP-2.0-S can re-parse the capture without
> re-running the engine, **which is the property that makes an expensive corpus
> survive a schema mistake.**

**WHERE THE CODE LIVES, because the gate makes it a design question.** Lifted
from revision 2, with its measured count replaced by the run that produces it:

> A third mode arm plus its `USAGE` paragraph crosses [the cap]. **So pass 2
> lives in a new module `crates/pistol-arena/src/capture.rs`**, and
> `bin/arena.rs` gains only a `Mode` variant, a dispatch pattern and a call. **If
> the `USAGE` text still carries it over, the `USAGE` constant moves to its own
> module** — a mechanical extraction that adds no behaviour and no rule-9 entry,
> which is preferable to a justification for a binary that is mostly help text.

The count is not restated here. **The run that decides it is
`tools/file_justification_check.sh`, CI gate 17**, whose cap is the number a
script compares against; `crates/pistol-arena/src/bin/arena.rs` is measured by
that gate and by nothing this document asserts. Two consequences the design
fixes rather than leaves open:

- The extraction target is `crates/pistol-arena/src/usage.rs`. **The binary is a
  separate crate from the library**, so the constant becomes `pub` and not
  `pub(crate)`, and a `pub` item takes a `///` doc under this project's style
  rule. `crates/pistol-arena/src/lib.rs:47-69` is a plain `pub mod` list and the
  binary already reaches into it (`crates/pistol-arena/src/bin/arena.rs:10-13`),
  so a new `pub mod usage;` is reachable exactly as the others are.
- The mode arm is added to the existing match at
  `crates/pistol-arena/src/bin/arena.rs:82-100`, and inherits the exclusive
  output claim by structure: `outpath::claim` is called at
  `crates/pistol-arena/src/bin/arena.rs:103`, **before** the mode match, so a
  third arm needs no O_EXCL construction of its own (D-200).
- **Two pieces of text move with the arm and are named because they are content,
  not formatting.** The `USAGE` constant gains a `--capture` paragraph in the
  shape of the two beside it — the flags in order, and what each refuses — and
  the fallback refusal at `crates/pistol-arena/src/bin/arena.rs:94-99`, which
  today enumerates the two existing modes, gains the third. A binary whose help
  text does not mention a mode it has is a binary whose help text is wrong.

---

## 2. WHICH POSITIONS ARE ASKED — decision 11, TAKEN

Lifted from revision 2, whose three limbs the rev-2 review verified against the
tree and passed:

> **Per TURN, not per ply.** The grounds are three and they agree: game rule 3
> makes the turn the unit of play; `depth_turns` — the only depth on the wire —
> is in turns; and `PositionSpec::Start` **cannot express a mid-turn position at
> all**, so a per-ply capture would need `PositionSpec::Set` and a second
> position grammar. Revision 1 assumed this reading as a fact about the domain;
> it is a decision and it is taken here.
>
> **THE SET.** Every turn boundary of every recorded game at which the engine can
> legally be asked:
>
> - **The initial position is asked as bare `position start`**, never
>   `position start moves` — which the engine refuses by name when no turns
>   follow it. `exchange::position_line` produces the refused form for an empty
>   slice, so **pass 2 does not use it for the empty case.**
> - **A DECIDED position is never asked.** `set_position` refuses a won position,
>   and asking one would earn an `error` and no `bestmove`. **This is the
>   protocol's own precondition, not an exclusion by outcome**: the terminal
>   prefix of a won game is not a position any engine can be asked about, so it
>   is not in the set at all. Revision 1's INVARIANT 4 forbade "exclusion by
>   outcome" and would have forced the hang.
> - **Book turns and forfeited games are asked like any other**, because those
>   ARE exclusions by meaning and they belong to WP-2.0-S.

The three sites, cited: `crates/pistol-engine/src/position.rs:8-14`
(`Start` *"always names a position at a turn boundary"*, and carries no phase);
`crates/pistol-engine/src/position_token.rs:84-98` (bare `start` parses to an
empty move list, `start moves` with nothing after it is refused by name);
`crates/pistol-arena/src/exchange.rs:154-161` (`position_line` emits the refused
form for an empty slice); `crates/pistol-engine/src/position.rs:68-73` (a won
position is refused because *"there is no move to ask this engine for (rule
4)"*).

**THE RANGE, STATED RATHER THAN IMPLIED** — the rev-2 review's MINOR G, and the
first thing an implementer needs. For a game whose report records `len` turns,
the asked prefixes are `k` from zero to `len` inclusive, **less `k = len` when
the last recorded turn wins**. Nothing else can be excluded, and nothing else
needs to be.

**WHY ONLY THE LAST PREFIX CAN EVER BE DECIDED**, restoring revision 1's
paragraph (loss L3) and extending it to the decidedness question (MINOR K):

> **`transcript::read` legality-checks every game through `pistol-core` at read
> time**, refusing the whole report on an illegal turn or on moves after a win.
> So **every move list pass 2 walks is a guaranteed legal prefix before pass 2
> exists** — it cannot be handed something that panics.

`crates/pistol-arena/src/transcript.rs:359-379` is that check: it replays each
move list through `GameState::make_turn` and refuses a game in which a winning
turn is followed by any further recorded turn. **So a decided prefix can only be
the last one**, and pass 2 determines which by the same means, replaying the
recorded list through `pistol-core` — the only judge of legality in this
workspace (rule 2) — and testing the final outcome. `Outcome` has exactly the two
variants `Ongoing` and `Win` (`crates/pistol-core/src/turn.rs:34-45`), so the
test is one comparison on data pass 2 already holds.

**INVARIANT 1** pins the set; **INVARIANT 2** pins that no asked position is
decided.

---

## 3. WHICH ENGINE ANSWERS, AND FROM WHICH SLOT

### 3.1 The refusal, rebuilt — revision 2's BLOCKING A

**Revision 2 refused a report whose two engine SECTIONS differ.** Every report
this pipeline can produce has sections that differ, always, at `label`, so that
mechanism refused one hundred per cent of its inputs. Three independent sites
make the labels differ and the third cannot be relaxed at all:

- `crates/pistol-arena/src/validate.rs:243-250` refuses a config whose two
  labels are equal, *"the two sides must be told apart in the report"*.
- `crates/pistol-arena/src/transcript.rs:189-194` refuses a report whose two
  seats carry one label, *"so no game in it can be attributed to a seat at
  all"*.
- `crates/pistol-arena/src/transcript.rs:289-300` **attributes every game's two
  seats by label**, refusing a game whose `p1`/`p2` are not the report's own two
  labels. Identical labels would make every game unattributable.

**THE COMPARISON IS ONE TYPE TO THE LEFT.** `EngineIdentity`
(`crates/pistol-arena/src/identity.rs:11-22`) carries `id_lines`,
`binary_sha256`, `config_sha256` and `weights_sha256` — **and no label** — and
`Transcript` holds one per slot (`crates/pistol-arena/src/transcript.rs:32-34`).
It derives `PartialEq` at `crates/pistol-arena/src/identity.rs:12`, so
**detection is `identities[0] == identities[1]` and needs no new predicate; the
REFUSAL MESSAGE then walks the four fields to name the one that differed.** The
two are separate and revision 4 stated them as one, which mattered because a
derived `PartialEq` cannot name a field and the mutant table registers a
comparison with separable limbs.

**MECHANISM.** Pass 2 **refuses, by name, a report whose two seats do not attest
the same engine** — the same `id_lines`, `binary_sha256`, `config_sha256` and
`weights_sha256`. A self-play report has two labels and one identity, which is
exactly what `configs/arena_smoke_v0.toml` writes today; an A-versus-B report has
two identities and is refused.

**AND THE EQUALITY HAS A PRECONDITION, WHICH IS STATED HERE RATHER THAN
DISCOVERED IN A PILOT.** `id_lines` is the handshake verbatim
(`crates/pistol-arena/src/identity.rs:74-82` puts `spoken.lines` in), and the
engine's first id line is the config path **as spelled on its command line** —
`format!("config {}", path.display())`
(`crates/pistol-cli/src/bin/pistol.rs:142-166`) — while `config_sha256` is the
file's CONTENT (`crates/pistol-arena/src/identity.rs:84`). **So two seats naming
one config file by two different path strings attest different identities and are
refused.** The report's two engine sections must therefore spell `config`
identically, which `configs/arena_smoke_v0.toml` already does
(`configs/arena_smoke_v0.toml:82` and `:88`), and **the refusal names the field
that differed**, because a refusal that only said "different engines" would be
unreadable on two sections that name one file.

**IT FAILS SAFE AND THAT IS WHY THE MECHANISM IS UNCHANGED BY IT.** Two DIFFERENT
engines can never compare equal — `binary_sha256` and `config_sha256` are content
(`crates/pistol-arena/src/identity.rs:56-87`) — so the refusal cannot admit a
two-teacher corpus. The only reachable error is a false refusal, loud and by
name, which is the direction hard rule 3 wants. **The in-crate tests cannot see
this**, because the stub identifies with one weights line only
(`crates/pistol-arena/src/bin/stub_engine.rs:262-266`) and emits no `config` id
line; it is pinned instead by the pilot's own report, whose two sections this
design requires to spell one path. **What the refusal forecloses is a corpus whose
labels came from two different teachers without saying so**, while
`capture_sha256` attested one experiment.

**WHICH SLOT ANSWERS: slot zero.** Open since the first review's MAJOR 5 and
answered here. `Transcript::engines[0]` and `identities[0]` are engine A
(`crates/pistol-arena/src/transcript.rs:29-34`). Once the two identities are
attested equal, the slots name **the same engine by content**, so the choice is
a choice of spelling and not of teacher — and it is written down anyway, because
an unstated choice is one the implementer makes and the capture cannot report.
Every position of every game is asked on slot zero's channel.

### 3.2 What it verifies, lifted verbatim from revision 2 (rev-2 review MAJOR 3, APPLIED)

> **AND IT VERIFIES WHAT IT SPAWNED.** The report carries an `EngineIdentity` per
> slot, captured by the original run. Pass 2 spawns its engine and verifies
> against that identity the way the arena's own replay does, so a capture cannot
> silently be taken from a rebuilt binary. **INVARIANT 3.**

The function is `replay::verify_engines`
(`crates/pistol-arena/src/replay.rs:216-241`), which re-captures each slot's
identity through `identity::capture` and compares it against the report's own,
*"not 'the engine has not changed since this pass started' but 'the engine is the
one the report attests'"* (D-252). Pass 2 calls it before spawning any channel,
exactly as `replay_pass` does at
`crates/pistol-arena/src/bin/arena.rs:173-174`. It verifies both slots even
though only slot zero is asked, because the report attests both and a drifted
slot one means the report is not the run it says it is.

---

## 4. WHAT IS WRITTEN, AND THE ONE NORMALISATION

**One line per asked position**, carrying: the position as sent; the `info
totals` line **as the engine wrote it, less the wall-clock fields**; the
`bestmove` line as the engine wrote it; and the game and turn indices.

### 4.1 The normalisation, lifted verbatim from revision 2

The rev-2 review attacked this paragraph on four routes, closed all four, and
called it *"the best-verified paragraph in the document"*:

> **THE NORMALISATION IS THE PROJECT'S OWN AND IS NOT A NEW DECISION.** ` nps <n>
> time <n>` is removed, by exactly the rule `tools/determinism.sh` states and
> gate 9 enforces: *"`nps` and `time` are the only fields two runs may disagree
> about."* **This is what makes INVARIANT 6 achievable at all**, and it costs no
> label — `nps` and `time` are facts about the machine, not about the position.
>
> **Nothing else is touched.** No field is reordered, renamed, dropped or
> combined. `nodes` stays the sum the engine printed; the score keeps whichever
> of its three spellings it arrived in; the solver fields appear exactly when the
> engine printed them. **Those are the meanings WP-2.0-S decides, and this
> package still decides none of them.**

The rule is at `tools/determinism.sh:153-154`; the fields it names are declared
*"a measurement of the machine, not of the search"* at
`crates/pistol-cli/src/report.rs:15-18`; and the reason the expression matches
whether or not the solver ran is that the solver block is interpolated between
`nodes` and `nps` (`crates/pistol-cli/src/report.rs:62-84`), leaving `nps` and
`time` adjacent in both spellings. **The invariant this revision adds is
INVARIANT 6**, which revision 2 had in prose and in no invariant at all (rev-2
MAJOR B): every captured line is the engine's own bytes less those two fields,
and the `bestmove` line is untouched.

### 4.2 The record's grammar — rev-2 MAJOR E, and it is decided here

Revision 2 named the sink and left the record's shape to the implementer while
INVARIANT 6 pinned its bytes. **Five fields, in this order, separated by a single
TAB:**

| # | field |
|---|---|
| 1 | the game index, as the report spells it |
| 2 | the prefix length `k` that was asked |
| 3 | the `position` line as sent |
| 4 | the normalised `info totals` line |
| 5 | the `bestmove` line |

**WHY A TAB AND NOT A SPACE.** Three of the five fields are whole
whitespace-bearing lines and one of them ends in a variable-length `pv`, so the
record has no fixed arity under a space. This crate has been bitten by exactly
that: `crates/pistol-arena/src/transcript.rs:124-131` refuses a path containing
whitespace *"because the format is whitespace-delimited and does not quote"*.
A TAB restores fixed arity because **no field can contain one**: every field is
built by a `pistol-cli` formatter that joins with single spaces —
`exchange::position_line` (`crates/pistol-arena/src/exchange.rs:154-161`),
`render_info` (`crates/pistol-cli/src/report.rs:82-97`) and `bestmove_line`
(`crates/pistol-cli/src/report.rs:106-108`) — over turn tokens, decimal numbers
and the three score spellings.

**AND THE CAPTURE DOES NOT ASSUME IT.** Field 4 and field 5 are **the engine's
own bytes off the channel**, and §3's refusal requires the two seats to attest
ONE engine rather than to be `pistol` — a third-party speaker satisfying the
handshake could emit anything, and `Channel` does not sanitise (it does
`from_utf8_lossy` then trims only a trailing run of newline and carriage return,
`crates/pistol-arena/src/channel.rs:96-106`). **So the refusal is load-bearing,
not belt-and-braces:** pass 2 refuses the run by name, naming the game and turn,
when a field it is about to write carries a TAB (hard rule 3).

**THIS IS A WRITE-SIDE REFUSAL AND IT IS PINNED AS ONE** — INVARIANT 6, not
INVARIANT 11. On READ a TAB inside a field is indistinguishable from an extra
field, so the loader's own guard is the arity check and nothing finer, and saying
otherwise would register a loader behaviour no loader can have. **The test is a
unit test over the record-writing function with a synthetic TAB-bearing field**,
which needs no engine at all: no engine in this tree can emit one, so a test
driven by an engine could never fail.

### 4.3 The file's shape, lifted from revision 2 (rev-2 review MAJOR 4, applied half)

> **THE FILE'S SHAPE.** `pistol_cli::corpus::emit::Fixture`: a header of `param`
> and `derived` lines, a body of one record per line, and the in-band
> `body_sha256` that type appends. **INVARIANT 6 pins the file byte-for-byte, so
> the shape is specified rather than left to the implementer** — which revision 1
> did not do.

The type is at `crates/pistol-cli/src/corpus/emit.rs:12-100`, reachable from
`pistol-arena` without any manifest change (`crates/pistol-cli/src/lib.rs:36`,
`crates/pistol-cli/src/corpus/mod.rs:6`). The header carries, as `param`, the
capture format version, the source report's `experiment_sha256` and
`source_sha256`, and the label `go` line; and as `derived`, the `capture_sha256`
of §5 and the counts of games and records —
`param` and `derived` being the type's own distinction between a choice and a
measurement (`crates/pistol-cli/src/corpus/emit.rs:36-44`).

**THE LOADER.** The capture is read back by `capture::read`, shaped like
`transcript::read` (`crates/pistol-arena/src/transcript.rs:135-209`): named
refusals, and **the whole file refused rather than partially read**. It verifies
the body against the digest the header claims, using the type's own
`claimed_body_digest` and `body_of`
(`crates/pistol-cli/src/corpus/emit.rs:102-118`), and refuses a record whose TAB
count is wrong **or any of whose five fields is empty**. No field of this record
can legitimately be empty — the first two are decimals, the third is at minimum
`position start`, and the last two are non-empty engine lines — so an empty field
is always an error, and a fixed arity makes it distinguishable from a missing one
(a record short of a field carries too few TABs; a record with an empty field
carries the right number). This is the *"documented, versioned schema with a loader test"*
the standing dispatch's requirement 2 asks for, for the capture record;
**INVARIANT 11** pins it.

### 4.4 The source, lifted verbatim from revision 2 (rev-2 review MAJOR 10, APPLIED)

> **THE SOURCE IS NAMED ON THE FACE OF THE FILE.** The header carries the source
> report's `experiment_sha256` **and** its `source_sha256`, so WP-2.0-S can find
> the report that holds the game outcomes and the forfeit flags this capture does
> not carry. Without it the outcome would be unrecoverable from the capture
> alone, and requirement 2 would fall between the two packages.

---

## 5. WHAT COUNTS AS THE SAME CAPTURE

`capture_sha256` over the canonical concatenation of **exactly three inputs**,
one per line:

| input | why it is in |
|---|---|
| the capture format version | this package's own record grammar |
| the source report's `experiment_sha256` | what was played |
| the label `go` line | what was asked |

**AND `arena_version` IS CARRIED NOWHERE — NOT IN THE DIGEST AND NOT IN THE
HEADER.** Revision 4 removed it from the digest and left §4.3 writing it into the
header as a `param`, so the document answered one question twice and differently.
It is out of both, for the one argument below.

**THE INSTRUMENT IS NOT A DIGEST INPUT, WHICH REVERSES REVISION 3 AND SAYS WHY.**
Revision 3 added `arena_version` as *"the INSTRUMENT"* and argued that without it
*"the only thing standing between [two captures] is a hand-maintained format
version"*. **That remedy was its own diagnosis.** `arena_version` is written from
`env!("CARGO_PKG_VERSION")` (`crates/pistol-arena/src/report.rs:130`) over a
workspace version that **has never moved**: `git log -p --follow -- Cargo.toml`
shows one `version = "0.0.1"`, from the first commit. A second hand-maintained
number buys no protection the first did not, and the version of a package is not
a revision of anything. **Two hand-maintained numbers are worse than one**,
because a reader takes the second for a mechanism.

**WHAT DISCHARGES `docs/process.md`'s "Instrument governing revision" IS A
REVISION, AND IT LIVES WHERE THAT RULE PUTS IT.** The rule asks that an artefact
producing a registered number be *"named in the pre-registration WITH ITS
REVISION"*. **So the pilot's pre-registration names pass 2's own commit SHA**,
which is a revision, moves whenever pass 2's behaviour moves, and is checkable —
none of which a package version is. Revision 3 cited that rule for something it
does not say.

**WHAT THE DIGEST THEREFORE DOES NOT PROTECT AGAINST, stated because a digest
oversold is worse than none.** A change to pass 2's normalisation or record
grammar, with the capture format version left unbumped by hand, produces a
DIFFERENT capture under the SAME `capture_sha256`. The format version is the one
hand-maintained guard, `a_capture_identity_moves_when_the_format_version_moves`
pins that it is an input at all, and **the governing revision in the pilot's
pre-registration is what actually binds a capture to the code that made it.**

**THE ENGINE IDENTITY IS OUT, and this is rev-2 MAJOR 5's second limb.**
Revision 2 added *"the engine identity pass 2 verified"* on top of
`experiment_sha256`. `experiment_digest` already closes over each engine's
`binary_sha256`, `config_sha256` and `weights_sha256`
(`crates/pistol-arena/src/report.rs:61-74`), so the second spelling was the same
fact twice — D-423's class inside a digest — with no canonical form of its own.
It is dropped.

**Not `source_sha256`**, lifted from revision 2, whose ground the rev-2 review
re-attacked and could not break:

> which digests the whole report file including its timing block: two captures
> over reports of one experiment taken on different days would otherwise differ
> for a reason that changes no answer. `source_sha256` is still **recorded**
> (§4) — it is provenance, not identity.

**Nothing about sampling is in it, because this package samples nothing and takes
no seed.** WP-2.0-S extends the digest when it adds a sampling rule. **INVARIANT
12** pins the three inputs, and the digest is computed by a **pure function over
them** so that a test can vary each one in turn — including the format version,
which revision 3 left as the one input with neither test nor mutant.

---

## 6. WHAT HAPPENS WHEN AN ASK FAILS

Lifted from revision 2 (rev-2 review MAJOR 1, APPLIED), with two rows added for
the two channel outcomes the table did not name (MINOR H):

| condition | pass 2's answer |
|---|---|
| the report is unreadable, or its two seats attest different engines | **refuse the run**, by name, before spawning anything |
| the spawned engine's identity does not match the report's | **refuse the run**, by name |
| an ask returns `error` | **refuse the run**, by name, naming the game and turn — it means §2's set is wrong, and a capture with a hole is worse than none |
| an ask returns nothing before the watchdog | **refuse the run**, by name |
| **the engine closes its pipe** (`Received::Closed`) | **refuse the run**, by name, naming the game and turn |
| **the engine writes an overlong non-line** (`Received::Overlong`) | **refuse the run**, by name, naming the game and turn |
| **the closing totals line is not recognised as one** (§8) | **refuse the run**, by name, naming the game and turn |
| **a field about to be written carries a TAB** (§4.2 owns the grammar) | **refuse the run**, by name, naming the game and turn |
| the totals line carries no score at all | **capture it as written.** The score's presence is a meaning question and belongs to WP-2.0-S |

> **Every failure is a refusal of the whole run and none is a skip**, because a
> capture that silently omits positions is a corpus whose gaps are invisible to
> its consumer.

The two added rows are the other two variants of `Received`
(`crates/pistol-arena/src/channel.rs:24-34`); the watchdog is the report's own
`hang_timeout_ms`, which `Transcript` carries for exactly this reason
(`crates/pistol-arena/src/transcript.rs:43-45`). **Pass 2 maps every one of these
to a refusal and never to a forfeit.** `exchange::ask` classifies them as
forfeits (`crates/pistol-arena/src/exchange.rs:44-89`) because it is refereeing a
game; pass 2 plays no game, and *"forfeit"* would name nothing. The reasoning is
`replay::run`'s own, which this package follows rather than restates: *"a
criterion over SOME of a report's games is a criterion over a sample nobody
registered"* (`crates/pistol-arena/src/replay.rs:16-19`).

**INVARIANT 9** pins that no position is silently skipped.

---

## 7. THE LABEL BUDGET'S KIND, AND ITS SPELLING

Lifted from revision 2 (rev-2 review MAJOR 8, substance taken):

> **`nodes`, and never `movetime_ms`.** The arena already refuses a movetime
> budget in the one place it validates — *"the one refusal this crate exists to
> make loudly"* — and the reason applies with more force here: a wall-clock
> budget makes a label a fact about the machine, so INVARIANT 6 could never hold.
> **The VALUE is a number and belongs to the pilot's pre-registration** (D-483);
> the KIND is a mechanism and belongs here.

The refusal it quotes is at `crates/pistol-arena/src/validate.rs:39-45`, and the
crate refuses a movetime budget in two further places:
`crates/pistol-arena/src/config.rs:114-134` (`BudgetSection::go_line` returns
`None` for `MovetimeMs`) and `crates/pistol-arena/src/bin/arena.rs:226-229`,
whose `unreachable!` records that validation has already fired.

**AND THE READER OF A SOURCE REPORT IS STRICTER THAN "NOT MOVETIME", WHICH
REVISION 3 GOT WRONG AND WHICH CHANGES A DESIGN DECISION.**
`crates/pistol-arena/src/transcript.rs:164-170` tests `kind != "nodes"` and
refuses **every** budget kind but `nodes` — `depth_turns` included — because
*"only a `nodes` budget replays: the whole premise is that a re-driven engine
answers what it answered"*. Revision 3 described that span as *"a movetime source
report is refused by name"*, and §14 was written on that description; the section
it produced could not execute, which is this revision's BLOCKING.

**THE CONSEQUENCE IS A REQUIREMENT ON PASS 1 AND IT IS RECORDED AS ONE.**
**Pass 1's arena experiment config must carry a `nodes` budget**, because pass 2
reads its report through `transcript::read` and a report at any other kind is
refused before a single position is asked. This is not a preference of this
design: it is inherited, it agrees with the dispatch's own *"games at the
standing"* node budget, and it means **no `depth_turns` report in this repository
can be a capture input** — which is exactly what `configs/arena_smoke_v0.toml`
is (`configs/arena_smoke_v0.toml:54-58`).

**THE EDIT, and it is MAJOR 8's unapplied half.** Two things revision 2 left to
the implementer:

- **The `go` line is built by `BudgetSection::go_line`**
  (`crates/pistol-arena/src/config.rs:120-134`) over `BudgetSection::Nodes`, and
  is never formatted a third time by hand. **INVARIANT 4's `go` is that line.**
- **The flag admits no other kind.** `--label-nodes <n>` takes a node count and
  there is no spelling of a movetime label budget to refuse — which is stronger
  than refusing one, and is why the review's proposed
  `a_movetime_label_budget_is_refused_by_name` has no site in this package and is
  **deliberately not registered**. What is registered in its place is the pair
  that can fail: `the_label_go_line_is_the_one_budget_section_spells`, and
  `a_capture_over_a_report_whose_budget_is_not_nodes_is_refused_by_name`, which
  pins the refusal pass 2 inherits from `transcript::read` **and is named for what
  that refusal actually is** — revision 3 named it for movetime, which is the
  mischaracterisation above wearing a test name. **This is a departure from a
  review's own FIX and is named as one**, because a test that cannot fail is worse
  than an absent one.
- The node count's **spelling** is validated and not merely its value, by the
  rule `workers_of` already states for the other command-line number
  (`crates/pistol-arena/src/bin/arena.rs:124-143`): a count this program will not
  echo back is refused, because it would otherwise land in a header describing a
  run nobody can reproduce by copying the line back.

---

## 8. THE `totals_of` WIDENING

Lifted from revision 2 (rev-2 review MAJOR 6, APPLIED), with its one unapplied
consequence taken:

> `exchange::totals_of` rises to `pub(crate)` and **gains nothing in this
> package.**
>
> **The visibility change is for WP-2.0-S**, so that package adds fields to one
> parser instead of writing a second and inheriting row (b)'s kill condition.
> `clippy::redundant_pub_crate` is a nursery lint and gate 4 denies only
> `clippy::all`, so the change is not gate-rejected. **INVARIANT 7** pins that it
> alters no output.

**THE EDIT — rev-2 MAJOR 7, and it is the half revision 2 sharpened rather than
fixed.** Revision 2 said pass 2 does not call `totals_of` at all. But pass 2 must
still tell `info totals …` from `info …`, and the totals marker exists for
exactly that reason (`crates/pistol-cli/src/report.rs:20-29`, D-80) — so a pass 2
that did not call it would ship **a widening with no consumer and a second
recogniser of the same marker, in one crate**, which is a small copy of the
duplication row (b) was killed for.

**So pass 2 identifies the totals line THROUGH `totals_of`**
(`crates/pistol-arena/src/exchange.rs:169-188`), testing `is_some()` and then
capturing the line's own bytes; the parsed triple is discarded, which is what
*"does not use it"* was reaching for. One recogniser, one parser, and the
widening has a consumer from the day it lands.

**One consequence, stated because it is a behaviour and not a detail:**
`totals_of` is a `?`-chain over `nodes`, `time` and `depth_turns`, so a malformed
totals line is not recognised as one. Such a line is then read as an ordinary
`info` line, the search closes with no totals line captured, and §6's rule fires:
**the run is refused by name.** That is the loud answer, and it is the one hard
rule 3 asks for.

**THE MUTANT REVISION 2 REMOVED STAYS REMOVED**, and its replacement is a
mutation this package's shipped code can take. Revision 1 registered *"a
`totals_of` lookup made load-bearing"*, which is a no-op because all three
lookups already carry `?`. The mutation that tests INVARIANT 7 is **a fourth
load-bearing lookup added to `totals_of`** — precisely branch B's registered
hazard, the `?`-chain that would suppress `compute.add`
(`crates/pistol-arena/src/exchange.rs:76-79`) and zero the SPRT report's node
counts. It is registered here because it is the guard WP-2.0-S inherits when it
adds `score` and `pv` as non-fatal `Option`s (D-542).

---

## 9. INVARIANTS

1. **The asked set is every turn boundary of every recorded game at which the
   engine can legally be asked**, book turns and forfeited games included; the
   prefixes are `k` from zero to `len`, less `k = len` when the last recorded
   turn wins.
2. **No asked position is decided**, and the initial position is asked as bare
   `position start`.
3. **Pass 2 refuses a report whose two seats do not attest the same engine
   identity, spawns slot zero, and verifies the engines it spawns against the
   identities the report recorded.**
4. **Every label `go` is preceded by a `newgame` on that channel**, no label `go`
   follows another without one, and the `go` line is the one
   `BudgetSection::go_line` spells.
5. **Pass 2 never plays a move.** Every position it sends is a prefix of the move
   list the report recorded.
6. **Every captured line is the engine's own bytes**, less ` nps <n> time <n>` on
   the totals line; no other field is reordered, renamed, dropped or combined,
   the `bestmove` line is untouched, and **a field carrying a TAB refuses the run
   before it is written** (§4.2).
7. **No behaviour rides along with `totals_of`'s visibility change**: the SPRT
   path still bills each game's compute from the totals line.
8. **A re-run of pass 2 over one report at one label budget produces a
   byte-identical capture file**, wall-clock fields having been normalised out by
   gate 9's own rule.
9. **Any failure refuses the whole run**; no position is silently skipped.
10. **Pass 1 is unmodified.**
11. **A capture file round-trips through its own loader FIELD BY FIELD**, and one
    whose body digest is wrong, whose record arity is wrong, or any of whose
    fields is empty is refused by name.
12. **The capture identity is a function of exactly three inputs**: the capture
    format version, the source report's `experiment_sha256`, and the label `go`
    line.

**TWO THINGS NO TEST PINS, BOTH DECLARED HERE RATHER THAN LEFT FOR A REVIEWER TO
FIND — and revision 4 had a third it did not declare, which §0.2b's N1 row
closes.**

**INVARIANT 10 (pass 1 is unmodified).** No unit test can compare the SPRT path's
output against a build that no longer exists. Its evidence is the diff: **the only
file whose SPRT-path BEHAVIOUR this package changes is
`crates/pistol-arena/src/exchange.rs`, and only its one visibility keyword.**
`crates/pistol-arena/src/bin/arena.rs` is edited too — a `Mode` variant, a
dispatch arm, a call, the `USAGE` extraction and the fallback refusal's mode list
(§1) — and it is the binary that runs the SPRT, which is why the sentence says
BEHAVIOUR and not files: §1 shows the two existing arms and `outpath::claim`
untouched, so a third arm beside them changes no path pass 1 takes. Revision 4
said "the only file", which was false as written and which m14's own remedy
widened. Plus CI gate 15, which runs the SPRT path end to end and compares two
runs' verdict blocks byte for byte (`tools/arena_smoke.sh:22-27`). **Revision 3 also leaned on
INVARIANT 7's test here and that leg is withdrawn**: read as a cross-build
comparison it was the very thing this paragraph calls impossible, so INVARIANT 7
is restated above as what a same-build test CAN falsify — that the compute is
still billed — and §10 names what it asserts.

**INVARIANT 3's limb "spawns slot zero".** No test pins it, **and the reason is
the same reason the choice is safe**: once the two identities are attested equal
the slots name one engine by content, so no observation can distinguish an ask on
slot zero from one on slot one. It is written down so the implementer does not
choose, not because a test could tell. Registering one would be registering a
test that cannot fail.

---

## 10. TESTS AND MUTANTS

Every test names the invariant or the section rule it pins. **Where a test is
driven by something other than the arena's stub, the driver is named**, because
the vacuity this arc has paid for four times comes from a test whose driver
cannot produce the thing it is testing.

| test | pins | driver, where it is not the stub |
|---|---|---|
| `the_asked_set_is_every_legal_turn_boundary` | 1 | |
| `a_book_turns_position_is_captured_like_any_other` | 1 | |
| `a_forfeited_games_positions_are_captured_like_any_other` | 1 | |
| `the_initial_position_is_asked_without_a_moves_keyword` | 2 | |
| `a_decided_terminal_position_is_never_asked` | 2 | |
| `a_report_whose_seats_attest_different_engines_is_refused_by_name` | 3 | |
| `a_self_play_report_whose_seats_carry_distinct_labels_is_accepted` | 3 | |
| `two_identities_differing_only_in_an_id_line_are_refused_naming_that_line` | 3 | **unit, over the identity comparison** — the stub emits no `config` id line, so §3's precondition is unreachable through an engine |
| `a_respawned_engine_that_does_not_match_the_report_is_refused` | 3 | |
| `every_label_go_is_preceded_by_a_newgame` | 4 | **the stub's new `demands_newgame_per_ask` behaviour** (below) |
| `the_label_go_line_is_the_one_budget_section_spells` | 4 | |
| `every_captured_position_is_a_prefix_of_the_reports_own_move_list` | 5 | |
| `a_captured_totals_line_keeps_every_field_but_nps_and_time` | 6 | **unit, synthetic totals line** |
| `the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line` | 6 | **unit, synthetic** — no engine in this package can emit the solver spelling (§14.1) |
| `a_captured_record_carries_the_normalised_totals_line` | 6 | stub, end to end — **the only test that dies when the CALL is deleted** (§14.1) |
| `a_totals_line_with_no_score_at_all_is_captured_as_written` | 6 | **unit, synthetic** — §6's last row, unpinned across three revisions |
| `a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote` | 6 | |
| `a_captured_field_containing_a_tab_refuses_the_run_by_name` | 6 | **unit, over the record writer** — no engine in this tree can emit a TAB |
| `the_sprt_reports_per_game_node_counts_are_billed_from_the_totals_line` | 7 | |
| `two_totals_lines_differing_only_in_nps_and_time_normalise_equal` | 8 | **unit, synthetic, both spellings** |
| `a_rerun_over_one_report_is_byte_identical` | 8 | stub; **a shape test, declared in §14.3** |
| `an_error_answer_refuses_the_run_and_names_the_game_and_turn` | 9 | |
| `a_report_pass_two_cannot_read_is_refused_by_name` | 9 | |
| `an_engine_that_stops_answering_refuses_the_run_at_the_watchdog` | 9 | |
| `an_engine_that_closes_its_pipe_refuses_the_run_by_name` | 9 | |
| `an_unrecognised_totals_line_refuses_the_run_and_names_the_game_and_turn` | 9 | |
| `a_capture_file_round_trips_through_its_own_loader_field_by_field` | 11 | |
| `a_capture_whose_body_digest_is_wrong_is_refused_by_name` | 11 | |
| `a_capture_record_with_the_wrong_field_count_is_refused_by_name` | 11 | |
| `a_capture_record_with_an_empty_field_is_refused_by_name` | 11 | |
| `two_reports_of_one_experiment_share_a_capture_identity` | 12 | |
| `two_captures_of_different_experiments_do_not_share_an_identity` | 12 | |
| `a_capture_identity_moves_when_the_format_version_moves` | 12 | |
| `a_capture_identity_moves_when_the_label_budget_moves` | 12 | |
| `a_capture_over_a_report_whose_budget_is_not_nodes_is_refused_by_name` | §7 | |
| `a_label_node_count_spelled_a_way_this_program_will_not_echo_back_is_refused` | §7 | |
| `a_capture_prints_a_manifest_row_naming_its_digests` | §13 | |

**THE ONE INSTRUMENT CHANGE THIS PACKAGE MAKES, AND WHY IT IS NOT OPTIONAL.**
`crates/pistol-arena/src/bin/stub_engine.rs` gains a behaviour
`demands_newgame_per_ask`, beside the existing `demands_newgame`. **The existing
one cannot witness INVARIANT 4**: its `told_new_game` flag is set on `newgame`
and **never cleared** (`crates/pistol-arena/src/bin/stub_engine.rs:267-289`), so
it observes a `newgame` per SPAWN and pass 2 sends one per ASK on one long-lived
channel — delete every send after the first and the existing witness stays green.
The new behaviour **clears the flag on `go`**, so a `position` arriving without an
intervening `newgame` earns the same named `error` line. That is exactly the
existing behaviour's own reasoning at one finer granularity, and its doc already
records why the granularity matters: an honest engine cannot tell, and *"deleting
the send left the whole workspace green"* (D-413,
`crates/pistol-arena/src/bin/stub_engine.rs:43-53`). **Without it the package's
headline coldness mechanism is registered against a mutant that survives.**

**MUTANTS:**

| mutation | the test that dies |
|---|---|
| the `newgame` removed from pass 2's loop | `every_label_go_is_preceded_by_a_newgame` (under `demands_newgame_per_ask`) |
| the `go` line formatted by hand instead of through `BudgetSection::go_line` | `the_label_go_line_is_the_one_budget_section_spells` |
| the normalisation's FUNCTION broken | `a_captured_totals_line_keeps_every_field_but_nps_and_time` |
| **the normalisation not APPLIED on the write path** — the call deleted, the function left intact | `a_captured_record_carries_the_normalised_totals_line` |
| a score-less totals line refused instead of captured | `a_totals_line_with_no_score_at_all_is_captured_as_written` |
| the normalisation widened to strip another NON-solver field | `a_captured_totals_line_keeps_every_field_but_nps_and_time` |
| the normalisation widened to strip a SOLVER field | `the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line` |
| the `bestmove` line normalised too | `a_captured_bestmove_line_is_byte_identical_to_what_the_engine_wrote` |
| the decided-position guard removed | `a_decided_terminal_position_is_never_asked` |
| `position start moves` used for the empty case | `the_initial_position_is_asked_without_a_moves_keyword` |
| the seat check made a LABEL comparison (revision 2's own defect) | `a_self_play_report_whose_seats_carry_distinct_labels_is_accepted` |
| the seat identity check removed | `a_report_whose_seats_attest_different_engines_is_refused_by_name` |
| `id_lines` dropped from the identity comparison | `two_identities_differing_only_in_an_id_line_are_refused_naming_that_line` |
| `replay::verify_engines` not called | `a_respawned_engine_that_does_not_match_the_report_is_refused` |
| an `error` answer skipped instead of refusing | `an_error_answer_refuses_the_run_and_names_the_game_and_turn` |
| a watchdog timeout skipped instead of refusing | `an_engine_that_stops_answering_refuses_the_run_at_the_watchdog` |
| an unrecognised totals line treated as an ordinary `info` line and the run completed | `an_unrecognised_totals_line_refuses_the_run_and_names_the_game_and_turn` |
| forfeited or book positions skipped | their two tests |
| a capture record's first two fields swapped on write | `a_capture_file_round_trips_through_its_own_loader_field_by_field`, **whose fixture must give the game index and the prefix length DIFFERENT values** — equal ones make the swap invisible |
| the loader's body-digest check removed | `a_capture_whose_body_digest_is_wrong_is_refused_by_name` |
| the loader's empty-field check removed | `a_capture_record_with_an_empty_field_is_refused_by_name` |
| the write-side TAB check removed | `a_captured_field_containing_a_tab_refuses_the_run_by_name` |
| `source_sha256` used as the identity | `two_reports_of_one_experiment_share_a_capture_identity` |
| the format version dropped from the identity | `a_capture_identity_moves_when_the_format_version_moves` |
| `experiment_sha256` dropped from the identity | `two_captures_of_different_experiments_do_not_share_an_identity` |
| the label `go` line dropped from the identity | `a_capture_identity_moves_when_the_label_budget_moves` |
| the manifest row not printed | `a_capture_prints_a_manifest_row_naming_its_digests` |
| a fourth load-bearing lookup added to `totals_of` | `the_sprt_reports_per_game_node_counts_are_billed_from_the_totals_line` |

**WHY THE LAST MUTANT KILLS THAT TEST, since revision 3 named a test that could
not have died.** A fourth `?` lookup — `value("solver_nodes")?`, say — returns
`None` for every committed, solver-off config, so `totals_of` returns `None`,
`compute.add` never fires (`crates/pistol-arena/src/exchange.rs:76-79`) and the
report's per-game `nodes_a`/`nodes_b` are zero. **The test therefore asserts what
a same-build run CAN show** — that a stub-driven arena report's per-game node
counts are the counts the stub's own totals lines carry, and are not zero —
rather than a cross-build byte comparison, which INVARIANT 10's paragraph says
cannot be written. That is what revision 3's
`raising_totals_of_leaves_the_sprt_report_byte_identical` was reaching for and
did not say.

**AND ONE TEST OBLIGATION THAT IS NOT A TEST NAME**, restated from revision 2 and
given the sites it lacked — §14 answers it in full:

> The re-run test **must not be driven by the arena's stub engine alone**, whose
> `nps` and `time` are hardcoded constants: against that engine the normalisation
> is unobservable and the test passes whether or not it exists.

---

## 11. WHAT THIS PACKAGE DOES NOT DECIDE

The label budget's VALUE and the pilot's `book_v2` range (both numbers, both the
pilot's pre-registration). And **every question of MEANING** — what the score,
the node counts and the provenance mean, which positions a trainer should use,
transposition dedup, and the census-minimum rule. Those are WP-2.0-S's.

**AND THE DISPATCH'S THROUGHPUT OBLIGATION IS THE PILOT'S, NAMED HERE BECAUSE
REVISION 3 LEFT IT IN NO PACKAGE AT ALL.** The standing dispatch asks for
*"throughput expectation stated as a shape, measured in the pilot, never guessed
(D-500's class)"*. The shape is: one label `go` at the label budget per asked
position, plus one `newgame` per asked position, whose memset cost §12 names and
does not guess. **The magnitudes — games per hour and labels per hour — are
measured in the pilot and registered there**, which is where D-483 and D-500
between them put them. §12's memset is the per-position COST and is not the
throughput; revision 3 offered it as though it were.

---

## 12. COLDNESS — restored from revision 1 (`5064b05`) §4

Revision 2 deleted this section entire, and with it D-540's obligation on the
pilot. It is quoted back, with a citation added at each site it names.

> **MECHANISM.** `newgame` before every label `go`. Verified end to end by two
> reviewers: `Table::clear` is a true `fill(EMPTY)`, not the epoch bump beside
> it; `Solver::reset` rebuilds its table rather than bumping an epoch;
> `Position::reset_to` unwinds the eval and replaces the `ThreatState`; `params`
> is immutable; `census` is `None` in every shipped path; and the `PvTable` is
> per-`Run`, not a `Searcher` field. **Nothing that could carry across a position
> survives.**
>
> **COST, NOT CLAIMED FREE.** A `newgame` fills every bucket of a table whose
> size the committed seats set, so it is a memset per captured position. **The
> pilot measures it** (D-500).
>
> **WHAT THE PILOT'S PRE-REGISTRATION OWES**: D-540's second clause — a
> **fresh-process agreement criterion**, proving the construction holds by
> agreement between a pass-2 capture and the same position re-asked in a fresh
> process, and **naming the defect class it excludes**, because a criterion that
> is a property of the named defect passes vacuously (D-527).

Each site, at this revision: `crates/pistol-engine/src/instance.rs:73-76`
(`new_game` sets a fresh `GameState` and calls `Searcher::clear`);
`crates/pistol-search/src/search.rs:229-239` (`clear` reaches the table, the
heuristics and the solver); `crates/pistol-search/src/tt/mod.rs:105-112`
(`Table::clear` is `buckets.fill([EMPTY; BUCKET_ENTRIES])` with `generation` and
`used` zeroed — not the `new_generation` epoch bump immediately below it at
`:114-118`); `crates/pistol-solver/src/solver.rs:195-203` (`reset` rebuilds
`SolverTT` rather than bumping the epoch);
`crates/pistol-search/src/position.rs:55-70` (`reset_to` undoes every stone from
the eval, replaces the `ThreatState`, and re-applies from the new state);
`crates/pistol-search/src/search.rs:57-79` (`Searcher`'s fields, with `params`
never reassigned and `census` `None` in every shipped path — *"the only callers
are this crate's own tests and the `trigger_census` example"*, `:200-208`);
`crates/pistol-search/src/search.rs:253` (`self.position.reset_to(state)` at the
top of every search); and `crates/pistol-search/src/pvs.rs:104-111` (`pv` is a
field of `Run`, which is per-search, and not of `Searcher`).

**AND THE TWO-PASS SHAPE IS WHAT MAKES IT SAFE**, which is the matrix's own
ground for row (g) over row (a): because the labelling pass runs over a WRITTEN
report, no `newgame` ever lands inside a game the SPRT path is playing. Game
isolation is inherited — `seats::with_seats` spawns per game and sends
`NEW_GAME` on every fresh spawn (`crates/pistol-arena/src/seats.rs:22-59`, and
the send at `:47`).

---

## 13. THE LEDGERS AND THE CORPUS MANIFEST — requirement 5, assigned and delivered

Revision 2 assigned requirement 5 to this package and declared it already
delivered by the fixture's own in-band digest. **That claim did not survive
checking** (rev-2 MAJOR 9): an in-band digest inside an uncommitted file is
consistent with itself whatever the file holds, and hard rule 8's manifest is a
**committed** index of uncommitted artifacts. Three obligations, each with an
owner:

**(a) THE CORPUS MANIFEST IS COMMITTED, AND THE PROGRAM DOES NOT WRITE IT.**
`docs/label_corpus_manifest.md` holds one row per capture: the `capture_sha256`,
the body digest, the source report's `experiment_sha256` and `source_sha256`, the
label `go` line, and the artifact's path. **The capture mode PRINTS that row on
stdout and never writes the file**, because `pistol-arena` writes nothing inside
the repository (`crates/pistol-arena/src/lib.rs:41-45`) and hard rule 8 says so
too. The row is added in the commit that records the run — the same rule
`docs/book_v2_ledger.md:16` states for its own table. **Printing it rather than
having a human retype it is D-543's remedy applied to a ledger row**: a number a
human retypes drifts from its run. `a_capture_prints_a_manifest_row_naming_its_digests`
pins the row's shape.

**AND THIS DECISION LEAVES ONE OF THE DISPATCH'S REGISTERED MUTANTS WITHOUT A
SITE, WHICH IS NAMED HERE RATHER THAN LEFT TO BE FOUND.** The standing dispatch
registers *"ledger overwrite → append test dies"*
(`docs/experiments/wp20_dispatches.md`, Development round item 2). **No program
in this package writes a ledger**, so there is nothing to make overwrite and the
mutant has no site. **What the mutant guarded — that a row is never lost — is
carried instead by the manifest being a committed document**: a row can only be
removed by a commit that removes it, which is a diff under review, where an
overwriting program's loss would have been silent. This is a departure from
governing dispatch text rather than from a review's FIX, so it is named with more
care and not less.

**(b) THE `book_v2` LEDGER ROW TRAVELS WITH PASS 1'S CONFIG, AND BOTH BELONG TO
THE PILOT'S PRE-REGISTRATION — NOT TO THIS PACKAGE.** Revision 2 said the row was
*"the arena's existing business"*; the arena writes no ledger and reads none.
Revision 3 then gave the config to this package while giving its range to the
pilot, **which is a contradiction and this revision withdraws it**: every file
under `configs/` is validated whole by CI gate 6 (`tools/config_check.sh`), and
hard rule 1 forbids a partial document — so an arena experiment config cannot
land before its `openings_skip`, `openings_take` and `budget.value` are chosen,
and those are exactly the registered numbers D-483 puts in the pre-registration.
**The ledger's own rule says the same thing**: a row is added *"in the same commit
that adds its arena config"* (`docs/book_v2_ledger.md:16`), and the rule's subject
is *"a new pre-registration"* — the ledger's table closes by recording that a
claimant has no row *"because neither has a committed pre-registration drawing
from this book"*.

**SO: THIS PACKAGE LANDS NO ARENA EXPERIMENT CONFIG.** The pilot's
pre-registration lands `configs/arena_wp20_label_pilot.toml` and its
`docs/book_v2_ledger.md` row together, in one commit, with its numbers chosen
there. What this package owes is the CONSTRAINT that config must satisfy, which
is §7's: **its budget kind is `nodes`**, because pass 2 can read no other report.

**(c) THE CENSUS CORPUS MANIFEST IS NOT THIS ARC'S.** D-539 moved census logging
out of WP-2.0 entirely, so requirement 5's third limb is WP-2.0b's and is named
here only so a successor stops looking for it.

---

## 14. WHAT PINS THE NORMALISATION, AND WHERE THE RE-RUN RECEIPT IS TAKEN

**Revision 3's answer to this question was the BLOCKING**, and it failed for a
reason worth keeping on the page: it named ONE site — an extension to
`tools/arena_smoke.sh` — and that site cannot execute, because the report that
script writes carries a `depth_turns` budget and pass 2 refuses every non-`nodes`
report (§7). **The replacement is three sites, each doing the part it can do,
with what each cannot do stated beside it.**

### 14.1 The normalisation is pinned by UNIT TESTS over the function AND one INTEGRATION test over the call

**The unit half is the site revision 3 should have named first**, and it closes
rev-2 MAJOR C's real complaint more completely than any engine-driven test can:
the normalisation is a pure function from one line to one line, so its behaviour
is checkable without an engine, in both spellings. **The integration half is what
revision 4 spent and revision 5 restores** — a function that is right and never
called is a corpus that is not reproducible.

Registered over SYNTHETIC totals lines:

- `a_captured_totals_line_keeps_every_field_but_nps_and_time` — the non-solver
  spelling: every field of `render_info`'s output survives but those two.
- `the_normalisation_removes_only_nps_and_time_from_a_solver_bearing_line` —
  **the solver spelling**, which no engine-driven test in this package can
  produce.
- `two_totals_lines_differing_only_in_nps_and_time_normalise_equal`, in both
  spellings — which is INVARIANT 8's actual content, tested directly.

**AND ONE INTEGRATION TEST BESIDE THEM, BECAUSE THE UNIT TESTS PIN THE FUNCTION
AND NOT THE CALL.** This is revision 4's own defect and it is the reason revision
5 exists: three unit tests over a pure function are all satisfied by a pass 2
that never calls it. **`a_captured_record_carries_the_normalised_totals_line`**
drives the stub end to end and asserts that the totals field of a written capture
record does **not** carry the stub's ` nps 1 time 0`
(`crates/pistol-arena/src/bin/stub_engine.rs:120-131`) and **does** carry every
other field the stub emitted. It cannot see the solver spelling — §14.3's limit —
but it is the only test that dies when the call is deleted, and deleting a call in
a loop is the likelier mutation than breaking a tested function.

**THE TWO LIMBS TOGETHER ARE WHAT INVARIANT 6 NEEDS**: the unit tests say the
normalisation is right in both spellings, the integration test says pass 2
performs it. Revision 4 had only the first and its mutant table claimed both.

**WHY THIS IS THE ONLY WAY THE SOLVER SPELLING CAN BE REACHED, and revision 3's
bound on its own vacuity was wrong.** Revision 3 wrote that *"the vacuity is
confined to one test"*. It is not: `render_info` emits the solver block only
`if info.solver_nodes > 0` (`crates/pistol-cli/src/report.rs:62-81`), the arena's
stub sets `solver_nodes: 0` (`crates/pistol-arena/src/bin/stub_engine.rs:120-131`),
and **every committed engine config has the solver off the search path** —
`configs/gate_v0.toml:94` is `on_search_path = false`, and §1 requires pass 1's
engine sections to name a committed config. So a normalisation widened to strip
`solver_root_nodes` would have died nowhere. **Against a synthetic line it dies
immediately**, and §4.1's whole argument — that the expression matches in both
spellings because the solver block is interpolated between `nodes` and `nps` — is
executable for the first time.

### 14.2 The end-to-end re-run receipt on the real binaries is the PILOT's

The standing dispatch's requirement 4 asks for *"a re-run receipt [that] proves
byte-identical output on a small range"*, and the pilot runs the real `pistol`
through the real `arena` over a `nodes`-budget report — the only kind pass 2 can
read (§7). **That receipt is the pilot's pre-registration's and is not this
document's**, and it is where INVARIANT 8 meets an engine whose `nps` and `time`
actually move.

### 14.3 The in-crate end-to-end test is a SHAPE test, and is declared one

`a_rerun_over_one_report_is_byte_identical`, driven by the arena's stub, cannot
observe the normalisation as a DIFFERENCE BETWEEN TWO RUNS: the stub's `nps` and
`time` are hardcoded (`crates/pistol-arena/src/bin/stub_engine.rs:120-131`), so
two runs agree with or without it. **It is registered anyway, as the test that
the pass is deterministic in everything else** — record order, header, digest —
and this document says what it cannot see rather than letting an implementer
discover it. **What it cannot see, §14.1's integration test can**: the stub's
constants are useless for comparing two runs and perfectly good for asserting
that ` nps 1 time 0` is absent from a record the pass wrote.

### 14.4 Why no `tools/` change, stated as a decision rather than an omission

Neither test crate can run both programs by a route that survives
`cargo test -p pistol-arena`: `crates/pistol-arena/tests/common/mod.rs:8-11`
offers only the arena and its stub, `CARGO_BIN_EXE_pistol` is defined only under
`crates/pistol-cli/tests/` (for instance
`crates/pistol-cli/tests/determinism_tests.rs:99`), and `pistol-cli` cannot
depend on `pistol-arena` without a cargo cycle. A same-directory
`current_exe()` route to a non-`[[bin]]` target exists in this crate
(`crates/pistol-arena/tests/sprt_power_tests.rs:19-37`) and is **rejected**: it
holds only under `cargo test --workspace`, and answering `cargo test -p
pistol-arena` with a skip is what hard rule 3 forbids.

**The remaining route is `tools/arena_smoke.sh`, and this design does not take
it.** It would need a second committed arena config at a `nodes` budget and a
second arena run inside CI gate 15 — a new document on gate 6's path and a new
per-CI-run cost, against a gate whose cost is pre-registered on its own face
(`tools/arena_smoke.sh:29-33`). **What it would buy is already bought, and by a
STANDING gate rather than by the pilot's one-off receipt**: the coupling this
package depends on is that `nps` and `time` are the only fields two runs may
disagree about, and **CI gate 9 asserts exactly that on every run**, over the same
`newgame`/`position`/`go` loop shape, with the same `sed`
(`tools/determinism.sh:153-154`). Revision 4 priced the declined route against
§14.2, which is a receipt taken once; the honest comparison is against gate 9,
which runs always. **The decision keeps the matrix's own claim true**: only
`pistol-arena` is touched (`docs/experiments/matrix_wp20_shape_selection.md`
§2), which revision 3's answer would have falsified.

---

## 15. THE ADR ACT THIS PACKAGE OWES AT LANDING

Hard rule 10 wants an amendment rather than silent drift, and one is owed:
**D-544 records the verified artefact as the `label_sha256` and this design calls
it `capture_sha256`.** `/usr/bin/grep -c "capture_sha256" docs/decisions.md`
returns zero. The rename was raised as a MINOR against revision 1 and against
revision 2 and applied in neither; **it is an ADR act, not a document edit**, and
it lands with the code.
