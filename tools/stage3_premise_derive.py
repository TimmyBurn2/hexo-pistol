#!/usr/bin/env python3
"""Re-derive the factor the WP-1.8c bracket demands, from that bench's own rows.

The bracket is an nps RATIO taken under a SHARED node budget
(crates/pistol-search/src/pvs.rs:140-142, :681), so a gated call returns its
nodes to the search, which then traverses further and fires more calls. The
ratio is therefore not linear in the call count and the factor cannot be read
off the ratio shortfall. This script states the relation, says honestly what
checking it against the record does and does not establish, and inverts it at
the registered bounds.

Two input modes, one code path for the arithmetic:
  <bench> [<capsweep>]        WP-1.8c's raw `row `/`cap ` artifacts
  --blocks <sweep> [...]      `tools/bench_block.sh` sweeps taken at any revision

Exit:  0 read and derived
       1 an input is malformed, or (WP-1.8c mode) the model does not reproduce
         the ratios the bench recorded
       2 THE RUN IS VOID -- an input is unreadable
"""

import sys
import re
import statistics
from collections import defaultdict

TOL = 0.0006  # absolute, against ratios recorded to four decimals

WANT = [
    ("CORPUS band 15", "bench_positions_v1.txt", "15", 0.50, 0.0809),
    ("CORPUS band 35", "bench_positions_v1.txt", "35", 0.50, 0.0458),
    ("TRIGGER-RICH", "bench_solver_positions_v1.txt", "trigger", 0.25, 0.0488),
]


def void(msg):
    print(f"stage3_premise_derive: RUN VOID: {msg}", file=sys.stderr)
    sys.exit(2)


def fail(msg):
    print(f"stage3_premise_derive: FAIL: {msg}", file=sys.stderr)
    sys.exit(1)


def slurp(path):
    try:
        with open(path, encoding="utf-8") as handle:
            return handle.read()
    except OSError as err:
        void(f"cannot read {path}: {err}")


def cell():
    return dict(rows=0, nodes=0, search=0, solver=0, time=0, reps=defaultdict(lambda: [0, 0]))


def add(agg, key, rep, nodes, search, solver, time_ms):
    spot = agg[key]
    spot["rows"] += 1
    spot["nodes"] += nodes
    spot["search"] += search
    spot["solver"] += solver
    spot["time"] += time_ms
    spot["reps"][rep][0] += nodes
    spot["reps"][rep][1] += time_ms


def read_rows(path):
    """WP-1.8c's raw `row ` lines, summed per (fixture, seat, band)."""
    agg = defaultdict(cell)
    for line in slurp(path).splitlines():
        if not line.startswith("row "):
            continue
        field = line.split()

        def after(name, default=0):
            return int(field[field.index(name) + 1]) if name in field else default

        nodes = after("nodes")
        search = after("search_nodes", 0)
        solver = after("solver_nodes", 0)
        if search == 0 and solver == 0:
            search = nodes
        add(agg, (field[1], field[2], field[6]), after("rep"),
            nodes, search, solver, after("time"))
    if not agg:
        fail(f"{path} holds no `row ` lines")
    return agg


def read_blocks(paths):
    """`tools/bench_block.sh` record lines, summed into the same shape.

    The BAND is a reading of the fixture's own `stones` annotation and never a
    position index: `stones 15` is band 15 and any other corpus count is band
    35, which is how WP-1.8c banded these same 24 entries. The trigger-rich
    fixture is one band.
    """
    agg = defaultdict(cell)
    for path in paths:
        text = slurp(path)
        if "bench_block: done:" not in text:
            fail(f"{path} has no `bench_block: done:` line -- that sweep did not complete")
        fixture = seat = None
        for line in text.splitlines():
            if line.startswith("bench_block: fixture "):
                fixture = line.split()[2].rsplit("/", 1)[-1]
            if line.startswith("bench_block: config "):
                seat = "ON" if line.split()[2].endswith("_on.toml") else "OFF"
            if "bench_block: record entry " not in line:
                continue
            if fixture is None or seat is None:
                fail(f"{path}: a record line came before its config/fixture header")
            field = line.split()

            def after(name, default=0):
                return int(field[field.index(name) + 1]) if name in field else default

            stones = field[field.index("stones") + 1]
            bnd = ("trigger" if fixture == "bench_solver_positions_v1.txt"
                   else ("15" if stones == "15" else "35"))
            nodes = after("nodes")
            search = after("search_nodes", 0)
            solver = after("solver_nodes", 0)
            if search == 0 and solver == 0:
                search = nodes
            add(agg, (fixture, seat, bnd), after("rep"), nodes, search, solver, after("time"))
    if not agg:
        fail("no record lines in any of the given sweeps")
    return agg


def ratio_of(a, c, u):
    """nps_ON / nps_OFF at u = solver visits per search node.

    nps_OFF = 1/a; nps_ON = (S+V)/(S.a + V.c). NOTE that with c taken as the
    residual (T_on - S.a)/V this is an IDENTITY equal to nps_ON/nps_OFF for any
    c -- see the banner `main` prints. It is written in this form because the
    INVERSION below needs a and c separated.
    """
    return a * (1.0 + u) / (a + u * c)


def u_required(a, c, target, t=0.0):
    """The u at which the ON seat's ratio equals `target`.

    `t` is the ON-seat-only cost paid at EVERY search node -- the trigger
    predicate, which the OFF seat never evaluates because it holds no solver.
    With it, ratio = a(1+u) / ((a+t) + u.c), inverting to
    u = (a(1-target) - target.t) / (target.c - a).

    The two degenerate branches mean OPPOSITE things and are returned by name so
    neither can be printed as the other.
    """
    denom = target * c - a
    numer = a * (1.0 - target) - target * t
    if denom <= 0:
        return "already"
    if numer <= 0:
        return "unreachable"
    return numer / denom


def branch_text(which):
    if which == "already":
        return ("ALREADY MET at every u (the bound is at or below the u -> infinity "
                "asymptote a/c, so no reduction is needed)")
    return ("UNREACHABLE BY ANY DETECTOR (the per-node trigger cost alone misses the "
            "bound, so gating every call still does not reach it)")


def band(name, off, on, bound):
    """a, c and u for one band. `c` is a RESIDUAL and is labelled one."""
    a = off["time"] * 1000.0 / off["search"]
    u = on["solver"] / on["search"]
    c = (on["time"] * 1000.0 - on["search"] * a) / on["solver"]
    return dict(name=name, a=a, c=c, u=u, bound=bound)


def medians(spot):
    nps = sorted(n * 1000.0 / t for n, t in spot["reps"].values() if t)
    if len(nps) < 2:
        return nps[0] if nps else 0.0, 0.0
    quart = statistics.quantiles(nps, n=4)
    return statistics.median(nps), quart[2] - quart[0]


def main():
    if len(sys.argv) < 2:
        void("usage: stage3_premise_derive.py <bench> [<capsweep>]\n"
             "       stage3_premise_derive.py --blocks <sweep> [<sweep> ...]")
    blocks = sys.argv[1] == "--blocks"
    agg = read_blocks(sys.argv[2:]) if blocks else read_rows(sys.argv[1])

    print("=== inputs, summed from the artifact's own record lines ===")
    rows = []
    for label, fixture, bnd, bound, recorded in WANT:
        off, on = agg.get((fixture, "OFF", bnd)), agg.get((fixture, "ON", bnd))
        if off is None or on is None:
            fail(f"{label}: no {'OFF' if off is None else 'ON'} rows")
        for seat, spot in (("OFF", off), ("ON", on)):
            med, iqr = medians(spot)
            print(f"{label:16s} {seat:<3s} rows {spot['rows']:4d} nodes {spot['nodes']:9d} "
                  f"search {spot['search']:9d} solver {spot['solver']:9d} "
                  f"time_ms {spot['time']:7d}  median nps {med:9.0f} "
                  f"IQR {100 * iqr / med if med else 0:5.2f}% of median")
        rows.append((band(label, off, on, bound), recorded, off, on))

    print()
    print("=== the model, and what checking it against the record DOES NOT show ===")
    print("ratio(u) = a(1+u) / (a + u.c);  a = us per search node (OFF seat),")
    print("c = us per solver visit (ON seat, a RESIDUAL), u = solver visits per")
    print("search node.")
    print()
    print("THIS IS AN IDENTITY, NOT A FIT, AND SAYING SO IS THE POINT. Because c is")
    print("the residual (T_on - S.a)/V, the term a + u.c collapses to T_on/S and the")
    print("whole expression collapses to nps_ON/nps_OFF for ANY c. The agreement")
    print("below therefore corroborates NOTHING about c. What it does exclude is")
    print("narrow and still worth having: WP-1.8c's recorded ratios are ratios of")
    print("per-rep MEDIANS while these are sums over every rep, so agreement says")
    print("the reps are homogeneous and this script's summation has no slip. The")
    print("load in the inversion is carried by the constant-c evidence further down.")
    bad = 0
    for spec, recorded, _off, _on in rows:
        predicted = ratio_of(spec["a"], spec["c"], spec["u"])
        ok = abs(predicted - recorded) <= TOL
        bad += 0 if ok else 1
        print(f"{spec['name']:16s} a {spec['a']:8.4f}  c {spec['c']:8.3f}  u {spec['u']:8.4f}"
              f"  -> ratio {predicted:.5f}  WP-1.8c recorded {recorded:.4f}  "
              f"{'AGREES' if ok else 'DIFFERS'}")
    if bad and not blocks:
        fail(f"{bad} band(s) the model does not reproduce; the derivation below is void")
    if bad:
        print(f"NOTE: {bad} band(s) differ from WP-1.8c's RECORDED ratio. In --blocks")
        print("mode that IS the measurement, not a defect: these rows are a different")
        print("revision and the recorded ratios are WP-1.8c's.")

    print()
    print("=== THE BACKFILL, at its endpoints, with no model and no fitted term ===")
    print("The OFF seat holds no solver, so it IS the gate-everything limit of any")
    print("detector. Per position, the two seats of the SAME bench:")
    for spec, _rec, off, on in rows:
        print(f"{spec['name']:16s} OFF search {off['search'] / off['rows']:9.1f}   "
              f"ON search {on['search'] / on['rows']:8.1f} + solver "
              f"{on['solver'] / on['rows']:9.1f} = {on['nodes'] / on['rows']:9.1f}")
    print("Gating every call turns each second line into the first: the nodes the")
    print("solver stops spending are nodes the SEARCH then spends, because")
    print("`total_nodes = search_nodes + solver_nodes` is what the budget stops on.")

    print()
    print("=== inverted at the REGISTERED bound: the u the bracket demands ===")
    print("t = 0 here, which is the FAVOURABLE assumption -- see the omitted term.")
    for spec, _rec, _off, _on in rows:
        need = u_required(spec["a"], spec["c"], spec["bound"])
        if isinstance(need, str):
            print(f"{spec['name']:16s} bound {spec['bound']:.2f}  {branch_text(need)}")
            continue
        print(f"{spec['name']:16s} bound {spec['bound']:.2f}  u_now {spec['u']:8.4f}"
              f"  u_needed {need:.5f}  FACTOR {spec['u'] / need:9.1f}x")

    print()
    print("=== the two factors a reader must not conflate ===")
    print("A detector is a per-node predicate, so what a design sets is the RATE:")
    print("firings per search node. The absolute count falls by less, because the")
    print("budget the gated calls do not spend is spent on search nodes and the")
    print("survivors fire across a larger tree.")
    print()
    print("THE SECOND COLUMN IS A SOLVER-VISIT FACTOR AND NOT A CALL-COUNT FACTOR.")
    print("calls = V / K with K the mean visits per fired call, and K cancels ONLY")
    print("IF IT IS INVARIANT UNDER GATING -- which a selective detector is exactly")
    print("what would break: keep the hard calls and K rises, keep the cheap proofs")
    print("and K falls. No counter exists to measure K, so it is named as an")
    print("assumption and never divided out silently.")
    for spec, _rec, _off, _on in rows:
        need = u_required(spec["a"], spec["c"], spec["bound"])
        if isinstance(need, str):
            continue
        now = spec["u"] / (1.0 + spec["u"])
        then = need / (1.0 + need)
        print(f"{spec['name']:16s} rate factor {spec['u'] / need:9.1f}x"
              f"   absolute solver-VISIT factor {now / then:7.1f}x"
              f"   (solver node share {now:.4f} -> {then:.4f})")

    print()
    if blocks:
        print("=== no hypothetical-k block in --blocks mode ===")
        print("`a` above is already the revision under measurement, so applying a")
        print("further whole-engine factor would count the same speedup twice.")
    else:
        print("=== the same inversion after a whole-engine nps gain of factor k ===")
        print("A search-node speedup divides a; the df-pn visit does not go through")
        print("pistol-eval, so c is held. Every k is a HYPOTHETICAL, not a measurement.")
        for k in (1.0, 1.5, 1.789, 1.917):
            line = [f"k {k:5.3f}"]
            for spec, _rec, _off, _on in rows:
                need = u_required(spec["a"] / k, spec["c"], spec["bound"])
                line.append(f"{spec['name']}: " + (branch_text(need) if isinstance(need, str)
                                                   else f"{spec['u'] / need:8.1f}x"))
            print("  " + "   ".join(line))

    print()
    print("=== THE OMITTED TERM, whose sign is known ===")
    print("The OFF seat holds no solver, so it never evaluates the trigger:")
    print("`self.solver.is_some()` short-circuits before `solver_verdict()`")
    print("(crates/pistol-search/src/pvs.rs:265-269). The ON seat pays the predicate")
    print("at EVERY search node. That cost, t us/node, is absent from `a` and is")
    print("absorbed by the residual `c`, so every factor above is a FLOOR: any")
    print("t > 0 makes the bracket harder, never easier. t_max is where the bound")
    print("goes unreachable for ANY detector, gating every call included:")
    for spec, _rec, _off, _on in rows:
        t_max = spec["a"] * (1.0 - spec["bound"]) / spec["bound"]
        print(f"{spec['name']:16s} bound {spec['bound']:.2f}  t_max {t_max:7.3f} us/node"
              f"  (= {t_max / spec['a']:.2f} x ONE SEARCH NODE at a = {spec['a']:.4f})")
    print()
    for t in (0.0, 0.03, 0.5, 2.0, 4.0):
        line = [f"t {t:5.2f} us"]
        for spec, _rec, _off, _on in rows:
            need = u_required(spec["a"], spec["c"], spec["bound"], t)
            line.append(f"{spec['name']}: " + ("UNREACHABLE" if isinstance(need, str)
                                               else f"{spec['u'] / need:9.1f}x"))
        print("  " + "   ".join(line))

    if not blocks and len(sys.argv) > 2:
        cap_sweep(sys.argv[2], rows)

    print()
    print("DERIVE_DONE")
    return 0


def cap_sweep(path, rows):
    """The ON-seat cap ladder: read for constant-c, NOT for the backfill."""
    text = slurp(path)
    pat = re.compile(r"^cap (\d+) positions (\d+) nodes (\d+) search (\d+) solver (\d+) "
                     r"f [\d.]+ time_ms (\d+) nps (\d+) ratio_vs_(\d+) ([\d.]+)")
    seen = []
    for line in text.splitlines():
        match = pat.match(line)
        if match:
            seen.append(match.groups())
    if not seen:
        fail(f"{path} holds no `cap ` rows")

    print()
    print("=== reading the cap sweep HONESTLY: it is a PRE-BUDGET-FIX run ===")
    print("docs/experiments/wp18c_design.md:948-951 flags it in the design's own")
    print("words -- the sweep predates the D-463 budget fix, so every ON row")
    print("OVERSHOOTS its budget, and the overshoot GROWS WITH THE CAP:")
    table = {}
    for cap, positions, nodes, search, solver, time_ms, _nps, off_nps, recorded in seen:
        per = int(nodes) / int(positions)
        table[cap] = (int(nodes), int(search), int(solver))
        print(f"  cap {cap:>5s}  {per:9.1f} nodes/position = {per / 50000:.2f}x the 50,000"
              f" budget   solver share {int(solver) / int(nodes):.5f}")
    print("So the raw counts are NOT comparable at fixed budget, and the naive")
    print("reading -- `solver nodes fell 6.6x while search nodes rose 6.7x` -- is in")
    print("the wrong frame. AT A FIXED BUDGET the same two rows say:")
    if "32" in table and "2048" in table:
        n32, s32, v32 = table["32"]
        n20, s20, v20 = table["2048"]
        print(f"  solver share {v20 / n20:.5f} -> {v32 / n32:.5f}"
              f"  = solver {(v20 / n20) / (v32 / n32):5.2f}x DOWN")
        print(f"  search share {s20 / n20:.5f} -> {s32 / n32:.5f}"
              f"  = search {(s32 / n32) / (s20 / n20):5.2f}x UP")
    print("Both corrections cut AGAINST the naive reading and FOR the finding.")

    print()
    print("=== what the sweep IS good for: constant-c over a wide range in u ===")
    us, cs = [], []
    for cap, _positions, _nodes, search, solver, time_ms, _nps, off_nps, recorded in seen:
        a = 1e6 / float(off_nps)
        c = (int(time_ms) * 1000.0 - int(search) * a) / int(solver)
        u = int(solver) / int(search)
        us.append(u)
        cs.append(c)
        predicted = ratio_of(a, c, u)
        ok = abs(predicted - float(recorded)) <= TOL
        print(f"cap {cap:>5s}  a {a:6.3f}  c {c:7.2f}  u {u:8.4f}"
              f"  -> ratio {predicted:.5f}  recorded {float(recorded):.4f}"
              f"  {'AGREES' if ok else 'DIFFERS'}")
    need = u_required(rows[0][0]["a"], rows[0][0]["c"], rows[0][0]["bound"])
    print(f"Across these rows u moves {min(us):.4f} -> {max(us):.4f} "
          f"({max(us) / min(us):.1f}x) on ONE fixture while the residual c moves")
    print(f"{min(cs):.2f} -> {max(cs):.2f} (+/-{100 * (max(cs) - min(cs)) / (2 * statistics.mean(cs)):.0f}%). "
          f"The u the bracket needs is {need:.5f} -- ONE order of")
    print("magnitude below this sweep's floor, not four. THAT is the evidence that")
    print("holding c constant through the inversion is legitimate, and it is the")
    print("only evidence for it here.")


if __name__ == "__main__":
    sys.exit(main())
