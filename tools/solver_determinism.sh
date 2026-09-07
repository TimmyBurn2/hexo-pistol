#!/usr/bin/env bash
#
# The solver's determinism seat (WP-1.8a, design §7): the selftest binary,
# built once, run twice in separate processes over the registered fixture,
# full transcript diffed. The solver consults no clock and no hasher
# iteration order, so two runs must agree on every printed field — value,
# node count, seesaw, digest, zone status (CLAUDE.md rule 4, D-7).
#
# What is NOT compared: nothing is exempt. The transcript has no wall-clock
# field, which is what makes byte equality the bar.
#
# Usage: tools/solver_determinism.sh
# Exit:  0 the two runs agree
#        1 they do not, or a run refused
#        2 the question could not be asked (VOID)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { echo "solver_determinism: FAIL: $*" >&2; exit 1; }
void() { echo "solver_determinism: RUN VOID: $*" >&2; exit 2; }

command -v cargo >/dev/null || void "cargo is not on PATH"

FIXTURE="crates/pistol-solver/tests/fixtures/solver_v0.txt"
CONFIG="configs/solver_v0.toml"
[ -f "$FIXTURE" ] || void "no fixture at $FIXTURE"
[ -f "$CONFIG" ] || void "no config at $CONFIG"

# THE BINARY IS THE ONE CARGO JUST NAMED, never a path this script spells
# (docs/decisions.md D-250, D-672). A literal `target/release/solver-selftest`
# has two failure modes and this gate was reproduced in BOTH: under a
# redirected `CARGO_TARGET_DIR` in a fresh worktree the path is absent and the
# gate voids on a subject that built fine, and in a tree that already holds an
# older build the path RESOLVES — to yesterday's binary, certifying a solver
# nobody just compiled. D-250 closed with "a fifth gate that ever runs a binary
# it did not build takes this block instead of a path of its own"; this is that
# gate taking it.
#
# EVERY REFUSAL BELOW IS A VOID AND NOT A FAIL (tools/SHELL_CHECKLIST.md item
# 12). The four sibling gates spell this ladder with `fail` because they define
# no void class; this gate does, and "I could not find the binary to run" is
# the answer not being taken, never the solver having lost determinism.
BUILD_LOG="$(cargo build --release --locked -p pistol-solver --bin solver-selftest \
	--message-format=json-render-diagnostics)" ||
	void "the build failed — cargo's own words are above"
# A library artifact carries `"executable":null` and cannot match. `sed` answers
# 0 on no match, so an empty result is a VALUE refused below and never a status
# tested (tools/SHELL_CHECKLIST.md item 1).
mapfile -t BUILT < <(sed -n 's/.*"executable":"\([^"\\]*\)".*/\1/p' <<<"$BUILD_LOG")
# What the stream NAMED against what this gate could READ: a path carrying a
# quote or a backslash matches neither class and must not pass as a bin cargo
# built nothing for. `grep -c` prints 0 and STILL exits 1 on no match (item 3),
# so the empty count is legitimate and its SPELLING is checked, not just its
# value (item 8).
NAMED="$(grep -c '"executable":"' <<<"$BUILD_LOG" || true)"
case "$NAMED" in
*[!0-9]* | "") void "the artifact-record count is not a number: \`$NAMED\`" ;;
esac
[ "$NAMED" -eq "${#BUILT[@]}" ] ||
	void "cargo named $NAMED executables and this gate could read ${#BUILT[@]} of them: a quote or a backslash in a path"
# ONE REFUSAL PER REASON (item 8): none at all, several, and then a named path
# that is absent, is not a regular file, or carries no `+x` — the last being
# the case `command -v` admits and exec answers with 126.
[ "${#BUILT[@]}" -ne 0 ] || void "cargo built no executable for --bin solver-selftest"
[ "${#BUILT[@]}" -eq 1 ] ||
	void "cargo named ${#BUILT[@]} executables for --bin solver-selftest: ${BUILT[*]}"
BIN="${BUILT[0]}"
[ -e "$BIN" ] || void "cargo named \`$BIN\` for --bin solver-selftest and nothing is there"
[ -f "$BIN" ] || void "cargo named \`$BIN\` for --bin solver-selftest and it is not a regular file"
[ -x "$BIN" ] || void "cargo named \`$BIN\` for --bin solver-selftest and it is not executable"

# SCRATCH SPACE, ASKED FOR BEFORE THE WORK (tools/SHELL_CHECKLIST.md item 12
# obligation 2, docs/decisions.md D-285). BOTH filesystems, because they are
# two: the scratch files go under `$TMPDIR` and the build goes to this
# repository's target tree, and on this machine those are a RAM-backed tmpfs and
# an nvme partition. A shortage on either otherwise reaches the log in `mktemp`'s
# or `cargo`'s vocabulary, which describes those tools rather than this gate.
PREFLIGHT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)/scratch_preflight.sh"
[ -x "$PREFLIGHT" ] || void "the scratch preflight is missing beside this script: $PREFLIGHT"
for SCRATCH_FS in "${TMPDIR:-/tmp}" "$ROOT"; do
	"$PREFLIGHT" "$SCRATCH_FS" ||
		void "no scratch room; the lines above name the filesystem"
done

OUT="$(mktemp -d)" || void "mktemp refused"
trap 'rm -rf "$OUT"' EXIT

# Two SEPARATE processes, the gate proper (D-7's own form).
"$BIN" "$FIXTURE" "$CONFIG" >"$OUT/run-a" 2>"$OUT/err-a" || fail "run A refused: $(cat "$OUT/err-a")"
"$BIN" "$FIXTURE" "$CONFIG" >"$OUT/run-b" 2>"$OUT/err-b" || fail "run B refused: $(cat "$OUT/err-b")"

# Positive content: a transcript of refusals would agree and prove nothing.
# One line per case plus the summary is what a working run prints.
CASES="$(grep -c '^case ' "$OUT/run-a" || true)"
SUMMARY="$(grep -c '^summary ' "$OUT/run-a" || true)"
case "$CASES:$SUMMARY" in
0:*|*:0) fail "run A printed $CASES case lines and $SUMMARY summaries — nothing was solved" ;;
esac
if [ "$(grep -c '^case ' "$OUT/run-b" || true)" -ne "$CASES" ]; then
	fail "run B printed a different number of cases than run A"
fi

if ! diff -u "$OUT/run-a" "$OUT/run-b" >"$OUT/diff"; then
	cat "$OUT/diff" >&2
	fail "the two runs disagree — the diff is above"
fi

echo "solver_determinism: PASS — $CASES cases, byte-identical transcripts"
