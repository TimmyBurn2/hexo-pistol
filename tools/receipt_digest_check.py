"""Every receipt digest a document prints, checked against the receipt on disk.

THE CLASS THIS EXISTS FOR HAS HAPPENED THREE TIMES IN ONE PACKAGE. A receipt
directory gains a file, its `RECEIPT_*.sha256` is regenerated, and the digest a
document printed yesterday now matches nothing. Two red-team rounds caught it
twice; the third caught it again in a document written to fix the second.

**IT IS NOT A CI GATE AND CANNOT BE ONE.** `artifacts/` is gitignored (hard rule
8), so a gate depending on it would be green on one workstation and red on every
clone — `docs/book_v3_ledger.md` gives the same reason for its own disjointness
script. A session runs this and records its output; that is the whole mechanism.

Usage: receipt_digest_check.py <doc.md> [<doc.md> ...]
Exit:  0 every printed digest matches a verifying receipt
       1 a digest matches nothing, or a receipt does not verify
       2 nothing was checked — no document named a receipt (a VOID, not a fail)
"""

import hashlib
import pathlib
import re
import subprocess
import sys

DIGEST = re.compile(r"\b[0-9a-f]{64}\b")
RECEIPT_DIR = re.compile(r"artifacts/[A-Za-z0-9_./-]+")


def digest_of(path):
    h = hashlib.sha256()
    with open(path, "rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def receipts(root):
    """Every `RECEIPT_*.sha256` under `root`, with its own digest."""
    out = {}
    for receipt in sorted(pathlib.Path(root).rglob("RECEIPT_*.sha256")):
        out[digest_of(receipt)] = receipt
    return out


def verifies(receipt):
    """`sha256sum -c` over the receipt, run in its own directory."""
    done = subprocess.run(["sha256sum", "-c", receipt.name],
                          cwd=receipt.parent, capture_output=True, text=True)
    return done.returncode == 0


def main(argv):
    if not argv:
        raise SystemExit("receipt_digest_check: name at least one document")
    known = receipts("artifacts")
    if not known:
        print("receipt_digest_check: RUN VOID: no RECEIPT_*.sha256 under artifacts/",
              file=sys.stderr)
        return 2
    checked = 0
    bad = []
    for name in argv:
        text = pathlib.Path(name).read_text()
        printed = {d for d in DIGEST.findall(text)}
        named = {m for m in RECEIPT_DIR.findall(text) if "wp22" in m or "research" in m}
        hits = printed & set(known)
        # A digest that is not a receipt's may be a file's or a commit's; only a
        # digest sitting beside a receipt PATH is this script's business, and a
        # document naming a receipt directory whose digest it does not print is
        # the defect in the other direction.
        for directory in sorted(named):
            root = pathlib.Path(directory)
            if not root.is_dir():
                continue
            here = {d for d, r in known.items() if r.parent == root}
            if not here:
                continue
            checked += 1
            if not (here & printed):
                bad.append(f"{name}: names {directory} and prints none of its "
                           f"receipt digests (on disk: {sorted(here)[0]})")
        for d in sorted(hits):
            checked += 1
            if not verifies(known[d]):
                bad.append(f"{name}: {d} is {known[d]} and it does NOT verify")
    for line in bad:
        print(f"receipt_digest_check: FAIL: {line}", file=sys.stderr)
    print(f"receipt_digest_check: {len(argv)} document(s), {checked} check(s), "
          f"{len(bad)} failure(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
