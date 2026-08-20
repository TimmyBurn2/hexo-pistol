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
# Match reports and baseline snapshots are additionally recognized by CONTENT —
# the first line of each is its schema header (`arena_report <n>` /
# `arena_report_aborted <n>`, crates/pistol-arena/src/report.rs;
# `baseline_snapshot <n>` / `baseline_snapshot_incomplete <n>`,
# tools/baseline_snapshot.sh) — because the patterns below match names, and a
# report committed as `report.txt` sailed past them (wp13_results §6b,
# docs/decisions.md D-203). A new record kind that routed around this
# recognizer instead of joining it would re-open that hole, so the snapshot
# joined it (docs/decisions.md D-230).
#
# THE GATE READS THE TRACKED BYTES AND NOT THE WORKING TREE (docs/decisions.md
# D-233). It used to open `$path` from the worktree, which is a DIFFERENT FILE
# from the one `git ls-files` named: staging a record and then overwriting the
# worktree copy with something harmless passed the gate green and the record went
# to HEAD, and a committed record whose worktree copy was merely deleted was
# skipped by `[ -f "$path" ] || continue` — along with the size ceiling for the
# same paths (both REPRODUCED in a scratch repository). D-203's contract is "a
# TRACKED file whose first line is the schema header", so the bytes read are the
# INDEX's: `git ls-files -s` names each path's blob and the content comes from
# that object. The skip is gone with the worktree read that needed it.
#
# Usage: tools/artifact_check.sh
# Exit:  0 clean, 1 something is tracked that should not be.

set -euo pipefail

# The leading-whitespace trim below is only as wide as the locale's notion of a
# space, and this gate had no locale of its own: `LC_ALL=C` and
# `LC_ALL=en_US.utf8` disagree about U+2028, so the same tracked file got two
# answers on two machines. Pinned to C, which makes the gate DETERMINISTIC and
# states its width exactly — ASCII whitespace, and no more. NAMED RESIDUAL: a
# first line led by a non-ASCII space is not trimmed and so is not recognized,
# which is the same class as D-203's content-signature residual and stands behind
# the name patterns below (docs/decisions.md D-233).
export LC_ALL=C

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

# Every tracked path beside the INDEX BLOB it names. `-s -z` prints
# `<mode> SP <object> SP <stage> TAB <path>` per NUL-terminated record, which is
# the only spelling that survives a path containing a newline.
paths=()
blobs=()
while IFS= read -r -d '' entry; do
	meta="${entry%%$'\t'*}"
	paths+=("${entry#*$'\t'}")
	meta="${meta#* }"
	blobs+=("${meta%% *}")
done < <(git ls-files -s -z)

# The sizes, in ONE process: `--batch-check` answers one line per requested
# object, in order, so the tracked SIZE is the blob's and not the worktree copy's.
sizes=()
if [ "${#blobs[@]}" -gt 0 ]; then
	mapfile -t sizes < <(printf '%s\n' "${blobs[@]}" | git cat-file --batch-check='%(objectsize)')
	[ "${#sizes[@]}" -eq "${#blobs[@]}" ] || {
		echo "artifact_check: git reported ${#sizes[@]} object sizes for ${#blobs[@]} tracked blobs" >&2
		exit 1
	}
fi

for i in "${!paths[@]}"; do
	path="${paths[$i]}"
	size="${sizes[$i]}"
	if [ "$size" -gt "$MAX_TRACKED_BYTES" ]; then
		violations+=("over ${MAX_TRACKED_BYTES}B: $path ($size bytes)")
	fi
	# The content signature, over the blob. `|| true` because a blob that is
	# empty, or whose only line has no trailing newline, returns nonzero under
	# `set -e` while still filling the variable; stderr is dropped because `read`
	# complains about NUL bytes in binary files it cannot match anyway, and
	# because `git cat-file` takes a SIGPIPE when this stops reading a large blob
	# after its first line. A CR is trimmed so a CRLF report cannot slip through.
	# A pathological first line is slurped whole, bounded in practice by the size
	# ceiling above.
	# The match is on the header's TOKENS, not the exact line: a consumer that
	# splits on whitespace reads `arena_report 4 extra` or a trailing-space
	# variant as a report, so the gate must too (RED-TEAM's bypass, D-205).
	# LEADING whitespace splits to exactly the same tokens and slipped both
	# recognizers, as did a UTF-8 BOM, so both are trimmed for the same reason
	# the trailing CR is (D-230). The violation quotes the NORMALIZED line, so a
	# reader can see what the gate matched rather than guess at the invisible
	# bytes it stripped.
	first=""
	IFS= read -r first < <(git cat-file blob "${blobs[$i]}" 2>/dev/null) 2>/dev/null || true
	first="${first%$'\r'}"
	first="${first#$'\xef\xbb\xbf'}"
	first="${first#"${first%%[![:space:]]*}"}"
	if [[ "$first" =~ ^arena_report(_aborted)?[[:space:]]+[0-9]+([[:space:]]|$) ]]; then
		violations+=("match report by content: $path (first line \"$first\")")
	fi
	if [[ "$first" =~ ^baseline_snapshot(_incomplete)?[[:space:]]+[0-9]+([[:space:]]|$) ]]; then
		violations+=("baseline snapshot by content: $path (first line \"$first\")")
	fi
done

if [ "${#violations[@]}" -gt 0 ]; then
	echo "artifact_check: FAIL" >&2
	printf '  %s\n' "${violations[@]}" >&2
	echo "artifact_check: artifacts are produced, sha-indexed by a manifest, and never committed" >&2
	exit 1
fi

echo "artifact_check: ok ($(git ls-files | wc -l) tracked files, none of them artifacts)"
