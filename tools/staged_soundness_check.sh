#!/usr/bin/env bash
#
# The staged soundness gate: the four parts D-316 names, one script
# (docs/decisions.md D-316; `U4_soundness_instrument.md` §8.7, §3.3 of
# docs/wp15b_impl_prompt.md).
#
# THE FOUR PARTS, each specified in exactly one place and none of them
# stubbed — the SEAM that would have blocked the differential gate's own
# fragment is closed (docs/decisions.md D-353, D-358):
#
#   1. THE TACTICAL SUITE UNDER STAGED — the sha-pinned tactical_staged_v0.txt
#      fixture, all twenty cases, through the real release binary
#      (tools/tactical_check.sh's own pattern).
#   2. THE DIFFERENTIAL GATE — S-M (docs/decisions.md D-323): per-node
#      EQUALITY of the staged generator's FILTERED-row emitted set against
#      R1, marked DEPENDS-OPEN-THEORY (D-321) at its own text, here.
#   3. THE COLONY FAMILY — six built distant-cluster cases.
#   4. THE PATTERN FIXTURES UNDER STAGED — the calculus's own named patterns,
#      run through the staged generator.
#
# WHY ONE SCRIPT AND NOT FOUR. The four parts are one soundness claim — "the
# staged generator never drops a cell a proven tactic needs" — read from four
# angles, and CLAUDE.md rule 6 wants that claim adjudicated as one gate rather
# than as four scripts a reader has to know to run together.
#
# Usage: tools/staged_soundness_check.sh
# Exit:  0 all four parts pass, 1 at least one does not, 2 THE RUN IS VOID —
#        no gate was adjudicated (tools/SHELL_CHECKLIST.md item 12). A void is
#        not a failure and must not be read as one.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { printf 'staged_soundness_check: FAIL: %s\n' "$*" >&2; exit 1; }
void() { printf 'staged_soundness_check: RUN VOID: %s\n' "$*" >&2; exit 2; }

for SCRATCH in "${TMPDIR:-/tmp}" "$ROOT"; do
	tools/scratch_preflight.sh "$SCRATCH" ||
		void "no part was adjudicated; the lines above name the filesystem"
done

step() { printf '\n=== staged_soundness_check: %s\n' "$*"; }

# ---- part 1: THE TACTICAL SUITE UNDER STAGED --------------------------------
step "1/4: THE TACTICAL SUITE UNDER STAGED (tactical_staged_v0.txt, release)"
FIXTURE="crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

# THE BINARY THIS GATE RUNS IS THE BINARY CARGO BUILT, from cargo's own
# artifact stream rather than a literal path — the same reasoning and the same
# refusal ladder tools/tactical_check.sh already carries (docs/decisions.md
# D-250): a hardcoded target/release/pistol can be stale under a redirected
# target directory, silently certifying yesterday's engine.
BUILD_LOG="$(cargo build --release --locked --quiet --bin pistol \
	--message-format=json-render-diagnostics)" || fail "the engine does not build"
mapfile -t BUILT < <(sed -n 's/.*"executable":"\([^"\\]*\)".*/\1/p' <<<"$BUILD_LOG")
NAMED="$(grep -c '"executable":"' <<<"$BUILD_LOG" || true)"
case "$NAMED" in
*[!0-9]* | "") fail "the artifact-record count is not a number: \`$NAMED\`" ;;
esac
[ "$NAMED" -eq "${#BUILT[@]}" ] ||
	fail "cargo named $NAMED executables and this gate could read ${#BUILT[@]} of them"
[ "${#BUILT[@]}" -ne 0 ] || fail "cargo built no executable for --bin pistol"
[ "${#BUILT[@]}" -eq 1 ] ||
	fail "cargo named ${#BUILT[@]} executables for --bin pistol: ${BUILT[*]}"
ENGINE="${BUILT[0]}"
[ -e "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and nothing is there"
[ -f "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and it is not a regular file"
[ -x "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and it is not executable"

"$ENGINE" selftest --fixtures "$FIXTURE" ||
	fail "THE TACTICAL SUITE UNDER STAGED did not meet its pre-registered threshold"

# ---- part 2: THE DIFFERENTIAL GATE — S-M ------------------------------------
step "2/4: THE DIFFERENTIAL GATE — S-M, marked DEPENDS-OPEN-THEORY (D-321)"
cargo test --locked --package pistol-search --test staged_differential_gate_tests ||
	fail "THE DIFFERENTIAL GATE (S-M) disagreed with R1 at a FILTERED node"

# ---- part 3: THE COLONY FAMILY ----------------------------------------------
step "3/4: THE COLONY FAMILY (six built distant-cluster cases)"
cargo test --locked --package pistol-search --test staged_colony_family_tests ||
	fail "THE COLONY FAMILY found a distant-cluster case the staged generator got wrong"

# ---- part 4: THE PATTERN FIXTURES UNDER STAGED ------------------------------
step "4/4: THE PATTERN FIXTURES UNDER STAGED (the calculus's own named patterns)"
cargo test --locked --package pistol-search --test staged_pattern_fixture_tests ||
	fail "THE PATTERN FIXTURES UNDER STAGED found a pattern the staged generator dropped"

printf '\nstaged_soundness_check: all four parts passed\n'
