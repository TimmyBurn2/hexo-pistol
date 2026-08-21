"""WP-1.5b §8.3 Criterion 1 — the seat/label attribution chain, checked end to end.

  usage: tools/wp15b_attribution_check.py <report> <engine-binary>

THIS IS AN INSTRUMENT. It produces the verdict `docs/experiments/wp15b_sprt_prereg.md`
registers as its Criterion 1, so CLAUDE.md names it there WITH ITS REVISION and a
change to it reopens that document's review exactly as an amendment would. It is
covered by `crates/pistol-cli/tests/wp15b_attribution_check_tests.rs`, which drives
this file — the shipped one — against a report it builds and against three corrupted
copies of it, one per link, plus the control that catches link 1a passing vacuously
(tools/SHELL_CHECKLIST.md item 10).

WHY PYTHON AND NOT SHELL. The checklist's whole subject is EXIT-0-WRONG-ANSWER in
`set -euo pipefail` scripts parsing output they did not produce, and this parses a
report field by field and compares numbers. A missing key here raises rather than
expanding to the empty string, and a comparison on the wrong field is a KeyError and
not a silent pass.

WHAT IT CANNOT SEE, stated because a criterion that hides its blind spot is worse
than none: it reads the report the arena wrote. It cannot tell whether a game was
LEGAL — whether the recorded stones form six in a line at all — because that is
adjudication and needs pistol-core. Link 1b checks only that the seat the report
credits is the seat that played the last turn.

Three links, each with a referent that does NOT share the suspect path:

  1a  LABEL -> ENGINE.  The report says which label sat in which seat. Replayed:
      the two free turns after the book are each some engine's FIRST search of
      the game, so a fresh process reproduces them exactly. The engine the
      report names must return the move the report records. The referent is the
      engine process; nothing about it passes through the arena.
  1b  MOVES -> RESULT.  Game rule 3: turn 1 is one stone, every later turn two,
      and a win completes on a placed stone, so the winner played the LAST turn
      and `result p1_win` holds exactly when the turn count is odd. The referent
      is the recorded move list read against a pinned game rule.
  1c  RESULT -> SCORE -> VERDICT.  Seat A's score is rebuilt from `game … p1
      <label> … result <token>`, which `games()` writes off `a_is_p1` directly,
      and required to agree with `counts`, every `pair … bucket/score_a` and the
      pentanomial — all of which come through `GameRecord::score_a`, a disjoint
      function. The referent is the other code path.

Exit 0 all three pass, 1 any fail, 2 the report could not be read.
"""

import re
import subprocess
import sys


def die(why):
    print(f"attribution_check: CANNOT READ: {why}")
    raise SystemExit(2)


def fields(line):
    """`game 0 opening 3 p1 r2 …` -> {'game': '0', 'opening': '3', 'p1': 'r2', …}."""
    parts = line.split()
    return {parts[i]: parts[i + 1] for i in range(0, len(parts) - 1, 2)}


def main():
    if len(sys.argv) != 3:
        die("usage: attribution_check.py <report> <engine-binary>")
    report, engine = sys.argv[1], sys.argv[2]
    text = open(report, encoding="utf-8").read()
    # The verdict block only. Everything past the marker report.rs declares
    # "excluded from every comparison", and a criterion resting on it is the
    # defect revision 1 shipped.
    text = text.split("\n# timing")[0]
    if not text.startswith("arena_report "):
        die(f"{report} is not a report carrying a verdict (its first token is not arena_report)")

    engines = {}
    for slot in ("a", "b"):
        found = re.search(rf"^engine {slot} label (\S+) .* config (\S+) config_sha256 ", text, re.M)
        if not found:
            die(f"no `engine {slot}` line")
        engines[slot] = {"label": found.group(1), "config": found.group(2)}
    label_a, label_b = engines["a"]["label"], engines["b"]["label"]
    if label_a == label_b:
        die(f"both seats carry the label `{label_a}`, so no attribution can be read at all")
    by_label = {engines[s]["label"]: engines[s]["config"] for s in ("a", "b")}

    opening_turns = re.search(r"^opening_turns (\d+)$", text, re.M)
    if not opening_turns:
        die("no `opening_turns` line")
    opening_turns = int(opening_turns.group(1))

    games, moves = [], {}
    for line in text.splitlines():
        if line.startswith("game "):
            games.append(fields(line))
        elif line.startswith("moves "):
            parts = line.split()
            moves[parts[1]] = parts[2:]
    if not games:
        die("the report records no games")

    failures, notes = [], []

    # ---- 1a  LABEL -> ENGINE, by replay ------------------------------------
    checked = discriminating = 0
    for game in games:
        index = game["game"]
        played = moves.get(index)
        if played is None:
            die(f"game {index} has no `moves` line")
        if len(played) != int(game["turns"]):
            die(f"game {index}: `turns {game['turns']}` against {len(played)} recorded turns")
        for free in (opening_turns, opening_turns + 1):
            # A 0-based index into the move list; `free` counts turns already
            # played, so this turn's mover is p1 when `free` is even.
            if free >= len(played):
                continue
            mover = game["p1"] if free % 2 == 0 else game["p2"]
            prefix = " ".join(played[:free])
            answers = {}
            for label, config in by_label.items():
                said = subprocess.run(
                    [engine, "--config", config],
                    input=f"position start moves {prefix}\ngo nodes 50000\nquit\n",
                    capture_output=True,
                    text=True,
                )
                best = [x for x in said.stdout.splitlines() if x.startswith("bestmove ")]
                if len(best) != 1:
                    die(
                        f"game {index} turn {free + 1}: `{engine} --config {config}` answered no "
                        f"single bestmove; it said: {said.stdout.strip()} {said.stderr.strip()}"
                    )
                answers[label] = best[0].split()[1]
            checked += 1
            if answers[label_a] != answers[label_b]:
                discriminating += 1
            if answers[mover] != played[free]:
                failures.append(
                    f"1a game {index} turn {free + 1}: the report attributes `{played[free]}` to "
                    f"`{mover}`, and `{mover}` ({by_label[mover]}) answers `{answers[mover]}`"
                )
    # NON-VACUITY. Two engines that agree everywhere satisfy 1a whichever way
    # the labels are attached, so the count of turns on which they DISAGREE is
    # what makes this a criterion rather than a formality.
    notes.append(f"1a: {checked} turns replayed, {discriminating} of them discriminating")
    if discriminating == 0:
        failures.append(
            "1a is VACUOUS on this input: the two engines answered identically on every turn "
            "replayed, so the check passes under any labelling"
        )

    # ---- 1b  MOVES -> RESULT, by game rule 3 -------------------------------
    judged = 0
    for game in games:
        if game["result"] == "capped" or game["end"] != "normal":
            continue
        judged += 1
        by_p1 = len(moves[game["game"]]) % 2 == 1
        claimed = game["result"] == "p1_win"
        if by_p1 != claimed:
            failures.append(
                f"1b game {game['game']}: {game['turns']} turns were played, so the last turn was "
                f"{'p1' if by_p1 else 'p2'}'s, and the report records `result {game['result']}`"
            )
    notes.append(f"1b: {judged} decided non-forfeit games adjudicated against the move list")
    if judged == 0:
        failures.append("1b is VACUOUS on this input: no game was decided by the rules")

    # ---- 1c  RESULT -> SCORE -> VERDICT, off the score_a path ---------------
    def score_a(game):
        if game["result"] == "capped":
            return 0.5
        winner = game["p1"] if game["result"] == "p1_win" else game["p2"]
        return 1.0 if winner == label_a else 0.0

    scores = [score_a(g) for g in games]
    derived = {
        "n": len(games),
        "wins_a": sum(1 for g, s in zip(games, scores) if s == 1.0),
        "losses_a": sum(1 for g, s in zip(games, scores) if s == 0.0),
        "capped": sum(1 for g in games if g["result"] == "capped"),
    }
    counts = re.search(r"^counts (.*)$", text, re.M)
    if not counts:
        die("no `counts` line")
    counts = fields(counts.group(1))
    for key, value in derived.items():
        if int(counts[key]) != value:
            failures.append(
                f"1c `counts {key} {counts[key]}` against {value} rebuilt from the `game` lines"
            )

    buckets = []
    for first in range(0, len(games) - 1, 2):
        buckets.append(round((scores[first] + scores[first + 1]) * 2))
    printed = [fields(line) for line in text.splitlines() if line.startswith("pair ")]
    if len(printed) != len(buckets):
        failures.append(f"1c {len(printed)} `pair` lines against {len(buckets)} pairs of games")
    for pair, bucket in zip(printed, buckets):
        if pair["bucket"] != f"p{bucket}" or abs(float(pair["score_a"]) - bucket / 4) > 1e-9:
            failures.append(
                f"1c `pair {pair['pair']} bucket {pair['bucket']} score_a {pair['score_a']}` "
                f"against p{bucket} / {bucket / 4:.9f} rebuilt from the `game` lines"
            )

    pentanomial = re.search(r"^pentanomial (.*)$", text, re.M)
    if not pentanomial:
        die("no `pentanomial` line")
    pentanomial = fields(pentanomial.group(1))
    for slot in range(5):
        mine = sum(1 for bucket in buckets if bucket == slot)
        if int(pentanomial[f"p{slot}"]) != mine:
            failures.append(
                f"1c `pentanomial p{slot} {pentanomial[f'p{slot}']}` against {mine} rebuilt from "
                f"the `game` lines"
            )
    notes.append(f"1c: {len(games)} games and {len(buckets)} pairs rebuilt off the score_a path")

    for note in notes:
        print(f"attribution_check: {note}")
    for failure in failures:
        print(f"attribution_check: FAIL {failure}")
    print(f"attribution_check: {'PASS' if not failures else 'FAIL'} — {len(failures)} failure(s)")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
