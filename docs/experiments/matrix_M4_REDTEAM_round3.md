# DECISION-RED-TEAM — MATRIX M4, REVISION 3 (round 3)

<!--
LANDED BY THE DISPATCHING SESSION, VERBATIM AS RETURNED by the fresh-context
red team. The dispatching session authored revision 3; it did not attack it.
F1 and F5 — the two findings against the author's own MEASURED cells — were
INDEPENDENTLY RE-RUN by the authoring session before the stop record was
written, and both reproduce. See `docs/experiments/matrix_M4_stop_round3.md`.
-->

**Pinned revision:** `9ce863f`.
**Matches HEAD:** **NO.** HEAD moved twice during this review (`9ce863f` → `0af32fb` → `809b5db`) — another session is committing to `dev`. **The matrix and the whole subject are byte-identical at `9ce863f` and at `809b5db`**: `git diff --stat 9ce863f..809b5db -- docs/experiments/matrix_M4_snapshot_config_seam_rev3.md tools/baseline_snapshot.sh crates/pistol-cli/tests/baseline_snapshot_tests.rs configs/ crates/pistol-cli/src/ crates/pistol-engine/src/` is empty. Every finding below therefore stands at both. Only `docs/` and `wp15b_mutation_witnesses.rs` moved.
**Fresh context:** I authored no revision of M4, no prior red team, and none of the subject. I edited nothing and ran no git write command in the live tree.
**Live tree at exit:** `git status --porcelain` → `?? docs/experiments/matrix_M3_selection.md` — an untracked file from the concurrent session, not mine. My scratch repo `/home/tom/.cache/m4rt` is removed (`ls` → "No such file or directory"). The pre-existing worktrees `/home/tom/.cache/fmtcheck` and `/home/tom/.cache/m3-author` are not mine.

---

## VERDICT

**Not every option falls — but the recommendation does, on all four of its grounds, and the field is incomplete for the third revision running.**

Four cells this matrix marks MEASURED do not reproduce or are false readings of what was measured, **and every one of them supports the author's own recommendation or the author's own new row.** That is D-318's recorded pattern recurring in the revision written to break it, for the fourth, fifth and sixth instances in this work package.

The two attacks the matrix itself named as the way to kill ground 2 — its self-declared load-bearing ground — **both succeed, measured.** The measurement the matrix declined to take for a third revision (N-K's cost) took eight lines and four minutes, and by the matrix's own flip trigger it flips the recommendation.

### Per-option survival, all thirteen rows

| Row | Survival after this round |
|---|---|
| **N-A′** optional `--config PATH` | **FALLS** (carried, unattacked here; falls on rule 3, not on a precedent) |
| **N-E** required `--config PATH` | **SURVIVES — AND LESS WOUNDED THAN THE MATRIX LEAVES IT.** The only ground stated against it (ground 2) is dead: measured, its headline exposure is already refused |
| **N-F** second committed script | **SURVIVES WOUNDED** (carried; unattacked this round). 646 lines confirmed |
| **N-G** environment variable | survives as a correct rejection (carried) |
| **N-H** a different instrument | survives as a correct rejection (carried) |
| **N-J** required run document | **SURVIVES.** Ground 2 is dead against it too |
| **N-K** config-pair / two-record mode | **SURVIVES, AND ITS SOLE BLOCKER IS GONE.** Cost now MEASURED: **8 added lines, 0 removed, 0 re-indented**, two complete records from one invocation |
| **N-L** re-pin the literal | **SURVIVES WOUNDED** (carried) |
| **N-B′** flip the committed config | survives as a rejection, but its ADR citation is misattributed and is a banned precedent (F8); the rule-6 half stands |
| **N-D′** null row | survives as a correctly recorded rejection |
| **N-M** closed-enum selector | **SURVIVES AS AN OPTION. RECOMMENDATION FALLS.** All four grounds broken (F1, F2, F7, F8); its cost cell's only MEASURED support does not reproduce (F5); its "MEASURED zero" carries no command (F9a) |
| **N-N** bind through the corpus fixture | **FALLS — but on its own failure mode (ii), not on ground 3.** Ground 3 prices a severing that cannot occur (F7) |
| **N-P** engine-agreement refusal | **FALLS AS SPECIFIED.** Its criterion is vacuous on the half that costs nothing, and its cost cell is false on the half that could fail (F4) |
| **N-Q** *(missing fourteenth row)* | `--config PATH` **containment-guarded to `configs/`** — absent from all three revisions and both prior red teams (F10) |

---

## FINDINGS

### F1 — KILL (fact 1's reading; ground 1). `engine_id config` is an echo, not an independent referent.

Fact 1 asserts: *"`engine_id config <path>` is the ENGINE's report of the document it actually loaded — a referent that does not share the script's variable."* **MEASURED FALSE.**

```
$ sed -n '130,136p' crates/pistol-cli/src/bin/pistol.rs
fn identity_lines(path: &Path, config: &Config) -> Vec<String> {
    let pistol_engine::config::CandidatePolicy::Radius { radius } = config.search.candidate_policy;
    let mut lines = vec![
        format!("config {}", path.display()),
...
$ printf 'pistol\nquit\n' | ./target/release/pistol --config ./configs/../configs/instrument_v0.toml 2>/dev/null | grep '^id config'
id config ./configs/../configs/instrument_v0.toml
```

The line is `format!("config {}", path.display())` — the argument string, echoed back, control-sanitised to `?` by `crates/pistol-cli/src/report.rs:151-162`. The script hands the engine `--config "$CONFIG"` (line 425) and gets `$CONFIG` back. It shares the script's variable **exactly**; it is the variable.

The record is therefore attested **two** ways, not three: the script's own content digest, and `engine_id candidate_policy radius 2`, which alone is derived from the document's *content* by a component that did not take the digest. The equivocation the prompt asked about is real and unresolved in the matrix's favour: **attested is not checked.** Nothing in the script, in its 29 tests, or in `tools/ci.sh` compares a record's config against the config the run was meant to take. Ground 1's *conclusion* (round 2's ground discriminates nothing) survives on a narrower footing; the *fact cell it rests on* does not.

### F2 — KILL (ground 2, the load-bearing one). Both attacks the matrix invited succeed.

Reproducer: an isolated scratch repo at `/home/tom/.cache/m4rt/repo` (copy of the shipped script, `configs/`, `openings_v1.txt`, a 2-position corpus at 15 and 35 stones, `git init`), the N-E arm added exactly the way `--corpus` is written:

```
--config) argument --config "$#" "${2:-}"; CONFIG="$ARG"; shift 2 ;;
```

Control run: exit 0, 1 s, `config configs/instrument_v0.toml 3579855e…`.

**(a) D-232's class — the newline LINE injection — is refused anyway, by something else in the script.**

```
$ EVIL=$'configs/evil\nladder_terminal forged cap_fired\nx.toml'; cp configs/instrument_v0.toml "$EVIL"
$ bash tools/baseline_snapshot.sh --config "$EVIL" --corpus corpus.txt --ladder-depth 1 --ladder-cap-s 5 --binary … ; echo "exit=$?"
baseline_snapshot: FAIL: sha256sum answered `\3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13  configs/evil\nladder_terminal forged cap_fired\nx.toml` for the config at configs/evil
ladder_terminal forged cap_fired
x.toml, which is not a digest
exit=1
```

GNU `sha256sum` escapes a newline-bearing filename by prefixing the line with `\`; `digest()`'s hex shape check (line 210-212) fires. Same for CR and backslash — both exit 1. **The class ground 2 names is already closed on the config path, by a check that exists for an unrelated reason.**

**(b) The residue is three lines, and it is a copy of the idiom already in the script.**

Admitted without it: TAB, ESC, U+2028, SPACE — all exit 0 under the COMPLETE kind token, characters raw in the invariant block. Adding, verbatim:

```
case "$CONFIG" in
*[![:print:]]*) fail "the config path \`$CONFIG\` has a character outside printable ASCII, and its WHOLE PATH is written into the record's invariant block" ;;
esac
```

```
TAB     exit=1  baseline_snapshot: FAIL: the config path `configs/g	f.toml` has a character outside printable ASCII…
ESC     exit=1  …
U2028   exit=1  …
SPACE   exit=0  baseline_snapshot: baseline_snapshot schema 1, 2 positions … ok
```

**(c) And that guard does not close the one that matters — which is open TODAY on the path the existing guard does cover.** A space is printable ASCII. It breaks the record's own parse rule (header: *"Key is the leading tokens, value is the rest of the line"*), shifting the digest field. On the **shipped, unpatched** `--corpus` path:

```
$ bash tools/baseline_snapshot.sh --corpus 'corpus with spaces.txt' … ; echo "exit=$?"
exit=0
$ grep -n '^corpus ' record
16:corpus corpus with spaces.txt sha256 622b8995… positions 2
```

So ground 2's *"a strictly wider check than the one that exists"* mis-describes both the check and what exists. As a **differential** ground separating N-M from N-E and N-J it is worth **three lines of copied idiom**. The matrix named this cell as its weakest and both of its own suggested attacks land. **KILL.**

### F3 — KILL (N-K's non-recommendation) — and a D-291 finding for the third revision running.

The matrix declines to recommend the option it calls dominant (*"removes the confound structurally and was never attacked on substance"*) **only** because *"its cost is unmeasured in every revision including this one."* CLAUDE.md: an estimate that could have been measured in seconds is a finding. It is measurable. I measured it.

Cheapest N-K, full diff against the shipped script — **8 added lines, 0 removed, no re-indentation**: two flag arms (`--config-b`, `--out-b`), three variable initialisers, and `for PASS in 1 2; do` / `done` wrapped round lines 415-646 with a one-line pass-2 rebind that re-runs `[ -f ]` and `digest`.

```
$ bash tools/baseline_snapshot.sh --config-b configs/instrument_staged_stub.toml \
    --out …/nk_a.txt --out-b …/nk_b.txt --corpus corpus.txt --ladder-depth 1 --ladder-cap-s 5 --binary …
baseline_snapshot: baseline_snapshot schema 1, 2 positions at nodes 50000 (registered), setup 2.77% ok -> …/nk_a.txt
baseline_snapshot: baseline_snapshot schema 1, 2 positions at nodes 50000 (registered), setup 1.80% ok -> …/nk_b.txt
exit=0 wall=4s
$ grep -n candidate_policy nk_a.txt nk_b.txt
nk_a.txt:14:engine_id candidate_policy radius 2
nk_b.txt:14:engine_id candidate_policy radius 3
$ grep -m1 '^position ' nk_a.txt; grep -m1 '^position ' nk_b.txt
position 1 early 15 nodes 50176 depth_turns 2 seldepth 3 hashfull 0 score cp -326 bestmove -1,5/1,3
position 1 early 15 nodes 50176 depth_turns 2 seldepth 3 hashfull 0 score cp -334 bestmove -1,5/1,3
```

Two complete records, two configs, two policies, one invocation, same revision and same `binary_sha256` by construction, ~2× wall (on the registered 24-position corpus: **2 × 33 s ≈ 66 s**, MEASURED from fact 1's own 33 s).

**Stated honestly:** this is a cost demonstration, not a shippable implementation. It carries no test (SHELL_CHECKLIST item 10 is the floor), the second config gets `[ -f ]` and `digest` but no guard, and one initialiser (`RECORDS=1`) is dead. The claim is only the one the matrix said it could not make: *the cost is small, and it was available in minutes on the machine the matrix was written on.*

The matrix's own flip trigger reads: *"The measurement of N-K's cost is taken and is small. Remedy: **flip to N-K**."* It is taken. It is small.

### F4 — KILL (N-P). Its criterion is the vacuous one CLAUDE.md names, and its cost cell is false where it isn't.

N-P failure mode (ii) is the matrix's own answer to the vacuous-criterion clause: *"the two sides are the script's variable and the ENGINE's report of what it loaded, which do not share an input."* **MEASURED FALSE by F1.** `[ "$CONFIG" = "$engine_id_config" ]` cannot fail for a wrong config — it is exactly *"internal agreement between components sharing an input,"* which CLAUDE.md says *"passes vacuously and is not a criterion."* It **can** fail for a control-character path, because `report.rs:151-162` rewrites control characters to `?` while the script's own line keeps them raw (measured: script writes `configs/evil^Iforged.toml`, engine writes `configs/evil?forged.toml`) — a false positive on a defect N-P does not target.

The other half — asserting `engine_id candidate_policy` is the radius the selected document commits — is genuine, and its cost cell is false: *"the two values are already both in the record… not new plumbing."* The record carries the engine's **reported** radius and no independently derived **expected** one. To check it the shell must parse the config's TOML — new plumbing, and a second reader of a document rules 1 and 2 keep in one place.

**On orthogonality:** N-P *is* orthogonal, as its own failure mode (i) says. Entering it as a row of the seam field inflates thirteen to a count that includes a non-rival; the honest place for it is a named separate decision, which the "NOT a flip trigger" bullet already concedes. Selecting N-M plus N-P would not dominate the recommendation — after F4, N-P adds one vacuous comparison and one unpriced TOML reader.

### F5 — KILL under R7 (fact 4 does not reproduce). N-M's cost cell has no surviving MEASURED support.

The matrix prints its command and ten lines of "verbatim output". GNU grep gives **eleven**:

```
$ command grep -rn "baseline_snapshot.sh" --include=*.rs --include=*.sh . | command grep -v '^./tools/baseline_snapshot.sh:' | wc -l
11
```

The omitted line is `./tools/bench_delta.sh:120` — a site that **names the shipped script**, in scope by fact 4's own definition (*"every site that invokes or names the shipped script, outside `docs/` and outside its own text"*). The derived claim *"Six of the ten hits are comments naming the script"* is wrong twice: eleven hits, seven comments.

Not tree drift — verified at the matrix's own revision:

```
$ git grep -n "baseline_snapshot.sh" 1d5af10 -- '*.rs' '*.sh' | wc -l          → 12
$ … | grep -v 'tools/baseline_snapshot.sh:' | wc -l                            → 11
$ git show 1d5af10:tools/bench_delta.sh | sed -n '118,122p'
# tools/baseline_snapshot.sh dropped `sha256sum`'s status into an `echo` argument
```

R7: *"A non-reproducing cell kills the option's GROUND, not just the cell."* Fact 4 is the only MEASURED support in N-M's cost cell. The three invocation constructions (`:171`, `:1037`, `:1206`) and the 29 tests *do* reproduce; the pasted output and the derived count do not.

### F6 — WOUND (fact 7's paste is edited, and the edit moves toward the thesis).

```
$ sed -n '23,26p' crates/pistol-cli/tests/fixtures/tactical_v0.txt
# Every case states the config it is a claim about, because a threshold means
# nothing without the search that has to meet it (CLAUDE.md rule 6):
#
#   configs/instrument_v0.toml   the INSTRUMENT config, candidate radius 2 since
```

The matrix prints three lines and replaces line 24's tail with `nothing without one...`. The real sentence says a threshold needs a **search**; the elided one reads as a threshold needing a **config**, which is the matrix's own thesis. R7 requires the output inline.

### F7 — WOUND/KILL (ground 3 prices a severing that cannot occur).

Fact 8 reproduces exactly — `grep -o` gives three, `grep -c` of the full digest gives one, and lines 414/478/482 are D-184 (the re-pin to `931c50b1…`), D-218 (*"both committed fixtures re-emit BYTE-IDENTICAL (`5ccc3dc0…`, `931c50b1…`)"*) and D-220 (the full digest in the registered bench provenance). The citations say what the matrix says.

The **cost** does not follow. The digest has exactly **one** code pin:

```
$ grep -rn "BENCH_POSITIONS_V1_SHA256\|931c50b1" --include=*.rs --include=*.sh --include=*.toml .
./crates/pistol-cli/tests/corpus_document_tests.rs:20:const BENCH_POSITIONS_V1_SHA256: &str =
./crates/pistol-cli/tests/corpus_document_tests.rs:21:    "931c50b19411eef5aaf0385df46782bb2babbf3443e65a6e56fe7bd169906e47";
```

The tree carries a landed, ADR-recorded procedure for moving a fixture digest (D-184: state the input's digest, diff before installing, stop if anything but the intended lines moved). And decisively — **by the matrix's own fact 6, the AFTER cannot be taken at all yet.** There is no registered BEFORE for a digest move to sever from; both records would be taken after the change, at the same new digest. *"SEVERS the corpus-identity claim between the BEFORE record and the AFTER record"* and *"the largest hidden cost in the field"* price the expensive version of a row that does not cost it. N-N still falls — on failure mode (ii), one fixture declaring one config means two configs are two fixtures and two workloads — but **ground 3 as stated is not why.**

### F8 — WOUND (ground 4 uses a banned precedent, and misattributes it).

Ground 4: *"N-B′ moves the committed default, which **rule 6 and D-204** put behind the operator's SPRT."*

```
$ sed -n '454p' docs/decisions.md | grep -o 'SPRT' | wc -l
0
```

D-204 does not contain the word. It rules that the tactical fixture's thresholds bind at whatever `configs/instrument_v0.toml` commits. Citing an ADR as authority for what should be chosen is a precedent ground, banned by R9 and by the matrix's own §4 (*"Precedent grounds are BANNED for this round (R9) and none is used"*). Rule 6 alone carries the point; D-204 adds nothing it says. This is the only precedent I found smuggled in as authority — fact 7's mechanism citations and N-N failure mode (iii) both disclaim precedent correctly and I accept them.

### F9 — WOUND (D-291 marks).

- **(a)** N-M's cost cell: *"**MEASURED zero path guards owed**"* carries **no command and no output**, against §3's own promise: *"Every number is MEASURED here with its command and output inline (R7), **including the zeros**."* It is a deduction wearing a MEASURED mark — D-291's exact class, in the recommendation's own cost cell.
- **(b)** N-N's cost cell: the digest *"appears **twice** in `docs/decisions.md` as registered provenance"* — unmarked, and contradicting the matrix's own fact 8, *"Three citations on **three** ADR lines."*
- **(c)** *"Selecting costs **one** fresh-context DECISION-RED-TEAM dispatch"* — unmarked, and false on the document's own history: this is the third, and none of the first two selected.

### F10 — MISSING ROW (the fourteenth). A `configs/`-containment-guarded path, and SHELL_CHECKLIST item 11.

```
$ grep -n -i 'containment\|item 11\|scoped to configs\|under configs/' \
    matrix_M4_snapshot_config_seam.md matrix_M4_snapshot_config_seam_rev3.md \
    matrix_M4_REDTEAM.md matrix_M4_REDTEAM_round2.md
(no hits for containment / item 11 in any of the four)
```

**N-Q: `--config PATH`, resolved against `$ROOT` and refused unless it resolves under `configs/`.** It is neither a closed enum over two names nor an arbitrary caller path. Because only `configs/<basename>` can ever reach the record, the **existing** basename allow-list loop at line 289 covers it by adding one word — no whole-path guard is owed at all. It closes N-M's own failure mode (ii) (*"a config the enum does not name cannot be measured at all… a real loss the moment a third registered document appears"*) and defuses N-M's own first flip trigger, without N-E's whole-path exposure. `tools/SHELL_CHECKLIST.md` item 11 — *"A CALLER'S PATH THAT FEEDS A DELETE OR AN OVERWRITE IS CONTAINMENT-GUARDED… ABSOLUTE-VALUE ESCAPE IS THE ATTACK"* — is this tree's own vocabulary for the row, written from a defect in this very script's `--out`, and it appears in **none** of the three revisions.

Related, and part of the same gap: the subject is a `tools/` script, so the round is bound to the checklist. Revision 3 says *"SHELL_CHECKLIST answered item by item"* inside two cost cells and **answers no item by name**. It never reaches item 11, and never reaches **item 12** (VOID vs FAIL) — N-M's "unrecognised token" is a FAIL, a named config that cannot be read may be a VOID, and the matrix classifies neither.

---

## WHAT I COULD NOT BREAK

- **Fact 1's record itself.** Re-taken: `exit=0 wall=33s`, head byte-identical to the paste except `revision`, which is a run-time fact by design. `config`, `binary_sha256`, `engine_id …`, `corpus … 931c50b1… positions 24` all match. The measurement is sound; only its reading (F1) is not.
- **Fact 2.** Both greps and both refusal runs reproduce verbatim, including the `entries()` observation — `[ -f ]` at line 272 really is what refuses, and `grep -v '^#' | grep . || true` really would have swallowed it into COUNT=0.
- **Fact 3's two commands.** `sed -n '289,294p'` and the grep reproduce verbatim. The asymmetry is real: line 443 writes a basename the guard checks, line 440 writes a whole path it does not. Only the *inference* from it fails.
- **Fact 5, and the derived claim.** Verbatim. And *"the fixture-copy site does not move"* **holds**: `ScratchRepo::new` is used by exactly one test (`grep -c` → 1) and every other test defaults to `root: repo_root()`, where the staged document will exist once WP-1.5b lands it.
- **Fact 6.** Twelve documents, no `configs/instrument_staged_v0.toml`. The AFTER really is blocked on a document no row produces.
- **Fact 8's citations.** All three, and the truncated-vs-full observation, exactly as printed.
- **N-M failure mode (iii).** `pistol --help` does offer `--config <path>`; N-M's surface really would be narrower than the binary it drives.
- **The three invocation constructions and the 29 tests.**
- **Hard rule 1 (item 3): I could not call N-M a breach, but its distinction is weaker than it says.** `Budget` (`crates/pistol-engine/src/budget.rs:21`) is a closed enum of **kinds**, each carrying a caller-supplied **value** that lives in the schema. N-M is a closed enum of **values**, living in the script. It reproduces rule 1's *vocabulary* — closed enum, no default, absent is an error — and inverts its *mechanism*, *"a default lives in exactly one schema place."* Rule 1 names defaults and N-M has none, so this is a WOUND, not a KILL; but the matrix's own concession (*"this row does not have an answer beyond the distinction"*) is the accurate one, and the distinction does not survive comparison with the enum rule 1 was written for.

---

## THE STRONGEST SURVIVING ATTACK PER SURVIVING OPTION (ADR-quotable)

- **N-E** — *"The whole-path exposure it owes is not D-232's line injection: that class is already refused on the config path by `digest()`'s hex shape check, because GNU `sha256sum` prefixes an escaped filename with `\` (reproduced, exit 1, named refusal), and the residue — TAB, ESC, U+2028 — is closed by a three-line copy of the guard already at line 289. What neither guard closes is a SPACE, which breaks the record's own leading-tokens parse rule, and that hole is open today on the corpus line the existing guard does cover."*
- **N-J** — as N-E, plus: *"it adds a second registered document, whose own revision the pre-registration must then name and whose amendment reopens the review."*
- **N-K** — *"Its cost is measured at eight added lines and 2× wall, but the eight lines carry no test driving the shipped script, and it owes on its second config exactly the caller-named-path obligation N-E owes on its first — three lines, measured, but owed."*
- **N-M** — *"Its arms are a token→path map that reproduces hard rule 1's vocabulary and inverts its mechanism: `Budget` is a closed enum of kinds whose values live in the schema, N-M a closed enum whose values live in the script; and a config the enum does not name cannot be measured at all, which becomes a real loss the moment a third registered document appears."*
- **N-F** — carried, unattacked this round; two instruments that must stay in step to be comparable.

---

## WHAT THE ARCHITECT IS LEFT WITH

1. **The recommendation of N-M has no surviving ground.** Ground 1 rests on a false reading of its own measurement; ground 2, the self-declared load-bearing one, dies to both attacks the matrix itself proposed and is worth three lines; ground 3 prices a severing that cannot occur while the AFTER is blocked; ground 4 leans on a banned, misattributed precedent. N-M remains a viable *option*; nothing selects it over N-E, N-J or N-K.

2. **This is not the "every option fell" stop.** N-E, N-J, N-K, N-F, N-L survive, and the ground round 2 and round 3 both aimed at N-E and N-J is now measured away rather than argued away.

3. **N-K's blocker is gone, and the matrix's own flip trigger fires.** Eight lines, one invocation, two records at two policies. Nothing in three revisions attacks N-K on substance, and the one thing that held it back is now on the record with a reproducer. If the architect selects anything from this field, N-K is the row whose stated reason for exclusion no longer exists.

4. **The field is still incomplete, for the third revision.** N-Q — a `configs/`-containment-guarded path — dominates N-M on N-M's own failure mode and N-E on the guard surface, and is named nowhere in three revisions or two red teams, though `tools/SHELL_CHECKLIST.md` item 11 is this project's own name for it. A round bound to that checklist answered none of its items by name.

5. **D-318's pattern recurred in the revision written to break it.** Three more cells this round: fact 4's output, fact 1's reading, fact 7's paste — plus N-P's cost cell and its vacuous-criterion defence. Every one favours the author's own recommendation or the author's own new row. That is the recurrence, not the instance, and it is now the third distinct authoring session to produce it in this work package.

6. **Two defects in the shipped instrument, found in passing, belonging to no row.** (a) A space in a caller-named `--corpus` path lands unescaped on the `corpus` line, breaking the record's own parse contract, exit 0, COMPLETE token — the printable-ASCII allow-list admits it by construction. (b) `report.rs:151-162` rewrites control characters to `?` in the engine's handshake while the script writes its own copy raw, so the two config lines in one record can disagree for a reason unrelated to the config. Neither is a matrix question; both are `tools/` findings the architect may want scheduled independently of this decision.

**I did not select. Selection is the architect's.**
