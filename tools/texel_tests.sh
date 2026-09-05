#!/usr/bin/env bash
#
# The offline eval-fitting and census-counting tools carry recorded numbers —
# Phase 1's fitted weight table and §B's census counts — and
# tools/SHELL_CHECKLIST.md's coverage rule wants at least one test DRIVING THE
# SHIPPED SCRIPT for anything under tools/ that produces one.
#
# IT LANDS AS A GATE BECAUSE A TEST NOTHING RUNS IS NOT A TEST. The suite
# existed for a whole package before any gate invoked it, and a fresh-context
# review found that it also self-SKIPPED to a pass when a gitignored artifact
# was absent — so in CI, where `artifacts/` need not exist, it would have
# reported success having checked nothing. The suite is hermetic now and this
# is what runs it.
#
# Usage: tools/texel_tests.sh
# Exit:  0 every check passed
#        1 a check failed
#        2 THE RUN IS VOID — python3 is missing, so nothing was adjudicated
#          (tools/SHELL_CHECKLIST.md item 12). A void is not a failure.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

void() {
	printf 'texel_tests: RUN VOID: %s\n' "$*" >&2
	printf 'texel_tests: no answer was taken; this is NOT a failure\n' >&2
	exit 2
}

command -v python3 >/dev/null || void "python3 is not on PATH"
[ -f tools/texel/test_texel.py ] || void "tools/texel/test_texel.py is missing"

python3 tools/texel/test_texel.py
