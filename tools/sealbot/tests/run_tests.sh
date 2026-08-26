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
# m3: p2 answers with ZERO stones every turn (F1's class: an empty reply
# must forfeit as an incomplete turn, never wedge the referee).
scratch.joinpath("m3_sealbot.json").write_text(json.dumps([
    [],
]))
scratch.joinpath("m3_pistol.json").write_text(json.dumps([]))
# m4: p2 submits THREE stones (F2/F6's class: illegal by count; the record
# must carry the ASKED turn number, and replay must agree with the referee).
scratch.joinpath("m4_sealbot.json").write_text(json.dumps([
    [[1, 1], [-1, 1], [2, 2]],
]))
scratch.joinpath("m4_pistol.json").write_text(json.dumps([]))
# m5/m6: MIXED over-submissions (R1's class) — a place-refusal at index 1
# (m5) and at index 0 (m6) inside a three-stone turn. The referee classifies
# by which stone failed FIRST, so both are illegal BY PLACE, and the replay
# must stop at exactly the referee's stone.
scratch.joinpath("m5_sealbot.json").write_text(json.dumps([
    [[1, 1], [0, 0], [2, 2]],
]))
scratch.joinpath("m5_pistol.json").write_text(json.dumps([]))
scratch.joinpath("m6_sealbot.json").write_text(json.dumps([
    [[50, 50], [1, 1], [-1, 1]],
]))
scratch.joinpath("m6_pistol.json").write_text(json.dumps([]))

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
scratch.joinpath("m3.toml").write_text(config("m3", "m3_sealbot.json"))
scratch.joinpath("m4.toml").write_text(config("m4", "m4_sealbot.json"))
scratch.joinpath("m5.toml").write_text(config("m5", "m5_sealbot.json"))
scratch.joinpath("m6.toml").write_text(config("m6", "m6_sealbot.json"))
PY
[ -s "$SCRATCH/m1.toml" ] || refuse "the config generator wrote nothing"

# --- run the shipped script four times ---------------------------------------
# timeout is the wedge guard: a referee that re-asks a silent engine forever
# (the F1 class) surfaces as exit 124 with a named refusal, not a hung suite.
for MATCH in m1 m2 m3 m4 m5 m6; do
  timeout 90 tools/sealbot/run_match.sh "$SCRATCH/$MATCH.toml" \
    || fail "run_match.sh refused, failed, or wedged on $MATCH"
done

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

# --- the replay checker: the second instrument, with negative controls ------
REPLAY="$MS_DIR/target/release/replay_check"
for MATCH in m1 m2 m3 m4 m5 m6; do
  "$REPLAY" "$SCRATCH/$MATCH" || fail "replay_check refused $MATCH's record"
done
# The control runs: a tampered record must FAIL, each naming a class the
# checker's own checks must reach. Without these, a checker that approves
# everything would pass the lines above by construction.
tamper() { # <dir> <label> <python-edit>
  local dir="$1" label="$2" edit="$3"
  rm -rf -- "$SCRATCH/$dir"
  cp -r "$SCRATCH/m1" "$SCRATCH/$dir"
  python3 - "$SCRATCH/$dir/g001.jsonl" "$edit" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
lines = [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]
edit = sys.argv[2]
changed = False
if edit == "winner":
    for entry in lines:
        if entry.get("event") == "game_end":
            entry["detail"] = entry["detail"].replace("winner p1", "winner p2")
            changed = True
elif edit == "extra_stone":
    for entry in lines:
        if entry.get("event") == "turn" and entry["outcome"]["kind"] == "continue" and entry["mover"] == "p2":
            entry["stones"] = entry["stones"] + [[5, 2]]
            changed = True
            break
elif edit == "mover":
    for entry in lines:
        if entry.get("event") == "turn" and entry["outcome"]["kind"] == "continue":
            entry["mover"] = "p1" if entry["mover"] == "p2" else "p2"
            changed = True
            break
if not changed:
    sys.exit(f"the {edit} tamper found nothing to change: the control is vacuous")
path.write_text("\n".join(json.dumps(e) for e in lines) + "\n", encoding="utf-8")
PY
  set +e
  "$REPLAY" "$SCRATCH/$dir" >/dev/null 2>&1
  local rc=$?
  set -e
  [ "$rc" -eq 1 ] || fail "replay_check exited $rc on the $label record, expected 1"
}
tamper m1_bad "winner-flipped" "winner"
tamper m1_extra "extra-stone" "extra_stone"
tamper m1_mover "mover-relabeled" "mover"

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
# Match 3: a zero-stone answer forfeits as an incomplete turn (the F1 class).
report = load(scratch / "m3" / "report.json")
game = report["games_detail"][0]
check(game["kind"] == "forfeit", "m3 ended in a forfeit")
check("p2 forfeited" in game["detail"], "m3's forfeiter is p2")
check("0 of 2 stones" in game["detail"], "m3's forfeit is a zero-stone incomplete turn")
lines = transcript_lines(scratch / "m3" / "g001.jsonl")
turns = [line for line in lines if line["event"] == "turn"]
check(len(turns) == 1, "m3's transcript holds the one zero-stone turn")
check(turns[0]["outcome"] == {"kind": "incomplete", "submitted": 0, "owed": 2},
      "the zero-stone turn is recorded incomplete 0-of-2")
check(turns[0]["turn"] == 2 and turns[0]["mover"] == "p2",
      "the zero-stone turn is recorded at turn 2, p2's")

# Match 4: a three-stone submission is illegal BY COUNT, recorded at the
# ASKED turn (F2/F6's class), and the replay checker agrees with the referee.
report = load(scratch / "m4" / "report.json")
game = report["games_detail"][0]
check(game["kind"] == "forfeit", "m4 ended in a forfeit")
check("more were submitted" in game["detail"], "m4's forfeit names the over-count")
lines = transcript_lines(scratch / "m4" / "g001.jsonl")
turns = [line for line in lines if line["event"] == "turn"]
check(len(turns) == 1 and turns[0]["turn"] == 2,
      "the over-submitted turn is recorded at the ASKED turn 2")
check(turns[0]["outcome"]["kind"] == "illegal" and turns[0]["outcome"]["stone"] == [2, 2],
      "the illegal stone is the first one past the owed count")
# Match 5/6: mixed over-submissions are illegal BY PLACE at the first
# refused stone (R1's class — the referee classifies by first failing index,
# never by length).
for name, illegal, why in (("m5", [0, 0], "already holds a stone"),
                           ("m6", [50, 50], "outside the legal region")):
    report = load(scratch / name / "report.json")
    game = report["games_detail"][0]
    check(game["kind"] == "forfeit", f"{name} ended in a forfeit")
    check(why in game["detail"], f"{name}'s forfeit names the refusal")
    turns = [line for line in transcript_lines(scratch / name / "g001.jsonl")
             if line["event"] == "turn"]
    check(len(turns) == 1 and turns[0]["outcome"]["kind"] == "illegal"
          and turns[0]["outcome"]["stone"] == illegal
          and turns[0]["turn"] == 2,
          f"{name}'s illegal stone is the first REFUSED one, at the asked turn")
print("sealbot-tests: PASS (all scripted matches matched their hand-derived outcomes)")
PY
