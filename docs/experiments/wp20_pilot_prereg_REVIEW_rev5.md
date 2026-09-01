# SCOPED RE-CHECK — `docs/experiments/wp20_pilot_prereg.md` revision 5

## Header

- **Revision adjudicated**: `972cdd1b88fec8d5b5a100a315f523b042fe3823` (branch `dev`).
- **Matches HEAD**: YES. `git rev-parse HEAD` → `972cdd1b88fec8d5b5a100a315f523b042fe3823`.
- **Tree state**: `git status --porcelain` is **EMPTY**.
- **Scope**: the 11 findings of `docs/experiments/wp20_pilot_prereg_REVIEW_rev4.md`
  (revision 4 = `950cad1`, FAIL, 0 BLOCKING / 5 MAJOR / 6 MINOR); MAJ-E′, the finding
  the author added by running his own remedy; D-555's recurrence class; §6.3's depth
  table and wall arithmetic against SLOT A; `docs/experiments/wp20_pilot_artifacts.md`;
  and anything new.
- **Round**: four of the four D-552 allows. **This is the last one, and a fifth
  failure returns the package to the architect.**
- **Reviewer**: fresh context. I wrote none of this and owe it no deference. Every
  verdict below is derived from `git diff 950cad1..972cdd1`, from the document as it
  now reads, and from the artifacts and the tree — **never** from §0.1's change table
  (D-550).

**What I read.** `docs/experiments/wp20_pilot_prereg.md` (all 1 159 lines);
`docs/experiments/wp20_pilot_prereg_REVIEW_rev4.md` (all 610);
`git diff 950cad1..972cdd1` in full; `docs/experiments/wp20_pilot_artifacts.md` (all
59); `artifacts/wp20pilot_dryrun_31c1cc1_v1.txt` (all 98 lines) and the three earlier
dry-run artifacts `..._071c65c_v1.txt`, `..._85e6261_v1.txt`, `..._f297eab_v1.txt`;
`crates/pistol-arena/src/bin/arena.rs` (`main`, `dispatch`, `count_of`, `workers_of`,
`replay_pass`, `run`); `crates/pistol-arena/src/passes.rs` (all 86);
`crates/pistol-arena/src/outpath.rs`; `crates/pistol-arena/src/error.rs` (the `Io`
variant and every `ArenaError::io(` site); `crates/pistol-arena/src/config.rs`
(`load`, `parse_unvalidated`); `crates/pistol-arena/src/identity.rs`
(`digest_of`, `capture`); `crates/pistol-arena/src/capture.rs` (`asked_prefixes`,
`normalise`, `run`'s walk); `crates/pistol-arena/src/bin/corpus-check.rs`
(`summarise`, `median`, `printable`, `main`); `crates/pistol-arena/src/labels_file.rs`
(the four token sets and the column order); `tools/cold_label_check.py` (its exit
discipline); `configs/arena_wp20_label_pilot.toml` at `31c1cc1`; `docs/decisions.md`
D-552–D-555; `CLAUDE.md`; `docs/process.md`.

**What I ran** (read-only; **no `cargo` in any form and no `tools/ci.sh`**):

- `git rev-parse HEAD`, `git status --porcelain`, `git log --oneline`, per-commit
  `git show --stat` for `071c65c`, `31c1cc1`, `972cdd1`
- `git diff --stat 31c1cc1 972cdd1 -- crates/ tools/ configs/` → **EMPTY** (the SLOT R1
  check §1 registers); the same over `950cad1..972cdd1` → **EMPTY** (no code moved this
  round); `git diff --stat 6e1fea3 972cdd1 -- crates/ tools/ configs/` → **NOT empty**
  (see MIN-5)
- a `python3` recomputation of all 17 manifest digests with `hashlib`
- a `python3` re-derivation of the depth table from the four `dryrun4` corpora,
  parsing column 10 with code sharing nothing with `corpus-check`, plus the observed
  value set of every closed-set column
- a `python3` re-derivation of `T`, the wall arithmetic and every row of RULE-1's
  sensitivity table
- `/usr/bin/grep` and `git grep` throughout (D-265)

**Where a build would settle a claim.** No finding below rests on one. The claims I
did not settle by execution are that the workspace compiles clean and that
`crates/pistol-arena/tests/labels_tests.rs` passes at `972cdd1`; the run that would
settle them is `tools/ci.sh` in a detached `git worktree add --detach` on `/home` with
its own `CARGO_TARGET_DIR`. See **MIN-5** — no such receipt is committed for this
revision's code.

---

## VERDICT: **FAIL**

| severity | count |
|---|---|
| **BLOCKING** | **0** |
| **MAJOR** | **6** |
| **MINOR** | **7** |

**Every one of the eleven findings was answered, MAJ-E′ was found by the author's own
remedy and is genuinely fixed, and the two things I was asked to recompute from
scratch both reproduce exactly.** The depth table, the four medians and means, `T = 13`,
every row of the sensitivity table, 32.5 %, 65 %, 253.5 T and all 17 manifest digests
are correct. `git diff --stat 31c1cc1 972cdd1 -- crates/ tools/ configs/` is empty, so
SLOT R1 is right this time; SLOT A now carries the revision line, the dirty-file count
and a digest for all four instruments; and its `corpus_check:` lines carry the `book`
field §4E registers, which is what MAJ-A and MAJ-E′ existed to force.

**It fails because D-555's class is still alive and the sweep did not catch it.** The
author's literal-string sweep found one residue; I found seven more, and four of them
are in the two paragraphs the sweep was aimed at. Two are arithmetic that the document
states and its own inputs refute (`MAJ-1`, `MAJ-4`); one is a bound the document now
carries three incompatible versions of (`MAJ-2`); two are citations to a printed line
and a digest that are not in the artifact the document names, and the digest was TRUE at
revision 4 and was spent by this round's own re-take of the dry run (`MAJ-3`, `MAJ-6`);
and one is a totality claim that revision 5 strengthened past what it can support, which
now misclassifies the single most reachable exit `2` in the whole pilot (`MAJ-5`).

---

## DISPOSITION OF THE 11 PRIOR FINDINGS

| # | finding | disposition | evidence now standing |
|---|---|---|---|
| **MAJ-A** | MIN-8's `book` fix changed a registered instrument at the commit whose §1 said none changed | **APPLIED BUT INTRODUCED A NEW DEFECT** | SLOT R1 is `31c1cc1` in all ten instrument rows (`:110-121`), in §4A/§4B/§4C/§4E and in §9.1's header. `git diff --stat 31c1cc1 972cdd1 -- crates/ tools/ configs/` is **empty** — the check §1:132-135 now registers on itself. `git show 31c1cc1:.../corpus-check.rs` prints `book`, SLOT A lines 39/47/55/63 carry `book 2 (no,yes)`, and my independent parse of all four corpora returns `book ∈ {no,yes}` at every budget, so §4E's registered line is producible at the registered revision. The finding's substance is dead and the remedy went past the ask: SLOT R1 became a slot with a mechanical self-check rather than a transcribed constant. But re-taking the dry run twice falsified two receipts elsewhere that were true of the old one — §7.1:871's capture digest (**MAJ-6**) and §6.3:723's "SLOT A's predecessor" (**MAJ-2**) — and added a duplicated sentence at `:137-139` (**MIN-3**). |
| **MAJ-B** | the coldness reconciliation asserted agreement its own arithmetic refuted | **APPLIED BUT INTRODUCED A NEW DEFECT** | `:911-923` retracts revision 4's reconciliation by name and rebuilds it on quantisation, which is the right frame. The new arithmetic is wrong in the same direction — it states an interval one of its two readings excludes, and derives a bound twice the one both support. **MAJ-1.** And §6.3:718 still carries the tighter claim the new paragraph retracts. **MAJ-2.** |
| **MAJ-C** | the `iff` made a mid-pass ENOSPC a STOP while the void class called it a VOID | **APPLIED** | `:495-506` makes limb (c) independent of where the failure lands and decides it by WHAT failed. I verified the named instance: `passes.rs:48` and `:78` render the whole output in memory and write once, so a disk-full in pass 2 or 3 raises `ArenaError::io("writing <path>")` → limb (c) → VOID, which is what `:497` says. The contradiction with `:496`'s bullet is gone. One residue at the OOM-kill boundary (**MIN-2**). |
| **MAJ-D** | the enumeration was not exhaustive; §5 carried two lists of one class | **APPLIED BUT INTRODUCED A NEW DEFECT** | All four named gaps are closed at `:455-476`: the command line (bullet 1), `outpath::claim` "for ANY reason" (bullet 2), the source report "absent" (bullet 3), and `--labels`' own capture read (bullet 6). The second list is gone; there is now one. But revision 5 broadened the claim from *"a capture or labels run exiting `2`"* to *"an exit `2` from ANY INSTRUMENT of this pilot … disjoint and exhaust the code"*, and pass 1's own exit-`2` refusals are in none of the three limbs. **MAJ-5**, and **MIN-1**. |
| **MAJ-E** | §8's block emitted no revision or engine receipt; "two ways and no others" was false | **APPLIED BUT INTRODUCED A NEW DEFECT** | The receipt is in the registered block at `:970-987` and is unconditional and first. "and no others" is gone, and `:1079-1082` says why. Its replacement — *"The two blocks differ in three ways"* — is a new false count: the dry run also suppressed `arena`'s capture and labels stdout, which the registered block prints, and that fourth difference is what makes §6.3's and §7.1's `captured 164 position(s)` citations false. **MAJ-3.** |
| **MIN-a** | "Both blocks time every pass" false — `labels2` unbracketed | **APPLIED BUT INTRODUCED A NEW DEFECT** | `:1009-1011` brackets `labels2`, and SLOT A line 78 carries `labels2-transform seconds=0`. The sentence was kept verbatim at `:1078` and is still false of SLOT A: six `--labels` runs, two timed; four `load_*` runs, none timed. **MIN-4.** |
| **MIN-b** | §6.3's arithmetic block called the transform MEASURED | **APPLIED** | `:732` reads *"two corpus transforms BOUNDED at under a second each"*; `:736-740` reads *"Every term is timed rather than asserted … Five of the six are positive measurements and the transform is a bound"*. `:686` matches. I checked all six occurrences of the count (`:524`, `:675`, `:738`, `:813-814`, `:880-881`) and they agree. |
| **MIN-c** | the manifest said the dry run is `tools/`-free | **APPLIED** | `wp20_pilot_artifacts.md:14-16`: *"**It is not `tools/`-free** — §8 runs `tools/cold_label_check.py` for criterion C-A"*, with the reason (it is a governed instrument). The remedy is better than the ask. |
| **MIN-d** | §9 and §9.1 read a MAXIMUM off a mean | **APPLIED BUT INTRODUCED A NEW DEFECT** | Both rows now say mean-and-bound and both say *"a quantity nothing here measured"*. The bound they chose is 165 s and the value they clear it against is 120 000 ms; 120 s does not exceed 165 s, so §9's registered slot check is FAILED by the committed value while §9.1 records *"yes"* and §9.1's closing paragraph records *"NO CORRECTION WAS NEEDED"*. **MAJ-4.** |
| **MIN-e** | the manifest omitted the two C-E injection corpora | **APPLIED** | `wp20_pilot_artifacts.md:33-34` indexes `corpus_grammar.txt` and `corpus_digest.txt`, with `:47-50` stating why a criterion's INPUTS are its evidence. Both digests recompute. A third row was added beyond the ask (`corpus_400000_c.txt`, the second timed transform) and it recomputes too. |
| **MIN-f** | "three decided games" was not arithmetic | **APPLIED** | `:613-620` replaces the number with the mechanism. I verified it against `capture.rs:27-43`: `asked_prefixes` yields `0..=len` less the terminal prefix when the game is decided, so a capped game gives `turn_cap + 1 = 41` and a game decided at turn `k` gives `k`. The worked numbers check: cost `41 − 20 = 21`, four of them 84 against a slack of 66; cost `41 − 39 = 2`, four of them 8. |

**Counts**: 5 APPLIED, 6 APPLIED-BUT-INTRODUCED-A-NEW-DEFECT, 0 PARTIALLY APPLIED,
0 NOT APPLIED. **Every finding was answered on its own terms and none was skipped or
softened; six of the eleven remedies broke something adjacent**, which is the same ratio
as the previous round and the reason for the verdict.

### MAJ-E′ — the author's own finding, verified in all three parts

The account at `:852-866` and `:975-982` is true and I checked every limb of it.

**(a) The receipt builds before it digests, and digests every binary the block runs.**
`:983` is `cargo build --release --workspace --locked || { echo "build failed"; exit 2; }`;
`:984` prints `git rev-parse HEAD` and `git status --porcelain | wc -l`; `:985-987` loops
over `"$P"`, `"$A"` and `target/release/corpus-check`; `:988` digests
`tools/cold_label_check.py`. That is every instrument §1 names that is not a tracked
source file. The comment gives the true reason — a clean tree is a fact about SOURCES
and a digest is the fact about the BUILD.

**(b) SLOT A carries those lines.** `artifacts/wp20pilot_dryrun_31c1cc1_v1.txt` lines
11–15 are the revision line, three `binary … sha256` lines and the script digest. The
predecessor's failure is real and reproducible from disk: `..._071c65c_v1.txt` line 2 is
`engine sha256 …` alone, and its four `corpus_check:` lines (26, 34, 42, 50) end
`to_move 2 (p1,p2); result 1 (capped); end 1 (normal)` with **no `book` field** — a
passing receipt over a stale loader, exactly as described.

**(c) The digests are consistent with what `31c1cc1` builds, by the test available
without a rebuild.** The artifact's revision line names `31c1cc187deba0375aa701cee8a467c46af5a0a7`,
and `git diff --stat 31c1cc1 972cdd1 -- crates/ tools/ configs/` is **empty**. SLOT R1 is
right. (I cannot rebuild and do not claim the digests are the ones a rebuild would
produce; the check the scope asked for passes.)

---

# MAJOR

## MAJ-1 — §7.2's replacement coldness arithmetic states an interval one of its two readings excludes, and the bound it registers is twice the one the two readings support

**File**: `docs/experiments/wp20_pilot_prereg.md:915-923`.

**What the document says**:

> The honest statement is about QUANTISATION: each pass is timed to the second, so a
> DIFFERENCE of two such readings carries plus or minus two seconds. At `200000` the
> difference is `85 - 83 = 2 s`, which is `2 ± 2`; at `400000` it is `165 - 165 = 0 s`,
> which is `0 ± 2`. **Both intervals contain everything from 0 to about 4 s over 164
> asks, so this instrument bounds the coldness overhead at roughly 24 ms per ask and
> resolves it no further.**

**What is wrong.** The quantisation model is right and the two intervals are right.
The sentence that spends them is false. `2 ± 2` is `[0, 4]`. `0 ± 2` is `[−2, +2]`, and
it does **not** contain 3 or 4. "Both intervals contain everything from 0 to about 4 s"
is false of the second interval, and the second interval is the one taken **at the
chosen budget** — the reading §6.3:713-717 calls the answer and the other one the
superseded predecessor.

The bound the two readings jointly support is their intersection: `[0, 4] ∩ [−2, 2] =
[0, 2]`, i.e. **at most about 2 s over 164 asks ≈ 12 ms per ask**. The `400000` pair
alone gives the same answer. `24 ms` is `4 s / 164` and is reachable only by attributing
to the `400000` reading an interval it excludes — which is what the false sentence
does. The registered bound is loose by a factor of two, in the direction that makes the
coldness cost look larger, and it is derived by the same move the paragraph opens by
retracting: taking one reading's headroom and asserting the other agrees.

**And its only quantitative input is not in any artifact this package indexes.** `85`
and `83` are `cold seconds=85` and `capture_200000 seconds=83` in
`artifacts/wp20pilot_dryrun_f297eab_v1.txt` — the artifact §7.1:854-857 disqualifies
because it *"ran against an uncommitted working-tree edit while its filename attributed
it to a revision that could not have produced it"*, D-479's class. SLOT A's own
`200000` capture reading is `84`, not `83` (line 52), and SLOT A has no cold reading at
`200000` at all. So `wp20_pilot_artifacts.md:5-6`'s opening claim — *"Every number
`docs/experiments/wp20_pilot_prereg.md` reads off a run is read off one of the files
below"* — is false of the two numbers on which the document's only registered coldness
bound rests, and the file they are read off is not indexed and was declared unusable.

**Why it matters.** §7.2's three findings are, by the section's own words, *"recorded
here because they bound what the closure may conclude"*. This is the one that carries a
number, and D-542 and `docs/experiments/wp20m_design.md` §12 both leave that number
open. A closure reading `24 ms/ask` reads a bound derived by an arithmetic error out of
a disqualified artifact, when the correct bound — `12 ms/ask`, from SLOT A alone — is
tighter, is a better answer for this document, and is fully supported by the artifact
the manifest does index.

**How I reproduced it.** `sed -n '905,925p' docs/experiments/wp20_pilot_prereg.md`;
`/usr/bin/grep -n "cold seconds\|capture_200000 seconds" artifacts/wp20pilot_dryrun_*.txt`;
arithmetic: `[0,4] ∩ [−2,2] = [0,2]`; `2/164 = 0.0122`; `4/164 = 0.0244`.

**Minimal remedy.** Replace the final sentence with: *"The `400000` reading is the
tighter of the two and it is at the chosen budget: `0 ± 2 s` over 164 asks bounds the
coldness overhead at about 12 ms per ask, and the `200000` pair is consistent with that
and no tighter. The `200000` figures are read off the dry run superseded at §7.1(i) and
are carried as a prior, not as evidence."* Then index that artifact in the manifest or
delete the `200000` sentence; a number quoted from an unindexed, disqualified run is
the class §7.1 exists to name.

## MAJ-2 — §6.3 still registers the coldness overhead as "below one-second resolution", which is exactly the reading §7.2 was rewritten to retract

**Files**: `docs/experiments/wp20_pilot_prereg.md:716-723` against `:915-923`.

**What §6.3 says**:

> **The coldness overhead is therefore below this instrument's one-second resolution**,
> which is a MEASURED answer to the cost `docs/experiments/wp20m_design.md` §12
> declines to guess … (The earlier `200000` reading put it at 2.4 %; **both are in SLOT
> A's predecessor** and neither changes a value here.)

**What is wrong.** Two things, and both are D-555's class — a remedy correct where it
was written and false somewhere else.

1. **The bound contradicts §7.2's.** "Below this instrument's one-second resolution"
   is `< 1 s / 164 = 6 ms per ask`. §7.2:919 registers *"roughly 24 ms per ask"*, and
   its whole argument is that a reading of `0` on an integer counter does **not** mean
   the true difference is under a second — *"a DIFFERENCE of two such readings carries
   plus or minus two seconds"*. §6.3 is the inference §7.2 was rewritten to withdraw,
   left standing two hundred lines above it. The document now carries three different
   bounds on one quantity: 6 ms (§6.3), 24 ms (§7.2), and the 12 ms both readings
   actually support.

2. **"SLOT A's predecessor" is now the wrong artifact.** SLOT A is
   `..._31c1cc1_v1.txt`; its predecessor is `..._071c65c_v1.txt`, whose `200000`
   capture reads `84` and which has no cold reading at `200000`. The 2.4 % pair lives in
   `..._f297eab_v1.txt`, three artifacts back. The pointer was true when SLOT A was
   `85e6261`; MAJ-A's and MAJ-E′'s re-takes falsified it, twice, and the sweep did not
   see it.

**Why it matters.** §6.3 is the section that fixes SLOT S2 and SLOT W, and it is where
a closure looks for a measured per-unit cost. The two statements license different
conclusions about the same quantity — one says the overhead is unmeasurably small, the
other that it is bounded but real — which is precisely D-424's test for a distinction
that does work, applied to two claims that cannot both be kept.

**How I reproduced it.** `sed -n '713,724p;915,923p' docs/experiments/wp20_pilot_prereg.md`;
`/usr/bin/grep -n "cold seconds\|capture_200000 seconds" artifacts/wp20pilot_dryrun_*.txt`.

**Minimal remedy.** Delete `:716-723`'s conclusion and its parenthetical and replace
with a pointer, per D-423: *"`l` and `c` are equal at the chosen budget to this
instrument's resolution; what that does and does not bound is §7.2's finding 3, which
owns the number."*

## MAJ-3 — §6.3 and §7.1 both cite a printed line that is not in SLOT A, and it is the citation for `p = 41`, the value that fixes the slice

**Files**: `docs/experiments/wp20_pilot_prereg.md:675`, `:681`, `:870` and `:1074-1078`,
against `artifacts/wp20pilot_dryrun_31c1cc1_v1.txt`.

**What the document says.** §6.3:675 heads its table *"**THE SIX MEASURED COSTS**, every
one from that artifact's own printed lines"* — "that artifact" being SLOT A — and
`:681` gives `p`'s source as:

> | `p` | **41** positions per game | `captured 164 position(s) from 4 game(s)` |

§7.1:870 repeats it as a receipt: *"`captured 164 position(s) from 4 game(s)` at each
candidate"*.

**What is wrong.** `/usr/bin/grep -n "captured" artifacts/wp20pilot_dryrun_31c1cc1_v1.txt`
returns **nothing**. The string appears in exactly one artifact on disk —
`..._f297eab_v1.txt`, lines 20, 26, 32, 38, 44 — which is the run §7.1:854-857
disqualifies. The `85e6261` and `071c65c` artifacts do not carry it either.

The cause is a difference between the two blocks that §8 does not declare. `passes.rs:48-56`
prints three lines to stdout after every capture — `arena: captured … position(s) from
… game(s) at <go>`, the `capture_manifest` row, and `arena: capture written to …` — and
`passes.rs:78-84` prints three more after every labels pass. §8's registered block
redirects nothing, so the pilot will print all six; the dry run's block suppressed them,
which is why SLOT A shows only `capture_50000 exit=0` / `capture_50000 seconds=24` where
`..._f297eab_v1.txt` shows the full stdout. That is a **fourth** way the two blocks
differ, and `:1074-1076` says *"The two blocks differ in three ways"* and enumerates
three.

**Why it matters.** `p = 41` is the value floor (b) is applied to; it fixes `T = 13`,
hence SLOT S1, hence §2's consumed range, hence the whole wall. Its stated evidence is a
line that is not in the artifact the document names, and the only place it *is* printed
is a run the document declares cannot be relied on. The VALUE survives — SLOT A line 62
reads `ok, 164 record(s)` and line 18 reads `n 4`, so `164 / 4 = 41` is derivable from
SLOT A — but the citation is false and §6.3:675's "every one" with it. This is the same
shape as revision 4's MAJ-E, one revision later: a claim about what the registered block
and the registered artifact contain, contradicted by both.

**How I reproduced it.** `/usr/bin/grep -n "captured\|position(s)" artifacts/wp20pilot_dryrun_31c1cc1_v1.txt`
(no match) against the same over `..._f297eab_v1.txt` (five matches);
`sed -n '44,60p;62,86p' crates/pistol-arena/src/passes.rs`;
`sed -n '673,682p;868,872p;1070,1082p' docs/experiments/wp20_pilot_prereg.md`.

**Minimal remedy.** Re-cite `p` from lines SLOT A does carry — *"`ok, 164 record(s)`
(C-E's control) over `n 4` (pass 1's summary)"* — correct §7.1:870 to the same, and add
the fourth difference to §8's enumeration: *"and the dry run discarded `arena`'s own
capture and labels stdout, which the block above keeps."*

## MAJ-4 — §9 registers a slot check the committed `hang_timeout_ms` fails, and §9.1 records that it passed

**Files**: `docs/experiments/wp20_pilot_prereg.md:1105` and `:1124`, against
`configs/arena_wp20_label_pilot.toml:44` and SLOT A line 60.

**What the document says.** §9:1105 registers the check:

> The slot pass confirms it **exceeds the dry run's whole capture pass** — which bounds
> any single ask inside it — with room, and a value that does not **is corrected before
> any game**.

§9.1:1124 records the answer:

> `run.hang_timeout_ms` | `120000` | … **yes** … A single ask cannot exceed the pass it
> sits in, so **165 s is the hard upper bound on the slowest one**, and the watchdog at
> 120 000 ms **clears even that**.

**What is wrong.** `120000 ms = 120 s`. The dry run's capture pass at the chosen budget
is `capture_400000 seconds=165` (SLOT A line 60), which the row itself names as the
bound. **120 s does not exceed 165 s.** The row asserts an inequality its own two
numbers reverse, and §9.1's closing paragraph then states *"NO CORRECTION WAS NEEDED AT
THIS PASS"* — over a check that, read as written, requires the value to be corrected
before any game.

The check is also unsatisfiable at any sane value, which is the deeper defect. The bound
it uses is the whole capture pass, and the pilot's capture pass is 26 games × 41
positions × 1.006 s ≈ **1 073 s**. A watchdog required to exceed that would have to be
about eighteen minutes — for a per-ask timeout on an ask whose measured mean is 1.006 s.
The remedy for MIN-d traded a quantity nobody measured (a maximum) for one that is
measured but is not a bound on the right thing.

**Why it matters.** §9 is the D-427 instrument, and D-427's whole motivating instance is
a config value that had drifted from the prose governing it with no gate to catch it.
Here the gate is present, it is registered, its own recorded answer contradicts it, and
the table says `yes`. The pilot will not in fact hang — the real per-ask cost is ~1 s and
120 s is ample — so no measurement is corrupted. What is defeated is the check.

**How I reproduced it.** `/usr/bin/grep -n "hang_timeout_ms" configs/arena_wp20_label_pilot.toml`
→ `120000`; `sed -n '60p' artifacts/wp20pilot_dryrun_31c1cc1_v1.txt` → `capture_400000 seconds=165`;
`sed -n '1105p;1124p' docs/experiments/wp20_pilot_prereg.md`. Watchdog semantics
confirmed per-ask at `capture.rs:255-262` (`ask(..., transcript.hang_timeout_ms, ...)`).

**Minimal remedy.** State the bound the instrument can actually carry. §9: *"The slot
pass confirms it exceeds the dry run's MEAN label ask by at least two orders of
magnitude; no per-ask maximum was measured, and the pass-level total is not a bound a
per-ask watchdog can be checked against."* §9.1: *"yes — the measured mean label ask is
1.006 s (`165 s / 164`), so 120 000 ms leaves two orders of magnitude; no per-ask
maximum was measured and none is asserted."*

## MAJ-5 — §5's new totality claim covers "any instrument of this pilot", and pass 1's own exit-`2` refusals fall in none of its three limbs — so the complement rule makes a stale `binary_sha256` a STOP

**File**: `docs/experiments/wp20_pilot_prereg.md:449-476` and `:485-491`, against
`crates/pistol-arena/src/bin/arena.rs:175-200` and `crates/pistol-arena/src/config.rs:197-204`.

**What the document says.** `:450-452`:

> **An exit `2` from any instrument of this pilot is exactly one of three things**, and
> the three are **disjoint and exhaust the code**

with (a) the pre-ask document refusals (VOID), (b) *"The complement of (a) among the
program's own refusals"* (STOP, V7-B), and (c) the environment, whose operative test is
*"an `ArenaError::Io` naming a failed read or write"* (VOID).

**What is wrong.** Revision 4 scoped the enumeration to *"a capture or labels run
exiting `2`"*. Revision 5 broadened it to every instrument — which includes §1's row 1,
`arena --config`. `run()` returns `Err` — and so exit `2` — from four sites before any
game, and none of them is in limb (a), none is an `ArenaError::Io`, and so limb (b)'s
complement rule sweeps all four into V7-B, **STOP the arc**:

1. **`ArenaConfig::load` (`arena.rs:180`)** — a missing config, non-TOML, an unknown
   field, a `validate` refusal. `config.rs:198-200` maps even a missing file to
   `ArenaError::config`, **not** `Io`, so limb (c) does not reach it.
2. **`openings::load` (`arena.rs:182-187`)** — an unreadable openings fixture, a take
   past the end of the book, a body digest that does not agree.
3. **`identity::capture` (`arena.rs:198-200`)** — a `binary_sha256` that does not match
   the built binary raises `ArenaError::EngineBinaryDigestMismatch`, a variant of its
   own.
4. **The handshake inside `seats::with_seats`** — an engine that does not speak.

Case 3 is not hypothetical: **§9.1:1128 names it as the one slot that cannot be
discharged before launch** — *"`binary_sha256` … re-read at the run's own launch,
immediately before pass 1, and corrected there if the binary was rebuilt"*. A rebuild
between the slot pass and pass 1 is the single most reachable exit `2` in this pilot,
and the classification sends it to V7-B, whose consequence is *"the arc STOPS"*. §5's
verdict table has no row for it either: V7-A is pass 1 exiting `1`, V7-B is
*"`--capture` or `--labels` exits `2`"*, and V8 is *"see below"*.

Limb (b) is also internally split on these: its heading is *"THE PROGRAM REFUSED
SOMETHING IT FOUND MID-WALK"*, its definition is *"the complement of (a)"*, and a
config refusal satisfies the definition while contradicting the heading. That is the
same ambiguity MAJ-D's remedy was meant to remove, relocated.

**Why it matters.** The document's own instruction is *"A refusal a reader cannot place
in that classification is itself a finding about this document"*, and this is a refusal
a reader **can** place, wrongly. V7-B and V8 have opposite consequences — return the
package versus fix the config and re-run once — and the fault here is "I could not
look", not "the pipeline is wrong". The error direction is conservative, so it cannot
turn a bad run into a good verdict; it can turn a one-line config correction into a
returned package.

**How I reproduced it.** `sed -n '175,200p' crates/pistol-arena/src/bin/arena.rs`;
`sed -n '197,220p' crates/pistol-arena/src/config.rs`; `sed -n '44,70p'
crates/pistol-arena/src/identity.rs`; `/usr/bin/grep -rn "ArenaError::io(" crates/pistol-arena/src/`
(eleven sites, none of them these); `sed -n '445,476p;485,491p' docs/experiments/wp20_pilot_prereg.md`.

**Minimal remedy.** One bullet on limb (a) and one clause on limb (b). Add to (a):
*"for `--config` — the arena config could not be read, parsed or validated, the openings
fixture could not be loaded, or a seat's binary does not digest to its `binary_sha256`
(`config.rs`, `openings.rs`, `identity::capture`); all are decided before the first
game."* And make (b)'s definition agree with its heading: *"the complement of (a) among
the program's own refusals, all of which arise mid-walk."* Then add a V-table row, or
say at V8 that pass 1 exiting `2` is a void like any other.

## MAJ-6 — §7.1's C-B receipt cell carries the capture digest of the SUPERSEDED dry run, contradicting SLOT A and the committed manifest in the same commit that produced both

**File**: `docs/experiments/wp20_pilot_prereg.md:871`, against
`artifacts/wp20pilot_dryrun_31c1cc1_v1.txt:70-71` and
`docs/experiments/wp20_pilot_artifacts.md:28-29`.

**What the document says**:

> | capture re-run (C-B) | `0`, and **`capture-determinism exit=0`** from `cmp -s` | both
> files `5fe1f1a36bef97d05679807c06df1efe85245ccd51362c6c670b5943ea95af20` |

**What is wrong.** SLOT A prints, at lines 70 and 71, the digest of both capture files:

```
807d56563cf1f618e337e15f6c7c8109d23a23f1a1f51f48b4a7d9098c7246eb  …/capture_400000.txt
807d56563cf1f618e337e15f6c7c8109d23a23f1a1f51f48b4a7d9098c7246eb  …/capture_400000_b.txt
```

and I recomputed both files on disk: `807d5656…`, matching. The committed manifest
carries `807d5656…` in both rows. `5fe1f1a3…` is the digest of
`/home/tom/pistol-runs/wp20pilot-dryrun2/capture_400000.txt` — the run at `85e6261`
that revision 5 superseded. `/usr/bin/grep -n "5fe1f1a3" docs/experiments/wp20_pilot_prereg.md`
returns exactly this one hit, and `/usr/bin/grep -n "807d5656"` returns none in the
pre-registration at all.

**This is D-555's class in its purest form.** The cell was **true at revision 4**, when
SLOT A was `..._85e6261_v1.txt`. MAJ-A's and MAJ-E′'s remedies re-took the dry run —
twice — and spent it. The author's own sweep is described in §0.1 as *"a sweep for every
phrase a remedy could have contradicted"*, and it did not reach this one, because the
phrase it would have had to search for is a 64-character hex string that appears nowhere
else in the document.

**Why it matters.** §7.1 is SLOT D — the section whose entire job is to be the receipt
that binds SLOT A's numbers to the run that produced them, and which spends fifteen
lines establishing that a measured number belongs to its run (D-479). Its own receipt
table now cites a digest from a run it declares superseded, and it cites it in the
one row that is criterion C-B's evidence on the stand-in. A successor checking the
document against the manifest finds two different digests for the same claim, in files
committed by the same commit.

**How I reproduced it.** `sed -n '871p' docs/experiments/wp20_pilot_prereg.md`;
`sed -n '66,71p' artifacts/wp20pilot_dryrun_31c1cc1_v1.txt`;
`sha256sum /home/tom/pistol-runs/wp20pilot-dryrun4/capture_400000{,_b}.txt`;
`/usr/bin/grep -n "5fe1f1a3\|807d5656" docs/experiments/wp20_pilot_prereg.md docs/experiments/wp20_pilot_artifacts.md`.

**Minimal remedy.** Replace the digest with `807d56563cf1f618e337e15f6c7c8109d23a23f1a1f51f48b4a7d9098c7246eb`,
and — since this cell duplicates two rows of the committed manifest, which is the file
that owns digests (D-423) — consider replacing it with *"both files, at the digest
`wp20_pilot_artifacts.md` indexes"*, so the next re-take cannot falsify it again.

---

# MINOR

## MIN-1 — §5's "disjoint" is false; limb (c)'s test claims three of limb (a)'s own bullets

**File**: `:451-452`. Limb (c)'s operative test is *"an `ArenaError::Io` naming a failed
read or write"*. Limb (a)'s bullet 2 (`outpath::claim` → `outpath.rs:16`, `"claiming
<path>"`), bullet 3 (`passes.rs:15,23`, `"reading <path>"`) and bullet 6 (`passes.rs:71`,
`"reading <path>"`) are all exactly that. An absent report file is (a) and (c) at once.
Both limbs are VOID, so no reading changes and this is minor — but the document asserts
disjointness and does not have it. Remedy: drop "disjoint and" and say the limbs may
overlap only where they agree.

## MIN-2 — an OOM-killed engine mid-capture is named by limb (b) and by limb (c)

**File**: `:495-499` against `:490-493`. Limb (c)'s prose names *"a process killed"*;
limb (b)'s list names *"an engine that … closed its pipe"*. On this machine those are
the same event: `CLAUDE.md`'s Environment section records tmpfs/RAM exhaustion as a live
failure mode, and pass 2 keeps a live seat while the cold check spawns 164 processes each
taking a 256 MiB table. Limb (c)'s operative test is an `ArenaError::Io`, and a dead
channel is not one, so the test sends it to (b) — STOP — while the prose sends it to (c)
— VOID. (`tools/cold_label_check.py` is clean here: `:169-176` raises `Void` on a
timeout, an `OSError` or a nonzero engine exit, so the same fault inside C-A is correctly
a void.) Remedy: put the operative test in (c) ahead of the prose — *"an `ArenaError::Io`,
**or a seat that died without speaking**"* — and remove "closed its pipe" from (b).

## MIN-3 — §1 states one claim, with the same citation, twice in three lines

**File**: `:137-139`. *"A change to any artefact in the table reopens this document
(`docs/process.md`, "Instrument governing revision"). Every artefact above except the
engine binary is a tracked file, so naming the commit names all of them at once, and a
change to any of them reopens this document (`docs/process.md`, "Instrument governing
revision")."* This round's own edit added the first half beside the second. D-423 is
explicit and §5 cites it as the reason for its own restructuring. Remedy: delete the
added clause; the sentence that survives already says it.

## MIN-4 — "both time every pass" is false of SLOT A

**File**: `:1078`. SLOT A runs six `--labels` passes and times two (lines 74, 78); the
four RULE-2 sweep passes at lines 37, 45, 53, 61 print `labels_NNNNN exit=0` with no
`seconds=`, and the four `load_NNNNN` runs are untimed too. MIN-a's remedy bracketed
`labels2` in §8 and re-used the sentence unchanged. Remedy: *"both time every pass §6.2
charges"*.

## MIN-5 — the manifest's heading says "CI at the registration head", and the receipt it indexes predates a 99-line change to a registered instrument

**File**: `docs/experiments/wp20_pilot_artifacts.md:55-59`. The only CI row is
`artifacts/wp20pilot_ci_6e1fea3_v1.txt` — `tools/ci.sh` at `6e1fea3`, which is revision
2's commit. `git diff --stat 6e1fea3 972cdd1 -- crates/ tools/ configs/` is **not
empty**: `crates/pistol-arena/src/bin/corpus-check.rs` (+99/−6) and
`crates/pistol-arena/tests/labels_tests.rs` (+115). So no committed receipt says the
tree is green at the revision that governs the run, and the receipt that exists predates
the change to the instrument §4E and §6.3 read their numbers off. Nothing in the
pre-registration cites CI (`/usr/bin/grep -n "ci.sh\|all gates passed"` → no hit), which
is why this is minor rather than major — but the heading asserts a property the file
below it does not have. Remedy: either re-title the section *"CI at revision 2's head,
before the `corpus-check` change"*, or take and index a receipt at the registration head.

## MIN-6 — the manifest says the dry run's commands are §8's, and §8 now says they differ

**File**: `docs/experiments/wp20_pilot_artifacts.md:12-14`: *"the dry run's literal
commands are the pre-registration's §8 with `configs/arena_wp20_label_pilot_dryrun.toml`"*.
§8:1074-1076 enumerates three differences, and MAJ-3 adds a fourth. Remedy:
*"the dry run's commands are §8's, with the differences §8 enumerates"*.

## MIN-7 — "TWO EARLIER DRY RUNS ARE SUPERSEDED" undercounts by one, and §7.2's "the SUPERSEDED dry run" no longer picks out a run

**File**: `:852-866` and `:911`. Four dry-run artifacts exist —
`..._f297eab_v1.txt`, `..._85e6261_v1.txt`, `..._071c65c_v1.txt`,
`..._31c1cc1_v1.txt` — and three are superseded. §7.1 names (i) `f297eab` and (ii)
`071c65c` and does not name `85e6261`, whose supersession §1:126-129 records from the
other side. §7.2:911's definite singular *"the SUPERSEDED dry run"* was unambiguous when
there was one; there are now three, and the reader has to know it means the first.
Remedy: *"THREE EARLIER DRY RUNS ARE SUPERSEDED"*, add the `85e6261` row (SLOT R1 moved
under it), and make §7.2 say *"the `f297eab` dry run, superseded at §7.1(i)"*.

---

# What I checked and found SOUND

Recorded so a successor does not re-derive it.

- **The depth table reproduces exactly, for the second review running, on a NEW corpus
  set.** Parsing column 10 of all 164 records in each of the four `wp20pilot-dryrun4`
  corpora with code sharing nothing with `corpus-check`: `50000: median 3.0, mean
  2.7195, min 1, max 4`; `100000: 3.0 / 3.0366 / 1 / 4`; `200000: 3.0 / 3.3049 / 1 / 5`;
  `400000: 4.0 / 3.6341 / 1 / 5`. Every cell of §6.3's table and every `corpus_check:`
  line in SLOT A (39, 47, 55, 63) matches. The even-count middle pairs are `(3,3)`,
  `(3,3)`, `(3,3)`, `(4,4)`, so `median`'s even branch cannot have moved an answer.
- **The `book` field is real and its value is right.** All four corpora carry
  `book ∈ {no, yes}`; SLOT A prints `book 2 (no,yes)` at every budget; `labels_file.rs:257-261`
  checks exactly `to_move`, `book`, `result`, `end`; `corpus-check.rs:82-95` prints all
  four plus `score_kind` in the order §4E:363-366 registers. MAJ-A's substantive defect
  — a registered output the registered revision could not produce — is gone.
- **All 17 manifest digests recompute**, including the SLOT A row
  (`3222ef19…`), all fifteen `wp20pilot-dryrun4` files, and the CI row. **Every claimed
  equality holds**: `capture_400000.txt = capture_400000_b.txt = 807d5656…`, and
  `corpus_400000.txt = _b = _c = d79f1d81…`. C-B's receipt on the stand-in is visible in
  a committed file, and the two C-E injection inputs are now indexed (MIN-e).
- **`T = 13` and every row of the sensitivity table re-derive.** Per-`T` cost
  `3.0 + 165.0 + 82.5 + 3.0 = 253.5`; floor 500 → `T = 7`, 574 positions, 1 776.5 s =
  29.6 min; floor 1 000 → 13, 1 066, 3 297.5 s = 55.0 min; floor 2 000 → 25, 2 050,
  6 339.5 s = 1 h 45.7; the ceiling → 56, 4 592, 14 198 s = 3 h 56.6, with 57 giving
  14 451.5 s, over 14 400. `82.5 / 253.5 = 32.54 %` and `165 / 253.5 = 65.09 %` are
  genuinely different quantities and both are used correctly.
- **The per-unit costs check out against SLOT A**: `g = 6/4 = 1.5`; `l = c = 165/164 =
  1.00610`; replay `6/4 = 1.5`; `p = 164/4 = 41` (the value is right — MAJ-3 is about
  its citation, not its size).
- **MIN-f's replacement arithmetic is correct at the code.** `capture.rs:27-43` yields
  `0..=len` less the terminal prefix when decided, so a capped game gives `turn_cap + 1
  = 41` and a game decided at turn `k` gives `k`; the worked costs (21 each, four
  spending 84 against a slack of 66; 2 each, four costing 8) are right.
- **MAJ-C's named instance is genuinely fixed.** `passes.rs` renders the whole output in
  memory and writes once, so an ENOSPC in pass 2 or 3 raises
  `ArenaError::io("writing <path>")` and lands in limb (c) as a VOID, which is what
  `:497` says and the opposite of what revision 4 said.
- **C-B is not vacuous, and its defect class is live.** `capture.rs:64-90`'s `normalise`
  strips only ` nps <n> time <n>`, so `hashfull` — a direct transposition-table occupancy
  readout — survives into every record and takes **14 distinct values** across the 164
  records of `capture_400000.txt`. A table that carried between asks would move it. The
  criterion compares two 57 601-byte files byte for byte, and pass 1 is excluded for the
  stated reason (`wall_ms` in the report).
- **`tools/cold_label_check.py`'s void discipline is correct.** `:169-176` raises `Void`
  on `TimeoutExpired`, on `OSError` and on a nonzero engine exit, and `:258-264` maps
  `Void` to exit 2. An engine killed by the environment during C-A is therefore a void
  and not a "disagreement", which is what C-A's registered consequence requires.
- **§9.1's SLOT P table matches the committed config exactly.** I read
  `git show 31c1cc1:configs/arena_wp20_label_pilot.toml` and every one of the fifteen
  rows agrees, `openings_take = 13` and `binary_sha256 = 180b4c40…` included.
- **§7.1's remaining receipt cells are true of SLOT A**: pass 1's `n 4 distinct-n 2`,
  `VERDICT inconclusive_degenerate` and `pass1 seconds=6`; `capture-determinism exit=0`
  and `labels-determinism exit=0`; `164 of 164 sampled record(s) agree byte for byte` at
  `go nodes 400000`; `replayed 4 of 4 game(s) … 0 divergence(s)` and `replay seconds=6`;
  `ok, 164 record(s)` with `book 2 (no,yes); result 1 (capped); end 1 (normal)`; and both
  C-E refusal messages verbatim, `ffc96a13…` / `532a23fb…` matching
  `corpus_400000.txt`'s own `# body_sha256` header. Only two cells of that table are
  false — `:870`'s `captured 164 position(s)` (MAJ-3) and `:871`'s capture digest
  (MAJ-6).
- **The round is scope-compliant under D-552.** `git diff --stat 950cad1..972cdd1` touches
  four files — the prereg, the manifest, the ledger line in `docs/decisions.md` (D-555),
  and the rev-4 review report — and **no code moved at all**
  (`git diff --stat 950cad1 972cdd1 -- crates/ tools/ configs/` is empty). No settled
  prose was re-opened.

---

# THE STRONGEST ATTACK THE DOCUMENT SURVIVED

**The attack.** *SLOT S1 — the whole slice, the consumed ledger range, and the wall — is
derived from a dry run played on a DIFFERENT opening book. The pilot draws from
`random_openings_v2.txt`, whose openings are three turns; the dry run drew from
`openings_v1.txt`, whose openings are four. §6.1 registers a floor in ASKED POSITIONS
and §6.3 satisfies it with `p = 41` measured on the four-turn book. A book whose
openings are one turn shorter leaves one more engine-chosen turn under the same cap, so
the per-game position count the pilot will actually produce is a different number from
the one the floor was applied to — and the document cannot check this, because §7 is
explicit that the dry run must never run on the registered workload. The floor is
therefore applied to a quantity measured on the wrong distribution, and no criterion in
§4 can falsify it: C-D reports whatever rate it finds, and §6's wall is an estimate the
document says cannot be failed. `T = 13` is an extrapolation across a book change with
no instrument behind it.*

**Why the document survives it — and this is the reviewer's answer, not the
document's.** The attack has the right target: `p` is the load-bearing quantity, and an
extrapolation across a book change with no check would be D-500's class wearing a
measurement's clothes. It is wrong that the extrapolation is uninstrumented, and it is
wrong about the mechanism.

§7:797-806 names the difference before I could and states the reason it does not
propagate: *"the positions a capped game contributes are `turn_cap + 1` — which is a
function of the CAP and not of the opening's length, so it is 41 under both books."*
That is a claim about the code, and it is checkable without running anything.
`capture::asked_prefixes` (`capture.rs:27-43`) replays the recorded turns, then returns
`(0..=game.moves.len())` less the terminal prefix if the game was decided. For a capped
game `game.moves.len()` is `turn_cap`, so the count is `turn_cap + 1 = 41` **regardless
of how many of those turns came out of the book** — the opening's turns are recorded
turns like any other and are asked like any other. The book change moves which positions
are asked; it cannot move how many a capped game contributes. I verified this against
the corpora as well: every game in every `dryrun4` corpus contributes exactly 41
records, and the `turns_played` column runs 0..40.

The residual — that a DECIDED game contributes fewer, and a shorter opening might
plausibly change how often games decide — is the one the attack could still hold, and
the document does not hide it. It is stated three times as an upper bound with its
error direction named (`:681`, `:757-760`, §10(iii)), it is quantified this round rather
than asserted (`:613-620`, the 66-position slack and the turn-`k` cost), the closure is
required to report the pilot's ACTUAL asked-position count beside the rate (`:623-625`),
and §10(iii) hands forward the exact consequence: throughput per POSITION survives, and
the plan must therefore be stated per position. A bound stated as a bound, with its
direction, its size, and the reporting obligation that lets a reader see whether it
bound, is not an unchecked extrapolation. **The attack fails, and it fails on a
mechanism I could confirm in fifteen lines of `capture.rs` rather than on the
document's say-so.**

**What this does not rescue.** None of my six MAJOR findings is about `p`'s size, RULE-1
or RULE-2. Five are D-555's class arriving for the third consecutive round, and the
migration is now visible as a pattern rather than a coincidence: **the remedies that
break something adjacent are the ones that ADD a totality claim.** Revision 3 said "and
nothing else"; revision 4 said "if and only if" and "two ways and no others"; revision 5
says "disjoint and exhaust the code", "three ways", "every one from that artifact's own
printed lines", and "Both intervals contain everything from 0 to about 4 s". Every one
of the four is a strengthening of a sentence that did not need strengthening, and every
one is false. The literal-string sweep cannot see them, because none of them repeats a
phrase from anywhere else — each is a NEW universal quantifier over a set the author did
not enumerate.

---

# CLOSING — IS THE DOCUMENT FIT TO GOVERN THE RUN?

**In my judgement: not as it stands, but the gap is narrow and it is not in the
instrument.**

The measured core of this pre-registration is sound and I say so without reservation. I
recomputed everything the scope asked me to recompute and everything I could reach
besides, from raw corpora and raw artifacts with my own code, and it all reproduces: the
four medians, the four means, `T = 13`, the four-row sensitivity table, the 253.5 T
arithmetic, 32.5 %, 65 %, and all 17 committed digests. SLOT R1 is correct for the first
time in this document's life and carries its own mechanical check. SLOT A now binds
itself to a revision, a tree state, and the digest of every binary that produced it —
which is more provenance than any artifact this package has previously carried and is
the direct product of the author catching his own remedy. The two criteria I attacked
for vacuity, C-A and C-B, are both live, and I can name the field (`hashfull`, 14
distinct values) that makes them so.

**Which findings could make the pilot's recorded conclusion wrong.** Two.

- **MAJ-5** is the only one that can put a wrong verdict in the closure. A stale
  `binary_sha256` at pass 1 — the fault §9.1 itself flags as undischargeable before
  launch — exits `2` and lands, by the document's own complement rule, on V7-B, which
  STOPS the arc and returns the package. That is a real run event given a wrong reading.
  The error direction is conservative (a void read as a stop, never a stop read as a
  pass), so it cannot bless a bad run; it can end a good one for a config typo.
- **MAJ-1** puts a wrong NUMBER in the pilot's own output. §7.2's findings are, by the
  section's own charter, what bounds what the closure may conclude, and the coldness
  bound is the one that carries a figure. `24 ms/ask` is twice what the readings support
  and is derived from an artifact §7.1 disqualifies and the manifest does not index. The
  closure will carry it forward.

**Which make the document worse without endangering its verdicts.** MAJ-2, MAJ-3, MAJ-4,
MAJ-6 and all seven MINORs. MAJ-2 is three inconsistent bounds on one quantity, which a
reader must resolve but which changes no criterion. MAJ-3 is a false citation for a value
independently recoverable from the artifact named two lines away — `p = 41` is right, its
receipt is not. MAJ-4 defeats a check without endangering a measurement: the real per-ask
cost is 1.006 s against a 120 s watchdog, so nothing will hang; what fails is §9's ability
to catch the next value that should not have passed. MAJ-6 is a wrong digest in a receipt
whose correct value is committed one file away. The MINORs are citations, counts and
headings. **None of these five can change a criterion's answer or a measured value; every
one of them makes the document unciteable at the point where it is wrong.**

**What I would tell the architect.** The correctness of this pilot's instrument is not
in doubt and has not been in doubt since revision 3. What has failed four times is the
document's prose discipline, and it has failed the same way each time for a reason that
is now legible: **the recurrences are overwhelmingly universal claims the author added
while fixing something else, plus receipts that a re-run of the dry run silently
falsified.** Nine of my thirteen findings fall under one editorial rule — *state no
totality, exhaustiveness or "and no others" claim the document does not itself enumerate
and check* — and two more (MAJ-2's stale pointer, MAJ-6's stale digest) fall under a
second — *a receipt that quotes a run's output by value is falsified whenever the run is
re-taken; quote the manifest, or re-derive the cell as part of re-taking*. The remainder
is two arithmetic slips whose correct forms I have written out above. Every remedy in
this report is a sentence or a cell; none requires a re-run, a rebuild or a re-measurement,
and none touches SLOT S1, SLOT S2, SLOT S3, SLOT W or SLOT R1, all of which I verified
independently and all of which are right.

If the architect's question is whether a fifth round could close this, the honest answer
is yes and that the remaining work is smaller and more mechanical than any previous
round's. If the question is whether the package may run against this document **today**,
the answer is no, and for exactly two reasons: **MAJ-5** gives the wrong verdict to a
reachable run event that §9.1 itself flags as undischargeable before launch, and
**MAJ-1** registers, as the pilot's own finding, a number the evidence does not support
and whose only inputs come from an artifact this package disqualified and does not index.
Those two are what the fifth round must fix; the other eleven are what makes the document
worse without making the pilot wrong.
