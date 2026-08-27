#!/usr/bin/env python3
"""WP-1.8b anchor-probe extractor (the command block's engine).

Reads sealbot anchor transcripts (JSONL), emits one solver fixture file per
to-move position (turn 2 onward: the prefix of stones placed before that
turn's own stones, origin included), plus an index TSV naming each case
game/turn/mover/engine. Deterministic: file order = transcript order.

usage: extract_probe_positions.py <out_dir> <transcript.jsonl> [...]
"""
import json
import sys
from pathlib import Path


def cells(stone):
    return f"{stone[0]},{stone[1]}"


def main():
    if len(sys.argv) < 3:
        sys.exit("usage: extract_probe_positions.py <out_dir> <transcript...>")
    out_dir = Path(sys.argv[1])
    out_dir.mkdir(parents=True, exist_ok=True)
    index_rows = []
    for path in sys.argv[2:]:
        stones = [(0, 0)]  # the server-played origin, p1 turn 1
        with open(path) as fh:
            for line in fh:
                rec = json.loads(line)
                if rec.get("event") != "turn":
                    continue
                turn, mover = rec["turn"], rec["mover"]
                engine = rec.get("engine", "?")
                stem = Path(path).stem
                name = f"{stem}-t{turn:02d}-{mover}"
                plies = " ".join(cells(s) for s in stones)
                fixture = out_dir / f"{name}.txt"
                # expect is a placeholder the probe driver never reads: the
                # probe records the solver's answer; it asserts no expectation.
                fixture.write_text(
                    f"case {name}\nplies {plies}\nexpect nowin\n"
                )
                index_rows.append(
                    f"{name}\t{stem}\t{turn}\t{mover}\t{engine}\t{len(stones)}"
                )
                for stone in rec["stones"]:
                    stones.append((stone[0], stone[1]))
    index = out_dir / "positions.tsv"
    index.write_text("case\tgame\tturn\tmover\tengine\tstones_before\n"
                     + "\n".join(index_rows) + "\n")
    print(f"extracted {len(index_rows)} positions -> {index}")


if __name__ == "__main__":
    main()
