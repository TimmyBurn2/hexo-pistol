# WP-2.0b — the artifact receipt

**Why this file is committed and the artifacts are not.** CLAUDE.md rule 8: nets,
books, match logs and bench outputs are never committed, and a committed manifest
may sha-index them. **This is that manifest** (D-469). Without it, the evidence
for every number this package's closure states would be one `rm -rf` from not
existing — and this project has already lost a set of review reports to exactly
that (the memory note about finding IDs pointing at `/tmp`).

**Where they live, and why not in `/tmp`.** Everything below is under
`artifacts/` (gitignored, on `/home`). The perf guard's own instrument was first
written to the session scratchpad, which is on a **24 GiB tmpfs** — a digest of
something a reboot removes — and REVIEW-impl round 1 raised it (obligations M8).
The instruments are exported here with the rest.

**The live digest list** is `artifacts/wp20b_MANIFEST.txt`, regenerated at
closure; the table below is the committed copy of it. A file whose digest
disagrees with this table is not the file the closure read.

## What each artifact is, and which claim stands on it

| file | what it is, and the claim it carries |
|---|---|
| `wp20b_MANIFEST.txt` | the digest list itself, taken at closure |
| `wp20b_perf_guard.sh` | THE INSTRUMENT — the design §9 block plus the third arm and the four counters `wp20b_impl.md` §3 registers |
| `wp20b_perf_report.py` | the reader that turns its raw output into the receipt's numbers, under all three estimators |
| `mutants.py` | the mutation driver: each mutant, the test that must die at it, and a patch reverted whether or not the run succeeded |
| `wp20b_identity_RECEIPT.txt` | **byte-identity, gate OFF, at the CLOSURE binary** — the two registered referents, the extraction rule applied, the named revision, and the diff showing what the rule excludes |
| `closure_{gate_v0,instrument_v0}_run{1,2}.txt` | the four records those digests are taken over |
| `wp20b_ci_closure_v1.txt` | **CI at the closure tree: 19 gates, EXIT=0** |
| `wp20b_perf_guard_v2.txt`, `wp20b_perf_dryrun_v3.txt` | the GOVERNED perf run and its dry run, both under the reviewed registration |
| `wp20b_mutants_v7.txt` | **THE MUTATION RECEIPT: 27 registered, 27 dead at their registered test, 0 alive, 0 faults** |
| `postchange_*`, `wp20b_perf_guard_v1.txt`, `wp20b_determinism_v1.txt`, `wp20b_mutants_v{1,2,3,4}.txt` | SUPERSEDED, kept because a superseded measurement stays on the record |
| `prechange2_*.txt`, `pistol_prechange_a56449b` | the referents, carried from WP-2.0 |
| `wp20b_determinism_v1.txt` | `tools/determinism.sh`, five seats, exit 0 |
| `wp20b_perf_dryrun_v2.txt` | the FIRST dry run, superseded: taken before the arm-order rotation existed, so it did not cover the instrument it was cited for |
| `wp20b_perf_guard_v1.txt` | the guard's raw per-position output, 300 rows |
| `wp20b_perf_RECEIPT.txt` | the guard's verdict against what was registered |
| `wp20b_mutants_v1.txt` | mutation run 1 — **the one that found a vacuous test**: 19 dead, M10 ALIVE |
| `wp20b_mutants_v2.txt` | run 2 after the fix: 20 dead at their registered test, 0 alive |
| `wp20b_mutants_v3.txt` | the fix round's run — **23 dead, M24 ALIVE**, EXIT=1: a FOURTH vacuous test |
| `wp20b_mutants_v4.txt` | **ABORTED** after 23 mutants on a stale patch anchor, EXIT=1 — the fault the harness now reports per mutant instead of exiting on |
| `wp20b_mutants_v5.txt`, `v6.txt` | superseded: v5 was clean at 26 but predates the round-2 fixes; v6 took **2 harness faults** from anchors my own fix had moved |
| `wp20b_perf_dryrun_v3.txt` | the guard's dry run at the FINAL instrument, six reps in six distinct arm orders |
| `wp20b_keypos_{corpus,trigger-rich}_{2048,16384}.txt` | **§9's `key_pos` obligation** — 798 firings over which the in-tree symmetry fold merged nothing |
| `ovn2_ci_a6777f4_v1.txt` | CI at the arc's start head: 19 gates, exit 0 |
| `ovn2_wp20b_stopped_SUPERSESSION.txt` | D-566's content-supersession receipt for the deleted STOP branch |

## The one thing a successor cannot reproduce from this table

`artifacts/pistol_prechange_a56449b` is a BINARY, and the byte-identity
obligation is a comparison against what it prints. It should be reproducible from `a56449b` by a
release build, but **NOBODY HAS DONE THAT AND THIS DOCUMENT PREVIOUSLY SAID
SOMEONE HAD** — it credited REVIEW-impl round 1 with a rebuild that round
explicitly did not perform (*"the binary was never rebuilt"*; its argument was
that a rebuild was not NEEDED, the only source change since the build being `///`
doc comments, which do not reach codegen). A rebuild is in any case a rebuild and
not a recovery, and a toolchain move would break it. **The digest is what binds
this file, and it is why the row above exists.**
