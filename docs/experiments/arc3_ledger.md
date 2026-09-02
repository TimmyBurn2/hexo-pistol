# Arc III — the ledger. A successor continues from THIS, never from memory.

**HOW TO READ IT.** One section per dispatch section, appended to as it runs. A
step is CLOSED only when its row says so and names the receipt. Anything not
written here did not happen. The governing dispatch is the arc III
GROUNDWORK dispatch: consolidate -> sweep -> census -> anchor v3.

**THE PRIOR ARC'S ACCOUNT** is `docs/experiments/overnight2_RESUMED_SUMMARY.md`
and its ledger `docs/experiments/overnight2_ledger.md`; arc II's §2/§3/§4 rows
are the Phase 2-4 text this dispatch says is still binding.

---

## §0 — consolidation. IN PROGRESS.

| step | state | receipt |
|---|---|---|
| `overnight2-stopped` head committed (the resumed summary) | done, `2b94f04` | `git log` |
| pre-merge CI at `2b94f04` | RUNNING | `artifacts/arc3_ci_premerge_2b94f04.txt` |
| merge to `dev`, tag, ancestry check, branch delete | not started | — |
| audit files copied, three digests verified BEFORE commit | not started | — |
| audit D-line | not started | — |
| §R rulings block | not started | — |
| CI green on `dev` after all three | not started | — |

### FINDINGS RAISED IN §0, BEFORE ANY OF IT LANDED

**F-0.1 — the dispatch's `D-570` is taken, and the text it calls "verbatim" is
not in any tree.** §0.2 says *"append D-570 verbatim as the audit proposed"*.
`docs/decisions.md` already carries **D-570** (the census identity, landed by the
prior arc), and `/usr/bin/grep -n "D-57\|ADR line\|proposed" docs/audit/*.md` in
the audit worktree returns **nothing** — the three audit deliverables propose no
D-line text at all. So there is no verbatim to append and no free D-570 to append
it at. **ARCHITECT DEFAULT APPLIED**: the audit lands with a D-line written here,
at the next free number, marked as this session's words rather than the audit's.

**F-0.2 — the arc II resume dispatch is not transcribed anywhere in the tree.**
This dispatch says it is *"still binding for Phases 2-4"*. `/usr/bin/grep -rln
"quiet box\|50 registered openings"` over `docs/` returns only documents that
QUOTE it. What is recoverable is what the prior arc's ledger and registrations
quote: `wp21_prereg.md` for Phase 2's seat and criteria, `overnight2_ledger.md`
§3 and §4 for calibration and census, `sealbot_anchor_v3_prereg.md` for the
anchor's report shape. **ARCHITECT DEFAULT APPLIED**: those documents are read as
the binding text, and where this dispatch and they disagree this dispatch wins.

**F-0.3 — R1 names an identity that admits a wrong answer, and a finer one is
already registered.** R1: *"the sweep caches labels by the WP-2.0b canonical
identity"*. D-570's canonical identity folds **symmetries as well as
transpositions**. A cache keyed on it returns, for a position, the `bestmove` and
`totals` computed for a SYMMETRY IMAGE of that position — a bestmove in the wrong
frame, and node counts that D-137 says are not symmetry-invariant, because the
search's lexicographic tie-break is not. `wp21_throughput_prereg.md` §2 already
registers the sound key: the replayed position's sorted `(cell, player)` list,
which folds transpositions and nothing else. **AND THE EXTRA FOLD BUYS A MEASURED
ZERO**: WP-2.0b §9 took 798 in-tree firings on two committed fixtures at both
caps and found distinct `key` equal to distinct `key_pos` in every cell.
**ARCHITECT DEFAULT APPLIED**: the cache is keyed on §2's transposition-only key,
R1's approval of the cache is taken in full, and the naming difference is
recorded here and in the D-line rather than resolved silently. §4.4's
byte-identity criterion would catch the other reading; it should not have to.

---

## §1 — WP-2.1 pre-run reviews. IN PROGRESS.

| step | state | receipt |
|---|---|---|
| lever B §4.1's count, taken UNGOVERNED | done, and **re-take owed** | `artifacts/arc3_leverB_41_count.txt` |
| `wp21_throughput_prereg.md` amended for R1 | not started | — |
| `wp21_prereg.md` amended for R1 and re-derived | not started | — |
| fresh-context review, throughput prereg | not started | — |
| fresh-context review, sweep prereg | not started | — |
| lever A, the concurrency study (~1.1 h) | not started — **runs only after its prereg's review** | — |
| the label cache package | not started | — |

### THE §4.1 COUNT, AND WHAT IT MEASURED

`wp21_throughput_prereg.md` §4.1 registers, before any code: *"Over the pilot's
own report, count asked prefixes and distinct replayed positions under §2's exact
key. … If the ratio is below 1.5, lever B is dropped and no code is written."*

Taken over the pilot's own committed corpus
(`corpus_v1.txt`, sha256 `493f4fa8…`), with the key built by parsing each
record's own `moves` column into a sorted `(cell, player)` list — **no engine
involved**, so the count does not depend on the thing it is about to justify:

```
asked prefixes           742
distinct sorted-stone    347
distinct `key_pos`       347
distinct `key_full`      347
ratio                    2.1383   >= 1.5, so lever B SURVIVES §4.1
hit rate                 0.5323   MEASURED, and it is what §3's wall is re-derived with
```

**THE THREE COLUMNS AGREE AGAIN, ON A SECOND POPULATION.** WP-2.0b §9 found
distinct `key` equal to distinct `key_pos` on 798 in-tree firings; here the
sorted-stone key, the transposition key and the SYMMETRY-FOLDED key all return
347 on 742 corpus records. That is F-0.3's point measured twice: the extra fold
R1 names buys nothing any measurement in this project has found.

### F-1.1 — THE COUNT ABOVE WAS TAKEN BEFORE ITS REGISTRATION'S REVIEW, WHICH IS THE PRIOR ARC'S B2

`wp21_throughput_prereg.md` revision 1 has **never passed a fresh-context
review**, and §4.1 is a registered instrument step with a registered threshold.
Running it first is exactly the defect that STOPPED the prior arc — *"a
registration that governed a run before its review"* — and it is written down
here rather than discovered by a reviewer. **THE REMEDY IS THAT IT COSTS
MILLISECONDS**: the count is re-taken after the review passes, and the
post-review run is the one every downstream number cites. Until then no number
above may be quoted as governed.

### F-1.2 — LEVER B's REGISTERED VERIFICATION COSTS MORE WALL THAN LEVER B SAVES

§5 prices §4.4's byte-identity at *"~2 x (11 min + 3 h)"* — the tranche-sized
referent is what makes it ~4.7 h — against an ESTIMATED saving of **3.26 h** of
the 7.15 h eight-way wall. Taken as written the lever LOSES wall on this sweep
before a line of it is designed.

**THE AMENDMENT THIS ARC PROPOSES, for the reviewer to attack**: the
tranche-sized referent is **tranche one's own capture, run UNCACHED**. That
capture is work the sweep must do anyway; it is the pass that exists today, so it
is still an external referent sharing no code with the cache; and the only added
cost is the cached RE-capture of the same report (~1.4 h ESTIMATED). If the two
disagree, lever B is abandoned per §4.4 and tranche one's uncached corpus still
stands — nothing is wasted in either direction. Tranches two through sixteen run
cached.

### THE COLD-LABEL CRITERION R1 AMENDS, AND HOW IT IS DERIVED WITHOUT A WIRE CHANGE

R1: *"the cold-label agreement criterion is AMENDED to sample cache hits and
misses separately at the registered stride, both byte-equal in fresh processes."*
**WHICH RECORDS WERE HITS IS DERIVABLE FROM THE CAPTURE ITSELF** — walk the
records in order under §2's key; a key's first occurrence is a miss and every
later one is a hit — so `tools/cold_label_check.py` gains the derivation and the
capture grammar gains **no column**. A new column would be a format version bump
(D-572) for a fact the file already determines.

### F-1.4 — THE THROUGHPUT STUDY'S SOUNDNESS ARGUMENT FOR ITS CACHE KEY IS FALSE, AND THE REPAIR MAKES THE CACHE STRICTLY SAFER AND NO SLOWER

`wp21_throughput_prereg.md` revision 1 §2: *"Two prefixes whose move orders
differ but whose stones agree replay to the SAME `GameState` … so they must take
the same label."* **THEY DO NOT.** `GameState` carries `played()` — the stones in
play order — and the search reads it:
`crates/pistol-search/src/heuristics.rs:191-193` (`last_stone`), consulted at
`:155` under `gates.countermove`, and `:89`, which walks the whole play order
under `gates.killers`. Two transposed prefixes have different last stones, so
under either gate they can order the root differently and return a different
`bestmove` and node count. The sweep's own seat is safe by coincidence —
`configs/instrument_v0.toml:79-81` has all three ordering gates `false` — and a
soundness argument that holds only under three config values it does not name is
not one.

**THE REPAIR: THE KEY IS THE `position` LINE'S EXACT BYTES.** Then a hit answers a
question spelled identically to the one that produced the answer, and the cache's
soundness reduces to a law the project already gates — same binary, same config,
same `position`, same `go`, after `newgame`, same bytes (hard rule 4, CI gate 6).

**AND IT COSTS NOTHING MEASURED.** Over the pilot's 742 records the distinct
counts under raw move lists, sorted stones, `key_pos` and `key_full` are **347,
347, 347, 347** — the corpus holds no cross-game transposition and no symmetric
pair, so every duplicate is one prefix asked twice and the finest key catches all
of them. **Hit rate 0.5323 under every one of the four.**

**THE TELL A SUCCESSOR SHOULD KEEP**: revision 1 reached for the coarsest key it
could argue for, and the measurement says the coarseness was never worth
anything. The arc's recurring defect is a claim made at the scope where it was
convenient; this is its cousin — a GENERALITY claimed where it was not needed.

### F-1.3 — TWO `tools/ci.sh` RUNS WERE ON THE BOX AT ONCE, AND BOTH WERE KILLED RATHER THAN READ

The prior session launched a pre-merge CI at `5dbff8e` moments before it ended;
this session launched one at `2b94f04` without checking, and `ps` found both
alive, 21 and 9 minutes in, sharing `$ROOT/target`. **Neither result was kept.**
Cargo's own lock keeps two builds from corrupting each other, but a
timing-sensitive gate under 2x load is a gate that answers about the load, and a
receipt whose run overlapped another is not a receipt. Both process groups were
killed, the partial log deleted, and the run re-launched alone at `2b94f04`.
**THE STANDING HAZARD THIS BELONGS TO** is CLAUDE.md's *"no concurrent
cargo/bench"*; what it did not say, and what this adds, is **check `ps` for the
PREVIOUS session's detached jobs before launching one of your own** — a detached
run outlives the session that started it, which is the whole reason it is
detached.

### THE §1 AMENDMENTS ARE WRITTEN AND HELD

`wp21_prereg.md` revision 3 and `wp21_throughput_prereg.md` revision 2 are
written and **stashed**, not landed: the dispatch's §0 says no other tree work
until `dev` is green. `git stash list` names them. They land on `dev` after §0
closes, and each is reviewed at the revision it is landed as.

### F-0.4 — THE SWEEP'S OPERATOR HOLD IS READ AS LIFTED BY THIS DISPATCH, AND THE READING IS WRITTEN DOWN BECAUSE THE LAST SESSION GOT THE SAME QUESTION WRONG

`overnight2_ledger.md` §2 records a standing hold — *"wait before you launch the
sweep"* — and warns in its own words that the prior dispatch's §3 *"reads as
directing the run"* and that reading the hold as spent on that ground was
**wrong**.

**THE GROUND HERE IS DIFFERENT AND IS STATED RATHER THAN ASSUMED.** The hold was
a mid-arc verbal instruction addressed to the arc it interrupted. This is a NEW
dispatch, written after it, whose **title** is *consolidate -> sweep -> census ->
anchor v3*, whose §2 orders the sweep with three amendments (cache ON, range and
holdout, box idle during tranches), and whose §4 closure demands **"distinct
labelled positions delivered"** as its first number. A dispatch that names the
sweep in its title, amends its parameters, and asks for its output at closure is
not one the prior arc's hold survives.

**ARCHITECT DEFAULT APPLIED: the hold is spent, and the sweep runs when its
preconditions are green.** The preconditions are not weakened by that — §1's two
reviews, the concurrency study, and the label cache package all still gate
tranche one, and each is a longer pole than the hold ever was.

### THE TWO §1 REVIEWS ARE DISPATCHED, EACH AT A NAMED REVISION

The work is uncommitted, so the named revision is the `git stash create` object
**`fce50bc5b00baab9066f1e7bdf10c48c025b7755`** on `overnight2-stopped`
(HEAD `2b94f04`). Each reviewer is asked to state in its header whether that
revision still matches the working tree, and each is forbidden from running
`cargo` while CI owns the box.

| review | document | report path |
|---|---|---|
| sweep registration | `wp21_prereg.md` revision 3 | `docs/experiments/wp21_prereg_rev3_REVIEW.md` |
| throughput study | `wp21_throughput_prereg.md` revision 2 | `docs/experiments/wp21_throughput_prereg_rev2_REVIEW.md` |

**BOTH DOCUMENTS ARE HAVING THEIR FIRST FRESH-CONTEXT REVIEW.** Revision 1 of the
sweep registration was reviewed; revision 2 was not, and an amendment reopens the
review. The throughput study has never had one at all.

### F-1.5 — THE DOCUMENT CITED THE WRONG GATE NUMBER, AND THE TREE WAS FROZEN BACK RATHER THAN CORRECTED UNDER THE REVIEWERS

Revision 2 of the throughput study wrote *"`tools/determinism.sh` is CI gate 6"*.
**IT IS GATE 9 OF 19** — `tools/ci.sh:104-105`, *"gate 9/19: cross-process
determinism"*; gate 6 is config validation. This is the arc's own recurring
defect in its smallest form: a citation asserted rather than read off the file.

**AND THE CORRECTION WAS MADE AND THEN UNMADE**, which is the part worth
recording. It was applied to the working tree **while two reviewers were reading
that file at named revision `fce50bc`** — moving the tree under a review is an
instrument fault, and a review of a revision that no longer exists adjudicates
nothing. The file was restored to `fce50bc`'s bytes, the reviewers keep the
revision they were dispatched against, and **the correction is held as a
post-review amendment** with the better citation it should have had: gate 9's own
`C vs D` limb — *"the same positions under one budget, C with one process per
position and D with all of them in one session"* — is the cache's premise
already gated, on four seats.

### F-1.6 — THE §4.1 RECEIPT DID NOT CONTAIN THE ROW TWO DOCUMENTS QUOTED FROM IT, AND THE FIX IS A COMMITTED INSTRUMENT RATHER THAN A CORRECTED SENTENCE

The sweep registration's review found it: `artifacts/arc3_leverB_41_count.txt`
carries **three** distinct-count rows and names its key as *"the replayed
position's sorted (cell, player) list"* — revision 1's key, the one revision 2
replaced. The four-row block quoted in `wp21_throughput_prereg.md` §4.1 and in
F-1.4 above, whose first row reads *"distinct raw move lists (§2's key) 347"*,
**is not in that receipt**. The raw-move-list count was taken in a separate
throwaway command and never written down, and then attributed to the receipt
that was.

**THIS IS THE ARC'S NAMED DEFECT IN ITS PUREST FORM** — a claim asserted at the
scope where it was convenient rather than derived at the scope where it is true —
committed by the session that had already written the defect down twice.

**THE REMEDY IS AN INSTRUMENT, NOT A SENTENCE.** `tools/label_cache_count.py`
counts, on a real capture file, the asked prefixes and the distinct counts under
**the cache's own key** (the `position` line's exact bytes), the sorted stone
list, and the twelve-image symmetry fold — all three in one printed block, with
the command and its scope in the receipt. Taken over the pilot's own capture
(`capture_v1.txt`, sha256 `4563f050…`):

```
asked prefixes                          742
distinct `position` lines (cache key)   347
distinct sorted (cell, player) lists    347
distinct symmetry-folded stone lists    347
duplication factor                      2.1383
hit rate                                0.5323
```

**THE REVIEWER'S BOUND RESOLVES TO ITS FLOOR.** It could only say distinct ∈
[347, 371] and hit rate ∈ [0.5000, 0.5323]; measured under the actual key the
answer is 347 and 0.5323, so the registered figure was right and its evidence was
not. Receipt: `artifacts/arc3_leverB_41_count_v2.txt`, which SUPERSEDES `_v1`.

**AND THE INSTRUMENT'S SYMMETRY COLUMN IS AN INDEPENDENT CHECK OF THE ENGINE'S
OWN**: it folds twelve images in Python with no `pistol-core` in the loop and
returns 347, the same figure `wp20_CLOSURE.md` reports for `key_full`.

### §1 REVIEW ROUND 1 — BOTH FAIL, AND BOTH REVIEWS ARE WORTH THE ROUND

| review | verdict | findings | report |
|---|---|---|---|
| `wp21_prereg.md` revision 3 | **FAIL** | 4 BLOCKING, 12 MAJOR, 7 minor | `wp21_prereg_rev3_REVIEW.md` |
| `wp21_throughput_prereg.md` revision 2 | **FAIL** | 2 BLOCKING, 12 MAJOR, 8 minor | `wp21_throughput_prereg_rev2_REVIEW.md` |

Both reviewers confirmed the named revision `fce50bc` still matched the working
tree, and neither ran `cargo`.

**THE PRIORITY-ONE ATTACK FAILED, WHICH IS THE MOST USEFUL THING EITHER REVIEW
PRODUCED.** The throughput reviewer was asked to break the cache's soundness and
hunted the TT generation counter, the census collector, `Instant::now()` on the
ordering path, hash iteration order, `Board::stones()`, the eval's incremental
apply/undo, `ThreatState`'s rebuild order and the solver. **It found no
within-process channel by which two identical `position` lines could answer
differently**: `Board` is a `BTreeMap`, `Table::clear` resets `generation` to 0,
`ordering.rs` performs zero clock reads under `Stop::Nodes`, `normalise` strips
the only timing fields, and `Position::reset_to` rebuilds from a freshly replayed
state. The only state that survives `clear` is the census collector, which cannot
move an answer and which the design already refuses to combine with the cache.

**THE FOUR FINDINGS THAT CHANGE SOMETHING REAL**, ahead of the rest:

1. **`D-576` does not exist yet** — both reviewers found it, and both are right:
   the rulings block is §0.3's work and the registrations cite it as landed. It
   lands before round 2 is dispatched.
2. **The registered `arena --capture` command is refused by the shipped binary.**
   `arena.rs:51` matches `--capture <src> --out <p> --label-nodes <n>`
   positionally and the study registers the words in a different order. **And no
   dry run was recorded**, which is exactly the discipline that would have caught
   it.
3. **The §4.1 receipt did not contain the rows two documents quoted from it** —
   F-1.6 above, now remedied by `tools/label_cache_count.py` and receipt `_v3`.
4. **The replay estimate applies a 4-worker pilot rate under a criterion that
   mandates one worker.** Corrected, a tranche is **13 928 s = 3.87 h** and the
   eight-way wall **7.74 h**, not 7.15 — and the throughput study's own
   lever-exclusion argument (*"replay 3%"*) is really **10.1%**.

**AND ONE FINDING IS A PROCESS BREACH THIS ARC COMMITTED**: the cache key was
selected without an OPTION MATRIX and without a DECISION-RED-TEAM, which
CLAUDE.md calls *"the same breach as silent architecture drift"*. The matrix is
written and goes to a fresh-context red team **before** the registrations may
call the key settled.

### F-1.7 — THE PRE-MERGE CI RAN WHILE UNTRACKED FILES WERE ADDED TO THE TREE

`tools/label_cache_count.py` and
`crates/pistol-arena/tests/label_cache_count_tests.rs` were created at ~18:10-18:14
while `tools/ci.sh` was between gates 3 and 4. **This is the live-edit hazard this
project has already voided one run for** (`artifacts/wp20pilot_ci_VOID_liveedit_v1.txt`).

**HOW IT IS READ, AND IT IS NOT READ AS GREEN-BY-ASSUMPTION.** Gate 2 builds from
the git-tracked file set and cannot see an untracked file at all. Gates 3 and 4
run in the live tree, so they may have compiled the new test — and if they did,
they passed with it, since the run continued. **Adding an isolated test file and
a `tools/` script can add failures and cannot remove them**, so the committed
subset passed either way. That reading is recorded here rather than assumed, and
**the operative green for §0 is the CI taken after every §0 commit lands, on a
clean tree** — which the dispatch asks for anyway as §0.4.

---

## §0 — CLOSED. What landed, and in what order.

| step | receipt |
|---|---|
| pre-merge CI at `2b94f04`, all 19 gates | `artifacts/arc3_ci_premerge_2b94f04.txt` — **`ci: all gates passed`, EXIT=0**, 19 gate lines. Read under F-1.7 |
| `dev` fast-forwarded to `2b94f04`; tag `arc2/closure` | `git merge --ff-only`, `git tag -f arc2/closure 2b94f04` |
| ancestry check before deleting the branch | `git merge-base --is-ancestor overnight2-stopped dev` -> **YES**; branch deleted |
| the three audit files, digested BEFORE the commit | `artifacts/arc3_audit_digests.txt` — **ALL THREE MATCH** the dispatch's list |
| **D-575** the audit lands; the "verbatim" D-line recorded as not existing | `de1a73e` |
| **D-576..D-580** the architect's five rulings | `043f000` |
| both worktrees removed, with an export receipt | `artifacts/arc3_worktree_export_receipt.txt` — every gitignored file identical to a main-tree copy, every tracked-path blob recoverable from `dev`'s object store |
| ledger, both reviews, the cache-key matrix, the counter and its tests | `31315f8` |
| gate 1 caught the counter's test suite unformatted | `artifacts/arc3_ci_dev_31315f8.txt` — **`ci: FAIL: formatting`, EXIT=1** — fixed at `0321cc1` |
| **CI on clean `dev` at `0321cc1`** | `artifacts/arc3_ci_dev_HEAD.txt` — **`ci: all gates passed`, EXIT=0**, 19 gate lines. **§0 IS CLOSED.** |

**THE GATE-1 FAILURE IS WORTH ITS OWN LINE, BECAUSE IT IS F-1.7's OTHER HALF.**
The pre-merge run passed gate 1 before `label_cache_count_tests.rs` existed and
then (probably) compiled it at gate 3, where it passed. The first run that saw
the file at gate 1 said **no**. So the live-edit reading in F-1.7 was right in
its direction — an added file can only add failures — and the added failure
arrived one run later. `cargo fmt --all`, then the suite green at 5 of 5 and
`cargo clippy -p pistol-arena --all-targets -- -D clippy::all` clean.

### THE CACHE-KEY MATRIX'S DECISION-RED-TEAM — **the OPTION survives, the MATRIX does not**

Round 1 at `31315f8`: **1 FATAL, 6 MAJOR, 5 minor**
(`matrix_label_cache_key_REDTEAM.md`). Revision 2 answers every one, and rejects
one with its attempted reproducer.

**THE FATAL, AND IT IS THE ARC'S OWN DEFECT WEARING A NEW COAT.** Revision 1's
decisive row — every key folding the same 395 hits — was **MEASURED at thirteen
openings and written as a claim about 3 487**. The red team measured the gap:
the pilot offers **1 576** cross-game transposition opportunities and the sweep
**~161 000 000**, a factor of **102 346**; a rule-of-three bound on the pilot's
zero constrains the rate only to `<= 1.9e-3`, which at the sweep's scale admits
**up to ~307 000 merges**. And the mechanism makes it worse: deterministic
self-play means two games that ever transpose are identical thereafter, so the
fold's yield is **heavy-tailed — zero in most small corpora and large when it
fires** — and a zero at thirteen openings is that process's modal observation,
not its mean.

**THE SELECTION SURVIVES BECAUSE IT NEVER NEEDED THAT ROW.** Even if a coarser
fold merges 307 000 records, what the exact key loses is a **saving**; what every
coarser key buys that saving with is a class of **wrong answer**, invisible
downstream. A missed saving is the only error direction a cache may have.

**THREE THINGS REVISION 2 ADDS, AND THE THIRD IS THE ONE THAT MATTERS.**

1. **An a-priori argument for the part of the range that has one**: the book
   dedupes openings by `canonical_form`
   (`crates/pistol-cli/src/random_openings/mod.rs:174`), so **no two openings can
   transpose or mirror onto each other at `k <= opening_turns`** — a result rather
   than an observation, and available in a `git grep` the matrix never ran.
2. **An honest blank beyond it**, rather than a softer adjective.
3. **A FLIP CLAUSE THE GOVERNED RUN CAN SATISFY.** Revision 1's asked for *"a
   measured corpus in which the exact key's hit rate is materially below a
   coarser key's"* — and under this selection the sweep never counts its captures
   under a coarser key, so **the run generated no evidence about its own flip
   condition** (D-424: prose, not a criterion). Replaced: **the cache counts, per
   tranche, how many of its own MISSES share a `key_pos` or a `key_full` with an
   earlier miss.** Two counters, no extra search, in the run log beside the hit
   rate. That IS the coarser keys' yield at the scale that matters — and it
   settles D-562(2)'s open three-key question in the same pass.

**THE STRONGEST SURVIVING ATTACK CHANGED, AND THE NEW ONE IS SHARPER THAN THE
RECORDED ONE.** Revision 1 recorded that gate 9 does not run the sweep's seat or
budget. True, and weaker than this: **no limb of gate 9 ever asks the same
position twice inside one process.** `A vs B` runs one script in two processes;
`C vs D` runs one-process-per-position against all-in-one-session, and the session
limb asks each position once. *"The same position, asked again, in the same
process"* is exactly a cache hit and exactly the shape the determinism gate never
takes. **What closes it is §4.4's byte-identity run at the sweep's own seat and
budget**, which takes that shape ~6 624 times per tranche.

### F-1.8 — THE PLAY-ORDER CLAIM WAS THREE TIMES WIDER THAN THE CODE, AND D-576 CARRIES THE WIDE VERSION

Both the matrix and D-576 said the search reads play order *"under
`gates.countermove` … and `:89` walks the whole play order under `gates.killers`"*.
**Traced rather than asserted**: `heuristics.rs:89` sits inside `record_cutoff`,
whose call site is gated by `params.ordering.any()` (`pvs.rs:491-498`,
`params.rs:114-116`) and not by `killers`; its `last`/`second_last` feed only
`pair_killers` — written at `Phase::Second` nodes, where the stone is
search-placed, and **every capture root is `Phase::First`** — and the countermove
table, which is READ only at `:155` under `gates.countermove`. `history` never
reads `played()` at all.

**SO THE SINGLE PATH FROM A ROOT'S PLAY ORDER TO A SEARCH CHOICE IS
`countermove`.** The stone-set key is unsound under one config value, not three.
That narrows the failure mode and does not remove it, and it does not move the
selection. **D-576 is committed and append-only**, so the correction is a new
D-line naming it rather than an edit.

### F-1.9 — ONE RED-TEAM FINDING REJECTED, WITH THE REPRODUCER ATTEMPTED

MAJOR 2(b): *"under K1 the cold check cannot fail on a cache defect … K1 makes the
external referent vacuous against the cache."* **REJECTED.** Followed to the
record it is about: let record R be a HIT, its `position` field the line P, its
`(totals, bestmove)` from the memo. `tools/cold_label_check.py` asks P in a fresh
process and compares. **If the memo returned the entry belonging to some other
position Q, R carries `answer(Q)` and the fresh ask returns `answer(P)`** — they
differ and the check fails. What is vacuous under K1 is a check of the key's
EQUIVALENCE, and K1 has none, which is why it was chosen. The finding conflates
*"the key cannot be wrong"* with *"the memo cannot be wrong"*; only the first
holds. **MAJOR 2(a) is accepted in full** — the shipped checker has no partition
and the matrix's cell said it did.

**DOCS WERE EDITED WHILE THAT RUN WAS IN FLIGHT, AND THE READING IS DERIVED RATHER
THAN ASSUMED.** F-1.7's hazard again, so the scope was checked instead of hoped:
gate 17 reads tracked `.rs` and `.sh` files, gate 18 reads `docs/decisions.md`
(untouched during the run), and gate 19's `DOCS` list is six named files
(`tools/label_consistency_check.sh:102-107`), none of them touched. **No gate in
`tools/ci.sh` reads `docs/experiments/*.md` outside that list**, so the edits
could not reach a gate.

---

## §1b — THE RECURRING DEFECT, AND THE FIX THAT WAS PROPOSED, ATTACKED AND WITHDRAWN

**THE OPERATOR'S INSTRUCTION**: *"go into next revision and really try to get to
the bottom of this and fix it sustainable."*

**WHAT WAS PROPOSED, AND IT IS WITHDRAWN.** A CI gate executing commands written
inside Markdown documents (`derivation_gate_design.md` revision 1). Its
fresh-context REVIEW-design returned **FAIL — 11 BLOCKING, 10 MAJOR, 3 minor**,
verdict *"do not build this package"*, and **the verdict is accepted in full**.
Revision 2 of that document is the withdrawal and carries the account.

**THE THREE FINDINGS THAT DECIDED IT, EACH RE-RUN HERE BEFORE BEING ACCEPTED.**

1. **THE GATE CANNOT REACH THE DOMINANT SUB-CLASS.** Most instances are a command
   run FAITHFULLY over the wrong scope — a `git grep` scoped to
   `crates/pistol-core/src` that missed its answer in `tests`, a line count scoped
   to `src` that counted `src/bin`. A block holding such a command is **green
   forever and still wrong**: the gate certifies output-given-command and the
   defect lives in the command.
2. **THE PROPOSAL'S OWN EVIDENCE WAS FABRICATED.** Its "best evidence" was a
   self-catch — a count it wrote as 84 where its block returned 88 — explained by
   a wider pathspec reaching `docs/audit/`, `docs/research/` and `tools/`
   subdirectories. **Those two directories contribute zero hits**
   (`git grep -ohE "…" -- docs/audit docs/research | wc -l` -> `0`). A mechanism
   was written for a discrepancy without checking the mechanism. **And the block
   was already red when it landed**: 98 at that commit, 89 at the one before.
3. **THE HEADLINE NUMBER DOES NOT REPRODUCE.** 37 of 57 (65%) was really **14 of
   57 (25%)** strictly. The criterion was two criteria in one sentence, and under
   its loose half the answer is 57 of 57 — **because all three reviewers were
   forbidden `cargo`**, so every finding was necessarily established by read-only
   means. A selection artefact, not a criterion. Its own population block asserted
   12 where its printed command returns 7.

**SO THE ANALYSIS WAS WRONG IN THE SAME WAY THE DOCUMENTS IT DIAGNOSED WERE
WRONG**, which is worth more than the gate would have been.

### WHAT IS TRUE AFTER THAT

**THE DEFECT IS NOT "THE CLAIM WAS NOT CHECKED" — IT IS "THE CLAIM WAS CHECKED
AGAINST THE WRONG POPULATION."** No mechanism that executes the author's own
chosen command can detect that, because the population is the author's judgement.
**What detects it is a second party choosing a different scope**, which is how
every instance in this arc was in fact caught.

**AND THE EVIDENCE CARRIES AN ASYMMETRY THAT SAYS WHERE THE RULE BELONGS.** D-568
and D-574 addressed it to AUTHORS; the arc that wrote them broke them four more
times and this arc broke them at least nine, three inside the document restating
them. The same arc briefed three fresh contexts to re-derive and they returned
**57 findings**. A rule addressed to the author of a claim does not fire; the same
rule addressed to a fresh context does — not a difference in diligence, since both
are the same kind of agent minutes apart, but in what each is doing.

### WHAT REPLACES IT — TWO SMALL CHANGES, NEITHER A NEW EXECUTION SURFACE

1. **GATE THE CITATION CHECKER THAT ALREADY EXISTS.**
   `tools/design_citation_check.py` is committed, tested, carries a `--proposes`
   discipline, and **is on no gate path** — `wp20m_DESIGN_STOP.md:111` recorded
   that gap and nothing acted on it. **The withdrawn package proposed building a
   tool that was already in the tree, because its author did not grep for one** —
   the same defect at the level of a package. Run today over this arc's three
   governing documents it catches one thing immediately: `wp21_prereg.md` names
   `tools/wp21_assemble.py`, an instrument revision 4 registers and the tree does
   not have.
2. **MOVE THE RE-DERIVATION BRIEF INTO `docs/process.md`**, addressed to the
   reviewer, with the clause that does the work: *a count the reviewer reproduces
   only by running the document's own command is NOT reproduced.* Independence of
   scope is the property that catches this class — and it is exactly the property
   a gate executing the document's own command destroys.

### STATE

| step | state |
|---|---|
| derivation gate | **WITHDRAWN** on its review's verdict; revision 2 is the account |
| gate the existing citation checker | owed, its own package, its own review |
| the `docs/process.md` reviewer clause | owed |
| `tools/ci.sh` gate-total derivation | **dropped with the package** — a separate change that was riding in it |

### THE REPLACEMENT LANDED, AND ONE MORE FALSE CLAIM CORRECTED ON THE WAY

**1. THE CITATION GATE.** `tools/governing_citation_check.sh` names the eight
documents that GOVERN something now and runs the already-committed
`tools/design_citation_check.py` over them; `tools/ci.sh` runs it as **gate 20**.

**THE LIST IS NAMED RATHER THAN GLOBBED, AND THE REASON IS MEASURED.** Run over
every document in `docs/`, the checker reports unreproduced citations in **70 of
~245 files** — almost all RECORDS whose citations were true when written and
rotted as the tree moved. **Demanding that a record match today's tree is
demanding that history be falsified**, so the gate covers documents where a stale
citation misleads rather than remembers. Adding one is a commitment; removing one
says it has become a record.

**AND THE GATE TOTAL NOW LIVES IN ONE PLACE.** `tools/ci.sh` asserted `19`
nineteen times — D-423 at nineteen, and adding a gate meant editing all of them.
`readonly GATE_TOTAL=20`, one literal, one place. **This is NOT the withdrawn
package's self-referential `grep` derivation**, which the review showed had four
silent-miscount modes; it is one number in one place, which is all D-423 asks.
**CLAUDE.md's own sentence was false on landing** — it said the script prints
`gate N/19` — and is corrected to `gate N/$GATE_TOTAL`. **That is an edit to the
operator's file and is flagged here rather than buried.**

**2. THE RE-DERIVATION CLAUSE**, now standing in `docs/process.md` and addressed
to the REVIEWER: *a count the reviewer reproduces only by running the document's
own command is NOT reproduced.* It no longer depends on whoever writes a dispatch
prompt.

### F-1.10 — THE MATRIX'S A-PRIORI ARGUMENT WAS FALSE AT k=2, AND THE MEASUREMENT IS BETTER THAN THE ARGUMENT WOULD HAVE BEEN

Revision 2 of the cache-key matrix replaced a bad MEASURED claim with an
a-priori one: *"the book dedupes openings by `canonical_form`, so no two openings
can transpose or mirror onto each other at `k <= opening_turns`."* **Deduping the
WHOLE opening constrains nothing about its PREFIXES.**
`wp21_throughput_prereg_rev3_REVIEW.md` BLOCKING 2 refuted it with a two-second
computation, and `tools/opening_prefix_fold.py` re-takes it here:

| `k` | exact-key classes | stone-set | symmetry | (tranche one, 218 openings) |
|---|---|---|---|---|
| 1 | 1 | 1 | 1 | every prefix is the forced origin stone |
| 2 | **213** | **213** | **171** | the a-priori claim is FALSE here |
| 3 | 218 | 218 | 218 | `canonical_form`'s dedupe — the only depth it held |

**WHAT THE FOLD IS WORTH: 42 extra searches a tranche — 0.72% of its ESTIMATED
5 819 misses.** So *"the fold merges nothing"* was wrong and *"the fold merges a
great deal and is worth under one percent"* is right; only the second is a fact
about the run. **AND ALL 42 SIT WHERE THE FOLD WOULD BE WRONG** — each is a
prefix whose symmetry partner is a different position, so a symmetry key answers
it with a `bestmove` in the wrong frame. That is a better argument for the exact
key than the one the matrix made up. Receipt
`artifacts/arc3_opening_prefix_fold.txt`; matrix revision 3.

**THE PATTERN, FOR A SUCCESSOR**: revision 1 of the matrix asserted a measurement
at the wrong population, revision 2 replaced it with an argument that was never
checked, and revision 3 replaced that with the measurement. **Two of the three
revisions reached for something cheaper than running the command.**

**GATE 20 IS GREEN AND THE RECEIPT IS ITS OWN LOG.**
`artifacts/arc3_ci_gate20.txt` — **`ci: all gates passed`, EXIT=0**, **20** gate
lines, and gate 20 itself reports 112 citations checked across the eight
governing documents, 0 unreproduced. The gate prints its own disclaimer, which is
the honest half: *"A GREEN RUN MEANS THE CITATIONS ARE REAL, NOT THAT THE
DOCUMENT IS RIGHT."*

---

## §1c — ROUND 2 FAILED ON BOTH REGISTRATIONS, AND THE SIX BLOCKING FINDINGS ARE ONE STRUCTURAL FACT

| document | round | verdict | findings |
|---|---|---|---|
| `wp21_prereg.md` rev 4 | 2 | **FAIL** | 3 BLOCKING, 12 MAJOR, 5 minor |
| `wp21_throughput_prereg.md` rev 3 | 2 | **FAIL** | 3 BLOCKING, 8 MAJOR |

Both reviews record real progress — 11 of 23 and 17 of 22 round-1 findings
genuinely CLOSED, each verified against the code rather than against the sentence
claiming it — and both fail on the same thing read two ways.

**THE SIX BLOCKING FINDINGS ARE FOUR PIECES OF MACHINERY THAT DO NOT EXIST.**

| the finding | the machinery |
|---|---|
| `--label-cache` is a registered command word the binary refuses | the cache flag, `crates/pistol-arena/src/bin/arena.rs` |
| the dry run is registered and not taken; three of its commands are unrunnable | `--partition`, `--skip/--take`, `--label-cache` |
| `wp21_prereg.md` names `tools/wp21_assemble.py` and the tree has no such file | the assembly instrument |
| §1/§5 computed on a superseded revision's arithmetic | (a real defect, and the only one of the six a round 3 could fix) |

**A ROUND 3 CANNOT FIX FIVE OF THE SIX.** `docs/process.md`'s dry-run discipline
requires the literal commands exercised BEFORE the review passes, and **a command
whose flag does not exist cannot be exercised by any amount of rewriting.** The
loop grant's third round is remedies-only; there is no remedy in prose for a flag
that is not implemented.

### THE DECISION: SPLIT, AND THE CODE GOES FIRST

**ARCHITECT DEFAULT APPLIED.** This is the dispatch's own *"STOP and split"*
applied one round early and for a reason the dispatch did not anticipate: the
registrations are not failing on their reasoning, they are failing because they
register an instrument the tree does not have. The order inverts:

1. **the label cache package** — design, REVIEW-design, IMPL, REVIEW-impl,
   RED-TEAM on the cache path, mutation receipt, CI. It brings `--label-cache`
   and the two coarser-fold counters D-581 registers.
2. **`tools/cold_label_check.py --partition hits|misses|all`** — required
   argument, class named on the summary line, coverage test.
3. **`tools/wp21_tranche_config.py --skip/--take`** — T-F's sub-range config
   comes from the shipped generator like every other.
4. **`tools/wp21_assemble.py`** — needed by §6 and not by tranche one; it may
   follow the sweep.
5. **the dry runs**, both documents', taken and recorded.
6. **round 3 of both registrations**, at a revision where every registered
   command runs — which is the first revision either of them could honestly have
   been reviewed at.

**THE COST OF HAVING DONE IT THE OTHER WAY ROUND IS TWO REVIEW ROUNDS**, and it
is worth naming: a registration that registers commands is not reviewable before
the commands exist, and both of these registered four.

**RESUME POINT FOR A SUCCESSOR: step 1 above.** Nothing in the sweep starts until
6 closes.

### THE CACHE DESIGN'S REVIEW — FAIL, AND THREE FINDINGS CHANGED THE MECHANISM

Round 1 at `239f21f`: **6 BLOCKING, 12 MAJOR, 4 minor**
(`wp21_label_cache_design_REVIEW.md`). Revision 2 answers every one and rejects
one with its reproducer.

**THE THREE THAT CHANGED THE MECHANISM RATHER THAN THE PROSE.**

1. **THE MEMO WAS A PARAMETER, AND `capture::run` IS `pub`.** The registration
   registers the memo's lifetime as an invariant — constructed inside `run`,
   dropped with it, never shared. Revision 1 threaded a `&mut LabelCache` in from
   the binary, which makes that invariant a property of **every caller, including
   ones that do not exist yet**, and a REVIEW-impl cannot check it by reading the
   function. Revision 2 passes a **MODE** and builds the map inside `run`: the
   map's type appears in no signature, so there is no way to share one.
2. **NOBODY HAD SAID WHICH SIDE OF `normalise` THE MEMO SITS ON.** `run` stores
   `normalise(&totals)?` into each record, stripping ` nps <n> time <n>` — the two
   machine-dependent fields. **A memo holding the RAW totals would write those
   bytes into the 53% of records that are hits**, and §4.4's byte-identity would
   fail on a difference that says nothing about the cache. Registered: the memo
   holds the POST-`normalise` pair, so a hit reproduces the record's bytes by
   construction.
3. **TWO GUARDS INSIDE `ask` STOP RUNNING ON HITS.** `channel.unsolicited()`
   (`capture.rs:241-246`) refuses an engine that spoke before it was asked;
   skipping the ask skips the check, so a stray line lands on a LATER prefix or is
   missed. **The uncached and cached passes must refuse the same input at the same
   place** — stronger than byte-identity on well-behaved input — so the guard is
   HOISTED into `run`'s loop (X3). The census guard beside it is moot under X1 and
   is said to be rather than left unsaid.

**AND ONE FINDING IS REJECTED WITH ITS REPRODUCER.** The review said the design
never mentions `no_tab`, which the registration names as a candidate cache
defect. **`no_tab` runs in `run`, on the constructed record, outside `ask`**
(`capture.rs:355-360`): a hit skips the ask, not the record construction, so it
cannot skip the guard. The normalise half of the same finding is accepted in full
and is §2.1.

**THE CITATION GATE WOULD HAVE CAUGHT THREE OF THE REVIEW'S FINDINGS AND THE
DESIGN WAS NOT ON ITS LIST.** Revision 1's ONE LINE said *"CI gate 6"* — config
validation — in a design whose governing registration corrects that exact error
by name; `:104-105` was the comment above the gate, not the gate. **The design
joins `tools/governing_citation_check.sh`'s list in the same commit**, and the
gate immediately catches the fourth: `label_cache_tests.rs`, a file §6 proposes
and the tree does not have, now declared.

### F-1.11 — THE RUST TOOLCHAIN CHANGED UNDER THE SESSION, AND EVERY EARLIER RECEIPT IS A 1.97.1 RECEIPT

`artifacts/arc3_ci_tools.txt` returned **`ci: FAIL: tests`, EXIT=1** at gate 3, and
**the failure is not a regression**. Four `error[E0514]: found crate … compiled by
an incompatible version of rustc`, in the doctest run, against rlibs the tree had
been carrying:

```
crate `serde` compiled by rustc 1.97.1 (8bab26f4f 2026-07-14)
help: please recompile that crate using this compiler (rustc 1.98.0 (88d9e12ae 2026-08-18))
```

**`rustup` INSTALLED 1.98.0 AT 20:21 TODAY, MID-SESSION** —
`ls /home/tom/.rustup/toolchains/` dates `1.98.0-x86_64-unknown-linux-gnu` to
20:21 and the stale `libserde` rlib to 2026-08-17. The run straddled the change.

**THREE CONSEQUENCES, AND THE THIRD IS THE ONE THAT OUTLIVES THE INCIDENT.**

1. **The run is a VOID and not a failure.** `tools/SHELL_CHECKLIST.md` item 12's
   distinction: no gate answered NO, the compiler refused to adjudicate. `ci.sh`
   spells it FAIL because `cargo` exits nonzero, which is a gap in the seam rather
   than in the gates — recorded, not fixed here.
2. **Every CI receipt earlier in this session was taken under rustc 1.97.1** —
   `arc3_ci_premerge_2b94f04.txt`, `arc3_ci_dev_HEAD.txt`, `arc3_ci_gate20.txt` —
   and so is every binary digest any of them attests. They remain true statements
   about the compiler that took them, and a successor comparing a digest across
   the boundary is comparing two compilers.
3. **D-577's "rebuild means re-record" bites before the package that coined it
   exists.** The codegen-tuning package is not scheduled and the toolchain moved
   anyway. **A pinned instrument binary must reproduce on any host, and it does
   not reproduce across a compiler change** — so the sweep's own
   `binary_sha256` slot (`wp21_prereg.md` §8) must be filled AFTER the toolchain
   settles, and the run log must name the `rustc --version` beside it. That is an
   amendment the registration owes and did not have.

### THE CACHE DESIGN, ROUND 2 — FAIL, AND ONE FINDING IS A CORRECTION TO WHAT THIS SESSION TOLD THE OPERATOR

Round 2 at `fde1497`: **4 BLOCKING, 7 MAJOR, 4 minor**, with round 1's 22
findings dispositioned **7 CLOSED, 6 PARTIAL, 9 OPEN**
(`wp21_label_cache_design_rev2_REVIEW.md`).

**REVISION 2's HEADER CLAIMED EVERY FINDING WAS DISPOSED OF AT THE SECTION THAT
OWNS IT, AND `git diff 239f21f fde1497` REFUTES IT**: §4 and §5 have **no hunks at
all**. The claim was about the document and the diff was one command away.

**THE FOUR BLOCKING, and three are things revision 2 never touched.**

1. **§2's body still said "CI gate 9 of 19 (`tools/ci.sh:104-105`)"** —
   byte-identical to revision 1. Revision 2 fixed the ONE LINE and printed the
   correction in its own header, four sections above the uncorrected sentence.
2. **§6's T2 named an instrument that does not exist** — *"measured from the
   stub's own count"*, and `stub_engine.rs` has no counter. **T2 is the only row a
   dead cache fails**: T1 and T6 pass because a cache that is parsed and dropped
   yields the uncached bytes, T3 is a CLI arm, T4's guard is unconditional, T5's
   counters are unconditional — and the registration's fallback,
   `tools/label_cache_count.py`, reads the capture file and returns the same
   number cached or uncached **because the file is identical by construction**.
   Revision 3 makes the ask count a registered OUTPUT: `asks < records` cached,
   `asks == records` uncached.
3. **§1's change list had dropped the lookup and the insert** while §5 went on
   mutating them as sites.
4. **§5 was unchanged**, so the guard revision 2 ADDED (X3) and the seam it added
   (the mode) had no registered mutants — D-553's own class.

### F-1.12 — "42 PER TRANCHE" IS TRANCHE ONE'S NUMBER, AND D-583 CARRIES THE ERROR

The reviewer re-implemented `canonical_form` in Python rather than running this
arc's instrument, confirmed tranche one's 213 -> 171, and then **ran it over all
sixteen**: `42, 51, 48, 55, 47, 48, 50, 44, 53, 48, 51, 60, 45, 56, 48, 46`, sum
**792**. Re-derived here with the committed instrument and identical. **42 is the
MINIMUM across the sixteen and the value at exactly one of them.** A closure
reading a tranche's `key_full` counter against 42 would read fourteen tranches as
discovering between 2 and 18 collisions that are the opening book. Corrected in
the receipt's addendum, in the design, and by **D-584**.

### F-1.13 — THE CLAIM THAT GATE 20 WOULD HAVE CAUGHT THE CITATION ERRORS IS FALSE, AND THIS SESSION TOLD THE OPERATOR IT

`tools/design_citation_check.py` refuses a path the tree does not hold and a line
number past end-of-file. **`tools/ci.sh:104-105` is in range in a 205-line file**,
so the gate returns **exit 0** on the very citation the design got wrong. The
withdrawn design's own §4 had said so — *"It would not have caught
`outpath.rs:9-24` (in range, wrong function)"* — so the claim contradicted a
limitation this arc had already written down, in the sentence arguing for the
gate's value.

**WHAT THE GATE ACTUALLY BUYS IS ROT AND NOTHING ELSE**: a path or line that
MOVED. It caught one real thing on landing and will catch more as the tree moves;
it catches no wrong-but-in-range citation, ever. **D-582's reviewer clause is the
only thing that does**, which is the whole reason the clause and not the gate is
the answer to the class.

### F-1.14 — THE TOOLCHAIN'S SECOND BITE: A NEW CLIPPY LINT MADE EIGHT SITES RED, ONE OF THEM THE FIPS-PINNED SHA-256

The clean rebuild under rustc 1.98.0 reached **gate 4** and returned
`ci: FAIL: clippy`. Not a regression either: **`clippy::chunks_exact_to_as_chunks`
is new in 1.98** and refuses `chunks_exact` with a constant chunk size. Eight
sites, in code that was clippy-clean an hour earlier:

```
crates/pistol-core/tests/common/sha256.rs      x3
crates/pistol-solver/tests/common/sha256.rs    x3   (D-37's deliberate duplicate)
crates/pistol-cli/src/sha256.rs                x2
crates/pistol-arena/src/score.rs               x2
crates/pistol-arena/src/transcript.rs          x1
crates/pistol-arena/tests/scoring_tests.rs     x1
crates/pistol-cli/tests/wp16_warm_attribution_check_tests.rs  x1
```

plus `clippy::useless_format` at `crates/pistol-arena/tests/replay_tests.rs:137`.

**THE MAPPING IS EXACT AND THAT IS WHY IT IS SAFE.** `as_chunks::<N>()` returns
the pair `chunks_exact` splits into — whole chunks, then the tail — so
`pistol-cli`'s streaming digest keeps `remainder()`'s meaning as the tuple's
second element, and `score.rs`'s deliberately dropped odd trailing game is still
dropped. **CHECKED RATHER THAN ARGUED**: both SHA-256 implementations still match
their published FIPS vectors, and the streaming one still passes
`a_streamed_digest_does_not_depend_on_where_the_pieces_were_cut` — the test D-572
landed for exactly this kind of edit.

**WHAT IT COSTS THE ARC** is one more instance of F-1.11's lesson: *"clippy clean
is mechanical law"* (CLAUDE.md) is a law against a MOVING standard, so a green CI
receipt is a statement about a toolchain as much as about a tree. The run log's
`rustc --version` line, registered this session in `wp21_prereg.md` §8, is the
thing that makes an old receipt readable rather than merely old.

### F-1.15 — GATE 17 READS THE INDEX, NOT THE DISK, AND AN UNSTAGED FIX READS AS NO FIX

The `--partition` tests pushed `cold_label_check_tests.rs` past rule 9's 300-line
soft cap, and gate 17 refused it: *"over the cap with no entry in
`docs/rule9_justifications.md`"*. The entry was written — and the gate went on
refusing.

**BECAUSE THE GATE READS THE REGISTRY FROM THE GIT INDEX**:
`tools/file_justification_check.sh:205` is `git cat-file blob ":$REGISTRY"`, and
`:245` walks `git ls-files -s`. That is deliberate — the gate answers about the
TRACKED set, which is what a reviewer and CI both care about — but it means **an
unstaged remedy is invisible to the instrument checking it**, and the refusal
message names the registry rather than the index, so it reads as *"you did not
write the entry"* when what happened is *"you did not stage it."*

`git add -A` and the gate turns: **359 tracked `.rs`/`.sh` files, 67 over the cap,
all registered.**

**IT IS THE SESSION'S THIRD ENVIRONMENT-SHAPED FAILURE IN A ROW** — a toolchain
that moved (F-1.11), a lint that arrived with it (F-1.14), and a gate whose
subject is the index while the author's subject is the disk. None is a defect in
the code under test, and all three cost a CI round.
