"""Draw the three disjoint position samples wp22_cap_prereg.md registers.

Positions are the corpus's OWN move order, fetched by joining the manifest row
to its corpus record. An earlier revision built fixtures from the manifest's
`key_seq` instead — the canonical image — on the argument that the census counts
symmetry-invariant keys so the two are the same census. THAT ARGUMENT IS FALSE
AND IT WAS MEASURED: the same twenty rows censused both ways give 229 against
236 firings and 206 against 210 distinct keys, because the search's tie-breaks
are coordinate-lexicographic and a mirrored board breaks them differently. The
keys are invariant; the search that finds them is not.

So the join is back, and every row's join is VERIFIED rather than trusted.
"""

import hashlib
import pathlib
import sys

MANIFEST = "artifacts/arc3r_sweep_deduped_manifest.txt"
TRANCHE = "/home/tom/pistol-runs/arc3r-sweep/tranche-{}/corpus.txt"
COLUMNS = ("# columns: corpus_index, record_number, key_seq, key_pos, key_full, "
           "depth_turns, result, end")
BODY_ROWS = 89805

# Manifest columns, and the corpus columns they must agree with
# (crates/pistol-arena/src/labels_file.rs:17-50).
M_KEY_POS, M_KEY_FULL = 3, 4
C_MOVES, C_KEY_POS, C_KEY_FULL = 2, 4, 5

# The registered slice boundaries: dry run, then calibration, then census.
DRY_RUN, CALIBRATION = 100, 600


def load():
    header_seen = False
    rows = []
    for line in open(MANIFEST):
        if line.startswith("#"):
            if line.rstrip("\n") == COLUMNS:
                header_seen = True
            continue
        row = line.rstrip("\n").split("\t")
        if len(row) != 8:
            raise SystemExit(f"draw: a manifest body row has {len(row)} columns, not 8")
        rows.append(row)
    if not header_seen:
        raise SystemExit("draw: the manifest does not carry the columns line this draw is written against")
    if len(rows) != BODY_ROWS:
        raise SystemExit(f"draw: the manifest holds {len(rows)} body rows, not {BODY_ROWS}")
    rows.sort(key=lambda r: hashlib.sha256(r[M_KEY_FULL].encode()).hexdigest())
    return rows


def slice_of(rows, which):
    if which == "dryrun":
        return rows[:DRY_RUN]
    if which == "calibration":
        return rows[DRY_RUN:CALIBRATION]
    if which == "census":
        return rows[CALIBRATION:]
    raise SystemExit("draw: which is dryrun, calibration or census")


def main(which, out):
    picked = slice_of(load(), which)
    bodies = {}
    fixture, expected = [], []
    for row in picked:
        index, record_number = int(row[0]), int(row[1])
        if index not in bodies:
            bodies[index] = [l.rstrip("\n").split("\t")
                             for l in open(TRANCHE.format(index)) if not l.startswith("#")]
        record = bodies[index][record_number - 1]
        # BOTH keys, because either alone leaves a shift undetectable on rows
        # that happen to share one of them.
        if record[C_KEY_FULL] != row[M_KEY_FULL] or record[C_KEY_POS] != row[M_KEY_POS]:
            raise SystemExit(
                f"draw: corpus {index} record {record_number} does not carry the manifest's keys"
            )
        moves = record[C_MOVES]
        fixture.append("start moves" if moves == "-" else f"start moves {moves}")
        expected.append(row[M_KEY_FULL])
    pathlib.Path(out).write_text("\n".join(fixture) + "\n")
    pathlib.Path(out + ".expected_key_full").write_text("\n".join(expected) + "\n")
    print(f"draw: {which} {len(picked)} position(s) -> {out}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
