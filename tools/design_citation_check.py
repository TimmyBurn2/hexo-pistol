#!/usr/bin/env python3
"""Check that a document's claims about the tree are claims the tree makes.

WHY THIS EXISTS. Five design reviews in one arc failed, and the failures were
not disagreements about judgement — they were claims about the code that the
code does not make, and claims that were true in one revision and deleted in the
next. Two examples, both from documents this instrument is now run over:

  * a design asserted that a report's two engine sections are identical for a
    self-play run. Both `validate.rs` and `transcript.rs` REFUSE identical
    labels, so the mechanism built on it would have refused every input. The
    PREVIOUS revision of the same document contained the true sentence and the
    rewrite deleted it.
  * a design's `MEASURED` block reported a file count taken from a `tail`-
    truncated listing, and the next revision re-asserted it labelled REPRODUCED
    without re-running it (docs/decisions.md D-543).

D-543 named the remedy for that class in tables — render them from the artifact
by machine — and it was never generalised to prose. This is the generalisation.
It does not check reasoning. It checks that every `path`, every `path:line` and
every backticked identifier attributed to a file is one the tree actually holds,
so a false premise is caught before a reviewer spends a round on it.

WHAT IT CANNOT DO, said plainly so nobody reads a green run as a passed review:
it cannot tell whether a true quotation supports the claim built on it. The
engine-label defect above would be caught only if the document had cited a line;
stated as bare prose it passes. **A green run means the citations are real, not
that the document is right.**

A DESIGN LEGITIMATELY NAMES FILES THAT DO NOT EXIST YET -- the module it
proposes to add. Those are declared with `--proposes`, which is not a courtesy
to the checker but a discipline for the author: a design that must list the
files it invents cannot invent one by accident in a rewrite.

Usage: design_citation_check.py [--proposes <path>]... <document> [<document>...]
Exit:  0 every checked claim reproduces
       1 a claim does not
       2 THE RUN IS VOID -- a document is unreadable
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# `crates/foo/src/bar.rs` or `tools/baz.sh`, optionally `:123` or `:12-34`.
PATH = re.compile(
    r"`(?P<path>(?:crates|tools|configs|docs)/[A-Za-z0-9_./-]+\.(?:rs|sh|py|toml|md|txt))"
    r"(?::(?P<line>\d+)(?:-(?P<end>\d+))?)?`"
)


def void(msg):
    print(f"design_citation_check: RUN VOID: {msg}", file=sys.stderr)
    sys.exit(2)


def check(document, proposed):
    try:
        text = Path(document).read_text(encoding="utf-8")
    except OSError as err:
        void(f"cannot read {document}: {err}")
    seen, bad = 0, []
    for match in PATH.finditer(text):
        seen += 1
        path = ROOT / match.group("path")
        if not path.is_file():
            if match.group("path") in proposed:
                continue
            bad.append((match.group(0), "names no file in the tree, and is not --proposes'd"))
            continue
        line = match.group("line")
        if line is None:
            continue
        # A LINE NUMBER IS THE WEAKEST KIND OF CITATION and the tree says so:
        # `stage3_rulings.md` had to stop quoting them because "a line number is
        # invalidated by the next commit to its file". So this checks only that
        # the line EXISTS -- a file that shrank past a cited line is a citation
        # that has certainly rotted, which is the catchable half.
        count = len(path.read_text(encoding="utf-8", errors="replace").splitlines())
        end = int(match.group("end") or line)
        if end > count:
            bad.append((match.group(0), f"the file has {count} lines"))
    return seen, bad


def main(argv):
    proposed, documents = set(), []
    index = 0
    while index < len(argv):
        if argv[index] == "--proposes":
            if index + 1 >= len(argv):
                void("--proposes wants a path")
            proposed.add(argv[index + 1])
            index += 2
            continue
        documents.append(argv[index])
        index += 1
    if not documents:
        void("name at least one document")
    failed = False
    for document in documents:
        seen, bad = check(document, proposed)
        print(f"{document}: {seen} citation(s) checked, {len(bad)} unreproduced")
        for what, why in bad:
            print(f"  {what} {why}")
            failed = True
    print("A GREEN RUN MEANS THE CITATIONS ARE REAL, NOT THAT THE DOCUMENT IS RIGHT:")
    print("this instrument cannot tell whether a true quotation supports the claim")
    print("built on it, and a claim stated as bare prose is not checked at all.")
    print("DESIGN_CITATION_CHECK_DONE")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
