#!/usr/bin/env python3
"""Extract HeXO sandbox share positions into a versioned JSONL corpus.

Reports; never repairs. A position that fails validation keeps its record with
`valid: false` and a populated `findings` list (WP-P1 step 4).
"""

# RULE9-JUSTIFICATION: the WP-P1 dispatch pins the extractor to one script, and
# the fetch, the parse and the V-checks share the record shape they build and
# annotate; splitting them would put that shape in a fourth file read by three.
# The gate reads .rs and .sh, so this marker is prose rule 9 still binds.

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import re
import subprocess
import sys
import time
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
LINKS = Path(__file__).resolve().parent / "links_v1.txt"
CACHE = REPO / "local" / "puzzle_cache"
OUT = REPO / "corpus" / "puzzles" / "hexo_discord_v1.jsonl"

BASE = "https://hexo.did.science/sandbox/"
RATE_LIMIT_SECONDS = 1.0

# The hydration payload the page inlines. Discovery (WP-P1 step 1) found the
# cells here and nowhere else: the rendered body is the clean-board default and
# the <meta> block carries counts only.
STATE_MARKER = "window.__IH3T_DEHYDRATED_STATE__="
POSITION_QUERY = "sandbox-position"

# LEGAL_RADIUS, pistol-core rules.rs (D-101, measured over the corpus in D-218).
LEGAL_RADIUS = 8

# The three line axes, pistol-core axis.rs: ConstQ, ConstR, ConstS.
AXES = ((0, 1), (1, 0), (1, -1))
WIN_LEN = 6

REPLAY_TITLE = re.compile(r"^(?P<p1>.+) vs (?P<p2>.+) - Replay Move (?P<n>\d+)/(?P<m>\d+)$")

# Deliberately narrow: a title asserts an outcome or it does not. `claim` is
# copied verbatim and never parsed into a distance (WP-P1 step 3).
OUTCOME_WORD = re.compile(r"\b(win|wins|winning|won|forced|mate|mated|loss|lose|lost|draw)\b", re.I)

PLAYER = {"player-1": 1, "player-2": 2}


# --- fetch ------------------------------------------------------------------


def cache_paths(share_id: str) -> tuple[Path, Path]:
    return CACHE / f"{share_id}.html", CACHE / f"{share_id}.fetch.json"


def fetch(share_id: str, offline: bool) -> tuple[str, dict]:
    """Return the raw page and its fetch sidecar, from cache when present.

    The sidecar carries `fetched_at` so a re-run from cache reproduces the
    output byte for byte; deriving it from the clock would not.
    """
    html_path, meta_path = cache_paths(share_id)
    if html_path.exists() and meta_path.exists():
        raw = html_path.read_bytes()
        meta = json.loads(meta_path.read_text())
        if meta["raw_sha256"] != hashlib.sha256(raw).hexdigest():
            raise SystemExit(f"{share_id}: cached page does not match its recorded sha256")
        return raw.decode("utf-8"), meta
    if offline:
        raise SystemExit(f"{share_id}: no cache entry and --offline was given")

    url = BASE + share_id
    done = subprocess.run(
        ["curl", "-sS", "--fail", "--max-time", "30", url],
        capture_output=True,
        check=True,
    )
    raw = done.stdout
    meta = {
        "url": url,
        "fetched_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat(),
        "raw_sha256": hashlib.sha256(raw).hexdigest(),
    }
    CACHE.mkdir(parents=True, exist_ok=True)
    html_path.write_bytes(raw)
    meta_path.write_text(json.dumps(meta, indent=1) + "\n")
    time.sleep(RATE_LIMIT_SECONDS)
    return raw.decode("utf-8"), meta


# --- parse ------------------------------------------------------------------


def parse_position(html: str, share_id: str) -> dict:
    """Pull the sandbox position out of the inlined hydration state.

    # Raises
    SystemExit when the marker, the query, or the position is absent: a page
    that does not carry a position is not a position to repair.
    """
    at = html.find(STATE_MARKER)
    if at < 0:
        raise SystemExit(f"{share_id}: no {STATE_MARKER} in the page")
    start = html.index("{", at)
    state, _ = json.JSONDecoder().raw_decode(html[start:])
    for query in state.get("queries", []):
        key = query.get("queryKey") or []
        if key and key[0] == POSITION_QUERY:
            data = query["state"]["data"]
            if data is None:
                raise SystemExit(f"{share_id}: the position query resolved to null")
            return data
    raise SystemExit(f"{share_id}: no `{POSITION_QUERY}` query in the hydration state")


def meta_description(html: str) -> str | None:
    found = re.search(r'<meta name="description" content="([^"]*)"', html)
    return found.group(1) if found else None


# --- turn structure ---------------------------------------------------------


def turn_of(index: int) -> int:
    """Which turn the placement at `index` (0-based) belongs to.

    Turn 1 is one stone, every later turn two (CLAUDE.md rule 3).
    """
    return 1 if index == 0 else (index + 1) // 2 + 1


def mover_of_turn(turn: int, opener: int) -> int:
    """Who owns `turn`, given who owns turn 1.

    The sandbox payload's `player-1`/`player-2` are identity labels, not play
    order: 17 of the 48 v1 positions open with `player-2`. Deriving the mover
    from the label rather than from the position would call every one of them
    invalid (WP-P1 findings, F-2).
    """
    return opener if turn % 2 == 1 else 3 - opener


def derived_state(cell_count: int, opener: int) -> tuple[int, int]:
    """The (to_move, placements_left) the turn structure implies for a board of
    `cell_count` stones opened by `opener`."""
    if cell_count == 0:
        return opener, 1
    if cell_count % 2 == 1:
        completed = (cell_count + 1) // 2
        return mover_of_turn(completed + 1, opener), 2
    completed = cell_count // 2
    return mover_of_turn(completed + 1, opener), 1


def group_turns(cells: list[dict]) -> list[list[int]]:
    groups: list[list[int]] = []
    for index in range(len(cells)):
        turn = turn_of(index)
        while len(groups) < turn:
            groups.append([])
        groups[turn - 1].append(index)
    return groups


# --- geometry (mirrors pistol-core; see MAPPING GATE in the report) ---------


def distance(a: tuple[int, int], b: tuple[int, int]) -> int:
    dq, dr = b[0] - a[0], b[1] - a[1]
    return (abs(dq) + abs(dr) + abs(dq + dr)) // 2


def within_radius(cell: tuple[int, int], placed: set) -> bool:
    return any(distance(cell, other) <= LEGAL_RADIUS for other in placed)


# --- validation (V1..V5); every failure is recorded, none is repaired -------


def validate(record: dict, cells: list[dict]) -> list[str]:
    findings: list[str] = []
    count = record["cell_count"]
    coords = [(c["q"], c["r"]) for c in cells]
    players = [c["player"] for c in cells]

    opener = players[0] if players else 1
    want_move, want_left = derived_state(count, opener)
    if record["placements_left"] != want_left:
        findings.append(
            f"V1 parity: {count} cells implies {want_left} placement(s) left, "
            f"the page states {record['placements_left']}"
        )
    if record["to_move"] != want_move:
        findings.append(
            f"V2 mover: {count} cells and the turn structure imply player {want_move} "
            f"to move, the page states player {record['to_move']}"
        )

    if len(set(coords)) != len(coords):
        findings.append("V1 cells: the same cell appears more than once")

    groups = group_turns(cells)
    placed: set = set()
    for turn_index, group in enumerate(groups, start=1):
        stones = [coords[i] for i in group]
        if not placed:
            placed.update(stones)
            continue
        # D-6: a turn is legal iff SOME ordering of its placements is.
        ok = False
        for first in range(len(stones)):
            head = stones[first]
            if not within_radius(head, placed):
                continue
            rest = [s for j, s in enumerate(stones) if j != first]
            if all(within_radius(s, placed | {head}) for s in rest):
                ok = True
                break
        if not ok:
            findings.append(
                f"V3 radius: turn {turn_index} has no ordering placing every stone "
                f"within {LEGAL_RADIUS} of the board"
            )
        placed.update(stones)

    run = longest_run(coords, players)
    if run is not None:
        cell, axis, length = run
        findings.append(
            f"V4 no run: a run of {length} for player {players[coords.index(cell)]} "
            f"along axis {axis} from ({cell[0]},{cell[1]}) — IllegalPosition per D-6"
        )

    want_p1 = sum(1 for i in range(count) if mover_of_turn(turn_of(i), opener) == 1)
    got_p1 = players.count(1)
    if got_p1 != want_p1:
        findings.append(
            f"V5 counts: the turn structure implies {want_p1} player-1 stones "
            f"and {count - want_p1} player-2, the page has {got_p1} and {count - got_p1}"
        )
    mismatched = [i for i in range(count) if players[i] != mover_of_turn(turn_of(i), opener)]
    if mismatched:
        findings.append(
            f"V5 sequence: {len(mismatched)} placement(s) whose player does not match "
            f"the turn structure, first at index {mismatched[0]}"
        )
    return findings


def longest_run(coords: list, players: list) -> tuple | None:
    """The first >=WIN_LEN same-player run found, as (start cell, axis, length)."""
    owner = {c: p for c, p in zip(coords, players)}
    for cell in coords:
        who = owner[cell]
        for axis in AXES:
            back = (cell[0] - axis[0], cell[1] - axis[1])
            if owner.get(back) == who:
                continue
            length, walk = 0, cell
            while owner.get(walk) == who:
                length += 1
                walk = (walk[0] + axis[0], walk[1] + axis[1])
            if length >= WIN_LEN:
                return cell, f"({axis[0]},{axis[1]})", length
    return None


# --- record -----------------------------------------------------------------


def build(share_id: str, html: str, meta: dict) -> dict:
    data = parse_position(html, share_id)
    if data.get("id") != share_id:
        raise SystemExit(f"{share_id}: the payload identifies itself as {data.get('id')!r}")
    position = data["gamePosition"]
    title = data["name"]

    cells = []
    for raw_cell in position["cells"]:
        who = PLAYER.get(raw_cell["player"])
        if who is None:
            raise SystemExit(f"{share_id}: unknown player {raw_cell['player']!r}")
        cells.append({"q": raw_cell["x"], "r": raw_cell["y"], "player": who})

    to_move = PLAYER.get(position["currentTurnPlayer"])
    if to_move is None:
        raise SystemExit(f"{share_id}: unknown currentTurnPlayer")

    record = {
        "id": share_id,
        "url": meta["url"],
        "title": title,
        "fetched_at": meta["fetched_at"],
        "raw_sha256": meta["raw_sha256"],
        "cells": cells,
        "cell_count": len(cells),
        "to_move": to_move,
        "placements_left": position["placementsRemaining"],
        "kind": "replay" if REPLAY_TITLE.match(title) else "constructed",
        "claim": title if OUTCOME_WORD.search(title) else None,
        "claim_status": "UNVERIFIED",
        "valid": True,
        "findings": [],
    }

    findings = validate(record, cells)
    findings.extend(order_findings(share_id, position["cells"]))
    findings.extend(meta_findings(html, record))
    record["findings"] = findings
    record["valid"] = not findings
    return record


def order_findings(share_id: str, raw_cells: list) -> list[str]:
    """D-6 identifies the phase-1 stone as the last listed stone of the mover, so
    a payload whose order is not stable makes every `placements_left == 1`
    record unusable."""
    ids = [c.get("moveId") for c in raw_cells]
    if any(i is None for i in ids):
        return ["ORDER: a cell carries no moveId, so the listed order is unwitnessed"]
    if ids != sorted(ids):
        return [f"ORDER: moveId order disagrees with the listed order (first at {ids.index(min(ids))})"]
    return []


def meta_findings(html: str, record: dict) -> list[str]:
    """The <meta> description states the same counts as the payload. Two channels
    that disagree mean one of them is not the position."""
    description = meta_description(html)
    if description is None:
        return ["META: the page carries no description meta tag"]
    stated = re.search(r"(\d+) placed cells", description)
    if stated and int(stated.group(1)) != record["cell_count"]:
        return [f"META: description says {stated.group(1)} cells, payload has {record['cell_count']}"]
    return []


# --- main -------------------------------------------------------------------


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--offline", action="store_true", help="fail rather than fetch")
    parser.add_argument("--out", type=Path, default=OUT)
    args = parser.parse_args()

    ids = [
        line.strip()
        for line in LINKS.read_text().splitlines()
        if line.strip() and not line.startswith("#")
    ]
    if len(set(ids)) != len(ids):
        raise SystemExit("links_v1.txt lists an id twice")

    records = []
    for share_id in ids:
        html, meta = fetch(share_id, args.offline)
        records.append(build(share_id, html, meta))
    records.sort(key=lambda r: r["id"])

    args.out.parent.mkdir(parents=True, exist_ok=True)
    with args.out.open("w") as handle:
        for record in records:
            handle.write(json.dumps(record, ensure_ascii=False) + "\n")

    digest = hashlib.sha256(args.out.read_bytes()).hexdigest()
    valid = sum(1 for r in records if r["valid"])
    print(f"{len(records)} records, {valid} valid, {len(records) - valid} with findings")
    try:
        shown = args.out.relative_to(REPO)
    except ValueError:
        shown = args.out
    print(f"{shown}  sha256 {digest}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
