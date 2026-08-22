# MATRIX M4 — **STOPPED AFTER THREE ROUNDS, NOTHING SELECTED.** The stop is the RECORDED TIE, and the tie has a cause

**Status: THREE AUTHORED REVISIONS, THREE FRESH-CONTEXT DECISION-RED-TEAMS, NO
SELECTION.** Revision 3 is `docs/experiments/matrix_M4_snapshot_config_seam_rev3.md`,
authored `9ce863f`; its attack is `docs/experiments/matrix_M4_REDTEAM_round3.md`.
Revisions 1 and 2 and their attacks are
`docs/experiments/matrix_M4_snapshot_config_seam.md`,
`matrix_M4_REDTEAM.md`, `matrix_M4_REDTEAM_round2.md`; D-318 records the second
stop. This stop is **D-324**.

> **NEITHER REVISION 3 NOR ITS RED TEAM IS EDITED BY THIS RECORD.** Everything
> here is stated above them, not inside them. A matrix corrected after its attack
> is a matrix that was never attacked.

## WHICH STOP THIS IS

Not the "every option fell" stop: **N-E, N-J, N-K, N-F and N-L all survive**, and
the ground round 2 and round 3 both aimed at N-E and N-J is now measured away
rather than argued away.

It is the disposition the round was pre-authorised to reach: *if no ground
discriminates after the field, the TIE ITSELF IS THE FINDING — record which
options are equivalent and under what measurement they would separate, stop, and
the architect selects on the recorded tie or orders the measurement.* That is
where this round arrives, and it arrives with a cause the two previous rounds did
not have.

## THE CAUSE — THE FIELD HAS TWO ORTHOGONAL AXES AND HAS BEEN SCORED AS ONE

This is the finding, and it is what three revisions of argument were actually
about:

- **AXIS A — how the config is NAMED.** N-A′ (optional path), N-E (required
  path), N-M (closed-enum token), N-N (through the fixture), N-Q (containment-
  guarded path, MISSING), N-G (environment), N-L (a literal, re-pinned by ADR).
- **AXIS B — how many RECORDS one invocation produces.** N-K (two configs, two
  records, one invocation) against everything else's one.

**N-K is not a rival of N-E, N-M, N-Q or N-L. It composes with one of them** — the
red team says so in its own strongest-attack line: N-K *"owes on its second config
exactly the caller-named-path obligation N-E owes on its first."* A matrix that
puts a two-record MODE in the same column as three naming MECHANISMS cannot
select, because its rows are not alternatives. That is why every round has found
its recommendation resting on a ground that "argues equally for" four or five
rows: the rows were never competing for the same slot.

## THE RECORDED TIE, stated so the architect can select on it without re-reading three rounds

**On axis A, after this round, nothing discriminates N-E from N-M from N-Q.**

- The ONE differential ground revision 3 offered — the guard surface — is
  **MEASURED away**. D-232's newline line-injection class is already refused on
  the config path by `digest()`'s hex-shape check (GNU `sha256sum` prefixes an
  escaped filename with `\`), reproduced at exit 1 with a named refusal; the
  residue (TAB, ESC, U+2028) is closed by **three lines** copying the guard
  already at `tools/baseline_snapshot.sh:289`.
- What survives is a **judgement**, not a measurement: whether a token→path map in
  a script satisfies hard rule 1's mechanism (*"a default lives in exactly one
  schema place"*) as well as it satisfies its vocabulary. The red team could not
  call N-M a breach and could not call the distinction sound: `Budget` is a closed
  enum of KINDS whose values live in the schema; N-M is a closed enum of VALUES
  living in the script.
- **N-Q is not in the field at all**, and by the red team's reading it dominates
  N-M on N-M's own failure mode (a config the enum does not name cannot be
  measured) and N-E on guard surface (only `configs/<basename>` can reach the
  record, so the EXISTING allow-list loop covers it for one word). It is this
  project's own `tools/SHELL_CHECKLIST.md` item 11, and the round bound to that
  checklist answered none of its items by name.

**On axis B, N-K's only stated blocker is gone.** Three revisions declined to
recommend it solely because its cost was unmeasured; the red team measured it —
**8 added lines, 0 removed, no re-indentation, two complete records at two
policies from one invocation, ~2× wall (2 × 33 s on the registered corpus)** — and
revision 3's own flip clause reads *"The measurement of N-K's cost is taken and is
small. Remedy: flip to N-K."* The clause was registered before the measurement,
which is the whole point of registering it. **It fires.** What it does not do is
settle axis A, because N-K still has to name its two configs somehow.

**Under what measurement axis A would separate:** none that this session can
identify. The remaining question is a rule-1 reading, and the red team's own
verdict is that it is a judgement the matrix conceded it had no answer to. The
architect either selects on the recorded tie — N-K on axis B, plus one of
{N-E + three guard lines, N-M, N-Q} on axis A — or orders a fourth round scoped to
axis A ALONE with N-Q authored into it.

## THE AUTHOR'S OWN DEFECTS — recorded first-person, because the pattern is the finding

D-318 recorded three MEASURED cells in this work package that did not reproduce,
all favouring their author's recommendation, and said the recurrence rather than
the instance was the finding. **It recurred here, in the revision written to break
it, and the author is this session.** Both decisive ones were re-run by this
session before this record was written, and both reproduce against it:

1. **Fact 4's "verbatim output" is not verbatim.** The pasted block carries ten
   lines; the command returns more, and the omitted one is
   `tools/bench_delta.sh:120`, a site that NAMES the shipped script and is in scope
   by fact 4's own definition. Re-run by this session at HEAD: the same filter
   returns **12** lines. The derived sentence — *"Six of the ten hits are
   comments"* — is wrong twice. Under R7 that kills N-M's cost cell's only MEASURED
   support.
2. **Fact 1's READING is false, and it is the one that carried ground 1.** The
   matrix called `engine_id config` *"the ENGINE's report of the document it
   actually loaded — a referent that does not share the script's variable"*.
   Re-run by this session:

   ```
   $ sed -n '130,136p' crates/pistol-cli/src/bin/pistol.rs
   fn identity_lines(path: &Path, config: &Config) -> Vec<String> {
       let pistol_engine::config::CandidatePolicy::Radius { radius } = config.search.candidate_policy;
       let mut lines = vec![
           format!("config {}", path.display()),
   $ printf 'pistol\nquit\n' | ./target/release/pistol --config ./configs/../configs/instrument_v0.toml 2>/dev/null | grep '^id config'
   id config ./configs/../configs/instrument_v0.toml
   ```

   It is `format!("config {}", path.display())` — the argument echoed back. It IS
   the script's variable. **The record is attested TWO ways, not three**, and the
   only content-derived attestation is `engine_id candidate_policy radius 2`.
   Ground 1's conclusion (round 2's ground discriminates nothing) survives on that
   narrower footing; the fact cell does not.
3. **Fact 7's paste is edited toward the thesis** — `tactical_v0.txt` line 24 says
   a threshold means nothing without *the search that has to meet it*, and the
   matrix's ellipsis renders it *"nothing without one"*, which reads as a config.
4. **N-P — the row this session added on its own measurement — falls**, and its
   vacuous-criterion defence falls with fact 1: the two sides it called
   independent share the script's variable exactly.
5. **Three numbers unmarked**, including a "MEASURED zero path guards owed" with
   no command, in the recommendation's own cost cell, in a document whose §3
   promises marks *"including the zeros"*.

**That is now three distinct authoring sessions producing this pattern in one work
package.** It is recorded here rather than in a finding list somebody else keeps.

## TWO DEFECTS IN THE SHIPPED INSTRUMENT, found in passing and owned by no row

Both are `tools/` findings, not matrix questions, and both are OPEN:

- **A SPACE in a caller-named `--corpus` path reaches the record unescaped**, exit
  0, under the COMPLETE kind token — breaking the record's own leading-tokens
  parse rule and shifting the digest field. The printable-ASCII allow-list admits
  it by construction. Reproduced by the red team on the shipped, unpatched script.
- **`crates/pistol-cli/src/report.rs:151-162` rewrites control characters to `?`**
  in the engine's handshake while the script writes its own copy raw, so a
  record's two config lines can disagree for a reason that has nothing to do with
  the config.

## WHAT THIS BLOCKS — unchanged from D-318, now for a third round

`tools/baseline_snapshot.sh` still has no config seam, so **no Staged snapshot can
be taken** and the registered above-marker quantity still has a BEFORE and no
AFTER. **U4-Z item 15 stays blocked and B2 — M4 has no ADR line — stays open.**
Independently of the seam, the AFTER is blocked on `configs/instrument_staged_v0.toml`,
which does not exist (MEASURED, revision 3 fact 6) and which no row produces.

## WHAT A FOURTH ROUND OWES, if the architect orders one rather than selecting

1. **Split the axes.** Axis A is the decision; axis B is a separate named decision
   whose own flip clause has already fired toward N-K.
2. **N-Q authored into the axis-A field** — the containment-guarded path — with its
   guard measured rather than argued.
3. **`tools/SHELL_CHECKLIST.md` ANSWERED ITEM BY ITEM AND BY NAME**, because the
   subject is a `tools/` script and three revisions have said "answered item by
   item" inside cost cells while answering none. Item 11 (containment) is the
   missing row's own name; item 12 (VOID vs FAIL) is unclassified for every
   naming option.
4. **No MEASURED cell without its command AND its complete output**, and a
   re-read of every pasted block against the command that produced it — which is
   where two of this round's three defects came from.

---

*Stop record for MATRIX M4 after round 3, written at the revision that carries the
red team. Nothing is selected. The tie is recorded, its cause is named, and the
author's own defects are in it.*
