# WP-1.5b IMPL governing prompt — REVIEW (prereg-style, fresh context)

## Header

**Pinned revision.** `cc6046121e23592454cd0e4f55d038dc8857518e`.

```
$ git rev-parse HEAD            # at entry
cc6046121e23592454cd0e4f55d038dc8857518e
$ git status --porcelain        # at entry
(empty)
```

**At exit** — re-taken after every finding below was substantiated:

```
$ git rev-parse HEAD
cc6046121e23592454cd0e4f55d038dc8857518e
$ git status --porcelain | wc -l
0
```

**Match with HEAD:** the pinned revision **still matches HEAD at exit**, and the
tree is clean at both entry and exit. Every command in this report was run
against that revision. This review wrote exactly one file — itself — after the
exit check; no other file in the repository was touched.

**Document and size, MEASURED:**

```
$ wc -l -c docs/wp15b_impl_prompt.md
    291   16260 docs/wp15b_impl_prompt.md
```

Sources read in full or in the cited part, MEASURED:

```
$ wc -l -c docs/experiments/U1_gate_supersession.md \
           docs/experiments/U2_node_protocol.md \
           docs/experiments/U3_tier_t.md \
           docs/experiments/U4_soundness_instrument.md \
           docs/experiments/section_owner_table.md \
           docs/decisions.md docs/ROADMAP.md
    306   17682 docs/experiments/U1_gate_supersession.md
    840   54870 docs/experiments/U2_node_protocol.md
    897   60330 docs/experiments/U3_tier_t.md
   1976  159207 docs/experiments/U4_soundness_instrument.md
    441   24938 docs/experiments/section_owner_table.md
    747  792753 docs/decisions.md
    208   12474 docs/ROADMAP.md
```

**Context freshness.** This session is a fresh context. It authored no revision
of `docs/wp15b_impl_prompt.md`, no revision of U1–U4, `WPQ_seed.md`,
`section_owner_table.md` or `wp15b_sprt_prereg.md`, no ADR line in
`docs/decisions.md`, and no prior review of any of them. It read CLAUDE.md
first, as its dispatch required.

**Scope, as given.** `docs/wp15b_impl_prompt.md` only, verified claim by claim
against its cited **primary sources** — the ADR line's own text in
`docs/decisions.md`, the unit section's own text, the tree itself — and never
against the prompt's paraphrase or its internal consistency alone.

**Out of scope, as given.** The four units' normative content (closed under
D-351; not re-litigated). The prompt's own revision/label history (it carries
none by design; D-311's amendment retired u-rev labels project-wide, and their
absence is not charged). Gate 15 (`tools/label_consistency_check.sh`) and the CI
gates.

---

## VERDICT

**FAIL — 0 BLOCKING, 6 MAJOR, 7 MINOR**

Derived mechanically from the finding headings in `# FINDINGS` below: six
`### MAJOR` headings, seven `### MINOR` headings, no `### BLOCKING` heading.

The document's core is sound. Its two hardest sections — §4's two-step hotspot
correction and §3.1/§3.2's nine registered conditions — reproduce against their
primary sources with the numbers exact, the D-329-vs-selection-record
attribution trap avoided, and §7's verbatim list byte-identical. What fails is
narrower and load-bearing: one manufactured dependency claim, one registration
substituted without naming the ADR debt it owes, a finish policy that both
requires and excuses the same gate, half a pre-registered consequence, a
precondition on U2's IMPL demoted to ordinary debt, and a false universal about
the document's own text in a document instructed to carry none.

---

# FINDINGS

### MAJOR 1 — §5 asserts a WP-1.6 dependency no cited source carries, and the ROADMAP points the other way

**Claim reviewed** (§5, `docs/wp15b_impl_prompt.md:213–222`), the
`SearchInfo.stages` bullet, cited to `U2_node_protocol.md` §U2-M item 2:

> **`SearchInfo.stages: StageCounters`** — the stage-share counter seam
> (`U2_node_protocol.md` §U2-M item 2): … **WP-1.6 (quiescence) blocks on this
> seam existing**, even though it consumes none of stage Q's widening-rate
> counters (those defer with `WPQ_seed.md` §7.2).

**Contradicting evidence.** The cited source says the opposite shape of thing.
`U2_node_protocol.md` §U2-M item 2 (U2:652–660):

> The rates: F/T/Q firing rates, the filtered-node rate, the `Cover::Impossible`
> rate and the overload-return rate. **The widening rate per node class … and
> the TT entries the truncation rule declines to store are stage Q's quantities
> and DEFER with it** (`WPQ_seed.md` §7.2); the counter seam described above is
> what a later WP reads them through, so the seam is not deferred.

"a later WP" there is the WP that owns the deferred stage-Q quantities — WP-1.5c
(D-310, D-315) — not WP-1.6. Nothing in item 2, in U2's own §U2-Z handoff to
WP-1.6, or anywhere else states that WP-1.6 blocks on the seam:

```
$ grep -n "WP-1\.6" docs/experiments/U2_node_protocol.md \
    docs/experiments/U3_tier_t.md docs/experiments/U4_soundness_instrument.md \
    docs/experiments/WPQ_seed.md docs/ROADMAP.md \
    docs/experiments/section_owner_table.md
docs/ROADMAP.md:126:and it does not block WP-1.6.
docs/ROADMAP.md:128:**WP-1.6 — threat-only zone-bounded quiescence**, under D-111's invariant: the
docs/experiments/U2_node_protocol.md:171:| 4 | `LAW-RIPOSTE` | **DEFERRED to WP-1.6.** …
docs/experiments/U2_node_protocol.md:172:| 5 | `LAW-LEDGER` | **DEFERRED to WP-1.6.** …
docs/experiments/U2_node_protocol.md:542:Out of scope. WP-1.6's, with `LAW-RIPOSTE` and `LAW-LEDGER` (U2-Z item 8).
docs/experiments/U2_node_protocol.md:672:Adopted verbatim: no quiescence (WP-1.6); …
docs/experiments/U2_node_protocol.md:711:8. `LAW-RIPOSTE` and `LAW-LEDGER` hand off to WP-1.6; …
docs/experiments/U2_node_protocol.md:771:### The handoff this unit carries to WP-1.6 …
docs/experiments/section_owner_table.md:227:| §18.4 the handoff to WP-1.6 | **U2** …
docs/experiments/U3_tier_t.md:677:- **The census is WP-1.6's to extend**: …
```

U2's §U2-Z handoff to WP-1.6 (U2:771–792) lists four things — `LAW-RIPOSTE` and
`LAW-LEDGER`, the settled node-protocol shape, the generalised overload verdict's
owed calculus amendment, and D-111's stand-pat rule. The counter seam is not
among them. The only ROADMAP sentence in the neighbourhood runs the other way
(ROADMAP:125–126, of WP-1.5c): *"PRIORITY: after WP-1.5b, and it does not block
WP-1.6."*

**Why it breaks.** This is the misattribution class D-322 and D-330 both landed
lines about and D-331 was landed for: a claim whose cited home does not carry it.
Its consequence is not cosmetic — it is a **priority argument**. Bolded in the
prompt, it tells an IMPL session that a downstream work package is blocked on
this seam, which is exactly the kind of sentence that promotes a deliverable over
its neighbours. It is also the strongest candidate falsifier a reader would reach
for against MAJOR 6's universal.

**Fix scope.** One sentence in §5. Either delete the WP-1.6 clause, or replace it
with what item 2 actually supports — *"a later WP reads the deferred stage-Q
rates through this seam, so the seam is not deferred with them"* — pointed at
§U2-M item 2. If a real WP-1.6 dependency exists, its home is U2's §U2-Z handoff
list and it is owed there first.

---

### MAJOR 2 — §4 calls the hotspot substitution "already landed"; it has no ADR line, D-263 stands unamended, and the ROADMAP still names the cover arithmetic

**Claim reviewed** (§4 heading and lead,
`docs/wp15b_impl_prompt.md:149`, `:161–166`):

> ## 4. The pre-registered hotspot — bracket and abort threshold (`docs/decisions.md` D-263, corrected by `U3_tier_t.md` §U3-M item 4)
> … Two things then happened, **both already landed and neither IMPL's to redo** …
> **IMPL's registered hotspot is Tier-T cell extraction, per `U3_tier_t.md`
> §U3-M item 4, not the cover arithmetic D-263 originally named**

**Contradicting evidence.** Three commands, none of which the document's own
framing survives.

(i) No ADR line records the substitution, and D-263 is unamended:

```
$ grep -n "D-263" docs/decisions.md
567:D-263: THE COVER ARITHMETIC IS WP-1.5b's PRE-REGISTERED HOTSPOT, …
$ grep -n "Tier-T cell extraction\|TIER-T CELL EXTRACTION\|Tier T extraction" docs/decisions.md
(no output)
```

(ii) U3 records that ADR line as **OWED and NOT LANDED**. `U3_tier_t.md` §U3-Z's
own lead-in (U3:689–692): *"items 2, 7 and 16 are this unit's own and **have not
landed**"*, and item 7 (U3:702–708) is precisely this line: *"**AND D-263 NAMED
THE WRONG HOTSPOT**: Tier-T cell extraction is MEASURED at about 6× both threat
queries combined. Registered here with its own bracket and abort threshold …"*

(iii) `docs/ROADMAP.md`:89–92 still states the opposite, and the ROADMAP changes
only by ADR (D-313: *"`docs/ROADMAP.md` changes only by ADR"*; D-314 the same):

> WP-1.5b's PRE-REGISTERED HOTSPOT is already named and it is not the eval: it is
> the cover arithmetic it will call per node, whose growth shape and unmeasured
> allocation-per-call are registered in docs/decisions.md D-263 …

And the first of the two "already landed" things is a design decision whose code
does not exist. `U2_node_protocol.md` §5.2's M5-E is *"Revision 4 computes each
once"* — a code block in a design unit, not a commit:

```
$ ls crates/pistol-search/src/
candidates.rs error.rs fallback.rs info.rs lib.rs ordering.rs params.rs
position.rs pv.rs pvs.rs score.rs search.rs stop.rs tt
$ grep -c pistol-solver crates/pistol-search/Cargo.toml
0
```

`staged.rs` does not exist and `pistol-solver` is not a dependency of
`pistol-search` — the same measurement `U4_soundness_instrument.md` U4-Z:1846–1849
records. U2's own closing line reads `*U2. IMPL has not started.*` (U2:840).

**Why it breaks.** §4 IS the rule-5 pre-registration this IMPL will be judged
against. Substituting a registered hotspot is exactly what CLAUDE.md rule 10
requires an ADR line for (*"Silent architecture drift is a breach; amend the ADR
instead"*), and U3 already knows this and books it as item 7. The prompt asserts
the substitution as settled fact and mentions the debt nowhere — not in §4, not
in §6's U3 bullet, not in §8's finish policy. The result is a governing document
under which an IMPL session benches Tier-T extraction while two landed documents
(`docs/decisions.md` D-263 and `docs/ROADMAP.md`) still say the hotspot is the
cover arithmetic, and D-263's own outstanding duty — *"measure these two at the
candidate counts its own generator produces"*, whose flip clause is *"Flips when
WP-1.5b measures it"* — is discharged by nobody. "Already landed" is the word
that hides all of this.

**Fix scope.** §4, two edits. Replace "both already landed" with what is true of
each (an adopted design decision in U2 §5.2 whose code IMPL writes; a
re-measurement in U3 §U3-M item 4). Add the ADR debt: U3-Z item 7's line is OWED,
D-263 and `docs/ROADMAP.md`:89–92 still name the cover arithmetic, and the
ROADMAP changes only by ADR. State whether D-263's own owed measurement travels
with the substitution or is discharged by it.

---

### MAJOR 3 — the finish policy requires all four soundness gates wired while §3.3 and §8's own exclusion list say the fourth cannot be written

**Claims reviewed**, three sites in the same document:

§3.3 (`:132–134`):
> **The differential gate's own script fragment cannot be written until §3.2's
> SEAM decision lands** — the other three parts are unaffected and are not
> blocked on it.

§8 item 2 (`:267–271`):
> `tools/staged_soundness_check.sh` exists, **wires all four §3.3 gates** into
> `tools/ci.sh`, is reviewed against `tools/SHELL_CHECKLIST.md` …, and is green —
> the differential gate's own part marked `DEPENDS-OPEN-THEORY` rather than
> silently green on an unresolved convention.

§8's exclusion list (`:283–285`) and closing line (`:290–291`):
> **Finish does not require:** … §3.2/§3.3's differential-gate SEAM decision, if
> still open when IMPL starts (carried forward as §6 debt, **not blocking**) …
> A landing that skips **any of 1–5** is not finished.

**Contradicting evidence.** §3.3's blocker is verbatim from the primary source.
`U4_soundness_instrument.md` §8.7 (U4:822–827):

> **THREE OF THE FOUR PARTS CAN BE SPECIFIED AND ONE CANNOT:** the differential
> gate's CRITERION is selected, but the SEAM by which a test observes the emitted
> set is a separate named decision D-323 leaves OPEN (D-115, round-1 F4), so **the
> script's differential part cannot be written until that decision is made.** The
> other three named gates are unaffected …

and U4-Z's live OPEN bullet (U4:1920–1930) re-checks it and adds *"**No matrix
has ever been authored for THIS one**, and no ADR line decides it."*

**Why it breaks.** With the SEAM open — which §8 explicitly contemplates ("if
still open when IMPL starts") — item 2 is unsatisfiable, and the closing line
converts an unsatisfiable item into "not finished." An IMPL session cannot obey
both sentences. Worse, the escape §8 offers points at the wrong open question:
`DEPENDS-OPEN-THEORY` is D-321's mark for the **convention** (minimum-cardinality
vs inclusion-minimal), an entirely different open item from the SEAM, and marking
a fragment `DEPENDS-OPEN-THEORY` does nothing to make it writable when the
observation seam is undecided. The prompt itself keeps the two apart correctly in
§3.2 and then conflates them here. This is a finish criterion that cannot be
evaluated, in the section whose whole job is to say when IMPL is done.

**Fix scope.** §8 item 2. Scope it to what §3.3 licenses: the script exists and
wires the three unblocked gates, with the differential part conditional on the
SEAM decision — and say explicitly what a landing does when the SEAM is still
open (ship three parts with the fourth named and stubbed as OPEN debt, or hold
item 2). Keep the `DEPENDS-OPEN-THEORY` clause, which is correct on its own
subject, and stop it standing in for the SEAM.

---

### MAJOR 4 — §3.4 carries half of §6.5's pre-registered consequence and points the other half's referent at the wrong section

**Claim reviewed** (§3.4, `docs/wp15b_impl_prompt.md:139–143`):

> **Pre-registered consequence, fixed before any gate runs:** if the soundness
> instrument (§3.2) shows C dropping a cell a proven tactic needs, C is replaced
> by B — strictly wider under the threshold reading — as an amendment with its
> own review, never a silent threshold move.

**Contradicting evidence.** The primary source states two branches under that
same heading, not one. `U3_tier_t.md` §6.5 (U3:290–297), in full:

> **Pre-registered consequence, fixed before any gate runs.** If the soundness
> instrument (**U4** … §8) shows C dropping a cell a proven tactic needs, C is
> replaced by B — which under the threshold reading is strictly wider — and the
> exchange is an amendment with its own review, never a threshold move. **And the
> branch revision 1 omitted:** if the instrument is GREEN while mutation M7
> (**U4** … §8.4; Tier T at ≥3 for the mover — option A) also SURVIVES, then the
> instrument has demonstrated it cannot tell A from C, C's entire ground is
> unmeasured, and that is recorded as such in the results rather than read as a
> confirmation of C.

The prompt reproduces the first branch and drops the second entirely — `M7`
occurs zero times in `docs/wp15b_impl_prompt.md`.

Two further inaccuracies ride on the same sentence. §6.5's referent for "the
soundness instrument" is **U4 §8** — the whole four-part gate — where the prompt
substitutes "(§3.2)", its own section for the **differential gate alone**. The
dropped branch's mutation M7 lives in U4 §8.4, the mutation ledger, which is
neither §8.2 nor anything the prompt's §3.2 covers. So the one cross-reference
the prompt supplies points away from where the missing half lives.

**Why it breaks.** The dropped branch is the half that prevents a green gate from
being read as evidence for C. It is a **registered consequence** in CLAUDE.md's
sense, written before the run, and it is the only thing standing between a green
soundness gate and an IMPL session banking C's ground as confirmed. The prompt
quotes the source's own heading verbatim — "Pre-registered consequence, fixed
before any gate runs" — which certifies to the reader that what follows is the
whole registration. Under D-346's boundary this is a **normative** claim, "a
measurement IMPL depends on"; removing half of one is exactly the disposal that
line forbids.

**Fix scope.** §3.4, one added sentence carrying §6.5's second branch with M7
named and pointed at `U4_soundness_instrument.md` §8.4, and the referent for "the
soundness instrument" retargeted from §3.2 to §3.3 (or directly to U4 §8).

---

### MAJOR 5 — U2-Z's rule-5 precondition on U2's IMPL is demoted to ordinary debt, and no build-order or finish clause carries it

**Claim reviewed** (§6, `docs/wp15b_impl_prompt.md:232–237`), the U2 bullet, under
the heading *"What is OPEN and this document does not resolve … Pointer-only"*:

> **U2** (`U2_node_protocol.md` §U2-Z): … rule 5 for the node protocol itself
> (`can_win_this_turn` + `blocking_covers` on every node has no expected-gain
> bracket, abort threshold or bench of its own — distinct from §4's Tier-T
> hotspot).

**Contradicting evidence.** The source does not carry this as debt. It carries it
as a **precondition with a named owner and a named deadline**.
`U2_node_protocol.md` §U2-Z OPEN (U2:826–836):

> **RULE 5 IS UNDISCHARGED FOR THE NODE PROTOCOL ITSELF** … the change that puts
> `can_win_this_turn` and `blocking_covers` on EVERY NODE still has no
> expected-gain bracket, no abort threshold and no IQR-gated bench reporting nps
> AND time-to-depth. **The carve does not close this and does not narrow it.** It
> is a rule-5 registration **the architect must place before U2's IMPL**, not a
> repair a carve may write.

The prompt's own §2 is the section that binds IMPL order, and it binds only
U2-before-U1. §8's finish policy has five items and none is this. §4 registers
the Tier-T hotspot and says nothing about the per-node threat-query change. So
the sentence "the architect must place [it] before U2's IMPL" survives nowhere in
the document that governs U2's IMPL.

**Why it breaks.** §6's framing sentence — *"What is OPEN and this document does
not resolve … Pointer-only, per unit's own OPEN list"* — is accurate for every
other bullet in that list and wrong for this one. The other bullets name work
that may run alongside or after IMPL; this one names work that must precede it,
and it is the architect's, not IMPL's. A governing prompt that files it beside
"the unmarked `23.2` in §6.3's failure-mode cell" has, in effect, licensed U2's
IMPL to start without it — which is the one thing the source forbids. It is also
the same rule-5 gap D-263 was landed to prevent (*"named now so that the first
per-node caller does not discover it"*), reaching IMPL undischarged from the
opposite direction to MAJOR 2.

**Fix scope.** Promote it out of §6's pointer list. Either §2 (build order) gains
a second binding clause — the architect's rule-5 registration for the node
protocol lands before U2's IMPL — or §8's finish policy gains it as item 0 with
its owner named. §6 keeps a pointer to wherever it goes.

---

### MAJOR 6 — the document opens with a false universal about its own text, in a document instructed to carry no self-state claim

**Claim reviewed** (header, `docs/wp15b_impl_prompt.md:6–7`):

> Every claim below cites its primary source; a reader who wants a finding's
> content or a design's argument reads it there, per D-331's fold law.

**Contradicting evidence.** The universal is falsified inside the document it
quantifies over. §4's *"Two things then happened, **both already landed** and
neither IMPL's to redo"* (`:161`) cites no source for "already landed" and no
source supports it — see MAJOR 2's three commands. §5's *"WP-1.6 (quiescence)
blocks on this seam existing"* cites `U2_node_protocol.md` §U2-M item 2, which
carries no such claim — see MAJOR 1's grep. And §3's lead certifies that *"the
argument for each lives at its ADR line and its unit's own §8/§9"* (`:52–53`),
which has no referent for §3.4: C at the threshold reading has no ADR line at all
(`U3_tier_t.md` §U3-Z item 2 is GATED and, per U3-Z's own lead-in, has not
landed), and its argument lives at U3 §6.1/§6.5, not at any §8 or §9.

The class is named in the tree, and this project has a landed policy for it.
D-346:

> A claim a document makes about **its own state** — its u-rev history, which of
> its own revisions were reviewed and with what verdict, what its own repairs
> reached, **universals about its own text** — that is found **false or
> unverifiable** is **STRUCK: deleted, and replaced with nothing.**

`U3_tier_t.md` §U3-Z:734–755 records that this unit "failed FOUR consecutive
rounds" on the neighbouring class — *"assertions about a set the author had not
enumerated, made at their own home, restating nothing"* — and
`U4_soundness_instrument.md` §U4-Z:1796–1816 records the same class failing three
units and asks the architect for a binding clause. §6 of this very prompt carries
that gap forward as OPEN debt (`:238–240`) while §1's header instantiates it.

Three siblings of the same shape, unchecked because they are unverifiable rather
than false: `:9–10` *"this document does not repeat that mapping"*; `:52`
*"None is restated beyond what IMPL needs to build it"*; `:226–227`
*"Pointer-only, per unit's own OPEN list."*

**Why it breaks.** The prompt's authoring instruction was *"No self-state claims
of any kind in this file."* Independently of that instruction, a false universal
about a governing document's own text is worse here than in a design unit: it is
the sentence a reviewer or an IMPL session uses to decide whether a bolded,
uncited claim needs checking. It certified `:161` and `:218` as sourced, and both
are not. `:52`'s universal is the one that told a reader C's argument is at an
ADR line that does not exist.

**Fix scope.** Strike `:6–7` and replace it with nothing, per D-346 — the
neighbouring sentence at `:5–6` (*"cited here and not restated"*) already carries
the navigational content, and the per-claim citations do the work the universal
was asserting. Treat the three siblings the same way. Then fix the two claims the
universal was false about, per MAJOR 1 and MAJOR 2.

---

### MINOR 1 — §3.2 calls `0 of 3406` a "legality-agreement figure"; it is the R1-referent-vs-shipped-cover-query agreement

**Claim reviewed** (§3.2 condition 2, `:99–101`):

> The `0 of 3406` **legality-agreement** figure **may not be cited as evidence
> about the convention** — R1 and `cover.rs` are blind to it together.

**Contradicting evidence.** D-323 condition (2) reads *"the `0 of 3406`
agreement MAY NOT be cited as evidence about the convention, because both
instruments are blind to it together"* — no "legality". The probe output the
figure comes from
(`matrix_M3_soundness_instrument_rev2.md`:213–218, reproduced at
`matrix_M3_REDTEAM_round2.md`:134–139) is:

```
side-positions              = 3406
R1 referent DISAGREES with shipped on 0 of 3406 side-positions
```

and rev2:241 glosses it *"the landed referent disagreed with the shipped queries
nowhere in the regime."* D-321 states the same: `cover.rs` *"is checked against an
independent referent at `crates/pistol-solver/tests/common/reference.rs` with 0
disagreements over 3406 side-positions."* It is an agreement about **cover
enumeration**, not about legality.

**Why it breaks.** The condition governs what the figure may and may not be cited
for; a wrong label on the quantity is the D-322 attribution class in miniature,
and it is gratuitous — D-323 supplies no adjective and none is needed.

**Fix scope.** Delete "legality-", or replace with "cover-enumeration
agreement".

---

### MINOR 2 — §3.3's header parenthetical contradicts its own table

**Claim reviewed** (§3.3, `:119–120`):

> The gate has four parts, named rather than lettered, **each specified in
> exactly one place** (`U4_soundness_instrument.md` **§8.3, §8.7**)

**Contradicting evidence.** The table three lines below names §8.2 for the
differential gate, not §8.7 — and it is right to, because that is what the
primary source says. `U4_soundness_instrument.md` §8.3's own table (U4:592–597)
gives `§8.3 below, first bullet` / **`§8.2`** / `§8.3 below` / `§8.3 below`, and
§8.7 (U4:803–809) repeats it: *"the differential gate (§8.2: S-M …) … **each
defined in exactly one place**."* §8.7 is the **wiring** sentence, not a
specification site for any of the four.

**Why it breaks.** A reader following the parenthetical to §8.7 for the
differential gate's specification finds the CI enumeration instead. §8.2 — the
one section that does specify it — is named nowhere in the header.

**Fix scope.** `(§8.3 and §8.2; wired at §8.7)`.

---

### MINOR 3 — §3.1 reads D-329's rung-(a) silence as support for N-E, which is the reading D-329 corrects

**Claim reviewed** (§3.1, `:58–61`):

> This is hard rule 1's fourth clause in its literal form — *"NO code-side
> default for any tunable"* — **not merely its spirit**: the three-row field's
> rung (a) was silent on defaults across every row, and N-E still refuses an
> absent flag by name at exit 1.

**Contradicting evidence.** D-329 introduces that silence as a **finding against**
resting on the clause, not for it:

> **RUNG (a) IS SILENT ACROSS THE FIELD**, and **this is where the matrix went
> wrong**: hard rule 1's fourth clause governs DEFAULTS — *"NO code-side default
> for any tunable — a default lives in exactly one schema place"* — and MEASURED,
> **none of the three rows has a default, all refusing an absent `--config` by
> name at exit 1**; the round-3 red team had already ruled that clause *"a WOUND,
> not a KILL"* for that reason …

CLAUDE.md's rule 1 fourth clause is correctly identified (splitting rule 1 on
`;`: explicit+complete / `deny_unknown_fields` / missing key = error / **NO
code-side default …** / `Budget` closed enum), and D-329 calls it the fourth
clause too. What is inverted is the inference: silence across every row is
evidence the clause does not distinguish N-E, and the prompt introduces it with a
colon as though it established the opposite. The prompt's "N-E still refuses an
absent flag by name at exit 1" is also singular where D-329's measurement is
*"all refusing"*.

**Why it breaks.** N-E's actual ground is rung (b) plus the three measured
findings — D-329 says so at length — and the prompt's own §3.1 never mentions
either. An IMPL session that reads this section for why the seam is shaped this
way gets the argument D-329 struck.

**Fix scope.** §3.1, one sentence. State the shape (required `--config`, no
default, whole-path guard), state that rung (a) was silent because all three rows
satisfied the clause, and point at D-329 for rung (b) — no defence of the
selection is owed here.

---

### MINOR 4 — §3's lead calls all four "landed selections" whose argument lives "at its ADR line"; §3.4's ADR line is gated and unwritten

**Claim reviewed** (§3 lead, `:50–53`):

> Each of the following is a **landed selection**, not a proposal … the argument
> for each lives at **its ADR line** and its unit's own **§8/§9**.

**Contradicting evidence.** §3.1/§3.2/§3.3 head with `docs/decisions.md` D-329 /
D-323 / D-316 and their arguments do live at U4 §9 / §8. §3.4 heads with
`U3_tier_t.md` §6.5 — no ADR line — and `U3_tier_t.md` §U3-Z item 2 (U3:694–700)
reads:

> 2. Tier-T option C at the threshold reading … **GATED — MAJOR 6: this line may
>    not be written until a fresh DECISION-RED-TEAM has attacked M1 AS AMENDED.**

with U3-Z's lead-in (U3:689–692) confirming *"items 2, 7 and 16 are this unit's
own and **have not landed**."* Its argument lives at U3 §6.1 and §6.5, not at any
§8 or §9.

**Why it breaks.** §3.4 discloses the gate correctly two paragraphs later, so a
careful reader recovers — but the lead's universal is what a reader consults to
know where authority lives, and for the one selection with no ADR line it sends
them to one.

**Fix scope.** §3 lead: "three carry a landed ADR line; §3.4's is GATED (U3-Z
item 2) and its argument lives at U3 §6.1/§6.5."

---

### MINOR 5 — §3.2 and §3.4 point their debt at §5; the OPEN-debt section is §6

**Claim reviewed**, two sites:

> §3.2 (`:113–114`): the SEAM is *"carried as debt in **§5** below, not resolved
> here."*
> §3.4 (`:147`): the gated ADR line is *"carried as debt in **§5** below."*

**Contradicting evidence.** §5 is *"Fixtures and configs IMPL produces"*; §6 is
*"What is OPEN and this document does not resolve"*, and both items are in it —
the SEAM at `:244` and the M1 red team at `:241–242`. §5's own bullet gets it
right (`:200`: *"carried as debt in §6"*), which is what makes the two others
plainly wrong rather than a renumbering choice.

**Why it breaks.** This is the "claim's home moved without the pointer following"
class, twice, and it lands in the two sections whose debts are the ones an IMPL
session most needs to find.

**Fix scope.** `§5` → `§6` at `:113` and `:147`.

---

### MINOR 6 — §3.4 renders the Tier-T qualification with `>=` where U3 §6.1 is explicit that the config key is spelled `= 2` and *means* ≥ 2

**Claim reviewed** (§3.4, `:137–138`):

> `tier_t_own_count >= 2`, `tier_t_opponent_count >= 3` (the **threshold**
> reading, not exact — `U3_tier_t.md` §6.1).

**Contradicting evidence.** `U3_tier_t.md` §6.1 (U3:141–143):

> **ADOPTED: the THRESHOLD reading.** `tier_t_own_count = 2` means own windows at
> count **≥ 2**; `tier_t_opponent_count = 3` means **≥ 3**.

The key/value spelling is `= 2` and `= 3`; the `≥` is the semantics the generator
implements. §6.1's entire content is that these two came apart once — *"revision
1 printed the threshold figure while §10 committed the exact one … **The option
committed was not the option measured.** An implementer following §10 literally
would have shipped a generator the matrix never evaluated."*

**Why it breaks.** §5 of the prompt has IMPL author four TOML documents, and TOML
carries no `>=`. The document never states the literal key spelling anywhere, so
the one thing IMPL types is left to inference — in the exact place where
inference produced the defect §6.1 exists to correct.

**Fix scope.** Render as U3 §6.1 does: keys `tier_t_own_count = 2` /
`tier_t_opponent_count = 3`, qualification at count ≥ 2 and ≥ 3.

---

### MINOR 7 — §5 overstates what `wp15b_sprt_prereg.md` §9's slots need, and §9.7 is a revision slot, not a config need

**Claim reviewed** (§5, `:206–209`):

> **These same four documents are what `docs/experiments/wp15b_sprt_prereg.md`
> §9's OPERATOR-CONFIRM slots need** before that document's governed run can be
> taken — in particular §9.7, the revision at which `tools/baseline_snapshot.sh`
> accepts `--config` (§3.1 above), and §9.4, the soundness gate (§3.3 above)
> green at the run's revision.

**Contradicting evidence.** §9 has seven slots
(`wp15b_sprt_prereg.md`:710–753). §9.1 `elo1`, §9.2 `binary_sha256`, §9.3
`openings_take`, §9.5 the calibration probe and §9.6 the run's revision involve
no staged config document. §9.7 reads:

> **9.7 The baseline snapshot's revision.** The commit at which
> `tools/baseline_snapshot.sh` accepts `--config` (§7A.2). At this document's
> revision it does not … **The slot is a REVISION, not a path.**

So §9.7's content is a revision of the *script*, not a config document — the
prompt's own gloss says so and the lead clause contradicts it. §9.4 (*"§8 of the
design must be GREEN at the revision this run measures"*) does depend on the
configs, through the gate; that dependency is real and is recorded independently
at `U4_soundness_instrument.md` U4-Z:1941–1945 (*"`configs/instrument_staged_v0.toml`
DOES NOT EXIST**, so the snapshot's AFTER is blocked"*).

**Why it breaks.** Overstated to the point of being wrong about the named slot,
in the bullet whose job is to tell IMPL why the four documents are on its
critical path. The true and sufficient statement is narrower.

**Fix scope.** §5, one sentence: the configs are what §9.4's soundness gate and
the snapshot's AFTER need; §9.7 is a separate slot for the revision at which the
script accepts `--config` (§3.1).

---

# VERIFIED WITH NO FINDING

Each item below was checked against the primary source the prompt cites — the
ADR line's own text, the unit section's own text, or the tree — and confirmed.

**§7's NOT IN SCOPE list is verbatim, mechanically.** Claim: *"Carried verbatim
from `U2_node_protocol.md` §14."*

```
$ python3 - <<'EOF'   # normalise whitespace, strip each side's lead-in, compare
IDENTICAL: True
no quiescence (WP-1.6); no killers/history/countermove (WP-1.7); no df-pn
(WP-1.8); no eval terms from `t` or `τ`; no dominance pruning beyond the staged
scheme; no `LEGAL_RADIUS` change; no ball-scan optimisation; no `pistol-eval`
refactor.
EOF
```

Byte-identical after whitespace normalisation, all eight items, same order. U2
§14's own lead-in is *"Adopted verbatim:"*, so the chain of custody holds.

**§5's absence claims — every path checked by `ls`, all five absent.**

```
$ for p in configs/instrument_staged_v0.toml configs/tactical_staged_v0.toml \
           configs/gate_staged_v0.toml configs/play_staged_v0.toml \
           crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt \
           tools/staged_soundness_check.sh; do
    [ -e "$p" ] && echo "PRESENT: $p" || echo "ABSENT:  $p"; done
ABSENT:  configs/instrument_staged_v0.toml
ABSENT:  configs/tactical_staged_v0.toml
ABSENT:  configs/gate_staged_v0.toml
ABSENT:  configs/play_staged_v0.toml
ABSENT:  crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt
ABSENT:  tools/staged_soundness_check.sh
$ ls configs/
arena_smoke_v0.toml  arena_wp13_fair_corpus.toml  arena_wp13_fair_random.toml
arena_wp13_r2_vs_r3_confirm.toml  arena_wp13_r2_vs_r3.toml
arena_wp15b_dryrun.toml  eval_v0_weights.toml  gate_v0.toml
instrument_r2_v0.toml  instrument_v0.toml  play_v0.toml
random_openings_v1.toml
$ ls crates/pistol-cli/tests/fixtures/
bench_positions_v1.txt  corpus_distance_v1.jsonl  corpus_synthetic_v1.jsonl
instrument_golden_v1.txt  openings_v1.txt  random_openings_v1.txt
spread_v1.txt  tactical_v0.txt
```

The MEASURED-absent claim at `:184–185` holds for all five named paths, and the
fixture directory the prompt names is the right one (`tactical_v0.txt` lives
there). `tools/staged_soundness_check.sh` is absent too, consistent with §8 item
2's "exists" being a deliverable.

**§5's four-document config table is exact against `U3_tier_t.md` §10.** All four
rows — document, mode, `quiet_radius`, `quiet_top_k`, `widen_schedule`, cut —
reproduce cell for cell against U3:350–355: `instrument_staged_v0.toml`
instrument/2/16/`[32]`/binds; `tactical_staged_v0.toml` instrument/2/**1024**/
`[2048]`/disabled; `gate_staged_v0.toml` instrument/1/**128**/`[256]`/disabled;
`play_staged_v0.toml` play/3/16/`[32]`/binds. §10's header — *"**FOUR** complete
documents, `deny_unknown_fields`, no code-side default for any value. This is the
one place the count is stated"* — is honoured: the prompt cites §10 and adds no
competing count.

**§4's quoted numbers all reproduce at their sources.**
- `10.51 % → 7.45 %`: `U2_node_protocol.md` §5.2 (U2:255–259), *"D-263's
  recomputed ceiling falls from **10.51 % to 7.45 %** of a fast node — a larger
  cut in the registered hotspot than any remedy D-263 names, taken by deleting
  work rather than accelerating it."* The prompt's "by deleting work rather than
  accelerating it" is that clause verbatim.
- `533 / 662 ns` against `86 ns`, "about 6×": `U3_tier_t.md` §U3-M item 4
  (U3:517–522), *"extracting Tier T's cells costs about **6×** both threat
  queries combined (533 ns with a reused buffer, 662 ns fresh, against 86 ns for
  the pair on the same harness)."*
- `1.05×`: U3:535, *"ABORT THRESHOLD: below 1.05×, or any regression in
  whole-search nps."* Reproduced exactly, both halves.
- `29.2 %`: U3:539–543, *"**U2** … §5.3 does not extract Tier T on the **29.2 %**
  of them that take a forced row, so the registered figure is a blended mean over
  two populations … The IMPL re-takes it on BATCHED nodes only."* The prompt's
  rendering — blended mean across BATCHED and the 29.2 % forced-row nodes,
  re-taken on BATCHED only before banking a bracket — is faithful, including the
  direction of the correction. `70.8 %` is its complement (U3-Z:810, *"100 minus
  `BATCHED nodes`"*) and the prompt correctly does not restate it.
- The BASELINE/accessor two-commit registration, the "no bracket may be derived
  before IMPL measures it" refusal, and the "never the snapshot, which reports
  `depth_turns`/`nodes` only" clause all reproduce at U3:526–538 essentially word
  for word.

**D-263's own text supports what §4 attributes to it.** Read in full at
`docs/decisions.md`:567: it names `blocking_covers` and `min_hitting_set_exceeds`
as WP-1.5b's hotspot, *"NAMED NOW SO THAT THE FIRST PER-NODE CALLER DOES NOT
DISCOVER IT"*, and closes the rule-5 inventory with *"a rule-5 verdict needs a
pre-registered hotspot, an expected gain bracket, an abort threshold and one
IQR-gated bench reporting nps AND time-to-depth, and **NONE of those is in this
line**."* The prompt's quotation is exact. (See REJECTED 3 for the one wrinkle I
considered and dismissed.)

**§3.1's four N-E conditions trace to D-329, all four, none dropped, added or
misnumbered.** D-329's *"**FOUR CONDITIONS RIDE WITH THE SELECTION:**"* block
(`docs/decisions.md`:703) gives (1) digest is `$3` not `$4`, `config <path> <sha>`
three fields, four-token reasoning belongs to the `corpus` line; (2) guard may NOT
reuse the basename loop, measured twice; (3) item-10 driving test for both new
refusal classes, two halves with a control, precedent 91 test lines for one arm at
`b067d47`; (4) item-12 sentence saying a config refusal is a FAIL, this script
having declared no void class. The prompt's 1–4 match in content and in number.

**The fifth residual is attributed to the selection record, not to D-329 — the
trap is avoided.** The prompt (`:78–82`) attributes the relative-base residual and
its R19 closure at `63eac4c` to *"(`U4_soundness_instrument.md` §U4-Z, the N-E
conditions bullet)"*. That bullet (U4:1833) is emphatic on exactly this point:
*"**it is recorded by the SELECTION RECORD and not by D-329** … Until u-rev 8
this parenthetical attributed the residual to 'recorded by D-329', and D-329
records nothing of the kind … **MAJOR 3** of `wp15b_U4_REVIEW_urev7.md`, an
ATTRIBUTION defect."* I checked D-329's text for `relative`, `CALLER_PWD` and F6:
none occurs. The prompt cites the right home and does not repeat the defect.

**§3.2's five S-M conditions trace to D-323, all five, in order.** D-323's
*"FIVE CONDITIONS RIDE WITH THE SELECTION"* block: (1) R1 REUSED by `#[path]`
include, second freshly-written referent FORBIDDEN without a registered agreement
criterion and a registered consequence for disagreement; (2) `0 of 3406` may not
be cited about the convention; (3) ships MARKED DEPENDS-OPEN-THEORY under D-321;
(4) S-N OWED and a FLIP TRIGGER, not a footnote; (5) registered numbers carry
their instrument, source committed in the selection record. The prompt's 1–5
match, with two verified elaborations: `docs/experiments/matrix_M3_selection.md`
is indeed the selection record D-323 names, and D-323's flip clause is *"Flips if
S-N is stated in a form that is green on a correct engine and affordable at a
sampled population, at which point M3 reopens as a two-row comparison"* — which
is what §3.2 condition 4 says, including "against S-M".

**§3.2's DEPENDS-OPEN-THEORY gloss is accurate.** *"(`DEF-T`'s
minimum-cardinality reading vs. `cover.rs`'s inclusion-minimal one) is OPEN
theory and the calculus is not amended"* — D-321: *"`DEF-T` defines the threat
NUMBER `t(F)` as the 'exact **minimum hitting set** …'; `crates/pistol-solver/src/cover.rs`
enumerates the **INCLUSION-MINIMAL** covers … it records that
`docs/research/threat_calculus_v1.md` is **NOT amended**."*

**§3.2's two "not decided by D-323" items are both live and correctly located.**
The SEAM: `U4_soundness_instrument.md` U4-Z:1920–1930, *"D-323 selects the
CRITERION (S-M) and explicitly does not decide how a test observes the emitted
set; **D-115's constraint on widening `pistol_search::staged` to `pub`** —
round-1 F4 … **No matrix has ever been authored for THIS one**, and no ADR line
decides it."* The prompt's parenthetical reproduces that phrase exactly. D-115
itself (`docs/decisions.md`:260) does carry the constraint: *"no item is made
public, no `pub(crate)` is widened and no signature is altered to let a test reach
it."* S-E's second half: U4-T's `visit_searches_every_forced_candidate` row
(U4:1346) — *"**NOT SELECTED AND NOT REJECTED, AND IMPL MAY NOT READ IT AS
REGISTERED** … This was **S-E, half two**: the always-on `assert!` in `visit`,
which is what sees a drop made AFTER generation"* — so the prompt's §U4-T citation
and its "for a post-generation drop" gloss are both right.

**§3.3's gate-names table reproduces U4 §8.3's table row for row.** U4:592–597
gives the same four names with the same specification sites — `§8.3 below, first
bullet` / `§8.2` / `§8.3 below` / `§8.3 below`. D-316 (`docs/decisions.md`:677)
confirms the four names and the letter retirement: *"THE RETIRED LETTERS ARE KEPT
AS A LOOKUP AND NOT ERASED: `U4_soundness_instrument.md` §8.3 opens with a table
mapping `(a)`/`(b)`/`(c)`/`(d)` to their gates."* The prompt's *"nothing new is
addressed by letter"* is U4 §8.3:605's *"Nothing below is addressed by letter."*
The one-script/`tools/ci.sh` claim and the SEAM blocker are U4 §8.7:803–827
verbatim in substance. (Only the header parenthetical is wrong — MINOR 2.)

**§5's fixture split is right.** *"twenty cases, fifteen at
`tactical_staged_v0.toml` and five at `gate_staged_v0.toml`"* — U4 §8.3:620–624
(*"`tactical_staged_v0.toml` for the fifteen `instrument_v0` cases and
`gate_staged_v0.toml` for the five gate cases"*), U4:641–642, U4:702–707
(*"MEASURED: `tactical_v0.txt` is 15 cases at `configs/instrument_v0.toml` and
**5 at `configs/gate_v0.toml`**"*), and U4-T:1344.

**§2's build order is accurate, and it is corroborated by a landed ADR.**
`U1_gate_supersession.md` §U1-B:233–235: *"**The binding order: U2's IMPL lands
before U1's gates are armed.** Arming them first makes CI red on a workspace that
has not changed; landing U2 first without U1 makes CI red on a workspace that
has. Neither is a defect in either unit."* The prompt's rendering is this,
compressed, with "but this one is binding" added — which §U1-B's own heading and
D-310 condition (4) support (*"IMPL ORDER IS BINDING even though review order is
free, because `pistol-solver` is absent from `[workspace.dependencies]` and from
`crates/pistol-search/Cargo.toml`"*). The two gate scripts the prompt names are
U1's (`U1_gate_supersession.md`:60–61). The premise is still true at this
revision:

```
$ grep -n -A6 "^\[workspace.dependencies\]" Cargo.toml
15-pistol-core   16-pistol-eval   17-pistol-search   18-pistol-engine   19-pistol-cli
$ grep -c pistol-solver crates/pistol-search/Cargo.toml
0
```

**§1's scope claims trace to their ADRs and to the ROADMAP.** D-310 (F and T
only, quiet stage and widening schedule defer to a follow-up carrying its own
design, matrix and SPRT); D-315 (WP-1.5c scheduled, `WPQ_seed.md` its input,
nothing inherited as settled, M2 a fresh matrix); D-313 (dominance pruning
deferred not dropped, scheduled with WP-1.5c, ROADMAP says so in its own text);
D-314 (*"WP-1.5b performs the PARENTHETICAL only … What moves the committed config
is the operator's SPRT"* — the prompt's *"nothing in this document authorizes
moving the committed default"* is the same claim); `docs/ROADMAP.md`:88 for
*"Neither includes any `pistol-eval` storage refactor; that is WP-1.9."* The M2
debt claim — *"Matrix M2 has never been authored in the form its own candidate
takes"* — is `WPQ_seed.md`'s THE M2 DEBT NOTE (:41–55): *"**M2 IS AN OPEN
SELECTION AND HAS NEVER BEEN IN A MATRIX IN ITS ADOPTED FORM** … So M2 is a
**FRESH matrix that has never been authored**, not a recovery."*

**The header's D-351 provenance claim is accurate.** D-351 (`docs/decisions.md`:747):
*"U1–U4, `WPQ_seed.md` and `section_owner_table.md` stop being REVIEW-design
subjects and become REFERENCE MATERIAL … `wp15b_impl_prompt.md`, to be authored
at this closure's step 3 from the units' normative content, is OWED as the new
governed design artifact IMPL will read."* §1's description of
`section_owner_table.md` as *"the index of which unit owns which part of the
superseded pre-carve design"* matches that table's own head (*"The map from
`docs/experiments/wp15b_design.md` at `6feb40a` … to the units the restructure
selected"*).

**§3.4's gate on the M1 red team is exact, cardinality included.**
`U3_tier_t.md` §U3-Z item 2 (U3:694–700): *"**GATED — MAJOR 6: this line may not
be written until a fresh DECISION-RED-TEAM has attacked M1 AS AMENDED.** … §6.5
records the attack against the option as it stood at revision 1, `ec8f7fb`,
before §6.1 flipped the reading from exact to threshold and before C was selected
under the flipped reading."* The prompt's *"the only attack M1 has ever had"* is
supported by U3-A's lineage table (U3:104–107): one DECISION-RED-TEAM against M1
at revision 1, then *"REVIEW-design | revisions 2–6 | all FAIL; **M1 was never
reopened on its merits**"* — and U3:883, *"**M1 AS AMENDED HAS NEVER BEEN
ATTACKED (MAJOR 6)**."*

**§6's U1 bullet is complete against U1-Z's OPEN list.** U1-Z OPEN (U1:293–302)
has exactly one bullet — *"The two clauses of §4.4's surviving attack that (f)
does not answer. A legitimate crate added inside the cone, and a workspace
version bump that is not a graph change at all"* — and the prompt carries both
clauses.

**§6's U3 and U4 bullets resolve to live OPEN content, none of it closed.**
Checked one by one against the units' OPEN lists: the self-completeness-claim
architect gap (U3-Z:734–755, live, and its U4 sibling at U4-Z:1796–1816); MAJOR
12's unmarked `23.2` with provenance undecided among three candidate cells
(U3-Z:757–772 and the table row at U3:805 — three candidates named, exactly as
the prompt says); the M1 red team (U3-Z item 2); the `quiet_top_k`/
`widen_schedule` D-scope (U3-Z:843–851, *"**The carve does not choose**"*); the
SEAM (U4-Z:1920–1930); N-E never attacked in its own right with D-333 ruling it
does not reopen the selection (U4-Z:1832 and `docs/decisions.md`:711); M3's
witness owed as a position and M6's second construction owed as a PARENT position
(U4-Z:1913–1919 and :1849–1856 — the prompt splits these correctly, which the
unit itself notes is a two-part residual); the snapshot's second instrument
(U4-Z:1862–1865, *"Replication is registered; the second instrument … are not"*);
SHELL_CHECKLIST reviews for both scripts (U4-Z:1866–1868). Every pointer lands on
live text; none lands on a struck or closed item.

**§8's finish-policy citations resolve.** `U2_node_protocol.md` §U2-T (U2:590),
`U3_tier_t.md` §U3-T (U3:474) and `U4_soundness_instrument.md` §U4-T (U4:1335)
all exist as test-row registries; `wp15b_sprt_prereg.md` §5 (:234) is *"Outcome
handling, written before game one"*, which is the right section for "reports a
verdict under that document's own §5"; §9.4 (:732–734) is the soundness gate slot
the prompt describes.

**The absence of u-rev labels is not charged**, per the dispatch and D-311's
appended amendment.

---

# REJECTED, WITH THE ATTEMPTED REPRODUCER

**1. §3.1 condition 2's failure mode differs from D-329's wording — traced and
dismissed.** The prompt says the basename-loop spelling leaves
*"`configs/spaced dir/instrument_v0.toml` reaching the record at exit 0"*; D-329
says *"measured twice as leaving the digest displaced at exit 0 under the COMPLETE
kind token."* I suspected an invented example. Reproducer:

```
$ grep -rn "spaced dir" docs/
docs/experiments/matrix_M4_axisA_selection.md:110:   that spelling leaves `configs/spaced dir/instrument_v0.toml` reaching the
docs/experiments/U4_soundness_instrument.md:902:>    leaves `configs/spaced dir/instrument_v0.toml` reaching the record at exit 0.
docs/experiments/U4_soundness_instrument.md:1141: … so copying it leaves `configs/spaced dir/instrument_v0.toml`
docs/experiments/matrix_M4_axisA_REDTEAM.md:617: … `configs/spaced dir/instrument_v0.toml` walks through at exit 0 with the digest
docs/experiments/matrix_M4_axisA_round4.md:143,146,170 …
```

The prompt's wording is verbatim from the selection record (:110) and from U4-Z's
own N-E conditions bullet (U4:902), which is what §3.1 cites. Not a finding: the
prompt reproduces the fuller of two accurate renderings of the same measurement.

**2. The truncated hard-rule-1 quotation.** The prompt quotes *"NO code-side
default for any tunable"* where D-329 and CLAUDE.md both continue *"— a default
lives in exactly one schema place."* I considered charging a selective quote,
since D-329 uses the dropped half against the clause (*"`configs/` is not one
schema, holding four engine configs, six arena match configs and a weights table"*).
Rejected: the prompt's use is the claim that N-E has no code-side default, which
is what the retained half states, and the dropped half is about where a default
lives — inapplicable to a row with none. The defect in that sentence is the
inference, charged separately as MINOR 3.

**3. D-263's "NONE of those" is a four-item list rendered as three.** The prompt
says D-263 *"states plainly that it carries no bracket, no abort threshold and no
bench — 'NONE of those is in this line.'"* D-263's antecedent list is four items,
the first being "a pre-registered hotspot" — which the line manifestly does
contain. Rejected as a finding against the prompt: the four-item reading is
self-contradictory in D-263's own text, the prompt's three-item reading is the
only coherent one, and charging it would be charging a wrinkle in the ADR line to
the document that reads it charitably.

**4. §6 is not exhaustive of the four units' OPEN lists.** MEASURED, §6 omits
from U4-Z: the U4-R restructure stop (D-334), the cross-unit-citation conversion
pass, the three MINORs F1/F2/F3, the gate's per-CI cost being ungrounded at its
dominant term (U4-Z:1936–1940, *"a re-derivation for S-M is owed"*), and item 15's
unreconciled blockage (U4-Z:1952–1962); and from U3-Z: B7's residual class and
the census's owed replication and second instrument. I attempted to charge this
as a completeness defect and could not substantiate it: §6 asserts no
completeness — *"Pointer-only, per unit's own OPEN list"* names its source rather
than certifying its extent — and D-351 keeps the units as REFERENCE material a
reader consults directly. The two IMPL-adjacent omissions (per-CI cost, item 15)
are worth an author's second look but are not findings against a document that
claims no exhaustive list. Recorded here so the next round does not re-derive it.

**5. §5's `SearchInfo.stages` content, apart from the WP-1.6 clause.** I checked
whether the rest of the bullet outran §U2-M item 2 and it does not: the five
rates (F/T/Q firing, filtered-node, `Cover::Impossible`, overload-return), the
"written from the same points `nodes`/`nps`/`time_ms` are, on every construction
path including both salvage ones", the zero-on-a-wall-clock-path consequence for
play-mode stage shares, and "the line protocol's output does not change" are all
U2:628–657 in substance and largely in wording. Only the WP-1.6 sentence is
unsupported (MAJOR 1).

**6. `configs/instrument_staged_v0.toml`'s absence as a fresh blocker.** I
checked whether §5 understates it, since U4-Z:1941–1945 makes it block the
snapshot's AFTER *"independently of M4's seam"*. Rejected: §5 lists the document
as an IMPL deliverable and §5's third bullet ties it to the SPRT pre-registration's
slots. The overstatement in that bullet is charged as MINOR 7; the absence itself
is handled.

---

*Review complete. Pin `cc6046121e23592454cd0e4f55d038dc8857518e`, matching HEAD,
tree clean, at both entry and exit.*
