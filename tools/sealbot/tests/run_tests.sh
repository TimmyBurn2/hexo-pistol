#!/usr/bin/env bash
#
# The matchserver test suite (tools/SHELL_CHECKLIST.md item 10, the coverage
# rule): this harness produces recorded numbers, so a test drives the SHIPPED
# run_match.sh — not an inner function — with scripted stub engines, and
# asserts POSITIVE facts hand-derived from the scripts (a refusing-everything
# run produces no games and cannot pass).
#
# Two matches:
#   m1  a scripted win: p1 builds (0,0)..(0,4), then wins at turn 7 with a
#       ONE-STONE turn — exercising rule 4's truncation, the win path, the
#       per-turn transcript, and the node accounting (3 answers x 7 nodes).
#   m2  a scripted forfeit: p2's first stone is far outside the radius-8
#       region, so the referee forfeits it and a wins without answering once.
# Plus the overwrite refusal: re-running m1's config must exit 2 by name.
#
# Item 11: the only deletion is the sweep of $SCRATCH, a fixed test-owned
# prefix under artifacts/ no other producer writes.
#
# Usage: tools/sealbot/tests/run_tests.sh
# Exit:  0 every assertion held; 1 one did not; 2 the harness was refused.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT"

fail() { printf 'sealbot-tests: FAIL: %s\n' "$*" >&2; exit 1; }
refuse() { printf 'sealbot-tests: REFUSED: %s\n' "$*" >&2; exit 2; }

SCRATCH="artifacts/pistol-testscratch-matchserver"
MS_DIR=tools/sealbot/matchserver
# Item 6/11: sweep a prefix this test owns, fixed and repo-relative.
rm -rf -- "$SCRATCH"
mkdir -p "$SCRATCH"

# --- the scripts and configs -------------------------------------------------
# Game 1: server plays (0,0) p1 t1; p2 answers at r=1 flanks; p1 builds q=0.
python3 - "$SCRATCH" <<'PY'
import json, pathlib, sys
scratch = pathlib.Path(sys.argv[1])

# m1: p1 (engine_a, stub_pistol) turn tokens for turns 3, 5, 7.
scratch.joinpath("m1_pistol.json").write_text(json.dumps([
    "0,1/0,2", "0,3/0,4", "0,5",
]))
# m1: p2 (engine_b, stub_sealbot) stones for turns 2, 4, 6.
scratch.joinpath("m1_sealbot.json").write_text(json.dumps([
    [[1, 1], [-1, 1]], [[2, 1], [-2, 1]], [[3, 1], [-3, 1]],
]))
# m2: p2's first answer is far outside the legal region -> illegal move.
scratch.joinpath("m2_sealbot.json").write_text(json.dumps([
    [[50, 50], [51, 51]],
]))
# m2: p1 (engine_a) is never asked; the script is empty.
scratch.joinpath("m2_pistol.json").write_text(json.dumps([]))

def config(name, sealbot_script):
    return f'''schema_version = 1
games = 1
turn_cap = 20
output_dir = "artifacts/pistol-testscratch-matchserver/{name}"

[engine_a]
kind = "pistol"
label = "stub-pistol"
command = ["python3", "tools/sealbot/tests/stub_pistol.py", "{scratch}/{name}_pistol.json"]
cwd = "."
nodes = 100
turn_timeout_seconds = 10.0

[engine_b]
kind = "sealbot"
label = "stub-sealbot"
command = ["python3", "tools/sealbot/tests/stub_sealbot.py", "{scratch}/{sealbot_script}"]
cwd = "."
time_limit_seconds = 0.05
turn_timeout_seconds = 10.0
'''

scratch.joinpath("m1.toml").write_text(config("m1", "m1_sealbot.json"))
scratch.joinpath("m2.toml").write_text(config("m2", "m2_sealbot.json"))
PY
[ -s "$SCRATCH/m1.toml" ] || refuse "the config generator wrote nothing"

# --- run the shipped script twice ---------------------------------------------
tools/sealbot/run_match.sh "$SCRATCH/m1.toml" || fail "run_match.sh refused or failed on m1"
tools/sealbot/run_match.sh "$SCRATCH/m2.toml" || fail "run_match.sh refused or failed on m2"

# --- the overwrite refusal (item 12: a refusal is exit 2, by name) -----------
set +e
REFUSAL_OUTPUT="$(tools/sealbot/run_match.sh "$SCRATCH/m1.toml" 2>&1)"
REFUSAL_RC=$?
set -e
[ "$REFUSAL_RC" -eq 2 ] || fail "re-running m1 exited $REFUSAL_RC, expected 2 (refusal)"
case "$REFUSAL_OUTPUT" in
  *"already holds a report"*) : ;;
  *) fail "re-running m1 refused without naming the report: $REFUSAL_OUTPUT" ;;
esac

# --- the replay checker: the second instrument, with a negative control ----
REPLAY="$MS_DIR/target/release/replay_check"
"$REPLAY" "$SCRATCH/m1" || fail "replay_check refused m1's record"
"$REPLAY" "$SCRATCH/m2" || fail "replay_check refused m2's record"
# The control run: a tampered record must FAIL. Without this, a checker that
# approves everything would pass the two lines above by construction.
cp -r "$SCRATCH/m1" "$SCRATCH/m1_bad"
python3 - "$SCRATCH/m1_bad/g001.jsonl" <<'PY'
import pathlib, sys
path = pathlib.Path(sys.argv[1])
text = path.read_text(encoding="utf-8")
# Flip the recorded winner: p1 won; the record now claims p2 did.
tampered = text.replace("winner p1 at turn 7", "winner p2 at turn 7")
if tampered == text:
    sys.exit("the tamper found nothing to change: the control is vacuous")
path.write_text(tampered, encoding="utf-8")
PY
set +e
"$REPLAY" "$SCRATCH/m1_bad" >/dev/null 2>&1
TAMPER_RC=$?
set -e
[ "$TAMPER_RC" -eq 1 ] || fail "replay_check exited $TAMPER_RC on a tampered record, expected 1"

# --- the assertions: hand-derived expectations, positive facts ---------------
python3 - "$SCRATCH" <<'PY'
import json, pathlib, sys

scratch = pathlib.Path(sys.argv[1])

def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)

def transcript_lines(path):
    with open(path, encoding="utf-8") as handle:
        return [json.loads(line) for line in handle if line.strip()]

def check(condition, what):
    if not condition:
        sys.exit(f"sealbot-tests: FAIL: {what}")
    print(f"  ok: {what}")

# Match 1: a scripted win.
report = load(scratch / "m1" / "report.json")
check(report["games"] == 1, "m1 played 1 game")
game = report["games_detail"][0]
check(game["kind"] == "win", "m1 ended in a win")
check("winner p1" in game["detail"], "m1's winner is p1 (engine_a's seat)")
check("turn 7" in game["detail"], "m1's win is at turn 7")
check("first-stone win" in game["detail"], "m1's win is a first-stone win (rule 4)")
check(report["a_as_p1"]["win"] == 1, "engine_a as p1: one win")
check(report["b_as_p2"]["loss"] == 1, "engine_b as p2: one loss")
check(report["compute"]["a"]["nodes_total"] == 21, "engine_a accounted 21 nodes (3 answers x 7)")
low, high = report["interval"]["wilson_95_low"], report["interval"]["wilson_95_high"]
# Wilson at 1 win of 1 decided, z=1.96, by hand: centre 0.6033, half 0.3967.
check(report["interval"]["decided"] == 1 and abs(low - 0.2066) < 0.001 and high == 1.0,
      "m1's interval is the hand-derived Wilson [0.207, 1.0] at 1/1")
lines = transcript_lines(scratch / "m1" / "g001.jsonl")
turns = [line for line in lines if line["event"] == "turn"]
check(len(turns) == 6, "m1's transcript holds 6 judged turns")
last = turns[-1]
check(last["turn"] == 7 and last["mover"] == "p1", "the last turn is p1's turn 7")
check(last["outcome"]["kind"] == "win" and last["outcome"]["first_stone_win"] is True,
      "turn 7's outcome is a first-stone win")
check(last["stones"] == [[0, 5]], "turn 7 submitted exactly the stone 0,5")
check(lines[0]["event"] == "game_start" and lines[-1]["event"] == "game_end",
      "the transcript opens with game_start and closes with game_end")
movers = [turn["mover"] for turn in turns]
check(movers == ["p2", "p1", "p2", "p1", "p2", "p1"],
      "the turn order alternates p2-first (the server played p1's opener)")

# Match 2: a scripted forfeit.
report = load(scratch / "m2" / "report.json")
game = report["games_detail"][0]
check(game["kind"] == "forfeit", "m2 ended in a forfeit")
check("p2 forfeited" in game["detail"], "m2's forfeiter is p2 (engine_b's seat)")
check("illegal move" in game["detail"], "m2's forfeit is an illegal move")
check(report["a_as_p1"]["win_by_opponent_forfeit"] == 1,
      "engine_a's win arrived by the opponent's forfeit, counted separately")
check(report["b_as_p2"]["forfeit"] == 1, "engine_b as p2: one forfeit")
check(report["compute"]["a"]["nodes_total"] == 0,
      "engine_a was never asked (0 nodes accounted)")
lines = transcript_lines(scratch / "m2" / "g001.jsonl")
turns = [line for line in lines if line["event"] == "turn"]
check(len(turns) == 1 and turns[0]["outcome"]["kind"] == "illegal",
      "m2's transcript holds the one illegal turn")
print("sealbot-tests: PASS (both scripted matches matched their hand-derived outcomes)")
PY
