#!/usr/bin/env python3
"""Rank a trigger census's candidate predicates IN VISITS, which is the unit the
bracket fixes, and under ONE recall definition, which is the direction the gate
pins.

THE RECALL DEFINITION, STATED ONCE. D-512's fixture is seven positions the
solver proves a WIN on for the side to move -- the ATTACKER direction,
`Solver::solve`. A census row also records the DEFENDER direction
(`solve_defender`): whether the OPPONENT forces a win against the mover, which
is a proven LOSS for the mover and a different quantity. Revision 1 of this
instrument scored `att_proved or def_proved` and called the result PROOFS KEPT.
On the corpus band-15 fixture that denominator is ENTIRELY defender-direction,
so the row it selected was chosen on losses and quoted against a gate that pins
wins. The two are separate columns here and are never summed: WINS is the gate's
recall, LOSSES is reported beside it, and a row's verdict reads only WINS.

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

Usage: stage3_census_rank.py --census <path> --fixture <path>
                             (--budget <visits> | --off <off-census> --share <frac>)
                             [--band 15|35|trigger] [--searches N] [--cap N]

`--off` + `--share` DERIVES the budget rather than taking it: the share is the
bracket-intrinsic figure (4.352 / 2.242 / 5.696 percent) and the OFF census
supplies that band's own `T_off`. A budget measured on one position set and
carried onto another is a borrowed denominator, which is what D-477 forbids.
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


def totals_of(off_census_path, fixture_path):
    """Each entry's OFF-seat node total, by band -- the `T_off` a budget is a
    share of."""
    bands = bands_of(fixture_path)
    totals = {}
    for line in read(off_census_path).splitlines():
        if not line.startswith("trigger_census: entry "):
            continue
        fields = line.split()
        index = int(fields[2])
        spot = {}
        at = 3
        while at + 1 < len(fields):
            spot[fields[at]] = fields[at + 1]
            at += 2
        if int(spot["solver_nodes"]) != 0:
            fail(f"{off_census_path}: entry {index} spent solver nodes; that is not an OFF seat")
        if index >= len(bands):
            fail(f"{off_census_path}: entry {index} is past {fixture_path}'s end")
        totals.setdefault(bands[index], []).append(int(spot["search_nodes"]))
    if not totals:
        fail(f"{off_census_path} holds no `trigger_census: entry` line")
    return totals


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


def won(row):
    """The ATTACKER direction proved: the side to move forces a win.

    This is the direction D-512's value fixture is stated in and the only one
    the recall gate reads.
    """
    return row["att_proved"] == "true"


def lost(row):
    """The DEFENDER direction proved: the mover is lost against best play.

    Recorded, reported, and never added to [`won`] -- summing them is the
    conflation this instrument's header exists to refuse.
    """
    return row["def_proved"] == "true"


def proved(row):
    """Either direction. Kept ONLY for the cache and signature figures, whose
    question is "would this firing be re-answered from a cache" -- a question
    about the firing and not about the direction its answer came from."""
    return won(row) or lost(row)


def visits(row):
    return int(row["att_visits"]) + int(row["def_visits"])


def signature(row):
    """Evidence of a repeated firing POSITION -- not proof of one.

    The census carries no position key, so this is the columns a repeat would
    share. It over-counts if two different positions agree on all of them, which
    makes every cache figure below an UPPER bound and is said so where printed.
    """
    keys = (
        "index", "turns", "mover_hot", "opp_hot", "mover_w1",
        "opp_w1", "mover_l3", "opp_l3", "cover", "covers",
        "att_visits", "def_visits",
    )
    # `cover`/`covers` are absent from a census taken before the column landed.
    # Reading them as a constant there keeps the OLD artifacts parseable and
    # keeps the signature of a NEW one strictly finer, which is the direction
    # that cannot inflate a cache figure.
    return tuple(row.get(key, "-") for key in keys)


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
    # ROW (b) — the mechanism §3(b) describes and revision 1 scored a substitute
    # for. It needs the `cover` column, which is why the census was re-run.
    ("(b) unanswerable: cover impossible", lambda r: r["cover"] == "impossible"),
    ("(b') answerable one way only", lambda r: r["cover"] == "minimal" and int(r["covers"]) == 1),
    ("(b'') cover impossible or one cover",
     lambda r: r["cover"] == "impossible"
     or (r["cover"] == "minimal" and int(r["covers"]) == 1)),
    # The WIN direction's own shape, read off the census: every attacker proof
    # in the in-sample run is either a mover-hot root win or a position where
    # the mover holds many live threes.
    ("(m) mover hot or mover_l3 >= 9",
     lambda r: int(r["mover_hot"]) > 0 or int(r["mover_l3"]) >= 9),
]


def scored(rows, predicate):
    """A candidate's rows, refusing a predicate this census cannot express.

    A row (b) predicate over a census with no `cover` column is the defect the
    matrix's §4.2 named: it scores a SUBSTITUTE and reads as though it scored the
    mechanism. Refusing is the only answer that cannot be misread (rule 3).
    """
    try:
        return [r for r in rows if predicate(r)]
    except KeyError as missing:
        return missing


def main(argv):
    census = fixture = None
    budget = None
    searches = None
    band = None
    cap = 2048
    off = None
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
        elif key == "--off":
            off = value
        elif key == "--share":
            share = float(value)
        else:
            fail(f"unknown option {key}")
    if census is None or fixture is None:
        fail("--census and --fixture are both required")
    derived_from = None
    if budget is None:
        if off is None or share is None:
            fail("give --budget, or --off and --share so the budget is derived")
        if band is None:
            fail("--off and --share derive a BAND's budget; name the band")
        totals = totals_of(off, fixture)
        if band not in totals:
            fail(f"--band {band} is not one of {sorted(totals)} in {off}")
        mean_off = sum(totals[band]) / len(totals[band])
        budget = share * mean_off
        derived_from = (off, share, mean_off, len(totals[band]))
        # A SEARCH THAT FIRES NOTHING IS STILL A SEARCH. Counting only the
        # entries that produced a row divides the visits by a denominator the
        # budget was not taken over -- the same borrowed-denominator defect one
        # level down, and it bit on the out-of-sample band 35, where one of
        # twelve entries never fires.
        if searches is None:
            searches = len(totals[band])
    elif off is not None or share is not None:
        fail("--budget and --off/--share are two answers to one question")

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
    firing_searches = len({row["index"] for row in rows})
    if firing_searches != searches:
        print(f"NOTE: {searches - firing_searches} of {searches} searches in this band"
              f" fire the trigger never. They are IN the denominator: a search that"
              f" spends no solver visit is a search the budget was taken over.")

    print(f"=== {census} against {fixture} ===")
    print(f"argv {' '.join(argv)}")
    print(f"band {band or 'all'}  searches {searches}  firings {len(rows)}"
          f"  visit budget {budget:.1f}/search  cap {cap}")
    if derived_from:
        off_path, share_used, mean_off, entries = derived_from
        print(f"BUDGET DERIVED: share {share_used * 100:.3f}% x T_off {mean_off:.1f}"
              f"  (mean over {entries} entries of {off_path})")
    else:
        print("BUDGET GIVEN on the command line.")
    wins = [r for r in rows if won(r)]
    losses = [r for r in rows if lost(r)]
    proving = [r for r in rows if proved(r)]
    total_visits = sum(visits(r) for r in rows)
    incumbent_per_search = total_visits / searches
    print(f"incumbent: {incumbent_per_search:9.1f} visits/search"
          f"  firings {len(rows)}"
          f"  REQUIRED CUT {incumbent_per_search / budget:.2f}x")
    print(f"RECALL DENOMINATORS, SEPARATE: WINS (attacker direction, the gate's)"
          f" {len(wins)}   LOSSES (defender direction) {len(losses)}"
          f"   either {len(proving)}")
    if not wins:
        print("  *** THIS BAND HAS NO WIN-DIRECTION PROOF AT ALL. Every WINS cell")
        print("  *** below is 0/0 and is printed as `-`: no row's recall is")
        print("  *** MEASURED here, and a row that looks perfect is vacuous.")
    print()
    print("THE TEST IS IN VISITS. A firing count priced at the population mean is")
    print("not the bracket's unit and the two disagree; the firing column is")
    print("reported and is not the test.")
    print("THE VERDICT READS `WINS`. `LOSSES` is beside it and is never added in.")
    print()
    print(f"{'candidate':<38} {'KEPT':>6} {'WINS':>7} {'LOSSES':>7} {'PREC':>7} "
          f"{'visits/search':>14} {'cut':>8} {'firings':>8}  budget")
    for name, predicate in CANDIDATES:
        kept = scored(rows, predicate)
        if isinstance(kept, KeyError):
            print(f"{name:<38} UNSCORABLE: this census has no {kept} column")
            continue
        kept_wins = [r for r in kept if won(r)]
        kept_losses = [r for r in kept if lost(r)]
        kept_visits = sum(visits(r) for r in kept) / searches
        share = len(kept) / len(rows)
        win_share = f"{len(kept_wins) / len(wins):7.3f}" if wins else "      -"
        loss_share = f"{len(kept_losses) / len(losses):7.3f}" if losses else "      -"
        precision = len(kept_wins) / len(kept) if kept else float("nan")
        cut = (total_visits / searches / kept_visits) if kept_visits else float("inf")
        verdict = "IN" if kept_visits <= budget else "out"
        if not kept:
            verdict = "IN (vacuous)"
        print(
            f"{name:<38} {share:6.3f} {win_share} {loss_share} {precision:7.4f} "
            f"{kept_visits:14.1f} {cut:7.2f}x {len(kept) / searches:8.2f}  {verdict}"
        )

    print()
    print("=== (i) THE CAP AS A LEVER: every invocation re-charged at min(visits, cap) ===")
    print("A proof is still REACHED at cap c iff the invocation that found it spent <= c.")
    for candidate_cap in (4096, 2048, 1024, 512, 256, 128):
        charged = 0
        won_reached = 0
        lost_reached = 0
        for row in rows:
            att, dfn = int(row["att_visits"]), int(row["def_visits"])
            charged += min(att, candidate_cap) + min(dfn, candidate_cap)
            if won(row) and att <= candidate_cap:
                won_reached += 1
            if lost(row) and dfn <= candidate_cap:
                lost_reached += 1
        per_search = charged / searches
        print(
            f"  cap {candidate_cap:5d}  {per_search:9.1f} visits/search"
            f"  {total_visits / searches / per_search:6.2f}x"
            f"  WINS {won_reached}/{len(wins)}  losses {lost_reached}/{len(losses)}"
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
          f"  WINS {len(wins)}/{len(wins)} BY CONSTRUCTION"
          f"  {'IN' if cached_visits <= budget else 'out'}")
    print("  A SIGNATURE IS EVIDENCE OF A REPEATED POSITION AND NOT PROOF OF ONE:")
    print("  the census carries no position key, so this over-counts if two")
    print("  different positions agree on every column. The figure is an upper")
    print("  bound and the row's kill condition is the counter that settles it.")

    print()
    print("=== (n) GATE THE DIRECTION: ask the attacker only ===")
    print("A row on the INVOCATION axis, not the firing axis. `att_proved` is")
    print("decided before `solve_defender` is reached, so dropping the defender")
    print("direction costs ZERO win recall BY CONSTRUCTION — and gives up every")
    print("proven LOSS, which is the whole of its kill condition.")
    attacker_only = sum(int(r["att_visits"]) for r in rows) / searches
    print(f"  attacker only    {attacker_only:9.1f} visits/search"
          f"  {total_visits / searches / attacker_only:6.2f}x"
          f"  WINS {len(wins)}/{len(wins)} by construction"
          f"  losses given up {len(losses)}"
          f"  {'IN' if attacker_only <= budget else 'out'}")
    print(f"  the defender direction is {1 - attacker_only * searches / total_visits:.1%}"
          f" of this band's solver visits")

    print()
    print("=== COMPOSED: the recall-preserving levers together ===")
    print("`att-only` marks a composition carrying row (n).")
    for entry in [
        ("opp_hot >= 3 + cache", lambda r: int(r["opp_hot"]) >= 3, None),
        ("cache + cap 512", lambda r: True, 512),
        ("opp_hot >= 3 + cache + cap 512", lambda r: int(r["opp_hot"]) >= 3, 512),
        ("cache + cap 4096", lambda r: True, 4096),
        ("(m) mover-side + cache",
         lambda r: int(r["mover_hot"]) > 0 or int(r["mover_l3"]) >= 9, None),
        ("(n) att-only + cache", lambda r: True, None, True),
        ("(n) att-only + cache + cap 512", lambda r: True, 512, True),
        ("(m) + (n) att-only + cache",
         lambda r: int(r["mover_hot"]) > 0 or int(r["mover_l3"]) >= 9, None, True),
    ]:
        label, predicate, candidate_cap = entry[:3]
        # Row (n) is a lever on the INVOCATION axis, so it composes by changing
        # what a kept firing COSTS rather than which firings are kept.
        att_only = len(entry) > 3 and entry[3]
        # A CACHE PRESERVES RECALL BY CONSTRUCTION, so the proof count here is
        # over DISTINCT proving signatures and not over rows: a proof proved once
        # is still proved when its repeats are served from the cache, and
        # counting rows would charge the cache for its own saving.
        kept = [r for r in rows if predicate(r)]
        win_signatures = {signature(r) for r in kept if won(r)}
        all_win_signatures = {signature(r) for r in rows if won(r)}
        seen, charged, reached_signatures = set(), 0, set()
        for row in kept:
            key = signature(row)
            if key in seen:
                continue
            seen.add(key)
            att, dfn = int(row["att_visits"]), int(row["def_visits"])
            if att_only:
                dfn = 0
            limit = candidate_cap or max(att, dfn)
            charged += min(att, limit) + min(dfn, limit)
            if won(row) and att <= limit:
                reached_signatures.add(key)
        reached = len(reached_signatures)
        per_search = charged / searches
        cut = total_visits / searches / per_search if per_search else float("inf")
        recall = (
            f"{reached}/{len(all_win_signatures)} distinct WINS"
            if all_win_signatures
            else "no win denominator"
        )
        print(
            f"  {label:<34} {per_search:9.1f} visits/search"
            f"  {cut:6.2f}x"
            f"  {recall}"
            f" (kept {len(win_signatures)} of them before the cap)"
            f"  {'IN' if per_search <= budget else 'out'}"
        )
    print("CENSUS_RANK_DONE")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
