#!/usr/bin/env python3
"""Read a trigger census and answer the questions the option matrix asks of it.

`crates/pistol-search/examples/trigger_census.rs` prints one row per solver
trigger firing. This reads those rows and reports, per band:

  * K -- MEAN VISITS PER INVOCATION, the quantity D-508 had to assume and
    D-510's counters made measurable. Every per-search CALL figure derived from
    the bracket is `visits / (invocations_per_firing x K)`, so K is what turns
    that figure from a bound into a measurement.
  * whether the ROOT fires, how often, and what it costs -- the question
    `docs/experiments/stage3_rulings.md` §1.4 item 2 left open.
  * for each candidate narrowing of the trigger, the fraction of firings it
    KEEPS and the fraction of PROOFS it keeps with them. That pair is the
    matrix's own ranking axis (D-516), and it is computed here rather than
    argued.

Banding is WP-1.8c's registered convention, read from the fixture the census
names: the trigger-rich fixture is one band, and a corpus entry is band 15 iff
its own `stones` annotation reads 15.

Usage: stage3_census_analyse.py --census <path> --fixture <path> [...]
Exit:  0 read and analysed
       1 an input is malformed
       2 THE RUN IS VOID -- an input is unreadable
"""

import sys

TRIGGER_FIXTURE = "bench_solver_positions_v1.txt"


def void(msg):
    print(f"stage3_census_analyse: RUN VOID: {msg}", file=sys.stderr)
    sys.exit(2)


def fail(msg):
    print(f"stage3_census_analyse: FAIL: {msg}", file=sys.stderr)
    sys.exit(1)


def read(path):
    try:
        with open(path, encoding="utf-8") as handle:
            return handle.read()
    except OSError as err:
        void(f"cannot read {path}: {err}")


def pairs(fields, start):
    out = {}
    index = start
    while index + 1 < len(fields):
        out[fields[index]] = fields[index + 1]
        index += 2
    return out


def bands_of(fixture_path):
    """Each entry's band, in the order the census walks them."""
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
    entries, rows = [], []
    for line in read(census_path).splitlines():
        fields = line.split()
        if line.startswith("trigger_census: entry "):
            spot = pairs(fields, 3)
            spot["index"] = int(fields[2])
            entries.append(spot)
        elif line.startswith("trigger_census: row entry "):
            spot = pairs(fields, 4)
            spot["index"] = int(fields[3])
            rows.append(spot)
    if not entries:
        fail(f"{census_path} holds no `trigger_census: entry` line")
    if len(entries) != len(bands):
        fail(
            f"{census_path} has {len(entries)} entries but {fixture_path} has "
            f"{len(bands)} -- the census and the fixture are not the same workload"
        )
    for spot in entries:
        spot["band"] = bands[spot["index"]]
    for spot in rows:
        spot["band"] = bands[spot["index"]]
    return entries, rows


def proved(row):
    return row["att_proved"] == "true" or row["def_proved"] == "true"


def visits(row):
    return int(row["att_visits"]) + int(row["def_visits"])


# Each candidate narrowing of the trigger, as a predicate over the columns a
# per-node detector could read. The names are the matrix's row letters.
CANDIDATES = [
    ("incumbent (any hot, either side)", lambda r: True),
    ("(a) both sides hot", lambda r: int(r["mover_hot"]) > 0 and int(r["opp_hot"]) > 0),
    ("(a) mover hot only", lambda r: int(r["mover_hot"]) > 0),
    ("(a) opponent hot only", lambda r: int(r["opp_hot"]) > 0),
    ("(a) a win-in-one-ply, either side",
     lambda r: int(r["mover_w1"]) > 0 or int(r["opp_w1"]) > 0),
    ("(a) mover has >= 2 hot windows", lambda r: int(r["mover_hot"]) >= 2),
    ("(a) either side has >= 2 hot windows",
     lambda r: int(r["mover_hot"]) >= 2 or int(r["opp_hot"]) >= 2),
    ("(a) mover hot and >= 2 live threes", lambda r: int(r["mover_hot"]) > 0
     and int(r["mover_l3"]) >= 2),
    ("(b) opponent hot and mover not (a must-block shape)",
     lambda r: int(r["opp_hot"]) > 0 and int(r["mover_hot"]) == 0),
    ("(e) root only", lambda r: int(r["turns"]) == 0),
    ("(e) turns <= 1", lambda r: int(r["turns"]) <= 1),
]


def main(argv):
    census = fixture = None
    index = 0
    while index < len(argv):
        key = argv[index]
        if key in ("--census", "--fixture"):
            index += 1
            if index >= len(argv):
                fail(f"{key} wants a value")
            if key == "--census":
                census = argv[index]
            else:
                fixture = argv[index]
        else:
            fail(f"unknown option {key}")
        index += 1
    if census is None or fixture is None:
        fail("--census and --fixture are both required")

    entries, rows = load(census, fixture)
    print(f"=== {census} against {fixture} ===")
    print(f"argv {' '.join(argv)}")

    for band in sorted({spot["band"] for spot in entries}, key=lambda b: (b == "trigger", b)):
        band_entries = [e for e in entries if e["band"] == band]
        band_rows = [r for r in rows if r["band"] == band]
        firings = sum(int(e["firings"]) for e in band_entries)
        invocations = sum(int(e["invocations"]) for e in band_entries)
        proofs = sum(int(e["proofs"]) for e in band_entries)
        solver_nodes = sum(int(e["solver_nodes"]) for e in band_entries)
        root_nodes = sum(int(e["root_nodes"]) for e in band_entries)
        root_fired = sum(1 for e in band_entries if int(e["root_nodes"]) > 0)
        if invocations == 0 or firings == 0:
            print(f"band {band:>7}: no firing at all in {len(band_entries)} entries")
            continue
        print()
        print(f"band {band:>7}  entries {len(band_entries)}  census rows {len(band_rows)}")
        print(
            f"  firings {firings} ({firings / len(band_entries):.2f}/search)"
            f"  invocations {invocations} ({invocations / firings:.3f}/firing)"
            f"  proofs {proofs}"
        )
        print(
            f"  MEASURED K {solver_nodes / invocations:8.1f} visits/invocation"
            f"   one firing costs {solver_nodes / firings:8.1f} visits"
        )
        print(
            f"  ROOT fired in {root_fired}/{len(band_entries)} searches"
            f"  root visits {root_nodes} = {100.0 * root_nodes / solver_nodes:5.2f}% of the band's"
            f"  ({root_nodes / len(band_entries):.0f}/search)"
        )
        proving = [r for r in band_rows if proved(r)]
        print(f"  proving firings {len(proving)} of {len(band_rows)}"
              f" = {100.0 * len(proving) / len(band_rows):5.2f}% precision at the incumbent")
        print("  candidate                                              KEPT   PROOFS KEPT  PRECISION")
        for name, predicate in CANDIDATES:
            kept = [r for r in band_rows if predicate(r)]
            kept_proofs = [r for r in kept if proved(r)]
            keep_share = len(kept) / len(band_rows)
            proof_share = (len(kept_proofs) / len(proving)) if proving else float("nan")
            precision = (len(kept_proofs) / len(kept)) if kept else float("nan")
            print(
                f"  {name:<52} {keep_share:6.3f} {proof_share:12.3f} {precision:10.4f}"
            )
    print("CENSUS_ANALYSE_DONE")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
