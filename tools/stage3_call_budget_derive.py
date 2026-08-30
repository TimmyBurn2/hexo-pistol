#!/usr/bin/env python3
"""Derive the per-search SOLVER BUDGET the WP-1.8c bracket leaves, at HEAD.

Ruling 1 of the overnight dispatch (docs/experiments/stage3_overnight_dispatch.md
§0.2) retires pass-rate targets and designs the resumed detector against a
per-search budget. That budget is a quantity the bracket already implies, and
this script reads it off the same `tools/bench_block.sh` artifacts
`tools/stage3_premise_derive.py` reads, by an independently written parser.

WHAT THE BRACKET ACTUALLY FIXES is a FRACTION: `V*/T`, the share of a search's
node budget the solver may take. Everything else printed below is that fraction
presented in a unit, and every unit needs something the bracket does not supply:

  visits per search  = fraction x T          -- T is the BENCH's node budget
  invocations        = visits / cap          -- cap is the BENCH SEAT's cap
  firings            = invocations / 2       -- a firing makes up to two

THE DIRECTION OF EVERY COUNT BELOW IS A LOWER BOUND, and the reason is one
division: `count = visits / cap` uses the cap as the per-call spend, and a call
that returns before the cap spends LESS -- so the same visit budget buys MORE
calls, never fewer. `K`, the mean visits per invocation, has no counter in the
artifacts this reads (D-465, D-508); at `K < cap` every count rises.

A FIRING IS NOT AN INVOCATION and the distinction is structural, not statistical.
One firing calls the attacker direction (`crates/pistol-search/src/pvs.rs:609`)
and then, unless that proved a win, the defender one (`:630`); both are capped at
the one `per_call_node_cap` read at `:592`, and both add their visits to the same
counter (`:610`, `:631`). A detector gates the FIRING -- the predicate sits at
`:265-272` -- so the firing row is the axis a design sets.

`t = 0` THROUGHOUT, which is the FAVOURABLE assumption: the ON seat pays its
trigger predicate at every search node and the OFF seat does not, so a real
detector's own per-node cost shrinks `u*` and with it every figure below. The
sensitivity block prints what that costs.

Usage: stage3_call_budget_derive.py --cap N --off <artifact>... --on <artifact>...
Exit:  0 read and derived
       1 an input is malformed, mis-flagged, or a band is missing a seat
       2 THE RUN IS VOID -- an input is unreadable
"""

import hashlib
import re
import sys

# The registered bounds of the WP-1.8c bracket, quoted at the band they bind.
BOUNDS = {"15": 0.50, "35": 0.50, "trigger": 0.25}
# The banding is WP-1.8c's registered convention, not a computation: the
# trigger-rich fixture is one band, and a corpus entry is band 15 iff its own
# `stones` annotation reads 15 -- every other corpus count is band 35, which is
# how that bench banded these same 24 entries.
TRIGGER_FIXTURE = "bench_solver_positions_v1.txt"
# A firing makes at most this many invocations (pvs.rs:609, :630).
INVOCATIONS_PER_FIRING = 2
# What `t` costs, in microseconds per search node, printed as a sensitivity.
T_SWEEP = (0.0, 0.03, 0.10, 0.50, 1.00)
# Ruling 6's registered ceiling: the fraction of its `t = 0` value the budget
# may fall to before the detector's own per-node test has eaten what it was
# meant to protect (docs/decisions.md D-514).
BUDGET_EROSION_FLOOR = 0.90
# What a firing costs, as a multiple of the cap, when an invocation returns
# EARLY rather than spending the cap. Printed because the direction of every
# count below turns on it and a reader must not have to derive it.
K_SWEEP = (1.0, 0.5, 0.25)
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


def band_of(fixture, stones):
    if fixture == TRIGGER_FIXTURE:
        return "trigger"
    return "15" if stones == "15" else "35"


def read(paths, want_seat):
    """Sum one seat's artifacts per band, refusing a leg passed under the wrong flag.

    The seat is read from the artifact's OWN `bench_block: config` line and
    checked against the flag it arrived under, because a partial swap survives
    every aggregate test downstream (the sibling instrument reads the config
    line for the same reason).
    """
    seats = {}
    for path in paths:
        try:
            with open(path, "rb") as handle:
                raw = handle.read()
        except OSError as err:
            void(f"cannot read {path}: {err}")
        text = raw.decode("utf-8", errors="replace")
        print(f"input {want_seat} {path} sha256 {hashlib.sha256(raw).hexdigest()}")
        if "bench_block: done:" not in text:
            fail(f"{path} has no `bench_block: done:` line -- that sweep did not complete")
        fixture, seen = None, 0
        for line in text.splitlines():
            if line.startswith("bench_block: fixture "):
                fixture = line.split()[2].rsplit("/", 1)[-1]
            if line.startswith("bench_block: config "):
                seat = "on" if line.split()[2].endswith("_on.toml") else "off"
                if seat != want_seat:
                    article = "an" if seat == "on" else "a"
                    fail(
                        f"{path} is {article} {seat}-seat sweep passed under --{want_seat}"
                    )
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


def u_star(a, c, bound, t):
    """The solver-visits-per-search-node the bracket demands, at trigger cost `t`."""
    denom = bound * c - a
    numer = a * (1.0 - bound) - bound * t
    if denom <= 0 or numer <= 0:
        return None
    return numer / denom


def parse(argv):
    cap, off_paths, on_paths, sink = None, [], [], None
    index = 0
    while index < len(argv):
        arg = argv[index]
        if arg == "--cap":
            index += 1
            if index >= len(argv) or not argv[index].isdigit() or int(argv[index]) == 0:
                fail("--cap wants a positive integer")
            cap = int(argv[index])
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
        index += 1
    if cap is None:
        fail("--cap is required: a call count without its cap is not a quantity")
    if not off_paths or not on_paths:
        fail("both --off and --on artifacts are required")
    return cap, off_paths, on_paths


def main(argv):
    cap, off_paths, on_paths = parse(argv)
    print("=== the run, and what it read ===")
    print(f"argv {' '.join(argv)}")
    print(f"cap {cap} visits per INVOCATION (the ON seat's committed per_call_node_cap)")
    off, on = read(off_paths, "off"), read(on_paths, "on")

    print()
    print("=== per-band inputs, summed from the artifacts' own record lines ===")
    rows = []
    for band in sorted(set(off) & set(on), key=lambda b: (b == "trigger", b)):
        o, n = off[band], on[band]
        if o["nodes"] == 0 or n["search"] == 0 or n["solver"] == 0:
            fail(f"band {band} has an empty seat; nothing to derive")
        if band not in BOUNDS:
            fail(f"band {band} has no registered bracket bound")
        a = o["us"] / o["nodes"]
        u = n["solver"] / n["search"]
        c = (n["us"] - a * n["search"]) / n["solver"]
        print(
            f"band {band:>7}  OFF rows {o['rows']:4d} nodes {o['nodes']:9d}  "
            f"ON rows {n['rows']:4d} search {n['search']:8d} solver {n['solver']:9d}"
            f"  a {a:8.4f} us/node  c {c:9.4f} us/visit"
        )
        rows.append((band, BOUNDS[band], a, c, u, o, n))

    print()
    print("=== WHAT THE BRACKET FIXES: the solver's share of a search's nodes ===")
    print("V*/T = u*/(1+u*). This is the only figure below that needs neither the")
    print("bench's node budget nor any seat's cap, and it is the one to carry.")
    for band, bound, a, c, u, _, _ in rows:
        star = u_star(a, c, bound, 0.0)
        if star is None:
            fail(f"band {band}: bound {bound} is unreachable by any detector at t=0")
        print(
            f"band {band:>7}  bound {bound:.2f}  u_now {u:8.4f} -> u* {star:.5f}"
            f"  (rate factor {u / star:8.1f}x)   SHARE {100.0 * star / (1.0 + star):7.3f}%"
        )

    print()
    print("=== the same share as a per-search count, and what each unit borrows ===")
    print("T_on is the ON seat's own per-position total; T_off is the OFF seat's.")
    print("They differ because a solver call absorbs its whole node count at once,")
    print("so the ON seat overshoots the budget by more. At the bound the detector")
    print("has gated nearly every call, so T_off is the nearer end -- both printed,")
    print("and the range is the honest statement.")
    for band, bound, a, c, u, o, n in rows:
        star = u_star(a, c, bound, 0.0)
        share = star / (1.0 + star)
        t_on = (n["search"] + n["solver"]) / n["rows"]
        t_off = o["nodes"] / o["rows"]
        now_solver = n["solver"] / n["rows"]
        print(f"band {band:>7}  T_on {t_on:9.1f}  T_off {t_off:9.1f}")
        print(
            f"           NOW: solver {now_solver:9.1f} visits"
            f" = {now_solver / cap:6.2f} invocations"
            f" = {now_solver / (cap * INVOCATIONS_PER_FIRING):6.2f} firings (at 2 caps each)"
        )
        for label, total in (("T_off", t_off), ("T_on", t_on)):
            visits = share * total
            print(
                f"           BUDGET at {label:5s}: {visits:8.1f} visits"
                f" >= {visits / cap:6.2f} invocations"
                f" >= {visits / (cap * INVOCATIONS_PER_FIRING):6.2f} firings"
            )

    print()
    print("=== the FAVOURABLE assumption, priced ===")
    print("t = us the ON seat pays per SEARCH node for its trigger predicate, which")
    print("the OFF seat never evaluates. Every figure above is at t = 0. A detector")
    print("adds its OWN per-node test on top, so this is the column that binds it.")
    for band, bound, a, c, u, o, n in rows:
        t_on = (n["search"] + n["solver"]) / n["rows"]
        cells = []
        for t in T_SWEEP:
            star = u_star(a, c, bound, t)
            if star is None:
                cells.append(f"t {t:4.2f}: UNREACHABLE")
                continue
            visits = (star / (1.0 + star)) * t_on
            cells.append(f"t {t:4.2f}: {visits / (cap * INVOCATIONS_PER_FIRING):5.2f}f")
        print(f"band {band:>7}  firings at T_on   " + "  ".join(cells))

    print()
    print("=== ruling 6's registered ceiling: where the detector's own cost eats the budget ===")
    print("t_90 is the per-node cost at which the band's budget falls to "
          f"{BUDGET_EROSION_FLOOR:.0%} of its t = 0")
    print("value. A detector whose own per-node test costs more than this has spent")
    print("what it was built to protect (docs/decisions.md D-514).")
    for band, bound, a, c, u, o, n in rows:
        star = u_star(a, c, bound, 0.0)
        share0 = star / (1.0 + star)
        target = BUDGET_EROSION_FLOOR * share0
        u_target = target / (1.0 - target)
        # u*(t) is affine in t: u* = (a(1-R) - R.t)/(R.c - a).
        denom = bound * c - a
        t_90 = (a * (1.0 - bound) - u_target * denom) / bound
        print(f"band {band:>7}  share0 {100.0 * share0:7.3f}%  t_90 {t_90:8.4f} us/node"
              f"  (= {t_90 / a:5.3f} x ONE SEARCH NODE at a = {a:.4f})")

    print()
    print("=== the direction, priced: what an EARLY-RETURNING invocation buys ===")
    print("`invocations = visits / cap` prices an invocation at the cap. K is the MEAN")
    print("visits an invocation actually spends; at K below the cap the same visit")
    print("budget affords MORE. K has no counter in these artifacts -- it is measured")
    print("by the counters D-510 landed, not here -- so the sweep stands in for it.")
    for band, bound, a, c, u, o, n in rows:
        star = u_star(a, c, bound, 0.0)
        share = star / (1.0 + star)
        t_on = (n["search"] + n["solver"]) / n["rows"]
        cells = []
        for k in K_SWEEP:
            per_firing = INVOCATIONS_PER_FIRING * k * cap
            cells.append(f"K={k:4.2f}cap: {share * t_on / per_firing:5.2f}f")
        print(f"band {band:>7}  firings at T_on   " + "  ".join(cells))

    print()
    print("K IS NOT DIVIDED OUT, AND THE DIRECTION IS ONE WAY. `invocations =")
    print("visits / cap` prices an invocation at the cap. One that returns earlier")
    print("costs less, so the same visit budget affords MORE invocations and more")
    print("firings, never fewer -- every count above is a LOWER BOUND. What is NOT")
    print("a lower bound is the visit budget itself, which is exact given the share")
    print("and the total it is taken of.")
    print("CALL_BUDGET_DONE")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
