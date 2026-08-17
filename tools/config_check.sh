#!/usr/bin/env bash
#
# Every config under configs/ parses and validates. A committed config that
# does not load is a broken contract, not a small mistake.
#
# Note for WP-05: eval.weights_file names configs/eval_v0_weights.toml, which is
# an eval weight table and not an engine config. The moment WP-05 writes that
# file this glob will pick it up and reject it loudly — which is the moment to
# tell the two kinds of document apart here, and to add the weights-file
# existence check that docs/decisions.md D-21 defers until then.
#
# Usage: tools/config_check.sh [path ...]   (default: every .toml under configs/)
# Exit:  0 all valid, 1 one or more rejected or nothing to check.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

shopt -s nullglob globstar

if [ "$#" -gt 0 ]; then
	files=("$@")
else
	files=(configs/**/*.toml)
fi

if [ "${#files[@]}" -eq 0 ]; then
	echo "config_check: FAIL: no config files under configs/" >&2
	echo "config_check: the committed instrument config is part of the contract" >&2
	exit 1
fi

echo "config_check: checking ${#files[@]} file(s)"
cargo run --quiet --locked --package pistol-engine --example validate_config -- "${files[@]}"
