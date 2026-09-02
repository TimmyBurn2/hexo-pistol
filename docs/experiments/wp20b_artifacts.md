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
closure; **the table below carries its digests**, so a file whose digest
disagrees with this table is not the file the closure read.

**AN EARLIER REVISION OF THIS DOCUMENT SAID THAT AND CARRIED NO DIGESTS AT ALL**
— `/usr/bin/grep -cE '[0-9a-f]{64}'` over it returned **0** — so the sentence
above named a comparison nobody could make, and this file was not the sha-index
rule 8 and D-469 lean on. REVIEW-impl round 3 found it (M1). **The failure it
would have caused is the one this document's own opening paragraph names**:
`rm -rf artifacts/` removes both the artifacts and the live manifest, and the
committed file that survives held not one digest to check a recovered copy
against.

## The index, with the digests the closure took

| sha256 | file |
|---|---|
| | **the instruments, at the digests docs/experiments/wp20b_impl.md §3.1 registers** |
| `648a96d87686dbead588cc1ff3753edabedc2963c1a5f4d101b7a9b7b9be7573` | `artifacts/wp20b_perf_guard.sh` |
| `d29222bea4a515d9e5a4dd780c317d73d44b9201cd23b55aeec4bea7cf95f985` | `artifacts/wp20b_perf_report.py` |
| `b2ae1fd63e98496d2a1f75f2ffbc3a31b0266ff5fac5c84e07be7db6c0feaf00` | `artifacts/mutants.py` |
| | **THE RECEIPTS THIS CLOSURE STANDS ON** |
| `e777941dd638df19d9e629e93e1890667ce001d4c9c4268b18a65bc4f0f6110c` | `artifacts/wp20b_identity_RECEIPT_v2.txt` |
| `b7d7965ad887cad9c3596397718f040b067052c61d9059c853c25bb79eda9b65` | `artifacts/wp20b_determinism_v2.txt` |
| `9a6a0a21092b36be8d070b17b1b79ffe8663c5cb5c23ac9cfc23cf279821c3aa` | `artifacts/wp20b_perf_RECEIPT.txt` |
| `437a9f38e1b0b5d4dd53f213b22446b1541a341d9e1fc235756c4b7570856e57` | `artifacts/wp20b_perf_guard_v3.txt` |
| `fc080c14853460204ddfa157b31a1bbff75a76c3a149c6454e0ea8ab5c6ed84a` | `artifacts/wp20b_perf_dryrun_v3.txt` |
| `084e9679032a65a1a2e8a2fe2fcddfea05d9a86b9493b8a85d7f725750d4f151` | `artifacts/wp20b_mutants_v8.txt` |
| `eafd8f163afbdffad4679667e226eddb44285371d25af817f37da5d2d32f7229` | `artifacts/wp20b_ci_closure_v2.txt` |
| `620eab286842927ef1f051ea7897d49091f2606183a51c6da1d302546210eda3` | `artifacts/closure2_gate_v0_run1.txt` |
| `b242d9027d150d6a48c31e5e1ff8049c9200dad039efeb24fe84cdfd141a8bb8` | `artifacts/closure2_gate_v0_run2.txt` |
| `c2880e00055a087f305e183ba97f64c3e94a0920f863c485054b8745c284e315` | `artifacts/closure2_instrument_v0_run1.txt` |
| `b4ba951d19df9535a040ff42de905987f9d5fccbf3a6d06258b82191a2f00d23` | `artifacts/closure2_instrument_v0_run2.txt` |
| | **the key_pos measurement (design §9's own obligation)** |
| `68b9b7fd9dc1c8b7f70102cc45a113f3c4e288136148720a0590eda4377cf0d7` | `artifacts/wp20b_keypos_corpus_2048.txt` |
| `5a10dd3eaa4fe1c4bfa9dadaec624ff05b46cdeb4246b8380d081ba58bea9c90` | `artifacts/wp20b_keypos_corpus_16384.txt` |
| `3a730d08751a5444f6bc2b4cf0dfe8d416ea7ae9a1e864dd8f0510c52ea8cc2b` | `artifacts/wp20b_keypos_trigger-rich_2048.txt` |
| `6b007a7c0df6f6a41e6d05ede392ad26dbcd18f3fd1b78de21f3006051bcf4c3` | `artifacts/wp20b_keypos_trigger-rich_16384.txt` |
| | **SUPERSEDED, kept because a superseded measurement stays on the record** |
| `86d338b68a54600f1cd24919ef141053033e9d7cc4935c1b4fdae14b93285b87` | `artifacts/wp20b_perf_RECEIPT_v2_SUPERSEDED.txt` |
| `3733f7b609ad66a119fbdc514b92a43d6169feb55b6247875d80a0c703ac971f` | `artifacts/wp20b_perf_guard_v2.txt` |
| `bdfc1ea0aa6b5b117ae2a13952bd756d40837270aa613e038a05309cac75b808` | `artifacts/wp20b_perf_guard_v1.txt` |
| `e8ed6f8740a3ba6b676580181b688cac63542d9f519d699df2305d9603ae75f0` | `artifacts/wp20b_perf_dryrun_v2.txt` |
| `bf07a4f6c6de7fa006c558a6bf1ff5ffb089fae5b56a0907d79ce08bfb624be9` | `artifacts/wp20b_identity_RECEIPT.txt` |
| `1ebdc4592e5084a024920ef796ece0553d0f6ab38ed943a4656ce236f3491fa7` | `artifacts/wp20b_ci_closure_v1.txt` |
| `b7d7965ad887cad9c3596397718f040b067052c61d9059c853c25bb79eda9b65` | `artifacts/wp20b_determinism_v1.txt` |
| `e3bb648fbe788062a5e82a3184bc1c1c2555df4027ae538404fa731077b3f45f` | `artifacts/wp20b_mutants_v1.txt` |
| `5c47b142d1d6088f12187aefa322435ce5b28bc14b56e8c3a5421a6b4df34b88` | `artifacts/wp20b_mutants_v2.txt` |
| `21c9537d8f07d5f5ced5dfc9a11ffe6d220b9a456667ca707d119bd90fd95747` | `artifacts/wp20b_mutants_v3.txt` |
| `145172fd21a258524262c86571f59f379ad37f00a40947b4a44f208a49d37885` | `artifacts/wp20b_mutants_v4.txt` |
| `759e4f59fbd14a85323aca2d3bbd7e813033194b2f413f0e3d16f34adf7aea0b` | `artifacts/wp20b_mutants_v5.txt` |
| `2dc91138088d77640cfc52493f24696d94278b09eba7b3234b08b90e145a19bb` | `artifacts/wp20b_mutants_v6.txt` |
| `822b9938a3b07d3ed4ad50b836ee5d2102cb84e7d7fac37a7295f566957891a9` | `artifacts/wp20b_mutants_v7.txt` |
| `c880d9e7d2e3d6aed70d38365adc69d3830448cec1e0761f7f515bd6bd3fb393` | `artifacts/closure_gate_v0_run1.txt` |
| `4e3332263039a210bb27d87ac88feb1f956a5ad78b6948ee71d7115aa9185429` | `artifacts/closure_gate_v0_run2.txt` |
| `532ee29942d4aefc69fc421b1c0f54e9a6833681992b1ebbc20dd4118c9e6225` | `artifacts/closure_instrument_v0_run1.txt` |
| `d3ccac0dad0858a4cf7e2f9ec5db0a8d871e5ac89c321b0dbe18e2acc8279c88` | `artifacts/closure_instrument_v0_run2.txt` |
| `acda26d3c554e4cbc1d91b91b28c4bc39f76d888dbc29385e8fe87d30b7b05b5` | `artifacts/postchange_gate_v0_run1.txt` |
| `45223c2f4fbafa7178c0ca28b32de947e9018a9bdb0f4fffe5340619e07e9055` | `artifacts/postchange_gate_v0_run2.txt` |
| `58718eb27d185105049fdb01ac29cdd91a97241a8222a06b190679bab5d9d2ff` | `artifacts/postchange_instrument_v0_run1.txt` |
| `dcfb6fd86c191057216ce377258c871d04098ac9876ac0ac5225a67a46e4d998` | `artifacts/postchange_instrument_v0_run2.txt` |
| | **the referents carried from WP-2.0, and the arc's own receipts** |
| `180b4c406b225fc81342bb8218b8546dda1ffac1a99f7eb91cdaf73d20253476` | `artifacts/pistol_prechange_a56449b` |
| `14f751bfcba38201f1af4dd742484ca187773065f57405f93d52fdad8c0a7c21` | `artifacts/prechange2_gate_v0_run1.txt` |
| `c42ee0403643e3913fa3407a31075a343388fe1dc46efeec74d839d7ae6258f0` | `artifacts/prechange2_gate_v0_run2.txt` |
| `c02a7b2c0a064f946ae564960ea81e2bdf4d30b552069df1be78e39e53372e17` | `artifacts/prechange2_instrument_v0_run1.txt` |
| `6051ab8e90dfb10308057fccabbbfbe3bb70f7c5d2ddc5be4dd6a0cb38f83fc6` | `artifacts/prechange2_instrument_v0_run2.txt` |
| `77c3d75a119953026cbd75418799fa934661ab871cb05ffafdf1c46415095c59` | `artifacts/wp20pilot_RUN_2cd4f79_v1.txt` |
| `c23980edc8edfbc8ad0ebd68d5c560c55feef922d574e730f953953d37d355b3` | `artifacts/ovn2_ci_a6777f4_v1.txt` |
| `ae18295e1a4e88116f51c9f326571df431f8c493c22d675da8a629cdcc977237` | `artifacts/ovn2_wp20b_stopped_SUPERSESSION.txt` |

**48 digests**, and `sha256sum -c` over them from the repository root is the check.

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
