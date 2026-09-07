#!/usr/bin/env python3
"""Per-answer wall and overshoot, per engine, from a match run's transcripts.

The match report publishes `a_answer_wall_ms` for slot A and nothing for slot B
(`tools/sealbot/matchserver/src/report.rs:101`), so a run has never reported both
sides' distributions. The per-game transcripts carry `wall_ms` for every turn of
both engines, and this reads them.

The first answer of each game is reported apart from the rest: a client that
respawns its engine per game and does not wait for it charges process start-up
to that answer, which is a term no engine's own overshoot bound covers.

Exit codes follow tools/SHELL_CHECKLIST.md item 12: 0 the answer was taken,
1 the answer is no (transcripts this cannot read), 2 no answer could be taken.
"""

import json
import pathlib
import statistics
import sys


class Unreadable(Exception):
    """The directory holds no transcripts this reader understands."""


class NoAnswer(Exception):
    """The run could not be looked at, which is not the same as an empty run."""


def answers(run_dir):
    """Every turn record of every game, in game order, as (game, engine, wall_ms).

    # Errors

    Raises `Unreadable` when no `g*.jsonl` file is present, or when a turn
    record does not carry the two fields this reader names.
    """
    directory = pathlib.Path(run_dir)
    if not directory.is_dir():
        # NOT the same thing as a run that produced nothing: no answer could be
        # taken at all (tools/SHELL_CHECKLIST.md item 12).
        raise NoAnswer(f"{run_dir} is not a directory")
    files = sorted(directory.glob("g*.jsonl"))
    if not files:
        raise Unreadable(f"{run_dir}: no g*.jsonl transcripts")
    out = []
    for path in files:
        for number, line in enumerate(path.read_text().splitlines(), start=1):
            record = json.loads(line)
            if record.get("event") != "turn":
                continue
            for field in ("engine", "wall_ms"):
                if field not in record:
                    raise Unreadable(f"{path}:{number}: turn record has no `{field}`")
            out.append((path.name, record["engine"], record["wall_ms"], record.get("engine_time_ms")))
    if not out:
        raise Unreadable(f"{run_dir}: transcripts hold no turn records")
    return out


def distribution(walls, budget_ms):
    """The registered five numbers, plus the overshoot the budget defines."""
    excess = [w - budget_ms for w in walls] if budget_ms else []
    ordered = sorted(walls)
    return {
        "n": len(walls),
        "median": statistics.median(walls),
        "p95": ordered[min(len(ordered) - 1, int(0.95 * len(ordered)))],
        "max": max(walls),
        "over_budget": sum(1 for x in excess if x > 0) if budget_ms else None,
        "max_excess": max(excess) if excess else None,
    }


def summarise(run_dir, budgets):
    """Per engine: the whole distribution, and it split first-of-game vs rest."""
    rows = answers(run_dir)
    per, first, rest, timed = {}, {}, {}, {}
    seen = set()
    for game, engine, wall, engine_ms in rows:
        per.setdefault(engine, []).append(wall)
        timed.setdefault(engine, [0, 0])
        timed[engine][0] += 1
        if engine_ms is not None:
            timed[engine][1] += 1
        key = (game, engine)
        (first if key not in seen else rest).setdefault(engine, []).append(wall)
        seen.add(key)

    out = {}
    for engine, walls in sorted(per.items()):
        budget = budgets.get(engine)
        out[engine] = {
            "budget_ms": budget,
            "all": distribution(walls, budget),
            "first_of_game": distribution(first.get(engine, []), budget),
            "later": distribution(rest.get(engine, []), budget) if rest.get(engine) else None,
            "engine_time_ms_reported": f"{timed[engine][1]} of {timed[engine][0]}",
        }
    return out


def render(run_dir, summary):
    lines = [f"# {run_dir}", "", "| engine | budget | n | median | p95 | max | over budget | max excess | engine_time_ms |", "|---|---|---|---|---|---|---|---|---|"]
    for engine, s in summary.items():
        a = s["all"]
        lines.append(
            f"| {engine} | {s['budget_ms'] or '—'} | {a['n']} | {a['median']:.0f} | {a['p95']} | "
            f"{a['max']} | {a['over_budget'] if a['over_budget'] is not None else '—'} | "
            f"{a['max_excess'] if a['max_excess'] is not None else '—'} | {s['engine_time_ms_reported']} |"
        )
    lines += ["", "| engine | first answer of each game (median / max excess) | every later answer |", "|---|---|---|"]
    for engine, s in summary.items():
        f, l = s["first_of_game"], s["later"]
        fx = "—" if f["max_excess"] is None else f"{f['median'] - (s['budget_ms'] or 0):.0f} / {f['max_excess']}"
        lx = "—" if not l or l["max_excess"] is None else f"{l['median'] - (s['budget_ms'] or 0):.0f} / {l['max_excess']}"
        lines.append(f"| {engine} | {fx} | {lx} |")
    return "\n".join(lines)


def main(argv):
    if len(argv) < 2:
        print(
            "usage: anchor_overshoot.py RUN_DIR [ENGINE_LABEL=BUDGET_MS ...]",
            file=sys.stderr,
        )
        return 2
    budgets = {}
    for pair in argv[2:]:
        if "=" not in pair:
            print(f"anchor_overshoot: `{pair}` is not LABEL=BUDGET_MS", file=sys.stderr)
            return 1
        label, value = pair.rsplit("=", 1)
        if not value.isdigit():
            print(f"anchor_overshoot: `{value}` is not a whole millisecond count", file=sys.stderr)
            return 1
        budgets[label] = int(value)
    try:
        summary = summarise(argv[1], budgets)
    except Unreadable as err:
        print(f"anchor_overshoot: {err}", file=sys.stderr)
        return 1
    except NoAnswer as err:
        print(f"anchor_overshoot: RUN VOID: {err}", file=sys.stderr)
        return 2
    except (OSError, json.JSONDecodeError) as err:
        print(f"anchor_overshoot: RUN VOID: {err}", file=sys.stderr)
        return 2
    print(render(argv[1], summary))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
