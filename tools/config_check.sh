#!/usr/bin/env bash
#
# Every document under configs/ parses and validates, and every engine config
# names a weights file that is actually there. A committed document that does not
# load is a broken contract, not a small mistake.
#
# Three document kinds live under configs/, with different schemas and separate
# versions: an engine config (pistol-engine's `Config`), an eval weight table
# (pistol-eval's `Weights`), and an arena config (pistol-arena's `ArenaConfig`).
# They are told apart by file name — `*_weights.toml` is a weight table,
# `arena_*.toml` is an arena config, anything else is an engine config — and
# each goes to its own validator, because a document checked against another
# schema would be rejected for every key it does not have (docs/decisions.md
# D-64, D-168).
#
# The arena's schema version is its own and is never compared with the engine's:
# they version different documents that change for different reasons.
#
# `--check-weights-file` is the weights-file existence check docs/decisions.md
# D-21 defers to a gate and D-66 puts in the validator example: config validation
# itself stays pure and offline, so this is where a config that points at a file
# nobody committed gets caught.
#
# Usage: tools/config_check.sh [path ...]   (default: every .toml under configs/)
# Exit:  0 all valid, 1 one or more rejected or nothing to check.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

shopt -s nullglob globstar

if [ "$#" -gt 0 ]; then
	explicit=1
	files=("$@")
else
	explicit=0
	files=(configs/**/*.toml)
fi

if [ "${#files[@]}" -eq 0 ]; then
	echo "config_check: FAIL: no config files under configs/" >&2
	echo "config_check: the committed instrument config is part of the contract" >&2
	exit 1
fi

configs=()
weights=()
arenas=()
for file in "${files[@]}"; do
	case "$(basename "$file")" in
	*_weights.toml) weights+=("$file") ;;
	arena_*.toml) arenas+=("$file") ;;
	*) configs+=("$file") ;;
	esac
done

echo "config_check: ${#configs[@]} engine config(s), ${#weights[@]} weight table(s), ${#arenas[@]} arena config(s)"

# In default mode both kinds are part of the committed contract, so an empty list
# is a missing file rather than nothing to do. With explicit paths, only what was
# named is checked.
if [ "$explicit" -eq 0 ]; then
	if [ "${#configs[@]}" -eq 0 ]; then
		echo "config_check: FAIL: no engine config under configs/" >&2
		exit 1
	fi
	if [ "${#weights[@]}" -eq 0 ]; then
		echo "config_check: FAIL: no eval weight table under configs/" >&2
		echo "config_check: eval.weights_file names a committed document" >&2
		exit 1
	fi
	if [ "${#arenas[@]}" -eq 0 ]; then
		echo "config_check: FAIL: no arena config under configs/" >&2
		echo "config_check: tools/arena_smoke.sh runs a committed one (CI gate 11)" >&2
		exit 1
	fi
fi

status=0

if [ "${#configs[@]}" -gt 0 ]; then
	cargo run --quiet --locked --package pistol-engine --example validate_config -- \
		--check-weights-file "${configs[@]}" || status=1
fi

if [ "${#weights[@]}" -gt 0 ]; then
	cargo run --quiet --locked --package pistol-eval --example validate_weights -- \
		"${weights[@]}" || status=1
fi

if [ "${#arenas[@]}" -gt 0 ]; then
	cargo run --quiet --locked --package pistol-arena --example validate_arena_config -- \
		"${arenas[@]}" || status=1
fi

exit "$status"
