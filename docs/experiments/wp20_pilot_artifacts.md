# WP-2.0 pilot — the artifact receipt

**Why this file is committed and the artifacts are not.** CLAUDE.md rule 8: nets,
books, match logs and bench outputs are never committed, and a committed manifest
may sha-index them. **This is that manifest.** Every number
`docs/experiments/wp20_pilot_prereg.md` reads off a run is read off one of the
files below, and without a committed digest the evidence for the value that fixes
65 % of the pilot's wall would be one `rm -rf` from not existing (D-469, and the
review finding that asked for this row).

**Where they live.** The dry run's own output is under `artifacts/` (gitignored);
the files it produced are under `/home/tom/pistol-runs/wp20pilot-dryrun2/`. A
successor who has neither can reproduce them: the dry run is
`tools/`-free and its literal commands are the pre-registration's §8 with
`configs/arena_wp20_label_pilot_dryrun.toml`, at the revision §1 names.

## The dry run, at `85e6261` with a clean tree

| sha256 | file | what it is |
|---|---|---|
| `4e716a4f7608485b4ec05cff048c67fa8b4e21deec6c4f61a448b457cefc73b4` | `artifacts/wp20pilot_dryrun_85e6261_v1.txt` | SLOT A — the run's whole transcript, every exit code and every elapsed second the pre-registration quotes |
| `05d8e2a40d8ed31ada3b51b66df6f18c214be0ab0d497b5d021bf035425e6774` | `/home/tom/pistol-runs/wp20pilot-dryrun2/report_v1.txt` | pass 1's report — 4 games, 164 asked positions |
| `0dbf4e2a156151567d395eae329daf513a9158397aa59d75be34a8aa7493c5c8` | `/home/tom/pistol-runs/wp20pilot-dryrun2/capture_50000.txt` | RULE-2 candidate `nodes 50000` (the GAME budget, the referent) |
| `fa1900dd15c20699a93453d819fa3f8b660b6e28f207bd26bd6f242daa23b16b` | `/home/tom/pistol-runs/wp20pilot-dryrun2/capture_100000.txt` | RULE-2 candidate `nodes 100000` |
| `f42599e7f7e9ba730e1cfd66b2892c2ec38829fedfde7bce607dd82ef9064fdc` | `/home/tom/pistol-runs/wp20pilot-dryrun2/capture_200000.txt` | RULE-2 candidate `nodes 200000` |
| `5fe1f1a36bef97d05679807c06df1efe85245ccd51362c6c670b5943ea95af20` | `/home/tom/pistol-runs/wp20pilot-dryrun2/capture_400000.txt` | RULE-2 candidate `nodes 400000` — **the selected budget** |
| `5fe1f1a36bef97d05679807c06df1efe85245ccd51362c6c670b5943ea95af20` | `/home/tom/pistol-runs/wp20pilot-dryrun2/capture_400000_b.txt` | the C-B re-run of the above |
| `9c47d831cf18e9ec6e7225afd8cb1c2ce05ab6363669d35dae4cae43bf2596e9` | `/home/tom/pistol-runs/wp20pilot-dryrun2/corpus_50000.txt` | the corpus RULE-2's 3.0 median at `50000` was read from |
| `5f91f65cb1091aaa378736f88144bc7b848a8b53ea9a05ecf28e91ecdaf6a6a7` | `/home/tom/pistol-runs/wp20pilot-dryrun2/corpus_100000.txt` | the corpus RULE-2's 3.0 median at `100000` was read from |
| `0f01920187675b8cfcb33cb37163b808349cd00938060faa885527d584473721` | `/home/tom/pistol-runs/wp20pilot-dryrun2/corpus_200000.txt` | the corpus RULE-2's 3.0 median at `200000` was read from |
| `099489f0f8baf320326a6c08d318a35208331381bb1fbf5337d8530a1483a51d` | `/home/tom/pistol-runs/wp20pilot-dryrun2/corpus_400000.txt` | the corpus RULE-2's **4.0** median was read from — the row that selected SLOT S2 |
| `099489f0f8baf320326a6c08d318a35208331381bb1fbf5337d8530a1483a51d` | `/home/tom/pistol-runs/wp20pilot-dryrun2/corpus_400000_b.txt` | the C-B re-run of the above |
| `10d2eeec89a24bce839fb5bf89973fde982de85d89e09a4f533e4144a581b395` | `/home/tom/pistol-runs/wp20pilot-dryrun2/replay_v1.txt` | C-C's replay document, 4 of 4 games, 0 divergences |

**Two rows are deliberately equal**: `capture_400000.txt` and
`capture_400000_b.txt` share a digest, and so do `corpus_400000.txt` and
`corpus_400000_b.txt`. **That equality IS criterion C-B's receipt on the
stand-in**, and it is visible here rather than only in the transcript.

**The four `corpus_*.txt` rows are what RULE-2 was decided on**: each was read
back by `crates/pistol-arena/src/bin/corpus-check.rs`, whose printed
`depth_turns median` line is the source of every cell in §6.3's depth table. A
successor who doubts that table re-runs `corpus-check` over these four files.

## CI at the registration head

| sha256 | file | what it is |
|---|---|---|
| `cb410160e244227f3fb21d9a1226ba5632aba2fbb820235710659f33b24c7e53` | `artifacts/wp20pilot_ci_6e1fea3_v1.txt` | `tools/ci.sh` at `6e1fea3`, all 19 gates, `ci: all gates passed` |
