# Overnight arc — the export receipt's anchor

**Why this file exists.** `artifacts/` is gitignored, so a `sha256sum` list
living only there anchors nothing: a successor cannot tell whether the artifact
they are reading is the one a document was written against. D-469 asks for a
digest receipt *committed or sha-anchored*; `ec48aea` recorded the lesson for
WP-1.9b, the Stage-3 premise closure §10 applied it ahead of the mistake, and a
red team found this arc had then made the mistake anyway
(`artifacts/stage3_rulings_redteam_v1.md`, BLOCKING 3).

**What the receipt covers.** This arc's artifacts, from the point the Stage-3
premise closure's own receipt stops. That receipt —
`artifacts/stage3_export_receipt_v1.txt`, sha256
`ec57ba2d3aa625a74f4860402dbfb3bc123a60dbef8aaf1eeeb62402b4fce974`, fourteen
files — is **complete for the run it covers and is not amended**; this one
continues it rather than replacing it.

---

## The anchor

**`artifacts/overnight_export_receipt_v1.txt` — sha256**

```
4a8bbb6d349631682fece77daa3bb292a816b37cc30fc6510cab072d50e84af6
```

Four files listed, the receipt itself excluded because a file cannot carry its
own digest.

## What is in it, and what each artifact is

| artifact | what it is |
|---|---|
| `stage3_call_budget_v2.txt` | **the governed call-budget derivation at HEAD** — the run every figure in `docs/experiments/stage3_rulings.md` §1 is cited from. Records its own argv and the sha256 of all four inputs |
| `stage3_call_budget_v1.txt` | **SUPERSEDED** — revision 1's run, kept because revision 1's figures are quoted in the red-team report and a superseded artifact that vanishes makes that report unreadable |
| `stage3_rulings_redteam_v1.md` | the fresh-context red team on the rulings at `2f8f836`: STANDS WITH CORRECTIONS, 3 BLOCKING / 11 MAJOR / 14 MINOR |
| `overnight_ci_premerge_21e05f8_v1.txt` | `tools/ci.sh` at `21e05f8`, the pre-merge gate run — 19/19, `ci: all gates passed` |

## The rule this file follows for the rest of the arc

**The receipt is re-taken and this anchor re-stated whenever the arc produces a
new governed artifact**, and the line above is the current one. A stale anchor
is worse than none, because it reads as a check that passed. When this arc
closes, its closure quotes this file rather than restating the list (D-423).
