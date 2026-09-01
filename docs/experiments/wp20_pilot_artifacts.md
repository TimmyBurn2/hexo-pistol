# WP-2.0 pilot — the artifact receipt

**Why this file is committed and the artifacts are not.** CLAUDE.md rule 8: nets,
books, match logs and bench outputs are never committed, and a committed manifest
may sha-index them. **This is that manifest.** Every number
`docs/experiments/wp20_pilot_prereg.md` reads off a run is read off one of the
files below, and without a committed digest the evidence for the value that fixes
65 % of the pilot's wall would be one `rm -rf` from not existing (D-469, and the
review finding that asked for this row).

**Where they live.** The dry run's own output is under `artifacts/` (gitignored);
the files it produced are under `/home/tom/pistol-runs/wp20pilot-dryrun4/`. A
successor who has neither can reproduce them: the dry run's literal commands are
the pre-registration's §8 with `configs/arena_wp20_label_pilot_dryrun.toml`, at
the revision §1 names. **It is not `tools/`-free** — §8 runs
`tools/cold_label_check.py` for criterion C-A — and saying so matters because that
script is one of the instruments a re-run must be taken at the same revision as.

## The dry run, at `31c1cc1` with a clean tree and a digest for every binary it ran

| sha256 | file | what it is |
|---|---|---|
| `3222ef191a96b3b1342ece1be8af21b807f706836a1df53ba22d2a5e64bc12d6` | `artifacts/wp20pilot_dryrun_31c1cc1_v1.txt` | SLOT A — the run's whole transcript, its provenance receipt, every exit code and every elapsed second the pre-registration quotes |
| `bfc16fc41de52015a18c657a6780d547adcdd1f329f822995904634ac6a44883` | `/home/tom/pistol-runs/wp20pilot-dryrun4/report_v1.txt` | pass 1's report — 4 games, 164 asked positions |
| `4bb0807c9491518e080d926f63dc9a5a4c86a9bff617ef701962aa2fa4405e48` | `/home/tom/pistol-runs/wp20pilot-dryrun4/capture_50000.txt` | RULE-2 candidate `nodes 50000` (the GAME budget, the referent) |
| `ae6be15a455e90684dda454d9912c22e9b857acc1c50b242e94f4dd1a9e7af1c` | `/home/tom/pistol-runs/wp20pilot-dryrun4/capture_100000.txt` | RULE-2 candidate `nodes 100000` |
| `38dda280e49af1d8f727d67ef4afc5609db10ea3ef143b60e719ed84c94c478f` | `/home/tom/pistol-runs/wp20pilot-dryrun4/capture_200000.txt` | RULE-2 candidate `nodes 200000` |
| `807d56563cf1f618e337e15f6c7c8109d23a23f1a1f51f48b4a7d9098c7246eb` | `/home/tom/pistol-runs/wp20pilot-dryrun4/capture_400000.txt` | RULE-2 candidate `nodes 400000` — **the selected budget** |
| `807d56563cf1f618e337e15f6c7c8109d23a23f1a1f51f48b4a7d9098c7246eb` | `/home/tom/pistol-runs/wp20pilot-dryrun4/capture_400000_b.txt` | the C-B re-run of the above |
| `31d5bfb4b28e2a170023cdb0d843fedf70102052ebafe5d89ff23837feb53f74` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_50000.txt` | the corpus RULE-2's 3.0 median at `50000` was read from |
| `9f163641b474bf8b35dba9973b1a19f9b83555686683833473d9550487324c09` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_100000.txt` | the corpus RULE-2's 3.0 median at `100000` was read from |
| `f21d1afaf50e6b8d2a5d3c6d60c60db7af30384a2562eea9003d425f166ece59` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_200000.txt` | the corpus RULE-2's 3.0 median at `200000` was read from |
| `d79f1d8107ea9eeb3bcc194f77d3f9cd115623e5a7e314c1eec9d8fe37914ccd` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_400000.txt` | the corpus RULE-2's **4.0** median was read from — the row that selected SLOT S2 |
| `d79f1d8107ea9eeb3bcc194f77d3f9cd115623e5a7e314c1eec9d8fe37914ccd` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_400000_b.txt` | the C-B re-run of the above |
| `d79f1d8107ea9eeb3bcc194f77d3f9cd115623e5a7e314c1eec9d8fe37914ccd` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_400000_c.txt` | the second transform, timed for §6.3's bound |
| `803a73d93d639127d59f1e70182c96d0e56f59c0033153c6ea9ba68df4e3de08` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_grammar.txt` | C-E run 2's input — one `key_pos` broken, body RE-DIGESTED, so it reaches the grammar |
| `30314914a83a47a7b29c1a303a28894435b78487fd2bc19c54f65bc4bef1adfe` | `/home/tom/pistol-runs/wp20pilot-dryrun4/corpus_digest.txt` | C-E run 3's input — one record appended, digest NOT brought back |
| `193f78213350ae561364d255ebe00ee2ac2a4eebd3b75ffe1e298973ce957eef` | `/home/tom/pistol-runs/wp20pilot-dryrun4/replay_v1.txt` | C-C's replay document, 4 of 4 games, 0 divergences |

**Two pairs are deliberately equal**: `capture_400000.txt` with
`capture_400000_b.txt`, and `corpus_400000.txt` with both `_b` and `_c`. **Those
equalities ARE criterion C-B's receipt on the stand-in**, visible here rather than
only in the transcript.

**The four `corpus_*.txt` candidate rows are what RULE-2 was decided on**: each
was read back by `crates/pistol-arena/src/bin/corpus-check.rs`, whose printed
`depth_turns median` line is the source of every cell in §6.3's depth table. A
successor who doubts that table re-runs `corpus-check` over these four files.

**`corpus_grammar.txt` and `corpus_digest.txt` are C-E's two injections**, indexed
because a criterion's INPUTS are as much its evidence as its outputs: the first
re-digests and so reaches the grammar, the second does not and is stopped by the
digest two checks earlier, and that is what makes them different guards.

## THE PILOT ITSELF, at `2cd4f79`

**These are the run the pre-registration governs.** Its transcript opens with the
provenance receipt §8 registers: the revision, a clean tree, and a digest for each
of the four instruments.

| sha256 | file | what it is |
|---|---|---|
| `77c3d75a119953026cbd75418799fa934661ab871cb05ffafdf1c46415095c59` | `artifacts/wp20pilot_RUN_2cd4f79_v1.txt` | the pilot's whole transcript — every exit code, every elapsed second, and every criterion's own output |
| `0378260b026709f7e89cc5ff326728f1d43c51d44b2c107a0f60f69c48712508` | `/home/tom/pistol-runs/wp20pilot-artifacts/report_v1.txt` | **THE PILOT'S REPORT** — 13 openings, 26 games, `VERDICT inconclusive_degenerate`, zero forfeits |
| `4563f0500e957fef28b7b0aba82ba2dbceca0e8a3d74e892924fb034dcde1808` | `/home/tom/pistol-runs/wp20pilot-artifacts/capture_v1.txt` | the capture, 742 asked positions at `go nodes 400000` |
| `4563f0500e957fef28b7b0aba82ba2dbceca0e8a3d74e892924fb034dcde1808` | `/home/tom/pistol-runs/wp20pilot-artifacts/capture_v2.txt` | the C-B re-run — byte-identical, `capture-determinism exit=0` |
| `493f4fa8b6fb3e395555a578e480917eb3ae05e53727953aae6febe873af9c4f` | `/home/tom/pistol-runs/wp20pilot-artifacts/corpus_v1.txt` | **THE CORPUS** — 742 labelled records, 347 distinct by every key |
| `493f4fa8b6fb3e395555a578e480917eb3ae05e53727953aae6febe873af9c4f` | `/home/tom/pistol-runs/wp20pilot-artifacts/corpus_v2.txt` | the C-B re-run — byte-identical, `labels-determinism exit=0` |
| `e493652c9309067791365aa617238755ed04c13e72a34d814777e207113d610c` | `/home/tom/pistol-runs/wp20pilot-artifacts/corpus_grammar.txt` | C-E run 2's input — `key_pos` broken, body RE-DIGESTED, refused on the grammar |
| `364a814c282d6c43bf2105301bb735b5961f9b77625d529587d882847cd77dd0` | `/home/tom/pistol-runs/wp20pilot-artifacts/corpus_digest.txt` | C-E run 3's input — record appended, digest NOT brought back, refused on the digest |
| `1b1527d2e93efc5b4b67688a90f1d98538ee1a5b0e86ed2b5361f8bcff264f2d` | `/home/tom/pistol-runs/wp20pilot-artifacts/replay_v1.txt` | C-C's replay document — 26 of 26 games, 0 divergences |

**The two byte-identical pairs are criterion C-B's receipt**, and they are read
from `cmp -s`'s exit code rather than from these digests compared by eye; the
digests are here because they are what a later reader cites.

## The call-site mutant receipts (D-553's corollary, the retro-check's closure)

| sha256 | file | what it is |
|---|---|---|
| `74dc9fdca092a3a1544dcae288adf47e39bd0b8af06834ced326bd51c6d726c1` | `artifacts/wp20pilot_mutants_4375ad9.txt` | nine call-removed mutants at `4375ad9`, run in a detached worktree on `/home` and never the live tree. **All nine DIE, each at its own registered test**: `no_tab`'s call, `classify`'s two refusal arms, `GameResult::from_token`'s refusal, `labels.rs`'s capture-identity, every-game and prefix bindings, `corpus-check`'s `printable` call, and `book`'s presence in the summary line |

## CI at the CLOSURE head

| sha256 | file | what it is |
|---|---|---|
| `c9a837c9ed76baaca92fbf507e9db25ab9213a7fae54f15a290d89cb72441966` | `artifacts/wp20_ci_CLOSURE_7af791a_v1.txt` | **THE CLOSURE RECEIPT.** `tools/ci.sh` at `7af791a`, all 19 gates, `ci: all gates passed`, `EXIT=0` — the head D-561 closes WP-2.0 at. Per the termination rule above, the commit recording this row changes documentation only |

## CI at the STOP head

**TWO RUNS ARE LISTED AND THE SECOND IS THE OPERATIVE ONE.** The first was taken
at `867a923`, before this manifest gained its own receipt row; adding that row
moved the head, so the run was re-taken at `205b6a8`. The recursion is recorded
rather than argued away, because "the later commit only touches a file no gate
reads" is exactly the kind of implication this session has been wrong about twice.

**AND IT TERMINATES BY A RULE RATHER THAN BY ANOTHER RUN, which is stated once
here because it is general.** Recording a gate receipt moves the head past the
run the receipt cites, so a receipt can never name its own commit and chasing it
never ends. The rule this project can hold is: **the receipt names the revision it
was TAKEN at, and the commit that records it changes documentation only.** That
is checkable — `git diff --stat 205b6a8 HEAD -- crates/ tools/ configs/` is empty
— and it is the check a reader should make rather than looking for a run at the
literal tip.

| sha256 | file | what it is |
|---|---|---|
| `09918bef9070ee3486360032c7e17dd074e806051b1ded5a2c2e32e9e3b1a602` | `artifacts/wp20pilot_ci_STOP_205b6a8_v1.txt` | **THE OPERATIVE RECEIPT.** `tools/ci.sh` at `205b6a8`, all 19 gates, `ci: all gates passed`, `EXIT=0` |
| `030dfee8f315a847dea4ddff61764f1b2e4c463cd9a7b50f7ec59923ae117388` | `artifacts/wp20pilot_ci_STOP_867a923_v1.txt` | `tools/ci.sh` at `867a923`, **all 19 gates**, `ci: all gates passed`, `EXIT=0` — the receipt D-556's STOP is cited at |

## CI at the registration head

| sha256 | file | what it is |
|---|---|---|
| `cb410160e244227f3fb21d9a1226ba5632aba2fbb820235710659f33b24c7e53` | `artifacts/wp20pilot_ci_6e1fea3_v1.txt` | `tools/ci.sh` at `6e1fea3`, all 19 gates, `ci: all gates passed` |
