# REVIEW — `docs/experiments/wp16_sprt_prereg.md` revision 4

**Revision reviewed**: `20f9b26bdbd85044abd8c46a9d937934c1a850e9`
(`20f9b26`, "docs(experiments): wp16_sprt_prereg.md revision 4 -- a fresh
opening slice").
**Still matches HEAD?** YES. `git rev-parse HEAD` was `20f9b26bdb…` at the
start of this review and `20f9b26bdb…` at the end; `git status --porcelain`
was empty at both points (this report is the only file this review creates).
**Reviewer**: fresh-context subagent, Claude Opus 5 (1M context).
**Date**: 2026-08-24.
**Scope**: revisions 3+4's amendments (§3, §5, §7, §7A.1, §8/§8.6, §9, §10,
§11), plus the two duties D-415 Ruling 1 added — re-run the mutation table at
the governing revision, and check the amended attribution semantics word by
word against Criterion 1''.

---

## VERDICT: **FAIL** — 1 BLOCKING, 2 MAJOR, 5 MINOR

The governed run must not be launched at this revision.

The BLOCKING finding is exactly the one D-415 Ruling 1(b) commissioned this
review to look for, and it is the same defect class three design rounds and one
implementation round died on — a document and its instrument describing
different things. **The direction is the safe one** (the shipped instrument is
*stricter* than the document, so no false PASS is reachable), which is why this
is a document defect and not an instrument defect. Nothing in the
implementation was found wrong: **all six mutations reproduce, all four artefact
digests reproduce, both dry-run criteria reproduce line for line, and the
`--release --locked` binaries at `bfdf933` hash to the two digests §7A.1
registers.**

---

## Findings

### BLOCKING 1 — §7A.1 registers a criterion the shipped instrument no longer implements: the entire PREMISE CHECK, the `status` cross-check, the halt invariant and the inert cross-check are absent from the document

D-415 Ruling 1(b) names five things this review must check against Criterion
1'' word by word: **the book precondition, the seating check, the opening
check, the `status` cross-check and the halt invariant**. Four of the five
appear nowhere in the pre-registration, and the fifth (the book precondition)
appears only as an unstated assumption inside a proof.

`clause_b` (`tools/wp16_warm_attribution_check.py:581-658`) performs three
refusals before it uses the proof at all — each an exit-2 VOID:

| Check | Code | In §7A.1? |
|---|---|---|
| the pair's two games declare the same `opening` | `if first["opening"] != second["opening"]: die(...)` | **NO** |
| the two games seat one label as p1 where the other seats it as p2 | `if first["p1"] != second["p2"] or first["p2"] != second["p1"]: die(...)` | **NO** — asserted as fact ("the seat credited in game two — the OTHER label"), never as a condition |
| the two games share their first `opening_turns` moves | `if one[:book] != two[:book]: die(...)` | **NO** — the phrase "book prefix" occurs nowhere in the document |

`check_coverage` (`:430-527`) performs two more the document does not carry:

| Check | Code | In §7A.1 / §8.6? |
|---|---|---|
| `status` must be `clean`/`divergence` **by name** and must agree with the divergence-record set | `:455-463` | **NO** |
| a divergent record's halt invariant `replayed_turns == at_turn - 1`, `compared_turns == at_turn - opening_turns`; a clean record's derived `compared_turns` | `:464-495` | **NO** — §8.6's W-1 registers node equality and nothing else |

And `main()` runs `cross_check` (`:662-703`, called at `:794`) — the adversarial reassignment
over the inert pairs, whose disagreement is a **FAILURE** (exit 1). §7A.1
never mentions it. It is the exact sentence §7A.1's verbatim quote of Criterion
1'' **truncates without an ellipsis**: the design's point 4 continues "The old
clause (b)'s adversarial-reassignment machinery is KEPT, but only as a
cross-check run over the INERT pairs alone … its result is cited in the report
as confirming evidence".

Separately, §7A.1's three-bullet enumeration is **not exhaustive**: a pair whose
two move lists are IDENTICAL *and* one of whose games forfeited matches none of
the three bullets (bullet 1 requires "neither forfeited"; bullets 2 and 3
require "they differ"). The code routes that case to `unattributable` — a clause
(b) FAILURE — and it is precisely the case D-413 MAJOR 3 / D-414 MAJOR 3 exist
about.

**Why this matters rather than being pedantry.** The pre-registration is what
the governed run is READ THROUGH. On a premise failure the instrument exits 2,
and §7A.1's exit-2 row registers "**Not a finding, and not evidence about any
engine** … The void is fixed and the answer re-taken." That is the wrong
reading for these three refusals: a pair whose games declare different openings,
or do not swap seats, or diverge inside the book, is a fact about the REPORT —
there is no void to "fix" and nothing to "re-take". A reader following the
document would look for a broken checker.

**Minimal reproducer** (the document side):

```
$ grep -n -i "premise\|book prefix\|cross-check\|halt" docs/experiments/wp16_sprt_prereg.md
770:  premise can be false while every visible number still looks plausible.
819:| **W-1** | **MET.** `W coverage: … 0 halted at a divergence`; …
```

Two hits, neither of them the thing: line 770 is §8.6 using "premise" of
Criterion 1'' as a whole ("Criterion 1'''s whole premise can be false"), and
line 819 is the word "halted" inside a quoted output string. **"book prefix",
"cross-check" and "halt invariant" occur nowhere in the document**, and
`opening_turns` occurs exactly once — in §4's report-format listing (line 232),
never in a criterion.

(the code side, showing the checks are real and load-bearing, is mutation M1
and M2 in the table below: deleting either kills a named test.)

**What would close it**: §7A.1 states the premise as a CHECKED condition with
its own registered consequence, distinguished from the generic exit-2 row;
§7A.1 or §8.6 registers the `status` cross-check and the halt invariant
alongside node equality; §7A.1 names the inert cross-check; and the bullet
enumeration covers identical-and-forfeited.

---

### MAJOR 2 — exit 3's registered consequence blames the ENGINE for a failure the instrument itself says it cannot attribute, and D-413 MEASURED an instrument defect landing there

§7A.1's exit table and §5's outcome table (both amended by revision 3) read:

> | 3 | DETERMINISM VIOLATION | **A HARD STOP BIGGER THAN THIS WP.** The engine's own instrument-mode guarantee (CLAUDE.md rule 4) **is failing**. …

The shipped instrument says the opposite, in the message it prints on that exit
(`tools/wp16_warm_attribution_check.py:172-183`):

> "this has TWO possible causes and this instrument cannot tell them apart: the
> ENGINE's own instrument-mode guarantee failing (CLAUDE.md rule 4), or the
> REPLAY not reproducing the sequence the run actually played. Naming only the
> first would send a reader hunting a defect in the subject when the instrument
> is what broke"

That wording is D-414's fix for D-413 MINOR 7. The document did not receive it.

This is not theoretical. Exit 3 is reached from `check_coverage`'s **node-count
referent** — a pure instrument signal:

```
$ sed -n '596,620p' crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs
fn a_clean_game_that_spent_different_nodes_replaying_is_a_determinism_violation() {
    …
    let tampered = honest.replacen("nodes_a 10", "nodes_a 11", 1);
    …
        Some(3),
        "equal moves at unequal cost takes the DETERMINISM-VIOLATION exit and no other. …"
```

and D-413 records the reviewer building a **cold-replay mutant of `arena
--replay`** which "reports `0 divergence(s)` and exit 0, and only the node-count
referent catches it, **at exit 3**". A cold replay is an instrument defect, not
an engine defect. Under the document as written, that would be recorded as the
engine's rule-4 guarantee failing — a false and serious claim about the subject,
registered in advance.

**Minimal reproducer**:

```
$ grep -n "instrument-mode guarantee (CLAUDE.md rule 4) is failing" \
    docs/experiments/wp16_sprt_prereg.md
289:| **A DETERMINISM VIOLATION — exit 3** | **A hard stop bigger than this WP**: the engine's own instrument-mode guarantee (CLAUDE.md rule 4) is failing. …   [§5]
449:| 3 | DETERMINISM VIOLATION | **A HARD STOP BIGGER THAN THIS WP.** The engine's own instrument-mode guarantee (CLAUDE.md rule 4) is failing. …   [§7A.1]
$ sed -n '172,183p' tools/wp16_warm_attribution_check.py     # the instrument's own words
```

---

### MAJOR 3 — §11 REVIEW STATE and §7A.1's provenance are silent that the registered instrument FAILED its own REVIEW-impl and that its second review was WAIVED

§7A.1 says warm replay "has now been BUILT (**D-407 through D-412**)". The
sequence does not stop at D-412:

- **D-413**: the fresh-context REVIEW-impl **FAILED** `1d1322d` — 1 BLOCKING,
  5 MAJOR, 7 MINOR. The BLOCKING is the very defect whose fix this review was
  commissioned to check.
- **D-414**: one licensed fix round closed all thirteen.
- **D-415 Ruling 1**: **no second full review round** — the mutation table
  stands in for it, and two duties were pushed onto *this* review instead.

The pre-registration cites **D-413 exactly once**, in §8.6's cost paragraph,
and only for a wall-clock digit ("`0.994x` by the fresh-context reviewer of the
implementation"). **D-414 and D-415 are never cited anywhere in the document.**
§11 REVIEW STATE covers the document's own review history in careful detail —
revision 1's FAIL, revision 2's PASS, revision 3's withdrawn review — and says
nothing about the instrument's.

CLAUDE.md makes the instrument's governing revision part of the
pre-registration ("THE INSTRUMENT HAS A GOVERNING REVISION TOO … Without this,
a run stands on an instrument whose own review had failed and is licensed by
argument rather than by this text"). A reader of §11 today would conclude the
registered instrument passed its review. It did not.

**Minimal reproducer**:

```
$ grep -c "D-414\|D-415" docs/experiments/wp16_sprt_prereg.md
0
$ grep -n "D-413" docs/experiments/wp16_sprt_prereg.md
827:on its own machine state (`docs/decisions.md` D-413). Three samples straddling
```

---

### MINOR 4 — the header's account of what revisions 3 and 4 changed is wrong in two places

The header states: "§1, §2, §4, §6, §7A.2 and §8.1 through §8.5 are UNTOUCHED;
**§3 is touched in that one row and nowhere else**".

Both halves are false:

- **§8.5 was touched.** `git diff 731150a..20f9b26` hunk `@@ -586,6 +723,118 @@`
  appends a four-line paragraph ("**§8.1 through §8.5 are revision 2's,
  unchanged…**") *inside* §8.5, before the `### 8.6` header. §1, §2, §4, §6 and
  §7A.2 are genuinely untouched (verified: no hunk falls in their line ranges;
  the `### 7A.2` header appears only as diff context).
- **§3 was touched in two rows, not one.** The same hunk that changes
  `openings_skip` also appends a sentence to the `openings_take` row
  ("UNCHANGED by revision 4, so §2's floor derivation is untouched: …").

Neither addition is substantive, which is why this is MINOR — but the dispatch's
own rule is that a section the document says it left alone but did not is a
finding.

**Reproducer**: `git diff 731150a..20f9b26 -- docs/experiments/wp16_sprt_prereg.md`
(hunk headers `@@ -155,8 +195,8 @@` and `@@ -586,6 +723,118 @@`).

---

### MINOR 5 — the Criterion 1'' quotation's provenance sentence is wrong, and the quote is silently truncated

§7A.1 introduces the quote as "**revision 2's text**, which that document's own
round-2 review verified clean in Part 1 and **which revision 3 carried
unchanged**".

The quote is verbatim against the design document at HEAD (`b6afd66`,
revision 3) but **NOT** against design revision 2 (`d9a2852`) — revision 3
reworded the cross-references `§4 point 3's` → `point 3's` twice inside the
quoted sentence:

```
$ python3 -c "…"   # normalized substring test, both revisions
VERBATIM SUBSTRING OF DESIGN (b6afd66): True
IN DESIGN REV2 (d9a2852): False
--- rev2
+++ rev3
-   NON-INERT pair (§4 point 3's exclusion, forfeits always non-inert) is
+   NON-INERT pair (point 3's exclusion, forfeits always non-inert) is
```

The change is a pure cross-reference reword and the criterion's meaning is
identical, so the *criterion* is sound; the *provenance sentence* is not. (The
"round-2 review verified clean in Part 1" half IS accurate — D-404 records Part
1 passing clean and both failures landing in the Option Matrix.)

The truncation is folded into BLOCKING 1 above: the quote ends mid-point-4,
with no ellipsis, dropping the sentence that keeps the inert cross-check.

---

### MINOR 6 — §8.6's two wall-clock figures cannot be sourced to the artefacts §8.6 pins by digest

§8.6 records `14.409 s` for the run and `14.368 s` for the replay, and computes
`14.368 / 14.409 = 0.997x`. The four artefacts it names hash exactly as
recorded (verified below), and their own `timing` lines say something else:

```
$ sha256sum artifacts/wp16_warmreplay_dryrun_run.txt
6e2a531c8e346b23a661fd96abef15f847e7c6f60cc0d8ac4a8813e7e007c793   # matches §8.6
$ grep wall_ms artifacts/wp16_warmreplay_dryrun_run.txt
timing n_workers 4 wall_ms 14341 discarded_in_flight 0 hang_timeout_ms 120000
$ grep wall_ms artifacts/wp16_warmreplay_dryrun_replay.txt
timing n_workers 4 wall_ms 14305 hang_timeout_ms 120000
```

14.341 s and 14.305 s, not 14.409 and 14.368 (a 68 ms / 63 ms gap, consistent
with an external `time` around the whole process, but the document does not say
so). The registered *ratio* is unaffected — 14305/14341 = 0.9975, still
`0.997x` — and the document explicitly disclaims the third digit, so this is
provenance hygiene rather than a wrong number.

---

### MINOR 7 — §7's cost table is stale and incomplete for the run it now governs

- The "Operator/session attention" row still reads "one launch, one report
  read, **one Criterion 1' run**" — Criterion 1' is the retired instrument;
  §7A.1 replaced it with 1''.
- The row omits the SECOND INSTRUMENT run and §7A.2's 24-position sweep, both
  of which §7A.1/§7A.2 require **at Step 6**.
- **§7A.2's instrument has no cost row at all.** CLAUDE.md requires a
  pre-registration to state what its governed run COSTS "so the proportion
  between the document and the run is visible on the document's own face"; the
  Doubt-2 sweep is part of that run and is unpriced.
- The second-instrument row's "minutes at `openings_take = 500`" is an
  unmarked extrapolation, though it sits under a MEASURED heading and the
  arithmetic (6.485 s / 4 pairs × 500 ≈ 13.5 min) is transparent.

Every other numeric claim in §7 and §8.6 IS marked MEASURED or ESTIMATED, and I
found no estimate that could have been measured in seconds and was not.

---

### MINOR 8 — instrument-revision bookkeeping in §10 is looser than §10 claims

§10 says the instruments are "**Named here with the governing revisions §7A.1
pins them at**". Two of the five it lists are not pinned there:

- `tools/wp15b_attribution_check.py` — §7A.1 gives only "at its own current
  revision" (no SHA). §8.2 (revision 2's text) says "the commit this document
  lands at", which resolves to `20f9b26`, so the revision is *derivable*; it is
  not *stated* where §10 says it is. (Independently verified unmodified: last
  touched at `a80a864`, long before this WP.)
- `tools/baseline_snapshot.sh` — pinned at `9282dd0` in **§7A.2**, not §7A.1.

Everything the criterion actually computes on IS pinned: the four arena source
files and `bin/arena.rs --replay` at `bfdf933`, the checker at `bfdf933`, and
the two binaries BY CONTENT. I found nothing the criterion depends on that is
missing from §10 — the binary-by-content pin transitively covers `exchange.rs`,
`channel.rs`, `game.rs`, `identity.rs` and `handshake.rs`.

---

## Attacks ATTEMPTED AND REJECTED (recorded with the reproducer, per CLAUDE.md)

### The proof in §7A.1 bullet 2 — I could not break it

Its load-bearing step is "that index's occupant searched exactly the same
prefixes in both games, so its warm table is in the same state in both". I
attacked it six ways:

1. **Different game lengths / one game ending before `t`.** `t` is the first
   index at which the two lists differ, taken over `min(len(one), len(two))`, so
   `t` is inside both games by construction. A prefix relation yields
   `witness is None` → the third bullet's named FAILURE. Not a counterexample.
2. **`t` inside the opening book** (nobody searched there, so there is no
   replay evidence at `t`). This is the real hole — and it is exactly what the
   book precondition closes. `clause_b` refuses such a pair by name (exit 2)
   and carries an explicit `unreachable` assert behind it. The *document* does
   not state the precondition (BLOCKING 1), but the *proof* is not falsifiable
   through this route against the shipped code.
3. **An asymmetric opening book.** The premise check compares the two games'
   `opening` field and their first `opening_turns` moves. Not reachable.
4. **A capped game.** Two capped games with identical lists are inert and forced
   to `p2` by `score_a`'s unconditional 0.5-both scoring, independently of the
   theorem; two capped games that differ have a `t` like any other pair.
5. **A forfeit.** A forfeited pair is excluded from inertness outright and its
   `t`, if any, is attributed normally; with no `t` it is the named FAILURE.
6. **The replay halted before `t`** (so no evidence at `t`). If any divergence
   exists, `main()` never calls `clause_b` at all — `classify()` runs first and
   resolves every divergence to either a CONFIRMED INVERSION (clause (a) fails,
   exit 1) or a determinism violation (exit 3). The clean-report premise is
   enforced by control flow, not assumed.

The label-inversion contradiction itself holds in both sub-cases (both games
inverted; exactly one game inverted): each forces one engine to answer both
`m1` and `m2` to the same position with the same query history. **Rejected — no
counterexample found.**

### The agreement criterion's false-disagreement mode — rejected

I attacked the registered agreement criterion ("for every game the cold checker
attributes by a discriminating replayed turn, the warm pass must record `status
clean`") on the ground that `wp15b`'s link 1a replays turns
`opening_turns` and `opening_turns + 1` COLD, and D-383 measured cold replay
disagreeing with a warm engine past its first search — which would produce
disagreements that are artefacts of the retired instrument, not defects.

**The attack fails.** Those two turns have *different movers*
(`tools/wp15b_attribution_check.py:257-259`: `mover = game["p1"] if free % 2 ==
0 else game["p2"]`), so each is the corresponding engine's FIRST search of the
game, where cold and warm coincide. That is precisely why the window could
never be widened, and why the criterion is safe. Verified empirically on the
dry-run report: `1a: 16 turns replayed, 10 of them discriminating, 8 of 8 games
directly attributed`, and all 8 warm records `status clean` — the criterion
HOLDS.

### CLAUDE.md's second-instrument requirements, checked off BY NAME

| Requirement (CLAUDE.md Process) | Where §7A.1 satisfies it | Verdict |
|---|---|---|
| agreement criterion **registered before either runs** | §7A.1 bullet 3; the document is UNREVIEWED and no governed run exists | MET |
| a **REGISTERED CONSEQUENCE** of disagreement | §7A.1 bullet 4, and §5's own row | MET |
| **NAMES THE STAGE UNDER DOUBT** | §7A.1 opening: "everything between the two engine processes and the printed verdict — the arena's seat bookkeeping, its pairing, its referee and its scoring" | MET |
| **says how the second instrument does not share it** | "THE STAGE IT DOES NOT SHARE … the WARM DRIVE" — the cold checker spawns one fresh process per query and never drives a game | MET |
| not "two instruments blind to the same stage" | "WHAT THEY ARE BOTH BLIND TO … the report WRITER"; agreement declared to be "evidence about the drive, not about the writer" | MET, and unusually honest |
| link 1b/1c excluded as "agreeing with themselves" | stated explicitly | MET |
| run COSTS stated on the document's face | §7 table — but see MINOR 7 | PARTIAL |

One operational softness, recorded rather than raised as a finding:
`wp15b_attribution_check.py` emits no per-game "confirmed inversion" token — it
prints `1a game <i> turn <t>: …` failure lines — so the second half of the
agreement criterion is mapped by hand. CLAUDE.md says these criteria are judged,
not mechanized, so this is acceptable as written.

### §8.6's dry-run rules, checked off BY NAME

| Rule (CLAUDE.md) | §8.6 | Verdict |
|---|---|---|
| input of the **SAME KIND**, differing only in identity | `configs/arena_wp16_dryrun.toml` — an arena config; wider trigger arm, `openings_v1.txt`, 4 openings | MET |
| **never the registered workload itself** | stated, and true (different book, different engine A) | MET |
| **records the dry-run input** | named, with the artefact digests | MET |
| **and its output** | the W-1/W-2 table, quoted line for line | MET — and reproduced below |
| **AND WHAT THAT OUTPUT MUST SHOW** | W-1 and W-2, each stated before the result | MET |
| **together with the DEFECT CLASS the criterion excludes** | W-1: "a replay that is not actually warm"; W-2: "an attribution criterion that cannot see a seat swap" | MET |
| **not a property the defect class PRESERVES** | see below | MET |
| **commands exercised at the GOVERNING revision** | see below | MET |

**W-1's non-vacuity — attacked and upheld.** The document rests it on the node
counts being an EXTERNALLY DERIVED REFERENT. That is true of the stage under
doubt: the report's `nodes_a`/`nodes_b` were folded by the GENERATION run and
written before the replay existed; the replay's are freshly accumulated
(`crates/pistol-arena/src/replay.rs:190` — `nodes: [compute[0].nodes,
compute[1].nodes]`, where `compute` is mutated by `ask` during the replay walk,
not copied from the report). The document also says the right thing about the
half that would be vacuous alone: "**`0 divergence(s)` ALONE WOULD NOT BE A
CRITERION**". D-413 independently MEASURED this — a cold-replay mutant reported
`0 divergence(s)` and exit 0, and only the node referent caught it. Residual,
recorded: both counts pass through the same `exchange::ask` folding code, so a
folding defect would corrupt both identically — but that is not the defect class
W-1 names, and §8.6 does not over-claim.

**W-2's non-vacuity — upheld.** "An instrument that passes everything fails
W-2; an instrument that refuses everything fails W-1" is a correct
discrimination argument, and W-2 is a real seeded defect on real documents (the
honest run's own bytes with `p1`/`p2` transposed).

**Taken at the governing revision — VERIFIED, and this was the item most worth
checking, because §8.6 claims to have applied that very rule to itself.** A
fresh worktree at `bfdf933` outside the repo, built `--release --locked`:

```
$ git worktree add /home/tom/pistol-verify/wt-bfdf933 bfdf933
$ cd /home/tom/pistol-verify/wt-bfdf933 && cargo build --release --locked
    Finished `release` profile [optimized] target(s)
$ sha256sum target/release/arena target/release/pistol
3ba8de615d4d708793d72c2f3c2f6c649811996bb331527e64d0f612a13aebc2  target/release/arena
b8d0dc963a2453e1eff69823629c37b23bafe419b9225f8af2401df519bc2673  target/release/pistol
```

Both match §7A.1 item 3 **exactly**. (`CARGO_TARGET_DIR` was unset, no
`sccache`, no `.cargo/config.toml` in the worktree or `$HOME` — a genuinely
independent build tree.)

**Does §3/§9/§11 moving in revision 4 disturb the instrument revision?** No, and
I checked rather than assumed: `git diff --stat bfdf933..20f9b26` is
`docs/experiments/wp16_sprt_prereg.md | 345 +++…` and nothing else. No
instrument file — source, tool, config or fixture — changed between the pinned
`bfdf933` and the governing `20f9b26`, so the binaries built at `bfdf933`
remain the binaries a build at `20f9b26` produces. §8.6's own reasoning ("a
Rust binary's bytes move when its source does") is correct and does not bite
here.

### §8.6's recorded results — independently re-executed, and they reproduce

All four artefacts hash exactly as §8.6 records:

```
$ sha256sum artifacts/wp16_warmreplay_dryrun_*.txt
cf91e3fa9484d1ffcd7e0573ef2f349452e8065fa14c5f45d9214d1e31ad6170  …_replay.txt
6e2a531c8e346b23a661fd96abef15f847e7c6f60cc0d8ac4a8813e7e007c793  …_run.txt
b63395e2b8c2d6f1d467920b6edcf5e167626ae07a19e3b252c86925901b4eca  …_swapped_replay.txt
377521bfd08408c395402d37e238ce9bdfeaebe5b26579358f0afb0001595882  …_swapped.txt
```

**W-1, re-run by this reviewer** (checker at `bfdf933`, binaries at `bfdf933`):

```
warm_attribution_check: W coverage: 8 game(s) accounted for — 8 replayed in full with every node count equal to the run's, 0 halted at a divergence. …
warm_attribution_check: W classification: 0 divergence(s), 0 confirmed inversion(s), 0 unexplained
warm_attribution_check: (b): 0 inert pair(s) excluded by theorem, 4 pair(s) directly attributed at their first differing searched turn, 0 unattributable
warm_attribution_check: cross-check: no inert pairs — the exclusion changed nothing
warm_attribution_check: 1b: 5 decided non-forfeit game(s) adjudicated against the move list
warm_attribution_check: 1c: 8 game(s) and 4 pair(s) rebuilt off the score_a path
warm_attribution_check: PASS — 0 failure(s)
exit=0
$ grep -o "compared_turns [0-9]*" …_replay.txt | awk '{s+=$2} END {print s}'
201
```

Every string matches §8.6 verbatim, including the **201 compared turns**.

**W-2, re-run**: `W classification: 8 divergence(s), 8 confirmed inversion(s),
0 unexplained` — every one at turn 5 — `FAIL — 13 failure(s)`, exit 1,
including `1c \`counts wins_a 2\` against 3 rebuilt from the \`game\` lines`.
Matches §8.6 verbatim.

**Second instrument, re-run**: `attribution_check: 1a: 16 turns replayed, 10 of
them discriminating, 8 of 8 games directly attributed by replay`, `PASS — 0
failure(s)`, exit 0, ~6 s wall. Matches. The registered agreement criterion
HOLDS on this input.

**Independent replication of the cost ratio** (a fourth sample, mine):

```
$ target/release/arena --replay …_run.txt --out <scratch>/rereplay.txt --workers 4
arena: replayed 8 of 8 game(s) …, 0 divergence(s)
timing n_workers 4 wall_ms 14380         # 14380/14341 = 1.003x
```

Four samples now straddle 1.0 (0.994, 0.997, 1.003, 1.003). "About one run" is
the right registered figure.

---

## Duty 3 — §3's fresh opening slice (revision 4)

| Claim | Verified? |
|---|---|
| the book holds 2000 openings | **YES**. `grep -c "^[^#]" crates/pistol-cli/tests/fixtures/random_openings_v1.txt` → `2000` |
| `skip 500, take 500` is disjoint from D-401's `skip 0, take 500` | **YES**. `crates/pistol-arena/src/openings.rs:137` — `parsed.drain(skip..skip + take)`; skip is a file-order prefix skip, so `500..1000` ∩ `0..500` = ∅ |
| `skip + take` fits the book | **YES**. 1000 ≤ 2000, and `openings.rs:126-133` makes `skip + take > total` a named refusal citing both keys, never a silent shortening |
| `skip` means what §3 assumes | **YES** — and D-202 is quoted accurately, including "the book is emitted in content-hash order, so any contiguous window is as much a sample as a prefix is" (D-143 confirms the content-hash emission order) |
| the skip enters `experiment_sha256` | **YES**. `crates/pistol-arena/src/report.rs:87` — `writeln!(canonical, "openings_skip {}", config.run.openings_skip)`, with the D-202 comment beside it |
| §2's pair-floor derivation turns on `take`, not on the window | **YES**. §2's derivation solves `n * (t1 * t_hat - t1^2/2) = h1` for a pair COUNT; it contains no reference to the window, the skip, or which openings are drawn. `git diff` shows §2 untouched |
| the header's account of what revisions 3 and 4 changed | **NO** — see MINOR 4 |

One consumer trap worth flagging to whoever reads the report (already documented
in `openings.rs:44-48`, not a finding against this document): `game … opening
<i>` stays WINDOW-relative, so this run's openings will be labelled `0..500`
exactly as D-401's retired run's were. The absolute book position is
`openings_skip + i`. The two runs' reports will look confusingly alike at the
`opening` field and are told apart only by `openings_skip` and
`experiment_sha256`.

---

## Duty 2 — the mutation table, RE-RUN at the governing revision

Separate worktree `/home/tom/pistol-verify/wt-mut` at `20f9b26` (never the live
tree; not under `/tmp`; `CARGO_TARGET_DIR` never exported). Baseline
`cargo test --workspace --locked` at `20f9b26`: **green**, every target `ok`.

| # | Mutation | Test(s) that fail | Exact result | Diagnostic? |
|---|---|---|---|---|
| 1 | delete `clause_b`'s book-prefix precondition | `a_pair_that_does_not_satisfy_the_proofs_premise_is_a_void_and_not_an_attribution` | `test result: FAILED. 10 passed; 1 failed` | **YES** — ``` `book` must refuse by name, saying `inside the 2-turn book`: exit Some(2)`` `` with the instrument's own stdout `CANNOT READ: pair 0: unreachable — the book check above already refused this` |
| 2 | restore the bare `if record["status"] != "clean": continue` in `check_coverage` | `a_replay_record_cannot_skip_its_own_coverage_checks_by_claiming_a_divergence` (+ `documents_that_are_not_about_each_other_are_a_void_and_not_a_finding` under the wider splice) | `FAILED. 10 passed; 1 failed` (narrow splice) / `FAILED. 9 passed; 2 failed` (wider splice — **exactly D-414's transcript**) | **YES** — "a status word that is neither must be a void; exit 0 would be the instrument printing `replayed in full` about a game it never checked" |
| 3 | `if one == two and not forfeited:` → `if one == two:` | `a_forfeit_sibling_of_an_inert_pair_is_not_excluded` | `FAILED. 10 passed; 1 failed` | **YES** — "a forfeit pair nothing tells apart is not a measurement: exit Some(0)" |
| 4 | rename `compared_turns` → `compared_seen` in `replay_report::render`'s format string | `a_real_run_and_its_real_replay_are_attributable_to_the_shipped_checker` **and** `a_transposed_report_is_caught_by_the_shipped_checker_over_real_documents` | `FAILED. 1 passed; 2 failed` | **YES** — `CANNOT READ: a record in one of the documents is malformed: KeyError('compared_turns')` |
| 5 | `replay::walk` does NOT return early on a divergence | `a_transposed_report_is_caught_by_the_shipped_checker_over_real_documents` **and** `a_swapped_seat_label_diverges_at_the_first_differing_turn` | `replay_chain_tests: FAILED. 2 passed; 1 failed`; `replay_tests: FAILED. 3 passed; 1 failed` | **YES** — "game 0: it diverged at turn 5, so 4 turn(s) were fed, and the record says `replayed_turns 11` — the replay did not halt where it says it halted" |
| 6 | delete the `NEW_GAME` send in `seats::with_seats` | `the_replay_path_sends_newgame_on_every_fresh_spawn_too` **and** `every_fresh_spawn_is_sent_newgame_before_it_is_given_a_position` | `replay_tests: FAILED. 3 passed; 1 failed`; `seat_setup_identity_tests: FAILED. 3 passed; 1 failed` — **exactly D-414's two-target claim** | YES for the seat test; the replay test's message dumps the whole report before the assertion text, which is noisy but names the case |

**All six mutants die. No mutation survived. D-414's transcript is
reproduced.** (Mutations 5 and 6 needed `--no-fail-fast` to surface the second
failing target — cargo stops after the first failing test binary, which is why a
single-target count can look smaller than D-414's.)

**The EQUIVALENT-MUTANT claim — CHECKED, and it is correct.** D-414 says
reverting the witness search to "the first difference at or past the book" is
now an equivalent mutant, harmless because the precondition makes the two
searches identical.

*Empirically*: applying it (`range(min(len(one), len(two)))` →
`range(book, min(len(one), len(two)))`) gives
`cargo test --workspace --locked --no-fail-fast` **exit 0, no test failed** —
the mutant survives, as claimed.

*By argument, which is what makes it EQUIVALENT rather than merely uncaught*:
the two searches differ only if some index `i < book` has `one[i] != two[i]`.
The precondition `one[:book] == two[:book]` is checked immediately before, and
Python slice equality entails element-wise agreement on
`range(min(book, len(one), len(two)))` **and** equal slice lengths — so no such
`i` can exist past the precondition. I could construct **no input** where they
differ, including the edge cases `book == 0` (both slices empty, both searches
start at 0) and `len(one) < book` (slice lengths differ → the precondition
already refuses). **Not BLOCKING.**

---

## Is there anything this document claims that I could not verify?

Yes — four things, all recorded rather than assumed:

1. **§8.6's `0.994x` third sample**, attributed to "the fresh-context reviewer
   of the implementation … on its own machine state (D-413)". I cannot
   reproduce another machine's state. My own independent replication gave
   `1.003x`, which is consistent with the registered reading ("about one run")
   and with D-415 Ruling 3.
2. **§8.6's absolute wall figures `14.409 s` / `14.368 s`** — see MINOR 6.
   They are not in the pinned artefacts, and the document does not say what
   measured them. The ratio they yield is reproducible; the figures are not.
3. **§7's ESTIMATE for the governed run** (`~2-3 core-hours, ~35-50 min wall at
   4 workers`). Unverifiable before the run, correctly marked ESTIMATE, and its
   derivation (D-398's node-ratio against D-292's anchor) is stated.
4. **§9.4's "no further amendment to `docs/wp16_quiescence_design.md` between
   D-394 and the run's launch revision"** — this is a Step-6 slot by
   construction and cannot be discharged at review time.

Sections §1, §2, §4, §6, §7A.2 and §8.1–§8.4 were confirmed untouched by
`git diff 731150a..20f9b26` and were NOT re-litigated. §8.5 was touched
(MINOR 4) and its added paragraph was read; it makes no new claim.

Rule 8 was checked and holds: `git status --porcelain` empty, `/artifacts/` in
`.gitignore` (line 19, confirmed by `git check-ignore -v`), and
`tools/artifact_check.sh` reports `artifact_check: ok (424 tracked files, none
of them artifacts)`.

`tools/SHELL_CHECKLIST.md` was NOT applied as a review target: `git diff
--stat bfdf933..20f9b26` shows this revision touches no file under `tools/`.
The tools/ changes it *registers* landed at `bfdf933`, whose own second review
round D-415 Ruling 1 waived — which is MAJOR 3's point, and which the document
should say on its face.
