"""Turn the sweep corpus into per-side window-count rows, once.

Extraction is the expensive half and it is pure: given the corpus digests, the
row file is a function of them. The trainer reads this and never the corpus, so
a fit is reproducible without re-walking the records.

MATE ROWS ARE KEPT AND LABELLED BY KIND. The row filter of
docs/experiments/wp22_phase1_design.md §3 has two clauses, one on the label and
one on the POSITION, and a filter applied here could not be counted downstream
or varied without re-walking the corpus.
"""

import hashlib
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import features as F

MANIFEST = "artifacts/arc3r_sweep_deduped_manifest.txt"
TRANCHE = "/home/tom/Projects/pistol-corpus/arc3r-sweep/tranche-{}/corpus.txt"

# labels_file.rs:17-50 — the column order, by name so a reader can check it.
MOVES, KEY_FULL, TO_MOVE = 2, 5, 6
SCORE_KIND, SCORE_VALUE, DEPTH, BOOK, RESULT = 7, 8, 10, 13, 14


def digest(path):
    h = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def main(out, manifest=MANIFEST, tranche=TRANCHE):
    wanted = {}
    with open(manifest) as handle:
        for line in handle:
            if line.startswith("#"):
                continue
            row = line.rstrip("\n").split("\t")
            wanted.setdefault(int(row[0]), set()).add((int(row[1]), row[4]))

    written = 0
    with open(out, "w") as sink:
        sink.write("# wp22 texel rows — a1..a6 tab b1..b6 tab label tab to_move "
                   "tab score_kind tab depth tab book tab result tab key_full_sha256\n")
        sink.write(f"# manifest_sha256 {digest(manifest)}\n")
        for index in sorted(wanted):
            path = tranche.format(index)
            sink.write(f"# corpus {index} sha256 {digest(path)}\n")
            with open(path) as handle:
                body = [l.rstrip("\n").split("\t") for l in handle if not l.startswith("#")]
            for record_number, manifest_key_full in sorted(wanted[index]):
                rec = body[record_number - 1]
                # THE JOIN IS VERIFIED ON EVERY ROW, not sampled. A join that
                # addresses the wrong record produces well-formed rows whose
                # features belong to another position, and nothing downstream
                # could tell.
                if rec[KEY_FULL] != manifest_key_full:
                    raise SystemExit(
                        f"extract: corpus {index} record {record_number} has key_full "
                        f"{rec[KEY_FULL][:40]!r}, the manifest says {manifest_key_full[:40]!r}"
                    )
                a, b = F.per_side_counts(F.stones_of(rec[MOVES]))
                sink.write("\t".join([
                    "\t".join(str(a[k]) for k in range(1, 7)),
                    "\t".join(str(b[k]) for k in range(1, 7)),
                    rec[SCORE_VALUE], rec[TO_MOVE], rec[SCORE_KIND], rec[DEPTH],
                    rec[BOOK], rec[RESULT],
                    hashlib.sha256(rec[KEY_FULL].encode()).hexdigest(),
                ]) + "\n")
                written += 1
    print(f"extract: {written} row(s) written, every score_kind KEPT")
    return written


if __name__ == "__main__":
    main(sys.argv[1])
