#!/usr/bin/env bash
#
# Every CI gate, runnable locally. CI runs exactly this script; there is no
# second, truer definition of the gates living somewhere else.
#
# Usage: tools/ci.sh
# Exit:  0 all gates pass
#        1 a gate failed
#        2 THE RUN IS VOID — no gate was adjudicated (tools/SHELL_CHECKLIST.md
#          item 12). A void is not a failure and must not be read as one.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

step() { printf '\n=== %s\n' "$*"; }
fail() { printf 'ci: FAIL: %s\n' "$*" >&2; exit 1; }
# THE VOID, CARRIED ACROSS THE SEAM (tools/SHELL_CHECKLIST.md item 12
# obligation 3). `fail` exits 1, so wrapping every gate as `gate.sh || fail ...`
# spells a gate's exit 2 as a FAILURE — the distinction the gates were given
# dies here, one call away from the gates that make it. `gate` preserves it:
# 0 passes, 2 is this run's void, anything else is the gate's own no.
void() { printf 'ci: RUN VOID: %s\n' "$*" >&2; exit 2; }
gate() {
	local what="$1"
	shift
	local rc=0
	"$@" || rc=$?
	case "$rc" in
	0) ;;
	2) void "$what took no answer; the lines above are the gate's own, and this is NOT a regression" ;;
	*) fail "$what" ;;
	esac
}

command -v cargo >/dev/null || fail "cargo is not on PATH"
command -v git >/dev/null || fail "git is not on PATH"
git rev-parse --is-inside-work-tree >/dev/null 2>&1 ||
	fail "not a git repository: one of the gates builds the git-tracked file set"

# SCRATCH SPACE, ASKED FOR BEFORE ANYTHING IS BUILT (tools/SHELL_CHECKLIST.md
# item 12, docs/decisions.md D-285). Gate 2 unpacks the whole tracked file set
# into a temporary directory and builds it — measured at 340552 KiB — and gate
# 3's suites build stub workspaces under the same `$TMPDIR`. A full `$TMPDIR`
# makes `cargo` answer in ITS OWN vocabulary (`Disk quota exceeded`), which
# reads downstream as a subject regression; on this machine `/tmp` is RAM-backed
# and has been filled by a single session. This asks in the gate's vocabulary,
# and a shortage is a VOID rather than a failure.
# BOTH FILESYSTEMS, because they are two. `$TMPDIR` holds gate 2's clone and
# gate 3's stub workspaces; gates 3, 4 and 8-12 build into `$ROOT/target`, and on
# this machine those are different devices — a 24 GiB tmpfs and an nvme
# partition. Preflighting only the first leaves the LARGER consumer unasked, and
# a shortage there still reaches the log in cargo's vocabulary through `fail`,
# which is the reading this block exists to prevent.
for SCRATCH in "${TMPDIR:-/tmp}" "$ROOT"; do
	tools/scratch_preflight.sh "$SCRATCH" ||
		void "no gate was adjudicated; the lines above name the filesystem"
done

# First because it is instant and needs no build: fastest possible feedback
# (docs/decisions.md D-30).
step "gate 1/19: cargo fmt --all --check"
cargo fmt --all --check || fail "formatting: run \`cargo fmt --all\`"

step "gate 2/19: build from the git-tracked file set"
# The point of this gate is to catch a build that depends on a file nobody
# tracked. The tracked set is the git index: it equals HEAD on a fresh checkout,
# and equals the about-to-be-committed tree when work is staged, so the gate
# gives the same answer before and after a commit (docs/decisions.md D-26).
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/repo"
git checkout-index --all --prefix="$WORK/repo/"
echo "ci: building $(git ls-files | wc -l) tracked files in $WORK/repo"
(cd "$WORK/repo" && cargo build --workspace --locked --quiet) ||
	fail "the tracked file set does not build on its own: something the build needs is untracked"
rm -rf "$WORK"
trap - EXIT

step "gate 3/19: cargo test --workspace --locked"
cargo test --workspace --locked || fail "tests"

# --all-targets so tests and examples are linted too, which is strictly more
# than CLAUDE.md asks for and costs nothing.
step "gate 4/19: cargo clippy --workspace --all-targets -- -D clippy::all"
cargo clippy --workspace --all-targets --locked -- -D clippy::all || fail "clippy"

step "gate 5/19: artifact rejection"
gate "artifact check" tools/artifact_check.sh

step "gate 6/19: config validation"
gate "config check" tools/config_check.sh

step "gate 7/19: perft oracle"
gate "perft oracle" tools/perft_check.sh

# The determinism law's executable form (CLAUDE.md rule 4, docs/decisions.md D-7).
# It runs last of the two engine gates because it is the slowest: two processes
# over the whole sha-pinned fixture set at two budgets.
step "gate 8/19: tactical fixture at its pre-registered threshold"
gate "tactical fixture" tools/tactical_check.sh

step "gate 9/19: cross-process determinism"
gate "determinism" tools/determinism.sh

# The search's oracle, and the last of the correctness gates because it is the
# longest: a full-width reference pays the candidate count squared per turn, so
# the third turn it certifies is minutes of release CPU. It runs after the
# determinism gate for the same reason that one runs after the tactical gate —
# the cheapest thing that can fail should fail first (docs/decisions.md D-106,
# D-120).
step "gate 10/19: differential search oracle"
gate "search oracle" tools/search_oracle_check.sh

# THE STAGED GENERATOR'S OWN SOUNDNESS GATE (docs/decisions.md D-316, D-361):
# the four parts THE TACTICAL SUITE UNDER STAGED, THE DIFFERENTIAL GATE (S-M),
# THE COLONY FAMILY, and THE PATTERN FIXTURES UNDER STAGED, wired as one script
# per docs/wp15b_impl_prompt.md §3.3. It runs immediately after the radius
# generator's own oracle for the same reason that one runs after the tactical
# gate: this is the staged generator's analogous oracle, and the cheapest thing
# that can fail should fail first, ahead of the play-mode ceiling below (which
# reuses this gate's release build).
step "gate 11/19: staged generator soundness (four parts)"
gate "staged soundness" tools/staged_soundness_check.sh

# The play-mode ceiling (WP-1.4, superseding docs/decisions.md D-95): release
# `cargo test` over the movetime and fallback suites, then the real binary over
# the sha-pinned spread fixture with every measured overshoot checked against
# N + play.movetime_epsilon_ms. After the instrument gates because it reuses
# their release build.
# THE SOLVER'S OWN ORACLE GATES (WP-1.8a, docs/experiments/wp18a_design.md §7):
# the four oracles — differential against the R3' brute-force reference,
# full-width proof-tree re-verification, the relevance-zone tolerance, and
# the TT cross-check — over the sha-pinned solver fixture, in release. They
# run after the search's own oracle for the same reason that one runs late:
# the cheapest thing that can fail should fail first, and gate (c)'s sigma
# sweep is the longest correctness check in the suite.
step "gate 12/19: solver oracle (four gates)"
gate "solver oracle" tools/solver_oracle_check.sh

# D-7's law gains its solver seat (WP-1.8a design §7): the selftest binary,
# two processes, byte-identical transcripts — value, node count, seesaw and
# proof digest all included, nothing exempt.
step "gate 13/19: solver determinism"
gate "solver determinism" tools/solver_determinism.sh

step "gate 14/19: movetime ceiling on the D-95 reproducer class"
gate "movetime ceiling" tools/movetime_check.sh

# The judge itself. It runs after the engine gates because it USES the engine:
# a smoke run that failed because the engine was broken would be a confusing
# place to find that out. It is a self-match, so its expected answer is knowable
# in advance and is asserted exactly, and it repeats the run to cover the
# arena's own determinism — which nothing else in this suite does
# (docs/decisions.md D-169).
step "gate 15/19: arena self-match smoke"
gate "arena smoke" tools/arena_smoke.sh

# THE SEALBOT ANCHOR PLATFORM'S OWN SUITE (REVIEW finding F5, SHELL_CHECKLIST
# item 10's binding letter): the local match server produces recorded numbers
# — anchor reports and transcripts — so a test in THIS suite drives the
# SHIPPED run_match.sh end to end with scripted stub engines, asserts
# hand-derived outcomes (a first-stone win, an illegal-move forfeit, a
# zero-stone answer, an over-submission), and runs the replay checker over
# every record plus three tampered-record negative controls. It sits after
# the arena smoke because it is the same kind of gate — a match harness
# driven end to end — and before the read-only gates, which stay last by
# their own stated rule.
step "gate 16/19: sealbot anchor platform suite"
gate "sealbot suite" tools/sealbot/tests/run_tests.sh

# CLAUDE.md rule 9's soft cap. Last because it is the only gate that reads the
# tracked files rather than building them, so it costs nothing to put it where a
# reader looks for the summary (docs/decisions.md D-131).
step "gate 17/19: file-justification check"
gate "file justification" tools/file_justification_check.sh

# The decision log's own integrity, and the last gate for the same reason the
# one above it is: it reads tracked bytes rather than building anything. `D-276`
# and `D-277` were each appended TWICE with different text and nothing detected
# it (docs/decisions.md D-279, D-284), and every ADR reference in this repository
# is by number.
step "gate 18/19: decision-key uniqueness"
gate "decision key check" tools/decision_key_check.sh

# The carve documents' own self-state, and the last gate for the reason the two
# above it are last: it reads tracked bytes rather than building anything. A
# one-line shell loop found two live head/foot disagreements in the six frozen
# WP-1.5b documents that ten review rounds had not recorded, in a class the
# matrix under attack asserted only a fresh reviewer's hand inventory could
# reach — and the loop MISSED A THIRD, because it read a fixed-depth tail
# (docs/decisions.md D-338 row R4, matrix_META1_REDTEAM.md M2 and M3).
step "gate 19/19: carve-document label consistency"
gate "label consistency" tools/label_consistency_check.sh

printf '\nci: all gates passed\n'
