#!/usr/bin/env python3
"""The interim pistol adapter for the hexo-bridge stdio v1 protocol.

For the day the HeXO remote is reachable again. The bridge's stdio adapter
spawns THIS program as its engine child; this program translates stdio v1
(JSON ops on stdin/stdout) into pistol's own line protocol, driving the real
pistol binary as ITS child. D-211 is the spec this implements, minus what it
assigns to pistol-api (which stays empty, CLAUDE.md rule 11): when that crate
is licensed it replaces this file, not the other way round.

Mapping (D-211):
  reset                        -> newgame            (reply {"ok":true,"v":1})
  setup + place stream         -> ONE `position` line, buffered until
                                 best_move, grouped into turns by rule 3
                                 (1 stone then 2 per turn), pairs spelled
                                 canonically (the parser refuses uncanonical
                                 spelling, D-46; replay accepts the canonical
                                 one whenever either order is legal, D-6)
  best_move time_ms            -> go movetime <ms>   (a ceiling, D-207)
  reply                        -> {"move": [...]}: 2 pairs, or 1 when the
                                 first stone already wins (rule 4)
  quit                         -> forwarded, exit 0

KNOWN LIMITS, recorded rather than hidden: the standard server's setup (one
cross at the origin) is the only setup this maps; a free_setup board is
refused loudly as D-211's pistol-api territory. And the reply's pair order is
the canonical spelling, which in the D-52 corner (a pair legal in only one
order) may not be the legal play order for a server that applies stones
strictly in order — the matchserver's pistol client recovers the true order
by replay; this interim adapter does not, because the python side has no
pistol-core to replay with.

Usage (as a bridge engine command):
  pistol_hexo_adapter.py --binary <pistol> --config <play-config.toml>
The pistol child runs with this process's working directory (its config's
internal paths resolve against it, as the pistol binary documents).

On any pistol refusal the adapter exits nonzero with the refusal on stderr,
which the bridge surfaces as a SubprocessEngineError (D-211: an empty move
list is never emitted).
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys

HANDSHAKE_OK = "pistolok"
BESTMOVE_PREFIX = "bestmove "
ERROR_PREFIX = "error "


class Adapter:
    def __init__(self, binary: str, config: str) -> None:
        self.child = subprocess.Popen(
            [binary, "--config", config],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        assert self.child.stdin is not None and self.child.stdout is not None
        self.setup: list[list[int]] = []
        self.places: list[list[int]] = []
        self.send("pistol")
        self.read_reply(lambda line: line == HANDSHAKE_OK)

    def send(self, line: str) -> None:
        assert self.child.stdin is not None
        self.child.stdin.write(line + "\n")
        self.child.stdin.flush()

    def read_reply(self, done) -> list[str]:
        """Read lines until `done(line)`; die on any refusal or EOF."""
        assert self.child.stdout is not None
        lines: list[str] = []
        while True:
            raw = self.child.stdout.readline()
            if not raw:
                self.die("pistol closed its stdout")
            line = raw.strip()
            if line.startswith(ERROR_PREFIX):
                self.die(f"pistol refused: {line}")
            lines.append(line)
            if done(line):
                return lines

    def die(self, why: str) -> None:
        sys.stderr.write(f"pistol_hexo_adapter: {why}\n")
        self.child.kill()
        sys.exit(1)

    def op_reset(self) -> dict:
        self.setup = []
        self.places = []
        self.send("newgame")
        return {"ok": True, "v": 1}

    def op_setup(self, op: dict) -> dict:
        cells = op.get("cells", [])
        if len(cells) > 1:
            self.die(
                "REFUSED: a free_setup board is D-211's pistol-api territory; the "
                "interim adapter maps only the standard single-cross setup"
            )
        self.setup = [[int(q), int(r)] for q, r, _ in cells]
        return {"ok": True}

    def op_place(self, op: dict) -> dict:
        self.places.append([int(op["q"]), int(op["r"])])
        return {"ok": True}

    def op_best_move(self, op: dict) -> dict:
        position = position_line([*self.setup, *self.places])
        if position:
            self.send(position)
        self.send(f"go movetime {int(op['time_ms'])}")
        best = None
        for line in self.read_reply(lambda line: line.startswith(BESTMOVE_PREFIX)):
            if line.startswith(BESTMOVE_PREFIX):
                best = line[len(BESTMOVE_PREFIX) :].strip()
        if best is None:
            self.die("pistol answered without a bestmove")
        pieces = [
            [int(part.split(",")[0]), int(part.split(",")[1])]
            for part in best.split("/")
        ]
        self.setup = []
        self.places = []
        return {"move": pieces}

    def op_quit(self) -> None:
        self.send("quit")
        self.child.wait()
        sys.exit(0)


def position_line(stones: list[list[int]]) -> str:
    """The move-list spelling of setup+places, or '' for an empty board.

    Rule 3's grouping: the first stone is a turn of one, the rest pair up. A
    stream that does not decompose is refused loudly — an odd trailing stone
    is a half-reported turn the bridge never sends mid-request, and anything
    worse is malformed input (CLAUDE.md rule 3: no silent repair).
    """
    if not stones:
        return ""
    rest = stones[1:]
    if len(rest) % 2 != 0:
        sys.stderr.write(
            "pistol_hexo_adapter: REFUSED: the stone stream does not decompose "
            "into turns\n"
        )
        sys.exit(1)
    tokens = [f"{stones[0][0]},{stones[0][1]}"]
    for index in range(0, len(rest), 2):
        a, b = sorted(
            ((rest[index][0], rest[index][1]), (rest[index + 1][0], rest[index + 1][1]))
        )
        tokens.append(f"{a[0]},{a[1]}/{b[0]},{b[1]}")
    return "position start moves " + " ".join(tokens)


def emit(obj: dict) -> None:
    sys.stdout.write(json.dumps(obj) + "\n")
    sys.stdout.flush()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="the interim pistol adapter for hexo-bridge stdio v1"
    )
    parser.add_argument("--binary", required=True)
    parser.add_argument("--config", required=True)
    args = parser.parse_args()

    adapter = Adapter(args.binary, args.config)
    for raw in sys.stdin:
        raw = raw.strip()
        if not raw:
            continue
        op = json.loads(raw)
        kind = op.get("op")
        if kind == "reset":
            emit(adapter.op_reset())
        elif kind == "setup":
            emit(adapter.op_setup(op))
        elif kind == "place":
            emit(adapter.op_place(op))
        elif kind == "best_move":
            emit(adapter.op_best_move(op))
        elif kind == "quit":
            adapter.op_quit()
        else:
            adapter.die(f"unknown op {kind!r}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
