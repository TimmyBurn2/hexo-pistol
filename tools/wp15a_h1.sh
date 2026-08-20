#!/usr/bin/env bash
#
# WP-1.5a's H1: does the threat generator change anything a shipped binary can
# observe? (docs/experiments/wp15a_prereg.md.)
#
# Usage: every binding below is read from the ENVIRONMENT and each is registered
#        in the pre-registration's §0.1. There are no defaults: a missing binding
#        is a named refusal, never a fallback (CLAUDE.md rule 1, rule 3).
#
# Exit:  0 CONFIRMED   H1-a and H1-b both hold
#        1 ABORT       a hypothesis is refuted — p is no longer 0, the solver
#                      reached codegen, or a behaviour line moved
#        2 RUN VOID    the instrument could not answer; not a verdict either way
#
# WHY THIS IS A FILE AND NOT 320 LINES INSIDE A PRE-REGISTRATION. It was those
# 320 lines through revision 10, and four consecutive review rounds each found a
# new BLOCKING defect in it — a trap that rewrote every verdict to 1, a guard
# that announced its own conclusion about an invocation that never answered, a
# `diff -u` whose mktemp paths voided a genuine ABORT, an adjudicator that read a
# dev-dependency as a refutation. Every one was found by an agent running the
# document by hand, because nothing else could run it. That is exactly
# tools/SHELL_CHECKLIST.md item 10's diagnosis and D-231's precedent, and the
# remedy is the one item 10 names: a script, in tools/, driven by a test in a
# suite CI runs, with a control.
#
# THE SNAPSHOT RUNS INSIDE THE PRISTINE CLONE, WHICH IS WHY THERE IS NO BLOB PIN
# HERE. Running it from the working tree meant its SCRIPT and its DATA INPUTS —
# configs, corpus, openings — were worktree files, and `git update-index
# --assume-unchanged` on any of them left `git status --porcelain` empty, the
# LANDING..HEAD diff empty, and a tampered config reaching H1-b as a VERDICT
# (exit 1) attributed to a defect class it does not belong to. Inside the clone
# every one of them is committed content at LANDING by construction, and the
# record's own `timing tree` token becomes a measurement instead of a constant.
# RULE9-JUSTIFICATION: one hypothesis, whose steps are ordered and interdependent.
# Every guard here exists so that the step after it can be attributed, and the
# ordering IS the content: the tree must be clean before the revisions mean
# anything, the revisions before the clone, the clone before the edge check, and
# the edge check before a build is worth paying for. Splitting the file would put
# a guard in one place and the claim it protects in another, which is the shape
# that let four review rounds each find a defect the previous round's fix had
# introduced. The one piece that IS separable — the p = 0 adjudicator — is
# separate, in tools/solver_edge_check.sh, because it is the piece a test needs
# to drive against workspaces of its own.
set -euo pipefail

refuse() { echo "wp15a_h1: $*" >&2; exit 2; }

# Object-name SPELLING, not just value (SHELL_CHECKLIST item 8): a branch name
# satisfies every git call below while a registered revision is one that does not
# move, and the attestation would then record `dev:` where a revision belongs.
is_hex() { case "$2" in '' | *[!0-9a-f]*) return 1 ;; esac; [ "${#2}" -eq "$1" ]; }
# An ALLOW-LIST, so pinning the locale makes this refuse MORE and never less
# (item 4). A newline in a path INJECTS LINES into the record it is printed into
# (item 9). LC_ALL is pinned for the test itself so its width does not depend on
# the caller's environment.
printable() { local v="$1"; case "$(LC_ALL=C printf '%s' "$v")" in *[![:print:]]*) return 1 ;; esac; }

need() { local name="$1" value="$2"; [ -n "$value" ] || refuse "$name unset"; }

BASE=${BASE:-};                 need BASE "$BASE"
BASE_SHA=${BASE_SHA:-};         need BASE_SHA "$BASE_SHA"
BASE_REV=${BASE_REV:-};         need BASE_REV "$BASE_REV"
BASE_TC=${BASE_TC:-};           need BASE_TC "$BASE_TC"
BASE_TC_SHA=${BASE_TC_SHA:-};   need BASE_TC_SHA "$BASE_TC_SHA"
LANDING=${LANDING:-};           need LANDING "$LANDING"
SUBJECT_CRATE=${SUBJECT_CRATE:-}; need SUBJECT_CRATE "$SUBJECT_CRATE"
SUBJECT_PATH=${SUBJECT_PATH:-};   need SUBJECT_PATH "$SUBJECT_PATH"
BUILD_PKG=${BUILD_PKG:-};         need BUILD_PKG "$BUILD_PKG"
BUILD_BIN=${BUILD_BIN:-};         need BUILD_BIN "$BUILD_BIN"
SNAPSHOT_REL=${SNAPSHOT_REL:-};   need SNAPSHOT_REL "$SNAPSHOT_REL"
EDGE_CHECK=${EDGE_CHECK:-};       need EDGE_CHECK "$EDGE_CHECK"

is_hex 40 "$BASE_REV" || refuse "BASE_REV is not a 40-hex object name: $BASE_REV"
is_hex 40 "$LANDING" || refuse "LANDING is not a 40-hex object name: $LANDING — a branch or tag \
name satisfies every git call below while a registered revision is one that does not move"
is_hex 64 "$BASE_SHA" || refuse "BASE_SHA is not a 64-hex digest: $BASE_SHA"
is_hex 64 "$BASE_TC_SHA" || refuse "BASE_TC_SHA is not a 64-hex digest: $BASE_TC_SHA"

REPO=${REPO:-}
if [ -z "$REPO" ]; then
	REPO="$(git rev-parse --show-toplevel 2>/dev/null)" || refuse "not inside a git repository"
fi
[ -n "$REPO" ] || refuse "cannot resolve the repository root"
for binding in REPO BASE BASE_TC SUBJECT_CRATE SUBJECT_PATH BUILD_PKG BUILD_BIN SNAPSHOT_REL EDGE_CHECK; do
	printable "${!binding}" || refuse "$binding holds a non-printable character and would inject \
lines into the record it is printed into"
done
# AND ENTER THE REPOSITORY. Every `git ls-files -- <pathspec>` below resolves its
# pathspec RELATIVE TO THE CURRENT DIRECTORY, so the same command run from a
# subdirectory asks a different question and answers "nothing untracked" with a
# stray file sitting on a build-reaching path — EXIT-0-WRONG-ANSWER selected by
# working directory (item 5).
cd "$REPO" || refuse "cannot enter the repository root $REPO"
# SNAPSHOT_REL is a path RELATIVE TO A REPOSITORY ROOT, and it is resolved
# against the PRISTINE CLONE rather than the working tree. `baseline_snapshot.sh`
# roots itself at `dirname($0)/..` and `cd`s there, so handing it the working
# tree's copy would make it read the working tree's configs and corpus no matter
# where it was invoked from — which is the whole gap this arrangement closes.
case "$SNAPSHOT_REL" in
/* | *..*) refuse "SNAPSHOT_REL must be a path inside the repository, not $SNAPSHOT_REL" ;;
esac
[ -x "$REPO/$SNAPSHOT_REL" ] || refuse "the snapshot instrument is not executable: $REPO/$SNAPSHOT_REL"
[ -x "$EDGE_CHECK" ] || refuse "the edge-check instrument is not executable: $EDGE_CHECK"

for binding in REPO BASE BASE_SHA BASE_REV BASE_TC BASE_TC_SHA LANDING \
	SUBJECT_CRATE SUBJECT_PATH BUILD_PKG BUILD_BIN SNAPSHOT_REL EDGE_CHECK; do
	printf 'wp15a_h1: binding %-14s %s\n' "$binding" "${!binding}"
done

WORK="$(mktemp -d -t wp15a_h1_work.XXXXXX)" || refuse "cannot create the work directory"
PRISTINE="$(mktemp -d -t wp15a_h1_clone.XXXXXX)" || refuse "cannot create the clone directory"
# item 7, and this one CHANGES VERDICTS: an EXIT trap's last command decides the
# script's status, so a bare `rm -rf` here returned 1 for a requested 0, 1 AND 2 —
# collapsing the three-way disjointness by housekeeping. Take rc first, make the
# removal unable to fail the trap, hand rc back.
cleanup() { local rc=$?; rm -rf -- "$WORK" "$PRISTINE" 2>/dev/null || true; return "$rc"; }
trap cleanup EXIT

# ---- The baseline record and its sidecar. `[ -s ]` STATS and `sha256sum -c`
#      READS, so an unreadable record reached the digest comparison and was
#      reported as "does not match" when it had never been read (item 8). ----
[ -s "$BASE" ] || refuse "baseline record missing or empty at $BASE"
[ -r "$BASE" ] || refuse "baseline record at $BASE is not readable"
printf '%s  %s\n' "$BASE_SHA" "$BASE" | sha256sum -c - >/dev/null \
	|| refuse "baseline record does not match its registered digest"
BASE_KIND="$(sed -n '1p' -- "$BASE")" || refuse "cannot read the baseline record's kind token"
[ "$BASE_KIND" = 'baseline_snapshot 1' ] \
	|| refuse "the baseline record is not a COMPLETE baseline_snapshot: $BASE_KIND"
[ "$(sed -n 's/^revision //p' -- "$BASE")" = "$BASE_REV" ] \
	|| refuse "baseline record is not at the registered baseline revision"
[ -s "$BASE_TC" ] || refuse "baseline toolchain sidecar missing at $BASE_TC"
[ -r "$BASE_TC" ] || refuse "baseline toolchain sidecar at $BASE_TC is not readable"
printf '%s  %s\n' "$BASE_TC_SHA" "$BASE_TC" | sha256sum -c - >/dev/null \
	|| refuse "baseline toolchain sidecar does not match its registered digest"

# ---- The candidate tree, clean, diagnosed in three arms because the refusal has
#      three reasons and one combined test gives one wrong answer (item 8). ----
DIRT="$(git status --porcelain)" || refuse "cannot read the working tree status"
if [ -n "$DIRT" ]; then
	TRACKED="$(git status --porcelain --untracked-files=no)" || refuse "cannot read the tracked-file status"
	[ -z "$TRACKED" ] || refuse "tracked files are modified: $TRACKED"
	STRAY="$(git ls-files --others --exclude-standard -- Cargo.toml Cargo.lock crates configs tools)" \
		|| refuse "cannot enumerate untracked files"
	[ -z "$STRAY" ] || refuse "untracked files on build-reaching paths: $STRAY"
	refuse "untracked files outside the build-reaching set: $DIRT"
fi

# ---- Revision assertions. HEAD need not EQUAL LANDING — docs commits land above
#      a work package and cannot reach a binary — but nothing BUILD-REACHING may
#      have moved between them. `tools` is deliberately NOT in this pathspec: the
#      instruments are named with their revisions instead (D-268), because a
#      drift assertion cannot both forbid new instruments and permit the one the
#      current revision adds, and the snapshot runs from the clone where it is
#      LANDING's by construction. ----
LAND_OBJ="$(git rev-parse --verify --quiet "$LANDING^{commit}")" \
	|| refuse "LANDING $LANDING is not a commit in this repository"
[ "$LAND_OBJ" = "$LANDING" ] || refuse "LANDING $LANDING resolves to $LAND_OBJ"
BASE_OBJ="$(git rev-parse --verify --quiet "$BASE_REV^{commit}")" \
	|| refuse "BASE_REV $BASE_REV is not a commit in this repository"
[ "$BASE_OBJ" = "$BASE_REV" ] || refuse "BASE_REV $BASE_REV resolves to $BASE_OBJ"
[ "$LANDING" != "$BASE_REV" ] || refuse "candidate and baseline are the same revision"
anc=0
git merge-base --is-ancestor "$LANDING" HEAD || anc=$?
case "$anc" in
0) ;;
1) refuse "LANDING $LANDING is not an ancestor of HEAD — this tree is not a continuation of the landing revision" ;;
*) refuse "git merge-base could not decide whether $LANDING is an ancestor of HEAD (status $anc)" ;;
esac
DRIFT="$(git diff --name-only "$LANDING" HEAD -- Cargo.toml Cargo.lock crates configs)" \
	|| refuse "cannot diff $LANDING against HEAD"
[ -z "$DRIFT" ] || refuse "build-reaching paths moved between LANDING and HEAD: $DRIFT"
# THE SUBJECT DIFF GUARD. With p = 0 an empty diff under the subject's path is
# also what "the work package never landed" looks like, so an empty one is a
# refusal and not a pass. No pipeline: `git diff --stat … | grep -q .` routed
# grep's no-match, git's bad-revision 128 and a SIGPIPE'd producer into ONE
# refusal naming only the first, and a nonexistent revision was adjudicated as
# this guard's own conclusion (items 1, 3 and 8).
SUBJECT_DIFF="$(git diff --name-only "$BASE_REV" "$LANDING" -- "$SUBJECT_PATH")" \
	|| refuse "cannot diff $SUBJECT_PATH between $BASE_REV and $LANDING — the guard did not \
answer, which is not the same as answering that the diff is empty"
[ -n "$SUBJECT_DIFF" ] || refuse "$LANDING changes nothing under $SUBJECT_PATH: with p = 0 an \
empty diff is also what 'the work package never landed' looks like"

# ---- The pristine clone, and the baseline's cleanliness attested by REBUILD
#      rather than by a token the record carries about itself. ----
BASE_DIGEST_RECORDED="$(sed -n 's/^binary_sha256 //p' -- "$BASE")"
[ -n "$BASE_DIGEST_RECORDED" ] || refuse "baseline record carries no binary_sha256"
git clone --quiet --no-hardlinks "$REPO" "$PRISTINE/repo" || refuse "cannot clone for the rebuild attestation"
( cd "$PRISTINE/repo" && git checkout --quiet "$BASE_REV" ) || refuse "baseline revision not in the clone"
CLONE_DIRT="$(cd "$PRISTINE/repo" && git status --porcelain)" || refuse "cannot read the clone's status"
[ -z "$CLONE_DIRT" ] || refuse "the pristine clone is not pristine: $CLONE_DIRT"
BASE_BIN="$(cd "$PRISTINE/repo" && cargo build --release --locked -p "$BUILD_PKG" --bin "$BUILD_BIN" \
	--message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
	|| refuse "baseline rebuild failed"
[ -s "$BASE_BIN" ] || refuse "baseline rebuild produced no binary"
BASE_BIN_DIGEST="$(sha256sum -- "$BASE_BIN" | cut -d' ' -f1)" || refuse "cannot digest the baseline rebuild"
[ "$BASE_BIN_DIGEST" = "$BASE_DIGEST_RECORDED" ] \
	|| refuse "baseline binary_sha256 does not reproduce from a pristine checkout of $BASE_REV — \
something uncommitted reached the baseline binary"
echo "wp15a_h1: baseline rebuild attests $BASE_DIGEST_RECORDED"

# ---- p = 0, on the RESOLVED GRAPH, workspace-wide, by a tested instrument that
#      prints its own answer (tools/solver_edge_check.sh). ----
( cd "$PRISTINE/repo" && git checkout --quiet "$LANDING" ) || refuse "landing revision not in the clone"
# BOTH READINGS ARE TAKEN BEFORE EITHER IS ACTED ON. Ordering the edge check to
# exit on refutation made the registered agreement criterion UNEVALUABLE in the
# one branch that matters: whenever instrument 2 said "edge", H1-a never
# reported, so two of the biconditional's four corners had no producer and could
# not have one — and the case where the two instruments disagree was exactly the
# case the run decided on one of them alone. The refutation is REMEMBERED here
# and adjudicated after H1-a, which costs one build on a run that is going to
# abort anyway and buys a transcript that carries both readings.
edge=0
"$EDGE_CHECK" "$PRISTINE/repo" "$SUBJECT_CRATE" || edge=$?
case "$edge" in
0) echo "wp15a_h1: p = 0 — no normal reverse-dependency on $SUBJECT_CRATE at $LANDING" ;;
1) echo "wp15a_h1: p = 0 REFUTED — the lines above name the dependents; H1-a is taken anyway, \
so both instruments' readings reach the record" ;;
*) refuse "the edge check could not answer at $LANDING (status $edge)" ;;
esac

# ---- H1-a: build LANDING, then build it with the subject reverted to BASE_REV. ----
CAND_BIN="$(cd "$PRISTINE/repo" && cargo build --release --locked -p "$BUILD_PKG" --bin "$BUILD_BIN" \
	--message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
	|| refuse "candidate build failed"
[ -s "$CAND_BIN" ] || refuse "candidate build produced no binary"
D_WITH="$(sha256sum -- "$CAND_BIN" | cut -d' ' -f1)" || refuse "cannot digest the candidate build"
cp -- "$CAND_BIN" "$WORK/with_subject" || refuse "cannot preserve the candidate binary"
# `rm -rf` FIRST: `git checkout BASE_REV -- <path>` merges the old files in and
# leaves every file the work package added on disk, and cargo auto-discovers
# `build.rs`, `src/bin/`, `benches/`, `examples/` and `tests/` by convention.
# What this produces is the subject at its BASELINE CONTENT, not its absence.
( cd "$PRISTINE/repo" && rm -rf -- "$SUBJECT_PATH" \
	&& git checkout --quiet "$BASE_REV" -- "$SUBJECT_PATH" ) \
	|| refuse "cannot restore $SUBJECT_PATH to its baseline content"
# `--locked` is dropped here and ONLY here: dropping a dependency edge moves
# Cargo.lock, so `--locked` would refuse. The guard it gives up is replaced by
# the two assertions below — which revision 10's comment claimed and did not have.
COUNTER_BIN="$(cd "$PRISTINE/repo" && cargo build --release -p "$BUILD_PKG" --bin "$BUILD_BIN" \
	--message-format=json-render-diagnostics | sed -n 's/.*"executable":"\([^"]*\)".*/\1/p' | tail -1)" \
	|| refuse "counterfactual build failed"
[ -s "$COUNTER_BIN" ] || refuse "counterfactual build produced no binary"
D_WITHOUT="$(sha256sum -- "$COUNTER_BIN" | cut -d' ' -f1)" || refuse "cannot digest the counterfactual build"
LOCK_DELTA="$(cd "$PRISTINE/repo" && git diff --shortstat -- Cargo.lock)" \
	|| refuse "cannot read the counterfactual's Cargo.lock delta"
# The SHAPE, not a string: the delta is pair-dependent. Reverting a subject that
# the baseline already carried moves nothing; reverting one the baseline did not
# have removes its edge. Either way cargo may only DELETE — an insertion means it
# re-resolved something else, which is the guard `--locked` gave up. `[1-9]`
# rather than the substring `insertion`, because ` 0 insertions(+)` contains it.
case "$LOCK_DELTA" in
'') ;;
*[1-9]" insertion"*) refuse "the counterfactual ADDED lines to Cargo.lock ($LOCK_DELTA): dropping \
an edge can only delete, so cargo re-resolved something else" ;;
*deletion*) ;;
*) refuse "unregistered Cargo.lock delta shape from the counterfactual: $LOCK_DELTA" ;;
esac
COUNTER_DIRT="$(cd "$PRISTINE/repo" && git status --porcelain -- ":(exclude)$SUBJECT_PATH" ':(exclude)Cargo.lock')" \
	|| refuse "cannot read the counterfactual tree's status"
[ -z "$COUNTER_DIRT" ] || refuse "the counterfactual moved files outside $SUBJECT_PATH and Cargo.lock: $COUNTER_DIRT"
echo "wp15a_h1: H1-a with subject    $D_WITH"
echo "wp15a_h1: H1-a without subject $D_WITHOUT"
echo "wp15a_h1: H1-a counterfactual lock delta:${LOCK_DELTA:- none}"
h1a=identical
[ "$D_WITH" = "$D_WITHOUT" ] || h1a=differs
echo "wp15a_h1: H1-a reading $h1a"
# THE REGISTERED AGREEMENT CRITERION, AND IT IS ONE-DIRECTIONAL — a biconditional
# here would be unsound, which this script's own test suite demonstrated before
# any reviewer saw it. THE TWO INSTRUMENTS DO NOT ASK THE SAME QUESTION: the graph
# asks whether the subject is LINKED; H1-a asks whether the subject's CONTENT
# CHANGE between BASE_REV and LANDING reaches codegen. A crate can be linked while
# a change inside it does not reach the binary — which is exactly the dead-code
# insensitivity the pre-registration already records as H1-a's instrument caveat,
# and a synthetic fixture with an unused edge reproduces it: `edge = 1` with two
# bit-identical binaries, which a biconditional would have called a disagreement
# and voided. So:
#
#   edge present  -> p != 0. ABORT. H1-a's reading is RECORDED and adjudicates
#                    nothing, because a linked crate whose diff happens to be dead
#                    is still a linked crate.
#   edge absent   -> H1-a MUST be identical. A crate outside the resolved graph
#                    cannot reach codegen, so a difference means the two
#                    instruments contradict each other: RUN VOID, never a verdict.
#
# The second line is the criterion that carries evidential weight, and it is the
# direction in which the instruments genuinely constrain one another.
if [ "$edge" -eq 1 ]; then
	echo "wp15a_h1: p != 0 — ABORT. H1-a read $h1a and adjudicates nothing here: a linked crate \
whose diff is dead code is still linked" >&2
	exit 1
fi
if [ "$h1a" = differs ]; then
	echo "wp15a_h1: the two instruments CONTRADICT each other — no dependent in the resolved \
graph, and the binary moved anyway. A crate outside the graph cannot reach codegen. No verdict \
is taken; both readings are above" >&2
	exit 2
fi
echo "wp15a_h1: H1-a CONFIRMED — $SUBJECT_CRATE contributes nothing to the shipped binary"

# ---- Toolchain, PRINTED into the transcript rather than written to a file the
#      EXIT trap deletes. `rustc -vV | head -1` takes SIGPIPE under pipefail —
#      capture whole, then slice. ----
RUSTC_VV="$(rustc -vV)" || refuse "cannot read rustc -vV"
RUSTC_LINE="$(printf '%s\n' "$RUSTC_VV" | sed -n '1p')" || refuse "cannot slice rustc -vV"
[ -n "$RUSTC_LINE" ] || refuse "rustc -vV produced no first line"
CARGO_LINE="$(cargo --version)" || refuse "cannot read cargo --version"
echo "wp15a_h1: toolchain candidate $RUSTC_LINE"
echo "wp15a_h1: toolchain candidate $CARGO_LINE"
if grep -Fxq -- "$RUSTC_LINE" "$BASE_TC" && grep -Fxq -- "$CARGO_LINE" "$BASE_TC"; then TC=yes; else TC=no; fi
echo "wp15a_h1: toolchain matches baseline: $TC"

# ---- H1-b: the invariant blocks, excluding the two lines a linked-crate change
#      is guaranteed to move. The snapshot runs INSIDE THE CLONE, so its script,
#      its config and its corpus are committed content at LANDING. ----
( cd "$PRISTINE/repo" && git checkout --quiet -- "$SUBJECT_PATH" Cargo.lock ) \
	|| refuse "cannot restore the clone to $LANDING after the counterfactual"
[ -x "$PRISTINE/repo/$SNAPSHOT_REL" ] \
	|| refuse "the clone at $LANDING carries no executable $SNAPSHOT_REL"
( cd "$PRISTINE/repo" && "./$SNAPSHOT_REL" --binary "$WORK/with_subject" --out "$WORK/cand.snapshot" ) \
	|| refuse "the candidate snapshot could not be taken"
inv() {
	local f="$1" n kind
	[ -s "$f" ] || { echo "wp15a_h1: snapshot missing or empty at $f" >&2; return 2; }
	kind="$(sed -n '1p' -- "$f")" || { echo "wp15a_h1: cannot read $f" >&2; return 2; }
	[ "$kind" = 'baseline_snapshot 1' ] \
		|| { echo "wp15a_h1: $f is not a COMPLETE baseline_snapshot record: $kind" >&2; return 2; }
	grep -q '^# timing' -- "$f" || { echo "wp15a_h1: no '# timing' marker in $f" >&2; return 2; }
	# `grep -c` prints 0 AND exits 1 (item 3), so `|| true` is load bearing; and
	# because it would equally mask a failing `sed`, the SPELLING is validated.
	n="$(sed -n '1,/^# timing/p' -- "$f" | grep -c . || true)"
	case "$n" in '' | *[!0-9]*) echo "wp15a_h1: could not count the invariant block of $f" >&2; return 2 ;; esac
	[ "$n" -ge 50 ] || { echo "wp15a_h1: invariant block short ($n) in $f" >&2; return 2; }
	sed -n '1,/^# timing/p' -- "$f" | sed '/^# timing/d' | grep -v '^revision \|^binary_sha256 ' || true
}
inv "$BASE" >"$WORK/inv.base" || refuse "baseline record failed its shape checks"
inv "$WORK/cand.snapshot" >"$WORK/inv.cand" || refuse "candidate record failed its shape checks"
INV_LINES="$(wc -l <"$WORK/inv.base")" || refuse "cannot count the baseline behaviour lines"
case "$INV_LINES" in '' | *[!0-9]*) refuse "the behaviour-line count is not a number: $INV_LINES" ;; esac
[ "$INV_LINES" -ge 50 ] || refuse "only $INV_LINES behaviour lines survive the exclusions; the comparison would be vacuous"
echo "wp15a_h1: H1-b comparing $INV_LINES behaviour lines (revision and binary_sha256 excluded)"
OTHER="$(git diff --name-only "$BASE_REV" "$LANDING" -- Cargo.toml Cargo.lock crates configs \
	":(exclude)$SUBJECT_PATH" | LC_ALL=C sort)" || refuse "cannot enumerate the other moved paths"
[ -z "$OTHER" ] || {
	echo "wp15a_h1: note — build-reaching paths outside $SUBJECT_PATH also moved:"
	echo "$OTHER" | sed 's/^/wp15a_h1:   /'
}
# `diff -u` writes both file PATHS and their mtimes to STDOUT, and $WORK is a
# mktemp name — so on the ABORT path three replications produced three distinct
# stdouts while §6 voids a run whose replications disagree, converting a genuine
# ABORT into a voided run. `--label` fixes it; `cmp` gives the verdict a NAMED
# line, which a bare `set -e` on `diff` did not (rule 3).
if cmp -s -- "$WORK/inv.base" "$WORK/inv.cand"; then
	echo "wp15a_h1: H1-b CONFIRMED — every behaviour line is byte-identical"
else
	diff -u --label baseline --label candidate -- "$WORK/inv.base" "$WORK/inv.cand" || true
	echo "wp15a_h1: H1-b FAILED — a behaviour line moved between $BASE_REV and $LANDING" >&2
	exit 1
fi
