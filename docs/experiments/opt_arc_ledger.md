# Optimization arc — the ledger. A successor continues from THIS, never from memory.

**HOW TO READ IT.** One section per package, appended to as it runs. A step is
CLOSED only when its row says so and names the receipt. Anything not written
here did not happen. The governing dispatch is archived verbatim at
`docs/experiments/opt_arc_DISPATCH.md`; the package order there is RULED and this
ledger never reorders it.

**THE PRIOR ARC'S ACCOUNT** is `docs/experiments/arc3_CLOSURE.md` and its ledger
`docs/experiments/arc3_ledger.md`. The arc began at `ffc5c10` (arc III's closing
commit) with `git worktree list` holding the main tree alone, no cargo process
alive, and `git status` clean — checked at 23:01 UTC on 2026-09-03 before
anything below ran.

**THE LOOP GRANT** is the dispatch's: up to three rounds per review gate, the
third remedies-only, a third failure is STOP and split (D-565's shape; D-585's
five was arc III's and expired with it).

---

## §P1 — threat-state maintenance (A-03). IN PROGRESS.

| step | state | receipt |
|---|---|---|
| re-profile at the package's own SHA (D-477) | done, `ffc5c10` | `artifacts/p1_profile_ffc5c10_v1.txt` sha256 `ed5f0cab…` |
| laziness-yield counters (the premise measured) | done, `ffc5c10` | `artifacts/p1_counters_ffc5c10_v1.txt` sha256 `f7e07c4f…` |
| option prototypes, each on its own local branch | done | `p1/mx-A` `cc4153b`, `p1/mx-D1` `5f41dc9`, `p1/mx-D2` `1c23954`, `p1/mx-E0` `eecf268`, `p1/mx-E` `4298ecd`, `p1/mx-EA` `e78a094` — measurement revisions, never landing candidates as committed |
| matrix benches, `tools/bench_delta.sh` path mode, 5 reps | done | `artifacts/p1_mx_bench_{A,D1,D2,E0,E,EA}_v1.txt`, digests in the matrix §4.1 |
| option matrix revision 1 | written | `docs/experiments/matrix_P1_threat_state.md` |
| DECISION-RED-TEAM round 1 | done — *"the recommendation SURVIVES but the matrix FAILS (revise: M1, M2, M3)"*; every number reproduced, three record defects, eight minors | `docs/experiments/matrix_P1_threat_state_REDTEAM.md` at stash `6099aa7`; its receipts exported to `artifacts/p1_rt_round1/` (`p1_rt_round1_digests.txt` `a63ee080…`) |
| matrix revision 2 | written — §0 answers M1–M3 and m1–m8; O-I added; §1.1 repo-wide; counters instrument restored in `artifacts/p1_counters_ffc5c10_v2.txt` (`65a16dee…`) | `docs/experiments/matrix_P1_threat_state.md` |
| DECISION-RED-TEAM round 2 | done — *"the recommendation SURVIVES but the matrix FAILS (revise: R2-M1, R2-M2)"*; every round-1 remedy executed and holding; the solver-on seat measured once (about 2 % / 0 %) | `docs/experiments/matrix_P1_threat_state_REDTEAM_round2.md` at stash `40b69e3`; receipts `artifacts/p1_rt_round2/` (`p1_rt_round2_digests.txt` `d5b695ee…`) |
| matrix revision 3 (remedies-only, the last granted round) | written — §0.1 answers R2-M1, R2-M2, R2-m1–m8; the prereg's identity leg gains the solver-on seat; the design's §6 corrected | `docs/experiments/matrix_P1_threat_state.md` |
| DECISION-RED-TEAM round 3 (remedies-only, against the diff) | done — *"the recommendation SURVIVES but the matrix FAILS (revise: R3-M1)"*; R3-M1 is in the prereg's identity block, every matrix line holds on execution; four matrix minors | `docs/experiments/matrix_P1_threat_state_REDTEAM_round3.md` at stash `e13c3e4` |
| round-3 disposition (F-P1.4) | ARCHITECT DEFAULT APPLIED, flagged for the operator | this ledger, F-P1.4 |
| matrix minors R3-m1–m4 corrected; scoped verification pass over the corrected lines and their defect class (D-598) | **PASS** — the four hold, the class sweep re-derived every number in the document and found three minors (V-1 "eleven" for ten undo sites, transcribed from round 1's prose; V-2 five readings headed "four" and misordered; V-3 a header sentence widened past what round 3 said), all three corrected in place. **THE MATRIX IS CLOSED**: revision 3 as amended, selection O-E | `docs/experiments/matrix_P1_threat_state_VERIFICATION.md` |
| selection record | written — O-E; round 3's strongest surviving attack carried verbatim | `docs/experiments/matrix_P1_threat_state_selection.md` |
| selection record | not started | — |
| design | written, revision 1 | `docs/experiments/p1_design.md` |
| REVIEW-design round 1 | done — *"VERDICT: FAIL (D1, D2)"*, nine minors; every premise attack on the mechanism run to ground, none landed | `docs/experiments/p1_design_REVIEW.md` at stash `faf7a54` |
| design revision 2 | written — D1 (M10 replaced; the equivalent filter mutant named and not registered), D2 (the run reader's reach `[-513, 512]`), m1–m9; M13/M14 added for the key and projection classes; `apply` only sets a bit, pruning in `restore` | `docs/experiments/p1_design.md` |
| REVIEW-design round 2 | done — *"VERDICT: FAIL (N1)"*, eight minors; every round-1 remedy held on execution. N1 is real: `apply` copied the enumeration and a mutant truncating it at the edge survived every test | `docs/experiments/p1_design_REVIEW_rev2.md` at stash `8a2333b` |
| design revision 3 (remedies-only, the last granted round) | written — `apply` calls `windows_through_indexed`; I5/I6 read every class set; M15 registered; N2–N9 | `docs/experiments/p1_design.md` |
| REVIEW-design round 3 (remedies-only, against the diff) | **PASS** — N1 executed in both halves (no enumeration copy remains; X9-shaped mutants die at I6 on the window round 2 found; M14/M15 die as stated); three minors after the pass, corrected in place: R1 I5's set clause was empty-against-empty under alternating sides (the fixture gained a one-side colouring, `p1/impl` re-mutated), R2 "two logs" in I7, R3 an attribution. **THE DESIGN GATE IS CLOSED** at revision 3 as amended | `docs/experiments/p1_design_REVIEW_rev3.md` at stash `e0ddc14` |
| IMPL | `p1/impl` = `e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd` (round 2's F14; before that `d42d33d0c8ad640d7138a14caa5a1176d49fd2c0`, round 1's F4 and F5 comments, and before that `3ef67068a15880c15e372b22565e0f4f37de8e76`) — the design gate is closed, so this is now IMPL proper: `724f050` plus I5's one-side colouring (round 3's R1); before that (round 2's N1: `apply` takes its windows from `pistol-core`'s enumeration; the boundary tests read every class set; the root doc's fifth passage; before that `ddb8a7b` (`5988cbe` plus the two in-crate doc corrections round 1's m5 named; before that (`38fbfb2` plus the removal of the store's unreachable cleared-bit branch, found by the first mutation run) in the measurement worktree (the E prototype plus the `line.rs` split, the contract docs, the crate-doc amendments, the `reset_to` comment, four new oracle tests); fmt and clippy clean, solver + search suites green; NOT landable until REVIEW-design passes and the code is re-checked against the reviewed design | `git -C /home/tom/pistol-wt/p1-measure log -1 p1/impl` |
| mutants at call sites (D-553), green before REVIEW-impl | **15 of 15 DEAD at `e32f8c4bf51ec99ec3ffcb3f898910de809bc7cd`, each with its death reason on the receipt** (run 10; `artifacts/p1_mutation_e32f8c4_v1.txt`, driver `artifacts/p1_mutation_driver.py`). M14 dies `THREAT_DESYNC: p2 stone on -32768,0 lands on a cell of its ConstR line that already holds one` — the refusal the design's §4 row names — and M12's call-site removal dies by the same refusal at the origin. Earlier: 15 of 15 at `d42d33d…` and at `3ef67068a158…`, 14 at `ddb8a7b`, 12 at `5988cbe` | `artifacts/p1_mutation_5988cbe_v1.txt` `833c20b6…`; driver `artifacts/p1_mutation_driver.py` `8205bfe7…`; earlier runs in `/home/tom/pistol-wt/p1-patches/p1_mutants_run{1,3}.log` |
| REVIEW-impl | round 0 VOID (rate limit, D-597, F-P1.6). **Round 2** at `d42d33d0c8ad640d7138a14caa5a1176d49fd2c0`: **FAIL (F11, F16; four minors)**, no BLOCKING standing and *"the code is not wrong on any check"*. All five round-1 findings verified discharged by execution; the mutation re-run byte-identical at 15 of 15 with M14's reason the refusal the design names; the suite 1 106 passed over 173 suites; all six gates green; and **the discharge shown to have moved nothing** — every changed line is a comment and the release binary's digest is unchanged from the previous revision and from the RED-TEAM's own subject, so the earlier round's results are results about this revision. F11: the ADR's headline said MEASURED of two unfilled slots and named one seat where three are registered. F16: the round was dispatched against two documents pinned to nothing, which this session then edited mid-review (F-P1.14). Round 1 at `3ef67068a158…` (full SHA in F-P1.10): **FAIL (F1)** — one BLOCKING finding, documentary: the ADR line the design's §2 requires was never appended. **The code passed every check**: the mutation receipt reproduced byte for byte, eight further defect classes the reviewer designed all died, the `[-513, 512]` chunk range formed by execution at all four lattice extremes, 1 106 tests passed / 0 failed, five gates green by their own log lines, fmt and clippy clean, rule 9 met, nothing outside Scope. Discharged: `D-603` appended; F4 and F5's comments corrected; F2's driver gap and F3's stale review row recorded below | `docs/experiments/p1_impl_REVIEW.md` |
| RED-TEAM (rules/data paths, adversarial inputs) | round 0 VOID (rate limit, D-597, F-P1.6). Round 1: **NO WRONG BYTE FOUND** — **1 167 813** state steps against the test crate's independent reference (every query, the snapshot, `masks`, `window_count`, whole-state equality on LIFO unwind) over all 64 chunk residues on all three axes, straddling and non-adjacent chunks, the four `i16` corners, the `q+r` extremes, 257-stone chains, dense grids, clones with diverging undos; **1 763** two-binary transcript pairs, **0** differing, over six committed configs including all three solver-armed ones, plus 56 hostile protocol inputs and 80 sessions exercising `reset_to`; determinism at five seats and the candidate against itself over 832 pairs; the solver, staged-soundness and search oracles all green. Resource shape on a 2 025-stone grid, solver armed: **+0.11 % peak RSS, −8.7 % wall**, same bestmove. No BLOCKING, no MAJOR | `docs/experiments/p1_REDTEAM.md` |
| bench pre-registration revision 1 | written | `docs/experiments/p1_bench_prereg.md` |
| its gate, round 1 | **FAIL (B1, B2; M1, M2, M3)** and ten minors. B1: §1.2's dry run was registered and never taken, so `bench_delta.sh` in `rev:` mode — the mode the landing bench uses and the only stage that criterion is about — had never been exercised in P1, every bench behind the matrix being PATH mode. B2: the instrument table omitted `configs/eval_v0_weights.toml`, which the harness reads live and digests. M1: one bracket registered for two bands whose grounds differ, the late band's wrong in the permissive direction. M2: the idle receipt could not print `idle` and matched its own invoking shell. M3: five gates named where six exist. **What held**: the §2.1 block re-executed independently, 128 bestmove and 0 error per side, both digests reproduced, R3-M1 closed with no new one in its place | `docs/experiments/p1_bench_prereg_REVIEW.md` at stash `988c572` |
| prereg revision 2 | written — per-band brackets from their own grounds, both missing instruments listed, the receipt respelled and tested, the block writing to a scratch directory, the ten minors taken | `docs/experiments/p1_bench_prereg.md` |
| its gate, round 2 | **FAIL (B3; M4, M5, M6)** and nine minors — **all thirteen the dispatcher's own**, and round 1's fifteen remedies each verified discharged by execution first. B3: the m9 insert landed between the abort's trigger and its consequence, so `[1.10, 1.15)` was licensed both to land and to STOP, and `[1.15, 1.190)` had no disposition at all. M4: the recorded block digest was of the extraction wrapper, not the block, and a second `sh` fence had made extraction ambiguous. M5: the run added in the same section falsified the drift figure that section registered (late spread 0.010, not 0.001). M6: the ADR line cited the withdrawn bracket and the withdrawn gate count | `docs/experiments/p1_bench_prereg_REVIEW_rev2.md` at stash `1ee0ddd` |
| prereg revision 3 | written — the disposition is one row per interval with none doubled and none unassigned; the block is the document's only `sh` fence and its extraction command is registered; the drift grounds are re-derived over all three runs of the pair; the history lives in one paragraph (round 2's n9); the six gate scripts named with derived revisions; D-603 corrected | `docs/experiments/p1_bench_prereg.md` |
| its gate, round 3 (remedies-only, the last the grant holds) | **FAIL (B4; M7)** — and under the grant this is a **STOP**. Twelve of the thirteen round-2 remedies hold on execution, including the registered extraction (one block, digest matching), the block itself (128 bestmove / 0 error, `284f70af…`, exit 0, tree unchanged), the receipt's three states without killing `set -e`, M5's arithmetic and all fourteen §0 revisions. **B4 is the B3 remedy reintroducing B3 one level up**: the new governing sentence gave three incompatible rules in 28 words — *worst band*, *lower ratio*, *lower row* — which come apart because the two brackets have different floors, so `early 1.205 / late 1.192` reads PASS under one clause and BELOW BRACKET under the others, and `1.310 / 1.280` reads PASS against STOP. M7: §0's *"Replication is the 5 reps"* survives while the new §1.2 says the 5 reps cannot see the drift the brackets are about | `docs/experiments/p1_bench_prereg_REVIEW_rev3.md` at stash `189c6d6` |
| revision 4 | written, **AND ITS GATE IS SPENT — these fixes are UNVERIFIED and this row does not claim otherwise.** One rule replaces three: each band is read against its own bracket and the package takes the more severe of the two rows, severity stated as a list, nothing comparing the bands to each other. M7's sentence deleted and pointed at §1.2 (D-424). The five minors taken. The author tested the new rule by exhaustion over nine ratio pairs — every pair yields exactly one disposition — which is evidence for the operator's ruling and not a substitute for the round the grant no longer holds | `docs/experiments/p1_bench_prereg.md` |
| **§P1 STOPS HERE, at the pre-registration's gate** | the CODE is done and verified (15 of 15 mutants dead with reasons, 1 106 tests, six gates green, a RED-TEAM finding no wrong byte in 1 167 813 state steps and 1 763 transcript pairs); what is spent is a DOCUMENT's review grant, and the document governs the landing bench that rule 5 requires before a perf change lands. The disposition is the operator's: a fourth round, D-598's scoped verification pass over the corrected lines and their defect class, or the split the dispatch's STOP protocol names. **Nothing is landed on `dev` and no governed run is taken until that ruling** | this ledger |
| revision 3's block, RUN before dispatch | **exit 0**, receipts `idle` both ends, 128 bestmove / 0 error per side, transcripts `284f70af…` — the same digest across three spellings and two independent reviewers — `git status` unchanged across the run. **Its first execution exited 1** and that is recorded rather than smoothed: a bare `pgrep` returning "no match" under `set -euo pipefail` killed the block after it printed its answer, which is `SHELL_CHECKLIST.md` item 12 in the receipt that says whether a measurement is trustworthy; §1.3 carries the three-way capture and the reason | `artifacts/p1_identity_dryrun_E_v3.txt` |
| the two owed RUNS, taken behind an idle gate before revision 2 was dispatched | **both criteria hold.** §2.4: revision 2's block re-run, both receipts printing `idle`, 128 bestmove / 0 error per side, transcript digest `284f70af…` — the same digest revision 1's run and the gate's reviewer each produced, so the rewrite moved the output and not the question. §1.2 (B1's remedy): `rev:` mode exercised for the first time in P1 — **1.258 early against the referent 1.257, 1.231 late against 1.240**, both inside ±0.03, node identity holding, exit 0. **The instrument built both sides itself and reproduced the hand-built binaries' digests exactly** (`78a7600a…`, `f333d101…`), which closes the matrix's fourth listed attack by instrument | `artifacts/p1_identity_dryrun_E_v2.txt` `dac89bfc…`, `artifacts/p1_dryrun_rev_mode_v1.txt` `a2f20ad5…` |
| landing bench, identity leg, solver oracle gates | not started | — |
| landing bench + identity leg, TAKEN on the finished code under D-604 | **nps 1.243 early / 1.217 late, both INSIDE the brackets registered before any run**; time-to-depth 1.203 / 1.190; node identity per position at both budgets in all five reps; idle receipts both ends; **identity leg IDENTICAL**, 128 bestmove / 0 error per side at three seats, transcripts `284f70af…`. The benched binary and the landed binary are one digest, `1413698a22ff…` | `artifacts/p1_landing_bench_v1.txt`, `artifacts/p1_identity_landing_v1.txt` |
| landing | **`56ee55fd92b6bf4f781bc795308cc98673305ff0` on `dev`**, one commit: eight source files, every P1 document, `D-603` and `D-604` | `git log -1` |
| CI at the landed commit | **twenty gate lines, `gate 1/20` through `gate 20/20`, `ci: all gates passed`, `CI_EXIT=0`** | `artifacts/p1_ci_56ee55f.txt` |
| closure | written | `docs/experiments/p1_CLOSURE.md` |
| **§P1 CLOSED** | the STOP recorded above was resolved by operator overrule (D-604), not read away: the disputed rule was fixed, the numbers were measured, and the fix is recorded as unreviewed | this ledger |

### FINDINGS RAISED IN §P1

**F-P1.1 — the premise's yield is measured and it is small.** The dispatch's
P1 text says the maintenance is *"paid on every place/undo regardless of
whether the node queries threat state"*. That is true of the mechanism and
nearly empty of yield: at the horizon `Run::quiescence` runs the free win-now
and overload checks through `staged_context()` even at `q_depth_turns = 0`
(`crates/pistol-search/src/quiescence.rs`, `gate_row`), so every leaf reads the
state, and the stones undone without a read are the TT-cut and rule-4-win
children only — 5.3 % / 8.6 % / 6.9 % of places at the three seats in the
counters artifact. The pending count at a flush is one at every flush but the
root's own. So "lazy" and "batched" are measured at ≈ 2 % and 0 % respectively,
and the package's lever is the cost of a touch, not its count. The matrix says
this with its numbers.

**F-P1.2 — D-254's flip clause fires.** D-254 recorded per-axis line bitboards
as *"an option nobody considered … recorded and not pursued"*, flipping *"when a
bench with `p > 0` names window lookup as a measured hotspot"*. The re-profile
names `WindowTable::masks` + `::set` at 17.05 % of wall. The option is in the
matrix and was prototyped rather than argued about.

**F-P1.4 — round 3's FAIL lands on the prereg's lines, and the split is taken
as the dispatch's STOP protocol spells it.** The matrix's third and last granted
round returned FAIL on one MAJOR (R3-M1) whose failing lines are
`p1_bench_prereg.md` §2.1–2.2 — the identity leg's command block, which read a
keyed fixture as bare and so registered a criterion no binary pair could meet —
and four MINOR corrections to the matrix that change no reading. The reviewer
wrote that *"whether a FAIL on the diff's prereg lines is the arc's
STOP-and-split or the prereg gate's first finding is a ruling, not a review
result"*. **ARCHITECT DEFAULT APPLIED, and it is the split, not the STOP**: the
dispatch's protocol is *"any document failing after the granted rounds = STOP
and split"*, and the split here is the one the documents already have — the
prereg is its own document with its own gate, which had spent no rounds, and
the failing block is corrected there (executed before that gate opens, per
D-591) and reviewed at that gate; the matrix's own lines never failed a round
(every number reproduced under the reviewers' own commands three times), so its
four minors are disposed of by D-598's scoped verification pass over the
corrected lines and their defect class rather than by a fourth round the grant
does not hold. **What would make this the wrong reading**: the operator ruling
that a FAIL on any line of the diff exhausts the matrix's gate regardless of
which document the line is in — in which case P1 STOPs here, with the matrix,
design and prereg drafts landed as records and the `p1/impl` branch held. The
operator may take that reading at any point before the landing bench runs;
nothing below is landed on `dev` until the gates that follow pass.

**F-P1.3 — a mislabelled binary, caught before it was cited.** The first E0
build failed on a dangling doc comment; the chain that followed copied the
previous build (EA's digest `8b33fff2…`) under the E0 name and launched a bench
on it. Caught by comparing digests before the artifact was read; the bench was
killed, its artifact deleted, the source fixed, the digest checked to differ
(`ba532dcf…`), and the bench re-taken. Recorded because *a binary named for a
revision is tied to it only by a digest check*, which is §6's fourth attack in
the matrix.

**F-P1.5 — the design's M10 is an equivalent mutant, found by the session while
writing the driver.** `p1_design.md` §4 registers M10 as *"snapshot filters no
axis (lists windows on all three axes through each bit)"*, dying at I1 because
*"a window with no stone would be listed"*. False: every window through an
OCCUPIED cell holds at least that stone, so the unfiltered enumeration lists
exactly the windows the filtered one does, three times over — the filter is
work, not correctness, and no test can kill its removal. The driver
(`/home/tom/pistol-wt/p1-patches/p1_mutants.py`) registers instead *"M10 the
snapshot's cell inverse off by one"*, which lists windows through the wrong
cell and dies at I8; the design's §4 row is corrected in its post-review
revision. Recorded because a mutation set with an equivalent mutant is a
receipt that counts a kill that cannot happen.

**F-P1.7 — the mutation driver counts a kill and never says WHY, and one review
row rode three documents on a word no single-test driver can establish.**
REVIEW-impl's F2: the receipt prints *"DEAD at <test>"* and nothing about the
mechanism, though rounds 2 and 3 of the design review had asked specifically
that M14's death be recorded as the class's — the reviewer executed it and found
M14 dies at I6 by the centre-bit refusal (`state.rs:108`) and at I1/I5/I8 by the
key inverse's `expect` (`line.rs:66`), which is the fact those rounds wanted and
the receipt could not carry. **The driver gains a first-panic-line capture and
the receipt is re-taken**; a driver that records why a mutant died closes the
class rather than the instance. F3: the row *"M2 dies at I5 alone"* was carried
through three rounds and is false — M2 kills eight of eleven oracle tests — and
*"alone"* is a claim a one-test-per-mutant driver cannot make in either
direction. The design's own §4 row never said it; the review rows did, and it
stops here.

**F-P1.6 — REVIEW-impl and the RED-TEAM were killed by the session's own rate
limit before either wrote a byte, and both are VOID under D-597.** Dispatched
against `p1/impl` = `3ef67068a15880c15e372b22565e0f4f37de8e76` at about 02:00 UTC on 2026-09-04; both agents died
mid-run on an HTTP 429 (*"session limit"*), neither wrote its report file, no
finding was raised and no verdict returned. D-597: *"a review whose agent dies
of a cause the subject cannot influence — a rate limit, a killed process, a
crashed harness, an exhausted filesystem — is VOID; the round is re-dispatched
at the SAME revision and the grant is untouched."* Both were re-dispatched at
05:55 UTC against the same revision, onto the four worktrees the dead agents had
already built (`review-p1-impl`, `review-p1-impl-mut`, `rt-base`, `rt-cand` —
all clean, their release binaries digested and named in the new briefs), so the
cost of the death is the reading, not the builds. **The session also changed
model at that point** (Fable 5.1 to Opus 5) and the arc's commit attribution
changes with it; the P1 commits already on `p1/impl` carry the earlier
attribution and are not rewritten.

**F-P1.8 — a registered dry run is not taken by registering it, and this arc
proved it on its own document.** The prereg's gate found (B1) that §1.2's dry
run — the one that exercises `bench_delta.sh` in `rev:` mode, the mode the only
banked number comes from — had been written, reviewed by its author, and never
run; every measurement behind the matrix was PATH mode over hand-built binaries,
which is precisely the attribution stage that criterion exists to close. It is
`docs/process.md`'s dry-run discipline failing in the one direction a reader
cannot see, because a registered-and-untaken run reads exactly like a taken one
on the document's face. **The structural answer taken here**: a dry run's section
carries its OUTPUT or it carries the words "not taken", and the two owed runs are
executed before the revision that claims them is dispatched (D-591's shape,
applied to a document's own registrations rather than to a reviewer's remedy).

**F-P1.9 — the box is shared and an idle receipt that cannot print `idle` is
not a receipt.** The prereg's registered receipt used an unbracketed `pgrep -af`
pattern, which matches the full command line of the shell invoking it, so it
reported itself on an idle machine and returned a line even for a pattern
matching nothing — measured by the gate's reviewer. Respelled with the bracket
idiom (`[c]argo`), tested both ways. **And the hazard is live, not theoretical**:
while this document was being revised, an unrelated project's `cargo test` was
observed building on the same machine, so every timing run in this arc is taken
behind a receipt that can actually say what else was running.

**F-P1.12 — THE DISPATCHER DID IT AGAIN, INSIDE THE EDIT FIXING IT.** While
writing the prereg's revision 3 — whose §0 gains a row per gate script precisely
because round 2's n8 found them unnamed — five of the six revisions were typed
into the table before the command that derives them had been run, and all five
were wrong (`9390b48, 39b0ce1, 2a12e3f, 6f7b26f, 6f7b26f` against the true
`0a80a7b, 55e4b56, 8ce13ff, e668dfa, 3916afd`). They were caught in the same
tool call, because the deriving command ran beside the edit rather than after
it, and corrected before the revision was dispatched. **The lesson is narrower
and more useful than F-P1.10's**: a document's table of revisions is FILLED FROM
the command's output, never typed and then checked — the check is what caught
this one, and a check is a thing that can be skipped where a fill cannot. Both
instances are the dispatcher's, in an arc whose every review round is about this
class.

**F-P1.10 — THE DISPATCHER INVENTED A CHARACTER OF THE SHA IT DISPATCHED
AGAINST, and the class is this arc's own.** Both the REVIEW-impl and RED-TEAM
briefs named the implementation as `3ef67062`. The tip of `p1/impl` is
**`3ef67068a15880c15e372b22565e0f4f37de8e76`** — the seven-character prefix
`3ef6706` is right and the eighth character was written from nowhere. The
RED-TEAM found it (`git rev-parse 3ef67062` echoes its argument back rather than
refusing, so the mistake is silent at the shell) and confirmed the subject by
BINARY DIGEST instead, which is why no work was wasted. **The lesson is the
arc's own, one level up**: `docs/process.md`'s re-derivation clause is about
counts and citations, and a revision is a citation — a SHA is quoted from `git
rev-parse` or not quoted at all, and never truncated by hand to a length nobody
printed. **Every citation of this implementation from here carries the full
forty characters.** Recorded rather than smoothed, because the arc has now put
this defect inside a matrix, a design, an ADR history and a dispatch, and the
dispatcher is not exempt from the rule it briefs reviewers with.

**F-P1.11 — an identity harness that cannot tell a REFUSAL from an AGREEMENT
reports agreement, and the red team caught its own.** Its first position
generator emitted the two cells of a turn unsorted, which the protocol refuses
(smaller cell first); **32 of 75 positions were silently refused and 192
transcript pairs compared two identical refusals** and were counted as identical
answers. The reviewer found it mid-run, rewrote the generator behind its own
validator, and made the runner record a refused position as VACUOUS rather than
SAME; the final counts carry no vacuous rows. **Anyone reusing an identity
harness in this arc carries the same guard** — it is D-90's positive-content
argument in a new place: *"nothing differed"* is also what a comparison of two
things that did nothing reports.

**F-P1.13 — a registered command is READ by its reviewers and RUN by nobody
until someone runs it, and this document's own receipt proved it twice.** The
prereg's identity block was reviewed by two fresh contexts, executed by both,
and still carried a digest that belonged to an extraction wrapper rather than to
the block (round 2's M4) — because *"the block"* was ambiguous once the document
printed a second `sh` fence, and each party extracted it differently. Revision 3
registers the extraction command itself and digests its output. Then revision
3's own first execution **exited 1 after printing the right answer**: the
hardened receipt called `pgrep` bare, and a bare `pgrep` returning "no match" —
the quiet box, the outcome the receipt exists to report — trips `set -e` before
the `case` can read it. **Both are `tools/SHELL_CHECKLIST.md` item 12's shape**
(*"two different things exit non-zero: the answer is no, and I could not take the
answer"*), landing in the receipt whose whole job is to say whether a
measurement can be trusted. The standing rule this leaves: **a document that
prints a command registers how to extract it, and the extraction's output is
what gets digested and run** — not a copy, not a wrapper, and not a reader's
reconstruction.

**F-P1.14 — THE DISPATCHER SENT A REVIEW AT A MOVING TREE AND THEN MOVED IT,
WHICH IS D-602's OWN RULE BROKEN IN THE REVIEW OF THE LINE D-602 PRECEDES.**
REVIEW-impl round 2 was dispatched with the implementation pinned by SHA and the
design pinned by stash, but with `docs/decisions.md` and `p1_bench_prereg.md`
named by PATH and pinned to nothing — and while the reviewer read them, this
session rewrote both: D-603's bracket, its revision pin and its gate count, and
the prereg from revision 2 to revision 3. **Two of the reviewer's findings closed
underneath it**, one of them BLOCKING when raised (its F10: the ADR named the
bracket `[1.20, 1.30]` that the prereg had withdrawn, so a late-band 1.295 was
inside the ADR's bracket and outside the prereg's, and 1.195 the reverse). The
reviewer's own words: *"neither closure can be reproduced by checking anything
out"*. **D-602 says a search reported at a revision is RUN at that revision and
that a working tree is not a revision**; a review is a search over documents, and
the same rule binds the brief that sends one. **THE PRACTICE FROM HERE**: every
document a review reads is pinned in the brief by SHA or by a `git stash create`
commit — not only the subject — and the dispatcher does not edit a pinned
document while its review runs; a correction that cannot wait is a re-dispatch.
The saving grace is only that the prereg's own gate found the same defect
independently at its round 2, from a pinned copy.

---

## §P2 — legality per candidate (A-04). CLOSED AS A MEASURED FINDING, NOT LANDED.

| step | state | receipt |
|---|---|---|
| re-profile at the package's own SHA (D-477) | done, `45df2c8` | `Board::check_placement` **23.25 %** on a 1 025-stone grid, **below 1.5 %** at the bench seat |
| the audit's named call site, checked | **A-04 IS WRONG ABOUT IT** — the dominant caller is `GameState::place` (`state.rs:163`), the rules' own validation of every stone the search plays; a prototype fixing only the candidate filter measured **4 %** | `docs/experiments/p2_registration.md` |
| implementation | `p2/impl` = `7731d71a77624baa12cee5d0dfbb54bde303c313`: one `BTreeMap` range bound in `in_legal_region`, exact because hex distance dominates the axial component; a differential test against a full sweep over ~70 000 cells; 173 suites green, clippy clean | that branch |
| expectation and abort, registered BEFORE the runs | expected null, 1.00–1.02; abort < 0.98 | `p2_registration.md` |
| bench at the registered seat | **0.994 / 0.995** — missed the expectation, above the abort; identity leg **IDENTICAL** | `artifacts/p2_bench_v1.txt`, `artifacts/p2_identity_v1.txt` |
| crossover measurement | **pays only beyond ~160 stones**; a 40-turn game ends at 79 | `p2_registration.md` |
| **VERDICT** | **NOT LANDED.** A measured structural floor is a finding, not a failure (rule 5). The change costs half a per cent at every size the engine plays and wins only past any game's reach | this ledger |

**F-P2.1 — THE AUDIT'S PERF RECEIPT NAMED A REAL HOTSPOT AND THE WRONG CALLER,
AND ONLY A PROFILE COULD TELL.** A-04 quotes `candidates.rs`'s
`filter(|cell| board.is_legal_placement(cell))` and calls the cost *"paid per
candidate"*. It is paid there, and that half is worth 4 %; the other 19 points
are `GameState::place` validating every stone the search puts down. **The
package that trusted the ledger row would have optimised the wrong line and
banked a fifth of the available win** — and then measured a null at its own
bench seat and had no idea why. What separated them was one `perf` run and a
`git grep` for the callers, which is D-477's *"re-profiled at the package's own
SHA, never inherited"* doing exactly the work it exists for.

**F-P2.2 — A HOTSPOT'S SIZE IS A FUNCTION OF THE POSITION, AND THE AUDIT
MEASURED IT WHERE THE ENGINE DOES NOT PLAY.** A-04's growth series runs to 4 097
stones. Rule 3 and the arena's 40-turn cap put a real game at **79**. At 79 the
optimisation measures **0.985**. A perf finding taken on a pathological position
is a finding about pathological positions, and the arc's standing rule from here
is that a package states the STONE COUNT its receipt was taken at and whether a
game reaches it.

---

## §P3 — codegen flags (D-577, D-14). LANDED.

| step | state | receipt |
|---|---|---|
| premise: the float hazard | ABSENT and checked — one file on no search path carries `f32`/`f64` | `p3_registration.md` |
| expectation, acceptance, abort, registered BEFORE the runs | 1.05–1.20 for LTO, 1.00–1.08 for `cgu1`; winner needs both bands AND byte-identity; abort if none reaches 1.02 | `p3_registration.md` |
| three variants benched, 5 reps, idle, node identity holding | `cgu1` **0.992 / 0.979** (a LOSS, below its own registered band), **`thin` 1.068 / 1.064**, `fat` 1.030 / 1.022 | `artifacts/p3_bench_{cgu1,thin,fat}_v1.txt` |
| identity leg on the winner | **IDENTICAL**, 128 bestmove / 0 error per side, three seats | `artifacts/p3_identity_thin_v1.txt` |
| **VERDICT** | **`lto = "thin"`, `codegen-units = 1` ACCEPTED and landed** | `Cargo.toml` |
| new pinned digest | `619b81c9…`, bit for bit the benched `thin` binary, superseding `1413698a…` | that binary |

**F-P3.1 — THE TWO DIALS ARE NOT INDEPENDENT AND THE AGGRESSIVE SETTING IS NOT
THE FAST ONE.** `codegen-units = 1` alone LOSES (0.992 / 0.979): one unit costs
more in worse in-crate inlining than it buys from whole-crate visibility. It only
becomes a win with thin LTO on top, and `fat` — the most aggressive setting
available — gives half the win back. **A package that had reasoned rather than
measured would have registered `fat` as the obvious best and banked 1.03 instead
of 1.07**, or registered `cgu1` as a free win and shipped a regression. D-14
deferred this decision *"until there is a bench to judge it by"* precisely
because the answer is not derivable from the flags' descriptions.

---

## TRANCHE 1 CLOSURE — the combined factor, MEASURED end to end, both terms named

`tools/bench_delta.sh rev:ffc5c10 rev:a517ae31bfdcd5ddd631b04d41b7dbef7f4d65a5 5`,
idle receipts both ends, node identity holding per position at both budgets in
all five reps, exit 0 (`artifacts/tranche1_combined_v1.txt`):

| | early | late |
|---|---|---|
| **tranche 1 combined, arc III's close to tranche 1's end** | **1.294** | **1.293** |
| P1, the threat state's store | 1.243 | 1.217 |
| P3, codegen | 1.068 | 1.064 |
| P2, the legality probe | not landed — 0.994 / 0.995 | |

**The combined number is measured, not composed**, which is what the dispatch
asked for and why: multiplying the two landed terms gives 1.327 early and 1.295
late, and the engine actually delivers **1.294 / 1.293**. The early band's
product overstates by 0.033, because the two changes share a hot path and the
second cannot re-win what the first already took. **CI at the landed commit:
twenty gate lines, `gate 1/20` through `gate 20/20`, `ci: all gates passed`,
`CI_EXIT=0`** (`artifacts/t1_ci_a517ae3.txt`).

**WHAT TRANCHE 1 COST AND WHAT IT BOUGHT.** Three packages, two landed, one
refused on its own measurement. Every one of the three produced a finding that
reading the audit could not have: P1's stated premise was worth under 3 % and
the lever was elsewhere; P2's hotspot was attributed to the wrong caller and its
remedy loses at every size a game reaches; P3's most aggressive setting is not
its fastest and its cheapest is a regression. All three were byte-identical
changes judged by proof and a clock, which is the last time in this arc that
will be true.

### ENVIRONMENT NOTES FOR §P1

- Measurement worktree `/home/tom/pistol-wt/p1-measure`, detached, own
  `CARGO_TARGET_DIR`; its untracked build logs and counter outputs are copied to
  `/home/tom/pistol-wt/p1-patches/` and the artifacts above. Removed only after
  the closure's export receipt (D-469).
- `tools/bench_delta.sh` prints its own `VERDICT` against D-220's `[1.4, 2.5]` /
  `1.15` thresholds; those are not this package's numbers (the prereg registers
  its own), and the verdict wording is quoted beside the ratios, never read as
  the package's verdict.

## TRANCHE 2 — the instruments. BOTH LANDED.

### §I1 — width histogram. LANDED at `792d018`.

`WidthHistogram` records emitted-set sizes per row class, plus Tier T's own
union and the quiet ball kept apart (a cap on one is not a cap on the other).
129 buckets: the first run at 34 put **42 %** of BATCHED nodes in the overflow,
and a cap cannot be chosen from a distribution whose largest bucket is
"everything above the top". Armed per ask by the `widths` budget token, so a
search that is not measuring fills nothing. Byte-identical when off.

**What it measured, which redirected W1**: BATCHED emitted mean 17.1 / median
12; Tier T's own union mean 13.0 / median **12**; the quiet ball mean 76.2 /
median **76**, on 1187 of 21746 BATCHED nodes. See `w1_calibration.md`.

### §I2 — the budget-overrun VOID class. LANDED at `6602706`.

`End::Void { nodes, budget }` and `Answer::Void`: an answer that spent more than
`FIRST_ITERATION_MULTIPLE = 4` times its node budget is VOID, not a loss. Void
games are skipped by `pair_buckets` and `pairs_without_forfeits`, so a broken
seat cannot be scored as a weak one. The multiple is MEASURED, not chosen: over
60 openings the first iteration's share of budget had median **0.090**, p90
0.1527 and max **0.1754**, so 4x sits well clear of the honest maximum while
still catching an order-of-magnitude overrun.

## TRANCHE 3 — width. BOTH GATED OFF ON A MEASURED NULL.

### §W1 — Tier-T width cap. `tier_t_top_k = 0` in all fifteen documents.

The cap was calibrated by a rule written down BEFORE the histogram was read: no
K below the measured median, because a cap below the median tests a different
engine. That gave K = 16. At K = 16 the cap BINDS on **23.3 %** of BATCHED nodes
and removes **11.2 %** of Tier-T cells, and changes the chosen move on **0 of
24** governed openings — so the SPRT was not run, because there is nothing for
it to see. K = 8 would change 16.7 % and moving to it after seeing the table is
the post-hoc threshold move D-374 forbids. Full table in `w1_calibration.md`.

### §W2 — root re-ordering. `root_reorder = false`.

Re-sort the root's candidates by the previous iteration's scores, below the
table move's promotion. **0 of 24** openings change at the governed budget
(~2 turns) and **0 of 12** at `nodes 400000` (~4 turns); throughput unchanged
(758 417 → 758 870 nps). The table move already leads, and at this reach the
order of what follows it does not decide anything. `w2_finding.md`.

## TRANCHE 4 — the pruning family.

### §S1 — aspiration windows. `aspiration_delta = 0`.

Each iteration after the first opens (previous − delta, previous + delta) and
re-searches on a fail. **Every width changes 0 of 16 openings AND every width is
slower**: 25 → 0.815, 50 → 0.508, 100 → 0.520, 200 → 0.561 of the full-window
time. A narrow window at this reach fails high or low often enough that the
re-searches cost more than the narrowing saves. `s1_finding.md`.

### §S2 — one-cell forced-reply extension. `extension_budget = 0`. SPRT h0.

The first package of tranches 3 and 4 to reach a match, and it lost it: **8 W /
61 L**, LLR pair −2.956 against −2.9444, n = 90 of a 600-pair cap, normalized
Elo −391.5. On the same nodes the extended seat reached **5 turns** where the
committed seat reached **6**. `s2_registration.md`, D-605.

**S2 also corrected the arc's own method** — see the closure's §1.

### FINDINGS RAISED IN §S2 / §S3

**F-S2.1 — `CARGO_TARGET_DIR` was exported around `cargo test`, in a session
whose own memory file names the failure and its count.** The verification
wrapper set `CARGO_TARGET_DIR=<worktree>/target` for the whole script, so
`cargo test` inherited it, so the scratch cargo workspaces
`solver_link_check_tests` builds shared one target directory and read each
other's `dep-info`. **8 of that suite's 19 tests failed and cargo stopped there**,
40 suites in — which reads as a broken commit and is not one. A worktree already
isolates `target/`; the variable adds nothing and breaks this. **Practice
changed**: the export is set for `cargo build` only, and every verification run
uses `--no-fail-fast` so one bad suite cannot hide the other 130.

**F-S2.2 — `pkill -f <pattern>` killed the issuing shell.** The harness runs each
command inside a `bash -c` whose command line CONTAINS the pattern text, so
`pkill -f s2_verify.sh` matched the pkill's own process and the shell died with
exit 144, silently taking the `setsid nohup` launch in the same command with it.
The SPRT appeared to have been launched and had not been. **Practice changed**:
bracket the first character (`pkill -f '[s]2_verify.sh'`), and confirm a detached
launch by `pgrep`-ing for the child rather than by the launcher's exit code.

**F-S2.3 — a budget granted on the way down was not given back on the abort
path.** The first draft restored the per-line extension budget after the child
loop only, so the `if self.aborted { return 0 }` path inside the loop leaked one
grant per abort. Unobservable in this package (an abort ends the search), but the
fix is two lines and the leak is not a thing to reason about twice. Found by
reading for early returns between the grant and the restore, which is the check
any borrow-and-return of shared state owes.

**F-S2.4 — a budget token that does not exist produced silent nothing.** A
fixed-depth comparison was written `go depth 3`; the protocol's budgets are
`depth_turns`, `nodes`, `movetime`. The engine refused each line correctly and by
name (hard rule 3 working), but the comparison harness grepped for `^bestmove`,
found none on either side, and reported **"0 of 0 differing"** — which reads as
a clean identity result. **Practice changed**: every comparison run counts
`^error` lines and its own denominator, and a denominator of zero is a failed
run, never a null result.

### §S3 — late move reductions. `lmr_min_depth_turns = 0`. SPRT UNDECIDED.

Ran the whole 600-pair cap without the LLR reaching either bound: n = 1200,
distinct-n 1200, `LLR pair -1.831`, normalized Elo −13.4, and **both seats
reached 6 turns** on nearly equal nodes. UNDECIDED is not a pass.
`s3_registration.md`, D-607.

## THE COMBINATION — the question the arc never asked until closure

Tranche 1's two perf packages WERE measured combined (1.294 / 1.293 against
1.327 predicted). The five behaviour-changing ones never were. W1 + W2 + S3
together reach **8 turns where the committed engine reaches 6, on 5.6 % fewer
nodes**, and lose **315 W / 391 L**, normalized Elo −34.2, h0 at n = 910. The
mechanisms compose and do what they were built to do; the extra depth is worth
negative Elo at eval v0. `opt_arc_combination_finding.md`, D-609.

## WHERE PERF STANDS, and the gap that names the next package

`opt_arc_perf_finding.md`, receipt `artifacts/opt_arc_perf_sweep_v1.txt`:
**1.28–1.40x faster for identical node counts**, and a **median depth of 2 turns
at the 0.5 s deployment budget** against sealbot's **5 turns at 0.3 s** on
~864 000 nps to this engine's ~488 000. Depth is logarithmic in nodes and the
branching factor is large, so a third more nodes moved the median depth only at
0.1 s.

## THE ANCHOR, and the reader that made it mean something

Anchor v4 on the platform's single opening read **0 W / 100 L** with an honest
denominator of 2 (D-606). The openings reader — designed, red-teamed three
rounds, never built — was built here, and anchor v5 over **50 paired openings**
reads **40 W / 60 L**, 100 distinct games (D-608). *(unequal movetime: pistol 500 ms, sealbot 300 ms — D-697, recorded after this document)* Building it exposed three
harness defects a single opening hides, including a `distinct_games` that
ignored the opening, and four of the reader's own tests that were vacuous
because the body digest masked every refusal they meant to prove.
