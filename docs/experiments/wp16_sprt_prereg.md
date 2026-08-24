# WP-1.6 — SPRT pre-registration: threat-only quiescence (`defensive_only`) vs the committed staged policy

**Revision 8. UNREVIEWED. It amends THE HEADER (this block), §5, §7A.1, §10 and
§11 — and nothing else. It MOVES THE INSTRUMENT: `385631f` re-pins §7A.1's
statistics layer, so this review reopens WHOLE. This document does NOT govern a
run until a fresh review passes it.**

**WHAT REVISION 8 IS, IN ONE SENTENCE: IT DELETES THE PARAGRAPH THAT HAS FAILED
THREE REVIEWS RATHER THAN WRITING A FOURTH VERSION OF IT.**

Revisions 5, 6 and 7 each tried to partition exit 2 into a VOID and a REFUSED
REPORT, and each partition failed its governing review — D-419 MAJOR A (an
enumeration presented as complete that was not), D-421 MINOR 2 (a "closed list
of four" with a fifth found in minutes), D-423 MAJOR 1 (a rule that
misclassified its own example, and was applied to one of the two places the
document stated it). **One MAJOR per revision, every one inside the paragraph
written to fix the last one.**

The operator OVERRULED it (**D-424**). The ground is that both sides of the
partition had the SAME registered consequence — the run is not a measurement,
the verdict is not read, nothing is in evidence about either engine — so the
distinction licensed no different reading of any number and was protecting
nothing. Pre-registration exists to stop a reading being chosen after the
numbers are seen; a rule that cannot change any reading is prose a reviewer must
still attack. §5 now carries **one** exit-2 row, and what to do next is read off
the instrument's own printed message, which this document does not register.

**THE SECOND STRUCTURAL CHANGE: THE TAXONOMY IS STATED ONCE.** It lived in §5
AND §7A.1, so every fix had to land twice, and revision 7's landed once — the
document shipped self-contradicting (D-423 MAJOR 1). §7A.1 now states what the
INSTRUMENT does and POINTS at §5 for what may be concluded. CLAUDE.md carries
this as a standing rule.

**THE THIRD: THE REGISTERED CHECK IS A TEST, NOT A GREP.** Revision 6 offered
four greps for its exit-1 invariant; its reviewer MEASURED the invariant false
with stdout on `/dev/full` and observed that none of the four could have
detected it, because they read the file's TEXT while the defect was in its
RUNTIME behaviour. Revision 7 added a fifth grep; its reviewer showed that one
was green at the revision where the defect was live (D-423 MAJOR 2). `385631f`
fixes the instrument instead, and the check is now the driving test that runs
the shipped script against a full device — a check three named mutations make
FAIL, which is what "a check the defect could falsify" means.

`385631f` also closes **D-422**'s standing debt in the same pass: the instrument
refuses a path that is not a regular file, where it used to block forever on a
FIFO. That debt was deferred because fixing it would move the governing
revision; the revision was moving anyway.

**Revision 7 (`1618467`) FAILED its scoped re-review — 0 BLOCKING, 2 MAJOR, 2
MINOR (D-423)** — which exhausted the cap the architect had pre-registered on
revision 6's review, and stopped the session. Revision 8 exists because the
operator then authorized the full fix rather than a fourth narrowing.

**Revision 6 (`3a198de`) amended the header, §5, §7, §7A.1, §10 and §11, and
was the first revision of this document that was NOT document-only.** Revisions
3, 4
and 5 could each say that `git diff --stat bfdf933..HEAD -- crates/ tools/`
prints nothing, and each did say it. **Revision 6 cannot say it and does not
pretend to**: commit `43e8a86` changes `tools/wp16_warm_attribution_check.py`,
which is THIS CRITERION'S OWN INSTRUMENT. Its governing revision therefore
MOVES from `bfdf933` to `43e8a86`; §7A.1's pin and §10's table are re-recorded
accordingly; and the sentence "commits after `bfdf933` on this branch are
DOCUMENT-ONLY" is RETIRED rather than quietly left standing where it has become
false. **The two BINARY digests are NOT affected and are NOT re-recorded** —
`43e8a86` touches `tools/` and one test file, nothing a release binary is built
from — and §7A.1 states that with the command that checks it.

**WHY THERE IS A REVISION 6.** Revision 5 (`de53f5d`) FAILED its governing
review — 0 BLOCKING, 2 MAJOR, 7 MINOR (**D-419**, report at
`docs/experiments/wp16_prereg_rev5_REVIEW.md`). The receipts rule revision 5
introduced WORKED, and both MAJORs are its dividend: the reviewer TESTED
registered receipts instead of reading them, and one of them was false for a
reachable input.

- **MAJOR B is closed IN CODE by `43e8a86`**, not by narrowing the document's
  receipt down to whatever the instrument happened to do. §7A.1 registered
  "Each prints under `CANNOT READ:` and exits **2**" of the three premise
  refusals; for a pair-mate whose move list stops inside the book, the
  instrument instead raised an uncaught `StopIteration`, printed no
  `CANNOT READ:` line, and exited **1** — the code this document registers as
  THE RUN IS NOT A MEASUREMENT, a finding about the ENGINES. A refusal wearing
  a finding's exit code. The architect licensed the instrument fix; the receipt
  is now TRUE rather than narrowed, and it has a fourth premise row of its own.
- **MAJOR A is closed by replacing an ENUMERATION with an INVARIANT.** Revision
  5 claimed "every one of those messages is quoted verbatim above" of its
  exit-2 partition while the instrument had 49 `die()` sites and §7A.1 quoted
  7. Revision 6 does not answer that with a longer list — a longer list is the
  same defect standing further away, and the same defect is what `43e8a86`
  removed from the instrument's own exception handler. Kind (i) becomes the
  CLOSED list it genuinely can be, kind (ii) becomes a REGISTERED CATCH-ALL
  residue, and the reader gets a rule that partitions the whole space instead
  of an enumeration to audit.
- **MINORs C through I are closed** in the sections they land in: a stale
  sample count (§7), a contradictory step number (§7), an estimate that was
  derivable from this document's own measurements (§7), a double pin (§10), an
  incomplete header list (this block), an over-claimed justification clause
  (§7A.1), and three missing receipts (§7A.1).

**WHY THERE WAS A REVISION 5.** Revision 4 (`20f9b26`) FAILED its governing
review — 1 BLOCKING, 2 MAJOR, 5 MINOR (**D-416**, report at
`docs/experiments/wp16_prereg_rev4_REVIEW.md`) — and the dispatch's own rule
for that step is a STOP, which is where the session stopped. The architect
authorized this revision. **The BLOCKING was that §7A.1 registered a criterion
the shipped instrument no longer implements**: the fix round after the
implementation review (**D-414**) added three premise refusals, a `status`
cross-check and a halt invariant to the INSTRUMENT, and the governing document
was never brought back into line — while §7A.1's "verbatim" quote of Criterion
1'' silently truncated the sentence describing a FAILURE path the instrument
runs. Revision 5 is the document catching up to the instrument, under a rule
stated at the head of §7A.1 and applied throughout it: **every criterion
sentence carries a receipt from the shipped instrument — an exit code, a quoted
message, or a driving test name — and no paraphrase survives.** **NO
INSTRUMENT CODE WAS TOUCHED BY REVISION 5**: `git diff --stat
bfdf933..de53f5d -- crates/ tools/` prints nothing, and every finding D-416
raised was a document finding, the instrument being stricter than the document
said rather than looser. (Written against revision 5's own SHA rather than
`HEAD`, because at revision 6 the `HEAD` form no longer prints nothing and a
historical claim must not be phrased so that a later commit falsifies it.)

**WHY THERE WAS A REVISION 4, recorded rather than folded into revision 3.**
Revision 3 was committed (`8820e91`) and its review was dispatched. Before that
review returned, this session found that §3 pinned `openings_skip = 0` — the
EXACT slice D-401's retired run drew — while the dispatch commissioning this
work calls for a FRESH opening slice at Step 7. The review was withdrawn rather
than allowed to finish, because a PASS on revision 3 would not have transferred
to the amended document and the amendment would have reopened it anyway
(CLAUDE.md: "an amendment reopens the review, however small the diff"). One
review at the governing revision is both cheaper and the rule. Revision 3 is
recorded here as committed-and-superseded, not as never-written: it governed
nothing and no run was taken under it.

**WHAT REVISION 3 CHANGES, AND WHY, IN ONE PARAGRAPH.** Revision 2 governed
the run D-401 took. That run FAILED its own Criterion 1' on clause (b) — 44 of
141 pairs (31.2%) link-1a-vacuous, and adversarially reassigning them moved
the verdict — so it was not a measurement, and D-402 retired it as evidence
for anything this WP concludes, under any criterion, ever. It is not re-read
here and nothing in this revision depends on it. What follows from that
failure is that Criterion 1' had an ARCHITECTURAL ceiling, not a tuning
problem: a cold subprocess cannot reproduce a warm engine past its first
search (D-383, MEASURED). Warm replay was licensed by D-384 and has now been
BUILT (D-407 through D-412), after three design-review rounds were closed in
favour of settling the mechanism by implementation. **Criterion 1'' replaces
Criterion 1' in §7A.1**, its new instruments are named with their governing
revision, its consequences are registered per exit code, a SECOND INSTRUMENT
and its agreement criterion are registered before either runs, and §8.6
records the new instruments' own dry run — two arms, one honest and one
seeded, each with the defect class it excludes. **Revision 4 added `openings_skip
= 500` to §3**, a fresh slice. **Revision 5 rewrote §7A.1 and corrected §5, §7,
§8.6, §10 AND §11, and rewrote this header block.** **Revision 6 amends this
header block, §5, §7, §7A.1, §10 and §11.** Stated exactly — and stated so as
to include the sections a revision rewrites while describing itself — because
this sentence has now been wrong twice, in two different ways, and a reviewer
had to find it both times: revision 4's version was wrong about §3 and §8.5
(D-416 MINOR 4), and revision 5's omitted §11, which it substantially rewrote
and whose rewrite WAS D-416's MAJOR 3 fix, and omitted the header block it was
itself rewriting (D-419 MINOR G). §3 is touched in TWO rows (`openings_skip`,
and a sentence added to `openings_take` saying §2's floor is unaffected); §8.5
IS touched, by a four-line paragraph saying §8.1-§8.5 govern revision 2's
instruments and §8.6 does not amend them; §9 gains §9.2a for the skip.
**UNTOUCHED across revisions 3, 4, 5 and 6: §1, §2, §4, §6, §7A.2, and §8.1
through §8.4.**

**Revision 2 (`731150a`) PASSED its own review (D-400) and governed the D-401
run.** Everything below that revision 3 does not name is revision 2's text,
unchanged.

**Revision 1 (`43b5d78`) FAILED its first fresh-context review** — one
BLOCKING defect and one MAJOR finding, both independently verified by the
reviewer against the live tree (config hashes, the `identity_lines` handshake
claim, a fresh from-scratch re-execution of the entire dry run byte-for-byte,
and the arena's own `Verdict` enum against §5's table — all of it held).
**BLOCKING**: §7A.2's agreement criterion let an `h1` verdict satisfy its
AGREEMENT clause unconditionally ("regardless of the bucket count") while its
own DISAGREEMENT clause was separately defined to fire on the identical
case — a contradiction with no stated precedence, exactly the
after-the-numbers ambiguity CLAUDE.md's registered-consequence rule forbids.
Fixed: the criterion is now a clean partition on the bucket count ALONE,
independent of the verdict; the "tactical correctness needn't mean shallower"
reasoning moved to how a DISAGREEMENT is investigated, not to whether the
criterion fires. **MAJOR**: §2's pair-floor paragraph asserted the 100-pair
floor stays conservative at `elo1 = 15` directionally, without computing
anything — fixed with an actual few-seconds derivation off `sprt.rs`'s own
LLR formula, solving for the `t_hat` D-190's own empirical elo1=25 crossings
imply and re-solving at `elo1 = 15` under the same decisiveness (~16.6 pairs
uncapped, ~10.0 capped — both ESTIMATES, both an order of magnitude under the
floor). This is an amendment, not a fresh draft: it reopens revision 1's
FAILED review exactly as CLAUDE.md's rule requires. §7A.1, §8, and every ADR
citation the reviewer independently re-verified are untouched.

**Provenance, stated so a reviewer does not have to reconstruct it from the ADR
log.** `docs/wp16_quiescence_design.md` revision 5 (`96c856c`) passed its fifth
fresh-context review (`wp16_design_REVIEW_rev5.md`) and closed the design gate
(`docs/decisions.md` **D-394**). Phase 2's IMPL landed the mechanism
(`de05f9c`) and its own rule-5 registered bench — `<= 2.0x` node-ratio pass,
`> 3.0x` ttd-ratio ABORT (**D-388**) — fired the ABORT outright even at the
narrowest tested setting: `q_depth_turns = 1`, both triggers live, node-ratio
2.57x, **ttd-ratio 4.52x** (**D-395**). The architect ruled (**D-396**) that
D-388's bracket stands exactly as fired and is retired as this WP's arbiter;
the governing question becomes whether the extension's STRENGTH gain outweighs
its MEASURED cost, a question only an SPRT can answer, and gated the wider,
costlier of the two triggers behind a new `q_triggers` config enum so the
FIRST SPRT tests the cheapest coherent hypothesis: `defensive_only` — trigger
(b) alone, never trigger (c). That delta landed (`8fe5abd`) and passed a
scoped fresh-context REVIEW-impl (**D-397**). A second, narrower rule-5-style
bench at `defensive_only`/`q_depth_turns = 1` measured node-ratio 2.48x,
**ttd-ratio 2.80x** (**D-398**) — cheaper than the full-trigger figure, exactly
as D-396's own rationale for gating the wider arm first predicts, but recorded
as MEASURED ACCOUNTING and explicitly **not** read against D-388's retired
bracket.

**This document is what D-396 calls Step 5**: the pre-registration for the
governed run Step 6 launches. It is a fresh draft, not an amendment — WP-1.6
has never had a prereg before this one — but it deliberately reuses
`docs/experiments/wp15b_sprt_prereg.md`'s revision-6 structure and its
Criterion 1' instrument verbatim, per the dispatch's own instruction, rather
than re-deriving a pattern this project already fought five review rounds to
get right.

**The run is this session's, per D-396's own delegation record**: "operator
authorization for this dispatch's slot-filling and launch actions (Steps 3-6,
including the governed SPRT run) is GRANTED and delegated to this session."
Unlike WP-1.5b's document, `elo1`, `openings_take` and `alpha`/`beta` are
therefore not **OPERATOR-CONFIRM** slots here — the dispatch fixes them
directly (§9.1, §9.3) — and this document's own §9 states plainly which slots
that delegation covers and which two (the run's binary digest and revision)
can only be filled at Step 6's own launch, by the same "rebuild means
re-record" discipline every SPRT document in this project uses.

---

## 1. What is being judged, and what is not

**The claim.** Threat-only, zone-bounded quiescence at the search horizon,
gated to trigger (b) only (`docs/wp16_quiescence_design.md` §3.2,
`q_triggers = "defensive_only"`, `q_depth_turns = 1`) is stronger than the
committed plain staged policy (`configs/instrument_staged_v0.toml`,
`q_depth_turns = 0` — the extension compiled in but never granted) at a fixed
node budget.

**What this run cannot judge, said first because rule 6 is about what a number
means.** SPRT judges STRENGTH, not soundness. Soundness is
`docs/wp16_quiescence_design.md`'s own gate — five fresh-context design
reviews, closed at D-394 — and Phase 2's correctness judges (`cargo test
--workspace`, `clippy -D clippy::all`, `tools/determinism.sh`,
`tools/perft_check.sh`, `tools/search_oracle_check.sh`, D-395's own header). A
green SPRT over an unsound extension is a measurement of nothing, and the
order matters here exactly as it did in WP-1.5b: §9.4 makes the design's gate
being green at the run's revision an explicit slot rather than an assumption.

**One axis separates the two seats, not several.** Unlike WP-1.5b (three axes:
selection, visibility, search value), this matchup differs on exactly one:
whether the horizon may extend by one further TURN under trigger (b) alone.
Candidate generation (staged tiers, the quiet cut), the quiet radius, and
everything else `configs/instrument_staged_v0.toml` and
`configs/instrument_staged_q_defensive_only_v0.toml` share is held fixed —
**CONFIRMED by diff, this session**: the two documents differ in exactly ONE
line, `q_depth_turns` (`0` vs `1`); `q_triggers` already reads
`"defensive_only"` on the committed baseline too (D-397's own committed
configs), so this run's ENTIRE claim is carried by that single key going from
`0` (the extension compiled in but never granted) to `1` (one turn of
extension granted under trigger (b)). **A verdict here is therefore a verdict
on trigger (b) plus its ply-1/ply-2 realisation and THE COMPLETION STONE, not
on the wider trigger (c) — D-396's own scoping, restated so a reader does not
read more into an `h1` than the matchup supports.**

---

## 2. The hypothesis and the verdict unit

- **H0**: `elo0 = 0` — the quiescence-extended seat is no stronger.
- **H1**: `elo1 = 15.0` — **FIXED by D-396's dispatch, not an OPERATOR-CONFIRM
  slot** (contrast WP-1.5b §9.1). Lower than WP-1.5b's `25.0`: that run judged
  a structural generation change with a MEASURED 2.09x-2.17x branching
  reduction as its prior; this run's own prior is D-395/D-398's cost
  accounting alone, which predicts no comparable branching effect — quiescence
  buys tactical accuracy at the horizon, not a narrower tree, and a smaller
  alternative is the honest target for that kind of gain (§6 expands this).
- `alpha = 0.05`, `beta = 0.05` — the values every prior SPRT run in this
  project used, kept for the same reason WP-1.5b kept them: two documents
  should not appear to disagree about a convention.
- **Verdict unit: the PAIR.** Both games of an opening are played, colours
  reversed, and the pentanomial pair outcome is the sample — `sprt.rs`'s own
  unit, unchanged from WP-1.5b.

**The pair floor: 100 pairs**, the same standing convention D-190 established
and every SPRT document in this project since has used (D-386's own
confirmatory run cleared it at 51 pairs). **Not re-derived from scratch for
`elo1 = 15`, but the directional claim IS computed rather than asserted** —
found in seconds from `crates/pistol-arena/src/sprt.rs`'s own LLR formula
rather than left as a hand-wave (CLAUDE.md's own "an estimate that could have
been measured in seconds is a finding" instruction), since a first draft of
this document did leave it as a directional assertion and that is a real gap
a reviewer named. D-190's "ten pairs" (uncapped) and "six pairs" (capped) at
`elo1 = 25` are EMPIRICAL crossings, not closed-form minimums — the LLR's
denominator `t1 * t_hat - t1^2 / 2` is unbounded as the sample variance
shrinks, so no first-principles minimum exists independent of an assumed
pentanomial shape. **What is computable**: solving `n * (t1(25) * t_hat -
t1(25)^2/2) = h1` for the `t_hat` D-190's own two empirical crossings imply
(`h1 = ln(0.95/0.05) = 2.944…`, this run's own bound too — alpha/beta
unchanged), then re-solving for `n` at `t1(15)` under the SAME implied
`t_hat` (the same DECISIVENESS of sample, not the same effect size): the
uncapped shape crosses at **~16.6 pairs** and the capped shape at **~10.0
pairs**. Both ESTIMATES, not measured — they assume this run's own sample
would be exactly as decisive as D-190's, which nothing here confirms — but
both are a full order of magnitude under the 100-pair floor, so the floor's
own protection (§5's confirmatory-run row fires below it regardless of where
the true minimum sits) holds with room, not merely by directional assertion.

---

## 3. The instrument

| | |
|---|---|
| arena | `target/release/arena`, built `--release --locked` at the run's revision |
| engine A | `configs/instrument_staged_q_defensive_only_v0.toml` — the quiescence-extended seat, label `staged_q` |
| engine B | `configs/instrument_staged_v0.toml` — the committed plain staged policy, unchanged, label `staged` |
| binaries | both seats run `target/release/pistol`, bound by `binary_sha256` (D-283 as qualified by D-294); recorded at Step 6's launch (§9.2) |
| book | `crates/pistol-cli/tests/fixtures/random_openings_v1.txt` — the PRIMARY SPRT book, 2000 openings, sha-pinned |
| `openings_take` | **500 — FIXED by D-396's dispatch**, not an OPERATOR-CONFIRM slot. Bound at `>= 100` by §2's floor; comfortably clears it. UNCHANGED by revision 4, so §2's floor derivation is untouched: it turns on the pair COUNT, which is `take`, and not on which window they come from |
| `openings_skip` | **500 — a FRESH SLICE, and revision 4's only substantive change.** D-401's retired run drew `skip 0, take 500`; this draws the next 500, disjoint from it by construction (docs/decisions.md D-202's own knob, and D-143: the book is emitted in content-hash order, so any window is as much a sample as a prefix is). The skip is inside `experiment_sha256` (D-202), so the two are formally different experiments and no reader can mistake this run for a re-read of one that is retired under any criterion, ever (D-402) |
| budget | `kind = "nodes"`, `value = 50000` — the registered snapshot budget every SPRT document in this project uses, so this run's numbers are comparable with the record. **NOT the same convention D-398 measured under**: D-398's own cost accounting used a FIXED-DEPTH methodology (`go depth_turns 3`, reading nodes/time to reach it), the inverse of the arena's FIXED-NODES-per-move convention this run uses. §7A.2's own agreement criterion is the instrument that reconnects the two conventions for THIS run's own matchup, at THIS budget |
| `turn_cap` | 40, as every prior run in this project |
| `n_workers` | 4 — WP-1.3's red-team clearance, unchanged |
| `hang_timeout_ms` | 120000 — liveness only, never an adjudication (D-159). **Freshly calibrated for THIS seat, §9.5** |

**Engine identity closes over the config and the weights** (D-198, D-199,
D-283 as qualified by D-294): the arena re-digests each seat's document before
every spawn and refuses a mismatch by name.

**The document that IS the experiment** is
`configs/arena_wp16_defensive_only_vs_staged.toml`, authored and committed at
Step 6's own launch (mirroring D-147/D-161: the report's `experiment_sha256`
is what a replication compares, not this file's path) — not authored here,
since it needs the launch-time binary digest §9.2 records.

---

## 4. What the run reports, and which lines are read

**Identical to WP-1.5b §4** — the same arena, the same `conclusion.rs`/
`report.rs`, unchanged by this WP. Reproduced here rather than referenced, per
this project's own "a pre-registration states what it needs on its own face"
practice:

```
arena_report <schema>
arena_version <v>
experiment_sha256 <hex>
openings_file <path>
openings_body_sha256 <hex>
openings_take <k> of <total>
openings_skip <k>
opening_turns <t>
game_cap <n>
budget <kind> <value>
turn_cap <t>
sprt elo0 <a> elo1 <b> alpha <c> beta <d>
bounds h0 <a> h1 <b>
engine <slot> label <l> binary <p> binary_sha256 <hex> config <p> config_sha256 <hex> weights_sha256 <hex>
engine_id <slot> <line>
game <i> opening <o> p1 <label> p2 <label> result <token> end <normal|forfeit> forfeit_by <label|none> reason <token|none> turns <t> dup_of <j|none> nodes_a <n> nodes_b <n> depth_a <d> depth_b <d> llr_game <x|none> llr_pair <x|none>
moves <i> <turn> <turn> …
refusal <i> <verbatim>
pair <i> opening <o> bucket p<k> score_a <x>
counts n <n> distinct_n <d> wins_a <w> capped <c> losses_a <l> forfeits <f> decided <k>
pentanomial p0 <a> p1 <b> p2 <c> p3 <d> p4 <e>
capped_fraction <x>
first_player_wins <k> of <n> decided_non_forfeit forfeits <f>[ conditional]
llr_game last <x|none>
llr_pair last <x|none>
nelo_pair <x> ci95 <y>
verdict <token>
verdict_unit pair
verdict_if_clean <token|none> pairs_dropped <k>
```

and below the `# timing` marker, machine-dependent and excluded from every
comparison:

```
timing n_workers <w> wall_ms <t> discarded_in_flight <k> hang_timeout_ms <t>
timing config_sha256 <hex>
timing_engine <slot> time_ms <t> searches <s>
```

**Per-side compute is a reporting requirement** (rule 6), taken from the two
`timing_engine` lines and reported alongside the verdict, not as an input to
it.

---

## 5. Outcome handling, written before game one

Fixed here so that no reading is chosen after the numbers — the same table
shape WP-1.5b registered and D-386 executed exactly as written.

| Outcome | Action |
|---|---|
| `verdict h1` **at or above 100 pairs** | The `defensive_only` extension is accepted as stronger AT THIS BUDGET AND BOOK. Per D-396's own FLIP clause, the WP closes: the committed instrument config moves to `staged_q(defensive_only, q_depth_turns=1)` in a separate commit after the run, subject to the closure gate in §6 below — never during a live sample. `defensive_and_offensive` is recorded as licensed-not-scheduled future work, exactly as D-396 states |
| `verdict h1` **below 100 pairs** | The action is WITHHELD. A confirmatory run on the DISJOINT book decides it: the same document with `openings_file` changed to `crates/pistol-cli/tests/fixtures/openings_v1.txt` (1591 openings) and `openings_take` re-stated, D-190's own instrument, exactly as D-385/D-386 executed for WP-1.5b. Both intervals are then reported together and both are read as optimistic (D-190's own coverage measurement) |
| `verdict inconclusive_degenerate` **with `distinct_n == n`** | Read the direction off the pentanomial, per WP-1.5b's own corrected reading (`Sample::is_degenerate` fires on a `p0` sweep exactly as on a `p4` sweep). Let `k` be the one occupied pentanomial slot. `k = 3` or `k = 4` → the H1 row above, subject to the same 100-pair floor. `k = 2` → the `h0` row — the modal outcome for two closely matched deterministic engines, and also what an all-capped run produces; report `capped_fraction` beside `k` since the two causes are different findings about the same number. `k = 0` or `k = 1` → the `h0` row. In every case `llr_pair last none`, so no crossing is quoted |
| `verdict inconclusive_degenerate` **with `distinct_n == n/2`** | The two seats played identical games — a document or digest error for two seats that differ in `q_depth_turns`/`q_triggers` by construction. Investigate the instrument, not the engine |
| `verdict h0` | The extension is NOT accepted. **This is a planning finding, never a threshold move.** Per D-396's FLIP clause: the WHOLE WP returns to the architect, with BOTH narrowing (a cheaper move-set width than `defensive_only` already is) and shelving (WP-1.6 parked, unimplemented in the committed instrument) explicitly on the table — this document does not pre-select between them. See §6 |
| `verdict inconclusive_at_game_cap` | Reported as inconclusive. No action. The sample is reported with its LLR and its distance from both bounds |
| `invalid_forfeit`, or `forfeits > 0` at any verdict | **The run is not a measurement.** Reported rather than discarded; re-run only after the forfeit's cause is found (D-158) |
| A PRE-GAME refusal — digest mismatch, an openings-digest mismatch, a config refusal, or an `--out` path that exists | **exit 2 and NO REPORT AT ALL.** Re-record the digest (§9.2) and re-launch; nothing has been measured |
| `arena_report_aborted` | No verdict exists. The games are a diagnostic and explicitly not a sample |
| **Criterion 1'' fails on the GOVERNED report — exit 1** (§7A, Doubt 1) | **The run is not a measurement, and it is not an `h0` either.** The verdict is not read and the committed instrument config does not move in either direction. See §7A.1 |
| **Exit 2 — NO ANSWER WAS TAKEN.** One consequence, no kinds. The instrument refused to read the two documents, and the reason is printed under `warm_attribution_check: CANNOT READ:` | **The run is not a measurement. The verdict is not read. Nothing about either engine is in evidence, in either direction.** In particular exit 2 is never an `h0`, never an attribution failure, and never evidence about the extension. What to do next — re-run, or go and find out how a report the arena could not have written came to exist — is read off the instrument's own message, which names the path, the record and the field. That is operational and this document does not register it. **Revisions 5, 6 and 7 each tried to partition this row into a VOID and a REFUSED REPORT and each partition failed its review** (D-419 MAJOR A, D-421 MINOR 2, D-423 MAJOR 1); the two sides always had the SAME consequence above, so the distinction licensed no different reading and was deleted rather than refined a fourth time (**D-424**) |
| **A DETERMINISM VIOLATION — exit 3** | **A hard stop bigger than this WP**, with TWO possible causes the instrument itself declines to choose between: the ENGINE's instrument-mode guarantee failing (CLAUDE.md rule 4), or the REPLAY not reproducing the sequence the run played. Revisions 3 and 4 named only the first; D-413's reviewer MEASURED an instrument mutant landing here with the engine healthy. Reported as such, never folded into an attribution count, nothing downstream of it is read, and WP-1.6 does not proceed until it is understood |
| **A TERMINATION THAT IS NONE OF 0, 1, 2 OR 3** | **A VOID ABOUT THE INVOCATION ENVIRONMENT, never about the engines and never about the report.** No verdict was delivered, so none is read; the invocation is re-issued and nothing is concluded from the terminated attempt. **Revision 8 narrows what can still land here.** Revision 7 had to register exit **120** (CPython's shutdown flush, with stdout on a full device) and a `KeyboardInterrupt`'s signal death; `385631f` closes both — every line goes through `say()`, every exit through `leave()`, an undelivered answer is downgraded to exit 2, and the handler catches `BaseException`. What remains is what no process can catch: `SIGKILL`, the OOM killer, a machine that stops. Those are facts about the box and this row is where they are read |
| **The two instruments DISAGREE** (§7A.1's registered agreement criterion) | The run is not a measurement, the verdict is not read, and the disagreement is investigated as an INSTRUMENT defect before anything is concluded about either engine |
| **A robustness FAILURE under the old Criterion 1' clause (b)** | Superseded, and kept so the supersession is visible rather than silent. Criterion 1' is no longer this document's instrument, and warm-replay — the thing D-384's flip clause said had to be built before any verdict was read — is built (D-407 through D-412) and IS Criterion 1'' |

**The turn cap and the book are part of the claim.** A verdict is about
`random_openings_v1` at 50 000 nodes with a 40-turn cap, and quoting it
without them would be a claim nobody measured.

---

## 6. The honest expectation, and what a negative result means

**This is a narrower, cheaper-to-be-wrong-about claim than WP-1.5b's.**
`docs/wp16_quiescence_design.md`'s own non-goal list defers the wider trigger
(c) and the whole of Stage 3's forcing search to later work; this run judges
only whether the narrowest defensible slice — one gated defensive trigger,
one turn of extension — already buys enough tactical accuracy to be worth
D-398's measured 2.80x ttd cost.

**The design's own headline claim is qualitative, not a branching-reduction
number.** Unlike WP-1.5b, there is no MEASURED reduction to quote as a prior
here — quiescence is not a narrowing device, it is a horizon-correctness
device. The one piece of direct, MEASURED evidence this document can offer
came out of this session's own dry run (§8.4): on bench position 1
(`src 00070cdd8fb87f42`), the wider `defensive_and_offensive` seat found a
`-mate 5` at `depth_turns 2` using 716 of its 50 000-node budget, where the
plain staged seat used the full budget to reach `depth_turns 3` and reported
`cp 180` — the seat WITHOUT quiescence did not see the mate at all at this
budget. That is a single anecdote, on the WIDER trigger, off the dry run's
matchup, and is recorded as illustrative context only (§8.4 states this
again) — but it is the shape of gain the design predicts: tactical
accuracy the static horizon eval misses, not depth.

**If the run is not SPRT-positive, that is a PLANNING finding and never a
licence to move a threshold, a budget or a book.** D-395's own finding already
prices in that the mechanism is sound and the width is costly; an `h0` here
would say the narrowest, cheapest width still does not buy enough to clear
`+15` normalised Elo against a 2.80x compute tax — informative about WHERE the
trade-off sits, not a verdict on whether threat-only quiescence as a class is
worth pursuing. Recording that in advance is what stops it being said
afterwards as an excuse. What it would NOT license: re-reading `elo1`,
changing the budget, changing the book, or running until a favourable stop.

---

## 7. Costs

Stated on the document's own face, DECLARED beside MEASURED wherever a
measurement exists.

| Item | DECLARED | MEASURED |
|---|---|---|
| One search at the registered budget, `defensive_only` seat | — | D-398: summed over 22 comparable bench positions at `depth_turns = 3`, node-ratio **2.48x**, ttd-ratio **2.80x** against the plain staged seat |
| The calibration probe (§9.5) | ~2 min | **MEASURED, this session, two independent 24-position sweeps at `go nodes 50000`**: worst single search 291 ms (run 1) / 289 ms (run 2). `hang_timeout_ms = 120000` leaves a **~412x** margin over the larger figure — R-9.5d's own `~24x` convention clears with wide room, discharged NO-CHANGE |
| The dry run (§8) | ~3 min | **MEASURED: 14.254 s of arena wall time** at 4 workers over 8 games, plus the release build (already current), the two external referent searches and the Criterion 1' replay |
| Criterion 1'' on the governed report — the warm-replay pass | ESTIMATED **~1x the governed run's own wall time**, since it re-runs every search every seat actually took | **MEASURED, §8.6, FOUR times: `0.997x`, `1.003x`, `0.994x` and `1.003x`** — straddling 1.0, so the registered figure is "about one run" and no third digit is load-bearing. At `openings_take = 500` that is a second pass of the same order as the run itself. (Revision 5 added §8.6's fourth sample and left this row saying "three times" — D-419 MINOR C) |
| Criterion 1'' — the statistics layer | — | **MEASURED, §8.6: `0.029 s` over 8 games**, plus one cold probe per divergence, expected zero on a clean report |
| The SECOND INSTRUMENT (§7A.1), `tools/wp15b_attribution_check.py` on the same report | — | **MEASURED, §8.6: `6.485 s` over 8 games** — 4 cold searches per pair at the registered budget, minutes at `openings_take = 500` |
| The governed run | ESTIMATED, scaled from D-398's own node-ratio against D-292's radius-policy anchor (5.44 core-hours / ~82 min wall at 2000 openings) — at `openings_take = 500` and roughly 2.5x the per-search cost of a plain-staged-only matchup, **ESTIMATE ~2-3 core-hours, ~35-50 min wall at 4 workers** | the run itself, Step 6 |
| §7A.2's own sweep (the completed-depth comparison) | **DERIVED FROM MEASURED — `≤14 s`, not the `~2 min` revision 5 carried.** §9.5's probe MEASURED 24 positions per sweep at `go nodes 50000` with a worst single search of **291 ms**, so a two-seat comparison is `2 × 24 × ≤0.291 s ≈ ≤14 s` — an upper bound that charges every position the WORST search's time. Revision 5's `~2 min` was anchored to §9.5's DECLARED figure rather than to its measurement: roughly 8x this document's own numbers, and derivable from them in seconds (D-419 MINOR E, and CLAUDE.md's "an estimate that could have been measured in seconds is a finding", D-291) | the sweep itself, **Step 6** — §7A.2's own words are "**TO BE RUN AT STEP 6**, alongside the governed SPRT run". Revision 5's row named Step 6 in its title and Step 7 in this column, contradicting itself and §7A.2 in one row (D-419 MINOR D) |
| Operator/session attention | one launch, one report read, **and the Criterion 1'' chain: one `arena --replay` pass, one `wp16_warm_attribution_check.py` run, one `wp15b_attribution_check.py` run as the second instrument**; every branch is in §5 | — |

---

## 7A. The doubts, their instruments, their agreement criteria and their consequences

### 7A.1 DOUBT 1 — the arena between the engines and the verdict

**THE STAGE UNDER DOUBT**: everything between the two engine processes and the
printed verdict — the arena's seat bookkeeping, its pairing, its referee and
its scoring. Identical stage to WP-1.5b's Doubt 1: this WP changes which
engine sits in which seat, not the arena code that scores them.

**THE RULE THIS SECTION IS WRITTEN UNDER, new in revision 5 and stated before
anything it governs.** Every criterion sentence below carries a RECEIPT from
the shipped instrument at the governing revision — the exact exit code, the
exact message text the instrument prints, or the name of the test that drives
that path. **No paraphrase survives.** The reason is this document's own
history: revision 4 described Criterion 1'' as the DESIGN states it, the fix
round (D-414) then added three premise refusals, a `status` cross-check and a
halt invariant to the INSTRUMENT, and revision 4 was never brought back into
line — so the governing document registered a criterion the instrument no
longer implemented (D-416's BLOCKING 1). That is the FIFTH instance in this
work package of a description and its code drifting apart (D-403, D-404,
D-406, D-413's MAJOR 4, D-416), and paraphrase is the mechanism every one of
them used. A sentence here that cannot be traced to a quoted string, an exit
code or a test name is a defect by construction.

**WHAT REVISION 3 CHANGED, AND WHY.** Revision 2 registered Criterion 1'
(`tools/wp15b_attribution_check.py`, a COLD two-turn replay). D-401's governed
run was taken under it and FAILED it on clause (b): 44 of 141 pairs (31.2%)
were link-1a-vacuous, and adversarially reassigning them moved the verdict
from `h0` to `inconclusive_at_game_cap`. That run is retired as evidence for
anything this WP concludes, under any criterion, ever (D-402), and it is not
re-read here. The ceiling that produced the vacuity is architectural: a fresh
subprocess is COLD, a live engine's transposition table is WARM
(`crates/pistol-engine/src/instance.rs` — `set_position` never touches the
searcher, only `new_game` clears it), so D-383 MEASURED a cold replay of any
turn past an engine's first search disagreeing with what that engine, played
live, actually answered. Two turns was the widest window that architecture
allowed.

**WARM REPLAY REMOVES THE CEILING RATHER THAN RAISING IT** (D-407 through
D-412): `arena --replay` spawns BOTH seats of every game through the same
`seats::with_seats` the generation path calls, feeds the report's own recorded
move list, and asks each seat at every one of its own turns THAT HAS A
RECORDED MOVE, through the same `exchange::ask` the referee calls — so each
engine sees precisely the exchanges it saw live and its table is in precisely
the state it was in. MEASURED on this WP's own dry-run report (§8.6): the cold
instrument reached **16** replayed turns, the warm one compared **201**, all
confirmed.

**THE RECEIPTS FOR THAT SENTENCE, added in revision 6.** It is the most
load-bearing "what the instrument does" claim in this section — the whole
criterion and the second instrument's stage-separation rest on it — and
revision 5 gave it none of the three receipts its own rule demands, while five
much smaller claims beside it carried full ones (D-419 MINOR I). The receipts
exist and are named here:

- **the shared spawn path**, `seats::with_seats` — driven by
  `the_replay_path_sends_newgame_on_every_fresh_spawn_too` and
  `every_fresh_spawn_is_sent_newgame_before_it_is_given_a_position`, the two
  tests D-414 records as pinning the spawn sequence;
- **the shipped chain end to end**, `crates/pistol-arena/tests/replay_chain_tests.rs`;
- **"every turn THAT HAS A RECORDED MOVE"** — the qualifier that makes a
  forfeited game's last, refused ask non-replayable, which is why §7A.1's
  forfeit row and `check_coverage`'s forfeiter branch exist at all.

The same rule applied to the two other unreceipted claims revision 5 left: the
second-instrument bullet's "the cold checker spawns one fresh process per query
and never drives a game at all" is `wp15b_attribution_check.py`'s `cold_answer`,
one `subprocess.run` per query with no game loop anywhere in the file; and the
exit-1 row's "a broken link 1b or 1c" is `link_1b`/`link_1c` appending to
`failures`, which reaches exit 1 through the single `NOT_A_MEASUREMENT` site
named in the invariant above.

**THE INSTRUMENT'S OWN REVIEW HISTORY, stated here rather than left to the ADR
log, because a document that registers an instrument owes its reader that
instrument's record.** The warm-replay implementation FAILED its first
fresh-context REVIEW-impl — 1 BLOCKING, 5 MAJOR, 7 MINOR, all reproduced
(**D-413**, report at `docs/experiments/wp16_replay_IMPL_REVIEW.md`). One fix
round closed all thirteen, and **every fix was verified by the mutation that
survived before it**, re-run in a separate worktree (**D-414**). The operator
then ruled that no second full implementation review would be held, on the
ground that the mutation table is the stronger verification, and moved two
duties onto THIS document's review instead: re-run the mutation table at the
governing revision, and check the amended attribution semantics against
Criterion 1'' word by word (**D-415**). Both were discharged: the reviewer of
revision 4 re-ran all six mutations at the governing revision and **all six
mutants died with D-414's transcript reproduced**, and it was duty (b) that
found D-416's BLOCKING — which is what revision 5 exists to close.

**THE INSTRUMENT, AND ITS GOVERNING REVISION.** Criterion 1'' is taken with a
chain of artefacts, each named here WITH the revision that governs this run,
per CLAUDE.md's instrument rule. A change to any of them reopens this
document's review exactly as an amendment would:

1. **The warm-replay pass** — `crates/pistol-arena/src/seats.rs`,
   `transcript.rs`, `replay.rs`, `replay_report.rs` and `bin/arena.rs`'s
   `--replay` mode, at commit `bfdf933`. **Unmoved by revision 6**:
   `git diff --stat bfdf933..HEAD -- crates/pistol-arena/` prints nothing.
2. **The statistics layer** — `tools/wp16_warm_attribution_check.py`, **at
   commit `385631f`, RE-RECORDED IN REVISION 8.** It was `bfdf933` through
   revision 5, `43e8a86` in revisions 6 and 7 (the MAJOR B fix), and `385631f`
   now: the delivery funnel that closes D-421 MAJOR 1 and the regular-file guard
   that closes D-422. Each re-record IS the amendment that reopens this
   document's review.
3. **The binaries those two actually run** — `target/release/arena`
   `sha256 3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2`
   and `target/release/pistol`
   `sha256 b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673`,
   built `--release --locked` at `bfdf933`. Rebuild means re-record, and
   re-recording is an amendment. **Independently verified**: revision 4's
   reviewer rebuilt in its own tree and got both digests exactly.
   **UNCHANGED THROUGH REVISIONS 6, 7 AND 8, and checkable rather than
   asserted**: `43e8a86` and `385631f` touch only
   `tools/wp16_warm_attribution_check.py` and
   `crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs` — a `tools/`
   script and a `tests/` target, neither of which a release binary is built
   from — so `git diff --stat bfdf933..HEAD -- crates/*/src/ Cargo.toml Cargo.lock`
   prints nothing and `sha256sum target/release/pistol target/release/arena`
   still prints the two digests above.

**THE SENTENCE THAT USED TO STAND HERE IS RETIRED.** Revisions 3, 4 and 5 each
said "Commits after `bfdf933` on this branch are DOCUMENT-ONLY … checkable in
one command, `git diff --stat bfdf933..HEAD -- crates/ tools/`, which must print
nothing." **That is FALSE at revision 6 and is removed rather than left to be
discovered**: `43e8a86` is not a document commit. What replaces it is the
narrower pair of checks in items 1 and 3 above, which are the claims revision 6
actually needs and can actually make. A reviewer should run them; the broad
command will now print, and printing is correct.

`tools/wp15b_attribution_check.py` is NOT modified by
this WP; it appears below as the SECOND INSTRUMENT, not as this criterion's,
and it carries its own revision there.

**CRITERION 1'', quoted from `docs/experiments/wp16_warm_replay_design.md` §4
point 4 IN FULL AND WITHOUT ELLIPSIS.** The quotation is verbatim against that
document at its revision 3 (`b6afd66`, the current text); revision 2's wording
of the same point differs only in writing "§4 point 3's" where revision 3
writes "point 3's", and the meaning is identical.

> A report is a measurement iff (a) zero divergence-confirmed inversions —
> every divergence found in point 2 above resolves to either "no divergence" or
> "confirmed inversion" (the other-engine match case), never left unclassified
> — and (b) every NON-INERT pair (point 3's exclusion, forfeits always
> non-inert) is directly attributed by first divergence. A DETERMINISM
> VIOLATION (point 2's other branch) is checked FIRST and, if found anywhere,
> stops the whole evaluation before (a)/(b) are even asked, per its own exit
> code. The old clause (b)'s adversarial-reassignment machinery is KEPT, but
> only as a cross-check run over the INERT pairs alone (expected to be a no-op,
> since point 3's theorem already fixes their bucket) — its result is cited in
> the report as confirming evidence, not as the thing the verdict depends on.

**The final sentence was truncated without an ellipsis in revisions 3 and 4.**
It is restored here, and its own consequence is registered below rather than
left implicit: the cross-check it describes is a FAILURE path in the shipped
instrument, not merely a citation.

---

#### What the instrument refuses BEFORE it applies the criterion at all

**FOUR** premise refusals, each an exit-2 REFUSAL of the report, in
`tools/wp16_warm_attribution_check.py`'s `clause_b`. They exist because both
arms of clause (b) begin "the two games agree up to t", and the arena
guarantees that by construction while this instrument must not assume it — the
reports it exists to judge are exactly the ones that might not be what they say
they are.

| The premise | The instrument's own words, verbatim |
|---|---|
| the pair's two games declare the SAME `opening` | `its two games declare openings {X} and {Y}, so they are not one opening played from both seats and no pair-level argument applies to them` |
| the two games SWAP the seats | ``its two games seat `{p1}`/`{p2}` and `{p1}`/`{p2}`, which is not one seating and its reverse — the pair's whole argument is that swapping the labels is the only difference between them`` |
| the two games share their first `opening_turns` moves, DIFFERING AT A TURN | ``its two games differ at turn {t}, which is inside the {n}-turn book, so they do not share the opening prefix every argument below assumes. Both arms of clause (b) begin "the two games agree up to t"; on this pair that is false before any engine was ever asked anything`` |
| **NEW IN REVISION 6** — the two games share their first `opening_turns` moves, DIFFERING ONLY IN LENGTH because one game's whole move list stops inside the book | ``game {j} records {k} turn(s), fewer than the {n}-turn book, and game {i} records {m} — their book prefixes differ in LENGTH and not in content, so there is no turn at which they disagree and no book they share. Both arms of clause (b) begin "the two games agree up to t"; on this pair there is no such t, because the shorter game stops inside the book`` |

Each prints under `warm_attribution_check: CANNOT READ:` and exits **2**.
Driving tests: `a_pair_that_does_not_satisfy_the_proofs_premise_is_a_void_and_not_an_attribution`
(`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs`), which seeds
the first three and requires exit 2 on each; and, for the fourth,
`a_pair_mate_shorter_than_the_book_is_a_refusal_and_not_a_crash`, which
requires exit 2, the `CANNOT READ:` prefix, the words `fewer than the 2-turn
book`, and NO `StopIteration` on stderr.

**MAJOR B, CLOSED IN CODE AND NOT BY NARROWING THIS RECEIPT.** Until `43e8a86`
the sentence "Each prints under `CANNOT READ:` and exits **2**" was FALSE, and
D-419's reviewer proved it false by TESTING it rather than reading it. The
third arm asked for the first index at which the two book prefixes differ; two
lists can be unequal because one is SHORTER, in which case no index differs at
all, the generator was empty, and `next()` raised `StopIteration`. That class
was not in the handler's caught tuple, so it escaped as a traceback: **no
`CANNOT READ:` line at all, and exit 1** — the code this document registers as
THE RUN IS NOT A MEASUREMENT, a finding about the ENGINES. A refusal wearing a
finding's exit code, inside the very `try`/`except` whose own comment said it
existed to prevent that.

The reviewer offered two closes: narrow this receipt to what the instrument
actually did, or obtain a licence to fix the instrument. **The architect
licensed the fix**, so the receipt above is now true rather than weakened, and
the refusal it describes has become the fourth row of the table rather than an
exception to a three-row one. The instrument's governing revision moves
accordingly (item 2 above), which is what makes this a revision and not an
erratum.

**THE REGISTERED READING OF THESE FOUR, corrected in revision 5.** Revision 4
registered exit 2 as "the void is fixed and the answer re-taken". **That is the
wrong reading for a premise refusal, and it is now stated correctly**: a pair
whose two games declare different openings, or do not swap seats, or diverge
inside the book, or stop inside it, is a fact about the REPORT. There is no
void to fix and nothing to re-take. **The report is REFUSED and it is not a
measurement**, and the thing to investigate is how a report the arena cannot
write came to exist — never the engines. A reader who followed revision 4's
text would have gone looking for a broken checker.

---

#### What the instrument checks about the REPLAY DOCUMENT itself

Two more exit-2 refusals, in `check_coverage`, neither registered before
revision 5.

- **The `status` word is not taken on trust.** It must be `clean` or
  `divergence` by name — ``game {i}: `status {s}` is neither `clean` nor
  `divergence` `` — and it must agree with the divergence records the document
  actually carries — ``game {i}: the replay record says `status {s}` and the
  document carries {0|1} `divergence` record for it``. Without this, a record
  merely CLAIMING to have diverged skips every coverage check and the node
  comparison, and is still summarised as replayed in full.
- **A divergent record's HALT INVARIANT is read off the document rather than
  assumed**: `replayed_turns == at_turn - 1` — ``it diverged at turn {t}, so
  {t-1} turn(s) were fed, and the record says `replayed_turns {n}` — the replay
  did not halt where it says it halted`` — and
  `compared_turns == at_turn - opening_turns`. A clean record's own
  `compared_turns` and `replayed_turns` are derived and checked the same way.

Driving test: `a_replay_record_cannot_skip_its_own_coverage_checks_by_claiming_a_divergence`,
which seeds four such documents and requires exit 2 on each, against a control
that must pass.

---

#### Clause (b)'s satisfaction condition — EXHAUSTIVE over four cases

Revision 4 gave three bullets and they did not cover the report space: a pair
with IDENTICAL move lists one of whose games forfeited matched none of them
(D-416). The four cases below are exhaustive by construction — a pair's two
move lists are either equal or not, and if equal either a game forfeited or
none did — and each names the branch that handles it, its exit and its test.

| The pair | What the instrument does | Exit | Driving test |
|---|---|---|---|
| move lists IDENTICAL, neither game forfeited | **INERT**, excluded by the theorem. The theorem's content is that IDENTICAL move lists force a 1-1 split whatever the replay saw: swapping the labels could not have changed a board at any ply, so whichever PLAYER INDEX wins one game wins the other, and the two games swap which LABEL holds that index. The bucket is ASSERTED, not assumed — a pair recorded at anything but `p2` prints ``(b) pair {i} … has two identical move lists, neither forfeited and both replayed clean — the inert-pair theorem forces a 1-1 split — and the report records bucket p{k}`` | 0 excluded / **1** if the bucket contradicts the theorem | `an_inert_pair_is_excluded_by_the_theorem_and_its_cross_check_is_a_no_op`; `an_inert_pair_the_report_did_not_score_one_all_is_a_finding` |
| move lists IDENTICAL, one game FORFEITED | **NOT inert** — forfeits are always non-inert, because a forfeit's outcome-deciding event has no recorded MOVE to warm-replay against, so "zero divergence across every recorded move" is vacuously true at exactly the ply that decided the result. No witness turn exists either, so the pair is UNATTRIBUTABLE and clause (b) FAILS on it: ``(b) pair {i} (opening {o}) is not inert (a forfeit ended one of its games) and its two games never differ at a turn either engine searched — one move list is a prefix of the other — so no replayed turn tells the two seats apart in it`` | **1** | `a_forfeit_sibling_of_an_inert_pair_is_not_excluded` |
| move lists DIFFER, and a witness turn `t` exists in both | **DIRECTLY ATTRIBUTED at `t`.** Both games agree up to `t` (the book refusal above is what makes that true), so the board at `t` is identical and the mover is the same PLAYER INDEX in both — and that index's occupant searched exactly the same prefixes in both games, so its warm table is in the same state in both. The replay measured the seat credited in game one answering `m1` there and the seat credited in game two — the OTHER label — answering `m2 != m1` there. Inverted labels would require one engine to answer both to the same position with the same history, which instrument-mode determinism forbids | 0 | `a_clean_replay_of_an_honest_report_is_attributable`; `a_forfeit_containing_pair_that_differs_at_a_searched_turn_is_attributed` |
| move lists DIFFER, no witness turn — one is a strict PREFIX of the other | **UNATTRIBUTABLE**, same message as the forfeit row with `its move lists differ` in place of the forfeit clause. Neither excluded nor attributed, and named rather than passed over | **1** | covered by the same case as the forfeit row |

**The proof in row 3 was attacked and held.** Revision 4's reviewer tried six
routes against it — differing game lengths, a witness inside the book, an
asymmetric opening book, capped games, forfeits, and a halted replay — and
broke none; the last is closed by control flow, since any divergence makes the
classifier exit 1 or 3 before clause (b) is reached at all.

**ROW 1'S GROUND IS THE THEOREM, NOT THE REPLAY — corrected in revision 6.**
Revision 5 grounded the exclusion on "both credited engines warm-replayed every
move, so the two seats are indistinguishable at every position either game
reached". For a pair whose two games are the book and nothing more,
`compared_turns` is 0 — the replay compared nothing — yet `clause_b` excludes
the pair on the strength of `one == two` alone, and D-419's reviewer reproduced
exactly that (MINOR H). **The conclusion survives untouched**, because the
theorem never needed the replay: identical move lists force a 1-1 split as a
matter of the move lists, and row 1 now says only that. The over-claiming
clause is gone.

---

#### The inert cross-check — restored, with its consequence registered

The sentence revisions 3 and 4 truncated describes a path the shipped
instrument RUNS, in `cross_check`. Registered here properly:

- **On a report with no inert pairs** it is a no-op and says so:
  `cross-check: no inert pairs — the exclusion changed nothing`.
- **On a report with forfeits** the run is already `invalid_forfeit` by the
  arena's own scoring rule and a pentanomial-only recomputation has no notion
  of a forfeit, so it is `skipped, not silently passed` — the instrument's own
  words, printed rather than left silent.
- **Otherwise it SELF-CHECKS FIRST**: its ported `sprt.rs`/`score.rs`
  arithmetic must reproduce the report's own printed verdict off the unmodified
  pentanomial, or it refuses at exit **2** (``the two machineries disagree on
  the honest input, so nothing computed from a hypothetical one can be
  trusted``). Only then does it flip the inert pairs' buckets.
- **A flip that MOVES the verdict is a FAILURE, exit 1**, not merely a citation:
  ``cross-check: reassigning the {n} pair(s) the inert theorem excludes moves
  the verdict from `{v}` to `{w}` — the theorem says this is impossible, so
  either the exclusion or the arithmetic is wrong``. The design calls the
  result "confirming evidence"; the instrument treats a NON-confirming result
  as a finding, and this document registers that stricter reading as the
  operative one.

Driving test: `an_inert_pair_is_excluded_by_the_theorem_and_its_cross_check_is_a_no_op`,
which requires the words `leaves the verdict `inconclusive_degenerate`
unchanged` in the output — so the cross-check is RUN, not merely argued.

---

**THE REGISTERED CONSEQUENCE OF EACH EXIT CODE LIVES IN §5 AND ONLY IN §5.**
This section states what the INSTRUMENT does — which branch, which message,
which test — and points at §5 for what may be CONCLUDED. It does not restate
§5's table, and that is a rule rather than a stylistic preference: revision 7
corrected the exit taxonomy in §5 and left this section's copy byte-identical,
so the document shipped self-contradicting and its reviewer found it in minutes
(D-423 MAJOR 1). A claim a document makes twice gets fixed once. The codes are
the instrument's own constants: `ATTRIBUTABLE = 0`, `NOT_A_MEASUREMENT = 1`,
`NO_ANSWER = 2`, `DETERMINISM_VIOLATION = 3`.

| Exit | What reaches it, in the instrument | Consequence |
|---|---|---|
| 0 | `main()` returns with an empty `failures` list, and every note was delivered | §5 |
| 1 | **the NAMED attribution findings, and nothing else** — a confirmed inversion; an unattributable pair; an inert pair whose bucket contradicts the theorem; a cross-check that moves the verdict; a broken link 1b or 1c. Every one is printed under `FAIL ` before the exit | §5 |
| 2 | every refusal, from `die()`. There are ~50 of them and this document does not enumerate or partition them — see §5's single exit-2 row and **D-424** | §5 |
| 3 | a divergence neither engine's answer explains; a clean game whose replay spent different nodes; a forfeiting seat that spent more replaying than the whole game cost it live | §5 |

**THE INVARIANT, and it is now TRUE AS WRITTEN rather than narrowed to fit:**

> **EXIT 1 ARISES ONLY FROM THE NAMED ATTRIBUTION FINDINGS.** Every other
> termination of the instrument is exit 0, exit 2, or exit 3.

Revision 6 registered this sentence and offered four `grep`s as its checks.
Revision 6's reviewer redirected the instrument's stdout to `/dev/full` and
MEASURED the sentence FALSE two ways — exit **120** under default buffering, and
exit **1 with a traceback and no `CANNOT READ:` line** under
`PYTHONUNBUFFERED=1`, the exact signature the catch-all handler exists to
abolish, surviving inside it (D-421 MAJOR 1). **None of the four greps could
have detected either**, because every one of them read the file's TEXT while the
defect was in its RUNTIME behaviour; four green checks and a false invariant were
perfectly consistent. Revision 7 answered by narrowing the sentence and adding a
fifth grep, and its reviewer showed that grep was green at the revision where the
defect was live, so it was no more a falsifier than the other four (D-423
MAJOR 2).

**SO THE CHECK IS NO LONGER A GREP.** `385631f` fixes the instrument — every
line now goes through `say()`, every exit through `leave()`, and any UNDELIVERED
answer is downgraded to `NO_ANSWER` whatever it would have been — and the
registered check is the test that drives it:

> `an_answer_that_cannot_be_delivered_is_no_answer_and_not_a_finding`
> (`crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs`), which runs
> the SHIPPED script with stdout on `/dev/full`, buffered and unbuffered, and
> requires exit **2** with no `Traceback` on stderr, against a control on the
> same fixture that must exit 0.

**THIS CHECK IS ONE THE DEFECT FALSIFIES, AND THAT IS SHOWN BY MUTATION RATHER
THAN CLAIMED**: removing `say()`'s guard (M6), making `leave()` ignore the
delivery flag (M7), or dropping its forced flush (M8) each makes that test FAIL.
A check that cannot fail is not a check, which is what the five greps were.

**THE HANDLER CATCHES `BaseException`, NOT `Exception`.** The narrower form left
`KeyboardInterrupt` outside the invariant and the comment that stood there
ARGUED it away rather than closing it; an argument is not an invariant.
`SystemExit` is re-raised first and untouched, because `die()`, `violation()` and
`leave()` all travel on it and each has already chosen its code.

**WHY A CATCH-ALL AND NOT A LONGER TUPLE.** The handler once named
`(KeyError, ValueError, IndexError)`. Two classes escaped it, both MEASURED
against this file: `StopIteration`, out of `clause_b`'s book-prefix arm
(D-419 MAJOR B), and `ZeroDivisionError`, out of the ported `sprt.rs`
arithmetic in `recompute_verdict` on a report declaring `alpha 1.0`. Each
exited 1 with a traceback. Adding a fourth class would have left a fifth. **The
defect was never that the tuple was one class short; it was that an invariant
was being asserted by an ENUMERATION** — the same shape as the exit-2 partition
§5 has now deleted, and as the "closed list of four" before it. Three instances,
one class, and D-424 is the rule that ends it.

| Exit | What reaches it | Consequence, registered here |
|---|---|---|
| 0 | Criterion 1'' holds | §5's table is read, and only then |
| 1 | **the NAMED attribution findings, and nothing else** — a confirmed inversion; an unattributable pair; an inert pair whose bucket contradicts the theorem; a cross-check that moves the verdict; a broken link 1b or 1c. Every one is printed under `FAIL ` before the exit | **THE RUN IS NOT A MEASUREMENT.** The verdict is not read, it is not an `h0` either, and the committed instrument config does not move in either direction |
| 2 | **two kinds, partitioned by RULE rather than by list.** **(i) a VOID — the CLOSED list**: a missing or unrunnable engine, an unreadable or non-UTF-8 document, an incomplete or abandoned replay pass, a budget this cannot replay. Not a finding, not evidence about any engine; the void is fixed and the answer re-taken. **(ii) a REFUSED REPORT — the REGISTERED CATCH-ALL**: every other exit 2, named or not, including the four premise refusals, the `status`/halt refusals, the cross-check's self-check, every report-internal contradiction, and any refusal nobody has enumerated — such as the handler's own `an unanticipated {Class} escaped this instrument`. **Nothing here is "fixed" and nothing is "re-taken"** — the report is not one the arena could have written, the run is not a measurement, and what is investigated is the report's provenance, never the engines | as stated per kind. **The reader does not need to match a message against a list**: kind (i) is closed above, so an exit 2 that is not one of those four IS kind (ii) by rule. That is the whole point of the change — revision 5 asked the reader to distinguish the kinds by text and did not give them enough text to do it |
| 3 | a divergence neither engine's answer explains; a clean game whose replay spent different nodes; a forfeiting seat that spent more replaying than the whole game cost it live | **A HARD STOP BIGGER THAN THIS WP.** Reported as such, never folded into an attribution count, and nothing downstream of it is read. WP-1.6 does not proceed until it is understood |

**A NOTE ON DIRECTION, so the exit-2 catch-all is not over-read as laxity.**
Every refusal it absorbs leaves the instrument STRICTER than this document
promises, never looser. No exit 0 is reachable through any of it, so no false
PASS is available — the catch-all can only turn a crash into a refusal, and a
refusal is already a non-measurement.

**EXIT 3 NAMES BOTH CAUSES, corrected in revision 5.** Revisions 3 and 4
registered it as "the engine's own instrument-mode guarantee is failing". The
instrument itself declines to say that, and it is right to: its own printed
second line reads ``this has TWO possible causes and this instrument cannot
tell them apart: the ENGINE's own instrument-mode guarantee failing (CLAUDE.md
rule 4), or the REPLAY not reproducing the sequence the run actually played``.
That is not hedging — D-413's reviewer MEASURED a cold-replay INSTRUMENT mutant
landing on exit 3 with the engine entirely healthy. A document that blamed the
subject where the instrument declines to would send a reader hunting a defect
in the engine, which is `tools/SHELL_CHECKLIST.md` item 12's whole subject.
Driving tests: `the_dual_engine_probe_tells_a_determinism_violation_from_an_inversion`,
`a_clean_game_that_spent_different_nodes_replaying_is_a_determinism_violation`,
`a_forfeiting_seat_that_spent_more_replaying_than_it_did_live_is_a_determinism_violation`.

**THE SECOND INSTRUMENT, AND ITS AGREEMENT CRITERION — registered before either
runs.** `tools/wp15b_attribution_check.py`, unmodified, at commit `bfdf933`
(unchanged by this WP; `git diff 8ca4063..bfdf933 -- tools/wp15b_attribution_check.py`
prints nothing), run on the SAME governed report.

- **THE STAGE IT DOES NOT SHARE**, named as CLAUDE.md requires: the WARM DRIVE.
  Criterion 1'''s whole load rests on a persistent per-game pair of processes
  reproducing the live game; the cold checker spawns one fresh process per
  query and never drives a game at all, so a defect in the warm drive cannot
  reach it.
- **WHAT THEY ARE BOTH BLIND TO**, named so their agreement is not over-read:
  the report WRITER. Both read the document `report.rs` produced, and neither
  can see a defect that corrupted it identically for both. Their agreement is
  evidence about the drive, not about the writer.
- **THE AGREEMENT CRITERION**: for every game the cold checker attributes by a
  discriminating replayed turn, the warm pass must record that game `status
  clean` — and for every game it calls a confirmed inversion, the warm pass
  must record a `divergence`. Their link 1b and link 1c outputs are NOT part of
  this criterion: those two links are the same computation in both files and
  agreeing there is agreeing with themselves.
- **THE REGISTERED CONSEQUENCE OF DISAGREEMENT**: the run is not a
  measurement, the verdict is not read, and the disagreement is investigated as
  an INSTRUMENT defect before anything is concluded about either engine. No
  margin is derived after the fact to decide which instrument to believe.
- **A false-disagreement mode was attacked and rejected** by revision 4's
  reviewer: one might fear the cold checker disagreeing with the warm pass by
  D-383's own cold/warm mechanism. It cannot, because
  `wp15b_attribution_check.py`'s two checked turns have DIFFERENT movers and
  are therefore each engine's FIRST search of the game, which is exactly where
  cold and warm coincide.

### 7A.2 DOUBT 2 — whether the extension changes what the search completes

This is WP-1.6's analogue of WP-1.5b's D-245 doubt, rebuilt for a mechanism
that predicts the OPPOSITE direction: WP-1.5b's staged generator narrows the
tree and should reach a DEEPER completed iteration at fixed nodes; this WP's
quiescence extension spends nodes AT the horizon rather than narrowing the
tree above it, so the registered prediction is **the same or a SHALLOWER**
completed `depth_turns` at the fixed 50 000-node budget, not a deeper one.

**THE INSTRUMENT: `tools/baseline_snapshot.sh --config`, now available at the
closed revision D-387 records** (`9282dd0`) — WP-1.5b's own Doubt 2 instrument
could not measure the staged seat at all at its authoring revision; this WP
inherits a working `--config` flag and needs no further instrument work.

**HOW IT DOES NOT SHARE THE STAGE**: identical to WP-1.5b's own reasoning —
the snapshot drives ONE engine through the line protocol and reads its own
output. No seat, no pairing, no referee, no scoring.

**THE AGREEMENT CRITERION, registered before either instrument runs, two-sided
and tie-aware — over the 24 bench positions, at `go nodes 50000` (the arena's
own budget), count the `defensive_only` seat's completed `depth_turns` against
the plain staged seat's in three buckets: strictly greater, equal, strictly
less.**

**THE PARTITION IS ON THE BUCKET COUNT ALONE, independent of the SPRT
verdict** — CLAUDE.md's own registered-consequence rule forbids a criterion
that leaves an after-the-numbers choice standing, and a verdict-conditioned
branch here would do exactly that (a first draft of this section made this
mistake and a fresh-context review caught it — recorded rather than silently
fixed, since an amendment reopens the review it corrects):

- **AGREEMENT** is the `defensive_only` seat STRICTLY GREATER on AT MOST 6 of
  the 24 positions — mostly same-or-shallower, consistent with the
  registered prediction that the extension spends nodes at the horizon rather
  than narrowing the tree above it.
- **DISAGREEMENT** is the `defensive_only` seat STRICTLY GREATER on MORE THAN
  6 of the 24 positions — the search is seeing DEEPER rather than more
  accurately, contrary to the mechanism the design claims.

6/24 = 25% is chosen as the complement of the ~75% "strictly-less-or-equal"
threshold a first draft of this criterion used and is JUDGED, not measured,
stated so a reviewer can attack it: D-398's own measured 2.48x node-ratio at
fixed depth predicts the inverse relationship holds at fixed nodes on MOST
positions, and 24 × 0.25 = 6 leaves margin below the ~92% (22/24) comparable
fraction D-398 itself measured for positions where the extra ply-1/ply-2
search still completes within budget.

**THE REGISTERED CONSEQUENCE**: on DISAGREEMENT, the work package does not
land on the SPRT alone, REGARDLESS OF THE VERDICT — the committed config does
not move even on an `h1` verdict, the disagreement is reported with both
numbers and all three buckets, and the next step is investigation, not a
re-run and not a re-reading of either threshold. **An `h1` verdict that also
falls in the DISAGREEMENT bucket is not read as confirming quiescence's
tactical-accuracy claim** — a real strength gain riding on a mechanism that is
making the search see deeper rather than more accurately is not the claim
§1 registers, and needs the same investigation a disagreeing `h0` would. This
is where the "tactical correctness at whatever depth is reached, not depth
itself" reasoning belongs — in how a DISAGREEMENT is investigated, not in
whether the criterion fires — so the criterion itself stays a clean partition
on one observable and a reader cannot pick a reading after seeing the
numbers.

**WHAT THIS INSTRUMENT STILL CANNOT SEE**: it is blind to the arena's score
path, as the SPRT is — precisely why Doubt 1 has its own instrument, and
precisely why running only one of the two would be one instrument reported
twice (CLAUDE.md's own "two instruments blind to the same stage" rule).

**TO BE RUN AT STEP 6**, alongside the governed SPRT run, not as part of this
document's own dry run (§8's dry run answers ATTRIBUTION syntax and the
Criterion 1'/2/3 chain over a DIFFERENT matchup by construction — see §8.1).

---

## 8. The dry run

CLAUDE.md: "A pre-registration's literal commands are exercised before its
review passes, on an input of the SAME KIND as the registered workload — the
same sort of artefact, differing only in identity — and never on the
registered workload itself."

**IT HAS NOW BEEN RUN.**

### 8.1 The input

`configs/arena_wp16_dryrun.toml` — an arena config, the same kind of artefact
as the registered one, differing in identity on every axis that matters:

- **engines**: `configs/instrument_staged_q_defensive_and_offensive_v0.toml`
  (label `staged_q_both`) against `configs/instrument_staged_v0.toml` (label
  `staged`) — the WIDER trigger arm, never the registered `defensive_only`
  candidate;
- **book**: `openings_v1.txt`, not the primary `random_openings_v1.txt`;
- **size**: `openings_take = 4`, eight games.

Deliberately the SAME as the governed run: the budget, the turn cap, the
worker count and the hang timeout, so the commands are exercised against the
shapes they will meet.

It is a real instance of the kind, not a synthetic stand-in: two genuinely
different engines (D-397's own unit tests already establish
`defensive_and_offensive` and `defensive_only` diverge behaviourally), played
through the real arena, over a real sha-pinned book.

### 8.2 The literal commands

`<scratch>` is any directory outside the repository (rule 8 — match logs are
artifacts). The `--out` path must not exist.

```
cargo build --release --locked --bin arena --bin pistol
sha256sum target/release/pistol
tools/config_check.sh configs/arena_wp16_dryrun.toml
target/release/arena --config configs/arena_wp16_dryrun.toml --out <scratch>/wp16_dryrun_report.txt
sha256sum configs/instrument_staged_q_defensive_and_offensive_v0.toml configs/instrument_staged_v0.toml
printf 'position start moves <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/instrument_staged_q_defensive_and_offensive_v0.toml
printf 'position start moves <P>\ngo nodes 50000\nquit\n' | target/release/pistol --config configs/instrument_staged_v0.toml
python3 tools/wp15b_attribution_check.py <scratch>/wp16_dryrun_report.txt target/release/pistol
```

`<P>` is BENCH POSITION 1 of `crates/pistol-cli/tests/fixtures/bench_positions_v1.txt`
(`src 00070cdd8fb87f42`, 15 stones) — the same position D-376/D-395/D-398 all
cite:

```
0,0 -1,1/1,0 0,1/0,2 -1,0/1,-1 0,-1/1,-2 0,-2/0,3 -1,-1/1,1 -1,2/-1,3
```

**`tools/wp15b_attribution_check.py` IS AN INSTRUMENT, NAMED HERE WITH ITS
REVISION** — the commit this document lands at. A change to it reopens this
document's review exactly as an amendment to the document does.

### 8.3 What the output must show, and the defect class each criterion excludes

#### Criterion 1' — DEFECT CLASS: SEAT/LABEL ATTRIBUTION INVERSION

Reused verbatim from WP-1.5b (§7A.1 above). **RESULT: PASS**, `0 failure(s)`,
exit 0 — see §8.4.

#### Criterion 2 — DEFECT CLASS: THE ENGINE LOADED A DIFFERENT DOCUMENT THAN THE REPORT NAMES

**WP-1.5b's own version of this criterion does not transfer to this WP, and
running the dry run is what found that out — exactly what the dry-run rule
exists for.** WP-1.5b's Criterion 2 reads the engine's own handshake line,
`engine_id <slot> candidate_policy …`, and requires it to differ between the
seats in the way the two documents differ. **MEASURED, this session**:
`crates/pistol-cli/src/bin/pistol.rs`'s `identity_lines` function (the
handshake's source) emits `candidate_policy staged quiet_radius <n>
quiet_top_k <k>` under `CandidatePolicy::Staged` — and, by the function's own
doc comment, "`tier_t_own_count`, `tier_t_opponent_count` and `widen_schedule`
do not ride on this line — U2-M item 2 names only `quiet_radius` and
`quiet_top_k`," a limitation that predates this WP and that WP-1.6's own
`q_depth_turns`/`q_triggers` fields were never added to either. **The dry
run's own report confirms this concretely**: both engines' `engine_id …
candidate_policy` lines read byte-identical —
`candidate_policy staged quiet_radius 2 quiet_top_k 16` — despite the two
configs genuinely differing in `q_depth_turns` (`1` vs `0`). **As WP-1.5b's
own Criterion 2 is written, it is VACUOUS for every matchup this WP can
register**: it would pass identically whether the engine actually loaded its
own named config or its sibling's, which is exactly the "a property the named
defect class PRESERVES" failure CLAUDE.md's criterion rule names.

**THIS CRITERION IS THEREFORE DEMOTED, not silently ported** — the same
treatment WP-1.5b gave its own Criterion 3 when an instrument could not
support a pass/fail reading. **What stands in for it**: Criterion 1's Link 1a
(§7A.1) already replays each engine's first free search as a genuinely
separate process, keyed to the report's own named config path, and D-397's
own unit tests establish that `defensive_only`, `defensive_and_offensive` and
plain staged are behaviourally distinct engines on real positions — so a
document swap between any two of this WP's seats would, on most openings,
also fail Link 1a's non-vacuity or its move-match check. **This is weaker than
a direct check and the gap is named rather than hidden**: a swap that happened
to produce the identical first move on a given opening's two free turns would
pass Link 1a undetected, where a genuine Criterion 2 would still catch it via
the handshake. **WHAT WOULD CLOSE THIS GAP, recorded rather than acted on**:
extending `identity_lines` to also emit `q_depth_turns`/`q_triggers` under
`Staged` — a small change to a PINNED handshake (`pistol.rs`'s own comment:
"its handshake is pinned byte-for-byte against the pre-WP-1.4 revision"),
which is a protocol amendment and not something this Step-5 authoring session
takes unilaterally. Licensed-not-scheduled, alongside `defensive_and_offensive`
itself (D-396).

#### Criterion 3 — THE EXTERNAL DEPTH REFERENT, demoted to what it can support

Context only, never a pass/fail gate on attribution — WP-1.5b's own framing,
kept. **MEASURED, this session, at bench position 1**: the `staged_q_both`
seat (`defensive_and_offensive`) returned `depth_turns 2`, `nodes 716`, `score
-mate 5` — an early stop on a FOUND MATE, using under 1.5% of the 50 000-node
budget. The plain `staged` seat returned `depth_turns 3`, `nodes 50176`
(the full budget), `score cp 180`. **This is the SAME degenerate,
mate-early-stop category D-395 and D-398 both name and exclude from any
depth-bucket tally** — a candidate finding a genuine forced win the baseline's
static horizon eval cannot see, terminating early with FEWER total nodes, not
a tree-narrowing effect. It is recorded here as illustrative context for §6's
honest expectation, not as a Criterion-3-style bucket count: with one position
in the sample and that position degenerate by the same rule §7A.2 already
names, no aggregate bucket comparison is meaningful from the dry run alone.
§7A.2's own bucket criterion is registered for the GOVERNED run's 24-position
sweep at Step 6, not for this single dry-run anecdote.

### 8.4 What the dry run recorded

Full report at `<scratch>/wp16_dryrun_report.txt`
(sha256 `d617ae7d5ad1cc9827a871c62df5ce54f4818dff7402d494765f3b93d2e38ae9`, not
committed — rule 8), produced by
`target/release/arena --config configs/arena_wp16_dryrun.toml`:

```
arena_report 4
experiment_sha256 62e2608c4d0fb9c128d98d00d24e362b9162c7cc2e5d2da7f99896875aab237f
openings_file crates/pistol-cli/tests/fixtures/openings_v1.txt
openings_take 4 of 1591
budget nodes 50000
turn_cap 40
sprt elo0 0.000000000 elo1 15.000000000 alpha 0.050000000 beta 0.050000000
engine a label staged_q_both config configs/instrument_staged_q_defensive_and_offensive_v0.toml config_sha256 bf2bb20cfc703bd65d6f96713c64707fdc1bafc840ca7f4ba97ddc567ea81785
engine b label staged config configs/instrument_staged_v0.toml config_sha256 986957cdcc72f67e44fcbc0a8c0b3ac3783fc954e89038fe1d794f09d806b825
binary_sha256 (both seats) b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673
counts n 8 distinct_n 8 wins_a 2 capped 3 losses_a 3 forfeits 0 decided 5
pentanomial p0 0 p1 1 p2 3 p3 0 p4 0
capped_fraction 0.375000000
verdict inconclusive_at_game_cap
```

**`verdict inconclusive_at_game_cap` at 4 pairs is EXPECTED and is not a
finding** — the dry run cannot and is not meant to cross an SPRT boundary at
these bounds; what it exercises is the commands and the criteria, per §8's own
opening quote.

**Criterion 1' (§8.3), run against this report**:

```
attribution_check: 1a: 16 turns replayed, 10 of them discriminating, 8 of 8 games directly attributed by replay
attribution_check: 1a robustness: no vacuous pairs — clause (b) holds trivially
attribution_check: 1b: 5 decided non-forfeit games adjudicated against the move list
attribution_check: 1c: 8 games and 4 pairs rebuilt off the score_a path
attribution_check: PASS — 0 failure(s)
```

exit 0. `tools/wp15b_attribution_check.py`, unmodified, correctly attributes
this WP's engines — it reads labels out of the report, not out of anything
staged-vs-radius-specific, so this confirms the instrument transfers before
Step 6 relies on it.

### 8.5 What the dry run is and is not

It is a real instance of the kind, and it found a real, load-bearing gap in a
reused criterion (§8.3's Criterion 2) that a synthetic stand-in would not have
surfaced — the whole justification for CLAUDE.md's "only a real instance of
the kind exercises ATTRIBUTION" rule. It is not a measurement of either
engine's strength, and nothing in §8.4 may be quoted as one.

**§8.1 through §8.5 are revision 2's, unchanged, and they govern the
instruments revision 2 registered.** Revision 3 changed the instrument, so the
dry-run rule binds afresh — §8.6 is that second dry run, and it does not
replace or amend anything above it.

### 8.6 The warm-replay dry run — revision 3's own, because revision 3 changed the instrument

Criterion 1'' is taken with artefacts that did not exist when §8.1-§8.5 were
written, so CLAUDE.md's dry-run rule binds them afresh: their literal commands
are exercised before this revision's review passes, on an input of the SAME
KIND as the registered workload and never on the registered workload itself.

**IT HAS NOW BEEN RUN.**

**The input.** `configs/arena_wp16_dryrun.toml` — the same document §8.1
already argues is a real instance of the kind, differing in identity on every
axis that matters (the WIDER trigger arm as engine A, the `openings_v1.txt`
book, four openings). Not D-401's report, which is never read again, and not
the registered workload.

**The literal commands.** `<scratch>` is any directory outside the repository.

    cargo build --release --locked --bin arena --bin pistol
    target/release/arena --config configs/arena_wp16_dryrun.toml --out <scratch>/run.txt
    target/release/arena --replay <scratch>/run.txt --out <scratch>/replay.txt --workers 4
    python3 tools/wp16_warm_attribution_check.py <scratch>/run.txt <scratch>/replay.txt target/release/pistol

and, for the seeded-defect arm, the same report with every `game` record's
`p1`/`p2` labels transposed and NOTHING else changed:

    target/release/arena --replay <scratch>/swapped.txt --out <scratch>/swapped_replay.txt --workers 4
    python3 tools/wp16_warm_attribution_check.py <scratch>/swapped.txt <scratch>/swapped_replay.txt target/release/pistol

**WHAT THE OUTPUT MUST SHOW, AND THE DEFECT CLASS EACH CRITERION EXCLUDES.**

**Criterion W-1 — the honest arm.** The replay must report `8 of 8 game(s)`
and `0 divergence(s)`, AND every game's `nodes_a`/`nodes_b` in the replay
document must EQUAL the same game's `nodes_a`/`nodes_b` in the report, and the
checker must exit 0.

- *The defect class this excludes*: **a replay that is not actually warm** —
  one that re-drives the engines cold, feeds them a desynchronised position
  sequence, or silently skips searches. This is the class that killed the
  window-widening design (D-383) and it is the only way Criterion 1'''s whole
  premise can be false while every visible number still looks plausible.
- *Why it is not vacuous, in the form CLAUDE.md asks for first*: the node
  counts are an EXTERNALLY DERIVED REFERENT. They were computed by the
  GENERATION path — `exchange::ask` folding each `info totals` line as the
  original run played — and written into the report before the replay existed;
  the replay recomputes them from a second, later set of processes that do not
  share that input. A cold or desynchronised replay reaches different
  positions, hence runs different searches, hence spends different nodes.
  **`0 divergence(s)` ALONE WOULD NOT BE A CRITERION**: a replay that never
  asked an engine anything reports zero divergences too. Node equality is the
  half that defect cannot preserve, and it is why the two halves are registered
  together rather than either alone.

**Criterion W-2 — the seeded-defect arm.** The label-transposed copy must
FAIL: `arena --replay` exit 1 with a divergence in every game, and the checker
exit 1 naming at least one CONFIRMED INVERSION.

- *The defect class this excludes*: **an attribution criterion that cannot see
  a seat swap.** Not hypothetical and not old — two revisions of a WP-1.5b
  pre-registration registered dry-run criteria that PASSED on an arena mutated
  to invert the entire verdict, which is why that document's own criterion
  became a three-link chain.
- *Why it is not vacuous*: W-2 is a criterion the honest arm's PASS cannot
  produce. An instrument that passes everything fails W-2; an instrument that
  refuses everything fails W-1. Only one that DISCRIMINATES satisfies both, and
  the swap changes nothing but the labels — every move, every result, every
  digest in the document is the honest run's own.

**THE REGISTERED CONSEQUENCE OF THE DRY RUN**: if either criterion is not met,
this revision does not go to review and the governed run is not launched.

**TAKEN AT THE GOVERNING REVISION, AND THE FIRST ATTEMPT WAS NOT.** An earlier
execution of this subsection used a `target/release/arena` built one commit
before `bfdf933`. A Rust binary's bytes move when its source does, comment or
not — its `-Cmetadata` hash closes over the source — so those bytes were not the
registered instrument's, and CLAUDE.md's instrument rule says a run stands on
the revision that governs it. The whole subsection was re-taken from a
`--release --locked` build at `bfdf933`; the figures below are that re-take.
Recorded rather than quietly re-run, because "a doc-comment edit moved the
instrument" is exactly the kind of thing a reviewer should be told rather than
have to notice.

**WHAT THE DRY RUN RECORDED.** Every artefact is gitignored (rule 8) and is
named here by content.

| | |
|---|---|
| run | `artifacts/wp16_warmreplay_dryrun_run.txt`, `sha256 6e2a531c8e346b23a661fd96abef15f847e7c6f60cc0d8ac4a8813e7e007c793`, **`timing n_workers 4 wall_ms 14341`** read off the artefact itself, `VERDICT inconclusive_at_game_cap` (four openings cannot cross a boundary and this is not read as anything) |
| warm replay | `artifacts/wp16_warmreplay_dryrun_replay.txt`, `sha256 cf91e3fa9484d1ffcd7e0573ef2f349452e8065fa14c5f45d9214d1e31ad6170`, `arena: replayed 8 of 8 game(s) … 0 divergence(s)`, **`timing n_workers 4 wall_ms 14305`** read off the artefact itself, **201 compared turns** summed over the eight `replay` records |
| **W-1** | **MET.** `W coverage: 8 game(s) accounted for — 8 replayed in full with every node count equal to the run's, 0 halted at a divergence`; `W classification: 0 divergence(s), 0 confirmed inversion(s), 0 unexplained`; `(b): 0 inert pair(s) excluded by theorem, 4 pair(s) directly attributed at their first differing searched turn, 0 unattributable`; `1b: 5 decided non-forfeit game(s)`; `1c: 8 game(s) and 4 pair(s) rebuilt off the score_a path`; `PASS — 0 failure(s)`, exit 0, in `0.029 s` |
| seeded swap | `artifacts/wp16_warmreplay_dryrun_swapped.txt`, `sha256 377521bfd08408c395402d37e238ce9bdfeaebe5b26579358f0afb0001595882`; its replay `artifacts/wp16_warmreplay_dryrun_swapped_replay.txt`, `sha256 b63395e2b8c2d6f1d467920b6edcf5e167626ae07a19e3b252c86925901b4eca` |
| **W-2** | **MET.** `arena: replayed 8 of 8 game(s) … 8 divergence(s)`, exit 1; checker `W classification: 8 divergence(s), 8 confirmed inversion(s), 0 unexplained` — every one at turn 5, the first turn either engine searched — and `FAIL — 13 failure(s)`, exit 1. Link 1c independently caught the same corruption from the other direction (`1c counts wins_a 2 against 3 rebuilt from the game lines`), which is the chain doing what a chain is for |
| second instrument | `tools/wp15b_attribution_check.py`, unmodified, on the SAME honest report: `1a: 16 turns replayed, 10 of them discriminating, 8 of 8 games directly attributed by replay`, `PASS — 0 failure(s)`, exit 0, `6.485 s`. **Its registered agreement criterion (§7A.1) HOLDS**: every game it attributes by a discriminating replayed turn, the warm pass records `status clean`. The 16-against-201 gap is the measurement of what revision 3 bought |

**THE REPLAY'S COST IS MEASURED, AND IT HAS NOW BEEN MEASURED FOUR TIMES.**
`14305 / 14341 = 0.997x` here, off the two artefacts' OWN `wall_ms` lines, which
is what a reader can recompute from the digests above. (A shell `time` around
the same two commands read `14.409 s` and `14.368 s` — the same ratio, larger
absolutes, because that wrapper also counts process startup and the summary
print. Revision 4 quoted the wrapper figures beside artefact digests that do
not carry them, which a reviewer could not source; the artefact figures are the
registered ones.) `1.003x` on the pre-revision execution
described above; `0.994x` by the fresh-context reviewer of the implementation,
on its own machine state (`docs/decisions.md` D-413); and `1.003x` again by
revision 4's own reviewer, re-executing this subsection independently (D-416).
Four samples straddling 1.0 is the honest reading, and it is the reading this
document registers: **the warm replay costs ABOUT ONE RUN**, which is what the
design DECLARED as "~1x". No single sample's third digit is load-bearing, and
none is quoted as though it were.

**WHAT THIS DRY RUN IS NOT.** It is not a measurement of either engine. Four
openings cannot cross an SPRT boundary at these bounds and are not meant to,
and nothing in it may be quoted as strength. It is also not a governed sample
and does not consume this document's first run.

---

## 9. FILL-IN slots

Per D-396's delegation record, most of what WP-1.5b left as OPERATOR-CONFIRM
is fixed directly by the dispatch that authorises this document. Two remain
genuinely deferred — not to an operator, but to Step 6's own launch, because
they can only be known then.

**9.1 `elo1`.** FIXED at `15.0` (§2), by D-396's dispatch. Not re-opened here.

**9.2 `binary_sha256`.** Recorded from `sha256sum target/release/pistol`
after `cargo build --release --locked --bin pistol` at Step 6's own launch
revision, for BOTH seats. Rebuild means re-record.

**9.2a `openings_skip`.** FIXED at `500` (§3) by revision 4, not an
OPERATOR-CONFIRM slot and not a launch-time one: it is decided HERE, before the
run, because which games are played is part of the experiment and choosing a
window after seeing anything would be the after-the-numbers move this document
exists to forbid.

**9.3 `openings_take`.** FIXED at `500` (§3), by D-396's dispatch. Clears the
100-pair floor (§2) with wide margin, matching WP-1.5b's own governed run
shape so the two are comparable in scale.

**9.4 The design gate's state.** `docs/wp16_quiescence_design.md`'s own review
sequence is GREEN as of D-394, at a revision this document's own provenance
paragraph names; Step 6 confirms `git log` shows no further amendment to that
document between D-394 and the run's own launch revision before treating this
slot as still discharged.

**9.5 The calibration probe.** Discharged, this session, per §7 —
`hang_timeout_ms = 120000` leaves a ~412x margin over the MEASURED worst
single search (291 ms) of the `defensive_only` seat at the registered
50 000-node budget, over two independent 24-position sweeps. NO-CHANGE, per
R-9.5d's own convention.

**9.6 The run's revision.** The commit Step 6's games are played at, recorded
before the first game, so the report's `experiment_sha256` has something to
be compared with.

**9.7 `configs/arena_wp16_defensive_only_vs_staged.toml`.** Authored and
committed at Step 6's launch, once §9.2/§9.6 are known — "the document that IS
the experiment" (§3).

---

## 10. What flips this document

An amendment to any section reopens its review, however small the diff. It
binds the instruments too, and revision 3 added several. Each is named below
with WHERE its revision is pinned, because revision 4 claimed §7A.1 pinned them
all and it did not:

| Instrument | Pinned at | Where |
|---|---|---|
| `crates/pistol-arena/src/seats.rs`, `transcript.rs`, `replay.rs`, `replay_report.rs`, `bin/arena.rs`'s `--replay` mode | `bfdf933` | §7A.1 |
| `tools/wp16_warm_attribution_check.py` | **`385631f`** — `bfdf933` through revision 5, `43e8a86` in revisions 6-7, `385631f` now | §7A.1 |
| `target/release/arena` and `target/release/pistol`, BY CONTENT | the two `sha256` digests, unchanged by revision 6 | §7A.1 (a rebuild is a re-record, and a re-record is an amendment) |
| `tools/wp15b_attribution_check.py` — the SECOND INSTRUMENT, not this criterion's | `bfdf933`, unchanged by this WP. **§8.2 pins the same file a second time**, as "the commit this document lands at"; the two pins denote IDENTICAL CONTENT — the file was last modified at `a80a864`, long before this WP, and `git diff --stat bfdf933..HEAD -- tools/wp15b_attribution_check.py` prints nothing — so neither is false, and revision 6 records both rather than leaving §10 claiming to be the only place (D-419 MINOR F) | §7A.1's second-instrument paragraph **and** §8.2 |
| `tools/baseline_snapshot.sh` | `9282dd0`, as §7A.2 states it | **§7A.2**, not §7A.1 |

The claim itself flips on the run (§5). The DOCUMENT flips if
`docs/wp16_quiescence_design.md`'s own gate is not green at the run's revision
(§9.4), if the committed staged policy or its `q_depth_turns`/`q_triggers`
defaults move before the run (which would change what engine B is), if the
arena's verdict vocabulary changes, or if `identity_lines` gains the
`q_depth_turns`/`q_triggers` fields §8.3's Criterion 2 finding names as the
fix — at which point Criterion 2 is re-registered as a real gate rather than
staying demoted, and that re-registration itself reopens this review.

---

## 11. REVIEW STATE

**REVISION 8 (this text) IS UNREVIEWED AND GOVERNS NOTHING YET.**

| Revision | State |
|---|---|
| 1 (`43b5d78`) | FAILED its first fresh-context review — one BLOCKING, one MAJOR |
| 2 (`731150a`) | Fixed both, reopened the review, and PASSED a scoped fresh-context review with zero findings against the fix (**D-400**). It governed the run D-401 took, which failed its own Criterion 1' and is retired as evidence for anything this WP concludes, under any criterion, ever (**D-402**) |
| 3 (`8820e91`) | Committed; its review was dispatched and then WITHDRAWN before returning, when the author found §3's `openings_skip` contradicted the commissioning dispatch's own Step 7. No run was taken under it |
| 4 (`20f9b26`) | **FAILED its governing review — 1 BLOCKING, 2 MAJOR, 5 MINOR (D-416)**, report at `docs/experiments/wp16_prereg_rev4_REVIEW.md`. The BLOCKING was that §7A.1 registered a criterion the shipped instrument no longer implemented. Per the dispatch's own rule for that step the session STOPPED; no run was taken |
| 5 (`de53f5d`) | **FAILED its governing review — 0 BLOCKING, 2 MAJOR, 7 MINOR (D-419)**, report at `docs/experiments/wp16_prereg_rev5_REVIEW.md`. Document-only, and its receipts rule verified overwhelmingly clean — all twelve quoted instrument strings character-exact, all ten named tests present and driving. **Both MAJORs were that rule's own dividend**: MAJOR A, an exit-2 enumeration presented as complete that was not; MAJOR B, a registered receipt found FALSE by being tested. MAJOR B was an INSTRUMENT defect, so the session stopped rather than editing around it. No run was taken |
| 6 (`3a198de`) | **FAILED its governing review — 0 BLOCKING, 1 MAJOR, 2 MINOR (D-421)**, report at `docs/experiments/wp16_prereg_rev6_REVIEW.md`. The first revision of this document that was NOT document-only. Its reviewer re-ran all five mutations (all killed), reproduced both binary digests byte-exactly, verified all nine quoted instrument strings character-exact and all seven of D-419's MINORs closed. **The MAJOR was the invariant revision 6 itself introduced**, found false by being TESTED with the instrument's stdout on `/dev/full`. The cap's FAIL-ON-THE-DIFF branch fired: one fix round, a scoped re-review, then STOP regardless |
| 7 (`1618467`) | The ONE licensed fix round under revision 6's cap. Closed the MAJOR by NARROWING the invariant and adding a fifth registered check. **FAILED its scoped re-review — 0 BLOCKING, 2 MAJOR, 2 MINOR (D-423)**, report at `docs/experiments/wp16_prereg_rev7_REVIEW.md`. The scoping HELD (proved by a per-section sha256 over all 28 sections) and MINOR 3 closed fully, but the exit-2 fix landed in §5 and not in §7A.1's copy, so the document shipped self-contradicting; and the fifth check was green at the revision where the defect was live, so it was no more a falsifier than the four it joined. The cap was exhausted and the session STOPPED |
| 8 (this text) | **The operator's OVERRULE (D-424), not a fourth narrowing.** DELETES the exit-2 partition that failed three reviews, on the ground that both its sides had the same registered consequence and so licensed no different reading. States the taxonomy ONCE, in §5, with §7A.1 pointing at it. Replaces the five greps with a driving test the defect falsifies, which required moving the instrument — `385631f`, which also closes D-422. **NOT document-only**; this review reopens WHOLE |

**THE INSTRUMENT THIS DOCUMENT REGISTERS HAS ITS OWN REVIEW RECORD, and it is
in §7A.1 rather than only in the ADR log**: the implementation FAILED its
REVIEW-impl with 1 BLOCKING and 5 MAJOR (**D-413**); one fix round closed all
thirteen findings with every fix verified by the mutation that survived before
it (**D-414**); the operator ruled that no second implementation review would
be held and moved two duties onto this document's review instead (**D-415**),
both of which were discharged — the mutation table re-ran at the governing
revision with all six mutants dying, and the word-by-word semantics duty is
what found D-416's BLOCKING. **Revision 6 adds one more entry to that record**:
`43e8a86`, the MAJOR B fix, which reproduced the defect against the pristine
instrument first, ships two driving tests against the SHIPPED script, and was
verified by a five-mutation table run in a separate worktree — every mutation
killed, including one whose only job is to prove the tests' own controls are
load-bearing.

A fresh review — SCOPED to revision 7's three fixes, because revision 6's whole
review has already been taken and its cap licenses exactly one fix round and
one scoped re-review — must pass before the governed run this document
describes may be launched. Any diff outside this document voids that scoping.
§9.2/§9.6/§9.7's remaining slots are filled at that run's own launch, after
this review is green.


