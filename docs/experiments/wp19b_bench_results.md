# WP-1.9b — the three runs, read against what was registered before them

Pre-registration: `wp19b_bench_prereg.md`, committed at `b60094e` (amended once at
`3e004e1`, before any run — see §1.1 below). Nothing here moves a registered
number (D-374).

---

## 0. The headline

**O-3 does not flip, and it is not close.** The hand-rolled open-addressing
probing table D-225 named and D-501 registered measures **1.518 early / 1.594
late** against the comparand's **1.783 / 1.909** — below it in BOTH bands, so not
even the one-band finding the terms contemplate. **D-501 is discharged by
measurement**, which is the outcome it said silence could not produce. O-2 is
confirmed, and what landed is O-2 moved inline.

---

## 1. The dry run — PASSES, and its referent was not this session's

`tools/bench_delta.sh rev:wp19/mx-base rev:wp19/mx-O1 5`,
`artifacts/wp19b_bench_dryrun_v1.txt`, exit 0.

| band | measured | O-1's recorded pair | difference | bound |
|---|---|---|---|---|
| early | **1.195** | 1.198 | 0.003 | ±0.08 |
| late | **1.242** | 1.242 | 0.000 | ±0.08 |

`bench_delta: node identity holds per position, both budgets, all reps`.

**What this establishes, which is ATTRIBUTION and not speed.** The referent is
another session's run at the same revision on the same instrument
(`artifacts/wp19_mx_bench_O1_fmt_v1.txt`), so it is external to everything this
session did. A swapped-sides defect answers near 0.83, a same-binary defect is
refused by the script's digest check, and a substituted fixture moves the ratio
off the referent. None of the three can produce 0.003 and 0.000. **It follows
that this machine reproduces the conditions the matrix's numbers were taken
under**, which is what makes the O-2 comparand quotable today without
re-measuring it.

### 1.1 The amendment the dry run's own discipline produced

The pre-registration first spelled this command with ONE rep. The instrument
refuses it — `tools/bench_delta.sh:136` is
`[ "$REPS" -ge 5 ] || fail "REPS must be an integer >= 5 (pre-registered)"` — so
that spelling could never have run. It was amended to 5 reps **before any run**,
recorded in the prereg rather than edited away, and the amendment makes the dry
run stricter and costlier, not weaker. This is the defect dry-run discipline
exists to catch, caught by dry-run discipline.

---

## 2. The FLIP bench — NO FLIP, in both bands

`tools/bench_delta.sh rev:a5c5661 rev:6ea88b2bad1008ce87d262639fd999b23f20f718 5`,
`artifacts/wp19b_bench_flip_v1.txt`, exit 0. The candidate is `wp19b/o3-fixed`:
O-3 written INLINE, after REVIEW-impl's fix round.

| band | O-3 (this run) | O-2 comparand (D-501, does not move) | flips? |
|---|---|---|---|
| early | **1.518** (IQR 1804 on 367590 nps) | 1.783 | **no — below** |
| late | **1.594** (IQR 1827 on 318676 nps) | 1.909 | **no — below** |

`bench_delta: node identity holds per position, both budgets, all reps`.
Time-to-depth, the declared cross-check: 1.538 early (|dev| 0.020), 1.586 late
(|dev| 0.009) — it agrees, which is what a cross-check agreeing looks like and
never corroboration.

**The instrument's own `VERDICT PASS` lines are quoted and are NOT this
document's verdict.** They read against `bench_delta.sh`'s standing `[1.4, 2.5]`
bracket, which descends from D-220's package. D-501's flip condition is a
comparison against 1.783 / 1.909, and O-3 fails it in both bands. The prereg said
this in advance.

### 2.1 What the three inline/module numbers now say together

| shape | vs `a5c5661` | source |
|---|---|---|
| O-2 inline | 1.783 / 1.909 | `wp19/mx-O2`, D-501 |
| O-2 in a module (shipped WP-1.9) | 1.508 / 1.579 | D-502 |
| **O-3 inline** | **1.518 / 1.594** | this run |

**O-3 inline lands on top of O-2-in-a-module, not on top of O-2 inline.** A
hand-rolled linear-probing table over the same key is roughly as fast as a
`HashMap` that has paid a module boundary, and about a seventh slower than the
same `HashMap` inline. The honest reading is narrow: `hashbrown`'s SIMD group
probe and unchecked indexing beat a bounds-checked linear walk on this workload,
at this occupancy, on this machine. **Nothing here says a probing table is
slower in general**, and the option is not re-opened by this line — it is
disposed of on a measurement instead of on the inference D-500 refused to accept.

### 2.2 What the run does NOT establish

It is not a strength claim and no SPRT is owed (D-495). It says nothing about
memory: O-3's table was never measured for footprint, because the flip trigger's
one axis is speed and a second axis measured after the fact is the ground D-500
struck.

---

## 3. The LANDING bench

Registered in `wp19b_bench_prereg.md` §2 before it was taken: direction FASTER in
both bands, outcome-B bracket **[1.10, 1.30]**, abort below 1.00.

**Taken TWICE, and the second is the governing one.** The first ran against
`1703d25`, the landing before REVIEW-impl's fix round; the shipped revision is
`13abe40`, and a run that does not name the revision that ships is the defect
D-500 recorded a fourth instance of. So it was re-taken rather than argued.

| band | v2 `13abe40` — **GOVERNING** | v1 `1703d25` — replication | registered bracket | ground it was registered on |
|---|---|---|---|---|
| early | **1.186** | 1.171 | [1.10, 1.30] — **INSIDE** both | 1/0.844 = 1.185 |
| late | **1.214** | 1.205 | [1.10, 1.30] — **INSIDE** both | 1/0.828 = 1.208 |

`bench_delta: node identity holds per position, both budgets, all reps` in both.
Time-to-depth, the declared cross-check, v2: 1.238 early (|dev| 0.052), 1.258
late (|dev| 0.044). Artifacts `artifacts/wp19b_bench_landing_v2.txt` (governing,
exit 0) and `_v1.txt` (replication, exit 0).

**Why a re-run at all, since the fix round touched one comment and some
`#[cfg(test)]` code — recorded because it surprised this session.** The two
revisions build DIFFERENT release binaries (`b66f88c9…` against `e0eb1b19…`),
and the same-path rebuild rules out the build directory: `1703d25` rebuilds to
`b66f88c9…` at a second path. The cause is that a `panic!`/`expect` embeds its
own `file!`/`line!`, so **adding two lines of comment above `undo`'s body moves
every panic location below it** and the binary changes while behaviour cannot.
Benign — and precisely why the receipt must name the revision that ships rather
than reason about a diff.

**The measured cost is recovered, and it lands where the prereg said it would.**
The bracket was registered by inverting WP-1.9's head-to-head pair, and the
governing run answers 1.186 against a predicted 1.185 and 1.214 against a
predicted 1.208 — **0.001 and 0.006 away**, with the replication 0.015 and 0.009
from it. That is the same composition check WP-1.9's closure used
and it closes to the same order. **What makes the two runs independent is stated
precisely, because an earlier draft of this sentence said "a day apart" and that
is false** (REVIEW-impl, MAJOR 1): they are 3h13m apart on the same day —
`artifacts/wp19_bench_inline_vs_module_v1.txt` at 14:54, this one at 18:07. The
independence that does the work is not elapsed time: they are separate runs of
separately built binaries, at different governing revisions, registered by
different sessions, and the second was registered before it was taken.

**The instrument's own `VERDICT BELOW-BRACKET` lines are quoted and are NOT this
document's verdict** (D-327 wants the gate's own words, and the prereg said in
advance what they refer to): they read against `bench_delta.sh`'s standing
`[1.4, 2.5]`, which is D-220's package's bracket for a whole change, not this
document's [1.10, 1.30] for a code-location move. Against the bracket registered
for THIS run, both bands are inside.

**No bracket moved.** The prereg was committed at `b60094e` before any run; its
one amendment (`3e004e1`) touched the dry run's rep count only, also before any
run, and is recorded in §1.1 above.

---

## 4. The IDENTITY leg — IDENTICAL, and to the same digest WP-1.9 recorded

`artifacts/wp19b_byte_identity_v2.txt`, at the SHIPPED revision (the `_v1.txt`
run governs `1703d25` and is superseded by it for the same reason the landing
bench was re-taken). Baseline `3c9e28b` (binary sha256 `ddbae8f3…`) against the
landed `13abe40` (binary sha256 `e0eb1b19…`) — the two digests DIFFER, so the
comparison cannot pass vacuously. 44 positions, both
determinism budgets, **88 searches, 422 output lines, 88 bestmoves, 0 error lines
on each side**, `nps` and `time` elided.

**RESULT: IDENTICAL**, digest of both
`0b1cb8054857e8a4a877297733d284b23efaeaad8ccd76f0a6a65d34b5512edf` — **the same
digest WP-1.9's own leg recorded** (`artifacts/wp19_byte_identity_v2.txt`). So
search output is unchanged from the pre-WP-1.9 baseline, through WP-1.9, through
this landing: three revisions of the eval's storage, one output.

This is what licenses reading the landing bench as a pure container move, and it
is why no SPRT is owed (D-495).
