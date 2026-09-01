# WP-2.0b DESIGN revision 8 — REVIEW-design (fresh context)

## Header

**Revision reviewed:** `b8cf9a0cbd0a598ed37fbe383cd6333c606c58fa` — a `git stash create`
object on top of `dev` HEAD `a56449b`. **The tree still matches at the end of this
review**: `git diff b8cf9a0 --stat` is empty on entry and on completion, and
`git worktree list` shows the main tree only. I created no worktree and modified no
file but this one.

I did not write this document and I am not any of the seven reviewers it answers.

**Read.** `docs/experiments/wp20b_design.md` in full (1134 lines);
`wp20b_design_rev7_REVIEW.md` in full; `wp20b_design_rev6_REVIEW.md` at its carried
findings; `CLAUDE.md`; the governing dispatch `wp20_dispatches.md` "WP-2.0b v2" in full
(its six-limb diff and the transcribed prompt); `wp21_DISPATCH.md` in full;
`docs/decisions.md` **D-249, D-253, D-291, D-423, D-424, D-465, D-512, D-517, D-522,
D-530, D-535, D-537, D-560, D-562, D-563, D-564**; `tools/baseline_snapshot.sh` header
and `:480-490`, `:568-600`; `tools/stage3_allocator_bound.py:125-140`;
`crates/pistol-core/src/symmetry.rs`, `zobrist.rs`, `state.rs:120-145`;
`crates/pistol-search/src/census.rs`, `pvs.rs:590-665`, `search.rs:195-320/375-540`,
`info.rs:230-250`; `crates/pistol-engine/src/engine.rs`, `position.rs:95-112`;
`crates/pistol-cli/src/budget_token.rs`, `protocol.rs:160-185`;
`crates/pistol-arena/src/capture.rs`, `capture_file.rs`, `labels.rs`, `passes.rs`,
`exchange.rs`; the three named config seats. **D-401 was not read.**

**Ran.**

- **§9's registered command block, VERBATIM**, extracted with `awk` from the document
  and executed in a profile-free `bash` (so `grep` is `/usr/bin/grep`, not the harness
  wrapper), `REPS=1`, `BIN=artifacts/pistol_prechange_a56449b`,
  seat `configs/bench_wp18c_solver_on.toml`. **AC1 is repaired.**
- **Timed the registered workload** from the same run's own `info totals` lines
  (20 searches at `nodes 50000` on the registered seat). **COST verified.**
- **Two further OFF-arm reps** of the same block to measure the instrument's own noise
  floor, which is the number §9's H1 needs and does not have.
- **Re-derived every cell of §1.1's table and of limb 5's search-share table**
  from the six `artifacts/wp20b_cap_out_*` files (`/usr/bin/grep`, `sed`,
  `LC_ALL=C sort -u`). **AC2 reproduces**, one digit excepted (AH1).
- **`sha256sum -c artifacts/wp20b_cap_RECEIPT.txt`** — twelve files, twelve `OK`.
- **Re-derived §9's three byte-identity digests** from
  `artifacts/prechange_*_run{1,2}.txt` and the pre-change binary. All three reproduce
  — and re-deriving them is how **AF1** was found.
- Verified `att_visits 11040 att_proved true` at
  `wp20b_cap_out_trigger-rich_on_16384.txt:34`, `won(row) = att_proved == "true"` at
  `tools/stage3_allocator_bound.py:133`, the 116 firings/ask mean (694/6 = 115.67),
  and **D-517's 9.05 firings per search** — reproduced exactly, 181 firings over 20
  searches at the registered seat and budget.

**Could not check.** The **22.99 µs** fold cost and the **24 000 (position, symmetry)
pairs**: both trace to `wp20b_decision_REDTEAM.md` rows R-COST and R-CHECK and neither
has an artifact in any receipt — the same gap rev 5, rev 6 and rev 7 all recorded.
Nothing in my verdict turns on either. D-560's ~119 800 ceiling and the 2.14x
duplication are marked ESTIMATED and inherited. I did not run `tools/ci.sh` or
`tools/determinism.sh`; no code has changed.

---

# THE DECISION

## 1. Is the DESIGN correct?

**Yes.** An implementer building exactly what §§2–8 specify produces a correct census
identity, a correct token, a correct sink and correct coldness. I attacked each and
verified each at source rather than by reading the document's own quotations:

- **Identity.** `canonical_key(stones) = canonical_form(stones).fold(ZERO, ^ cell_key)`
  is well-formed against the real signatures (`symmetry.rs:165` takes
  `&[(Coord, Player)]`, `zobrist` exports `cell_key(at, player)`), and it reaches
  **exactly** `wp20s_design.md` §8's equivalence: `crates/pistol-arena/src/labels.rs:203`
  is `key_full: render_key_full(&canonical_form(&stones))`, so C2 is `key_full`'s own
  fold in a second spelling. F2's mapping of options A and B onto `key_pos`
  (`labels.rs:202`, `state.key()`) and `key_seq` (`labels.rs:201`,
  `canonical_sequence`) is right at the source.
- **Side-to-move.** The objection that a stones-only key under-counts is answered, and
  the answer is the state machine's own: `state.rs:129-133` reads *"for an ongoing game
  the stone count fixes the turn, the phase and the mover together, so two positions
  this key cannot tell apart are the same position."* The one hole I looked for — game
  rule 4's truncated turn breaking the parity — is closed by the fact that a truncated
  turn is a decided position and a census firing is by construction not one
  (`pvs.rs:640-646` panics by name on a decided position).
- **Coldness.** `pvs.rs:604`'s `let (state, threats, _) = self.position.staged_context()`
  puts `state` inside the closure the key would be computed in, so `state.board().stones()`
  is reachable with no extra hashing on the non-census path. Verified at the exact
  line numbers the document cites — `:602`, `:604`, `:623`, `:635` all land.
- **The arming rule, which is the one place a wrong ordering would panic in
  production.** I checked the control flow the rule depends on: `self.census` is moved
  out at `search.rs:381` and back at `:525`, and **between those two lines there is no
  early return and no `?`** — every `return Err` is in `check_root` (`:538/:544/:547`),
  called at `:249` before the move-out, and the root-proof `return Ok` at `:319` is
  before it and pushes its row into `self.census` directly (`push_root_census`). So
  `take_trigger_census` cannot meet a `None` on any path a census `go` can take, and
  §6.1's stated order is sound rather than merely plausible.
- **The sink.** `capture.rs:172` returns `Step::Ignore` for any line starting `info `
  and `:229` `continue`s on it, so §3.1's "zero census bytes at exit 0" is real and the
  sink is genuinely load-bearing. `capture_sha256` digests `label_go <go_line>`
  (`:103-109`), so a census-on capture really does get a different digest, exactly as
  §3 says.

**Two gaps that are spec/test gaps rather than wrong answers** are recorded as AG4 and
AG5 below: the fourth-word refusal is a stated behavioural requirement with no test in
§8, and one of the dispatch's four named mutants is unaccounted for.

## 2. Is the DOCUMENT sufficient to build from?

**§§2–8: yes**, with three one-clause ambiguities an implementer resolves at the site
(AH2 — who populates `SearchOutcome`'s census field; AB5 carried — whether the census
file's payload keeps the `info census ` prefix; AB2 carried — the root site's capture
is not the tree site's). None of the three has a resolution that fails silently: every
wrong choice dies at test 1, 13 or 14.

**§9: no.** Its first registered obligation cannot be satisfied by any correct
implementation (AF1), and its registered hypothesis H1 is defined twice, inconsistently,
with no rejection region (AG2, AG3).

**§1.1's registration: no.** The calibration run's size is derived with the wrong
firing rate and its position source is unnamed (AF2).

## 3. VERDICT: **FALLS**

## 4. Where the blocker is, and whether surgery lands it

**The blocker is entirely in the DOCUMENT's registered instruments and advisory
sections. Nothing an implementer builds is wrong.** §§2–8 — C2, T1+T3, the wire format,
the coldness argument, the diff table, the arming rule, the artifact spec, the eight
invariants, the eighteen tests, the seven call-removed mutants — survived every attack I
could mount, as they did for the rev-6 and rev-7 reviewers, and I add no finding against
any of them that changes what is built.

**Yes, the package lands with surgery, and the surgery is not a design decision.** Both
blockers are corrections of transcription, and both corrections are computable from
artifacts already exported:

- **AF1** is one line and two digests. Replace §9's extraction rule with D-253's
  generalising wording and substitute the two referents **I have already computed**:
  `70e08fc506ec9e98…` (`gate_v0`) and `1846b99b59465bc1…` (`instrument_v0`).
- **AF2** is the deletion of two sentences — the multiplication block and the *"under
  an hour of machine time"* claim — from §1.1, from `wp21_DISPATCH.md` §4.4 and from
  D-563. §1.1's MEASUREMENT is what the operator needs and it is untouched by the
  deletion; the sizing is an add-on that D-424 does not protect, because it is a
  numeric claim and it is false.
- **AG1** is one clause in the ONE LINE FOR THE MORNING.

**Neither blocker needs a ninth design revision**, because neither is a choice between
options. There is nothing here for a design session to decide; there is text to correct
and two digests to paste.

## 5. What the operator should do with this package

**Hand §§2–8 to IMPL unchanged. Apply three corrections to §1.1, §9 and the headline as
transcription — not as a design revision — and let REVIEW-impl carry the rest.**

Concretely, in order:

1. **Correct §9's byte-identity obligation (AF1) before IMPL starts.** It is the
   package's first STOP condition and, as registered, it fires on every correct build.
   The digests above are turnkey; add the rider in AH5.
2. **Cut §1.1's calibration sizing (AF2)** — the block from *"Its size follows from
   this arc's own MEASURED inputs"* through *"answerable in under an hour of machine
   time"* — from all three documents, keeping the REGISTERED sentence (two arms, caps
   2048 and 16384, records excluded from the corpus) and dropping the number. Either
   re-derive it with the cap-16384 firing rate, or say the cost is unsized. The
   calibration run is not this package's to execute, so this cut blocks nothing.
3. **Fix the ONE LINE FOR THE MORNING's stale "tranche" (AG1).**
4. **Give H1 the one clause it needs (AG2, AG3) at the closure revision**, where §9
   already says the guard is re-registered and where a two-armed dry run costs seconds.
   The band must be registered before the guard runs — CLAUDE.md forbids the post-hoc
   move — and the noise floor I measured (below) is the number to register against.
5. **Add two rows to §8 (AG4, AG5)** at implementation time; both are tests, and
   REVIEW-impl is the right reader for them.

**Do not stop and split.** The substance has been stable since revision 5 and three
independent reviewers have now failed to break it. Splitting would spend another arc
re-deriving §§2–8.

---

# REPAIR AUDIT

## AC1 — DISCHARGED, and I ran it rather than read it

The block now prints `position="${line%% #*}"` — two percent signs — and the comment
quoting `tools/baseline_snapshot.sh:485` prints `${1%% #*}`, which is what that line
actually is (verified at the source: `tail_of() { printf '%s' "${1%% #*}"; }`).

I extracted the block with `awk '/^for rep in/,/^done$/'` — no retyping — prefixed
`REPS=1`, `FIXTURE`, `BIN`, and ran it under `bash --noprofile --norc` so `grep`
resolves to `/usr/bin/grep` and not the harness wrapper:

```
bestmove lines        20
bestmove 0,0           0
refusals, OFF arm      0
refusals, ON arm      20   `error Protocol: `go` takes one budget, and `census` follows it`
```

**Twenty entries, twenty real searches, no empty board.** The ON arm's twenty refusals
are the pre-change binary declining a token that does not exist yet, which §9's dry-run
paragraph states in its own words (*"Only the OFF arm can run"*); at the closure HEAD
both arms exist and the block's *"a refusal on any line VOIDS the run"* applies to a
binary that has the token.

One detail worth recording because it is the mechanism §3 depends on: the refusal text
interpolates the **third** word (`budget_token.rs:44-51`, `[_, _, extra, ..]`), which
is why the design's fourth-word requirement is a real change and not a restatement —
and why AG4 matters.

## AC2 — DISCHARGED on the measurement; the consequence is right and is not over-cautious

My independent re-derivation of `search_nodes / 400000` from the six exported files:

| entry | cap 2 048 | cap 16 384 |
|---|---|---|
| trigger-rich 0 | 15 619 (3.905 %) | 14 889 (3.722 %) |
| trigger-rich 1 | 5 464 (1.366 %) | 4 716 (1.179 %) |
| trigger-rich 2 | 24 614 (6.154 %) | 6 926 (1.732 %) |
| corpus 0 | 13 882 (3.471 %) | 3 752 (0.938 %) |
| corpus 1 | 40 162 (**10.041 %**) | 2 123 (0.531 %) |
| corpus 2 | 3 138 (0.785 %) | 1 097 (**0.274 %**) |

Every cell of §1.1's limb-5 table reproduces to the digit it prints, one excepted
(AH1: `10.03 %` should be `10.04 %`). `40 162 → 2 123` is **18.92x**, as stated. The
whole §1.1 headline table also reproduces: firings 294/41/400/63, distinct 49/25/26/12,
win-proofs 0/1/0/0, loss-proofs 4/0/0/0.

**Is the consequence right, or over-cautious?** It is right, and it is the cheaper
option on both axes, which is what settles it.

The design draws: a tranche split across two caps ships a production corpus labelled by
two instruments, so the cap comparison must be a dedicated calibration run excluded from
the corpus. Three things make that correct rather than cautious.

1. **The hazard is real and it is not a provenance column.** D-562(3) puts the census on
   the corpus's own games from game one, so a two-cap tranche is a two-cap *corpus*. At
   cap 16384 the search that produces the label receives 0.27–3.72 % of its budget
   against 0.78–10.04 % at 2048 — the label is the answer of a search that saw that
   share. These are two label distributions, not one with a flag.
2. **D-562(2)'s dedup destroys the flag even if you add one.** The registered default is
   *"one record per distinct position (three-key agreement), deeper label wins, tie to
   first"*, applied at corpus assembly. Two records of one position from two caps
   collapse to one by a rule that reads *depth*, and depth is exactly what the cap
   moves. A provenance column does not survive that.
3. **The calibration is strictly cheaper than a tranche**, so nothing is bought by the
   riskier route. (Its *stated* cost is wrong — AF2 — but even the corrected ~2 h is far
   under a tranche.)

The one thing the consequence does **not** establish, and the design does not claim it
does, is which cap is better. It establishes that the question may not be answered
inside a corpus tranche. That is the right scope.

## AD1 — DISCHARGED on direction; the referent is falsifiable; there is NO double-counting

**The common-mode argument is correct.** Writing `S` for the base search, `C` for the
correctly-guarded census cost and `L` for a leak paid regardless of the token, a
single-binary ON/OFF nps ratio reads `S/(S+C)` without the leak and `(S+L)/(S+L+C)` with
it — strictly closer to 1.000. In the sharpest case the criterion is exactly inverted: a
key hoisted out of the `is_some()` guard is paid on both arms *and removes `C` from the
ON arm*, so the ratio becomes **exactly 1.000** while a correct implementation sits
slightly below it. H1 = 1.000x on the ON/OFF ratio was a criterion the named defect
class not merely preserves but *produces*. The correction is right and it is the
correction `docs/process.md`'s vacuity rule demands.

**The cross-binary referent is falsifiable.** A leak is paid by the post-change binary's
token-OFF path and by nothing in `artifacts/pistol_prechange_a56449b`, so it moves that
ratio and nothing common-mode does. I confirmed the referent is executable today by
driving it at the registered seat and fixture.

**Its resolution, measured, because §9 asserts one without a number.** Three reps of the
OFF arm, same binary, same seat, same fixture, paired per position:

```
rep2/rep1 : mean 0.9990  sd 0.0072  min 0.9870  max 1.0149   (n = 20)
rep3/rep1 : mean 1.0040  sd 0.0077  min 0.9875  max 1.0208   (n = 20)
aggregate summed time: r1/r2 = 0.9963, r1/r3 = 1.0022
```

So the paired mean at the registered REPS=5 resolves to roughly **±1 %**, and a
per-position band below **±2 %** is noise. Against that: a fold leaked to per-node costs
`search_nodes × 22.99 µs` ≈ 14 853 × 22.99 µs ≈ **0.34 s on a 4.24 s search, ~8 %** —
caught with room to spare. A fold leaked one level up, to once per firing outside the
guard, costs 9.05 × 22.99 µs = **0.21 ms**, five orders below the floor — not caught,
and correctly excluded by test 17 instead, which §9 says.

**Double-counting with §9's byte-identity obligation: none.** The two obligations name
the same two binaries and share nothing else:

| | byte-identity | H1 |
|---|---|---|
| quantity | output bytes above `# timing` | nps |
| seats | `gate_v0`, `instrument_v0` — **gate OFF** | `bench_wp18c_solver_on` — **gate ON** |
| fixture | 24 standing positions @ 50 000 | 20 solver-bench entries @ 50 000 |
| what it can see | a leak that changes a move, a score or a node count | a leak that costs time and changes no output |
| what it is blind to | a key computed and discarded | a leak that changes output but not time |

A key computed per node and thrown away moves **no** invariant-block byte — node counts
are unchanged — and moves H1 by 8 %. They are complementary, and on top of that
byte-identity runs at a seat where by F3 the census closure is unreachable at all, so it
could not see a leak inside the firing path even in principle. **The design does not say
any of this**; one clause reconciling the two would help a successor, and under D-424 I
do not hold its absence against the verdict. What I do hold against it is that the
byte-identity half of the pair is broken — **AF1**.

## AD2 — DISCHARGED

The header no longer claims six families of finding IDs are cited at their sites; it
says *"Findings are cited where a claim in this document turns on one; the reports are
the record of the arc and this document does not restate them."* That is the cheaper
deletion AD2 named, and it is now true. The revision number, the review count and the
seven named reports are all correct.

## AD3 — DISCHARGED

§1.1 now registers *"the **measured pair**, cap 2048 and cap 16384 — not 512, which this
section never measured"*, and `wp21_DISPATCH.md` §4.4 names the same two caps. The
extrapolation across a 4x cap change is gone.

## AD4 — DISCHARGED IN FORM, FAILED IN ARITHMETIC

AD4 asked for the power composition rather than the cost claim. §1.1 now supplies a
composition — and the composition is wrong. See **AF2**. The three inputs I could check
are individually right (1/798 = 0.1253 %; 694/6 = 115.67; 53/3 = 17.67 s), the internal
multiplication is right, and the marking is honest about the pooled rate's weakness.
What is wrong is applying the small cap's firings-per-ask to the large cap's arm, in a
section that measured that rate collapsing two limbs above.

The composition's honesty marking (*"ten events per arm separates a 2x rate difference
and not a 1.3x one"*; *"bounds the rate from above rather than estimating it"*) **does**
honestly cover the weakness it names — the pooled rate resting on one event, and the
small-cap arm possibly returning nothing. It does not cover, and does not mention, the
converse: the single observed event is at the **large** cap, where the arm's own
observed rate is 1-in-104, **7.7x the pooled figure**. Pooling across the very variable
the run exists to separate is assuming the null in order to size the test for it. That
is a methodological blemish rather than a defect, because the run is registered at a
**fixed** 8 000 firings rather than run-until-10-events, so no optional stopping enters
and no conclusion moves. I record it and do not hold it against the verdict.

## The COST figure — DISCHARGED and VERIFIED BY MEASUREMENT

§9 now reads *"a MEASURED 4.75 s per search at `nodes 50000` on this seat (4 searches in
19 s, this revision, with `artifacts/pistol_prechange_a56449b`)"*, with the explicit
correction that the rate is **not** the dry run's.

My own measurement, from the twenty `info totals` lines of the verbatim run:

```
n = 20   sum 90 132 ms   mean 4 506.6 ms   min 1 ms   max 18 898 ms
=> 4.51 s per search  =>  200 searches = 15.0 min
first four searches: 4243 + 5623 + 4369 + 4246 = 18 481 ms  =  "4 searches in 19 s"
```

The stated provenance now reproduces to the second, and the ~16 min figure is right to
the digit (my 15.0 min differs only by which four searches the sample takes — the
distribution is heavily skewed, four of the twenty finishing under 100 ms). AE1 is
closed.

---

# ATTACKS ON REVISION 8's NEW CONTENT

## Limb 5 and the `att_visits 11040` argument — both hold

Limb 5's table reproduces (above). The `att_visits` argument is verified at the byte:
`artifacts/wp20b_cap_out_trigger-rich_on_16384.txt:34` carries
`att_visits 11040 att_proved true`, and 11 040 > 2 048, so the small cap forecloses that
proof structurally rather than searching and missing. It is the strongest single fact
in §1.1 and it is now on the page. The claim is correctly bounded — n = 1, no rate — and
correctly corroborated by D-530's independent mechanism rather than presented as one.

## The D-535/D-522 supersession paragraph — the reading is RIGHT

I read both ADR lines in full. D-535 is **RULING 1 OF THREE** and D-537 is **RULING 3 OF
THREE** — one operator ruling event, so there is no supersession between them to get
wrong. D-535 retires *"wins-only"* **as the census GATE's ranking direction**, on the
ground that D-512 names D-510's `proofs` counter, which increments at the attacker `Win`
arm and the defender `Win` arm alike, for the census denominator. It expressly preserves
*"the distinction itself: the two directions are different quantities, they are reported
separately, and a row's LOSS-side recall is not evidence about its WIN-side recall or
the reverse."*

D-537's clock is a different object: *"a registered minimum of **win-proving firings on
disjoint positions**"*, in the operator's own words. So the design's reading — the gate
ranks both, the clock counts wins, and the two proof columns are never summed — is
correct on both limbs, and the shipped instrument agrees:
`tools/stage3_allocator_bound.py:133`, `won(row) = row["att_proved"] == "true"`.

The rev-7 finding AE2 is discharged: the clause D-535 actually retired
(*"which is the gate's direction"*) is gone, and the citation now names the superseding
line.

## The calibration-run registration — see AF2

## H1's cross-binary referent — see AD1, AG2, AG3

---

# NEW FINDINGS

## BLOCKING

### AF1 — §9's byte-identity obligation is UNSATISFIABLE AS REGISTERED: the digested range contains `revision` and `binary_sha256`, both of which necessarily move, and D-253 settled this for this exact instrument

§9 registers, in its own words that this is the obligation's whole content:

> *"**BYTE-IDENTITY, GATE OFF.** Two-binary diff over the standing position set, output
> digest equal to the pre-change engine's. … **The extraction rule is stated exactly,
> because this obligation's whole content is which bytes are compared**:*
> ```
> sed -n '1,/^# timing/p' <record> | sha256sum      # the marker line INCLUDED
> ```
> *… `configs/gate_v0.toml` `7f8a6f97…`, `configs/instrument_v0.toml` `06490795…` …
> **The closure re-takes it against the post-change binary with the token off.**"*

**The invariant block's first five lines, read from the exported record:**

```
1  baseline_snapshot 1
2  schema 1
3  revision a56449baeebc3519385b32059d2dea76612d1554
4  binary_sha256 180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476
5  config /home/tom/.cache/wp20b-baseline/configs/gate_v0.toml 4af71088…
```

`tools/baseline_snapshot.sh:581` sets `REVISION="$(git rev-parse HEAD)"` and there is no
override; `BINARY_SHA256` is the digest of the binary under test. **Line 3 changes
because the closure HEAD is not `a56449b`. Line 4 changes because the post-change binary
contains new code in `pistol-core`, `pistol-search`, `pistol-engine` and `pistol-cli`,
all of which the shipped binary links.** So the registered comparison fails for every
possible post-change build, correct or not — and *"byte-identity mismatch"* is the first
item on §9's own STOP list. **The package as registered stops itself at closure
regardless of whether the code is right.**

**This is not an open question in this repository. D-253 decided it, for this
instrument, on a comparison of exactly this shape** (emphasis added):

> *"both the design session's migration and a reviewer's textually-different one produce
> a `tools/baseline_snapshot.sh` invariant block BYTE-IDENTICAL to stock on all 24
> positions, **differing only in `binary_sha256`**; this implementation reproduces that a
> third time, and **ITS PAIR DIFFERS IN TWO INVARIANT LINES AND NOT ONE — `revision` …
> as well as `binary_sha256` — because that third comparison SPANS A COMMIT** … D-249's
> wording for the same instrument — **byte-identical excluding the `revision` line** —
> is the one that generalises."*

The script's own header says the same thing from the other side: the block is invariant
across *"two runs **at the same revision**"*. §9's rule is the right rule for the
replication it already performed (two runs at `a56449b`, which is what the receipt
measures and what I reproduced) and the wrong rule for the cross-revision comparison the
closure must make. **The document's three digests are correct and its use of them is
not.**

**Why blocking rather than a transcription nit.** It is the same defect class as AC1 one
obligation up: a registered command that, executed as written, answers a different
question from the one it was registered to answer. Rev 7 fell on AC1 for exactly this,
and the reason it gave applies verbatim — the document declares its own registrations
reopened by an edit, so an implementer cannot fix it in place, and a closure receipt
that cites this rule while having used a different one is the AE1 defect the arc has
already paid for once.

**The repair, computed, so it costs no run.** Drop the two lines and substitute the two
referents. Both reproduce across the two pre-change runs of each config:

```
$ sed -n '1,/^# timing/p' <record> | /usr/bin/grep -v '^revision \|^binary_sha256 ' | sha256sum

gate_v0        run1 70e08fc506ec9e98f0843c8ea8f57ccb9daa8ca6538c8151f30a99bb1fe9f5be
gate_v0        run2 70e08fc506ec9e98f0843c8ea8f57ccb9daa8ca6538c8151f30a99bb1fe9f5be
instrument_v0  run1 1846b99b59465bc14b3232bad19a74d4da3dd9893d7a2fc3676ecd5da9aa754d
instrument_v0  run2 1846b99b59465bc14b3232bad19a74d4da3dd9893d7a2fc3676ecd5da9aa754d
```

`binary_sha256` is then checked separately and expected to **differ** — which is the
honest form, since a post-change binary whose digest did *not* move would mean the diff
never entered the binary. See **AH5** for the rider this repair needs.

**One thing AF1 makes retrospectively load-bearing, in the design's favour:** §4's
decision to add **no handshake line** is now doing more work than it claims. With the
corrected extraction rule the ten `engine_id` lines are still inside the digested range,
so an eleventh would break the comparison exactly as §4 says. That paragraph survives
AF1 intact and is strengthened by it.

### AF2 — §1.1's registered calibration run is sized with the SMALL cap's firing rate for BOTH arms; §1.1 limb 2 measures that rate collapsing 6.35x–7.17x two paragraphs above, so the run is ~2.1 h and not "under an hour" — and the figure has already reached D-563 and `wp21_DISPATCH.md`

§1.1's derivation:

```
pooled win-proof rate  = 1 in 798 firings          = 0.125 %
firings per ask        = 116                       (§2, MEASURED mean)
=> 10 win-proofs per arm needs 7 980 firings       = 69 asks
ON-seat ask cost       = 53 s / 3 asks             = 17.7 s
=> 20.3 min per arm, 40.5 min for both
```

followed by *"**So the question the operator has been asked to rule on unmeasured is
answerable in under an hour of machine time**"*, and **REGISTERED**: two arms, caps 2048
and 16384, **each run to a fixed 8 000 firings**.

**Every input is checkable and the arithmetic is internally consistent** — 1/798 =
0.1253 %, 7 980/116 = 68.8, 69 × 17.67 s = 20.4 min. **The defect is that `116` is a
cap-2048 number applied to both arms.** §2's basis is stated as `nodes 400000, cap 2048,
gate on`, and §1.1's own limb 2 reads: *"**Raw firings collapse — 294 → 41 (7.17x) and
400 → 63 (6.35x)** — because a call permitted eight times the visits fits into the budget
eight times less often."*

Re-derived from the same exported per-entry lines:

| fixture | cap | firings/ask | wall/ask | asks for 8 000 firings | arm cost |
|---|---|---|---|---|---|
| corpus | 2 048 | 133.3 | 17.67 s | 60 | **17.7 min** |
| corpus | 16 384 | **21.0** | 16.67 s | **381** | **105.8 min** |
| trigger-rich | 2 048 | 98.0 | 49.67 s | 82 | 67.6 min |
| trigger-rich | 16 384 | **13.7** | 41.67 s | **585** | **406 min** |

Using the design's own preferred inputs (corpus fixture, pooled rate on the small arm),
the registered run is **20.4 + 128 = ~2.5 h**; using corpus rates throughout, **~2.1 h**.
Either way the large-cap arm alone is **5.2x–6.3x** its stated 20.3 min and the total is
**3.0x–3.6x** the stated 40.5 min. *"Under an hour of machine time"* is false by the
section's own measurement, and the direction of the error flatters the recommendation
the section is making.

**And the registered run names no position source, which is the sharper half.** At cap
16384 it needs 381–585 asks. The only position sources §1.1 names are the two committed
bench fixtures — **24 and 20 entries**. A full pass of both at cap 16384 yields roughly
`24×21 + 20×13.7 ≈ 780` firings, an order short of 8 000; reaching 8 000 means sixteen
to nineteen passes over the same positions, and the engine is deterministic, so every
pass after the first reproduces the same firings, the same rows and the same keys. The
registered comparison is *"win-direction proofs on **disjoint keys**"* — the quantity
that stops accumulating after pass one. Under the alternative reading, that the
calibration runs on `book_v2` openings (the only reading under which *"its records are
excluded from the corpus by construction"* has content, since you exclude only
corpus-shaped records), the supply is fine and the cost is ~2.6 h. **The two readings
differ by a factor of about seven in cost and by everything in validity, and the
document does not say which it means.**

**Why blocking, and why it is nonetheless severable.** It is a REGISTERED run whose
stated affordability is falsified by the same section's own artifacts — the exact D-291
standard this document invokes against others one paragraph earlier
(*"this revision measured it rather than extrapolating (D-291)"*) — and the figure has
propagated to `wp21_DISPATCH.md` §4.4 and into **D-563**, the ADR line the operator will
read when ruling. But this package does not execute the run, §10.1 keeps the cap with
the operator, and **§1.1's measurement is untouched by deleting the sizing**. So the cut
in §4 of THE DECISION discharges it without a ninth revision.

## MAJOR

### AG1 — the ONE LINE FOR THE MORNING still recommends the tranche §1.1 withdraws, in the first paragraph of the document

`:9`: *"…so the cap is NOT settled here, and **§1.1 names the one tranche that would
settle it.**"*

`:299-300`: *"An earlier revision proposed that *'tranche one runs both caps, for the
price of one tranche'*. **That proposal is withdrawn on limb 5's own finding**"*, and
`:317`: *"the design registers that rather than a tranche."*

The document's summary states the proposal its body retracts, 290 lines apart, in the
line the operator reads first and the one the standing closure format puts at the top.
`wp21_DISPATCH.md` §4.4 carries the **corrected** text, so the harm is bounded — but
D-423 is precisely about this (*"A CLAIM THE DOCUMENT MAKES TWICE IS A DEFECT
WAITING"*), and the two spellings now disagree. One clause: *"§1.1 names the
forty-minute calibration run that would settle it"* — with the number handled per AF2.

### AG2 — H1 is defined twice, inconsistently, and the corrected referent inherits the old rationale

§9, first bullet: *"**H1: the ON/OFF nps ratio is `1.000x`.**"*
§9, second bullet: *"**H1's REFERENT IS THE PRE-CHANGE BINARY, NOT THE OTHER ARM** …
**H1 is the post-change binary's token-OFF nps against the PRE-CHANGE binary's**"*.

Both sentences name H1 and they name different measurements. The second explicitly
corrects the first, so a careful reader recovers the intent — but H1 is the registered
name that a closure receipt must report a number against, and the document gives it two
definitions. The design keeps both comparisons (the ON/OFF one for the 0.95 abort), so
both numbers will exist and the ambiguity is recoverable at closure; it should not have
to be.

**And the first bullet's justification is now attached to the wrong referent.** It reads
*"The fold's predicted cost at this seat and budget is 9.05 firings … times a MEASURED
22.99 µs, or ~0.21 ms per search, which is below what this comparison resolves."* That
is a statement about `C`, the ON-arm cost — which is absent from **both** sides of the
cross-binary comparison, since both run token-OFF. The reason the corrected H1 predicts
1.000x is simpler and stronger: **nothing is added to the token-OFF path at all**. Say
that instead.

(The `9.05` input is sound: I reproduced D-517's figure exactly at the registered seat
and budget — 181 firings over 20 searches, mean **9.05**.)

### AG3 — H1 has a referent but no rejection region, and the band must be registered before the guard runs

D-249's `1.000x` is falsifiable because its adjudication is **exact and noise-free**:
*"the baseline snapshot's INVARIANT block being byte-identical excluding the `revision`
line."* §9 transplants the number onto a real-valued, noisy timing ratio and states no
tolerance. The `0.95` abort is explicitly scoped to the other comparison
(*"The single-binary ON/OFF comparison is retained for the 0.95 gross-regression
abort"*), so as written H1 has no rejection region and a closure receipt reporting
"H1 = 0.981" would not be adjudicable.

CLAUDE.md forbids fixing this after the run (*"Pre-register verdicts before experiments
— no post-hoc threshold moves"*), so the band is owed **before** the guard's first run.
§9's own *"INSTRUMENT REVISION: the closure HEAD at which the guard runs"* is where it
belongs, and the number is measured above: paired per-position nps over three reps of
one binary gives **sd 0.0075, range 0.987–1.021**, so a **±2 % paired band at REPS=5** is
the honest floor and anything tighter is noise. That band still catches the leak class
that matters (a per-node fold is ~8 % at this seat) and correctly does not claim to
catch the one test 17 owns.

### AG4 — the fourth-word refusal is a stated requirement of §3 and §6 with no test in §8

§3: *"**A FOURTH word is refused naming the FOURTH word, not the token** — a refusal
that names the wrong token is one a driver cannot act on."* §6's diff table repeats it:
*"the optional third word, its named refusal, and the fourth-word refusal naming the
fourth word."*

§8's eighteen tests contain no row for it. Test 6 is
`a_third_word_that_is_not_the_census_token_is_refused_naming_it` and pins the third-word
case only. The mutant is obvious and survives: leave `[_, _, extra, ..]` as it is today
(`budget_token.rs:44-51`), which names the **third** word for a four-word line — I
confirmed that is exactly what the shipped binary does, since my ON-arm run produced
``and `census` follows it`` for `go nodes 50000 census`.

This is hard rule 3's territory (a named refusal) and CLAUDE.md's own rule that *"when a
comment and a test could carry the same fact, the test carries it."* One row in §8:
`a_fourth_word_is_refused_naming_the_fourth_word`, seat (b), no search needed.

### AG5 — the dispatch names four mutants; §8 addresses three and never says what became of the fourth

WP-2.0b v2's obligations list four by name:

```
identity column dropped   -> schema test dies          (§8 mutant 1 / test 1)   OK
token check call removed  -> zero-bytes test dies      (§8 / test 5)            OK
warm-table read introduced -> D-527 seat dies          -- ABSENT --
transposition ruling inverted -> fixture dies          (§8 / test 4)            OK
```

The third appears nowhere in §8's mutant table. The **answer** exists — §5's *"Cold-table
discipline is untouched because this package adds no table and reads none"* — and I
believe it: `canonical_form` reads the board and `cell_key` is pure in
`(q, r, colour, FIXED_SEED)`, so a warm-table-read mutant has no code to be generated
from. But §5 is 200 lines from §8, is not connected to the mutant, and §10 does not list
this among the departures it is otherwise scrupulous about. A REVIEW-impl checking the
design against its dispatch will look for the fourth mutant and not find it.

One clause under §8's mutant table: the warm-table mutant class is empty because the
package adds no table, and the closure states that rather than reporting three of four.

## MINOR

### AH1 — `10.03 %` is `10.04 %`

`40 162 / 400 000 = 10.0405 %`. §1.1's limb-5 table and its `0.78–10.03 %` range both
print `10.03`. The rev-7 reviewer derived `10.04` and so did I. No conclusion moves;
recorded because it is a number the document marks MEASURED and re-deriving it is one
division.

### AH2 — the document does not say who populates `SearchOutcome`'s census field

§6's `info.rs` row says *"`SearchOutcome` gains the census rows"*; §6.1's rule says the
engine's order is `collect → search → take → stop`. If `search()` itself filled the
field, `take_trigger_census` would return an empty vector and §6.1's *"take comes before
stop"* would have no content. The coherent reading is that the **engine** fills it —
`let mut outcome = searcher.search(...)?; outcome.census = searcher.take_trigger_census();
searcher.stop_trigger_census();` — and that reading is required, because `protocol.rs:171`
holds only a `dyn Engine` and has no other route to the rows.

Not blocking, because every wrong choice fails loudly rather than silently: filling the
field inside `search()` leaves `self.census` at `Some(vec![])`, `take` returns empty, and
tests 1, 13 and 14 die. But one clause naming the filler — and naming
`search.rs:525`'s overwrite block as the place, beside `outcome.info.solver_calls` — would
foreclose the one hazard the site itself warns about in its own
`REVIEW-impl W-1` comment: *"the salvage/fallback arms construct these as zero, and
without this overwrite a Deadline answer would break the registered sum law."* The root
early return at `:319` needs the same clause.

### AH3 — carried unrepaired from earlier rounds: AB1, AB2, AB4, AB5, AE3, AE4, AE5

All still stand at revision 8 and none changes the verdict.

- **AB1** — §6.2's *"every existing caller stay as they are"* is still contradicted by
  §6's own `capture.rs (run)` row. The intended claim is about the *return* contract.
- **AB2** — §5's *"the root site has the identical shape"*: the **guard** is identical
  (`search.rs:304-307`), the **capture** is not (`root_census_columns(&mut self.position)`
  against the tree closure's `state`). Milder than rev 6 made it: `state: &GameState` is
  the enclosing `search()`'s own parameter and is in scope at `:304`, so the implementer
  has a one-line path. One clause.
- **AB4** — §1.1 still cites `census.rs:41-58` (the struct body) for a split argued at
  `:36-40` (the doc comment). I read both; `:36-40` is the stronger citation.
- **AB5** — §6.2 still does not say whether the census file's payload keeps the
  `info census ` prefix, which fixes the `# body_sha256` and test 14's oracle. Either
  answer is defensible; the file cannot have both.
- **AE3 / AE4** — no clause was added registering a two-armed dry run at the closure
  revision, or naming REVIEW-impl as the discharge for the instrument revision no review
  has seen. AG3's band should be registered in the same clause.
- **AE5** — §1.1 still frames D-537's minimum as *"a numerator over a denominator"*. It
  is one count. Harmless in the direction it is used; D-424 covers leaving it.

### AH4 — the ground for rejecting D' overstates, though the decision is right

§2 rejects D' because *"its representative — 'the position whose key is least' — is not
`canonical_form`'s representative, so it IS the fourth notion of sameness."* Strictly,
D' computes the **same equivalence relation** — two positions related by a symmetry have
the same twelve-key multiset and therefore the same minimum — and only the chosen
representative differs, and the representative is never exposed. The decision is
nonetheless right and the document also gives the reason that carries it:
*"C2 inherits a meaning two reviews have passed; that is worth more than 23 µs a
firing"*, and 23 µs × 9.05 firings against a 4.5 s search is nothing. Recorded so a
successor re-opening D' does not think the equivalence was the objection.

### AH5 — AF1's repair needs a path rider

The invariant block also carries
`config /home/tom/.cache/wp20b-baseline/configs/gate_v0.toml <sha>` and
`engine_id config <the same path>`. Both are inside the digested range under the
corrected rule, and both encode the **worktree path** the pre-change record was taken in.
The closure's post-change run must therefore use a config at an identically-spelled path,
or those two lines move and the comparison fails for a reason that is not the diff. One
clause beside the corrected referents; the alternative — excluding a third and fourth
line — is worse, because `baseline_snapshot.sh`'s own header enumerates the four paths
above the marker deliberately.

### AH6 — test 16 could discharge the dispatch's wording literally at no cost

The dispatch's obligation reads *"census state newgame-cleared and seated if it exists."*
The design's answer is that the per-`go` disarm means `new_game` finds nothing to clear
(§6.1 point 3), which I verified is true of the real `Searcher::clear` (it touches table,
heuristics and solver, not `census`), and which is argued rather than silently departed
from. Test 16 drives two consecutive `go`s and not a `new_game` between them. Adding the
`new_game` changes no reading — that is exactly why it is minor — but it makes the test
answer the obligation in the obligation's own words.

---

# THE STRONGEST SURVIVING ATTACK ON REVISION 8

**Revision 8 was told to run its registered instrument instead of reading it. It ran the
one it had been caught on, and re-derived the one it had been caught on — and the
obligation printed four lines above the repaired block, whose three digests the same
revision quotes as MEASURED, has never been executed against the thing it exists to
compare.**

The pattern is now four rounds old and it is getting narrower each time, which is what
makes it worth naming rather than a fifth restatement. Rev 6 said the repairs were
validated against the review and not the world. Rev 7 said the verification stopped at
the boundary the previous reviewer had drawn. Revision 8 is better than both: it ran the
block, it exported a corrected summary, it regenerated a receipt that verifies twelve for
twelve, it re-pointed H1 at a referent that a leak actually moves, and it corrected a
provenance to a rate I reproduced to the second. Every one of those is a real improvement
and I confirmed every one of them by running it.

**But `7f8a6f97…` and `06490795…` were produced by comparing the pre-change binary with
itself.** That comparison is the one the receipt records, the one the design calls
*"MEASURED rather than quoted"*, and the only one anybody has ever run. The comparison the
obligation actually makes — pre-change against post-change, across a commit — has a
different answer for a structural reason that a landed ADR (D-253) worked out three
hundred lines of log ago, on this instrument, on a comparison of exactly this shape, and
wrote down as *"the one that generalises."* The design's §9 declares that *"this
obligation's whole content is which bytes are compared"*, states the rule exactly, and
states it wrong — so a correct implementation of this package would fail its own first
STOP condition and the closure would report a byte-identity mismatch caused by nothing.

The lesson the arc keeps buying and not yet owning is not *run the instrument*. It is
**run the instrument against the thing it will be run against.** A dry run at the wrong
budget was AE1. A block executed in the author's shell rather than as printed was AC1. A
digest taken from a binary against itself, standing in for a digest across a commit, is
the same move a third time — and this time it is in the obligation that decides whether
the package landed, rather than in the guard that decides whether it is fast.

The second-strongest attack is that revision 8 wrote a power composition because a
reviewer asked for one, and built it out of the one constant its own section had just
finished measuring as non-constant. AD4 asked *does half a tranche clear the bar*; the
answer supplied is a multiplication in which `116 firings per ask` — a cap-2048
measurement, stated as such in §2 — is silently made cap-independent two paragraphs after
limb 2 reports it collapsing 6.35x. The composition is what a reviewer asked for; the
input is what the section itself refuted. That is the same failure as AF1 wearing
arithmetic instead of a digest: a number carried across a boundary the document had
already measured as a boundary.

**What survives all of it, and should be said plainly because seven rounds of findings
can bury it: §§2–8 are right.** I went at the identity, the side-to-move argument, the
arming rule's panic path, the sink, the coldness proof and the emission ordering at the
source rather than through the document's quotations, and every one of them holds — the
citations land at the exact line numbers, `key_full` really is `canonical_form` folded,
the control flow between `search.rs:381` and `:525` really does have no early exit, and
the census really is dropped on the floor by `classify` today. **The thing to be built is
correct. What is wrong is two of the instruments that would judge it, and both are
correctable by substitution rather than by design.**
