#!/usr/bin/env bash
#
# The WP-1.5b carve documents' SELF-STATE: a document's head u-rev equals its
# foot u-rev, and every count it states about its own enumeration equals what it
# enumerates.
#
# WHY THIS EXISTS. `docs/experiments/matrix_META1_REDTEAM.md` M3 ran a one-line
# shell loop over the six carve documents and found TWO live head/foot
# disagreements that no review round had recorded — one of them manufactured at
# `4fd88ec`, the commit AFTER `wp15b_U4_REVIEW_urev8.md` reported the identical
# defect in a sibling as its BLOCKING finding. That is docs/decisions.md D-335's
# generator (2), a universal about the document's own state, false in the commit
# that asserts it; and the whole point of M3 is that a STRUCTURE found it in one
# second in a class E2 said only a fresh reviewer's hand inventory can reach.
# D-338 records it as missing row R4.
#
# AND THE LOOP THAT FOUND TWO MISSED A THIRD. It read a 3-line tail. Three of the
# six documents close with an italic paragraph WRAPPED over several lines, and
# `U4_soundness_instrument.md` closes with one that is a single very long line
# behind a `---` rule: on U4 that loop printed a truncated `foot=u-rev` with no
# number, which its author read as noise rather than as a refusal. U4 was head
# `u-rev 8` against foot `u-rev 7` the whole time. So this gate does not read a
# tail of fixed depth; it reads the CLOSING BLOCK, and it refuses a foot whose
# label is not resolvable rather than reporting nothing.
#
# AND THE SECOND SPELLING WAS WORSE THAN THE FIRST. Reading "the block after the
# last line starting with an asterisk and a capital" fixed U4 and opened an
# EXIT-0-WRONG-ANSWER: a document whose true closing paragraph is STALE, followed
# by a trailing italic aside naming the HEAD's u-rev, reported `foot=u-rev 4 OK`
# and exited 0 — the defect this gate exists for, passing, in the one code path
# whose whole job is to prevent it (REVIEW-impl BLOCKING-1 against `908a2f7`,
# reproduced). So the unit is neither a tail nor a paragraph but the CLOSING
# REGION: everything below the document's last `---` rule, which must name
# exactly one u-rev. MEASURED: all six subject documents name exactly one there.
#
# THE FOLD LAW IS WHAT MAKES THE FOOT RESOLVABLE (docs/decisions.md D-331). A
# closing line that carries the u-rev LABEL and nothing else cannot go stale
# against the head independently of the head. A closing line that also recounts
# what each u-rev did names several u-revs, and there is then no fact of the
# matter about which one is the document's own — which is why "the foot names
# more than one u-rev" is a REFUSAL here and not a tolerance. Both repaired
# documents were repaired that way.
#
# WHAT THE SECOND CHECK IS FOR. `matrix_META1_REDTEAM.md` M2: both landed claim
# inventories ship a headline count of their own table that their own table
# falsifies — fifty-four rows under a stated thirty-four, eleven failing rows
# under a stated six — uncaught by every round including the reviewer who read
# the earlier one closely. Those two live in REVIEW REPORTS, which are outside
# this gate's subject and are a reviewer's own text this project does not edit.
# Inside the subject the same form appears twice over, and it is checked here:
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
# names cannot falsify. Run against `1f834ca`, the revision before this round's
# repairs, this gate refuses THREE of the six documents — U1 and U4 for an
# unresolvable foot, `section_owner_table.md` for head 6 against foot 5. That is
# one more than the loop it is built from found. And the gate refuses outright if
# any of its three checks finds NOTHING to check, so a document set that drifts
# out from under the extraction is a refusal rather than a silent green.
#
# WHAT THIS IS NOT. It is not a check that the u-rev is the RIGHT one — that a
# document at u-rev 8 has had eight revisions is judged, not mechanized. It is
# not a check of any count stated in PROSE about something the document does not
# itself enumerate; the two forms above are anchored, and a form this gate cannot
# resolve is not a form it guesses at. And it is not a citation gate: existence-
# checking `D-nnn` references was measured vacuous over these same six documents
# (576 references, 67 keys, zero dangling — `matrix_META1_REDTEAM.md` K2), which
# is why the row this gate implements is R4 and not that one.
#
# RULE9-JUSTIFICATION: one gate, one subject, three checks that share it. The
# head/foot check and the two counted forms all read the SAME six documents in
# the same awk pass, and the larger half of this file is the reasons each check
# is spelled the way it is — a closing BLOCK rather than a fixed tail, an
# unresolvable foot as a refusal rather than a tolerance, and a non-vacuity floor
# under each extraction. Splitting them splits the subject list and the
# extraction with it, and two subject lists that drift apart is the class this
# gate exists for. Its self-test and its documented WHY are what a reader of a
# gate that landed one commit after the defect it catches needs to see.
#
# Usage: tools/label_consistency_check.sh
# Exit:  0 every document's self-state is consistent
#        1 a document disagrees with itself, or an extraction found nothing —
#          AN ANSWER, and it is no
#        2 THE RUN IS VOID: no answer was taken, the environment having refused
#
# THE THIRD CODE IS THE ITEM (tools/SHELL_CHECKLIST.md item 12). Absent git, a
# directory that is not a repository, an unreadable blob and a failed `mktemp`
# are not "a document disagrees with itself" — they are "I could not look", and
# spelling them 1 makes `ci: FAIL: label consistency` indistinguishable in a log
# from a carve document actually carrying a stale label.

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
# Every line is buffered and the work is done in END. A streaming pass carried
# state across paragraph boundaries — `infoot` was set and never reset, so the
# "closing block" became whatever followed the LAST asterisk-uppercase line in
# the file, and a trailing italic aside naming a DIFFERENT u-rev masked a stale
# foot at exit 0 (REVIEW-impl BLOCKING-1 against 908a2f7, reproduced). Buffering
# costs nothing at these sizes and removes the class.
{ L[NR] = $0 }

END {
	n = NR
	head = ""; headline = 0; sum_seen = 0; group_seen = 0

	# --- CHECK A, the head label ---------------------------------------------
	# FIRST match only: the head is the document's own u-rev, and later prose
	# mentioning a u-rev is not it.
	for (i = 1; i <= n && head == ""; i++) {
		if (L[i] ~ /^\*\*u-rev [0-9]+/) {
			s = L[i]
			sub(/^\*\*u-rev /, "", s)
			sub(/[^0-9].*$/, "", s)
			head = s
			headline = i
		}
	}
	printf "HEAD %s %d\n", (head == "" ? "NONE" : head), headline

	# --- CHECK A, the CLOSING REGION -----------------------------------------
	# NOT "the last paragraph", and NOT a tail of fixed depth. The closing region
	# is everything after the document's LAST horizontal rule, and it must name
	# exactly ONE u-rev.
	#
	# A fixed tail was the first spelling and it missed U4 entirely, whose
	# closing paragraph is one very long line. "The block after the last
	# asterisk-uppercase line" was the second and it is worse: a trailing aside
	# that happens to name the head's u-rev makes a stale foot read as clean, at
	# exit 0. The REGION is the honest unit — a document says its u-rev once,
	# below the rule, or it does not say it resolvably at all. MEASURED: all six
	# subject documents carry exactly one `u-rev <n>` after their last rule.
	rule = 0
	for (i = n; i >= 1; i--) {
		if (L[i] ~ /^---[[:space:]]*$/) { rule = i; break }
	}
	if (rule == 0) {
		printf "FOOTNORULE 0 0\n"
	} else {
		k = 0; first = ""; firstline = 0
		for (i = rule + 1; i <= n; i++) {
			rest = L[i]
			# match(), not a greedy sub(): `sub(/^.*u-rev /, ...)` takes the LAST
			# occurrence, which on U4's repaired closing line silently yielded an
			# EMPTY label. Measured, during this gate's build.
			while (match(rest, /u-rev [0-9]+/)) {
				tok = substr(rest, RSTART + 6, RLENGTH - 6)
				if (k == 0) { first = tok; firstline = i }
				k++
				rest = substr(rest, RSTART + RLENGTH)
			}
		}
		if (k == 1) printf "FOOT %s %d\n", first, firstline
		else printf "FOOTBAD %d %d\n", k, rule
	}

	# --- CHECK B, the counted forms ------------------------------------------
	heading_n = ""
	for (i = 1; i <= n; i++) {
		if (L[i] ~ /^#+ /) {
			heading_n = ""
			if (match(L[i], /the [0-9]+ /)) heading_n = substr(L[i], RSTART + 4, RLENGTH - 5)
			continue
		}

		# B2, the group-count line: `**Label (n):**` over a backtick list.
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

		# B1, the summand line: `3 + 11 + 5 + 2 + 2 = **23**`.
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
# A gate nobody has watched refuse is not a gate. Each of the three checks is
# watched saying "no" about a shape that must never appear, and watched saying
# "yes" about the shape the real documents have — a refusal without a control is
# satisfied by a gate that refuses everything.

seed_head_foot() { # $1 = head u-rev, $2 = closing region text
	printf '# T\n\n**u-rev %s.** A seeded document.\n\n---\n\n*T, %s*\n' "$1" "$2"
}

SEED_OK="$(seed_head_foot 4 'u-rev 4. The label alone.' | awk -f "$WORK/extract.awk")"
case "$SEED_OK" in
*'HEAD 4 3'*'FOOT 4 7'*) ;;
*) fail "self-test: the clean seed is head 4 / foot 4 and the extraction said: $SEED_OK" ;;
esac

SEED_STALE="$(seed_head_foot 4 'u-rev 3. The label alone.' | awk -f "$WORK/extract.awk")"
case "$SEED_STALE" in
*'HEAD 4 3'*'FOOT 3 7'*) ;;
*) fail "self-test: a foot one behind the head must extract as 4 and 3; it gave: $SEED_STALE" ;;
esac

SEED_AMBIG="$(seed_head_foot 4 'u-rev 4. u-rev 3 was a carve.' | awk -f "$WORK/extract.awk")"
case "$SEED_AMBIG" in
*'FOOTBAD 2 5'*) ;;
*) fail "self-test: a closing region naming two u-revs must be FOOTBAD 2; it gave: $SEED_AMBIG" ;;
esac

# THE MASKING ASIDE — REVIEW-impl BLOCKING-1 against `908a2f7`, reproduced and
# then pinned here. A stale foot followed by a trailing italic note naming the
# HEAD's u-rev read as `foot=u-rev 4 OK` at exit 0 under the previous spelling,
# which located the foot by the LAST asterisk-uppercase line. The closing REGION
# sees both labels and refuses.
SEED_MASK="$(printf '# T\n\n**u-rev 4.** x\n\n---\n\n*T, u-rev 3. The label alone.*\n\n*Folded into u-rev 4 of the seed, see its own foot.*\n' |
	awk -f "$WORK/extract.awk")"
case "$SEED_MASK" in
*'FOOTBAD 2 5'*) ;;
*) fail "self-test: a stale foot masked by a trailing aside must be FOOTBAD 2, not a pass; it gave: $SEED_MASK" ;;
esac

# The wrapped closing block, and the long single line behind a rule: the two
# shapes a fixed-depth tail could not both read.
SEED_WRAP="$(printf '# T\n\n**u-rev 9.** x\n\n---\n\n*T, u-rev 9. What each\nrevision did is the head block'"'"'s, and this\nline restates none of it.*\n' |
	awk -f "$WORK/extract.awk")"
case "$SEED_WRAP" in
*'FOOT 9 7'*) ;;
*) fail "self-test: a WRAPPED closing block must resolve to u-rev 9; it gave: $SEED_WRAP" ;;
esac

SEED_NORULE="$(printf '# T\n\n**u-rev 4.** x\n\n*T, u-rev 4.*\n' | awk -f "$WORK/extract.awk")"
case "$SEED_NORULE" in
*'FOOTNORULE'*) ;;
*) fail "self-test: a document with no closing rule has no closing region; it gave: $SEED_NORULE" ;;
esac

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
echo "label_consistency_check: self-test passed — a clean pair, a stale foot, an ambiguous region, a MASKING ASIDE, a wrapped block, a missing rule, a summand line, a two-word label and a summand against its table row"

# --- the tracked bytes -------------------------------------------------------
#
# THE INDEX IS WHAT COMMITS (tools/SHELL_CHECKLIST.md item 5). `git ls-files`
# names a PATH, and opening that path reads the WORKTREE file of that name: stage
# a stale label and overwrite the worktree copy with a repaired one, and a gate
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

	head=""; headline=""; foot=""; footline=""; footbad=""; footnorule=""
	# ONE pass over the records, split to the WIDEST record's field count. Read
	# with four fields, a SUM record's fourth and fifth collapse into one and the
	# arithmetic comparison is then `32` against `32 32` — measured, on this
	# gate's first run against its own subject (tools/SHELL_CHECKLIST.md item 8:
	# validate the spelling, not only the value).
	while read -r kind f2 f3 f4 f5; do
		case "$kind" in
		HEAD) head="$f2"; headline="$f3" ;;
		FOOT) foot="$f2"; footline="$f3" ;;
		FOOTBAD) footbad="$f2"; footline="$f3" ;;
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
		FOOTNORULE) footnorule=1 ;;
		SEEN) ;;
		*) fail "$doc: the extraction emitted a record kind this gate does not define: \`$kind\`" ;;
		esac
	done <<<"$RECORDS"

	if [ -n "$footnorule" ]; then
		printf 'label_consistency_check: %s: no closing `---` rule, so the document has no CLOSING REGION and its u-rev label is not locatable\n' "$doc" >&2
		BAD=$((BAD + 1))
	elif [ -n "$footbad" ]; then
		printf 'label_consistency_check: %s: the closing region below the rule at line %s names %s u-rev labels; exactly one is resolvable\n' "$doc" "$footline" "$footbad" >&2
		printf 'label_consistency_check: %s: the fold law (docs/decisions.md D-331) is the repair — the closing line carries the LABEL and points at its home for the rest\n' "$doc" >&2
		BAD=$((BAD + 1))
	elif [ "$head" = "NONE" ]; then
		printf 'label_consistency_check: %s: no `**u-rev <n>**` head label; the EXTRACTION is wrong, or the document lost its label\n' "$doc" >&2
		BAD=$((BAD + 1))
	elif [ "$head" != "$foot" ]; then
		printf 'label_consistency_check: %s: head u-rev %s (line %s) against foot u-rev %s (line %s)\n' "$doc" "$head" "$headline" "$foot" "$footline" >&2
		BAD=$((BAD + 1))
	else
		printf 'label_consistency_check: %-46s head=u-rev %-3s foot=u-rev %-3s OK\n' "$doc" "$head" "$foot"
	fi
done <<<"$DOCS"

# --- non-vacuity -------------------------------------------------------------
#
# A criterion the named defect cannot falsify is not a criterion (CLAUDE.md).
# Each of the three checks has live subjects in the tracked tree TODAY — six
# head/foot pairs, two summand lines, four group counts — and a run that finds
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
	fail "$BAD self-state disagreement(s) above; a document that misdescribes its own state is docs/decisions.md D-335's generator (2), and D-338 records this gate as the row that reaches it"

echo "label_consistency_check: $DOCS_SEEN documents, $SUM_COUNT summand line(s), $ROW_COUNT summand row(s), $GROUP_COUNT group count(s) — every document agrees with itself"
