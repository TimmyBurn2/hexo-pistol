#!/usr/bin/env bash
#
# Run one match through the local HeXO match platform: build the matchserver,
# run it on a config, then digest everything it wrote for the ADR.
#
# tools/SHELL_CHECKLIST.md obligations this script answers by name: item 1
# (every substitution lands in a variable before it reaches a record), item 3
# (no bare grep pipelines), item 8 (spelling checked, one refusal per reason),
# item 9 (the config path is caller-controlled and reaches messages, so it is
# guarded for printability), item 11 (the output_dir the config names is
# resolved only repo-relative and never upward; the script itself deletes
# nothing), item 12 (exit codes below distinguish run from refusal).
#
# Usage: tools/sealbot/run_match.sh <config.toml>
# Exit:  0 the match ran and its report was written — the RESULT lives in the
#          report, not here: forfeits, caps and losses are findings a match
#          exists to record, never script failures;
#        2 refused: bad usage, unreadable config, failed build, or an output
#          directory that already holds a report (an anchor never overwrites).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

fail() { printf 'run_match: REFUSED: %s\n' "$*" >&2; exit 2; }

[ "$#" -eq 1 ] || fail "usage: tools/sealbot/run_match.sh <config.toml>"

# Item 9: the config path is caller-controlled text that reaches messages and
# records; refuse anything unprintable rather than interpolate it.
case "$1" in
  *[![:print:]]*) fail "config path is not printable text" ;;
esac
# Item 11, capture direction: a caller's relative path resolves against where
# the caller stood, captured BEFORE the cd below.
CALLER_DIR="$PWD"
case "$1" in
  /*) CONFIG="$1" ;;
  *) CONFIG="$CALLER_DIR/$1" ;;
esac
[ -f "$CONFIG" ] || fail "no config file at $CONFIG"

cd "$ROOT"

command -v cargo >/dev/null 2>&1 || fail "cargo is not on PATH"
MS_DIR=tools/sealbot/matchserver
[ -f "$MS_DIR/Cargo.lock" ] || fail "no Cargo.lock under $MS_DIR; generate and commit one"

# The build's own failure vocabulary is the compiler's; wrapping it would only
# mangle it. The subshell keeps the cd local to the build.
( cd "$MS_DIR" && cargo build --release --locked ) || fail "matchserver build failed"
BIN="$MS_DIR/target/release/pistol-matchserver"
[ -x "$BIN" ] || fail "build produced no executable at $BIN"

# The binary refuses (exit 2) on its own named grounds — a config it cannot
# parse, an output directory that already holds a report — and its refusal
# text reaches stderr unchanged.
"$BIN" "$CONFIG"

# The output directory, for the digest: read from the config with tomllib,
# refusing a bad parse by name rather than swallowing it.
OUT_DIR="$(python3 - "$CONFIG" <<'PY'
import sys, tomllib
try:
    with open(sys.argv[1], "rb") as handle:
        config = tomllib.load(handle)
    print(config["output_dir"])
except Exception as error:
    sys.exit(f"reading output_dir from {sys.argv[1]}: {error}")
PY
)"
[ -n "$OUT_DIR" ] || fail "config has an empty output_dir"
# Item 11, write direction: this path feeds a listing and the record below;
# absolute or upward-traversing values are refused so the record can only name
# a place under the repository root.
case "$OUT_DIR" in
  /*) fail "output_dir must be repository-relative, got $OUT_DIR" ;;
  *..*) fail "output_dir must not traverse upward, got $OUT_DIR" ;;
esac
REPORT="$ROOT/$OUT_DIR/report.json"
[ -f "$REPORT" ] || fail "no report.json under $OUT_DIR: the match did not write its report"

printf 'run_match: report %s\n' "$OUT_DIR"
# Digests for the ADR: every artifact the run wrote, named by content. A
# missing file is a real failure (the run claimed to write it), not a skip.
( cd "$ROOT/$OUT_DIR" && sha256sum report.json report.txt ./*.jsonl ) \
  || fail "digesting the artifacts under $OUT_DIR failed"
