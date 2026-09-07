#!/usr/bin/env python3
"""The sealbot engine shim: JSON lines in, JSON lines out.

The matchserver's sealbot seat drives this shim; the shim drives sealbot.
Everything sealbot-specific lives here (its python, its module paths, its
HexGame replay), so the matchserver's client stays a thin protocol driver and
no game rule is implemented outside pistol-core (tools/sealbot/README.md).

This is the same replay pattern hexo-bridge's seal_perf_engine.py ships,
reached over a simpler contract:

    argv:  sealbot_shim.py <bot_dir> <root_dir>
    stdin:  one JSON object per request:
            {"setup": [[q, r], ...], "moves": [[q, r], ...],
             "time_limit": <seconds>}
    stdout: `sealbot_shim: ready` once the bot is constructed, THEN one JSON
            object per reply:
            {"moves": [[q, r], ...], "engine_time_ms": <int>}

The readiness line is written to stdout AND stderr, and the client reads the
stdout one before it starts timing: without it the interpreter start and the
extension import land inside the first measured answer of every game
(docs/decisions.md D-699). `engine_time_ms` is the bot's own elapsed time for
the answer, in whole milliseconds.

`setup` is the server-played opening (the origin cross); `moves` is every
stone after it, in true play order. `time_limit` comes with every request —
the matchserver's config is the single source of the budget — and a request
without one falls back to a conservative default rather than guessing big.

On any failure the shim exits nonzero WITHOUT a reply: the matchserver reads
that as an engine failure (a forfeit), never as a move.
"""

from __future__ import annotations

import json
import sys
import time


READY = "sealbot_shim: ready"


def main() -> int:
    if len(sys.argv) != 3:
        sys.stderr.write("usage: sealbot_shim.py <bot_dir> <root_dir>\n")
        return 2
    bot_dir, root_dir = sys.argv[1], sys.argv[2]
    default_limit = 0.1

    sys.path.insert(0, bot_dir)
    from minimax_cpp import MinimaxBot

    sys.path.insert(0, root_dir)
    from game import HexGame

    bot = MinimaxBot(default_limit)
    # BOTH STREAMS, deliberately. stdout is what the client reads before it
    # starts the clock, so the interpreter start, the extension import and this
    # construction are not charged to the first answer of a game. stderr keeps
    # the line the run's per-game `.stderr` files have always carried, which is
    # what proves one shim process per game.
    for stream in (sys.stdout, sys.stderr):
        stream.write(READY + "\n")
        stream.flush()

    for line in sys.stdin:
        line = line.strip()
        if not line or line == "quit":
            break
        request = json.loads(line)
        game = HexGame(win_length=6)
        game.reset()
        # The server-played opening first, then every stone in play order —
        # sealbot's own HexGame applies its own turn model to both.
        for q, r in request.get("setup", []):
            game.make_move(int(q), int(r))
        for q, r in request.get("moves", []):
            game.make_move(int(q), int(r))
        bot.time_limit = float(request.get("time_limit", default_limit))
        started = time.monotonic()
        result = bot.get_move(game)
        # The bot's OWN elapsed time, not the server's wall: the difference
        # between the two is the seat's overhead, and until this was reported
        # nothing could measure it (docs/decisions.md D-699). It is `get_move`'s
        # whole span, so it carries sealbot's untimed setup and its rollback --
        # it is the engine's elapsed time for the answer, not pure search.
        engine_time_ms = int((time.monotonic() - started) * 1000)
        reply = {
            "moves": [[int(q), int(r)] for q, r in result],
            "engine_time_ms": engine_time_ms,
        }
        sys.stdout.write(json.dumps(reply) + "\n")
        sys.stdout.flush()
    return 0


if __name__ == "__main__":
    sys.exit(main())
