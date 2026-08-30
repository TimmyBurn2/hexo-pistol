#!/usr/bin/env python3
"""Derive the per-search SOLVER CALL BUDGET the WP-1.8c bracket leaves, at HEAD.

Ruling 1 of the overnight dispatch retires pass-rate targets and designs the
resumed detector against a per-search call budget. That budget is a number the
bracket already implies, and this script reads it off the same
`tools/bench_block.sh` artifacts `tools/stage3_premise_derive.py` reads --
by an INDEPENDENT code path, so agreement on the shared intermediate
(`u_needed`, and the rate factor over it) is a cross-check of both.

The budget is stated in CAPPED-CALL EQUIVALENTS at the ON seat's committed
`per_call_node_cap`: `calls = solver_nodes / cap`. That is a LOWER bound on the
call count, because a call that returns before the cap spends fewer visits --
`K`, the mean visits per fired call, has no counter at HEAD (D-465, D-508), so
it is named here and never divided out silently.

Usage: stage3_call_budget_derive.py --cap N --off <artifact>... --on <artifact>...
Exit:  0 read and derived
       1 an input is malformed, or a band is missing a seat
       2 THE RUN IS VOID -- an input is unreadable
"""

import re
import sys

# The registered bounds of the WP-1.8c bracket, quoted at the band they bind.
BOUNDS = {"15": 0.50, "35": 0.50, "trigger": 0.25}
RECORD = re.compile(
    r"^bench_block: record entry (\d+) stones (\S+) rep (\d+) .*?"
    r"\bnodes (\d+)\b(?:.*?\bsearch_nodes (\d+)\b.*?\bsolver_nodes (\d+)\b)?"
    r".*?\btime (\d+)\b"
)


def void(msg):
    print(f"stage3_call_budget_derive: RUN VOID: {msg}", file=sys.stderr)
    sys.exit(2)


def fail(msg):
    print(f"stage3_call_budget_derive: FAIL: {msg}", file=sys.stderr)
    sys.exit(1)


# The banding is WP-1.8c's registered convention, not a computation: the
# trigger-rich fixture is one band, and a corpus entry is band 15 iff its own
# `stones` annotation reads 15 -- every other corpus count is band 35, which is
# how that bench banded these same 24 entries.
TRIGGER_FIXTURE = "bench_solver_positions_v1.txt"


def band_of(fixture, stones):
    if fixture == TRIGGER_FIXTURE:
        return "trigger"
    return "15" if stones == "15" else "35"


def read(paths):
    seats = {}
    for path in paths:
        try:
            with open(path, encoding="utf-8") as handle:
                text = handle.read()
        except OSError as err:
            void(f"cannot read {path}: {err}")
        if "bench_block: done:" not in text:
            fail(f"{path} has no `bench_block: done:` line -- that sweep did not complete")
        fixture, seen = None, 0
        for line in text.splitlines():
            if line.startswith("bench_block: fixture "):
                fixture = line.split()[2].rsplit("/", 1)[-1]
            match = RECORD.match(line)
            if not match:
                continue
            if fixture is None:
                fail(f"{path}: a record line came before its fixture header")
            seen += 1
            _, stones, _, nodes, search, solver, time_ms = match.groups()
            key = band_of(fixture, stones)
            spot = seats.setdefault(key, dict(rows=0, nodes=0, search=0, solver=0, us=0))
            spot["rows"] += 1
            spot["nodes"] += int(nodes)
            spot["search"] += int(search) if search is not None else int(nodes)
            spot["solver"] += int(solver) if solver is not None else 0
            spot["us"] += int(time_ms) * 1000
        if seen == 0:
            fail(f"{path} holds no `bench_block: record entry` line")
    return seats


def main(argv):
    cap, off_paths, on_paths, sink = None, [], [], None
    i = 0
    while i < len(argv):
        arg = argv[i]
        if arg == "--cap":
            i += 1
            if i >= len(argv) or not argv[i].isdigit() or int(argv[i]) == 0:
                fail("--cap wants a positive integer")
            cap = int(argv[i])
        elif arg == "--off":
            sink = off_paths
        elif arg == "--on":
            sink = on_paths
        elif arg.startswith("--"):
            fail(f"unknown option {arg}")
        else:
            if sink is None:
                fail("a path before --off or --on")
            sink.append(arg)
        i += 1
    if cap is None:
        fail("--cap is required: the budget is stated in capped-call equivalents")
    if not off_paths or not on_paths:
        fail("both --off and --on artifacts are required")

    off, on = read(off_paths), read(on_paths)
    print("=== per-band inputs, summed from the artifacts' own record lines ===")
    print(f"cap = {cap} visits per call (the ON seat's committed per_call_node_cap)")
    rows = []
    for band in sorted(set(off) & set(on), key=lambda b: (b == "trigger", b)):
        o, n = off[band], on[band]
        if o["nodes"] == 0 or n["search"] == 0 or n["solver"] == 0:
            fail(f"band {band} has an empty seat; nothing to derive")
        if band not in BOUNDS:
            fail(f"band {band} has no registered bracket bound")
        bound = BOUNDS[band]
        a = o["us"] / o["nodes"]
        u = n["solver"] / n["search"]
        c = (n["us"] - a * n["search"]) / n["solver"]
        print(
            f"band {band:>7}  OFF rows {o['rows']:4d} nodes {o['nodes']:9d}  "
            f"ON rows {n['rows']:4d} search {n['search']:8d} solver {n['solver']:9d}"
            f"  a {a:8.4f} us/node  c {c:9.4f} us/visit"
        )
        rows.append((band, bound, a, c, u, n))

    print()
    print("=== the budget, per search, in capped-call equivalents ===")
    print("T is the ON seat's own per-position total (search + solver); the bracket")
    print("does not move it, it moves the SPLIT. At the bound, S* = T/(1+u*).")
    for band, bound, a, c, u, n in rows:
        denom = bound * c - a
        if denom <= 0:
            fail(f"band {band}: bound {bound} is unreachable by any detector at t=0")
        u_needed = a * (1 - bound) / denom
        total = (n["search"] + n["solver"]) / n["rows"]
        s_star = total / (1 + u_needed)
        v_star = total - s_star
        calls_now = (n["solver"] / n["rows"]) / cap
        calls_star = v_star / cap
        print(
            f"band {band:>7}  bound {bound:.2f}  u_now {u:8.4f} -> u* {u_needed:.5f}"
            f"  (rate factor {u / u_needed:8.1f}x)"
        )
        print(
            f"           T {total:9.1f}  now: search {n['search'] / n['rows']:8.1f}"
            f" solver {n['solver'] / n['rows']:9.1f} = {calls_now:6.2f} capped calls"
        )
        print(
            f"           at the bound: search {s_star:9.1f} solver {v_star:9.1f}"
            f" = {calls_star:6.2f} capped calls   BUDGET"
        )
    print()
    print("K IS NOT DIVIDED OUT: `calls = solver_nodes / cap` assumes a call spends")
    print("the cap. A call that returns earlier spends less, so every count above is")
    print("a LOWER bound on the calls, and the BUDGET is a lower bound too --")
    print("the detector may afford fewer whole calls than the figure names, never more.")
    print("CALL_BUDGET_DONE")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
