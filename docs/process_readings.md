# Process readings — the T-bucket, on disk

Adversarial READINGS of CLAUDE.md's two pre-registration paragraphs (the dry-run rule,
D-227; the proportionality rule, D-228). A reading is a way the text, followed exactly as
written, permits a bad outcome or fails to settle a question a session will actually face.

**Why this file exists.** D-242 counts EIGHT such readings; D-235 carries five (T1–T5) and
D-247 records that **T6, T7 and T8 have no content anywhere** — archaeology was exhausted
across the repository and the session scratchpads and recovered nothing. D-247 also forbids
reconstruction: *"a plausible invention in [an append-only law log] is worse than an absent
line."* The operator's ruling was therefore to **RE-DERIVE** rather than reconstruct, and to
keep the result on disk so the next occurrence of this gap is a file that is missing rather
than a memory that is.

**The bucket's boundary (D-242).** An item belongs here if it is an AMBIGUITY — a reading the
text does not settle. Anything exploitable against RESULT INTEGRITY — where following the text
as written produces a wrong number or licenses a run that should have been refused — is
EXCLUDED from the bucket and takes an amendment immediately.

---

## 1. How the re-derivation was run

One fresh-context subagent was given, verbatim and as its ONLY material: CLAUDE.md's
governing-revision paragraph, its dry-run paragraph, its proportionality paragraph, and the
committed text of D-227 and D-228. It was instructed to read **no repository file** — in
particular not `docs/decisions.md` (which carries T1–T5), not any pre-registration, and not
CLAUDE.md itself — and to produce readings in a fixed form: statement, ground in quoted words,
bite, disposition candidates. It ran no tool.

**Its independence note is part of the record**, including the disclosure it volunteered: the
harness injected a copy of CLAUDE.md and a memory index into its context before it saw the
task, so it had seen the project instructions without opening the file. It confined the
derivation to the quoted blocks and named three places it was tempted to look further and did
not. That disclosure is why the corroboration below is worth something: a reader that had seen
the *conclusions* — T1–T5 — could not corroborate them.

**What the method bought, and it is the load-bearing result.** The re-derivation reproduced
**four of the five known readings blind**:

| re-derived | reproduces | note |
|---|---|---|
| T-R1 | **T2 (first half) + T3** | KIND is not REPRESENTATIVENESS; and the one-instance case, arrived at independently as its third bite |
| T-R2 | **T2 (second half)** | RECORDING is not CHECKING — almost word for word |
| T-R3 | *none* | **new** |
| T-R4 | *none* | **new** |
| T-R5 | **T4** | the self-judging cheapness trigger, plus new content (reconciliation, the unlegislated expensive branch) |
| T-R6 | **T5** | no consequence for disagreement — already resolved by D-245 — plus new content (independence; the unranked pair) |

A method that re-derives four known readings from the same text is a method whose two novel
readings are worth taking seriously. **It is not evidence that T-R3 and T-R4 ARE the lost
items** and no such claim is made: D-247's prohibition stands, these are re-derivations and
they SUPERSEDE the lost three rather than recovering them.

---

## 2. The three that take the vacant slots

### T6 — the instrument can change underneath identical literal commands

*The rule constrains the dry run's input and calls the commands "literal", but never fixes
WHICH ARTEFACT the command is, so a pre-registration whose registered instrument is a named
script — or a block printed in the document but executed by other means — can have its
instrument change between the dry run and the governed run with no amendment, no reopened
review, and no breach of a single word.*

**Ground.** "A pre-registration's literal commands are exercised…" and "This constrains the dry
run's input" — input is named as the thing constrained; the command's provenance and the tree
it executes against are not mentioned. The paragraph above binds "the revision that GOVERNS the
run", which reads naturally as the pre-registration's revision, and "an amendment reopens the
review, however small the diff" presupposes a diff in the document.

**Bite.** The registered command is one line naming a repository script. The dry run runs it at
tree SHA X and records its output. Two commits later the script's parsing changes in the way
this codebase has repeatedly produced — a pipeline that keeps exiting 0 while parsing something
else — and the governed run executes the identical literal line against a different instrument.
The document is byte-identical, so no amendment, so no reopened review, so the dry run that
"exercised" the commands exercised an instrument that no longer exists. The mirror case: the
document prints a pipeline and the session runs it inside a script with different shell
options; it is genuinely unsettled whether the literal command is what the document prints or
what the shell executed, and in D-227's own cited failure those two differed in exactly the way
that mattered.

**Disposition candidates.** (a) the pre-registration pins the instrument's revision (script SHA
or tree SHA) beside its cost statement, and an instrument change counts as an amendment — cheap,
and it makes "the revision that governs the run" name both revisions explicitly; (b) require
registered commands to be self-contained with no script indirection — honest, but it drags real
instruments into prose and cuts against keeping recorded numbers behind tested scripts;
(c) declare that a change to the instrument reopens the review even though the document did not
change — the right rule, but it must be said, because the amendment clause is written entirely
in terms of diffs to the document.

**STATUS: CLOSED BY D-268**, which adopts disposition (a)+(c) as one clause. Recorded here with
its full text anyway, because a reading disposed of in the same round is the one a later reader
is most likely to re-derive and think new.

### T7 — the dry run and the amendment clause have no terminating condition

*The dry run must occur before the review passes and must be written into the pre-registration;
writing it in is an amendment; an amendment reopens the review "however small the diff" — so the
text as written has no terminating condition, and the sentence that was added to defuse this
seam closes only its other half.*

**Ground.** "exercised before its review passes" + "The pre-registration records the dry-run
input and its output" + "Reviews of superseded revisions do not transfer — an amendment reopens
the review, however small the diff." D-227 confirms the seam was seen once: the "does not
consume the pre-registration's first run" sentence exists because "a reader could conclude the
dry run is that first run and that the rule contradicts itself." The same reader can conclude
that the RECORDING of the dry run is an amendment reopening the review the dry run was
performed for, and that sentence does not touch this.

**Bite.** A session dry-runs, records input and output, and submits. The reviewer observes that
the document now differs from the revision the dry run was performed against and asks whether
the dry run holds at THIS revision. Two exits: re-run and re-record, which amends again and
loops — in practice two or three wasted review cycles on a document governing a nine-second
run, itself a proportionality failure under the sibling paragraph; or declare recording-only
amendments review-exempt, inventing a class of exempt amendment the clause above expressly
denies, and setting the precedent for the next "small" diff. A quieter variant: an amendment
touching only prose — the cost statement, the agreement criterion — leaves the commands
byte-identical. The review does not transfer. Does the dry run? The text says nothing and both
answers are defensible on its face.

**Disposition candidates.** (a) state that the dry run attaches to the COMMANDS AND THE
INSTRUMENT, not to the revision: it transfers across any amendment that leaves both unchanged
and is re-run otherwise — one sentence, terminates the loop, and it makes T6's
instrument-pinning load-bearing rather than decorative; (b) state that the dry run precedes
first submission and its recording is part of the revision reviewed, so no amendment occurs —
clean, but it forbids a reviewer from demanding a re-run without a full amendment cycle;
(c) leave it and pay the cycles, which is affordable exactly once.

**STATUS: OPEN.** It is a live cost right now — the WP-1.5a pre-registration is at revision 9
and every revision since 3 has re-run its own blocks.

### T8 — the second instrument has no independence condition, and the pair is unranked

*The paragraph does not settle whether replication and a second instrument are jointly required
or are alternative answers to doubt, imposes no independence condition on the second
instrument, and — having required that the consequence of disagreement be REGISTERED — sets no
floor on what that consequence may be, so "disagreement is noted and the verdict stands"
satisfies every word while restoring the discretion the clause exists to remove.*

**Ground.** "answered by REPLICATION and by a SECOND INSTRUMENT whose agreement criterion is
registered before either runs" — conjunctive in grammar, but "doubt … is answered by" states a
sufficient remedy, and a session reading the pair as a menu is not plainly misreading;
"a SECOND INSTRUMENT", unqualified, with no condition of independence; "the pre-registration
states … what DISAGREEMENT DOES to the verdict" — that it does something is required, what it
does is not.

**Bite.** Two instruments sharing the suspect stage — two timing harnesses reading the same
instrumented counter, two counting commands built on the same `grep` idiom that miscounted in
D-221 — agree exactly and are wrong together; every word is satisfied and the agreement is
evidence of nothing but a shared dependency. Second: the consequence registered honestly in
advance is "on disagreement, report both and prefer the higher-resolution instrument."
Disagreement happens. The verdict stands. Nothing was decided after the numbers and nothing was
learned from them. Third, the unranked case, which is D-228's own arithmetic pointed the other
way: eight replications of instrument A give SD 0.457 pp while instrument B lands 3 pp off.
Replication says the measurement is stable; the second instrument says it is biased. The
paragraph names both devices and never ranks them, and a session with a shipping decision in
hand will cite the one that agrees with it.

**Disposition candidates.** (a) require the registered consequence to come from a closed set
containing at least one verdict-defeating outcome — "disagreement beyond the criterion retires
the verdict; the instrument is re-derived" — closing the null-consequence hole for the price of
a small enum in the one schema place; (b) add the independence condition: the second instrument
must not share the stage under doubt, and the pre-registration names that stage, which is what
makes agreement mean anything; (c) settle the conjunction and rank the devices explicitly —
replication answers noise, a second instrument answers bias, and disagreement defeats the
verdict however tight the replication set is.

**STATUS: OPEN.** D-245 closed T5's missing-consequence half by requiring a registered
consequence; T8 is the observation that a REGISTERED consequence can still be no consequence,
and that the second instrument was never required to be independent of the first.

---

## 3. The two graded EXCLUDED-CLASS

These are **not** bucket items. Under D-242's boundary they take an amendment immediately.

### T-R2 → T2, second half. **DISPOSITIONED BY D-269.**

*"records the dry-run input and its output" requires RECORDING and nowhere requires anyone to
hold an expectation of what that output should be, so a dry run that reproduces the D-221
failure exactly — wrong count, or no output at all — satisfies every word and the review it
gates passes on it.* Ground: there is no verb of comparison anywhere in the paragraph, no
registered expected value, and no state in which a dry run FAILS; "exercised" is the strongest
word used of the commands, and exercising is not checking. Bite: D-221's own commands.
`grep -c 'btree.*search' … > 0` writes a file named `0` and prints nothing; the faithful record
is *"input: …; output: (none)"*, and nothing makes "(none)" a failure. The substring case is
worse because it looks healthy: recording "37" is a faithful record, and only someone who
independently knows the stand-in's true count knows that 37 counted `pistol_search` too.

This is the reading D-269 lands as law. **It was re-derived blind**, which is the corroboration
D-269 rests on.

### T-R5 → T4, sharpened. **OPEN, AND OWED A RULING.**

*The proportionality paragraph's only substantive obligation is gated on "where the run is
cheap"; cheapness is decided by a cost statement the same session writes BEFORE the run, which
nothing validates and nothing reconciles afterwards; and the expensive branch is left wholly
unlegislated — so an honest but wrong forecast of expense discharges the replication and
second-instrument duty and re-licenses the single-sample-plus-derived-margin instrument the rule
exists to forbid.*

Ground: "A pre-registration STATES what its governed run COSTS" — states, not measures; "visible
on THE DOCUMENT'S OWN FACE" — the face of the document, not the record of the run; and the
prohibition on the derived margin sits INSIDE the cheap branch and appears nowhere else. Bite:
D-223's own workload with one sentence changed — documented as "≈45 min machine time, operator
attending" because the author has not run it yet; on its face not cheap; one run per binary
defended by a decomposed 3.0 pp margin is then fully compliant; the run turns out to take
seconds and no sentence reopens the verdict.

Proposed disposition, from the grading: **the cost statement is RECONCILED** — the report states
observed cost beside forecast cost, and a run that proves cheap owes the cheap-branch duties
before its verdict counts. The remedy is retroactive and affordable by construction, since
replicating a cheap run is cheap.

**Why it is not amended in the same round as T1 and T2, stated rather than assumed.** The
closure session was dispatched to land T1 and T2 and this reading is neither; it arrived from a
re-derivation the same session commissioned, and a session promoting its own subagent's grading
straight into CLAUDE.md is the shape D-242's bite-time rule exists to slow down. **The exposure
is prospective, not live**: the two documents that would be governed by it already do what it
asks — the WP-1.5a pre-registration's §6 states MEASURED costs and corrects two rows that were
labelled measured and did not reproduce, and D-236 states its cost before the run (7–9 minutes,
no operator attention) and confirms it after (433.3 s and 433.9 s). No standing verdict in this
project rests on a forecast that was never reconciled. See D-270 for the ruling this is waiting
on.

---

## 4. T-R1, recorded for completeness

*The paragraph fixes that the dry-run input be of the SAME KIND but never says who fixes the
kind or at what grain, and the two directions of error defeat opposite halves of the rule: a
kind drawn broadly enough to be convenient leaves ATTRIBUTION untested, while a kind drawn
tightly enough to guarantee attribution hands the session the governed run's answer before the
pre-registration is even reviewed.* Its third bite is T3 arrived at independently: where the
registered workload is the only instance of its kind, "never on the registered workload itself"
is unsatisfiable and the rule is either void or satisfied by silently re-describing the kind.

Graded AMBIGUITY: the tight-kind branch abuts result integrity, but the proportionality
paragraph expressly declines to catch known-answer runs, so this is a gap the text admits rather
than one it conceals. **What the text does conceal is that its own sibling rule manufactures
that gap by mandating a maximally workload-like pre-run.** T-R1 adds no slot — T2's first half
and T3 already hold it — and its sharpening is recorded here.

---

*Changes to this file follow the ADR process. A reading that closes takes an ADR line and its
STATUS here is updated by that line; a new reading takes the next free number and says which
paragraph it reads.*
