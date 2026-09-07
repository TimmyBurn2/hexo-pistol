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
    out, refused = [], []
    for path in files:
        for number, line in enumerate(path.read_text().splitlines(), start=1):
            record = json.loads(line)
            if record.get("event") != "turn":
                continue
            for field in ("engine", "wall_ms"):
                if field not in record:
                    raise Unreadable(f"{path}:{number}: turn record has no `{field}`")
            # A TURN THE ENGINE NEVER ANSWERED IS NOT AN ANSWER. The referee
            # writes a record with `wall_ms: 0` and a null engine time on any
            # engine failure, pregame ones included, and counting those as
            # answers poisons every statistic here: measured, a single timeout
            # in a two-game run reported `3 of 4` engine times and dragged the
            # first-of-game median excess to -150 ms, which would read as a
            # start-up charge that had been fixed.
            if record.get("outcome", {}).get("kind") == "engine_failure":
                refused.append((path.name, record["engine"]))
                continue
            out.append((path.name, record["engine"], record["wall_ms"], record.get("engine_time_ms")))
    if not out:
        raise Unreadable(f"{run_dir}: transcripts hold no answered turns")
    return out, refused


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


def gap(rows):
    """`wall_ms - engine_time_ms` per answer: the seat's non-search overhead.

    Both ends matter. The upper end is the overhead the seat is charged for;
    the LOWER end is what refuses a seat reporting something other than its own
    elapsed time — a shim answering with its configured budget reports more than
    the wall it was measured over, and shows up here as a negative.
    """
    values = sorted(w - e for _, _, w, e in rows if e is not None)
    if not values:
        return None
    return {
        "n": len(values),
        "min": values[0],
        "median": statistics.median(values),
        "p95": values[min(len(values) - 1, int(0.95 * len(values)))],
        "max": values[-1],
        "negative": sum(1 for v in values if v < 0),
    }


def summarise(run_dir, budgets):
    """Per engine: the whole distribution, and it split first-of-game vs rest."""
    rows, refused = answers(run_dir)
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
        mine = [r for r in rows if r[1] == engine]
        engine_total = sum(e for _, _, _, e in mine if e is not None)
        out[engine] = {
            "budget_ms": budget,
            "all": distribution(walls, budget),
            "first_of_game": distribution(first.get(engine, []), budget),
            "later": distribution(rest.get(engine, []), budget) if rest.get(engine) else None,
            "engine_time_ms_reported": f"{timed[engine][1]} of {timed[engine][0]}",
            "gap": gap(mine),
            "wall_ms_total": sum(walls),
            "engine_time_ms_total": engine_total if timed[engine][1] else None,
            "refused_turns": sum(1 for _, who in refused if who == engine),
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

    lines += ["", "| engine | wall - engine_time (min / median / p95 / max) | negative | turns refused |", "|---|---|---|---|"]
    for engine, s in summary.items():
        g = s["gap"]
        cell = "— (no engine time reported)" if g is None else (
            f"{g['min']} / {g['median']:.0f} / {g['p95']} / {g['max']}"
        )
        neg = "—" if g is None else g["negative"]
        lines.append(f"| {engine} | {cell} | {neg} | {s['refused_turns']} |")

    lines += ["", "| engine | wall total | engine-time total | share of wall |", "|---|---|---|---|"]
    for engine, s in summary.items():
        et = s["engine_time_ms_total"]
        share = "—" if et is None or not s["wall_ms_total"] else f"{100 * et / s['wall_ms_total']:.2f}%"
        lines.append(
            f"| {engine} | {s['wall_ms_total']} ms | {'—' if et is None else f'{et} ms'} | {share} |"
        )
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
