# MATRIX M4 — the snapshot's config seam — **REVISION 3, AUTHORED, NOT SELECTED**

Status: **AUTHORED. AWAITS A THIRD FRESH-CONTEXT DECISION-RED-TEAM.** Nothing
below is selected. Subject: `tools/baseline_snapshot.sh`. Owning unit:
`docs/experiments/U4_soundness_instrument.md` §9.

Revisions 1 and 2, their two attacks and the stop are
`docs/experiments/matrix_M4_snapshot_config_seam.md`,
`docs/experiments/matrix_M4_REDTEAM.md`,
`docs/experiments/matrix_M4_REDTEAM_round2.md` and **D-318**. Neither prior
revision is edited by this file, and no row is re-argued here on a ground its own
red team already falsified.

## WHO AUTHORED THIS, AND WHAT D-318 SAID IT MUST CARRY

D-318: *"a THIRD round, the architect's and explicitly not this session's, since a
third field authored by the session that wrote the first two — now populated with
rows both red teams supplied — completes D-305's measured pattern instead of
breaking it."* **The session authoring this revision authored neither prior
revision**, and the round is scheduled by the architect (ruling R9). D-318's four
requirements are answered in order:

1. **The two missing rows are entered** — **N-M**, the closed-enum selector, and
   **N-N**, binding the config through the corpus fixture.
2. **The framing that excluded them twice is dropped.** The question is no longer
   "how may a caller name a path"; it is *"how does a registered instrument bind
   the one input that distinguishes its two records"*, which admits selectors that
   name no path at all.
3. **Every number is MEASURED here with its command and output inline (R7),
   including the zeros**, or is marked CARRIED with its source, or is marked
   ESTIMATED.
4. **No ground below rests on D-252, on D-283, or on B3's shape comparison.**
   D-252 selected nothing (D-288), D-283 is unattacked by its own text, and
   D-316's own residual says the same of B3. Precedent grounds are BANNED for this
   round (R9) and none is used. Where this matrix cites a shipped MECHANISM in the
   tree, it cites it as a measured fact about what exists and costs, never as an
   authority for what should be chosen — and it says so at each site.

**R9 also fixes one thing this matrix may not use:** `docs/experiments/wp15b_sprt_prereg.md`'s
sentence that *"MATRIX M4 ADOPTS adding `--config`"* **is not a ground**. It is a
draft's forward reference to a decision no matrix has made, it governs nothing, and
it appears in no cell below.

---

# FACTS — MEASURED HERE, each with its command and its verbatim output (R7)

Taken over the tree at `1d5af10`. The named prior for this round is D-318's
record that **three cells marked MEASURED in this work package did not
reproduce**, all three supporting their author's own recommendation. Every fact
below is a command a red team runs.

**Fact 1 — the config literal is at ONE site, and the record it produces names
the config THREE times over, one of them in the engine's own words.** A full
snapshot was taken for this matrix:

```
$ S=$SECONDS; bash tools/baseline_snapshot.sh --out <scratch>/snap_before.txt; echo "exit=$? wall=$((SECONDS-S))s"
exit=0 wall=33s
$ head -16 <scratch>/snap_before.txt
baseline_snapshot 1
schema 1
revision 1d5af104c3e9be6e4f7d8231888dc1373cb70d44
binary_sha256 a7f519fade1124780463293b86e27cbdd0732540a84aa75acaed4de4689f03ce
config configs/instrument_v0.toml 3579855e7cf23d07c54c431e42656818838230aded9d921215cad1cee8b9ec13
engine_id name pistol
engine_id version 0.0.1
engine_id protocol v0
engine_id mode instrument
engine_id budgets depth_turns nodes
engine_id config configs/instrument_v0.toml
engine_id eval handcrafted_v0
engine_id tt_bytes 268435456
engine_id candidate_policy radius 2
engine_id weights_sha256 41ef549666d787bf404b2922ce644b7797a49680f09366e6fef6239b3f91f2c3
corpus bench_positions_v1.txt sha256 931c50b19411eef5aaf0385df46782bb2babbf3443e65a6e56fe7bd169906e47 positions 24
```

**33 s, MEASURED, one run, this session.** Read the record: `config <path>
<sha256>` is the script's own variable plus a CONTENT DIGEST; `engine_id config
<path>` is the ENGINE's report of the document it actually loaded — a referent
that does not share the script's variable; and `engine_id candidate_policy radius
2` is the engine reporting **the very quantity the two records are supposed to
differ in**. All three are above the `# timing` marker, on invariant lines.

**Fact 2 — the default literal, and the guard that already refuses a missing
config.**

```
$ grep -n 'CONFIG=' tools/baseline_snapshot.sh
170:CONFIG="configs/instrument_v0.toml"
$ grep -n '\[ -f' tools/baseline_snapshot.sh
271:[ -f "$CONFIG" ] || fail "no config at $CONFIG"
272:[ -f "$CORPUS" ] || fail "no corpus at $CORPUS"
273:[ -f "$OPENINGS" ] || fail "no opening corpus at $OPENINGS"
310:[ -f "$BINARY" ] ||
```

One site. A named-but-missing config is already refused by name at exit 1:

```
$ bash tools/baseline_snapshot.sh --corpus /nonexistent/corpus.txt 2>&1 | tail -1
baseline_snapshot: FAIL: no corpus at /nonexistent/corpus.txt
$ bash tools/baseline_snapshot.sh --corpus /nonexistent/corpus.txt >/dev/null 2>&1; echo "exit=$?"
exit=1
```

**This was checked by RUNNING it rather than by reading `entries()`, and the
reading would have been wrong**: `entries()` is `grep -v '^#' "$1" | grep . ||
true`, which swallows a missing file into COUNT=0, and the reason the refusal is
correct is the separate `[ -f ]` at line 272 — not the counter.

**Fact 3 — THE GUARD ASYMMETRY, and it is the fact that separates the selector
rows.** The script guards the caller-named paths whose BASENAMES reach the
invariant block, and the config's FULL PATH reaches that block unguarded because
it is a literal today:

```
$ sed -n '289,294p' tools/baseline_snapshot.sh
for named in "$CORPUS" "$OPENINGS"; do
	case "${named##*/}" in
	*[![:print:]]*) fail "the corpus path \`$named\` has a character outside printable ASCII in its file name, and its name is written into the record's invariant block" ;;
	esac
done
$ grep -n 'echo "config \|echo "corpus ' tools/baseline_snapshot.sh
440:	echo "config $CONFIG $CONFIG_SHA256"
443:	echo "corpus $(basename "$CORPUS") sha256 $CORPUS_SHA256 positions $COUNT"
```

The class the allow-list exists for is D-232's, **REPRODUCED**: a newline in a
caller-named path injected attacker-chosen LINES into the invariant block with the
record still exiting 0 under the COMPLETE kind token. Note the shape: line 443
writes a BASENAME and the guard checks a basename; **line 440 writes a WHOLE
PATH**. Any option that lets a caller supply that path owes the D-232 guard
extended to the whole path, which is a strictly wider check than the one that
exists.

**Fact 4 — the caller audit: every site that invokes or names the shipped script,
outside `docs/` and outside its own text.**

```
$ grep -rn "baseline_snapshot.sh" --include=*.rs --include=*.sh . | grep -v '^./tools/baseline_snapshot.sh:'
crates/pistol-solver/tests/wp15b_census.rs:55:/// The registered corpus: the same 24 positions `tools/baseline_snapshot.sh`
crates/pistol-arena/tests/binary_binding_tests.rs:271:/// `tools/bench_delta.sh` and `tools/baseline_snapshot.sh` with `command -v` and
crates/pistol-arena/tests/artifact_gate_tests.rs:89:/// (tools/baseline_snapshot.sh, docs/decisions.md D-230).
crates/pistol-cli/tests/bench_delta_tests.rs:302:    // The sibling of tools/baseline_snapshot.sh's empty-digest defect, one
tools/artifact_check.sh:15:# tools/baseline_snapshot.sh) — because the patterns below match names, and a
crates/pistol-cli/tests/baseline_snapshot_tests.rs:1://! `tools/baseline_snapshot.sh` — the standing before/after instrument for
crates/pistol-cli/tests/baseline_snapshot_tests.rs:171:        let script = self.root.join("tools/baseline_snapshot.sh");
crates/pistol-cli/tests/baseline_snapshot_tests.rs:384:            "tools/baseline_snapshot.sh",
crates/pistol-cli/tests/baseline_snapshot_tests.rs:1037:        .arg(repo("tools/baseline_snapshot.sh"))
crates/pistol-cli/tests/baseline_snapshot_tests.rs:1206:        .arg(repo("tools/baseline_snapshot.sh"))
$ grep -c '#\[test\]' crates/pistol-cli/tests/baseline_snapshot_tests.rs
29
```

**THREE invocation constructions** (`:171`'s `go()` helper, `:1037`, `:1206`), one
fixture-copy site (`:384`), one usage block (`tools/baseline_snapshot.sh:109`).
Six of the ten hits are comments naming the script, not callers. `tools/ci.sh`
does not invoke it. **29 tests.**

**Fact 5 — the fixture copy that makes "zero code change" false for any option
that moves the default.**

```
$ sed -n '383,390p' crates/pistol-cli/tests/baseline_snapshot_tests.rs
        for file in [
            "tools/baseline_snapshot.sh",
            "configs/instrument_v0.toml",
            "configs/eval_v0_weights.toml",
            "crates/pistol-cli/tests/fixtures/openings_v1.txt",
        ] {
```

The scratch-repo fixture copies `configs/instrument_v0.toml` **by name**. An
option that changes which document the default names changes this list.

**Fact 6 — the committed config set today, and the document the AFTER run needs,
which does not exist.**

```
$ ls -1 configs/ | tr '\n' ' '
arena_smoke_v0.toml arena_wp13_fair_corpus.toml arena_wp13_fair_random.toml arena_wp13_r2_vs_r3_confirm.toml arena_wp13_r2_vs_r3.toml arena_wp15b_dryrun.toml eval_v0_weights.toml gate_v0.toml instrument_r2_v0.toml instrument_v0.toml play_v0.toml random_openings_v1.toml
```

Twelve documents; **`configs/instrument_staged_v0.toml` is not among them.** U3
(u-rev 3) §10 commits it as the SPRT seat and *"the snapshot's AFTER"*. So the
AFTER run is blocked on a document WP-1.5b lands, whichever seam is selected —
an ordering fact, not a cost of any row.

**Fact 7 — the engine's own seam is an arbitrary PATH, and its fixture seam is
the opposite.** Cited as a measured fact about mechanisms that exist, **not as a
precedent ground** (R9):

```
$ ./target/release/pistol --help 2>&1 | sed -n '4,8p;16,18p'
  pistol --config <path>                      speak the line protocol on stdin
  pistol perft --depth <turns> [--plies "<q,r> …"]
  pistol selftest --fixtures <path>
  pistol --help

  --fixtures    a sha-pinned tactical fixture. It names the config it was
                pre-registered against, so `selftest` takes no --config.
$ sed -n '23,26p' crates/pistol-cli/tests/fixtures/tactical_v0.txt
# Every case states the config it is a claim about, because a threshold means
# nothing without one...
#   configs/instrument_v0.toml   the INSTRUMENT config, candidate radius 2 since
```

Both shapes are BUILT in this tree: the engine takes a caller-named path, and the
tactical fixture declares its own config so its consumer takes none.

**Fact 8 — the snapshot corpus's digest is a comparability key, and it has a
second consumer.**

```
$ sed -n '55,56p' crates/pistol-solver/tests/wp15b_census.rs
/// The registered corpus: the same 24 positions `tools/baseline_snapshot.sh`
/// measures, read from the sha-pinned fixture rather than restated.
$ grep -o '931c50b1[0-9a-f…]*' docs/decisions.md
931c50b1…
931c50b1…
931c50b19411eef5aaf0385df46782bb2babbf3443e65a6e56fe7bd169906e47
```

Three citations on **three** ADR lines — `docs/decisions.md:414` (D-184, the
re-pin that records the digest moving), `:478` (D-218, which re-emits both
fixtures byte-identical and cites this digest as the evidence that nothing
moved), and `:482` (D-220, the full digest in the official bench verdict's
registered provenance). **The truncated form
is why a `grep -c` of the FULL digest returns 1 and not 3**; that check was run
both ways here rather than reported once, because a count of a digest is exactly
the kind of cell this work package has three times marked MEASURED and got wrong.

`bench_positions_v1.txt`'s digest `931c50b1…` is D-184's pin, is carried in
D-220's registered bench provenance, is on the invariant line of every record
(fact 1), and the same fixture is read by `wp15b_census.rs`. **Changing the
fixture's bytes changes that digest**, and the digest is how two records are known
to have measured the same workload.

**CARRIED numbers, cited and NOT re-measured here:** N-F's *"646 lines"* (round
2), N-L's *"Zero code change is MEASURED FALSE"* (round 2, R6), N-B′'s unmarked
*"15"* (round 2, R10), and revision 2's *"BEFORE re-taken 34.5 s"* — superseded
for this round by fact 1's own 33 s.

---

# THE FIELD — thirteen rows

Ten carried with the standing round 2 gave them, plus the two rows D-318 names as
missing and one this author's own measurement produced. **Nothing here is
selected.**

## The ten carried rows

| Option | What it does | Standing after round 2 |
|---|---|---|
| **N-A′** optional `--config PATH`, default kept | a seventh flag arm, default unchanged | **FALLS** — a caller who forgets gets the old behaviour from the very flag added to prevent it; rule 3 forbids skip-with-default. (Round 2 notes one of its three executioners was a void precedent; it falls on the other two) |
| **N-E** required `--config PATH`, no default | same arm, `CONFIG` starts unset | **SURVIVES WOUNDED, RECOMMENDATION FALLEN** — three of its four grounds were falsified; the fourth (refuse the silent record) discriminates it from nothing, and fact 1 here shows that ground is void for every row |
| **N-F** a second committed script | `tools/baseline_snapshot_staged.sh` | **SURVIVES WOUNDED** — CARRIED 646 lines duplicated or a shared body extracted; two instruments that must stay in step to be comparable |
| **N-G** an environment variable | `PISTOL_SNAPSHOT_CONFIG` | **SURVIVES AS A CORRECT REJECTION** — provenance the record cannot attest, EXIT-0-WRONG-ANSWER by construction |
| **N-H** a different instrument | take the Staged number from `tools/bench_delta.sh` | **SURVIVES AS A CORRECT REJECTION** — that script has no config seam either, and its output is a ratio, not the per-position record |
| **N-J** a required snapshot RUN DOCUMENT | a document naming config, corpus, budget | **SURVIVES** — its round-2 rejection fell. It is a caller-named required path, so fact 3's guard obligation applies to it exactly as to N-E |
| **N-K** a config-PAIR / two-record mode | one invocation takes both configs and emits both records | **SURVIVES WOUNDED** — the only row that removes the confound structurally; its cost is an unmeasured comparative chain and it was omitted from the guard enumeration |
| **N-L** re-pin the literal by ADR | change line 170, no flag | **SURVIVES WOUNDED** — its "Zero code change" is CARRIED MEASURED FALSE (fact 5 is why), and it cannot hold two records at once |
| **N-B′** flip the committed config to staged | make staged the committed default | **SURVIVES AS A CORRECT REJECTION** — rule 6 puts that move behind the operator's SPRT (D-204), not behind an instrument's convenience |
| **N-D′** take no Staged snapshot (null row) | report the radius numbers only | **SURVIVES AS A CORRECTLY RECORDED REJECTION** — the depth claim goes unmeasured; recorded because a null row nobody states is a rejection nobody can check |

## N-M — a REQUIRED CLOSED-ENUM SELECTOR, `--config {instrument|staged}`

| | |
|---|---|
| **What it does** | One required arm taking a TOKEN, not a path. The token maps to a committed document: `instrument` → `configs/instrument_v0.toml`, `staged` → `configs/instrument_staged_v0.toml`. An absent or unrecognised token is refused by name at exit 1. No default |
| **Cost** | A `tools/` change: SHELL_CHECKLIST answered item by item, plus the coverage rule's test driving the shipped script. **MEASURED caller work from fact 4: three invocation constructions and one usage block**; the fixture-copy site (fact 5) does not move, because the `instrument` arm still names `configs/instrument_v0.toml`. **ESTIMATED 3 new tests** — the refusal when the token is absent, the refusal when it is unrecognised, and one asserting each arm's document reaches the record's `config` line. **MEASURED zero path guards owed**: guards for caller-relative resolution and for fact 3's injection class exist only where a caller supplies a path, and this row supplies none |
| **Failure modes** | (i) **The arms are code-side literals** — two where there was one. Hard rule 1 forbids a code-side DEFAULT, and an enum with no default is not that; but a reviewer may hold that the token→path map is "a default living outside its one schema place", and this row does not have an answer beyond the distinction. (ii) A config the enum does not name cannot be measured at all, which is refusal by construction and is a real loss the moment a third registered document appears (fact 6 shows the second one does not exist yet). (iii) **The instrument's surface becomes NARROWER than the binary it drives** — fact 7: `pistol` itself takes `--config <path>`. (iv) It is the first config-scope flag anywhere in `tools/`, so its shape is what later scripts copy |

## N-N — BIND THE CONFIG THROUGH THE CORPUS FIXTURE

| | |
|---|---|
| **What it does** | The workload fixture declares the config it is a claim about; the script reads it from there and takes no config argument at all. `--corpus` already exists, so the caller-named surface is unchanged |
| **Cost** | **MEASURED, and it is the largest hidden cost in the field**: fact 8. `bench_positions_v1.txt`'s sha256 is on the invariant line of every record taken, is D-184's pin, appears twice in `docs/decisions.md` as registered provenance including D-220's bench verdict, and the same fixture is read by `crates/pistol-solver/tests/wp15b_census.rs`. Declaring a config inside it changes those bytes and therefore that digest, which **severs the corpus-identity claim between the BEFORE record and the AFTER record** — the one property the two records must share. Avoiding that costs a SECOND fixture, at which point the workload differs between the two records by construction |
| **Failure modes** | (i) It conflates the two axes the comparison varies — same workload, two policies — by making the workload name the policy. (ii) Two records under two configs need two fixtures, and two fixtures are two workloads. (iii) The shape is landed and working elsewhere (fact 7, `selftest --fixtures`), which is evidence it is BUILDABLE and says nothing about whether it fits an instrument whose fixture digest is a comparability key; this row does not argue from that precedent |

## N-P — REFUSE A RECORD WHOSE ENGINE-REPORTED CONFIG DISAGREES WITH THE REQUESTED ONE

**THIS ROW IS THIS AUTHOR'S OWN AND HAS NEVER BEEN ATTACKED.** It is entered
because fact 1 produced it, and it is marked so a reader can discount it exactly
as far as that warrants.

| | |
|---|---|
| **What it does** | After the handshake, compare the script's `$CONFIG` against the engine's own `id config` line and refuse on disagreement; likewise assert `engine_id candidate_policy` is the one the selected document commits |
| **Cost** | **MEASURED: the two values are already both in the record** (fact 1), so the change is a comparison and a refusal, not new plumbing. **ESTIMATED 1–2 tests** |
| **Failure modes** | (i) **IT IS NOT A RIVAL.** It answers "was the record taken at the config that was asked for", not "how is the config asked for", so it composes with N-E, N-M, N-J, N-K or N-L and replaces none of them. A matrix that selected it would have selected nothing about the seam. (ii) Its criterion could be read as internal agreement — but the two sides are the script's variable and the ENGINE's report of what it loaded, which do not share an input, so it is a genuine cross-check rather than a component agreeing with itself. (iii) It cannot fire until an arm exists that can request the wrong config, so it has no value under N-L |

---

# RECOMMENDATION

**N-M — the required closed-enum selector.**

Grounds, in the order they bind. Each is MEASURED and each is DIFFERENTIAL — it
separates N-M from a named rival — and none rests on a precedent decision (R9).

1. **THE GROUND THE PREVIOUS RECOMMENDATION RESTED ON IS VOID, AND IT IS VOID FOR
   EVERY ROW.** Round 2's surviving ground was *"refuse the silently-wrong-config
   record"*. Fact 1 shows there is no silent record to refuse: the config's path
   and content digest, the engine's own `id config` report, and
   `engine_id candidate_policy radius 2` all sit above the marker on invariant
   lines. A record taken at the wrong config is WRONG AND ATTESTED THREE WAYS. So
   that ground discriminates nothing, this matrix does not use it, and what it was
   reaching for is N-P's agreement check, which is orthogonal to the seam.
2. **Against N-E and N-J, on fact 3's measured guard asymmetry.** The script
   already carries a printable-ASCII allow-list on the BASENAMES of the two
   caller-named fixtures, because D-232's newline injection into the invariant
   block was REPRODUCED. The config line writes a WHOLE PATH (line 440), not a
   basename, so an option admitting a caller-named config owes that guard extended
   to whole paths — a strictly wider check than the one that exists, on the
   instrument whose review round closed recently. **N-M owes zero path guards
   because it admits no path.** This is a difference in the guard surface, measured
   from the script's own text, not a preference between flag shapes.
3. **Against N-N, on fact 8.** Binding the config through the fixture changes the
   fixture's bytes, hence its sha256 — and that digest is the comparability key
   between the two records the whole exercise exists to compare, is D-184's pin,
   is cited in D-220's registered provenance, and is read by a second consumer.
   N-M changes no fixture and no digest.
4. **Against N-L and N-B′, on what the instrument must hold at once.** The
   registered quantity is a BEFORE and an AFTER of the same workload (fact 1's
   record is the BEFORE, re-taken this session at 33 s). N-L re-pins one literal
   and can express one config at a time; N-B′ moves the committed default, which
   rule 6 and D-204 put behind the operator's SPRT rather than behind an
   instrument's convenience. N-M expresses both, by construction, with no default
   to forget.

**What this recommendation does NOT claim:** that N-M beats N-K. N-K — one
invocation, both configs, both records — removes the confound structurally and
was never attacked on substance. It is not recommended here only because its cost
is unmeasured in every revision including this one, and this matrix declines to
recommend an option whose cost nobody has measured. **If the architect wants the
stronger option, the measurement to order is N-K's, not another round of
argument.**

## THE WEAKEST CELL, named so the red team starts there

**Ground 2 is the load-bearing one and it prices a guard that does not exist
yet.** Fact 3 measures the asymmetry (basename guarded, whole path unguarded) but
the injection it protects against is D-232's REPRODUCED defect on the corpus path,
not a reproduction on the config path — because there is no config flag to
reproduce it through. So ground 2 is an inference from a measured asymmetry, not a
measured exploit. A red team can attack it two ways: build the flag in a worktree
and show the injection is refused anyway by something else in the script, or show
the guard extension is one line and therefore not a difference worth selecting on.

Second: **N-M's arms are literals, and ground 1's own logic can be turned on
them.** If the objection to a code-side default is that a tunable's value should
live in one schema place, then a token→path map in the script is that same shape
with an extra step, and the honest answer is only that nothing is chosen when the
caller says nothing.

## WHAT FLIPS IT

- **A registered snapshot at a config outside the committed pair is needed** — an
  experimental document, or a third registered config (fact 6 shows even the
  second does not exist yet). Remedy: **flip to N-E**, with fact 3's guard extended
  to the whole path and the three refusals it implies stated in the ADR line.
  Reachable: the arm's parsing changes from a token match to a path plus guards.
- **The measurement of N-K's cost is taken and is small.** Remedy: **flip to
  N-K**, which dominates N-M on the confound and subsumes its selector. Reachable
  and cheap to trigger — it is a costing exercise, not a design round.
- **A reviewer holds the enum's token→path map to be a code-side default under
  rule 1.** Remedy: the map moves into a committed document the script reads —
  which is **N-J** in a narrower form — and the ADR line records that rule 1 was
  read as binding the MAP and not only the DEFAULT.
- **NOT a flip trigger:** whether `tools/baseline_snapshot.sh` also gains N-P's
  agreement check. That is a separate named decision; selecting it changes no cell
  in this table, and refusing it changes none either.

## COST OF THE DECISION THIS MATRIX FEEDS

Selecting costs one fresh-context DECISION-RED-TEAM dispatch. Re-running every
fact above costs **MEASURED 33 s** for fact 1's snapshot (the dominant one; it is
a real run of the shipped instrument, not a stand-in) plus a handful of greps.
**The governed run this seam exists for costs one BEFORE and one AFTER at 33 s
each on this machine, and the AFTER cannot be taken at all until
`configs/instrument_staged_v0.toml` exists (fact 6).** At that price, doubt about
any number here is answered by REPLICATION, not by a margin.

---

*Matrix M4, revision 3. Authored by a session that authored neither prior
revision, scheduled by the architect under R9. Thirteen rows: ten carried with
their round-2 standings, two entered because D-318 named them missing, one entered
because this session's own measurement produced it and marked as this author's.
NOT SELECTED. Awaits a third fresh-context DECISION-RED-TEAM.*
