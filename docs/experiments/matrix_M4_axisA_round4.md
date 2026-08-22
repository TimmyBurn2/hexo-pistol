# MATRIX M4, AXIS A — round 4, scoped to axis A alone, with N-Q authored into it

**This document does one thing.** D-324 stopped M4 on a recorded tie and offered
the architect two dispositions: select on the tie, or *"order a fourth round
scoped to axis A ALONE with N-Q authored into it"*. This is that round. It does
not reopen axis B — D-324 records that axis B's flip clause has already FIRED
toward **N-K**, on a measurement taken before this round began — and it does not
relitigate the ten rows whose standings round 3 carried. **The field here is
{N-E, N-M, N-Q} and nothing else**, because that is the tie D-324 recorded.

Authored at `a102c6a`. All measurements pinned at **`b067d47`**, in an isolated
worktree; `b067d47` matters because it changed the very guard these options are
costed against.

## WHO AUTHORED THIS, AND THE R11 SPLIT DECLARED ON ITS FACE

Authored by the session dispatching WP-1.5b's design closure. **Under D-328
(architect ruling R11) this author wrote NO MEASURED CELL.** Every `MEASURED`
value below was produced by a separate measurement agent with no stake in the
outcome, instructed not to recommend, which ran each command and returned its
complete output; this document cites those logs. The grounds, the failure modes
and the recommendation are this author's and are attackable as such.

D-328's ground is that three consecutive sessions of this work package shipped
cells marked MEASURED that did not reproduce, each moving toward its author's
recommendation. **The split earned its place in this round rather than merely
being observed**: the measurer falsified the one number this author would
otherwise have inherited and repeated — see FACT 5.

---

# THE DIFFERING-GROUND TEST, and why N-Q passes it

**The test.** A row admitted into a field that has already STOPPED must state a
ground on which it is not equivalent to any member of the recorded tie — a ground
that, if measured, would place it where no tied row sits. A ground on which the
candidate merely joins one tied row against another is not differing ground: it
re-argues the tie instead of breaking it, and re-argument is what three rounds
already spent. Applied to N-Q against the tie D-324 records on axis A, which is
{N-E, N-M}: **against N-M, N-Q differs twice.** It closes N-M's own stated
failure mode (ii) — *"a config the enum does not name cannot be measured at
all"* — because it admits any `configs/<basename>` rather than two enumerated
names; and it does not raise the hard-rule-1 question AT ALL, where that question
is the single thing D-324 names as axis A's surviving obstacle (*whether a
token→path map in a script satisfies hard rule 1's MECHANISM as well as its
VOCABULARY, given that `Budget` is a closed enum of KINDS whose values live in
the schema while N-M is a closed enum of VALUES living in the script*). N-Q puts
no enum of values in the script, so the judgement that stopped the round is not
asked of it. **Against N-E, N-Q differs on what can reach the record**, which
FACT 4 measures one escape at a time rather than arguing as a class.
**VERDICT: ADMISSIBLE.**

**Two cautions are recorded WITH the verdict rather than after it**, because both
are the row's soft spots and a red team should not have to find them unaided.
First, the round-3 attack costs N-Q at *"adding one word"* to the line-289 loop;
that phrasing counts the GUARD and omits the containment predicate, which is
N-Q's own mechanism and is not free — FACT 6 measures how far off it is. Second,
the round-3 attack claims N-Q *"is this project's own `tools/SHELL_CHECKLIST.md`
item 11"*. **It is not, and this author records the over-claim rather than
inheriting it.** Item 11's own scope is *"Any binding consumed by `rm`, `mv`, or
a write"*, and `$CONFIG` is consumed by `[ -f ]`, by `digest`, by the engine
invocation, and by an `echo` into the record — a READ and a record write of the
VALUE, not a write to the PATH. The item that governs `$CONFIG` is **item 9**
(*what reaches a record is caller-controlled*). Item 11's reasoning pattern is
still the right shape to borrow; its authority is not available to be cited.

Admissibility is not discrimination. Whether N-Q separates the field is decided
by the facts below and by the attack on them.

---

# THE MEASURED FACTS

Cited from the measurement agent's logs, pinned at `b067d47`. The agent's
counting rule, stated by it and applied identically to all three options: each
patch applied to a clean `b067d47` checkout of `tools/baseline_snapshot.sh`
alone; counts are `git diff --numstat` for that one file; an added line is
COMMENT if its first non-whitespace character is `#`, else CODE. No option added
a blank line, so code + comment = added total for all three.

### FACT 1 — the current state. **MEASURED**

`grep -n 'CONFIG=' tools/baseline_snapshot.sh` → one hit, `182:CONFIG="configs/instrument_v0.toml"`.
`./tools/baseline_snapshot.sh --config configs/instrument_v0.toml` →
`baseline_snapshot: FAIL: unknown argument `--config``, exit 1. There is no
caller-facing config seam; the four `--config` occurrences in the file are the
flag passed to the ENGINE, plus the emit-block comment.

### FACT 2 — what each option costs in the script. **MEASURED**

| option | added | removed | added CODE | added COMMENT |
|---|---|---|---|---|
| **N-E** | 22 | 8 | **7** | 15 |
| **N-M** | 21 | 6 | **10** | 11 |
| **N-Q** | 32 | 8 | **12** | 20 |

`bash -n` passes on all three.

### FACT 3 — all three work, and all three refuse an absent flag. **MEASURED**

Each patched script takes a snapshot at exit 0 and writes
`config configs/instrument_v0.toml 3579855e…`. With `--config` absent, each
refuses at exit 1: `--config is required and has no default`. N-M's enum refuses
an unrecognised token by name: ``--config takes `instrument` or `staged`, got `bogus``.

### FACT 4 — N-Q's containment predicate, one escape at a time. **MEASURED**

**No escape got through.** Each case run separately against the shipped patched
script:

| case | result |
|---|---|
| (a) `configs/instrument_v0.toml` | ACCEPT, exit 0 |
| (b) `../etc/passwd`, and `../../../etc/passwd` | REFUSED exit 1, named, resolved path quoted |
| (c) `/etc/hostname` — ABSOLUTE-VALUE ESCAPE | REFUSED exit 1, named |
| (d) `configs/../configs/instrument_v0.toml` | ACCEPT exit 0, **NORMALISED** — record carries `configs/instrument_v0.toml` |
| (e) symlink inside `configs/` → `/etc/hostname` | REFUSED exit 1, named |
| (f) `configs/nonexistent.toml` | REFUSED exit 1, `no config at …` |
| (g) `configs/spaced name.toml` | REFUSED exit 1 — **by the guard, not by containment** |
| (h) absolute path INSIDE `configs/` | ACCEPT exit 0, recorded root-relative |
| (i) `configs` (the directory itself) | REFUSED exit 1 |
| (j) `configs_evil/…` and `configs/../configs_evil/…` — sibling-prefix | REFUSED exit 1 |
| (k) `configs/tab\tname.toml`, `configs/nl\nname.toml` | REFUSED exit 1 on the printable-ASCII arm |

Cases (i) and (j) were run by the measurer unprompted; (j) is the classic
prefix-match bug and the predicate does not have it.

### FACT 5 — **D-324's THREE-LINE CLAIM IS FALSIFIED, AND IT IS WRONG IN KIND AND NOT ONLY IN COUNT.** MEASURED

D-324 records axis A's guard ground as measured away because the residue *"is
closed by THREE LINES copying the guard already at `tools/baseline_snapshot.sh:289`"*.

- The citation was exact at its own revision: at `b067d47^`, line 289 IS that
  guard and it carried ONE arm, so copying it was `case` + 1 arm + `esac` = three
  lines. `b067d47` added the SPACE arm, so the same copy is now **four** code
  lines. That is the small half.
- **The large half: copying that guard does not close the defect at all.** The
  loop guards `${named##*/}`, a BASENAME; the emit block writes `$CONFIG`, a
  WHOLE PATH. The measurer built the literal reading of D-324 — `$CONFIG` added
  to the existing `for named in` loop, no whole-path guard — and drove it:

  ```
  --config 'configs/spaced dir/instrument_v0.toml'
  EXIT:0
  kind token:  baseline_snapshot 1
  config line: config configs/spaced dir/instrument_v0.toml 3579855e…
  4th token a reader would take as the digest: dir/instrument_v0.toml
  ```

  Exit 0, COMPLETE kind token, fields shifted — **the same
  exit-0-wrong-answer class `b067d47` was written to close on `--corpus`,
  reproduced on the config path through the remedy D-324 registered.** A
  caller-named config owes a NEW whole-path guard; the existing loop cannot be
  reused for it without changing the record to carry a basename, which is a
  schema change and was not measured.

### FACT 6 — N-Q's containment does NOT subsume the guard; the costs are additive. **MEASURED**

FACT 4 cases (g) and (k) are the proof: `configs/spaced name.toml` and
`configs/tab\tname.toml` are genuinely under `configs/` and pass containment,
and both break the record. **N-Q owes the same 4 guard lines N-E owes, PLUS 5
containment lines.** The round-3 attack's "one word" is not a small
underestimate of N-Q's guard cost; it is an account of a guard N-Q still needs in
full.

### FACT 7 — N-M's "zero path guards owed" reproduces. **MEASURED**

Under N-M, `$CONFIG` is assigned only at two committed literals. No caller byte
reaches it, and adversarial tokens die at the enum at exit 1 — including
`configs/spaced dir/instrument_v0.toml`, `/etc/hostname`, and a token containing
a newline.

### FACT 8 — the staged config does not exist, and that blocks every option equally. **MEASURED**

`ls configs/instrument_staged_v0.toml` → `No such file or directory`. The full
`configs/` listing carries no `staged` document. **This is NOT an N-M-specific
cost**, and D-324's framing invites reading it as one: N-E and N-Q would be
handed the same missing path. What is N-M-specific is only that its enum names
the missing document by a token.

### FACT 9 — the retrofit cost is identical for all three. **MEASURED**

Making `--config` required breaks existing callers. `git diff --numstat` on
`crates/pistol-cli/tests/baseline_snapshot_tests.rs` → `2 0`, **identical for
N-E, N-M and N-Q**, differing only in the literal. All three then pass the
shipped-script suite 30/30. The measurer's first retrofit of ONE line failed
1/30 for all three, because a second invocation site exists outside the `go()`
funnel — found by running the suite, not by reading it.

### FACT 10 — what the governed run costs. **MEASURED**

One snapshot on the registered corpus: **33.1 s** wall, replicated at **33.2 s**.
The same workload through the N-Q-patched script: **33.0 s**. Invariant blocks
are byte-identical across the replicates AND across the seam. **The config seam
adds no measurable wall time and changes not one byte of the invariant block.**

---

# N-Q — a REQUIRED `--config PATH`, CONTAINMENT-GUARDED TO `configs/`

| | |
|---|---|
| **What it does** | One required arm taking a PATH, as `pistol` itself does. The path is resolved with `realpath -m` and REFUSED unless it resolves under `$ROOT/configs/`; on acceptance it is rewritten to its root-relative form, so the record is independent of where the checkout lives. No default: `CONFIG` starts empty and an absent `--config` is refused by name at exit 1 (FACT 3), so nothing falls back to the literal that stands at line 182 today (FACT 1) |
| **Cost** | **MEASURED, FACT 2: 32 added / 8 removed, of which 12 are CODE** — the most of the three. **MEASURED, FACT 6: 5 containment lines PLUS the same 4 whole-path guard lines N-E owes**, because containment admits a spaced or tab-bearing name inside `configs/` and those break the record. **MEASURED, FACT 9: 2 further lines** to retrofit the test suite, identical for all three options. **MEASURED, FACT 10: no measurable wall-time cost and no change to the invariant block** |
| **Failure modes** | (i) **It is the most expensive row in the field by every count taken** — most added lines, most added code lines — and the extra buys containment on a binding that feeds a READ, which `SHELL_CHECKLIST` item 11 does not require and item 9 does not ask for. (ii) **Its extensibility is dated later than the need.** FACT 8: the second registered config does not exist, so the failure mode N-Q closes in N-M is not yet a live loss for anybody. (iii) **The normalisation in FACT 4(d)/(h) is a behaviour this field never asked for**: the record records the canonical path and not the caller's spelling, which is a reproducibility gain and simultaneously a silent rewriting of a caller-supplied value — and this document does not measure which the matrix should prefer. (iv) It is the first config-scope flag anywhere in `tools/`, so its shape is what later scripts copy — a failure mode it shares with N-M and N-E and which none of them escapes |

---

# THE AXIS-A FIELD AFTER N-Q

| | N-E | N-M | N-Q |
|---|---|---|---|
| added / added CODE (FACT 2) | 22 / **7** | 21 / **10** | 32 / **12** |
| whole-path guard owed (FACT 5, 6) | **4 code lines, NEW** | **none** (FACT 7) | **4 code lines, NEW** |
| containment lines | none | none | **5** |
| retrofit (FACT 9) | 2 | 2 | 2 |
| values enumerated in the script | no | **two paths** | no |
| a config outside the set is measurable | yes, any path | **no** | yes, any `configs/` document |
| record names a committed, re-runnable document | by caller discipline | by construction | **by construction** |
| blocked by the missing staged config (FACT 8) | yes | yes | yes |

---

# RECOMMENDATION — **N-Q**, and the ground is not the one this round was expected to turn on

**The recommendation is N-Q, and rung (a) of the tiebreak ladder is where it is
won.** The ladder's first rung is hard-rule-1 config law conformance — explicit,
closed-enum, deny-unknown, one schema home — and the last of those four is the
one that separates this field.

- All three are **explicit** and all three **deny an absent value** by name at
  exit 1 (FACT 3). Rung (a)'s first and third criteria do not separate them.
- **N-M fails "one schema home".** It is the only row that puts the VALUES — two
  committed document paths — inside the script, which is a second home for a set
  that already has one. This is precisely the disanalogy D-324 could not resolve
  and stated correctly: `Budget` is a closed enum of KINDS whose values live in
  the schema, and N-M is a closed enum of VALUES living in the script. Round 4
  does not resolve that judgement by argument; it declines to need it, because
  two rows do not raise it.
- **Between N-E and N-Q, "one schema home" separates them, and it is the reading
  this author is prepared to be attacked on.** N-E bounds the admissible set
  nowhere: the set of documents a record may name is unstated. N-Q's bound is the
  `configs/` directory — which is not a new home invented for the seam but THE
  home the repository already keeps its configs in. So N-Q's admissible set has
  exactly one home, and that home is a directory listing rather than a literal in
  a script. On rung (a)'s own fourth criterion, N-Q conforms and N-E is silent.

**This is a discrimination and so the ladder stops at rung (a); rungs (b) and (c)
are NOT reached.** Recorded plainly because it matters: **had the ladder reached
rung (b), fewest MEASURED added lines, N-E would have won it** — 22/7 against
N-Q's 32/12 (FACT 2). The recommendation is therefore made against the row that
is cheapest on the next rung down, and this author states that rather than
leaving a reader to compute it.

**The second ground, stated as support and not as the selector**, is what the
record's config line is FOR. It is provenance (D-198): a reader re-runs the run
from it. N-Q is the only option under which "the record names a committed
document at a checkout-independent path" is true BY CONSTRUCTION (FACT 4(d),
(h)) rather than by caller discipline (N-E) or at the price of a second home
(N-M).

## THE WEAKEST CELL, named so the red team starts there

**N-Q's cost cell, and specifically the claim that 5 containment lines are worth
paying.** FACT 6 already destroyed the round-3 attack's case that they were
nearly free. The honest position is that N-Q is the most expensive row in the
field and its extra lines buy a property the checklist does not require for a
read-only binding (item 11 does not reach `$CONFIG`; item 9 does, and item 9 is
satisfied by the guard both N-E and N-Q owe). **If the rung-(a) reading of "one
schema home" above does not hold — if a directory is not a "schema home" in the
sense hard rule 1 means — then rung (a) does not discriminate, the ladder falls
through to rung (b), and N-E wins on measured lines.** That single sentence is
the whole of the attack surface, and it is stated here rather than discovered.

## WHAT FLIPS IT

- **A reviewer holds that `configs/` is not a "schema home" under hard rule 1.**
  Remedy: rung (a) does not fire, the ladder falls to rung (b), **flip to N-E**
  with its 4 whole-path guard lines. Reachable and cheap — it is a reading, not a
  measurement.
- **A registered config outside `configs/` is needed** — an operator's
  experimental document held outside the tree. Remedy: **flip to N-E**; N-Q
  refuses it by construction (FACT 4(c)), which is the cost of its own selector.
- **The normalisation of FACT 4(d)/(h) is judged a defect** — a caller-supplied
  value silently rewritten in a provenance field. Remedy: N-Q keeps the caller's
  spelling and records the resolved path beside it, at a cost this round did not
  measure.
- **NOT a flip trigger:** the missing `configs/instrument_staged_v0.toml`. FACT 8
  blocks all three equally and selects nothing.

## COST OF THIS ROUND, on its own face

This round cost one measurement-agent dispatch and one red-team dispatch. Every
fact above is re-runnable in **seconds** except FACT 10, which is **33 s per
replicate** and was replicated twice. **At that price, doubt about any number
here is answered by RE-RUNNING IT, never by a margin derived to defend it** —
and FACT 5 is what that policy bought this round: the number D-324 registered,
re-run rather than inherited, did not survive.

---

*Matrix M4, axis A, round 4. Authored under D-324's own fourth-round clause by
the session dispatching design closure, with every MEASURED cell produced by a
separate measurement agent under D-328. Field: {N-E, N-M, N-Q}. Recommends N-Q.
NOT SELECTED — awaits a fresh-context DECISION-RED-TEAM.*
