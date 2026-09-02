#!/usr/bin/env bash
#
# Every citation a GOVERNING document makes is one the tree makes.
#
# WHAT THIS IS. A thin wrapper that names the documents currently GOVERNING
# something and runs `tools/design_citation_check.py` over them. The tool is not
# new (docs/decisions.md D-549); what is new is that it runs on a gate, a gap
# docs/experiments/wp20m_DESIGN_STOP.md:111 recorded and nothing acted on.
#
# WHY A NAMED LIST AND NOT A GLOB. Run over every document in `docs/`, this check
# reports unreproduced citations in seventy files. Almost all are RECORDS —
# reviews, closures, ledgers — whose citations were true when written and rotted
# as the tree moved, and demanding that a record be edited to match today's tree
# is demanding that history be falsified. The list holds documents that GOVERN a
# run or a decision now, where a stale citation misleads rather than remembers.
# Adding a document here is a commitment; removing one says it has become a
# record. Same shape as `tools/label_consistency_check.sh`'s own `DOCS` list.
#
# WHY `--proposes`. A registration legitimately names an instrument it does not
# yet have. Declaring it is a discipline rather than a courtesy: a document that
# must list what it invents cannot invent one by accident in a rewrite.
#
# Usage: tools/governing_citation_check.sh
# Exit:  0 every citation reproduces
#        1 a citation does not
#        2 THE RUN IS VOID — no answer was taken (tools/SHELL_CHECKLIST.md
#          item 12). A void is not a failure and must not be read as one.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VOID=2

void() {
	printf 'governing_citation_check: RUN VOID: %s\n' "$*" >&2
	printf 'governing_citation_check: no answer was taken; this is NOT a failure\n' >&2
	exit $VOID
}

command -v python3 >/dev/null || void "python3 is not on PATH"
[ -f tools/design_citation_check.py ] || void "tools/design_citation_check.py is missing"

# The documents that GOVERN something as of this revision.
GOVERNING=(
	CLAUDE.md
	docs/ROADMAP.md
	docs/process.md
	docs/experiments/anchor_v3_openings_design.md
	docs/experiments/matrix_label_cache_key.md
	docs/experiments/sealbot_anchor_v3_prereg.md
	docs/experiments/wp21_label_cache_design.md
	docs/experiments/wp21_prereg.md
	docs/experiments/wp21_throughput_prereg.md
)

# Files a governing document names and the tree does not hold yet, each with the
# document that proposes it. An entry that outlives its document is a finding.
PROPOSES=(
	# wp21_prereg.md §8: the assembly instrument §6's manifests need.
	tools/wp21_assemble.py
	# wp21_label_cache_design.md §6: the suite the cache package brings.
	crates/pistol-arena/tests/label_cache_tests.rs
)

for doc in "${GOVERNING[@]}"; do
	[ -f "$doc" ] || void "the governing document $doc is missing"
done

ARGS=()
for path in "${PROPOSES[@]}"; do
	ARGS+=(--proposes "$path")
done

printf 'governing_citation_check: %d governing document(s), %d proposed path(s)\n' \
	"${#GOVERNING[@]}" "${#PROPOSES[@]}"
python3 tools/design_citation_check.py "${ARGS[@]}" "${GOVERNING[@]}"
