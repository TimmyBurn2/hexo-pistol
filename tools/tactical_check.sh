#!/usr/bin/env bash
#
# The tactical gate: the sha-pinned fixture, every case, at the threshold the
# fixture pre-registered before the suite was first run (CLAUDE.md rule 7 and
# §Process).
#
# In release, and through the engine binary rather than through `cargo test`,
# because the deep cases need it — the DEBUG cost, not the release cost. Measured
# on the development machine: all twenty cases, each searched twice, cost 4.8 s
# here and 63 s as the ignored debug test. No case in the fixture runs at the
# shipping radius 3: the depth-3 cases run at configs/gate_v0.toml's radius 1 and
# the rest at the instrument radius 2, so the 84-100 s in that file's measurement
# table is why gate_v0 exists and is not what this script costs. `cargo test` runs
# the cheap half of the suite and the fixture's pin; this runs all of it. The same
# split `tools/perft_check.sh` uses for the movegen oracle (docs/decisions.md
# D-54).
#
# `selftest` also searches every case twice — once from a fresh engine and once
# from an engine that has played and been told `newgame` — and any disagreement
# fails the gate whatever the tactical threshold says: the determinism law is not a
# percentage (CLAUDE.md rule 4).
#
# Each case names the config it is a claim about, so this script names none: a
# threshold means nothing without the search that has to meet it, and putting the
# config on the command line would let the two drift apart.
#
# Usage: tools/tactical_check.sh
# Exit:  0 the suite meets its threshold, 1 it does not.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

FIXTURE="crates/pistol-cli/tests/fixtures/tactical_v0.txt"

fail() { printf 'tactical_check: FAIL: %s\n' "$*" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo is not on PATH"
[ -f "$FIXTURE" ] || fail "no fixture at $FIXTURE"

echo "tactical_check: building the engine (release, locked)"
# THE BINARY THIS GATE RUNS IS THE BINARY CARGO BUILT, and the path comes from
# cargo's own artifact stream rather than from a literal here. Cargo's target
# directory is redirectable three ways — `CARGO_TARGET_DIR`, `[build] target-dir`,
# and `[build] target`, which moves the artifact into a per-triple subdirectory
# that an explicit `--target-dir` does not remove — and a hardcoded
# `target/release/pistol` then runs whatever STALE binary sits at that path while
# the build goes elsewhere. REPRODUCED both ways: with the directory redirected
# and a stale binary left behind, this gate printed `selftest: 20 of 20 cases
# solved` and exited 0 for a freshly built engine that fails the suite; with
# nothing at that path, bash's 127 flowed into the refusal below and blamed the
# ENGINE for a tactical regression that never happened. The first is
# EXIT-0-WRONG-ANSWER in the sense tools/SHELL_CHECKLIST.md opens with, and the
# second is the wrong diagnosis rule 3 forbids (docs/decisions.md D-250).
BUILD_LOG="$(cargo build --release --locked --quiet --bin pistol \
	--message-format=json-render-diagnostics)" || fail "the engine does not build"
# The artifact records that name an executable; a library artifact carries
# `"executable":null` and cannot match. `sed` answers 0 on no match, so an empty
# result is a VALUE to refuse below and never a status to test.
mapfile -t BUILT < <(sed -n 's/.*"executable":"\([^"\\]*\)".*/\1/p' <<<"$BUILD_LOG")
# What the stream NAMED, against what this gate could READ: a path carrying a
# quote or a backslash matches neither class above and must not be mistaken for a
# bin cargo built nothing for. `grep -c` prints 0 and STILL exits 1 on no match
# (tools/SHELL_CHECKLIST.md item 3), so the empty count is a legitimate answer and
# gets `|| true` rather than a death; its SPELLING is then checked, not just its
# value (item 8).
NAMED="$(grep -c '"executable":"' <<<"$BUILD_LOG" || true)"
case "$NAMED" in
*[!0-9]* | "") fail "the artifact-record count is not a number: \`$NAMED\`" ;;
esac
[ "$NAMED" -eq "${#BUILT[@]}" ] ||
	fail "cargo named $NAMED executables and this gate could read ${#BUILT[@]} of them: a quote or a backslash in a path"
# ONE REFUSAL PER REASON (tools/SHELL_CHECKLIST.md item 8): no executable at all,
# several executables, and then a named path that is absent, is not a regular
# file, or carries no `+x` — the last being the case `command -v` admits and exec
# answers with 126.
[ "${#BUILT[@]}" -ne 0 ] || fail "cargo built no executable for --bin pistol"
[ "${#BUILT[@]}" -eq 1 ] ||
	fail "cargo named ${#BUILT[@]} executables for --bin pistol: ${BUILT[*]}"
ENGINE="${BUILT[0]}"
[ -e "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and nothing is there"
[ -f "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and it is not a regular file"
[ -x "$ENGINE" ] || fail "cargo named \`$ENGINE\` for --bin pistol and it is not executable"

"$ENGINE" selftest --fixtures "$FIXTURE" ||
	fail "the tactical suite did not meet its pre-registered threshold"
