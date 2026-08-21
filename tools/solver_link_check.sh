#!/usr/bin/env bash
#
# Is any source file of one crate an INPUT to any binary this workspace ships?
#
# Usage: tools/solver_link_check.sh <workspace-root> <crate-path>
# Exit:  0 no source under <crate-path> reaches any shipped binary
#        1 it does — the offending files are named on stdout
#        2 the question could not be answered (never a verdict)
#
# WHY THIS IS NOT A DEPENDENCY-GRAPH CHECK. `tools/solver_edge_check.sh` asks
# whether a crate is LINKED, on cargo's resolved normal-edge graph. That is a
# real question and it is not this one: a crate with NO manifest edge at all
# still reaches the binary through `include!`, `include_str!` or
# `include_bytes!`, and a decision red-team reproduced exactly that — the graph
# check answered "no normal reverse-dependency, exit 0" on a tree where mutating
# the crate moved the shipped binary's digest. A graph claim is not a codegen
# claim.
#
# WHY IT IS NOT A TWO-BUILD DIGEST COMPARISON EITHER. That was the shape of the
# hypothesis this gate replaces, and it has a measured FALSE CONFIRMED inside its
# own stated coverage: with both builds sharing one target directory — which is
# how it was written — a build script that reads the crate without declaring
# `rerun-if-changed` leaves a stale artefact in `OUT_DIR`, the second build
# compiles nothing, and two identical digests are reported for a binary whose
# behaviour moved. A comparison is only as good as the independence of the things
# compared, and that independence was never stated.
#
# WHAT IS ASKED INSTEAD: rustc's own bookkeeping. Every binary cargo builds gets
# a dep-info file beside it listing every source that went into it, so "did this
# crate reach that binary" is answered by reading that list. It costs no extra
# build — the artefacts are the ones CI already produces — it covers EVERY
# shipped binary rather than one, and its refusal NAMES THE FILE instead of
# printing two hexes, which is CLAUDE.md's rule that a gate claim cites the
# gate's own log output.
#
# THE ONE THING IT CANNOT SEE, stated here because a gate that hides its blind
# spot is worse than no gate: what a BUILD SCRIPT read. `OUT_DIR` code is an
# input and the build script's own sources are recorded, but its reads are not.
# So this script REFUSES TO ANSWER when the workspace has any build script,
# rather than answering under an assumption it cannot check.
#
# RULE9-JUSTIFICATION: one question, and the reasons it is asked this way rather
# than the two obvious ways are the larger half of the file — both of those ways
# were tried, both are in this repository's history, and both are wrong for
# reasons a reader would otherwise rediscover.
set -euo pipefail

fail() { echo "solver_link_check: $*" >&2; exit 2; }

[ "$#" -eq 2 ] || fail "usage: solver_link_check.sh <workspace-root> <crate-path>"
ROOT="$1"
CRATE_PATH="$2"

# SHELL_CHECKLIST item 11's shape check, and item 9's: both reach a printed record.
case "$CRATE_PATH" in
	'') fail "the crate path is empty" ;;
	/* | *..*) fail "the crate path must be repository-relative, not $CRATE_PATH" ;;
	*[![:print:]]*) fail "the crate path holds a non-printable character" ;;
esac
case "$ROOT" in *[![:print:]]*) fail "the workspace root holds a non-printable character" ;; esac
[ -d "$ROOT" ] || fail "no such workspace root: $ROOT"
[ -f "$ROOT/Cargo.toml" ] || fail "no Cargo.toml at the workspace root: $ROOT"
command -v cargo >/dev/null || fail "cargo is not on PATH"
command -v realpath >/dev/null || fail "realpath is not on PATH"
command -v jq >/dev/null || fail "jq is not on PATH; the workspace's target metadata cannot be read"

ROOT_ABS="$(cd "$ROOT" && pwd -P)" || fail "cannot canonicalise the workspace root"
# `-ms`, NOT `-m`. Resolving symlinks moves a source OUT of the crate it lives
# in: a review made `crates/subject/src/lib.rs` a symlink to `crates/shared/real.rs`,
# dep-info recorded the path under the crate, `realpath -m` resolved it away and
# the gate answered EXIT 0 on a tree whose binary observed the subject. `-ms`
# normalises `..` lexically — which is what the canonicalisation was added for —
# without following links.
CRATE_ABS="$(realpath -ms -- "$ROOT_ABS/$CRATE_PATH")" || fail "cannot canonicalise $CRATE_PATH"
[ -d "$CRATE_ABS" ] || fail "no such crate directory: $CRATE_PATH"

# THE PREMISE, CHECKED RATHER THAN ASSUMED. Refusing (2) rather than passing (0)
# is the difference between "no" and "I cannot tell".
# ENUMERATED FROM CARGO'S OWN TARGET METADATA, NOT FROM A FILENAME. `find -name
# build.rs` does not see a script named by the manifest's `build =` key, and a
# review drove exactly that past this check: a `build = "custom_build.rs"` that
# baked the subject into the binary reached EXIT 0 through the guard written to
# refuse it. Cargo reports a `custom-build` target for both spellings.
META="$(cd "$ROOT_ABS" && cargo metadata --format-version 1 --no-deps 2>/dev/null)" \
	|| fail "cannot read the workspace metadata at $ROOT_ABS"
BUILD_SCRIPTS="$(printf '%s' "$META" \
	| jq -r '.packages[] | .name as $p | .targets[] | select(.kind[] == "custom-build") | "\($p):\(.src_path)"')" \
	|| fail "cannot enumerate the workspace's build scripts"
# SCOPE, STATED: `--no-deps` means WORKSPACE MEMBERS. Registry dependencies of
# this workspace do run build scripts — measured, four of them — and dep-info
# records nothing about what any of them read either. Nothing in this check
# speaks for those; what it speaks for is that no MEMBER of this workspace has
# one, which is the only half a workspace author controls.
[ -z "$BUILD_SCRIPTS" ] || fail "a workspace member declares a build script and dep-info does \
not record what a build script READ, so no answer about $CRATE_PATH was taken: $BUILD_SCRIPTS"

# SCRATCH SPACE, BEFORE THE BUILD AND IN THIS GATE'S VOCABULARY. A full
# filesystem makes `cargo` answer `Disk quota exceeded (os error 122)`, this
# gate's build refusal fires, and what reaches the log is `cannot build the
# workspace's binaries` — correct, exit 2, and READ AS A SOLVER-LINK REGRESSION
# by the standing test that drives it. That happened: `/tmp` here is RAM-backed
# at 24 GiB and one session filled it (docs/decisions.md D-281, D-285;
# tools/SHELL_CHECKLIST.md item 12). The preflight is a sibling script so the
# number and its measurement live in one place.
PREFLIGHT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/scratch_preflight.sh"
[ -f "$PREFLIGHT" ] || fail "the scratch preflight is missing beside this gate: $PREFLIGHT"
# TWO KINDS, NOT ONE (item 8). The preflight distinguishes its own exit 1 —
# "the caller called this wrong" — from exit 2, "the run is void", and folding
# them into one message asserts a space shortage where there may be none: a
# malformed floor binding printed `the RUN VOID above names the filesystem`
# above a line that named no filesystem and no shortage. That binding is retired
# (docs/decisions.md D-306) and the distinction is not: the preflight still exits
# 1 on its own arguments, which is the branch the `*)` arm below catches.
PF_RC=0
bash "$PREFLIGHT" "$ROOT_ABS" || PF_RC=$?
case "$PF_RC" in
0) ;;
2) fail "not enough scratch space under $ROOT_ABS to build this workspace; the RUN VOID above names the filesystem, and NOTHING about $CRATE_PATH was measured" ;;
*) fail "the scratch preflight refused its own arguments (exit $PF_RC); nothing about $CRATE_PATH was measured" ;;
esac

# THE BINARY SET IS WHAT CARGO JUST BUILT, not a glob over `target/`. An unscoped
# `*.d` glob picks up `lib<crate>.d` — including the subject crate's own library
# — and reports hits on a clean tree.
BUILD_JSON="$(cd "$ROOT_ABS" && cargo build --locked --workspace --bins \
	--message-format=json-render-diagnostics 2>/dev/null)" \
	|| fail "cannot build the workspace's binaries at $ROOT_ABS"
EXECUTABLES="$(printf '%s\n' "$BUILD_JSON" | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | LC_ALL=C sort -u)"
[ -n "$EXECUTABLES" ] || fail "cargo reported no executables for this workspace"

BIN_COUNT=0
INPUT_COUNT=0
HITS=""
while IFS= read -r exe; do
	[ -n "$exe" ] || continue
	BIN_COUNT=$((BIN_COUNT + 1))
	dep="${exe}.d"
	[ -s "$dep" ] || fail "no dep-info beside $exe — the input list cannot be read"
	# The first line is `<target>: <source> <source> …`. An escaped space would
	# make whitespace splitting wrong, so refuse rather than mis-parse (item 8).
	line="$(sed -n '1p' -- "$dep")" || fail "cannot read $dep"
	case "$line" in *'\ '*) fail "$dep records a path containing a space; this parser would split it" ;; esac
	deps="${line#*: }"
	this_bin=0
	for src in $deps; do
		INPUT_COUNT=$((INPUT_COUNT + 1))
		this_bin=$((this_bin + 1))
		# CANONICALISE BEFORE MATCHING. rustc records paths AS WRITTEN: a source
		# reached from `src/bin/` is recorded as
		# `crates/pistol-cli/src/bin/../../../pistol-solver/src/lib.rs`, and a
		# plain substring match on the crate path returns ZERO hits on the very
		# file it exists to catch — EXIT-0-WRONG-ANSWER, found by building this.
		# ANCHORED TO THE ROOT THIS GATE WAS GIVEN, never to the caller's
		# working directory. `realpath -ms` resolves a RELATIVE entry against
		# the process's cwd, and this loop runs wherever the caller stood — the
		# two `cargo` calls above `cd` only inside their own subshells. Cargo
		# emits relative entries whenever `build.dep-info-basedir` is set, a
		# supported config key, and the same tree then answers three different
		# ways as a function of the caller's `cd`: exit 0 with the wrong answer
		# before the existence check below existed, exit 2 with it, and exit 1
		# — the correct answer — only from the root.
		case "$src" in
		/*) abs="$(realpath -ms -- "$src")" ;;
		*) abs="$(realpath -ms -- "$ROOT_ABS/$src")" ;;
		esac || fail "cannot canonicalise a dep-info entry of $dep"
		# EVERY ENTRY NAMES A FILE THAT IS THERE. rustc records the sources it
		# READ, so an entry that is not a path is not a missing input — it is
		# THIS PARSER having split the line in the wrong place, and the answer
		# it would then give is about a file set nobody compiled. Without this
		# the first-separator split and a last-separator split are
		# indistinguishable on every well-formed line, which is exactly why a
		# mutation from `${line#*: }` to `${line##*: }` — keeping only the last
		# input per binary — survived eleven tests (docs/decisions.md D-281's
		# S29).
		[ -e "$abs" ] || fail "$dep lists \`$src\`, which names nothing on disk; a dep-info entry that is not a source path means this parse is wrong, and a wrong parse answers about a file set nobody built"
		case "$abs" in
		"$CRATE_ABS"/*) HITS="$HITS$abs <- $exe"$'\n' ;;
		esac
	done
	# PER BINARY, not a global floor. A global threshold is a magic number that
	# a fixture cannot meet and that MASKS the real failure: one binary's list
	# failing to parse while the others carry the total past the bar. Every
	# binary rustc built has at least its own entry point among its inputs.
	[ "$this_bin" -ge 1 ] || fail "$dep listed no source inputs for $exe; the parse is wrong"
done < <(printf '%s\n' "$EXECUTABLES")

# A count nobody checks is a count that can silently become zero.
[ "$BIN_COUNT" -ge 1 ] || fail "no shipped binaries were examined; the enumeration is wrong"
# THE BREADTH CLAIM, CHECKED. `cargo build --workspace --bins` SILENTLY SKIPS bin
# targets whose `required-features` are unmet and packages under `[workspace]
# exclude`, so the gate would print a narrower count and answer 0 with no hint it
# had narrowed. Compare what cargo DECLARES against what it BUILT.
DECLARED="$(printf '%s' "$META" | jq -r '[.packages[].targets[] | select(.kind[] == "bin")] | length')" \
	|| fail "cannot count the workspace's declared binary targets"
case "$DECLARED" in '' | *[!0-9]*) fail "the declared binary count is not a number: $DECLARED" ;; esac
[ "$DECLARED" -eq "$BIN_COUNT" ] || fail "this workspace declares $DECLARED binary targets and \
only $BIN_COUNT were built and examined — a bin behind an unmet required-feature or an excluded \
package would narrow this answer silently"

echo "solver_link_check: $BIN_COUNT shipped binaries, $INPUT_COUNT source inputs, subject $CRATE_PATH"
if [ -z "$HITS" ]; then
	echo "solver_link_check: NO source under $CRATE_PATH is an input to any shipped binary"
	exit 0
fi
printf '%s' "$HITS" | sed 's|^|solver_link_check:   |'
echo "solver_link_check: $CRATE_PATH REACHES a shipped binary — the lines above name the files" >&2
exit 1
