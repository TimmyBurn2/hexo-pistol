#!/usr/bin/env bash
#
# The WP-1.5b carve documents' SELF-STATE: every count a document states about
# its own enumeration equals what that enumeration holds.
#
# WHY THIS EXISTS. `docs/experiments/matrix_META1_REDTEAM.md` M3 ran a one-line
# shell loop over the six carve documents and found TWO live head/foot
# revision-label disagreements that no review round had recorded — one of them
# manufactured at `4fd88ec`, the commit AFTER `wp15b_U4_REVIEW_urev8.md`
# reported the identical defect in a sibling as its BLOCKING finding. That is
# docs/decisions.md D-335's generator (2), a universal about the document's own
# state, false in the commit that asserts it; and the whole point of M3 is that
# a STRUCTURE found it in one second in a class E2 said only a fresh reviewer's
# hand inventory can reach. D-338 records it as missing row R4.
#
# THE HEAD/FOOT u-rev LABEL CHECK THIS GATE ONCE RAN HERE IS RETIRED
# (docs/decisions.md D-311's appended amendment). Revision identity for these
# six documents is now the commit SHA, not an in-document label: a label is a
# SECOND, HAND-MAINTAINED COPY of a fact git already carries, and it is a
# defect-prone one — `e42ca88` appended 14 lines to `U4_soundness_instrument.md`
# WITHOUT bumping its head label, so `u-rev 9` came to name two different texts,
# the exact ambiguity a label exists to prevent, produced by the commit that
# amended D-311 to reaffirm the per-commit bump rule the label had just broken.
# A `git log`/`git show` against the document's own path is not defect-prone the
# same way: it cannot go stale independently of the bytes it describes. The
# self-referential claims a label invited — "THIS u-rev HAS NOT BEEN REVIEWED",
# "REVIEW STATUS", a document's own lineage of its post-carve u-revs — are
# struck from the documents themselves under D-346; this gate no longer checks
# for their staleness because the class it was checking for no longer has
# anywhere to occur.
#
# WHAT REMAINS, AND WHY IT IS A DIFFERENT CLASS. `matrix_META1_REDTEAM.md` M2:
# both landed claim inventories ship a headline count of their own table that
# their own table falsifies — fifty-four rows under a stated thirty-four, eleven
# failing rows under a stated six — uncaught by every round including the
# reviewer who read the earlier one closely. Those two live in REVIEW REPORTS,
# which are outside this gate's subject and are a reviewer's own text this
# project does not edit. Inside the subject the same form appears twice over,
# and it is checked here. NEITHER FORM NAMES A REVISION OR A u-rev; both are
# ordinary self-consistency of a count against what it counts, independent of
# the retired label:
#
#   1. THE SUMMAND LINE — `20 + 4 + 5 + 3 = **32**`. The arithmetic must hold,
#      and where the section heading above it states a count ("§11 — the 32 test
#      rows"), the heading and the total must agree. Two live instances.
#   2. THE GROUP-COUNT LINE — `**U2 (20):**` introducing a backtick-quoted list.
#      The stated number must equal the number of names in the group. Four live
#      instances.
#   3. THE SUMMANDS AGAINST THE TABLE ABOVE THEM. Where a summand line sits under
#      a table with one data row per summand, each summand must equal what its
#      row enumerates. Five live instances, all in `section_owner_table.md` §7.
#      THIS IS THE ONLY CHECK THAT REACHES §7 AT ALL, whose owners' items live in
#      table CELLS rather than in a backtick group — and without it the other two
#      checks are a property the defect PRESERVES, since a row that loses an item
#      leaves both the arithmetic and the heading untouched (REVIEW-impl MAJOR-2,
#      reproduced).
#
# NOT VACUOUS, AND MEASURED SO. CLAUDE.md forbids a criterion that the defect it
# names cannot falsify. Each of the three checks has live subjects in the
# tracked tree TODAY — two summand lines, five summand rows, four group counts —
# and a run that finds fewer has lost its subject rather than found it clean.
#
# WHAT THIS IS NOT. It is not a check that a document's revision is the RIGHT
# one, or current — that is `git log` against the path, judged by a reader, not
# mechanized. It is not a check of any count stated in PROSE about something the
# document does not itself enumerate; the two forms above are anchored, and a
# form this gate cannot resolve is not a form it guesses at. And it is not a
# citation gate: existence-checking `D-nnn` references was measured vacuous over
# these same six documents (576 references, 67 keys, zero dangling —
# `matrix_META1_REDTEAM.md` K2), which is why the row this gate implements is R4
# and not that one.
#
# Usage: tools/label_consistency_check.sh
# Exit:  0 every document's counted claims agree with themselves
#        1 a document's stated count disagrees with what it counts, or an
#          extraction found nothing — AN ANSWER, and it is no
#        2 THE RUN IS VOID: no answer was taken, the environment having refused
#
# THE THIRD CODE IS THE ITEM (tools/SHELL_CHECKLIST.md item 12). Absent git, a
# directory that is not a repository, an unreadable blob and a failed `mktemp`
# are not "a document disagrees with itself" — they are "I could not look", and
# spelling them 1 makes `ci: FAIL: label consistency` indistinguishable in a log
# from a carve document actually carrying a bad count.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

fail() { printf 'label_consistency_check: FAIL: %s\n' "$*" >&2; exit 1; }
# THE VOID, NAMED. Not `fail`: no answer about the documents was taken.
void() { printf 'label_consistency_check: RUN VOID: %s\n' "$*" >&2; exit 2; }

# ARGUMENTS ARE NOT SILENTLY IGNORED (tools/SHELL_CHECKLIST.md item 8;
# docs/decisions.md D-251 MINOR-3).
[ "$#" -eq 0 ] || fail "this gate takes no arguments and was given: $*"

# THE SUBJECT, PRINTED ON EVERY RUN rather than hidden in the source: a list
# maintained by memory is a list nobody re-reads (docs/decisions.md D-275). These
# are the six documents D-337 froze and D-338 released — the four WP-1.5b design
# units, the seed and the carve's owner table.
DOCS='docs/experiments/U1_gate_supersession.md
docs/experiments/U2_node_protocol.md
docs/experiments/U3_tier_t.md
docs/experiments/U4_soundness_instrument.md
docs/experiments/WPQ_seed.md
docs/experiments/section_owner_table.md'

command -v git >/dev/null || void "git is not on PATH, so the tracked bytes cannot be read"
command -v awk >/dev/null || void "awk is not on PATH, and it is this gate's whole extraction"
git rev-parse --is-inside-work-tree >/dev/null 2>&1 ||
	void "not a git repository: this gate reads the TRACKED bytes of its subject"

# NAMED, not a bare `set -e` death (tools/SHELL_CHECKLIST.md item 1).
WORK="$(mktemp -d)" || void "mktemp could not make a scratch directory"
# The trap preserves the body's status rather than replacing it with `rm`'s
# (item 7): a cleanup that fails must not turn a clean run into a refusal.
trap 'rc=$?; rm -rf "$WORK"; exit "$rc"' EXIT

# --- the extraction ----------------------------------------------------------
#
# One awk pass per document, emitting records the shell adjudicates. Written to a
# file rather than inlined so the SELF-TEST below drives the very same program
# the tracked documents are read by; a self-test against a second copy tests the
# copy.

cat >"$WORK/extract.awk" <<'AWKEOF'
# Every line is buffered and the work is done in END.
{ L[NR] = $0 }

END {
	n = NR
	sum_seen = 0; group_seen = 0

	# --- THE COUNTED FORMS ----------------------------------------------------
	heading_n = ""
	for (i = 1; i <= n; i++) {
		if (L[i] ~ /^#+ /) {
			heading_n = ""
			if (match(L[i], /the [0-9]+ /)) heading_n = substr(L[i], RSTART + 4, RLENGTH - 5)
			continue
		}

		# THE GROUP-COUNT LINE: `**Label (n):**` over a backtick list.
		if (L[i] ~ /^\*\*[A-Za-z0-9 ]+ \([0-9]+\):\*\*/) {
			label = L[i]; sub(/^\*\*/, "", label); sub(/ \(.*$/, "", label)
			stated = L[i]
			sub(/^\*\*[A-Za-z0-9 ]+ \(/, "", stated); sub(/\):\*\*.*$/, "", stated)
			ticks = gsub(/`/, "`", L[i])
			for (j = i + 1; j <= n && L[j] ~ /[^[:space:]]/; j++) ticks += gsub(/`/, "`", L[j])
			# THE LABEL IS LAST. The extraction's own character class admits a
			# space in a label, and a positionally-read record then misaligns
			# every field after it: a CORRECT two-word group was refused with
			# `group New states Plan and enumerates 4 4` (REVIEW-impl MAJOR-3,
			# reproduced). Last field absorbs the spaces.
			printf "GROUP %d %d %d %s\n", i, stated + 0, int(ticks / 2), label
			group_seen++
			continue
		}

		# THE SUMMAND LINE: `3 + 11 + 5 + 2 + 2 = **23**`.
		if (L[i] ~ /^[0-9]+( \+ [0-9]+)+ = \*\*[0-9]+\*\*/) {
			stated = L[i]; sub(/^.* = \*\*/, "", stated); sub(/\*\*.*$/, "", stated)
			terms = L[i]; sub(/ = .*$/, "", terms)
			parts = split(terms, part, / \+ /)
			total = 0
			for (t = 1; t <= parts; t++) total += part[t] + 0
			printf "SUM %d %d %d %s\n", i, total, stated + 0, (heading_n == "" ? "-" : heading_n)
			sum_seen++

			# AND THE SUMMANDS AGAINST WHAT THE TABLE ABOVE ENUMERATES. The
			# arithmetic and the heading agreeing with each other is a property
			# the named defect PRESERVES — a table row that loses an item leaves
			# both untouched (REVIEW-impl MAJOR-2, reproduced). `section_owner_table.md`
			# §7 is reachable only this way: its owners' items live in table
			# CELLS, not in a backtick group.
			r = i - 1
			while (r >= 1 && L[r] !~ /[^[:space:]]/) r--
			rows = 0
			while (r >= 1 && L[r] ~ /^\|/) { rowline[rows] = L[r]; rows++; r-- }
			# Drop the separator and the header it follows, counting only data.
			data = 0
			for (t = rows - 1; t >= 0; t--) {
				if (rowline[t] ~ /^\|[ :|-]+\|[[:space:]]*$/) { data = 0; continue }
				datarow[data] = rowline[t]; data++
			}
			if (data == parts) {
				for (t = 0; t < data; t++) {
					cell = datarow[t]
					sub(/^\|[^|]*\|/, "", cell)   # drop the owner column
					sub(/\|[[:space:]]*$/, "", cell)
					gsub(/[[:space:]]/, "", cell)
					items = (cell == "" ? 0 : split(cell, it, /,/))
					printf "SUMROW %d %d %d %d\n", i, t + 1, part[t + 1] + 0, items
				}
			}
		}
	}
	printf "SEEN %d %d\n", sum_seen, group_seen
}
AWKEOF

# --- the self-test, before the tracked files ---------------------------------
#
# A gate nobody has watched refuse is not a gate. Each check is watched saying
# "no" about a shape that must never appear, and watched saying "yes" about the
# shape the real documents have — a refusal without a control is satisfied by a
# gate that refuses everything.

SEED_SUM="$(printf '## 4. S — the 32 test rows\n\n**A (2):** `x`, `y`.\n\n**B (30):** `z`.\n\n20 + 12 = **32**\n' |
	awk -f "$WORK/extract.awk")"
case "$SEED_SUM" in
*'SUM 7 32 32 32'*) ;;
*) fail "self-test: a summand line totalling 32 under a heading stating 32 must extract as 32 32 32; it gave: $SEED_SUM" ;;
esac
case "$SEED_SUM" in
*'GROUP 3 2 2 A'*'GROUP 5 30 1 B'*) ;;
*) fail "self-test: the group form must count backticked names, LABEL LAST; it gave: $SEED_SUM" ;;
esac

# A MULTI-WORD LABEL — REVIEW-impl MAJOR-3, reproduced. The extraction's own
# character class admits a space, and the record's label therefore goes LAST so a
# positionally-read field never absorbs half of it.
SEED_LABEL="$(printf '**New Plan (2):** `x`, `y`.\n' | awk -f "$WORK/extract.awk")"
case "$SEED_LABEL" in
*'GROUP 1 2 2 New Plan'*) ;;
*) fail "self-test: a two-word group label must land in the record's LAST field; it gave: $SEED_LABEL" ;;
esac

# THE SUMMANDS AGAINST THE TABLE — REVIEW-impl MAJOR-2, reproduced. Arithmetic
# and heading agreeing is a property a table row that loses an item PRESERVES.
SEED_ROWS="$(printf '## 7. S — the 6 items\n\n| Owner | Items |\n|---|---|\n| **A** | 1, 2, 3 |\n| **B** | 4, 5 |\n\n3 + 3 = **6**\n' |
	awk -f "$WORK/extract.awk")"
case "$SEED_ROWS" in
*'SUMROW 8 1 3 3'*'SUMROW 8 2 3 2'*) ;;
*) fail "self-test: the second summand is 3 against a row enumerating 2 and must be reported; it gave: $SEED_ROWS" ;;
esac
echo "label_consistency_check: self-test passed — a summand line, a two-word label and a summand against its table row"

# --- the tracked bytes -------------------------------------------------------
#
# THE INDEX IS WHAT COMMITS (tools/SHELL_CHECKLIST.md item 5). `git ls-files`
# names a PATH, and opening that path reads the WORKTREE file of that name: stage
# a stale count and overwrite the worktree copy with a repaired one, and a gate
# reading paths passes it while the real bytes go to HEAD.

BAD=0
DOCS_SEEN=0
# NOT `SUMS` AND `GROUPS` (tools/SHELL_CHECKLIST.md item 8's neighbourhood: one
# spelling per number). `GROUPS` is one of bash's OWN special variables — the
# array of the caller's group IDs — so `GROUPS=$((GROUPS + 1))` is not the
# counter it looks like. MEASURED, on this gate's second run against its own
# subject: four group records went in and the summary line printed `1000 group
# count(s)`, exit 0. A gate that prints a wrong number and passes is this
# checklist's whole subject, and it arrived through a NAME rather than through
# any of the parsing the checklist warns about.
SUM_COUNT=0
GROUP_COUNT=0
ROW_COUNT=0

while IFS= read -r doc; do
	[ -n "$doc" ] || continue
	BLOB=""
	# `-s -z` prints `<mode> SP <object> SP <stage> TAB <path>` per
	# NUL-terminated record, the only spelling that survives a path holding a
	# newline (item 9).
	while IFS= read -r -d '' entry; do
		meta="${entry%%$'\t'*}"
		meta="${meta#* }"
		object="${meta%% *}"
		[ -z "$BLOB" ] || fail "$doc resolves to more than one tracked blob; this gate reads exactly one"
		git cat-file blob "$object" >"$WORK/blob" 2>/dev/null ||
			void "git could not read the tracked blob $object for $doc"
		BLOB="$WORK/blob"
	done < <(git ls-files -s -z -- "$doc")
	# NEVER a silent skip: a tracked-but-absent subject is a refusal, not a
	# document that happens not to be checked (item 5).
	[ -n "$BLOB" ] || fail "$doc is not in the git index; this gate's subject list names it"

	DOCS_SEEN=$((DOCS_SEEN + 1))
	RECORDS="$(awk -f "$WORK/extract.awk" "$BLOB")"

	# ONE pass over the records, split to the WIDEST record's field count. Read
	# with four fields, a SUM record's fourth and fifth collapse into one and the
	# arithmetic comparison is then `32` against `32 32` — measured, on this
	# gate's first run against its own subject (tools/SHELL_CHECKLIST.md item 8:
	# validate the spelling, not only the value).
	while read -r kind f2 f3 f4 f5; do
		case "$kind" in
		SUM)
			SUM_COUNT=$((SUM_COUNT + 1))
			if [ "$f3" != "$f4" ]; then
				printf 'label_consistency_check: %s:%s the summands total %s and the line states %s\n' "$doc" "$f2" "$f3" "$f4" >&2
				BAD=$((BAD + 1))
			fi
			# `-` is "the governing heading states no count", which is not a defect.
			if [ "$f5" != "-" ] && [ "$f5" != "$f4" ]; then
				printf 'label_consistency_check: %s:%s the heading states %s and the stated total is %s\n' "$doc" "$f2" "$f5" "$f4" >&2
				BAD=$((BAD + 1))
			fi
			;;
		SUMROW)
			ROW_COUNT=$((ROW_COUNT + 1))
			if [ "$f4" != "$f5" ]; then
				printf 'label_consistency_check: %s:%s summand %s is %s and the table row above enumerates %s\n' "$doc" "$f2" "$f3" "$f4" "$f5" >&2
				BAD=$((BAD + 1))
			fi
			;;
		GROUP)
			GROUP_COUNT=$((GROUP_COUNT + 1))
			if [ "$f3" != "$f4" ]; then
				printf 'label_consistency_check: %s:%s group %s states %s and enumerates %s\n' "$doc" "$f2" "$f5" "$f3" "$f4" >&2
				BAD=$((BAD + 1))
			fi
			;;
		SEEN) ;;
		*) fail "$doc: the extraction emitted a record kind this gate does not define: \`$kind\`" ;;
		esac
	done <<<"$RECORDS"
done <<<"$DOCS"

echo "label_consistency_check: $DOCS_SEEN document(s) read"

# --- non-vacuity -------------------------------------------------------------
#
# A criterion the named defect cannot falsify is not a criterion (CLAUDE.md).
# Each of the three checks has live subjects in the tracked tree TODAY — two
# summand lines, five summand rows, four group counts — and a run that finds
# fewer has lost its subject rather than found it clean.
EXPECT_DOCS="$(printf '%s\n' "$DOCS" | wc -l)"
[ "$DOCS_SEEN" -eq "$EXPECT_DOCS" ] ||
	fail "the subject list names $EXPECT_DOCS documents and $DOCS_SEEN were read"
[ "$SUM_COUNT" -ge 2 ] ||
	fail "the summand-line check found $SUM_COUNT lines to check; it had 2 live subjects when it landed, so a smaller number is the EXTRACTION drifting off its subject, not the documents going clean"
[ "$ROW_COUNT" -ge 5 ] ||
	fail "the summand-against-table check found $ROW_COUNT row(s) to check; it had 5 live subjects when it landed, so a smaller number is the EXTRACTION drifting off its subject, not the documents going clean"
[ "$GROUP_COUNT" -ge 4 ] ||
	fail "the group-count check found $GROUP_COUNT groups to check; it had 4 live subjects when it landed, so a smaller number is the EXTRACTION drifting off its subject, not the documents going clean"

[ "$BAD" -eq 0 ] ||
	fail "$BAD self-state disagreement(s) above; a document that misdescribes its own count is docs/decisions.md D-335's generator (2), and D-338 records this gate as the row that reaches it"

echo "label_consistency_check: $DOCS_SEEN documents, $SUM_COUNT summand line(s), $ROW_COUNT summand row(s), $GROUP_COUNT group count(s) — every document agrees with itself"
