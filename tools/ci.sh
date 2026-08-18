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
step "gate 1/9: cargo fmt --all --check"
cargo fmt --all --check || fail "formatting: run \`cargo fmt --all\`"

step "gate 2/9: build from the git-tracked file set"
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

step "gate 3/9: cargo test --workspace --locked"
cargo test --workspace --locked || fail "tests"

# --all-targets so tests and examples are linted too, which is strictly more
# than CLAUDE.md asks for and costs nothing.
step "gate 4/9: cargo clippy --workspace --all-targets -- -D clippy::all"
cargo clippy --workspace --all-targets --locked -- -D clippy::all || fail "clippy"

step "gate 5/9: artifact rejection"
tools/artifact_check.sh || fail "artifact check"

step "gate 6/9: config validation"
tools/config_check.sh || fail "config check"

step "gate 7/9: perft oracle"
tools/perft_check.sh || fail "perft oracle"

# The determinism law's executable form (CLAUDE.md rule 4, docs/decisions.md D-7).
# It runs last of the two engine gates because it is the slowest: two processes
# over the whole sha-pinned fixture set at two budgets.
step "gate 8/9: tactical fixture at its pre-registered threshold"
tools/tactical_check.sh || fail "tactical fixture"

step "gate 9/9: cross-process determinism"
tools/determinism.sh || fail "determinism"

# Gates CLAUDE.md names that have nothing to run against yet. They are listed
# here, and not silently absent, so that adding the work package that creates
# them also means deleting its line from this list.
printf '\n=== pending gates (nothing to run yet)\n'
echo "  file-justification check — OWED: a file now exceeds the soft cap and"
echo "    carries its why-justification by hand (crates/pistol-search/src/pvs.rs)."
echo "    Mechanizing it needs a marker convention (docs/decisions.md D-118)."

printf '\nci: all gates passed\n'
