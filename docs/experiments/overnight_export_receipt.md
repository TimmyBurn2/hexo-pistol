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
8203da5c35a5ed9883f4867cfa562de3e539a2c67eb9f35a800e028a4b1c138a
```

**Thirty-one files listed**, the receipt itself excluded because a file cannot
carry its own digest. The two anchor seats are DIRECTORIES: their forty
transcripts and eighty stderr files are sha-indexed in each seat's own
`MANIFEST.sha256`, and it is the MANIFEST that this receipt lists — one
indirection, stated so nobody reads the shorter list as the whole export.

## What is in it, and what each artifact is

| artifact | what it is |
|---|---|
| `stage3_call_budget_v3.txt` | **the governed call-budget derivation at HEAD** — the run `docs/experiments/stage3_rulings.md` §1 is cited from. Carries its own argv, the sha256 of all four inputs, ruling 6's `t_90` ceiling, and the six refusal legs |
| `stage3_call_budget_v2.txt`, `_v1.txt` | **SUPERSEDED** — revisions 2 and 1's runs, kept because the review reports quote their figures and a superseded artifact that vanishes makes a report unreadable |
| `stage3_trigger_census_corpus_v1.txt` | the trigger census over the corpus bench fixture: one row per firing, with the O(1) columns and what the solver answered |
| `stage3_trigger_census_trigger_v1.txt` | the same over the trigger-rich fixture |
| `stage3_census_analysis_v1.txt` | **where the measured `K` comes from** (D-517), and the option matrix's §5 ranking |
| `book_v2_power_v1.txt` | `book_v2`'s size grounds: the dry run against D-187's four recorded figures, and the registered sweep that answered `P = 4000` |
| `stage3_rulings_redteam_v1.md` | the fresh-context red team on the rulings at `2f8f836`: STANDS WITH CORRECTIONS, 3 / 11 / 14 |
| `stage3_rulings_review_rev2.md` | the scoped re-review at `ba8e6b2`: FAIL, 1 new BLOCKING / 2 MAJOR / 6 MINOR |
| `counter_unit_impl_REVIEW.md` | REVIEW-impl on the solver call counters: PASS WITH FINDINGS, 0 / 5 / 5 |
| `book_v2_impl_REVIEW.md` | REVIEW-impl on `book_v2`: PASS WITH FINDINGS, 3 / 5 / 5 |
| `anchor_v2_impl_REVIEW.md` | REVIEW-impl on the movetime seat: PASS WITH FINDINGS, 2 / 6 / 7 |
| `overnight_ci_premerge_21e05f8_v1.txt` | `tools/ci.sh` at `21e05f8` — 19/19, the pre-merge gate run |
| `overnight_ci_postmerge_2f8f836_v1.txt` | the same at `2f8f836` — 19/19, post-merge |
| `overnight_ci_7240e37_v1.txt` | the same at `7240e37` — 19/19, the three landed units |
| `book_v2_power_refusals_v1.txt` | the power instrument's seven refusal legs and its determinism check — criteria C and D of `book_v2`'s registration |
| `book_v2_loadability_v1.txt` | **every entry of `book_v2` through the guarded bench block** (D-475): `4500 entries x 1 reps = 4500 totals lines, 0 refused` |
| `overnight_mutation_receipts_v1.txt` | five mutants, all dying: the `book_v2` output name, and each of the four solver-counter increment sites |
| `overnight_worktree_receipt_v1.txt` | what each worktree's gitignored `artifacts/` and `sessions/` held before removal (D-469) — nothing needing export, checked rather than asserted |
| `sealbot_anchor_v2_seat1/*` | **the governed anchor, seat 1** (gates off): 20 W / 20 L, 2 distinct games, 320 answers at median 500 ms |
| `sealbot_anchor_v2_seat2/*` | **the governed anchor, seat 2** (solver gate on): 0 W / 40 L, 2 distinct games, 240 answers at median 1225 ms |
| `stage3_census_rank_v1.txt` | the option field ranked IN VISITS, the unit the bracket fixes — with the cap sweep, the verdict-cache bound and the compositions |
| `matrix_stage3_DECISION_REDTEAM.md` | the decision red team on the matrix at `1bc2788`: **FALLS**, 5 BLOCKING / 9 MAJOR / 5 MINOR, and five rows it would add |
| `sealbot_anchor_v2_dryrun_*/report.json` | the four registered dry runs, including the two mode-pin runs that must forfeit at the handshake and do |

## The rule this file follows for the rest of the arc

**The receipt is re-taken and this anchor re-stated whenever the arc produces a
new governed artifact**, and the line above is the current one. A stale anchor
is worse than none, because it reads as a check that passed. When this arc
closes, its closure quotes this file rather than restating the list (D-423).
