#!/usr/bin/env bash
#
# CLAUDE.md rule 9's soft cap, mechanized: a tracked `.rs` file over the cap has
# to say WHY in a comment carrying the marker, and that why may not state a line
# count (docs/decisions.md D-131, amending D-118).
#
# The three things this gate is, precisely:
#
#   1. A cap. 300 lines, which is what rule 9's "~300-line soft cap" is worth as
#      a number a script can compare against. Over it is not a failure — rule 9
#      calls the cap soft and means it — it is a demand for a sentence.
#   2. A marker convention. `RULE9-JUSTIFICATION:` in a comment line, followed by
#      the why on that same line. A checker has to RECOGNIZE a justification, and
#      prose cannot be recognized, so the marker is the interface: it goes in the
#      module doc where the argument already lives, not in a lint-suppression
#      pragma somewhere the reader will not look.
#   3. A count ban. Rule 9 says counts are derived and never asserted, so a why
#      that says how long the file is fails the gate. What this catches is the
#      literal form — a numeral beside "line" or "lines". A count spelled out in
#      words gets past it, and that is named here rather than implied: the check
#      is a guard against the obvious mistake, not a proof.
#
# The gate self-tests before it runs, over seeded files in a temporary directory:
# a checker nobody has watched fail is not a checker, and this one is asked to
# say "no" about a file that will not exist until somebody writes it.
#
# Usage: tools/file_justification_check.sh
# Exit:  0 every tracked .rs file is under the cap or justified, 1 otherwise.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { printf 'file_justification_check: FAIL: %s\n' "$*" >&2; exit 1; }

command -v git >/dev/null || fail "git is not on PATH"

# The cap, and the marker every file over it must carry.
SOFT_CAP=300
MARKER='RULE9-JUSTIFICATION:'

# One file's verdict, on stdout: `under`, `justified`, `unjustified`, `empty` or
# `counted`. Everything this gate knows is in here, so that the seeded self-test
# below exercises the same function the tracked file set goes through.
verdict() {
	local file="$1" lines line why
	lines="$(wc -l <"$file")"
	if [ "$lines" -le "$SOFT_CAP" ]; then
		echo "under"
		return
	fi
	# A comment line, because rule 9 asks for a justification COMMENT: the marker
	# in a string literal or an identifier is not one.
	line="$(grep -m1 -E "^[[:space:]]*//.*${MARKER}" "$file" || true)"
	if [ -z "$line" ]; then
		echo "unjustified"
		return
	fi
	why="$(sed -E "s/.*${MARKER}[[:space:]]*//" <<<"$line")"
	if [ -z "$why" ]; then
		echo "empty"
		return
	fi
	# "counts are derived, never asserted" (rule 9).
	if grep -qiE '[0-9]+[[:space:]]*-?[[:space:]]*lines?([^a-z]|$)|lines?[[:space:]]+[0-9]+' <<<"$why"; then
		echo "counted"
		return
	fi
	echo "justified"
}

# --- the self-test, on files nobody tracks --------------------------------

SEED="$(mktemp -d)"
trap 'rm -rf "$SEED"' EXIT

seed() {
	local name="$1" head="$2"
	{
		[ -n "$head" ] && printf '%s\n' "$head"
		for _ in $(seq 1 "$((SOFT_CAP + 1))"); do echo "// filler"; done
	} >"$SEED/$name"
}

seed over_unjustified.rs ""
seed over_justified.rs "//! # ${MARKER} the recursion's parts are not independent"
seed over_empty.rs "//! # ${MARKER}"
seed over_counted.rs "//! # ${MARKER} it is 348 lines and every one earns its place"
printf 'fn main() {}\n' >"$SEED/under.rs"

expect() {
	local got
	got="$(verdict "$SEED/$1")"
	[ "$got" = "$2" ] ||
		fail "self-test: seeded $1 should be '$2' and the check said '$got'"
}

expect over_unjustified.rs unjustified
expect over_justified.rs justified
expect over_empty.rs empty
expect over_counted.rs counted
expect under.rs under
echo "file_justification_check: self-test passed on 5 seeded files (cap $SOFT_CAP)"

# --- the tracked file set -------------------------------------------------

OVER=0
BAD=()
while IFS= read -r file; do
	case "$(verdict "$file")" in
	under) ;;
	justified)
		OVER=$((OVER + 1))
		echo "file_justification_check: over the cap and justified: $file"
		;;
	unjustified) BAD+=("$file: over the cap with no $MARKER comment") ;;
	empty) BAD+=("$file: $MARKER carries no why") ;;
	counted) BAD+=("$file: $MARKER states a line count, and counts are derived") ;;
	esac
done < <(git ls-files '*.rs')

if [ "${#BAD[@]}" -gt 0 ]; then
	printf 'file_justification_check: %s\n' "${BAD[@]}" >&2
	fail "${#BAD[@]} tracked file(s) over the soft cap without a why (CLAUDE.md rule 9)"
fi

echo "file_justification_check: $(git ls-files '*.rs' | wc -l) tracked .rs files, $OVER over the cap, all justified"
