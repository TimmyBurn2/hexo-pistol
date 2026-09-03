#!/usr/bin/env python3
#
# Every `<doc>.md` revision N a GOVERNING document states is the revision that
# document's own title line holds (docs/decisions.md D-599, D-600, D-601, D-602).
#
# WHY THIS EXISTS, AND WHY GATE 20 CANNOT DO IT. `tools/design_citation_check.py`
# refuses a `path` or a `path:line` the tree does not hold. A "revision N" is
# neither: it is a claim about ANOTHER document's title line, and the citation
# that keeps going stale is written as a bare basename (`` `wp21_prereg.md` ``)
# with no directory prefix, so it does not even match that checker's path
# pattern. The class is doubly invisible to it, and it was caught five separate
# times by fresh reviewers reading two files each — the labour D-582 says a
# mechanism should take off a reviewer once the class is mechanical.
#
# THE RULE IT ENFORCES is narrow on purpose: only citations BETWEEN documents on
# gate 20's own GOVERNING list are checked, because both sides are then current
# and a mismatch is always a defect. A document that cites a revision of
# something not on that list is citing a record, and a record's citations were
# true when written.
#
# EXEMPTIONS ARE NAMED, WITH THEIR REASON, AND NEVER INFERRED. A document whose
# review gate has CLOSED keeps the citations it closed with: its later revisions
# carry implementation obligations only, so its GOVERNING block is a record of
# what governed it, not a live claim. `--exempt <path>` says so per document.
# The exemption is by that reason and not by a disclaimer's reach: the design's
# own "citations are at <commit>" pin covers its `file:line` code citations and
# says nothing about its `revision N` document citations (D-600).
#
# Usage: revision_citation_check.py [--exempt <path>]... <document> [<document>...]
# Exit:  0 every revision citation between listed documents reproduces
#        1 one does not
#        2 THE RUN IS VOID -- a document is unreadable or states no revision
#          (tools/SHELL_CHECKLIST.md item 12). A void is not a failure.

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# THE TWO CITING FORMS, and both are in this tree: `` `name.md` revision 7 ``
# and `` revision 7 of `name.md` ``. Matching only the first leaves the second
# INVISIBLE rather than merely unflagged, which is the shape of a check that
# reports zero because it looked for the wrong thing. The name may be a bare
# basename or a path; both normalise to a basename before lookup, because the
# same document is cited both ways here.
CITATION = re.compile(
    r"`(?P<name>[A-Za-z0-9_./-]+\.md)`\s+revision\s+(?P<revision>\d+)"
    r"|revision\s+(?P<revision2>\d+)\s+of\s+`(?P<name2>[A-Za-z0-9_./-]+\.md)`",
    re.IGNORECASE,
)

# The document's own claim about itself, on its first line:
# `# WP-2.1 — the production label sweep. RUN REGISTRATION, revision 9.`
TITLE_REVISION = re.compile(r"revision\s+(?P<revision>\d+)", re.IGNORECASE)


def void(message):
    print(f"revision_citation_check: RUN VOID: {message}", file=sys.stderr)
    print(
        "revision_citation_check: no answer was taken; this is NOT a failure",
        file=sys.stderr,
    )
    sys.exit(2)


def title_revision(path):
    """The revision a document's own first line states, or None if it states none.

    Read from the FIRST line only. A document states its revision once, in its
    title; a `revision N` further down is a citation of something else, and
    reading those would make a document its own referent.

    AND A CITATION ON THE TITLE LINE IS NOT THE TITLE'S OWN REVISION. A title
    that names another document before naming itself would otherwise hand this
    checker that other document's number as the truth every later comparison is
    made against — an exit-0 wrong answer, and the one a checker must not have.
    So every citing form is struck from the line first, and what remains is the
    document's own claim.
    """
    try:
        first = path.read_text(encoding="utf-8").split("\n", 1)[0]
    except OSError as why:
        void(f"cannot read {path}: {why}")
    except UnicodeDecodeError as why:
        void(f"{path} is not UTF-8: {why}")
    found = TITLE_REVISION.search(CITATION.sub("", first))
    return int(found.group("revision")) if found else None


def check(document, governing, exempt):
    """Every citation in `document` of a revision of another governing document."""
    path = ROOT / document
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as why:
        void(f"cannot read {document}: {why}")
    except UnicodeDecodeError as why:
        void(f"{document} is not UTF-8: {why}")
    seen, bad = 0, []
    for match in CITATION.finditer(text):
        raw = match.group("name") or match.group("name2")
        cited_revision = match.group("revision") or match.group("revision2")
        name = Path(raw).name
        if name not in governing or name == Path(document).name:
            continue
        seen += 1
        cited = int(cited_revision)
        actual = governing[name]
        if actual is None:
            void(f"{governing_path[name]} states no revision on its first line")
        if cited != actual:
            bad.append((match.group(0).strip(), name, cited, actual))
    return seen, bad


def main(argv):
    exempt, documents = set(), []
    index = 0
    while index < len(argv):
        if argv[index] == "--exempt":
            if index + 1 >= len(argv):
                void("--exempt wants a path")
            exempt.add(argv[index + 1])
            index += 2
            continue
        documents.append(argv[index])
        index += 1
    if not documents:
        void("name at least one document")

    # TWO GOVERNING DOCUMENTS SHARING A BASENAME WOULD SHADOW EACH OTHER, and a
    # citation of the shadowed one would be compared against the other's title —
    # an exit-0 wrong answer. A collision is a VOID, loudly, rather than an
    # answer taken from whichever document was listed last.
    global governing_path
    governing_path = {}
    for document in documents:
        name = Path(document).name
        if name in governing_path and governing_path[name] != document:
            void(
                f"two governing documents share the basename `{name}` "
                f"({governing_path[name]} and {document}); a citation of either "
                f"would be checked against the other"
            )
        governing_path[name] = document
    governing = {Path(d).name: title_revision(ROOT / d) for d in documents}

    failed = False
    for document in documents:
        if document in exempt:
            print(f"{document}: EXEMPT, its citations are not checked")
            continue
        seen, bad = check(document, governing, exempt)
        print(f"{document}: {seen} revision citation(s) checked, {len(bad)} stale")
        for quoted, name, cited, actual in bad:
            print(f"  {quoted} — {name} is at revision {actual}")
            failed = True
    print("A GREEN RUN MEANS EVERY CITED REVISION IS THE CITED DOCUMENT'S OWN:")
    print("this instrument reads first lines, not content, and says nothing about")
    print("whether the revision it names is the one the claim needed.")
    print("REVISION_CITATION_CHECK_DONE")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
