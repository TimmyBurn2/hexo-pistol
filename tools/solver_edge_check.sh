#!/usr/bin/env bash
#
# Does anything in a cargo workspace take a NORMAL dependency on one crate?
#
# Usage: tools/solver_edge_check.sh <workspace-root> <crate-name>
# Exit:  0 no normal reverse-dependency anywhere in the workspace
#        1 at least one, named on stdout
#        2 the question could not be answered (RUN VOID, never a verdict)
#
# WHAT THIS IS FOR. The WP-1.5a pre-registration's `p = 0` claim is that no
# binary in this workspace links `pistol-solver`, and H1 rests entirely on it.
# This script is the adjudicator, and it is a SEPARATE FILE because the previous
# two adjudicators were both wrong and neither was testable: they lived inside a
# pre-registration's prose, so every defect in them was found by a reviewer
# running the document by hand (docs/decisions.md D-231's lesson, and
# tools/SHELL_CHECKLIST.md item 10's rule). Taking a workspace root and a crate
# name as ARGUMENTS is what lets a test drive the shipped script against scratch
# workspaces it controls, with a control run, rather than only against the one
# repository whose answer is already known.
#
# WHY NOT A SUBSTRING COUNT OVER `*Cargo.toml`. That was the adjudicator through
# revision 9 and it is wrong in both directions, both measured: a COMMENT naming
# the crate in an unrelated manifest produced "2 or more means an edge" on a tree
# with no edge, and a real `[dependencies."pistol-solver"]` entry — valid
# TOML that cargo resolves — was INVISIBLE to it. Manifest text is not the
# dependency graph.
#
# WHY NOT `cargo tree -i`'s EXIT STATUS. That was revision 10's adjudicator and
# it is wrong too, measured three ways: a `[dev-dependencies]` entry, a
# `[build-dependencies]` entry and an off-target `[target.'cfg(windows)']` entry
# each exit 0 while `--edges normal` leaves the tree EMPTY, so "exit 0" read as
# "a normal edge exists" turns the legitimate dev-dependency the oracle's
# test-tree home relies on into a refutation of p = 0. Status also collides three
# ways at 101: no such package, an AMBIGUOUS specification, and any other cargo
# failure. The exit status of this command answers a different question than the
# one being asked.
#
# WHAT IS ADJUDICATED ON: the STDOUT of a workspace-wide inverted tree. Measured
# on this workspace:
#
#   no edge anywhere              -> 1 line   (the crate's own root line)
#   a normal edge from an arena   -> 2 lines  (root + the dependent, NAMED)
#   a dev-dependency from the cli -> 1 line   (--edges normal excludes it)
#
# So the reading is: MORE THAN ONE LINE MEANS A NORMAL REVERSE-DEPENDENCY, and
# the lines past the first name who. Nothing is parsed out of them; they are
# counted and printed, which is also how this gate cites its own log output
# (CLAUDE.md Workflow) rather than a wrapper's exit status.
#
# --workspace, not `-p <one-member>`: the claim is about the WHOLE workspace and
# a single-member probe missed an edge into a sibling that ships two binaries of
# its own, reaching a CONFIRMED verdict on a tree that had one.
set -euo pipefail

fail() { echo "solver_edge_check: $*" >&2; exit 2; }

[ "$#" -eq 2 ] || fail "usage: solver_edge_check.sh <workspace-root> <crate-name>"
ROOT="$1"
CRATE="$2"

# A crate name reaches a printed record; guard it as an ALLOW-LIST so pinning the
# locale makes the refusal as WIDE as possible (SHELL_CHECKLIST items 4 and 9).
case "$CRATE" in
	'') fail "the crate name is empty" ;;
	*[![:print:]]*) fail "the crate name holds a non-printable character" ;;
	-*) fail "the crate name '$CRATE' starts with a dash and would be read as an option" ;;
esac
[ -d "$ROOT" ] || fail "no such workspace root: $ROOT"
[ -f "$ROOT/Cargo.toml" ] || fail "no Cargo.toml at the workspace root: $ROOT"
command -v cargo >/dev/null || fail "cargo is not on PATH"

# READABILITY FIRST, so "cargo could not answer" and "there is no edge" are two
# reasons with two refusals rather than one status meaning either (item 8). This
# probe does NOT name the crate, so it cannot be confused by a bad crate name.
( cd "$ROOT" && cargo tree --locked --workspace --edges normal ) >/dev/null 2>&1 \
	|| fail "cargo cannot resolve the workspace's normal-edge graph at $ROOT"

# The crate must BE in the workspace. Without this, a typo in the crate name is
# indistinguishable from "no edge" — the failure the substring count made when it
# answered 0 and the document read it as good news.
( cd "$ROOT" && cargo tree --locked --workspace --edges normal -i "$CRATE" ) >/dev/null 2>&1 \
	|| fail "'$CRATE' is not a package in the workspace at $ROOT, or its specification is \
ambiguous; either way no answer about its reverse-dependencies was taken"

# The answer. Captured, counted, and PRINTED — never an exit status.
TREE="$( cd "$ROOT" && cargo tree --locked --workspace --edges normal -i "$CRATE" 2>/dev/null )" \
	|| fail "the inverted tree could not be taken for '$CRATE'"
LINES="$(printf '%s\n' "$TREE" | grep -c . || true)"
case "$LINES" in
	'' | *[!0-9]*) fail "the inverted tree's line count is not a number: $LINES" ;;
esac
[ "$LINES" -ge 1 ] || fail "the inverted tree for '$CRATE' is empty, which the readability \
probe should have made impossible"

# CARGO PRINTS ABSOLUTE PATHS, AND THE CALLER'S WORKSPACE IS OFTEN A `mktemp`
# CLONE — so printing the tree verbatim puts a per-run directory name into a
# record that a pre-registration requires to be byte-identical across
# replications. That is the same defect class as a `diff -u` header leaking its
# `mktemp` paths, and it would void every replicated run rather than only a
# failing one. Caught by this script's own test suite on its first execution.
# Bash substring replacement, not `sed`: the root is a path, and a path is not a
# regular expression.
ROOT_ABS="$(cd "$ROOT" && pwd)" || fail "cannot canonicalise the workspace root $ROOT"
TREE_PRINT="${TREE//"$ROOT_ABS"/<workspace>}"

printf 'solver_edge_check: inverted normal-edge tree for %s (%s lines)\n' "$CRATE" "$LINES"
printf '%s\n' "$TREE_PRINT" | sed 's/^/solver_edge_check:   /'

if [ "$LINES" -eq 1 ]; then
	echo "solver_edge_check: NO normal reverse-dependency on $CRATE anywhere in the workspace"
	exit 0
fi
echo "solver_edge_check: $CRATE HAS normal reverse-dependencies — the lines above name them" >&2
exit 1
