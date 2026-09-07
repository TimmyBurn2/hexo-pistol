# tools/ shell checklist

Every script in `tools/` runs under `set -euo pipefail` and parses output it did
not produce. Three review rounds in a row found the same material class in them,
and it is not "shell is fiddly": it is **EXIT-0-WRONG-ANSWER** — a gate that
prints a verdict, exits 0, and is wrong. A refusal that exits 1 is a working
gate having a bad day. A pass that should have been a refusal is a gate that
does not exist, and nothing downstream can tell.

A REVIEW-impl or RED-TEAM prompt for a change under `tools/` cites this file, and
a reviewer answers its items by name. It is not mechanized: no script checks it.

---

## 1. A command substitution whose status is DISCARDED

`echo "sha256 $(sha256sum "$F" | cut -d' ' -f1)"` cannot fail. The substitution
runs in a subshell, its status is the `echo`'s ARGUMENT, and `set -e` never sees
it: an unreadable `$F` yields an empty field, exit 0, and a complete-looking
record. Take the value into a variable first, check its SHAPE, and refuse by
name. The same applies to `"${x:-$(cmd)}"` and to any substitution inside a
`printf` format or argument.

`VAR="$(cmd)"` does propagate under `set -e` — but a bare `set -e` death prints
nothing, which is rule 3's other failure: name the refusal.

A `fail` inside a command substitution exits **only the subshell**. A helper that
must be able to refuse sets a global and is called as a statement.

## 2. A pipeline in a `then` body is not a pipeline in a condition

`if cmd | grep -q x; then` is exempt from `set -e`. `cmd | grep -q x` as a
statement is not: it kills the script, and under `pipefail` so does a failure
anywhere in the pipeline. This is how a script died one line BEFORE the refusal
written for that exact case. Decide per pipeline: a test goes in a condition, a
side effect gets `|| true` with the reason in a comment, and a real failure gets
a named `fail`.

## 3. `grep` under `pipefail`

`grep` exits 1 on NO MATCH. That is a normal answer, not an error, and under
`pipefail` it takes the whole pipeline down. Every `grep` in a statement position
needs `|| true` (and a comment saying an empty result is legitimate) or a `fail`
that says what was not found. `grep -c` prints `0` and STILL exits 1.

And a substring is not a token: `grep -c 'btree.*search'` matched
`pistol_search` and retired the document its count governed (D-221, D-223).
Anchor, or match the field.

## 4. `LC_ALL`, and which direction it moves a guard

Pin the locale for DETERMINISM — `$EPOCHREALTIME` writes the locale's decimal
separator, `sort` collates by it. But a character CLASS is only as wide as the
locale says, so a correctness guard written as `[[:cntrl:]]` under `LC_ALL=C` is
ASCII-only: it refused LF and admitted U+2028 and U+0085. Write the guard as an
ALLOW-LIST (`*[![:print:]]*`) so the pin makes the refusal as wide as possible,
never as narrow. Ask of every class: does the locale pin make this refuse more or
less?

## 5. The index is what commits; the working tree is not

`git ls-files` names a PATH. Opening that path reads the WORKTREE file of that
name, which is a different file: stage a violation, overwrite the worktree copy
with something harmless, and a gate reading paths passes it while the real bytes
go to HEAD. Read the tracked bytes — `git ls-files -s -z` for path + blob, then
`git cat-file blob` — and never `[ -f "$path" ] || continue`, which turns a
tracked-but-absent file into a silent skip. Count the file set with the SAME
enumeration the loop used, or the summary line will describe a set nobody
counted.

## 6. A sweep by prefix must own the prefix

Anything that DELETES what it did not create is namespaced with a token no other
producer writes. `pistol-` is this workspace's own naming scheme — every crate
directory carries it — so a sweep matching it removed `pistol-core` and three of
its siblings out of a `TMPDIR`, from a test that passed. Scratch directories
carry `pistol-testscratch-`, both suites spell it, and a test says so.

## 7. Traps

A housekeeping command that fails turns a completed run into a failure, and a
deliberate `void` (2) into a `fail` (1) — the misreading item 12 exists to
prevent. **THE MECHANISM IS `set -e`, NOT "the trap's last command wins", AND AN
EARLIER REVISION OF THIS ITEM NAMED THE WRONG ONE.** MEASURED, bash 5.3.15, a
script exiting 2 with a cleanup whose `rm` is refused:

| trap body, under `set -euo pipefail` | cleanup fails | cleanup succeeds |
|---|---|---|
| `rm -rf "$W"` | **1** | 2 |
| `rc=$?; rm -rf "$W"; exit "$rc"` | **1** | 2 |
| `cleanup() { local rc=$?; rm -rf "$W"; return "$rc"; }` | **1** | 2 |
| `rc=$?; rm -rf "$W" \|\| echo … >&2; exit "$rc"` | **2** | 2 |

**TAKING `rc` IS NOT ENOUGH AND BUYS NOTHING ON ITS OWN**: under `set -e` the
failing `rm` terminates the shell where it stands, so the `exit "$rc"` or
`return "$rc"` written to preserve the status is NEVER REACHED, and the first
three rows are one behaviour wearing three spellings. Without `set -e` the status
survives all four (`trap 'false' EXIT; exit 3` exits **3**), which is why reading
the rule rather than running it kept the wrong mechanism in place.

So: take `rc=$?` first, **give every command in the trap its own `|| …` so none
of them can fail**, and `exit`/`return "$rc"` last. A cleanup that could not
clean up says so on stderr — rule 3 wants the reason named — and does not touch
the verdict, because a leaked scratch directory is not a wrong answer and a
masked exit code is. A second `trap … EXIT` REPLACES the first — one trap, one
cleanup, and a second temporary directory goes inside the first. Order matters
where the commands interact (`git worktree prune` declines to prune a directory
that still exists).

## 8. One spelling per number, one refusal per reason

`[ 010 -ge 1 ]` is true because bash reads octal; the engine then reads decimal
10 and the record quotes `010`. `+50000` and ` 50000` pass a numeric test and
land in an invariant line unnormalized. Validate the SPELLING, not just the
value.

**AND A SINGLE COMBINED TEST GIVES A WRONG DIAGNOSIS. `command -v` IS NOT A
USABILITY TEST AND IT ADMITS FIVE OUTCOMES, ONE OF WHICH IS "you may run this".**
MEASURED on this machine, bash 5.3.15, each in a clean `env -i` shell:

| what is on PATH | `command -v` | what running it does |
|---|---|---|
| a real executable | ACCEPTS, absolute path | runs |
| an unfindable name | declines | — |
| a DIRECTORY of that name | declines — **the same answer as unfindable** | — |
| a FIFO | **ACCEPTS**, absolute path | every read BLOCKS, forever |
| a file with no `+x` | **ACCEPTS**, absolute path | exec answers **126** |
| a function, alias or builtin | **ACCEPTS**, and returns the NAME, not a path | runs something that is not the tool |

The last two are the fourth and fifth cases `docs/ROADMAP.md`'s WP-1.10 left
open as "an amendment to item 8 for the fourth case bash admits". They are the
worst two: the check says yes and the run does something else, which is this
file's whole subject. **Six outcomes, five refusals, and each names its own
reason** — `tools/require_tool.sh` is where they live, and a `tools/` script
resolves a program through it rather than through `command -v`. The script keeps
its OWN class: the resolver exits 2 for "this is not a runnable program" and 1
for "you called me wrong", and the caller decides whether that is its `fail` or
its `void`.

**THE DIRECTORY ROW IS WHY THIS IS NOT A ONE-LINE FIX.** `type -t` reports
nothing for it, exactly as for an absent name, so a resolver written on `type`
alone reproduces the conflation it was written to end — which is what the first
draft of `require_tool.sh` did, caught by its own test. PATH is walked to tell
the two apart.

## 9. What reaches a record is caller-controlled

A basename, a label, a revision string: if it is interpolated into a document
somebody parses, a newline in it INJECTS LINES. Guard the value at the boundary,
quote the input back in the refusal, and use `${x##*/}` rather than `basename`
(a command substitution strips the trailing newline the refusal is for).

## 10. THE COVERAGE RULE

**Any `tools/` script that produces a recorded number carries at least one test.**

Not a self-test inside the script — a test in a suite CI runs, driving the
SHIPPED script, in a scratch directory or a scratch git repository, with a
control run so a pass cannot come from a gate that refuses everything.

`tools/bench_delta.sh` produced this project's OFFICIAL perf verdict (D-220) with
zero tests until D-231. Two rounds of defects in it were found by reviewers
running it by hand, which is why two rounds of defects in it went unbound. A
number nothing tests is a number nothing defends.

## 11. A CALLER'S PATH THAT FEEDS A DELETE OR AN OVERWRITE IS CONTAINMENT-GUARDED

**Any binding consumed by `rm`, `mv`, or a write is guarded so that its resolved
path is provably under the root the script means. ABSOLUTE-VALUE ESCAPE IS THE
ATTACK**, and a `cd` is not a guard: `( cd "$ROOT" && rm -rf -- "$P" )` deletes
whatever `$P` names when `$P` is absolute, because a `cd` constrains relative
paths and nothing else.

This is not hypothetical and it is not old. `tools/wp15a_h1.sh` guarded
`SNAPSHOT_REL` against `/*` and `*..*` and left `SUBJECT_PATH` — **the one
binding it `rm -rf`s** — unguarded. An absolute value lying inside the real
repository passed every check above the deletion (`git diff --name-only -- <abs>`
resolves it happily) and **removed the operator's working tree**; at the
registered bindings that is `crates/pistol-solver`, the work package under test.
The refusal that fired named a git pathspec error and never said a deletion had
happened. `..` was already caught, by git refusing the pathspec — so the guard
that existed covered the case that was already covered and missed the one that
was not.

**The overwrite direction is the same defect with a quieter blast radius.**
`tools/baseline_snapshot.sh` `cd`s to `$ROOT` and then writes `--out`; measured,
`--out relative_probe.txt` issued from `/tmp` wrote its record **into the
repository root** — a file the caller never asked for, in a tree whose
cleanliness other gates adjudicate on. A caller's relative path is resolved
against the directory the CALLER was standing in, captured before the `cd`.

**The sweep, not the instance.** This item exists because the fix for the
deletion was scoped to one variable while its parameterised sibling — the one
that deletes — sat one guard away. So: enumerate every destructive site in the
script, trace each target to its ORIGIN, and classify. A target derived from
`mktemp -d` or from `$WORK/...` is script-created and needs nothing; a target
that came from an argument, an environment binding or a config value is
caller-supplied and is guarded or is a finding. None of this by memory — the
enumeration is the evidence.

**Why this is item 11 and not item 10.** Items 1-10 are cited BY NUMBER in
`docs/decisions.md`, in `docs/experiments/`, and in several scripts' own
comments; renumbering to put this beside its siblings would silently retarget
every one of those citations. The coverage rule stays 10 and stays the capstone.

## 12. A GATE DISTINGUISHES **RUN VOID** FROM **FAIL**, BY NAME

**Two different things exit non-zero: "the answer is no" and "I could not take
the answer." A gate that spells them the same way turns every environmental
accident into a regression report**, and the reader who acts on it goes looking
for a defect in the subject.

Not hypothetical, and not old. `/tmp` on this machine is RAM-backed at 24 GiB.
A session filled it; `cargo` then failed with `Disk quota exceeded (os error
122)`; `tools/solver_link_check.sh` did the right thing and exited **2** with
`cannot build the workspace's binaries` — and the standing test that drives it
asserts `exit 0`, so what a reader saw was a red solver-link gate,
indistinguishable in the log from the solver having been linked into a shipped
binary. The gate was honest and the reading was wrong, because nothing carried
the distinction across the seam (docs/decisions.md D-281, D-285).

So, three obligations, and a reviewer asks for each by name:

1. **A code per kind.** `0` the answer is yes, `1` the answer is no, `2` no
   answer was taken. A gate with no void class says so in its usage block rather
   than leaving a reader to infer it from silence.
2. **PREFLIGHT WHAT THE RUN NEEDS AND VOID EARLY.** A gate that writes scratch
   asks whether there is room BEFORE it does the work, and refuses as a void
   naming the filesystem, what is available and what it wanted —
   `tools/scratch_preflight.sh`. Discovering the shortage through a tool's own
   error message is discovering it in the tool's vocabulary, and that vocabulary
   describes the tool, not the gate.
3. **THE DISTINCTION SURVIVES THE SEAM.** A test that drives a gate asserts on
   the code it expects AND says, in the failure message, what the other codes
   would have meant. `assert_eq!(code, Some(0))` is a test that reports a void
   as a regression, which is exactly the reading above.

**Why this is item 12 and not item 3's second half.** Items 1-11 are cited BY
NUMBER in `docs/decisions.md`, in `docs/experiments/` and in several scripts'
own comments; renumbering to put this beside the exit-status items would
silently retarget every one of those citations. That is item 11's own reasoning,
applied to the item that follows it.

---

## APPENDIX — THE `info totals` CONSUMER REGISTER

**Not an item, and deliberately unnumbered.** Items 1-12 are cited BY NUMBER in
`docs/decisions.md`, in `docs/experiments/` and in several scripts' own
comments, and this is a register rather than a rule a reviewer answers — item
11's reasoning about renumbering, applied to something that is not an item at
all.

**WHY IT EXISTS.** `docs/audit/repo_audit_2026-09.md` row A-08 found the
`info totals` grammar parsed in eight homes, each keying on its own literal, and
priced a shared reader as a `tools/` package of its own. **A shared reader is
NOT built here.** What the register buys instead is the thing an audit had to
re-derive: a change to the grammar has a list of everything that reads it, so
the next `git grep` is not the only way anyone finds out.

**THE PRODUCER OF RECORD** is `crates/pistol-cli/src/report.rs:33-39` —
`TOTALS_MARKER`, D-80's marker discipline, and the reason the closing line is
distinguishable from a per-depth one at all. Two more producers exist and must
match it because drivers parse what they emit:
`crates/pistol-arena/src/bin/stub_engine.rs:631` and
`tools/sealbot/tests/stub_pistol.py:86`.

**THE CONSUMERS.** The enumeration is

```
git grep -n totals -- tools 'crates/*/src/*' ':!tools/SHELL_CHECKLIST.md'
```

read and classified by hand — a site that FOLDS already-parsed numbers, such as
`crates/pistol-arena/src/record.rs:113`, is not a consumer of the grammar and is
not listed. **TWO EARLIER REVISIONS OF THIS APPENDIX CITED COMMANDS THAT COULD
NOT HAVE PRODUCED THIS TABLE, AND THE SECOND WAS WRITTEN TO FIX THE FIRST.**
Revision one searched for `info totals` and for `' totals '` in quotes, and an
`awk '/ totals /{` puts the discriminator between SLASHES, so it matched neither
`awk` consumer — including the row it advertised as the one the audit had
missed. Revision two narrowed the pattern to `" totals "` and added the pathspec
`crates/*/src`, and **it reached SEVEN of the eleven**: a git pathspec's `*` does
not cross `/`, so `crates/*/src` matches no file at all (`git grep -c "" --
'crates/*/src'` returns nothing), and rows 1-3 do not contain the padded string
in the first place — they key on the bare token (`words.contains(&"totals")`,
`totals_of`, `a totals line`). **THE PATTERN AND THE PATHSPEC WERE BOTH WRONG,
AND FIXING ONLY THE PATHSPEC WOULD STILL HAVE MISSED THREE ROWS.** That is
`docs/process.md`'s named class twice over, a claim checked against the wrong
population, inside the register whose closing line demands re-derivation — the
lesson being that a re-derivation command is itself an instrument and is RUN
before it is published, not read.

The command above REACHES every one of the eleven: each row below has at least
one line in its output. It is deliberately a superset — 148 lines, against 35
for the revision that reached seven — because a hand-classified register wants
over-approximation, and the cost of a line that is read and rejected is nothing
beside the cost of a consumer never seen. Row 4's second site
(`pistol_client.rs:241`) matches through the const `TOTALS_MARKER` declared at
`:41`, which is the line the command returns; the classifier reads the file from
there.

| # | site | what it keys on |
|---|---|---|
| 1 | `crates/pistol-arena/src/exchange.rs:66-69` | `words.contains(&"totals")` — the per-depth/closing discriminator, negated |
| 2 | `crates/pistol-arena/src/exchange.rs:242-251` | `fields_of` plus `nodes`, `TIME_FIELD` and `depth_turns` by name |
| 3 | `crates/pistol-arena/src/capture.rs:59-90` | finds `nps` and requires `time` to follow it, to strip the two wall-clock fields |
| 4 | `tools/sealbot/matchserver/src/pistol_client.rs:41,241` | the substring `" totals "` |
| 5 | `tools/cold_label_check.py:234,241` | `line.startswith("info totals ")`, with a named refusal when none was written |
| 6 | `tools/baseline_snapshot.sh:501,648` | `grep ' totals '`, then one field by name |
| 7 | `tools/bench_block.sh:260,264` | `grep -c '^info totals '`, then `sed -n 's/^info totals //p'` |
| 8 | `tools/bench_delta.sh:379-390` | `awk '/ totals /'` |
| 9 | `tools/determinism.sh:190` | `grep -c '^info totals depth_turns [1-9][0-9]* '` |
| 10 | `tools/movetime_check.sh:131` | `sed -n 's/^info totals .* time \([0-9]\+\) .*/\1/p'` |
| 11 | `tools/staged_cover_bench.sh:147-157` | `awk '/ totals /'`, `nodes` and `time` by field name — the same reader as row 8 |

**ELEVEN SITES IN TEN FILES, WHERE A-08 SAID EIGHT — and the difference is the
point of writing them down.** Three are new to this register:
`tools/staged_cover_bench.sh` (found by the review of the round that wrote this
appendix, and missed by that round's own command),
`crates/pistol-arena/src/capture.rs` (A-08 named `exchange.rs` and not the
capture pass's own normalisation, which parses the same line for a different
reason) and `tools/bench_delta.sh` (A-08 missed it; it produces this project's
OFFICIAL perf verdict, D-220). One of A-08's is gone rather than fixed:
`artifacts/wp20b_perf_guard.sh` was uncommitted and `artifacts/` is not tracked
(CLAUDE.md rule 8), so it took its reader with it.

**AND THE PER-DEPTH FORM IS THE SAME GRAMMAR.** `tools/baseline_snapshot.sh`
parses `^info depth_turns ` at `:739` and `:780` as well as the totals line at
`:501` and `:648`, so a change following this table alone would check two of
that file's four sites. The rows above are the CLOSING line's consumers; a
grammar change is not finished at them.

**WHAT A CHANGE TO THE GRAMMAR OWES.** Every row above, checked; the three
producers, checked against each other; the per-depth sites; and this table
re-derived with the command above, because a register that is not re-derived is
a list, and D-671 is this project's own record of what a list does when the
population moves under it.
