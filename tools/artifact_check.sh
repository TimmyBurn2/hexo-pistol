#!/usr/bin/env bash
#
# Artifacts are never committed (CLAUDE.md rule 8). Nets, books, match logs and
# bench dumps are produced, sha-indexed by a manifest, and kept out of the
# history. This gate fails if any of them reached the index.
#
# It also enforces the 10 MB per-file ceiling that CLAUDE.md rule 7 puts on
# fixtures, applied to every tracked file: no tracked file has any business
# being that large.
#
# Match reports are additionally recognized by CONTENT — the first line of a
# report is its schema header (`arena_report <n>` / `arena_report_aborted <n>`,
# crates/pistol-arena/src/report.rs) — because the patterns below match names,
# and a report committed as `report.txt` sailed past them (wp13_results §6b,
# docs/decisions.md D-203).
#
# Usage: tools/artifact_check.sh
# Exit:  0 clean, 1 something is tracked that should not be.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

MAX_TRACKED_BYTES=$((10 * 1024 * 1024))

# Git pathspecs; `*` crosses directory separators here, so these match at any
# depth.
ARTIFACT_PATTERNS=(
	'*.bin'
	'*.net'
	'*.nnue'
	'*.book'
	'*.npz'
	'*.pt'
	'*.pth'
	'*.safetensors'
	'*.onnx'
	'*.pgn'
	'*.matchlog'
	'*.match.jsonl'
	'artifacts/*'
	'logs/*'
	'bench-out/*'
)

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || {
	echo "artifact_check: not a git repository" >&2
	exit 1
}

violations=()

while IFS= read -r -d '' path; do
	violations+=("artifact tracked: $path")
done < <(git ls-files -z -- "${ARTIFACT_PATTERNS[@]}")

while IFS= read -r -d '' path; do
	[ -f "$path" ] || continue
	size="$(stat -c %s "$path")"
	if [ "$size" -gt "$MAX_TRACKED_BYTES" ]; then
		violations+=("over ${MAX_TRACKED_BYTES}B: $path ($size bytes)")
	fi
	# The content signature. The bash builtin `read` costs no process per file;
	# `|| true` because an empty file, or one whose only line has no trailing
	# newline, returns nonzero under `set -e` while still filling the variable;
	# stderr is dropped because `read` complains about NUL bytes in binary
	# files it cannot match anyway. A CR is trimmed so a CRLF report cannot
	# slip through. A pathological first line is slurped whole, bounded in
	# practice by the size ceiling above.
	first=""
	IFS= read -r first <"$path" 2>/dev/null || true
	first="${first%$'\r'}"
	if [[ "$first" =~ ^arena_report(_aborted)?\ [0-9]+$ ]]; then
		violations+=("match report by content: $path (first line \"$first\")")
	fi
done < <(git ls-files -z)

if [ "${#violations[@]}" -gt 0 ]; then
	echo "artifact_check: FAIL" >&2
	printf '  %s\n' "${violations[@]}" >&2
	echo "artifact_check: artifacts are produced, sha-indexed by a manifest, and never committed" >&2
	exit 1
fi

echo "artifact_check: ok ($(git ls-files | wc -l) tracked files, none of them artifacts)"
