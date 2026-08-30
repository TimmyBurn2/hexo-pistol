#!/usr/bin/env python3
"""Rank a trigger census's candidate predicates IN VISITS, which is the unit the
bracket fixes.

`tools/stage3_census_analyse.py` reports the census's own quantities — K, the
root's share, and each candidate's KEPT and PROOFS KEPT. It also applied a
REACHES-BUDGET test in FIRINGS, priced at the population mean cost of a firing,
and a DECISION-RED-TEAM struck it: converting a visit budget into a firing
budget is valid only if a predicate's kept firings cost the mean, and the census
records what they actually cost. On band 15 the two tests DISAGREE on the one
predicate that reaches the bracket.

So this instrument answers the same question in the unit `docs/experiments/stage3_rulings.md`
§1.1 names as the one the budget is denominated in:

    VISITS/search = sum over KEPT firings of (att_visits + def_visits) / searches

and compares it against the bracket's own visit budget. The firing figure is
still reported; it is no longer the test.

It also prices the two levers the field forgot:
  * the CAP, by re-charging every invocation at `min(visits, cap)` and counting
    which proofs are still reached;
  * a VERDICT CACHE, by counting repeated firing signatures — an UPPER bound,
    because a signature is evidence of a repeated position and not proof of one.

Usage: stage3_census_rank.py --census <path> --fixture <path> --budget <visits>
                             [--band 15|35|trigger] [--searches N] [--cap N]
Exit:  0 read and ranked
       1 an input is malformed
       2 THE RUN IS VOID -- an input is unreadable
"""

import sys
from collections import Counter

TRIGGER_FIXTURE = "bench_solver_positions_v1.txt"


def void(msg):
    print(f"stage3_census_rank: RUN VOID: {msg}", file=sys.stderr)
    sys.exit(2)


def fail(msg):
    print(f"stage3_census_rank: FAIL: {msg}", file=sys.stderr)
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


def load(census_path, fixture_path):
    bands = bands_of(fixture_path)
    rows = []
    for line in read(census_path).splitlines():
        if not line.startswith("trigger_census: row entry "):
            continue
        fields = line.split()
        spot = {}
        index = 4
        while index + 1 < len(fields):
            spot[fields[index]] = fields[index + 1]
            index += 2
        spot["index"] = int(fields[3])
        if spot["index"] >= len(bands):
            fail(f"{census_path}: entry {spot['index']} is past {fixture_path}'s end")
        spot["band"] = bands[spot["index"]]
        rows.append(spot)
    if not rows:
        fail(f"{census_path} holds no `trigger_census: row` line")
    return rows


def proved(row):
    return row["att_proved"] == "true" or row["def_proved"] == "true"


def visits(row):
    return int(row["att_visits"]) + int(row["def_visits"])


def signature(row):
    """Evidence of a repeated firing POSITION -- not proof of one.

    The census carries no position key, so this is the columns a repeat would
    share. It over-counts if two different positions agree on all of them, which
    makes every cache figure below an UPPER bound and is said so where printed.
    """
    return tuple(
        row[key]
        for key in (
            "index", "turns", "mover_hot", "opp_hot", "mover_w1",
            "opp_w1", "mover_l3", "opp_l3", "att_visits", "def_visits",
        )
    )


CANDIDATES = [
    ("incumbent (any hot, either side)", lambda r: True),
    ("(a/g) opp_hot >= 2", lambda r: int(r["opp_hot"]) >= 2),
    ("(a/g) opp_hot >= 3", lambda r: int(r["opp_hot"]) >= 3),
    ("(a/g) opp_hot >= 4", lambda r: int(r["opp_hot"]) >= 4),
    ("(a) either side >= 2 hot", lambda r: max(int(r["mover_hot"]), int(r["opp_hot"])) >= 2),
    ("(a) a win-in-one-ply, either side",
     lambda r: int(r["mover_w1"]) > 0 or int(r["opp_w1"]) > 0),
    ("(a) mover hot", lambda r: int(r["mover_hot"]) > 0),
    ("(j) not the root", lambda r: int(r["turns"]) > 0),
    ("(j) the root only", lambda r: int(r["turns"]) == 0),
    ("(g)+(j) opp_hot >= 3 and not the root",
     lambda r: int(r["opp_hot"]) >= 3 and int(r["turns"]) > 0),
]


def main(argv):
    census = fixture = None
    budget = None
    searches = None
    band = None
    cap = 2048
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
        elif key == "--fixture":
            fixture = value
        elif key == "--budget":
            budget = float(value)
        elif key == "--band":
            band = value
        elif key == "--searches":
            searches = int(value)
        elif key == "--cap":
            cap = int(value)
        else:
            fail(f"unknown option {key}")
    if census is None or fixture is None or budget is None:
        fail("--census, --fixture and --budget are all required")

    rows = load(census, fixture)
    if band is not None:
        known = {row["band"] for row in rows}
        if band not in known:
            fail(f"--band {band} is not one of {sorted(known)} in this census")
        rows = [row for row in rows if row["band"] == band]
    if searches is None:
        searches = len({row["index"] for row in rows})
    if searches == 0:
        fail("no searches; nothing to rank")

    print(f"=== {census} against {fixture} ===")
    print(f"argv {' '.join(argv)}")
    print(f"band {band or 'all'}  searches {searches}  firings {len(rows)}"
          f"  visit budget {budget:.1f}/search  cap {cap}")
    proving = [r for r in rows if proved(r)]
    total_visits = sum(visits(r) for r in rows)
    print(f"incumbent: {total_visits / searches:9.1f} visits/search"
          f"  proving firings {len(proving)}")
    print()
    print("THE TEST IS IN VISITS. A firing count priced at the population mean is")
    print("not the bracket's unit and the two disagree; the firing column is")
    print("reported and is not the test.")
    print()
    print(f"{'candidate':<38} {'KEPT':>6} {'PROOFS':>7} {'PREC':>7} "
          f"{'visits/search':>14} {'cut':>8} {'firings':>8}  budget")
    for name, predicate in CANDIDATES:
        kept = [r for r in rows if predicate(r)]
        kept_proofs = [r for r in kept if proved(r)]
        kept_visits = sum(visits(r) for r in kept) / searches
        share = len(kept) / len(rows)
        proof_share = len(kept_proofs) / len(proving) if proving else float("nan")
        precision = len(kept_proofs) / len(kept) if kept else float("nan")
        cut = (total_visits / searches / kept_visits) if kept_visits else float("inf")
        verdict = "IN" if kept_visits <= budget else "out"
        print(
            f"{name:<38} {share:6.3f} {proof_share:7.3f} {precision:7.4f} "
            f"{kept_visits:14.1f} {cut:7.2f}x {len(kept) / searches:8.2f}  {verdict}"
        )

    print()
    print("=== (i) THE CAP AS A LEVER: every invocation re-charged at min(visits, cap) ===")
    print("A proof is still REACHED at cap c iff the invocation that found it spent <= c.")
    for candidate_cap in (2048, 1024, 512, 256, 128):
        charged = 0
        reached = 0
        for row in rows:
            att, dfn = int(row["att_visits"]), int(row["def_visits"])
            charged += min(att, candidate_cap) + min(dfn, candidate_cap)
            if row["att_proved"] == "true" and att <= candidate_cap:
                reached += 1
            elif row["def_proved"] == "true" and dfn <= candidate_cap:
                reached += 1
        per_search = charged / searches
        print(
            f"  cap {candidate_cap:5d}  {per_search:9.1f} visits/search"
            f"  {total_visits / searches / per_search:6.2f}x"
            f"  proofs reached {reached}/{len(proving)}"
            f"  {'IN' if per_search <= budget else 'out'}"
        )

    print()
    print("=== (h) A VERDICT CACHE: repeated firing signatures — an UPPER BOUND ===")
    counts = Counter(signature(r) for r in rows)
    distinct = len(counts)
    first_visits = {}
    for row in rows:
        key = signature(row)
        if key not in first_visits:
            first_visits[key] = visits(row)
    cached_visits = sum(first_visits.values()) / searches
    print(f"  distinct signatures {distinct} of {len(rows)} firings"
          f"  = {len(rows) / distinct:.2f} firings per signature")
    print(f"  {cached_visits:9.1f} visits/search if a repeat is free"
          f"  {total_visits / searches / cached_visits:6.2f}x"
          f"  proofs reached {len(proving)}/{len(proving)} BY CONSTRUCTION"
          f"  {'IN' if cached_visits <= budget else 'out'}")
    print("  A SIGNATURE IS EVIDENCE OF A REPEATED POSITION AND NOT PROOF OF ONE:")
    print("  the census carries no position key, so this over-counts if two")
    print("  different positions agree on every column. The figure is an upper")
    print("  bound and the row's kill condition is the counter that settles it.")

    print()
    print("=== COMPOSED: the two recall-preserving levers together ===")
    for label, predicate, candidate_cap in [
        ("opp_hot >= 3 + cache", lambda r: int(r["opp_hot"]) >= 3, None),
        ("cache + cap 512", lambda r: True, 512),
        ("opp_hot >= 3 + cache + cap 512", lambda r: int(r["opp_hot"]) >= 3, 512),
    ]:
        # A CACHE PRESERVES RECALL BY CONSTRUCTION, so the proof count here is
        # over DISTINCT proving signatures and not over rows: a proof proved once
        # is still proved when its repeats are served from the cache, and
        # counting rows would charge the cache for its own saving.
        kept = [r for r in rows if predicate(r)]
        proving_signatures = {signature(r) for r in kept if proved(r)}
        seen, charged, reached_signatures = set(), 0, set()
        for row in kept:
            key = signature(row)
            if key in seen:
                continue
            seen.add(key)
            att, dfn = int(row["att_visits"]), int(row["def_visits"])
            limit = candidate_cap or max(att, dfn)
            charged += min(att, limit) + min(dfn, limit)
            if row["att_proved"] == "true" and att <= limit:
                reached_signatures.add(key)
            elif row["def_proved"] == "true" and dfn <= limit:
                reached_signatures.add(key)
        reached = len(reached_signatures)
        per_search = charged / searches
        print(
            f"  {label:<34} {per_search:9.1f} visits/search"
            f"  {total_visits / searches / per_search:6.2f}x"
            f"  proofs {reached}/{len(proving_signatures)} distinct"
            f" (of {len(proving)} firings)"
            f"  {'IN' if per_search <= budget else 'out'}"
        )
    print("CENSUS_RANK_DONE")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
