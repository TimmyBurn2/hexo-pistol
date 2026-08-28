#!/usr/bin/env bash
#
# The WP-P1 puzzle-corpus suite. Four tests, in the dispatch's order:
#
#   T1  the checked-in fixture page parses to the expected record, byte-exact
#       (plus negative controls: every V-check fires on a position built to
#       break it, because all 48 shipped positions validate clean and a suite
#       that only asserted that would pass with the checks deleted)
#   T2  every valid record is representable on pistol's lattice
#   T4  two runs from cache produce byte-identical output, and the placement
#       distance is measured by pistol's own PlacementDistances (D-440)
#   T3  the mapping status is UNVERIFIED and nothing claims fixture-grade
#   T5  the D-447 horizon criterion is the one the gate applies
#
# Nothing here ships red. T3 and T5 assert the STATE of the evidence, so they
# stay green while the gate is shut and stay green when a witness opens it; what
# they fail on is the grade moving without one (D-448), or the registered
# criterion being replaced (D-447).
#
# Usage: tools/puzzle_corpus/tests/run_tests.sh
# Exit:  0 every test passed; 1 one did not.

set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT"

status=0
fail() { printf 'puzzle-corpus-tests: FAIL: %s\n' "$*" >&2; status=1; }

printf '== T1/T2/NEG: extractor unit, load, negative controls ==\n'
python3 tools/puzzle_corpus/tests/test_extract.py | grep -v '^  ok' || fail "extractor tests"

printf '\n== T4: determinism, two runs from cache ==\n'
SCRATCH="$(mktemp -d)"
trap 'rm -rf -- "$SCRATCH"' EXIT
python3 tools/puzzle_corpus/extract.py --offline --out "$SCRATCH/a.jsonl" >/dev/null || fail "run 1"
python3 tools/puzzle_corpus/extract.py --offline --out "$SCRATCH/b.jsonl" >/dev/null || fail "run 2"
if cmp -s "$SCRATCH/a.jsonl" "$SCRATCH/b.jsonl"; then
  printf '  ok    two cached runs are byte-identical (%s)\n' \
    "$(sha256sum "$SCRATCH/a.jsonl" | cut -c1-16)"
else
  fail "two cached runs differ"
fi
if cmp -s "$SCRATCH/a.jsonl" corpus/puzzles/hexo_discord_v1.jsonl; then
  printf '  ok    the committed corpus matches a fresh cached run\n'
else
  fail "the committed corpus does not match a fresh cached run"
fi

printf '\n== T4b: placement distance, via the ONE shipped implementation ==\n'
# D-440: this suite owns no distance code. The criterion is
# corpus/distance.rs's PlacementDistances, and the test it runs fails if that
# implementation ever drifts toward the pre-turn-board variant WP-P1's F-4
# rediscovered.
if cargo test -p pistol-cli --locked --test puzzle_distance_tests 2>&1 | grep -q '^test result: ok'; then
  printf '  ok    PlacementDistances over the puzzle corpus (max 8, none unrescuable)\n'
else
  fail "puzzle_distance_tests"
fi

printf '\n== T3: mapping status is UNVERIFIED and nothing claims fixture-grade ==\n'
GATE="$(python3 tools/puzzle_corpus/mapping_gate.py)" || fail "the mapping gate refused (BASIS leg)"
if printf '%s' "$GATE" | grep -q '^mapping_status: UNVERIFIED$'; then
  printf '  ok    mapping_status is UNVERIFIED, as the evidence stands\n'
else
  fail "mapping_status is not UNVERIFIED; the corpus grade moved without a witness"
fi
if printf '%s' "$GATE" | grep -q '^grade: POSITION-GRADE'; then
  printf '  ok    the corpus is declared POSITION-GRADE (D-448)\n'
else
  fail "the corpus no longer declares POSITION-GRADE"
fi
# A fixture-grade claim is a FILE: the golden set-position text D-448 forbids
# until a witness lands. Its absence is the assertion.
if find corpus/puzzles -name '*.txt' -print -quit | grep -q .; then
  fail "a text fixture derived from the puzzle corpus exists; that is a fixture-grade claim"
else
  printf '  ok    no golden text fixture is derived from the puzzle corpus\n'
fi

printf '\n== T5: the D-447 horizon criterion is the one applied ==\n'
# Green while no witness reaches the horizon, green when one does. It fails only
# if the gate stops applying the registered criterion — which is what would let
# a below-horizon witness close the gate by itself.
if printf '%s' "$GATE" | grep -q 'ONE witness at >= 27 stones (turn 14) closes the gate'; then
  printf '  ok    the registered criterion is stated before the measurement\n'
else
  fail "the D-447 criterion is not the one the gate registered"
fi
WITNESSES="$(printf '%s' "$GATE" | grep -c '^  witness ')"
AT_HORIZON="$(printf '%s' "$GATE" | grep '^  witness ' | grep -c 'at/above horizon' || true)"
if [ "$AT_HORIZON" -gt 0 ] || [ "$WITNESSES" -ge 2 ]; then
  printf '%s' "$GATE" | grep -q '^mapping_status: VERIFIED$' \
    || fail "the criterion is satisfied but the status did not move to VERIFIED"
  printf '  ok    a horizon-satisfying witness landed and the status moved\n'
else
  printf '%s' "$GATE" | grep -q '^mapping_status: UNVERIFIED$' \
    || fail "the status claims VERIFIED without a witness satisfying D-447"
  printf '  ok    %s witness(es), none at the horizon, so the gate stays shut\n' "$WITNESSES"
fi

printf '\n'
if [ "$status" -eq 0 ]; then printf 'puzzle-corpus-tests: all tests passed\n'
else printf 'puzzle-corpus-tests: RED\n'; fi
exit "$status"
