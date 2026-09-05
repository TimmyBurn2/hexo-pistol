"""Turn the sweep corpus into (features, label) rows, once.

Extraction is the expensive half and it is pure: given the corpus digests, the
row file is a function of them. The trainer reads this and never the corpus, so
a fit is reproducible without re-walking 225 MB of records.
"""

import hashlib
import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).parent))
import features as F

MANIFEST = "artifacts/arc3r_sweep_deduped_manifest.txt"
TRANCHE = "/home/tom/pistol-runs/arc3r-sweep/tranche-{}/corpus.txt"

# labels_file.rs:17-50 — the column order, by name so a reader can check it.
MOVES, KEY_FULL, TO_MOVE = 2, 5, 6
SCORE_KIND, SCORE_VALUE, DEPTH, BOOK, RESULT, END = 7, 8, 10, 13, 14, 15


def digest(path):
    h = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def main(out):
    manifest_rows = [l.rstrip("\n").split("\t")
                     for l in open(MANIFEST) if not l.startswith("#")]
    wanted = {}
    for row in manifest_rows:
        wanted.setdefault(int(row[0]), set()).add((int(row[1]), row[4]))

    written, skipped_kind = 0, 0
    with open(out, "w") as sink:
        sink.write("# wp22 texel rows — f1..f6 tab label tab to_move tab depth "
                   "tab book tab result tab key_full_sha256\n")
        sink.write(f"# manifest_sha256 {digest(MANIFEST)}\n")
        for index in sorted(wanted):
            path = TRANCHE.format(index)
            sink.write(f"# corpus {index} sha256 {digest(path)}\n")
            body = [l.rstrip("\n").split("\t")
                    for l in open(path) if not l.startswith("#")]
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
                if rec[SCORE_KIND] != "eval":
                    skipped_kind += 1
                    continue
                f = F.features(F.stones_of(rec[MOVES]))
                key_digest = hashlib.sha256(rec[KEY_FULL].encode()).hexdigest()
                sink.write("\t".join([
                    "\t".join(str(f[k]) for k in range(1, 7)),
                    rec[SCORE_VALUE], rec[TO_MOVE], rec[DEPTH],
                    rec[BOOK], rec[RESULT], key_digest,
                ]) + "\n")
                written += 1
    print(f"extract: {written} row(s) written, {skipped_kind} skipped as not score_kind eval")


if __name__ == "__main__":
    main(sys.argv[1])
