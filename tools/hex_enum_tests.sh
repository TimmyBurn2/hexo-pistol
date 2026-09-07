#!/usr/bin/env bash
#
# The Stage-E hex threat enum and its corpus census produce recorded numbers —
# the class count k per length, the code count, observations per code and the
# two variance terms — and tools/SHELL_CHECKLIST.md's coverage rule wants at
# least one test DRIVING THE SHIPPED SCRIPT for anything under tools/ that
# produces one.
#
# THE SUITE IS HERMETIC AND THE CENSUS IS STILL DRIVEN. The labelled corpus
# lives outside the repository (docs/decisions.md D-636), so census.py takes its
# manifest and tranche template from argv and the test writes a synthetic corpus
# of its own, with a control run whose labels are all equal — without that, a
# between-class variance term could be any number at all and still pass.
#
# Usage: tools/hex_enum_tests.sh
# Exit:  0 every check passed
#        1 a check failed
#        2 THE RUN IS VOID — python3 is missing, so nothing was adjudicated
#          (tools/SHELL_CHECKLIST.md item 12). A void is not a failure.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

void() {
	printf 'hex_enum_tests: RUN VOID: %s\n' "$*" >&2
	printf 'hex_enum_tests: no answer was taken; this is NOT a failure\n' >&2
	exit 2
}

# Programs are resolved through the resolver, never `command -v` — item 8's
# table and docs/decisions.md D-683 say why. This script keeps its own class.
REQUIRE_TOOL="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/require_tool.sh"
[ -x "$REQUIRE_TOOL" ] || void "the tool resolver is missing beside this script: $REQUIRE_TOOL"

"$REQUIRE_TOOL" python3 >/dev/null || void "python3 is not usable"
for SUITE in tools/hex_enum/test_hex_enum.py tools/hex_enum/test_census.py; do
	[ -f "$SUITE" ] || void "$SUITE is missing"
done

# Two suites because the enum's properties are checked by ENUMERATION and the
# walk's by driving the shipped scripts over a synthetic corpus; one file
# carrying both was over rule 9's cap for no reason a reader benefits from.
python3 tools/hex_enum/test_hex_enum.py
python3 tools/hex_enum/test_census.py
