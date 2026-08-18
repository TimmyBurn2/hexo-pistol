#!/usr/bin/env bash
#
# Every CI gate, runnable locally. CI runs exactly this script; there is no
# second, truer definition of the gates living somewhere else.
#
# Usage: tools/ci.sh
# Exit:  0 all gates pass, 1 a gate failed.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

step() { printf '\n=== %s\n' "$*"; }
fail() { printf 'ci: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
command -v git >/dev/null || fail "git is not on PATH"
git rev-parse --is-inside-work-tree >/dev/null 2>&1 ||
	fail "not a git repository: one of the gates builds the git-tracked file set"

# First because it is instant and needs no build: fastest possible feedback
# (docs/decisions.md D-30).
step "gate 1/12: cargo fmt --all --check"
cargo fmt --all --check || fail "formatting: run \`cargo fmt --all\`"

step "gate 2/12: build from the git-tracked file set"
# The point of this gate is to catch a build that depends on a file nobody
# tracked. The tracked set is the git index: it equals HEAD on a fresh checkout,
# and equals the about-to-be-committed tree when work is staged, so the gate
# gives the same answer before and after a commit (docs/decisions.md D-26).
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/repo"
git checkout-index --all --prefix="$WORK/repo/"
echo "ci: building $(git ls-files | wc -l) tracked files in $WORK/repo"
(cd "$WORK/repo" && cargo build --workspace --locked --quiet) ||
	fail "the tracked file set does not build on its own: something the build needs is untracked"
rm -rf "$WORK"
trap - EXIT

step "gate 3/12: cargo test --workspace --locked"
cargo test --workspace --locked || fail "tests"

# --all-targets so tests and examples are linted too, which is strictly more
# than CLAUDE.md asks for and costs nothing.
step "gate 4/12: cargo clippy --workspace --all-targets -- -D clippy::all"
cargo clippy --workspace --all-targets --locked -- -D clippy::all || fail "clippy"

step "gate 5/12: artifact rejection"
tools/artifact_check.sh || fail "artifact check"

step "gate 6/12: config validation"
tools/config_check.sh || fail "config check"

step "gate 7/12: perft oracle"
tools/perft_check.sh || fail "perft oracle"

# The determinism law's executable form (CLAUDE.md rule 4, docs/decisions.md D-7).
# It runs last of the two engine gates because it is the slowest: two processes
# over the whole sha-pinned fixture set at two budgets.
step "gate 8/12: tactical fixture at its pre-registered threshold"
tools/tactical_check.sh || fail "tactical fixture"

step "gate 9/12: cross-process determinism"
tools/determinism.sh || fail "determinism"

# The search's oracle, and the last of the correctness gates because it is the
# longest: a full-width reference pays the candidate count squared per turn, so
# the third turn it certifies is minutes of release CPU. It runs after the
# determinism gate for the same reason that one runs after the tactical gate —
# the cheapest thing that can fail should fail first (docs/decisions.md D-106,
# D-120).
step "gate 10/12: differential search oracle"
tools/search_oracle_check.sh || fail "search oracle"

# The judge itself. It runs after the engine gates because it USES the engine:
# a smoke run that failed because the engine was broken would be a confusing
# place to find that out. It is a self-match, so its expected answer is knowable
# in advance and is asserted exactly, and it repeats the run to cover the
# arena's own determinism — which nothing else in this suite does
# (docs/decisions.md D-169).
step "gate 11/12: arena self-match smoke"
tools/arena_smoke.sh || fail "arena smoke"

# CLAUDE.md rule 9's soft cap. Last because it is the only gate that reads the
# tracked files rather than building them, so it costs nothing to put it where a
# reader looks for the summary (docs/decisions.md D-131).
step "gate 12/12: file-justification check"
tools/file_justification_check.sh || fail "file justification"

printf '\nci: all gates passed\n'
