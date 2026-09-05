# WP-2.2 §B census pre-registration — fresh-context REVIEW, revision 4. Verdict PASS.

**Reviewed**: `docs/experiments/wp22_census_prereg.md`, revision 4 (its first line
states the revision), at named revision `d664368` on `dev`.

**Does it match HEAD?** **Yes.** `git rev-parse HEAD` =
`d6643686a8a3ed47dcbdfc58ea7cf3d7e8cebcb6`; `git status --porcelain` is empty. The
whole tree, instruments included, is byte-identical to the revision under review.

**VERDICT: PASS.** All four MAJOR remedies landed. Nothing the fixes introduced can
change the count. Seven MINOR, one of which (n1) should be fixed before the run is
launched because it is the pin a successor will cite; none of them blocks.

**This is a remedies-only third round, so I applied the stated bar and not a
general-review bar**: a defect that cannot change the number is MINOR, however
irritating. I record below what I tried in order to break the count and could not.

---

## 0. What I ran, and what it consumed

Everything at `d664368`. **No `cargo` was run in the live tree** (D-592); the
mutation work was done on a `git archive HEAD` unpack under the session
scratchpad, never the live tree, per CLAUDE.md's Process section.

**The governed census slice (rows 600 onward) is untouched by this review.** I ran
no search at all: every number below is a re-tally of data already on disk, plus
five seeded mutations of the test suite. Wall cost of the whole review is under two
minutes of compute.

**Marked limit.** Like round 2, my re-tallies are blind to anything
`trigger_census` itself gets wrong; I did not re-run the binary. Round 2 discharged
that with a 4-position replay that came back byte-identical, and I did not
reproduce that check. What I did add that round 2 did not: an **independent
step-by-two parser** run against the shipped one over every census file on disk
(§4.6), which is the one thing that would catch the shipped reader mis-reading a
row.

---

## 1. The four remedies

| # | round-2 finding | verdict | evidence |
|---|---|---|---|
| **M1** | criterion stated two ways; §7 never said "class" | **LANDED** | `/usr/bin/grep -in "class" ` over §7 now returns **two** hits: bullet 1 *"whether the registered minimum (28 CLASSES) is cleared. §2 owns the criterion; this section does not restate it"*, and bullet 5 *"the cumulative CLASS curve"*. The retired terms are gone from §7 — `distinguishable` and `trial` survive only at line 28 (intro, see n5) and line 127 (a quotation of the binomial's assumption). §1 now reads *"counted against D-537's floor of 28 **in the currency §2 registers**"*; §6 is reduced to a pointer. §2's table is the single statement. **No section applies the floor to the key count any more**: every one of the ten occurrences of `28` was inspected (§4.4). |
| **M2** | `roots <= classes <= keys` asserted as a theorem | **LANDED** | §2 line 80 now says *"IS NOT A THEOREM"* and gives the mechanism (`turns` is root-relative). `census_classes.py:62-66` raises `OrderingViolated`. I built a violating census file **by hand from the shipped print format** (not from the test's helper) and drove the **CLI**, not `tally()`: `OrderingViolated: roots 2 <= classes 2 <= keys 1 is false…`, **exit 1, no count printed**. The refusal precedes all output, so a violated run cannot be read. |
| **M3** | the tests were invoked by nothing and self-SKIPPED to a pass | **LANDED** | Gate 18 exists (`tools/ci.sh:189-190`), runs `tools/texel_tests.sh`, which runs the suite. `GATE_TOTAL=21`; the step strings are **1..21, sequential, no gap, no duplicate**, and no gate name is duplicated (§4.3). Hermetic: the suite **passes on a `git archive HEAD` unpack with no `artifacts/` directory at all**, exit 0, and `/usr/bin/grep -n artifacts` over the three files returns only a comment. **Zero occurrences of skip language.** The partition is read from `stage3_allocator_bound.py` itself — proved by mutating the **allocator's** own `COLUMNS` and watching the gate go red (§4.2, mutation M-b). Five seeded mutations, five kills. |
| **M4** | the independence REASON was wrong | **LANDED** | §2 lines 65-78 register the tightening on the larger-minimum permission, then say in terms: *"It is NOT registered on the ground that classes restore the binomial's independence, which the round-2 review refuted by measurement"*, give `6/8 = 0.750`, give **7** root searches, mark the re-solve **ESTIMATED at n = 42-150** (D-291 compliant), and assign it to round 3. Both MEASURED figures reproduce independently (§4.5). The logic of the surviving ground is sound: `classes >= 28` **and** `classes <= keys` gives `keys >= 28`, so the registration is strictly stronger than D-537's literal figure — and the second conjunct is now the thing M2 checks rather than assumes. |

---

## 2. Did the fixes break anything? No.

**The pilot reproduces to the digit through the modified instrument.**

```
$ python3 tools/texel/census_classes.py artifacts/wp22_census_pilot/calibslice_c2048.txt 500
census_classes: 5935 firing(s)
  win-proving distinct KEYS    104   (D-537's literal unit, D-570)
  win-proving distinct ROOTS   36
  win-proving column CLASSES   62   (the alternative's own partition)
  loss-direction distinct keys 91   [separate, never summed]
  per position over n=500: keys 0.208 roots 0.072 classes 0.124
  class curve (positions:classes) — read it for SATURATION, never extrapolate it:
    100:11  200:18  300:37  400:47  500:62
```

62 / 104 / 36, 5 935 firings, curve `11 18 37 47 62`. Identical to revision 3 and to
round 2's independent tally. `OrderingViolated` does not fire.

**The new refusal path cannot fire spuriously on real data.** All six real census
files satisfy the ordering, with room:

| file | firings | roots | classes | keys |
|---|---|---|---|---|
| `wp22_census_pilot/calibslice_c2048.txt` | 5 935 | 36 | 62 | 104 |
| `wp22_cap_dryrun_v2/d3_c2048.txt` | 1 163 | 8 | 26 | 86 |
| `wp22_cap_dryrun_v2/d3_c8192.txt` | 390 | 8 | 14 | 17 |
| `wp22_cap_dryrun_v2/d3_c16384.txt` | 267 | 9 | 15 | 19 |
| `wp22_cap_dryrun_v2/dry2_c2048.txt` | 229 | 2 | 3 | 4 |
| `wp22_cap_dryrun_v2/dry2_moveorder_c2048.txt` | 236 | 2 | 3 | 4 |
| `wp22_cap_dryrun_v2/d3.txt` (zero-firing control) | 0 | 0 | 0 | 0 |

**This is the one place the author departed from round 2's prescribed fix, and I
attacked it.** Round 2 said *"Delete `roots <= classes`, which licenses nothing and
is false"*; revision 4 instead **enforces both halves**. That converts a false
theorem into a refusal condition, so a run where two roots' win-proving firings all
land in one class would be refused although its class count is a perfectly good
tightening. I measured the margin across pilot prefixes:

| n | roots | classes | keys | `classes − roots` |
|---|---|---|---|---|
| 50 | 4 | 4 | 4 | **0** |
| 100 | 7 | 11 | 13 | 4 |
| 200 | 11 | 18 | 22 | 7 |
| 300 | 19 | 37 | 64 | 18 |
| 400 | 26 | 47 | 78 | 21 |
| 500 | 36 | 62 | 104 | **26** |

**The margin widens monotonically in n.** The tightest point is the very start
(equality at n = 50, which passes, since the check is `<=`). At the registered
n = 800 the trend is strongly away from the boundary. The kept half is therefore
harmless at the registered n — and it is strictly conservative in the right
direction: it can only refuse, never over-report. **The departure survives.**

**A zero-firing run does not trip it either** (`0 <= 0 <= 0`), and §9 already
catches that case as VOID AND REPAIR, so the two guards do not collide.

---

## 3. Re-derivation ledger

| # | claim | where | my check | result |
|---|---|---|---|---|
| 1 | pilot: 5 935 firings, 104 keys, 36 roots, 62 classes, 91 loss keys | §5 / §2 | shipped script **and** my independent step-2 parser | **identical on both** |
| 2 | class curve `100:11 200:18 300:37 400:47 500:62` | §5 | re-tally, binned by `entry` | **identical** |
| 3 | floor of 28 crossed between n = 200 and n = 300 | §5 | prefix accumulation | crosses in that interval (18 at 200, 37 at 300) |
| 4 | n = 800 is "more than twice" the crossing point | §5 | round 2's crossing n = 268; 2 × 268 = 536 < 800 | **true** |
| 5 | 44 min at 3.31 s/position | §5 | `RECEIPT.md`: `WALL = 1654 s / 500 = 3.31`; 800 × 3.31 = 2 648 s = 44.1 min | **identical** |
| 6 | "28 classes reachable from as few as **7** root searches" | §2 | my own greedy over the classes-per-root distribution `[7,5,4,4,4,3,3,3,2,2,…]` | **7 roots, union 30** |
| 7 | "86 win-proving keys come from 8 roots distributed `[35,33,11,3,1,1,1,1]`" | §5 | tally of `d3_c2048.txt` | **identical, all four** |
| 8 | "held over 600 positions and **6 366** keys" | §2 | distinct in-tree keys over pilot + dry-run slice | **6 366 exactly**; and **0 keys carry more than one `turns` value**, so the headroom claim holds |
| 9 | requiring 28 classes is strictly larger than requiring 28 keys while `classes <= keys` | §2 | `classes >= 28 ∧ classes <= keys ⟹ keys >= 28` | **valid** |
| 10 | class rates 0.26 / 0.14 / 0.15, spread 1.86x | `matrix_wp22_cap_decision.md` (corrected at `d664368`) | 26/100, 14/100, 15/100; 0.26/0.14 = 1.86 | **identical** — but see n5 |
| 11 | §6b instrument revisions | §6b | `git log --diff-filter=A` and `git log -1` per file | `trigger_census.rs` **0f58533 ✔**, `draw_census_samples.py` **42967e0 ✔** (round-2 m1 landed), `fixture_key_full.rs` **42967e0 ✔**, `census_classes.py` **0097f83 ✘ — last touched `d664368`** (n1) |

---

## 4. What I attacked, in detail

### 4.1 The refusal, driven end to end rather than through the test

I did not accept the seeded test as proof. I wrote a two-row census file by hand in
`trigger_census`'s print format — one key, two roots, two `turns` values, giving
`keys 1 / roots 2 / classes 2` — and ran `census_classes.py` as a program. It exits
**1** with `OrderingViolated` naming all three counts, and **prints nothing before
raising**, so there is no partial output a reader could mistake for a result. The
guard sits in `tally()`, and `main()` calls `tally()` on its first line, so the CLI
— the only reporting path — is covered. `curve()` is unguarded but reports no count.

### 4.2 The gate, proved by seeded breakage

Five mutations on an isolated `git archive` unpack, each run through
`tools/texel_tests.sh`:

| mutation | exit | killed by |
|---|---|---|
| M-a `census_classes.COLUMNS` drops `covers` | 1 | partition test |
| **M-b the ALLOCATOR's own `COLUMNS` renamed** | **1** | **partition test** — this is the one that proves the pin reads `stage3_allocator_bound.py` itself, not a copy; it is exactly round-2 M3 point 3 |
| M-c the `raise OrderingViolated` removed | 1 | ordering test (`no exception raised`) |
| M-d roots counted from `key` instead of `entry` | 1 | uncaught `OrderingViolated` in the hermetic test (loud traceback, not a silent pass) |
| M-e classes not collapsed (key added to the tuple) | 1 | hermetic count test |

**Five for five.** The suite is not decorative.

I also confirmed the void seam: `texel_tests.sh` exits **2** when `python3` is
absent, and `ci.sh`'s `gate()` maps 2 to `void` and everything else non-zero to
`fail`, so the new gate honours SHELL_CHECKLIST item 12 the way the other twenty do.

### 4.3 The renumbering

The class D-423 warns about, checked exhaustively rather than spot-checked:

```
readonly GATE_TOTAL=21
gate 1..21, in order, one per step string, no gap, no duplicate number
```

`bash -n` parses both `ci.sh` and `texel_tests.sh`. No duplicate gate *names*. The
four cheap document gates all still pass at `d664368`:
`file_justification_check` (368 tracked files, 72 over the cap, all registered),
`decision_key_check` (621 keys, no repeat), `label_consistency_check` (6 documents,
all self-consistent), `governing_citation_check` (`REVISION_CITATION_CHECK_DONE`).
`wp22_census_prereg.md` is on gate 21's own `GOVERNING` list, so its `path` and
`revision N` citations are mechanically checked — but **not its commit SHAs**,
which is why n1 below had to be found by hand.

### 4.4 Is the criterion stated exactly once?

I inspected every one of the ten occurrences of `28` in the document. Nine are the
single registered statement (§2), the floor's provenance (§1, §5, both citing
D-537's derivation), the class-curve crossing (§5, correctly about classes), the
flip clause (§5, correctly about classes), the independence critique (§5's *"28
keys are reachable from ONE search tree"*, correctly labelled as keys and not as a
criterion), and §2's own reasoning. **No section adjudicates the key count against
28 any more.** The tenth is §7's parenthetical *"(28 CLASSES)"* — a restatement,
but one that **agrees** with §2 and is immediately followed by *"§2 owns the
criterion; this section does not restate it"*. Round 2's M1 was a *conflict*; what
remains is a pointer that names its referent. Not a finding.

### 4.5 The M4 ground, attacked

The surviving ground is *"requiring 28 classes is strictly larger than requiring 28
keys whenever `classes <= keys` holds"*. I tried three ways to break it:

- **Is the implication valid?** Yes, trivially, and M2's guard is precisely the
  check that the conjunct holds on the governed data. The two remedies interlock
  correctly: M4's ground is only sound while M2's guard passes, and M2's guard
  refuses the run otherwise. That is the right coupling.
- **Is `6/8 = 0.750` right?** It is round 2's measurement of the reference band and
  I did not re-derive it from `stage3c_allocator_bound_v1.txt`; §2 uses it only to
  *disclaim* a rationale, so an error there would weaken nothing the run concludes.
- **Does D-537 actually permit a larger minimum?** Substantively yes — see n2 for
  the wording defect, which does not touch the licence, because **D-619 (3) rules
  it directly**: *"the CLASS count is the registered minimum and is what licenses
  detector round 3"*. The prereg implements a logged operator decision; it does not
  need D-537 to say the words.

### 4.6 The row reader, cross-checked against an independent parser

`census_classes.rows_of` builds its dict with `range(2, len(words) - 1)` — **step
one, not two** — so it emits overlapping pairs: the intended `name → value` pairs
*and* spurious `value → next-name` pairs. The intended entries survive only while no
value token collides with a field name. This is pre-existing (revision 4 did not
touch it), but it is the one thing that could make the governed run produce a wrong
number without anyone noticing, so I measured it:

- an independent parser that steps by **two** and asserts an even field count with
  no duplicate name **agrees with the shipped reader on every file**: 8 281 census
  rows across all twenty `artifacts/wp22_*` outputs, all five tallies identical;
- **0 value tokens collide with a field name** over those 8 281 rows;
- `cover` is the only string-valued column and its domain is
  `{impossible, minimal, none}`, none of which is a field name — confirmed against
  the shipped printer in `crates/pistol-search/src/census.rs`.

**The reader cannot corrupt the count on this row format.** Recorded as n6 so a
successor changing the row format knows the coupling exists.

---

## 5. New findings — all MINOR

**n1 — §6b pins `census_classes.py` at a revision that does not contain the M2
remedy, and the header repeats it. Fix before the run is launched.**
§6b's row reads *"`tools/texel/census_classes.py` | `0097f83`, gated by
`tools/texel_tests.sh` | all three counts, the class curve, **and the ordering
refusal**"*, and line 3 states *"Governing revision: `0097f83`"*. Both are now
false:

```
git show 0097f83:tools/texel/census_classes.py | grep -c OrderingViolated   ->  0
git cat-file -e 0097f83:tools/texel_tests.sh                               ->  absent
git diff --stat 0097f83..d664368 -- crates tools configs  -> 4 files, +133 −25
```

Revision 4 modified the instrument and did not move its own pin — which is
round-2's m1 recurring, in the one row where it is no longer immaterial, since the
bytes now differ. **I considered this MAJOR and settled on MINOR**, for three
reasons: no count changes (the two revisions agree on every non-refusing input, and
the ordering holds on all six real files with a widening margin); the row is
self-refuting, since it cites a gate that does not exist at the revision it names,
so no successor can actually follow it to `0097f83`; and the run governed by
`d664368` is reviewed — by this report — so CLAUDE.md's review-transfer rule is
satisfied in substance. **Fix**: `0097f83` → `d664368` in §6b and on line 3. Two
tokens. No gate checks commit SHAs (§4.3), so nothing will catch it later.

**n2 — §2 presents a quotation D-537 does not contain, for the clause carrying the
whole M4 remedy.** §2: *"D-537 … expressly permits **"a larger one with grounds"**"*.
D-537's text has no `larger` in it at all; what it says is *"TWO CONDITIONS THE
RULING FIXES AND A SUCCESSOR MAY NOT LOOSEN"*. The quoted string traces to
`matrix_wp22_cap_decision.md:207`, which attributes it to D-537 in the same
paraphrase-as-quotation shape. **Substance survives**: "may not loosen" does not
forbid tightening, and D-619 (3) rules the licence directly. **Fix**: attribute the
permission to D-619 (3), or quote D-537's actual "may not loosen" and draw the
inference in the document's own voice.

**n3 — §7 registers only the keys-per-root distribution.** Round-2's M4 minimal fix
asked for *"the classes-per-root and keys-per-root distributions"*, since classes
are now the licensing unit. §7 line 199 still says only keys-per-root. §2 assigns
the class-currency re-solve (ESTIMATED n = 42-150) to round 3, and **this run is the
only source of the distribution round 3 needs to do it**. Does not change this run's
count. **Fix**: add four words to §7.

**n4 — `tools/ci.sh`'s decision-key comment block now sits above the wrong gate.**
The paragraph explaining *"`D-276` and `D-277` were each appended TWICE"* was left
in place while the new gate 18 was inserted beneath it, so it now introduces
"offline texel and census tooling" and gate 19 (decision-key uniqueness) has lost
its rationale. Cosmetic; every number is right.

**n5 — the intro and §4 still quote the currency D-619 retired, and revision 4
corrected the *other* copy.** Line 28: *"distinguishable-trial rate **0.10 / 0.07 /
0.08** per position across the three rungs, a spread of **1.4x**"*, described as
*"the quantity this census exists to produce"*. The same commit corrected exactly
those figures in `matrix_wp22_cap_decision.md` to **0.26 / 0.14 / 0.15**, spread
**1.86x**. So two live documents that previously agreed (both in the retired
currency) now **disagree** — which is the D-423 shape this revision exists to
remove, landed in one place and not the other. The cap conclusion is unaffected
(1.86x is still far below the key count's 4.5x) and the cap is fixed by D-618
regardless, so no count moves. **Fix**: copy the corrected sentence across.

**n6 — the row reader steps by one (§4.6).** Pre-existing, MEASURED benign over
8 281 rows with zero collisions. Recorded because it is a latent coupling to the
row format, not because it is wrong today.

**n7 — `curve()`'s x-axis ends at the last win-proving firing, not at `n`.**
`highest = max(entry) + 1` over win-proving rows only, so if the tail of the run
proves nothing the curve stops early and a reader cannot see the flat tail — which
is the exact evidence §5's flip clause turns on. At the MEASURED root rate of 0.072
the chance of a whole empty 100-bin at the end is `(1 − 0.072)^100 ≈ 6 × 10⁻⁴`, so
this is a remote reporting risk, not a live one. The total class count is taken by
`tally()` and is unaffected. Pre-existing.

**Carried over from round 2, not landed, still MINOR**: m3 (§8's "0 of 20" where the
receipt says 0 of 100), m4 (§5's "still accelerating" — the increments are
`11, 7, 19, 10, 15`; "not saturating" is the supportable claim), m5 (§3's "89 205
positions" over-counts under §2's own identity — 87 228 distinct `key_full`), m6
(§4's `<census slice prefix>` is still a placeholder and §5 now registers n = 800;
§3 fixes the order, so *which* 800 is determinate, but the command is not
executable as written), m8 (§3 binds a successor to the retired
`wp22_cap_prereg.md`). m7 is now **discharged in substance**: the pilot's digest
`acdaee2b…0d542a` is sha-anchored by `wp22_census_prereg_rev3_REVIEW.md`, committed
at `d664368`, which satisfies rule 8's "a committed manifest may sha-index them"
even though §5 itself still does not name it.

---

## 6. What I attacked that survived

Stated explicitly, because a third-round PASS is worth only as much as the attacks
behind it:

1. **The refusal path, driven from the CLI on a hand-built input** — not through the
   test's own helper. It refuses, exit 1, before any output. Survived.
2. **The claim that the tests would catch a real drift** — five seeded mutations,
   including mutating the *allocator's* `COLUMNS` rather than the census script's.
   All five killed. Survived.
3. **The hermeticity claim** — run on a `git archive HEAD` unpack with no
   `artifacts/` directory in existence. Exit 0, 27 checks, zero skip language.
   Survived.
4. **The renumbering** — all 21 step strings, `GATE_TOTAL`, gate-name uniqueness,
   `bash -n`, and the four cheap gates re-run. Survived.
5. **The author's departure from round 2's prescribed M2 fix** (keeping
   `roots <= classes` instead of deleting it) — measured the margin across ten pilot
   prefixes; it widens monotonically from 0 at n = 50 to 26 at n = 500, so the kept
   half cannot fire at the registered n = 800, and it can only ever refuse, never
   over-report. Survived.
6. **The shipped row reader's off-by-one pairing** — the one live path to a silently
   wrong number. Cross-checked against an independent step-two parser over 8 281
   rows across every census artifact on disk: identical on all five tallies, zero
   name/value collisions. Survived.
7. **The M4 ground's logic** — `classes >= 28 ∧ classes <= keys ⟹ keys >= 28`, with
   the second conjunct now checked rather than assumed. The two remedies interlock
   in the correct direction. Survived.
8. **Whether the criterion is now single-voiced** — all ten occurrences of `28`
   inspected individually. No section adjudicates the wrong count. Survived.
9. **Whether the guard could fire on a legitimate zero-firing run** — `0 <= 0 <= 0`
   passes, and §9 catches that case separately. The two guards do not collide.
   Survived.

**I did not re-open the reversal.** Round 2 adjudicated it, attacked it four ways,
and I found nothing in revision 4 that disturbs it; the pilot figures it rests on
reproduce exactly through the modified instrument.

**The run is runnable, the count it produces is defended by a gate that fails when
it should, and the registration now says once what licenses round 3.** Fix n1 before
launching — it is two tokens and no gate will catch it — and the rest can ride.
