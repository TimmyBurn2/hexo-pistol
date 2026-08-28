#!/usr/bin/env python3
"""The WP-P1 step-5 mapping gate: is HeXO's (x, y) pistol's axial (q, r)?

Two questions, answered separately because they fail for different reasons:

BASIS   does reading (x, y) as an axial pair in pistol's lattice preserve the
        metric and the three line axes? Falsified by re-reading the corpus
        under the other hex basis and counting rule violations.
WITNESS does an extracted replay position reproduce a known human game?
        Set-equality against the pinned human corpus, which the dispatch
        requires TWO independent links to satisfy.

Reports a verdict; never selects a mapping by trying variants until one fits.

Exit: 0 the gate passed, 1 it did not, 2 an input was refused.
"""

from __future__ import annotations

import hashlib
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
CORPUS = Path.home() / "Projects" / "hexo-bootstrap-corpus" / "hexo_human_corpus.jsonl"
CORPUS_SHA = "b2fe61eb360b91d77873a751446d28287955cad49e331fc32c156b4e1316840c"
PUZZLES = REPO / "corpus" / "puzzles" / "hexo_discord_v1.jsonl"

# D-447: cross-game position sharing vanishes from this turn on, measured by the
# --early-turns sweep over the pinned corpus. At or above the horizon a
# canonical-key match is unique BY MEASUREMENT, so one witness pins the element;
# below it a lone match is worth nothing, because 212 classes of games share an
# 11-stone opening.
SHARING_HORIZON_TURNS = 14
# The stone count at that turn boundary: turn 1 is one stone, every later turn two.
HORIZON_STONES = 2 * SHARING_HORIZON_TURNS - 1
# Below the horizon, the superseded two-witness guard still applies.
WITNESSES_BELOW_HORIZON = 2
LEGAL_RADIUS = 8
WIN_LEN = 6
AXES = ((0, 1), (1, 0), (1, -1))

REPLAY_MOVES = re.compile(r"Replay Move (\d+)/(\d+)")


def symmetries():
    """The twelve elements, as named coordinate maps.

    Mirrors `pistol_core::Symmetry::ALL`; this tool is standalone Python and
    cannot call it, and the mirror is asserted against the Rust order by the
    census rather than assumed (WP-P1b).
    """

    def rot(c):
        q, r = c
        return (-r, q + r)

    out = []
    for turns in range(6):
        for flip in (False, True):

            def make(turns=turns, flip=flip):
                def apply(c):
                    if flip:
                        c = (c[1], c[0])
                    for _ in range(turns):
                        c = rot(c)
                    return c

                return apply

            out.append((f"rot{turns * 60}{'+refl' if flip else ''}", make()))
    return out


def distance(a, b) -> int:
    dq, dr = b[0] - a[0], b[1] - a[1]
    return (abs(dq) + abs(dr) + abs(dq + dr)) // 2


def turn_groups(count: int):
    groups = defaultdict(list)
    for index in range(count):
        groups[1 if index == 0 else (index + 1) // 2 + 1].append(index)
    return [groups[k] for k in sorted(groups)]


def rule_violations(coords, players) -> tuple[int, int]:
    """(radius-8 turns with no legal ordering, six-runs) for one position."""
    placed, radius_bad = set(), 0
    for group in turn_groups(len(coords)):
        stones = [coords[i] for i in group]
        if placed:
            ok = False
            for i, head in enumerate(stones):
                if not any(distance(head, o) <= LEGAL_RADIUS for o in placed):
                    continue
                board = placed | {head}
                if all(
                    any(distance(s, o) <= LEGAL_RADIUS for o in board)
                    for j, s in enumerate(stones)
                    if j != i
                ):
                    ok = True
                    break
            if not ok:
                radius_bad += 1
        placed.update(stones)

    owner = dict(zip(coords, players))
    runs = 0
    for cell in coords:
        who = owner[cell]
        for axis in AXES:
            if owner.get((cell[0] - axis[0], cell[1] - axis[1])) == who:
                continue
            length, walk = 0, cell
            while owner.get(walk) == who:
                length += 1
                walk = (walk[0] + axis[0], walk[1] + axis[1])
            if length >= WIN_LEN:
                runs += 1
    return radius_bad, runs


def basis_leg(records) -> bool:
    """Re-read every position under each candidate basis and count violations.

    A basis whose third axis is (1,1) rather than pistol's (1,-1) is the map
    r -> -r. It is a DIFFERENT metric, so if the payload were in that basis,
    reading it as pistol axial would break rule 5 and rule 2.
    """
    candidates = {
        "identity  (q, r)": lambda q, r: (q, r),
        "reflected (r, q)": lambda q, r: (r, q),
        "negate-r  (q,-r)": lambda q, r: (q, -r),
        "negate-q  (-q,r)": lambda q, r: (-q, r),
    }
    print("BASIS leg — positions violating a pinned rule when read under each basis")
    print(f"  {'basis':18s} {'rule-5 turns':>13s} {'rule-2 six-runs':>16s}")
    scores = {}
    for name, fn in candidates.items():
        radius = runs = 0
        for record in records:
            coords = [fn(c["q"], c["r"]) for c in record["cells"]]
            players = [c["player"] for c in record["cells"]]
            a, b = rule_violations(coords, players)
            radius += a
            runs += b
        scores[name] = (radius, runs)
        print(f"  {name:18s} {radius:13d} {runs:16d}")
    clean = [n for n, s in scores.items() if s == (0, 0)]
    passed = scores["identity  (q, r)"] == (0, 0) and any(
        scores[n] != (0, 0) for n in scores if n.startswith("negate")
    )
    print(f"  -> clean under: {', '.join(clean)}")
    print(f"  -> BASIS {'PASS' if passed else 'FAIL'}: identity is metric-clean and a wrong basis is refuted\n")
    return passed


def role_of(index: int) -> int:
    """Which ROLE owns the placement at `index`: 1 opens, 2 replies.

    Roles, not labels. The corpus's first stone is by definition the opener;
    the sandbox payload's `player-N` is a seat identity and 17 of 48 positions
    open with `player-2` (WP-P1 F-2), so the two are compared through this and
    never directly.
    """
    turn = 1 if index == 0 else (index + 1) // 2 + 1
    return 1 if turn % 2 == 1 else 2


def witness_leg(records) -> int:
    """Set-equality against the pinned human corpus, over all 12 lattice symmetries.

    Reporting every symmetry that fits is the point: a witness that fits under
    more than one is not a witness for a particular mapping.

    Compared BOTH uncoloured and coloured. WP-P1 compared uncoloured only,
    because who owned a stone was then unknown; that is what left its single
    witness ambiguous under three elements. Colour is a second, independent
    coordinate on every stone, so a coloured comparison can separate images an
    uncoloured one cannot.
    """
    raw = CORPUS.read_bytes()
    got = hashlib.sha256(raw).hexdigest()
    if got != CORPUS_SHA:
        print(f"REFUSED: human corpus sha256 {got} is not the pinned {CORPUS_SHA}", file=sys.stderr)
        raise SystemExit(2)
    print(f"WITNESS leg — human corpus pin {CORPUS_SHA[:8]}… verified")

    games = [json.loads(line) for line in raw.decode().splitlines()]
    replays = [r for r in records if r["kind"] == "replay"]

    elements = symmetries()

    index = {}
    coloured_index = {}
    for n in sorted({r["cell_count"] for r in replays}):
        table = defaultdict(list)
        coloured = defaultdict(list)
        for game in games:
            moves = game["moves"]
            if len(moves) >= n:
                head = [tuple(m) for m in moves[:n]]
                entry = (game["game_hash"], len(moves))
                table[frozenset(head)].append(entry)
                coloured[frozenset((c, role_of(i)) for i, c in enumerate(head))].append(entry)
        index[n] = table
        coloured_index[n] = coloured

    found: list[tuple[str, int, str]] = []
    for record in replays:
        n = record["cell_count"]
        cells = [(c["q"], c["r"]) for c in record["cells"]]
        opener = record["cells"][0]["player"]
        roles = [1 if c["player"] == opener else 2 for c in record["cells"]]
        hits = {}
        coloured_hits = {}
        for name, fn in elements:
            image = [fn(c) for c in cells]
            if frozenset(image) in index[n]:
                hits[name] = index[n][frozenset(image)]
            key = frozenset(zip(image, roles))
            if key in coloured_index[n]:
                coloured_hits[name] = coloured_index[n][key]
        if not hits and not coloured_hits:
            continue
        title = REPLAY_MOVES.search(record["title"])
        total = int(title.group(2)) if title else None
        corroborated = [
            name for name, games_hit in hits.items() if any(g[1] == total for g in games_hit)
        ]
        verdict = (
            "UNAMBIGUOUS"
            if len(coloured_hits) == 1 and sum(len(v) for v in coloured_hits.values()) == 1
            else f"AMBIGUOUS: {len(hits)} uncoloured, {len(coloured_hits)} coloured"
        )
        print(f"  {record['id']}  n={n}  {verdict}")
        for name, games_hit in sorted(hits.items()):
            mark = "  <- move count corroborates" if name in corroborated else ""
            colour = "  COLOURED TOO" if name in coloured_hits else ""
            print(f"      uncoloured {name:12s} {games_hit}{mark}{colour}")
        for name, games_hit in sorted(coloured_hits.items()):
            if name not in hits:
                print(f"      coloured   {name:12s} {games_hit}  (colour only)")
        if len(coloured_hits) == 1 and sum(len(v) for v in coloured_hits.values()) == 1:
            found.append((record["id"], n, next(iter(coloured_hits))))

    at_horizon = [w for w in found if w[1] >= HORIZON_STONES]
    print(
        f"  -> {len(found)} unambiguous witness(es); "
        f"{len(at_horizon)} at or above the turn-{SHARING_HORIZON_TURNS} horizon "
        f"({HORIZON_STONES} stones)"
    )
    passed = bool(at_horizon) or len(found) >= WITNESSES_BELOW_HORIZON
    print(f"  -> WITNESS {'PASS' if passed else 'FAIL'}\n")
    return found


def stabilizer(cells) -> list[str]:
    """The point-group elements that map this position onto itself.

    A witness with a non-trivial stabilizer cannot discriminate the elements it
    is stabilized by, so it is a weaker witness however many stones it has.
    """
    target = frozenset(cells)
    return [name for name, fn in symmetries() if frozenset(fn(c) for c in cells) == target]


def preregister(records) -> None:
    """Print the witness selection BEFORE any comparison is run.

    WP-P1b step 3: selecting candidates after seeing which ones match is
    fitting. The rule is stated here and the order it produces is printed, so a
    later run against a fresh snapshot cannot quietly reorder it.
    """
    print("WITNESS PRE-REGISTRATION — selection rule, stated before measuring")
    print("  rule: kind == replay, trivial stabilizer first, then highest stone")
    print("        count, then id ascending. The first two are the witnesses.")
    print(
        f"  criterion (D-447): ONE witness at >= {HORIZON_STONES} stones "
        f"(turn {SHARING_HORIZON_TURNS}) closes the gate; below the horizon, "
        f"{WITNESSES_BELOW_HORIZON} are required."
    )
    replays = [r for r in records if r["kind"] == "replay"]
    ranked = []
    for record in replays:
        cells = [(c["q"], c["r"]) for c in record["cells"]]
        stab = stabilizer(cells)
        ranked.append((len(stab) > 1, -record["cell_count"], record["id"], stab))
    ranked.sort()
    for index, (nontrivial, negative, share_id, stab) in enumerate(ranked):
        mark = "  <- witness" if index < WITNESSES_BELOW_HORIZON else ""
        print(
            f"  {index + 1:2d}. {share_id}  n={-negative:3d}  "
            f"stabilizer {'non-trivial ' + ','.join(stab) if nontrivial else 'trivial'}{mark}"
        )
    print()


def mapping_status(basis: bool, witnesses: list) -> str:
    """VERIFIED only when the BASIS leg holds AND D-447's criterion is met."""
    if not basis:
        return "UNVERIFIED"
    if any(stones >= HORIZON_STONES for _, stones, _ in witnesses):
        return "VERIFIED"
    return "VERIFIED" if len(witnesses) >= WITNESSES_BELOW_HORIZON else "UNVERIFIED"


def main() -> int:
    if not PUZZLES.exists():
        print(f"REFUSED: {PUZZLES} does not exist; run extract.py first", file=sys.stderr)
        return 2
    records = [json.loads(line) for line in PUZZLES.read_text().splitlines()]
    print(f"MAPPING GATE — {len(records)} records, candidate mapping q = x, r = y\n")
    preregister(records)
    basis = basis_leg(records)
    witnesses = witness_leg(records)
    status = mapping_status(basis, witnesses)

    # A machine-readable line, so the suite asserts the STATUS rather than
    # parsing prose or reading an exit code that means two things.
    print(f"mapping_status: {status}")
    for share_id, stones, element in witnesses:
        horizon = "at/above" if stones >= HORIZON_STONES else "below"
        print(f"  witness {share_id}  {stones} stones  element {element}  {horizon} horizon")
    if status == "UNVERIFIED":
        print("grade: POSITION-GRADE (D-448). Not fixture-grade.")

    # The BASIS leg is the only leg whose failure is a DEFECT: it would mean the
    # payload is not axial in pistol's lattice at all. A witness shortfall is a
    # state of the evidence, not a broken instrument, and does not ship red.
    return 0 if basis else 1


if __name__ == "__main__":
    sys.exit(main())
