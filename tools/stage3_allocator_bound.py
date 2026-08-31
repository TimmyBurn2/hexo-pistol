#!/usr/bin/env python3
"""Score row (e) — the per-search solver BUDGET allocated by a precision score —
and bound what any such score can reach.

WHY THIS EXISTS AND WHY `stage3_census_rank.py` COULD NOT DO IT. That instrument
scores PREDICATES: a predicate keeps a fraction of firings and costs whatever
that fraction costs, so its budget verdict is a measurement. An ALLOCATOR is not
a predicate. It carries a budget and stops when the budget is spent, so it
REACHES THE BUDGET BY CONSTRUCTION on every band, in sample and out, and its
entire question is RECALL: does the score rank the win-proving firings into the
part of the order the budget pays for? Ranking rows (a)/(b)/(g)/(j) and row (e)
on one table would report a budget verdict that means two different things.

TWO FRAMES, AND THEY ARE NOT THE SAME NUMBER.

  * AGGREGATE — the bracket is an nps ratio over a whole bench, so what it fixes
    is total solver visits over total searches. A detector free to spend heavily
    on one position and nothing on the next is inside it.
  * PER SEARCH — the form the governing dispatch names ("a PER-SEARCH SOLVER
    CALL BUDGET"). This is STRICTLY harder, because the wins are not spread
    evenly: a search holding six proofs cannot pay for all six out of one
    search's budget however good the score is.

Both are reported. Reading a per-search claim off an aggregate mean is the
substitution D-477 exists to forbid.

THE BOUND. A score is a function of the columns, so it cannot tell two firings
apart when every column agrees. The finest partition it can act on is the
column-vector CLASS, and the best any score can do is choose classes subject to
the budget — a knapsack. That number is an UPPER BOUND fitted with full
knowledge of which classes hold wins; no score that has to GENERALISE reaches it
except by luck, and the gap between it and the measured scores is the honest
statement of how much is left on the table.

Usage: stage3_allocator_bound.py --census <on> --off <off> --fixture <positions>
                                 --share <frac> --band 15|35|trigger
Exit:  0 scored
       1 an input is malformed
       2 THE RUN IS VOID -- an input is unreadable
"""

import sys
from collections import defaultdict

TRIGGER_FIXTURE = "bench_solver_positions_v1.txt"
COLUMNS = (
    "turns", "mover_hot", "opp_hot", "mover_w1", "opp_w1",
    "mover_l3", "opp_l3", "cover", "covers",
)


def void(msg):
    print(f"stage3_allocator_bound: RUN VOID: {msg}", file=sys.stderr)
    sys.exit(2)


def fail(msg):
    print(f"stage3_allocator_bound: FAIL: {msg}", file=sys.stderr)
    sys.exit(1)


def read(path):
    try:
        with open(path, encoding="utf-8") as handle:
            return handle.read()
    except OSError as err:
        void(f"cannot read {path}: {err}")


def bands_of(fixture_path):
    name = fixture_path.rsplit("/", 1)[-1]
    bands = []
    for line in read(fixture_path).splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        if name == TRIGGER_FIXTURE:
            bands.append("trigger")
            continue
        after = line.split("stones")
        if len(after) < 2:
            fail(f"{fixture_path}: an entry with no `stones` annotation")
        bands.append("15" if after[-1].split()[0] == "15" else "35")
    return bands


def fields(line, start):
    words = line.split()
    spot = {}
    index = start
    while index + 1 < len(words):
        spot[words[index]] = words[index + 1]
        index += 2
    return spot, words


def firings(census_path, bands):
    rows = []
    for line in read(census_path).splitlines():
        if not line.startswith("trigger_census: row entry "):
            continue
        spot, words = fields(line, 4)
        spot["entry"] = int(words[3])
        if spot["entry"] >= len(bands):
            fail(f"{census_path}: entry {spot['entry']} is past the fixture's end")
        spot["band"] = bands[spot["entry"]]
        rows.append(spot)
    if not rows:
        fail(f"{census_path} holds no `trigger_census: row` line")
    if "cover" not in rows[0]:
        fail(f"{census_path} has no `cover` column: row (e)'s score reads it")
    return rows


def totals(off_path, bands):
    per_band = defaultdict(list)
    for line in read(off_path).splitlines():
        if not line.startswith("trigger_census: entry "):
            continue
        spot, words = fields(line, 3)
        index = int(words[2])
        if int(spot["solver_nodes"]) != 0:
            fail(f"{off_path}: entry {index} spent solver nodes; that is not an OFF seat")
        per_band[bands[index]].append(int(spot["search_nodes"]))
    if not per_band:
        fail(f"{off_path} holds no `trigger_census: entry` line")
    return per_band


def visits(row):
    return int(row["att_visits"]) + int(row["def_visits"])


def won(row):
    """The ATTACKER direction proved. The gate's direction, and the only one
    this instrument counts (D-522)."""
    return row["att_proved"] == "true"


SCORES = {
    "mover_hot desc, then mover_l3": lambda r: (-int(r["mover_hot"]), -int(r["mover_l3"])),
    "mover_l3 desc": lambda r: -int(r["mover_l3"]),
    "opp_hot asc": lambda r: int(r["opp_hot"]),
    "opp_hot desc": lambda r: -int(r["opp_hot"]),
    "cover impossible, then fewest": lambda r: (
        0 if r["cover"] == "impossible" else 1, int(r["covers"])),
    "shallowest first": lambda r: int(r["turns"]),
    "deepest first": lambda r: -int(r["turns"]),
    "mover_l3 - opp_l3 desc": lambda r: -(int(r["mover_l3"]) - int(r["opp_l3"])),
    "opp_w1 desc": lambda r: -int(r["opp_w1"]),
}


def greedy(rows, budget, order_key):
    """Admit in score order, skipping a firing that does not fit. Returns
    (admitted, wins admitted, visits spent)."""
    left, admitted, wins, spent = budget, 0, 0, 0
    for row in sorted(rows, key=order_key) if order_key else rows:
        cost = visits(row)
        if cost <= left:
            left -= cost
            admitted += 1
            spent += cost
            wins += 1 if won(row) else 0
    return admitted, wins, spent


def knapsack_bound(rows, budget):
    """The best win count ANY column-score can admit inside `budget`.

    Exact over classes, on a scaled capacity so the table stays small; the scale
    ROUNDS COSTS UP and the capacity DOWN, so the answer can only understate the
    bound, never overstate it -- the safe direction for a number used to say a
    field is not information-starved.
    """
    classes = defaultdict(lambda: [0, 0])
    for row in rows:
        key = tuple(row[column] for column in COLUMNS)
        classes[key][0] += 1 if won(row) else 0
        classes[key][1] += visits(row)
    scale = 8
    capacity = int(budget) // scale
    table = [0] * (capacity + 1)
    for wins, cost in classes.values():
        weight = max(1, -(-cost // scale))
        if weight > capacity:
            continue
        for room in range(capacity, weight - 1, -1):
            candidate = table[room - weight] + wins
            if candidate > table[room]:
                table[room] = candidate
    return max(table), len(classes), sum(1 for w, _ in classes.values() if w)


def main(argv):
    census = off = fixture = band = None
    share = None
    index = 0
    while index < len(argv):
        key = argv[index]
        index += 1
        if index >= len(argv):
            fail(f"{key} wants a value")
        value = argv[index]
        index += 1
        if key == "--census":
            census = value
        elif key == "--off":
            off = value
        elif key == "--fixture":
            fixture = value
        elif key == "--share":
            share = float(value)
        elif key == "--band":
            band = value
        else:
            fail(f"unknown option {key}")
    if None in (census, off, fixture, band) or share is None:
        fail("--census, --off, --fixture, --share and --band are all required")

    bands = bands_of(fixture)
    rows = [row for row in firings(census, bands) if row["band"] == band]
    per_band = totals(off, bands)
    if band not in per_band:
        fail(f"--band {band} is not one of {sorted(per_band)} in {off}")
    searches = len(per_band[band])
    per_search = share * (sum(per_band[band]) / searches)
    aggregate = per_search * searches
    wins = [row for row in rows if won(row)]

    print(f"=== row (e), the budget allocator — {census} band {band} ===")
    print(f"argv {' '.join(argv)}")
    print(f"searches {searches}  firings {len(rows)}  "
          f"budget {per_search:.1f} visits/search = {aggregate:.0f} aggregate")
    print(f"WIN-direction denominator {len(wins)} (D-522: the attacker direction, "
          f"and losses are not added in)")
    if not wins:
        print("NO WIN-DIRECTION PROOF ON THIS BAND. Recall is not measurable here and")
        print("no ordering below is evidence about any row. Nothing further is printed.")
        print("ALLOCATOR_BOUND_DONE")
        return 0

    print()
    print("AN ALLOCATOR REACHES THE BUDGET BY CONSTRUCTION. The verdict is recall.")
    print()
    print("--- PER-SEARCH frame (the dispatch's form): each search gets its own budget")
    by_entry = defaultdict(list)
    for row in wins:
        by_entry[row["entry"]].append(visits(row))
    affordable = 0
    for entry, costs in sorted(by_entry.items()):
        costs = sorted(costs)
        left, take = per_search, 0
        for cost in costs:
            if cost <= left:
                left -= cost
                take += 1
        affordable += take
        verdict = "all fit" if take == len(costs) else f"ONLY {take} of {len(costs)} fit"
        print(f"    search {entry:3d}: wins cost {costs} = {sum(costs)} "
              f"vs this search's {per_search:.0f} -> {verdict}")
    print(f"    *** CEILING, per-search frame: {affordable}/{len(wins)} = "
          f"{affordable / len(wins):.3f} — no score beats it, because the budget is")
    print(f"        spent per search and the wins are not spread evenly.")
    print()
    print("--- AGGREGATE frame (what the bracket itself fixes)")
    cheapest = sorted(visits(row) for row in wins)
    left, take = aggregate, 0
    for cost in cheapest:
        if cost <= left:
            left -= cost
            take += 1
    print(f"    every win costs {sum(cheapest)} of {aggregate:.0f} aggregate "
          f"({sum(cheapest) / aggregate:.2f}x the budget)")
    print(f"    *** ORACLE, aggregate frame: {take}/{len(wins)} = {take / len(wins):.3f}")
    best, class_count, win_classes = knapsack_bound(rows, aggregate)
    print(f"    *** BOUND over the census COLUMNS: {best}/{len(wins)} = "
          f"{best / len(wins):.3f}   ({class_count} distinct column-classes, "
          f"{win_classes} holding a win)")
    print("        This is what a score fitted WITH FULL KNOWLEDGE of which classes")
    print("        hold wins could reach. It is an upper bound and it is IN-SAMPLE by")
    print("        construction; a score that must generalise is a different object.")
    print()
    print("--- MEASURED: orderings over the columns, aggregate frame")
    for name, key in SCORES.items():
        admitted, kept, spent = greedy(rows, aggregate, key)
        print(f"    {name:<32} admits {admitted:3d}/{len(rows):3d}  "
              f"spends {spent:6d}  WIN RECALL {kept}/{len(wins)} = {kept / len(wins):.3f}")
    print("ALLOCATOR_BOUND_DONE")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
