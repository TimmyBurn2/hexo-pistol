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
step "gate 1/6: cargo fmt --all --check"
cargo fmt --all --check || fail "formatting: run \`cargo fmt --all\`"

step "gate 2/6: build from the git-tracked file set"
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

step "gate 3/6: cargo test --workspace --locked"
cargo test --workspace --locked || fail "tests"

# --all-targets so tests and examples are linted too, which is strictly more
# than CLAUDE.md asks for and costs nothing.
step "gate 4/6: cargo clippy --workspace --all-targets -- -D clippy::all"
cargo clippy --workspace --all-targets --locked -- -D clippy::all || fail "clippy"

step "gate 5/6: artifact rejection"
tools/artifact_check.sh || fail "artifact check"

step "gate 6/6: config validation"
tools/config_check.sh || fail "config check"

# Gates CLAUDE.md names that have nothing to run against yet. They are listed
# here, and not silently absent, so that adding the work package that creates
# them also means deleting its line from this list.
printf '\n=== pending gates (nothing to run yet)\n'
echo "  perft oracle             — needs pair-move generation (pistol-core)"
echo "  determinism self-test    — needs a search (pistol-search, pistol-engine)"
echo "  file-justification check — needs a file that exceeds the soft cap"

printf '\nci: all gates passed\n'
