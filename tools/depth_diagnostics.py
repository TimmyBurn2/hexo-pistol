#!/usr/bin/env python3
"""Read M7 and M9 out of arena SPRT reports that were already recorded.

The reports are `arena_report 4` records: whitespace-separated key/value lines,
one `game` line per game and one `pair` line per opening pair. Nothing here
runs an engine and nothing here is a strength claim (D-696).

Exit codes follow tools/SHELL_CHECKLIST.md item 12: 0 the answer was taken,
1 the answer is no (a report this cannot read), 2 no answer could be taken.
"""

import hashlib
import sys

GAME_FIELDS = (
    "opening p1 p2 result end forfeit_by reason turns dup_of "
    "nodes_a nodes_b depth_a depth_b llr_game llr_pair"
).split()

# The field a type-D partition needs and no arena report has ever written.
ROOT_SCORE_FIELDS = ("root_score", "score_a", "eval", "root_eval")


class Unreadable(Exception):
    """The file is not an arena report this reader understands."""


def parse(path):
    """Return (header, games, pairs) for one arena report.

    # Errors

    Raises `Unreadable` when the file does not open with an `arena_report`
    line, when a `game` line does not carry the fields this reader names, or
    when the header's own counts disagree with the lines present.
    """
    with open(path, encoding="utf-8") as handle:
        lines = handle.read().splitlines()
    if not lines or lines[0].split()[:1] != ["arena_report"]:
        raise Unreadable(f"{path}: no `arena_report` line at the top")

    header, games, pairs = {}, [], []
    for line in lines:
        words = line.split()
        if not words:
            continue
        if words[0] == "game":
            games.append(_record(words, path))
        elif words[0] == "pair":
            pairs.append(_kv(words[2:]))
        elif words[0] in ("engine", "engine_id", "moves", "timing", "timing_engine"):
            continue
        elif len(words) >= 2 and words[0] not in header:
            header[words[0]] = " ".join(words[1:])
    if not games:
        raise Unreadable(f"{path}: no `game` lines")
    return header, games, pairs


def _kv(words):
    return dict(zip(words[0::2], words[1::2]))


def _record(words, path):
    # words[1] is the game index; the rest is key/value.
    fields = _kv(words[2:])
    missing = [f for f in GAME_FIELDS if f not in fields]
    if missing:
        raise Unreadable(f"{path}: `game` line missing {', '.join(missing)}")
    return fields


def root_score_gap(games):
    """Which of the root-score field names a type-D partition needs are present."""
    seen = set()
    for game in games:
        seen |= {f for f in ROOT_SCORE_FIELDS if f in game}
    return sorted(seen)


def summarise(path):
    header, games, pairs = parse(path)
    labels = sorted({g["p1"] for g in games} | {g["p2"] for g in games})
    if len(labels) != 2:
        raise Unreadable(f"{path}: {len(labels)} engine labels, expected 2")

    buckets = {}
    for pair in pairs:
        buckets[pair["bucket"]] = buckets.get(pair["bucket"], 0) + 1

    results = {}
    for game in games:
        results[game["result"]] = results.get(game["result"], 0) + 1

    turns = [int(g["turns"]) for g in games]
    # Reached depth is recorded per SLOT (a/b); the slot a label occupies is
    # fixed for a run, so read it off the first game rather than assuming it.
    first = games[0]
    slot_of = {first["p1"]: "a", first["p2"]: "b"}
    if len(slot_of) != 2:
        raise Unreadable(f"{path}: game 0 has one engine on both seats")

    per_label = {}
    for label in labels:
        slot = slot_of[label]
        depths = [int(g[f"depth_{slot}"]) for g in games]
        nodes = [int(g[f"nodes_{slot}"]) for g in games]
        wins = [g for g in games if g["result"] == f"p{1 if g['p1'] == label else 2}_win"]
        per_label[label] = {
            "mean_depth": sum(depths) / len(depths),
            "max_depth": max(depths),
            "total_nodes": sum(nodes),
            "wins": len(wins),
            "mean_turns_of_its_wins": (
                sum(int(g["turns"]) for g in wins) / len(wins) if wins else None
            ),
        }

    return {
        "path": path,
        "sha256": hashlib.sha256(open(path, "rb").read()).hexdigest(),
        "labels": labels,
        "games": len(games),
        "pairs": len(pairs),
        "turn_cap": header.get("turn_cap"),
        "budget": header.get("budget"),
        "results": results,
        "capped_fraction": results.get("capped", 0) / len(games),
        "pentanomial": {k: buckets.get(k, 0) for k in ("p0", "p1", "p2", "p3", "p4")},
        "mean_turns": sum(turns) / len(turns),
        "mean_turns_decided": (
            sum(int(g["turns"]) for g in games if g["result"] != "capped")
            / max(1, sum(1 for g in games if g["result"] != "capped"))
        ),
        "per_label": per_label,
        "root_score_fields_present": root_score_gap(games),
    }


def render(runs):
    out = []
    out.append("| run | n | cap | pentanomial p0/p1/p2/p3/p4 | capped | mean turns (all / decided) |")
    out.append("|---|---|---|---|---|---|")
    for r in runs:
        p = r["pentanomial"]
        out.append(
            f"| `{r['path'].split('/')[-1]}` | {r['games']} | {r['turn_cap']} | "
            f"{p['p0']}/{p['p1']}/{p['p2']}/{p['p3']}/{p['p4']} | "
            f"{r['results'].get('capped', 0)} ({r['capped_fraction']:.3f}) | "
            f"{r['mean_turns']:.1f} / {r['mean_turns_decided']:.1f} |"
        )
    out.append("")
    out.append("| run | arm | mean reached depth | max | wins | mean turns of its wins |")
    out.append("|---|---|---|---|---|---|")
    for r in runs:
        for label, v in r["per_label"].items():
            mt = v["mean_turns_of_its_wins"]
            out.append(
                f"| `{r['path'].split('/')[-1]}` | {label} | {v['mean_depth']:.2f} | "
                f"{v['max_depth']} | {v['wins']} | {'—' if mt is None else f'{mt:.1f}'} |"
            )
    out.append("")
    for r in runs:
        p = r["pentanomial"]
        largest_cell = max(p, key=lambda k: p[k])
        strict = sum(1 for k in p if p[k] == p[largest_cell]) == 1
        res = r["results"]
        largest_res = max(res, key=lambda k: res[k])
        strict_res = sum(1 for k in res if res[k] == res[largest_res]) == 1
        out.append(
            f"- `{r['path'].split('/')[-1]}` sha256 `{r['sha256'][:16]}…`: "
            f"largest pentanomial cell **{largest_cell}**"
            f"{'' if strict else ' (TIED, not strict)'}; "
            f"largest result class **{largest_res}**"
            f"{'' if strict_res else ' (TIED, not strict)'}; "
            f"root-score fields present on `game` lines: "
            f"{r['root_score_fields_present'] or 'NONE'}"
        )
    return "\n".join(out)


def main(argv):
    if len(argv) < 2:
        print("usage: depth_diagnostics.py REPORT [REPORT ...]", file=sys.stderr)
        return 2
    try:
        runs = [summarise(p) for p in argv[1:]]
    except Unreadable as err:
        print(f"depth_diagnostics: {err}", file=sys.stderr)
        return 1
    except OSError as err:
        print(f"depth_diagnostics: RUN VOID: {err}", file=sys.stderr)
        return 2
    print(render(runs))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
