#!/usr/bin/env bash
#
# Every document under configs/ parses and validates, and every engine config
# names a weights file that is actually there. A committed document that does not
# load is a broken contract, not a small mistake.
#
# Four document kinds live under configs/, with different schemas and separate
# versions: an engine config (pistol-engine's `Config`), an eval weight table
# (pistol-eval's `Weights`), an arena config (pistol-arena's `ArenaConfig`) and a
# random-openings config (pistol-cli's `RandomOpeningsConfig`). They are told
# apart by file name — `*_weights.toml` is a weight table, `arena_*.toml` is an
# arena config, `random_openings_*.toml` is a book generator's — and each goes to
# its own validator, because a document checked against another schema would be
# rejected for every key it does not have (docs/decisions.md D-64, D-168, D-176).
#
# Anything else is an engine config. That fallback is why a new kind must be
# named here the day it is committed: an unrecognised document is not skipped, it
# is checked against the engine's schema and refused for every key it lacks.
#
# Each schema version is its own and none is ever compared with another: they
# version different documents that change for different reasons.
#
# `--check-weights-file` is the weights-file existence check docs/decisions.md
# D-21 defers to a gate and D-66 puts in the validator example: config validation
# itself stays pure and offline, so this is where a config that points at a file
# nobody committed gets caught.
#
# Usage: tools/config_check.sh [path ...]   (default: every .toml under configs/)
# Exit:  0 all valid
#        1 one or more documents were rejected, or there was nothing to check
#        2 THE RUN IS VOID — a validator could not be BUILT, so no document was
#          adjudicated (tools/SHELL_CHECKLIST.md item 12)
#
# THE VOID IS REAL HERE AND AN EARLIER REVISION SAID IT WAS NOT. This gate is
# not a pure reader: it BUILDS five validators, and `cargo run` conflates "the
# document is bad" with "the binary would not compile" in one exit 1. On a full
# disk that read as a broken config — cargo's vocabulary answering a question
# about a document. The builds are therefore separated out below and a build
# failure is a void.

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
books=()
solvers=()
for file in "${files[@]}"; do
	case "$(basename "$file")" in
	*_weights.toml) weights+=("$file") ;;
	arena_*.toml) arenas+=("$file") ;;
	random_openings_*.toml) books+=("$file") ;;
	solver_*.toml) solvers+=("$file") ;;
	*) configs+=("$file") ;;
	esac
done

echo "config_check: ${#configs[@]} engine config(s), ${#weights[@]} weight table(s), ${#arenas[@]} arena config(s), ${#books[@]} book config(s), ${#solvers[@]} solver config(s)"

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
		echo "config_check: tools/arena_smoke.sh runs a committed one (CI gate 13 of 16)" >&2
		exit 1
	fi
	if [ "${#books[@]}" -eq 0 ]; then
		echo "config_check: FAIL: no random-openings config under configs/" >&2
		echo "config_check: a committed one is the only home of the parameters" >&2
		echo "config_check: random_openings_v1.txt was generated with (D-177)" >&2
		exit 1
	fi
	if [ "${#solvers[@]}" -eq 0 ]; then
		echo "config_check: FAIL: no solver config under configs/" >&2
		echo "config_check: the solver's tunables live in exactly one committed place (WP-1.8a)" >&2
		exit 1
	fi
fi

void() {
	printf 'config_check: RUN VOID: %s\n' "$*" >&2
	exit 2
}

# THE BUILD IS SEPARATED FROM THE RUN so the two answers stay apart. Only the
# validators a non-empty bucket needs are built, so an explicit-path run pays
# for one and not five.
build_validator() {
	local package="$1" example="$2"
	cargo build --quiet --locked --package "$package" --example "$example" ||
		void "the $example validator does not build; nothing was adjudicated and no \
document is implicated"
}

status=0

if [ "${#configs[@]}" -gt 0 ]; then
	build_validator pistol-engine validate_config
	cargo run --quiet --locked --package pistol-engine --example validate_config -- \
		--check-weights-file "${configs[@]}" || status=1
fi

if [ "${#weights[@]}" -gt 0 ]; then
	build_validator pistol-eval validate_weights
	cargo run --quiet --locked --package pistol-eval --example validate_weights -- \
		"${weights[@]}" || status=1
fi

if [ "${#arenas[@]}" -gt 0 ]; then
	build_validator pistol-arena validate_arena_config
	cargo run --quiet --locked --package pistol-arena --example validate_arena_config -- \
		"${arenas[@]}" || status=1
fi

if [ "${#solvers[@]}" -gt 0 ]; then
	build_validator pistol-solver validate_solver_config
	cargo run --quiet --locked --package pistol-solver --example validate_solver_config -- \
		"${solvers[@]}" || status=1
fi

if [ "${#books[@]}" -gt 0 ]; then
	build_validator pistol-cli validate_random_openings_config
	cargo run --quiet --locked --package pistol-cli --example validate_random_openings_config -- \
		"${books[@]}" || status=1
fi

exit "$status"
