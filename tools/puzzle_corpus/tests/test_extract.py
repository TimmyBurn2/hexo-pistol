#!/usr/bin/env python3
"""WP-P1 extractor tests. Run via tools/puzzle_corpus/tests/run_tests.sh.

The negative controls carry the weight here: all 48 shipped positions validate
clean, so a suite that only asserted that would pass with every check deleted.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
FIXTURES = HERE / "fixtures"

spec = importlib.util.spec_from_file_location("extract", REPO / "tools/puzzle_corpus/extract.py")
ex = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ex)

FAILURES: list[str] = []


def check(name: str, condition: bool, detail: str = "") -> None:
    if condition:
        print(f"  ok    {name}")
    else:
        print(f"  FAIL  {name} {detail}")
        FAILURES.append(name)


# --- T1: the fixture page parses to exactly the expected record -------------

FIXTURE_ID = "n2sqzrf"
EXPECTED = {
    "id": "n2sqzrf",
    "url": "https://hexo.did.science/sandbox/n2sqzrf",
    "title": "aspratt vs srishtithisside - Replay Move 11/29",
    "fetched_at": "2026-08-28T14:51:12+00:00",
    "raw_sha256": "b77032b7a546d2032d9e504c9b619611d71b52ed10128ad8428d5757bd4ee8fd",
    "cells": [
        {"q": 0, "r": 0, "player": 2},
        {"q": 1, "r": -1, "player": 1},
        {"q": 0, "r": 1, "player": 1},
        {"q": 1, "r": 0, "player": 2},
        {"q": 0, "r": -1, "player": 2},
        {"q": 2, "r": 0, "player": 1},
        {"q": 2, "r": -1, "player": 1},
        {"q": -1, "r": 0, "player": 2},
        {"q": -2, "r": 1, "player": 2},
        {"q": 2, "r": -2, "player": 1},
        {"q": 1, "r": -2, "player": 1},
    ],
    "cell_count": 11,
    "to_move": 2,
    "placements_left": 2,
    "kind": "replay",
    "claim": None,
    "claim_status": "UNVERIFIED",
    "valid": True,
    "findings": [],
}


def test_fixture_parses_byte_exact() -> None:
    print("T1 extractor unit: the fixture page parses to the expected record")
    html = (FIXTURES / f"{FIXTURE_ID}.html").read_text()
    meta = json.loads((FIXTURES / f"{FIXTURE_ID}.fetch.json").read_text())
    check(
        "the fixture matches its recorded sha256",
        hashlib.sha256((FIXTURES / f"{FIXTURE_ID}.html").read_bytes()).hexdigest()
        == meta["raw_sha256"],
    )
    got = ex.build(FIXTURE_ID, html, meta)
    check("the record is byte-exact", json.dumps(got) == json.dumps(EXPECTED),
          f"\n        got {json.dumps(got)}")
    check("the opener is player-2, so the label is not play order", got["cells"][0]["player"] == 2)


# --- T2: every valid record is representable as a pistol Board --------------


def test_valid_records_are_representable() -> None:
    print("T2 load: every valid record is representable on pistol's lattice")
    records = [json.loads(l) for l in (REPO / "corpus/puzzles/hexo_discord_v1.jsonl").read_text().splitlines()]
    valid = [r for r in records if r["valid"]]
    check("there is something to load", len(valid) > 0)
    for record in valid:
        coords = [(c["q"], c["r"]) for c in record["cells"]]
        in_range = all(-32768 <= v <= 32767 for c in coords for v in c)
        check(f"{record['id']}: coordinates fit the i16 lattice (D-34)", in_range)
        check(f"{record['id']}: no cell is occupied twice", len(set(coords)) == len(coords))


# --- Negative controls: each validator fires on a position built to break it -


def record_of(cells, to_move, left):
    return {"cell_count": len(cells), "to_move": to_move, "placements_left": left}


def test_validators_fire() -> None:
    print("NEG negative controls: each check fires on a position built to break it")

    # A legal 5-stone board: turn1 p1, turn2 p2 x2, turn3 p1 x2.
    good = [
        {"q": 0, "r": 0, "player": 1},
        {"q": 1, "r": 0, "player": 2},
        {"q": 2, "r": 0, "player": 2},
        {"q": 0, "r": 1, "player": 1},
        {"q": 0, "r": 2, "player": 1},
    ]
    check("the control board itself is clean", ex.validate(record_of(good, 2, 2), good) == [])

    bad_left = ex.validate(record_of(good, 2, 1), good)
    check("V1 fires on a wrong placements_left", any(f.startswith("V1 parity") for f in bad_left))

    bad_move = ex.validate(record_of(good, 1, 2), good)
    check("V2 fires on a wrong to_move", any(f.startswith("V2 mover") for f in bad_move))

    # A stone 9 away from every other, with no partner able to bridge.
    far = good + [{"q": 40, "r": 0, "player": 2}, {"q": 41, "r": 0, "player": 2}]
    check(
        "V3 fires on a turn beyond LEGAL_RADIUS",
        any(f.startswith("V3 radius") for f in ex.validate(record_of(far, 1, 2), far)),
    )

    # Six in a row along ConstR for p1.
    run = [{"q": i, "r": 0, "player": 1} for i in range(6)] + [
        {"q": i, "r": 3, "player": 2} for i in range(5)
    ]
    check(
        "V4 fires on a six-run (IllegalPosition, D-6)",
        any(f.startswith("V4 no run") for f in ex.validate(record_of(run, 2, 2), run)),
    )

    # One stone repainted: the counts move AND the sequence breaks.
    swapped = [dict(c) for c in good]
    swapped[1]["player"] = 1
    out = ex.validate(record_of(swapped, 2, 2), swapped)
    check("V5 fires on counts off the turn structure", any(f.startswith("V5 counts") for f in out))
    check("V5 fires on a sequence off the turn structure", any(f.startswith("V5 sequence") for f in out))

    dup = good + [{"q": 0, "r": 0, "player": 2}, {"q": 5, "r": 0, "player": 2}]
    check(
        "a repeated cell is caught",
        any(f.startswith("V1 cells") for f in ex.validate(record_of(dup, 1, 2), dup)),
    )

    check(
        "ORDER fires when moveId disagrees with the listed order",
        ex.order_findings("x", [{"moveId": 2}, {"moveId": 1}]) != [],
    )
    check("ORDER fires when moveId is absent", ex.order_findings("x", [{}]) != [])

    # The wrong hex basis must be refutable, not merely different.
    coords = [(c["q"], -c["r"]) for c in run]
    players = [c["player"] for c in run]
    check("the six-run survives r -> -r detection as a different axis",
          ex.longest_run(coords, players) is not None)


def main() -> int:
    test_fixture_parses_byte_exact()
    test_valid_records_are_representable()
    test_validators_fire()
    print()
    if FAILURES:
        print(f"FAILED: {len(FAILURES)} check(s): {', '.join(FAILURES)}")
        return 1
    print("all checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
