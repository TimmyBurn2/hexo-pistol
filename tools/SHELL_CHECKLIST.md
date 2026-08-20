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

The EXIT trap's LAST command decides the script's exit status: a housekeeping
listing that fails turns a completed run into a failure. Take `local rc=$?` as
the trap's first statement and `return "$rc"` as its last. A second
`trap … EXIT` REPLACES the first — one trap, one cleanup, and a second temporary
directory goes inside the first. Order matters where the commands interact
(`git worktree prune` declines to prune a directory that still exists).

## 8. One spelling per number, one refusal per reason

`[ 010 -ge 1 ]` is true because bash reads octal; the engine then reads decimal
10 and the record quotes `010`. `+50000` and ` 50000` pass a numeric test and
land in an invariant line unnormalized. Validate the SPELLING, not just the
value. And a single combined test gives a wrong diagnosis: `command -v` declines
a directory and an unfindable name identically, and ACCEPTS a FIFO that then
blocks every read — three reasons, three refusals.

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
