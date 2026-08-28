# WP-P1c step 0 — corpus provenance: STOP AND ESCALATE

Queue gate satisfied: WP-1.8b is closed (D-442 is its closure pin).

**Step 0's stop condition fires. No client was written, no endpoint was probed,
and step 1 was not started.** The dispatch is explicit that this finding is
larger than anything downstream of it and is not fixed by writing a second
acquisition path next to it. It is not fixed here.

Report lives in pistol beside the WP-P1/WP-P1b reports because no hexo-bridge
code was written and the affected consumers are here.

## What the acquisition is, with receipts

The corpus is an external HuggingFace dataset, never committed (rule 8):

    $ cd ~/Projects/hexo-bootstrap-corpus && git remote -v
    origin  https://huggingface.co/datasets/timmyburn/hexo-bootstrap-corpus (fetch)

    $ git log --oneline | head -2
    1a82e15 Sync corpus from hexo-pi scraper: 8300 -> 8698 human games (+398)
    313a2e2 Sync corpus from hexo-pi scraper: 8300 -> 8698 human games (+398)

    $ git show --stat 1a82e15
    Author: T <timmyburn@users.noreply.huggingface.co>
    Date:   Tue Jul 7 18:58:54 2026 +0000
     dataset_metadata.json | 8 ++++----

`dataset_metadata.json` records the population but not the acquisition:

    "n_games": 8698, "n_duplicates_dropped": 0,
    "source_filter": "rated, >=20 moves, decisive by six-in-a-row",
    "input_dirs": ["data/corpus/raw_human"],
    "sha256": "b2fe61eb…", "created_at": "2026-07-07T18:54:09Z"

`input_dirs` names a LOCAL DIRECTORY inside the scraper project — not an
endpoint. That directory does not exist on this machine:

    $ find /home/tom -maxdepth 6 -type d -path "*corpus/raw_human*"
    (no output)

## The four questions step 0 asks

| question | answer |
| --- | --- |
| what fetched it | a "hexo-pi scraper", named **only** in a commit message |
| from which endpoint | **NOT RECORDED ANYWHERE** |
| authenticated or not | **NOT RECORDED ANYWHERE** |
| reproducible today | **NO** |

## Why "not documented" is a finding and not an oversight

The whole history of the dataset repo was checked, not just its tip:

    $ git log --pretty=format: --name-only --all | LC_ALL=C sort -u | grep -v '^$'
    .gitattributes
    README.md
    SCHEMA.md
    bootstrap_corpus.npz
    bootstrap_corpus_pretrain_v6.npz
    bootstrap_corpus_v6w25.npz
    bootstrap_corpus_v7.npz
    dataset_metadata.json
    hexo_human_corpus.jsonl

    $ git log --oneline --all | wc -l
    29

**Across 29 commits, only data files and docs were ever committed. No scraper,
no ingest script, no endpoint, no acquisition code has ever existed in this
repo**, and no commit message anywhere names a URL, API, endpoint or token
(`git log --all --pretty="%s%n%b" | grep -iE "http|api|endpoint|token|auth"` is
empty).

Neither sibling repo holds the path either:

- hexo-bridge: `grep -rniE "corpus|scrap|ingest|hexo-pi|huggingface|bootstrap"`
  over `*.py`/`*.md`/`*.toml` matches only four unrelated uses of the word
  "bootstrap" (the htttx engine-session dial bootstrap).
- pistol: the corpus appears only as a cited external artifact —
  `docs/experiments/wp18a_design.md:544` and `wp18a_impl_REVIEW.md:53` cite
  `timmyburn/hexo-bootstrap-corpus`@`1a82e15`, and D-434 describes its role.
  Nothing describes how it was obtained.

The "hexo-pi" project is not on this machine
(`find /home/tom -maxdepth 4 -iname "*hexo-pi*"` is empty), and no scraper
script exists anywhere on it.

## What this puts at risk

`b2fe61eb…` is the population under every one of these:

- **D-218**, the 2812-placement refutation that closed SB-65 and kept
  `LEGAL_RADIUS` at 8 — a rules-truth ruling.
- **D-446**, this round's census: 0 duplicate games, which is what cleared the
  Stage-2 leakage concern.
- **D-447**, the turn-14 sharing horizon, now the registered witness criterion.
- **D-434**, Stage-2 Texel-style calibration and the independent holdout.

None of these is wrong. All of them are unfalsifiable in the one way that
matters: the `source_filter` ("rated, >=20 moves, decisive by six-in-a-row") is
a claim about a selection nobody can re-execute or audit. If that filter was
applied differently than recorded — or if the scrape missed a population
segment — every measurement above inherits the bias silently, and the sha pin
does not help, because a pin fixes WHICH BYTES were measured and says nothing
about WHICH GAMES REACHED THEM. That distinction is the same one D-448 already
records for the puzzle corpus's own SHA.

## What I did not do

No endpoint was probed. Step 1 would have determined whether
`https://hexo.did.science/games` serves history without a credential — one
`curl`, read-only — but step 1 is gated behind step 0, and step 0 says stop.
Probing would also have started building the second acquisition path the
dispatch names as the wrong fix.

Old-pin integrity (test 2) verified regardless, before and after:

    $ sha256sum ~/Projects/hexo-bootstrap-corpus/hexo_human_corpus.jsonl
    b2fe61eb360b91d77873a751446d28287955cad49e331fc32c156b4e1316840c
    $ git status --short        # in the corpus repo
    (clean)

## What the architect is being asked to decide

1. **Where is the hexo-pi scraper?** If it exists off this machine, the fix is
   to bring it under version control and re-run it with a wider range — the
   dispatch's own preferred path, and it makes D-218's population auditable.
2. **If it is gone**, then the corpus is an unreproducible artifact underneath a
   rules-truth ruling, and that is a decision to record explicitly rather than
   leave implicit — including whether D-218 stands on a population that can no
   longer be re-derived.
3. **Only after that**, whether to authorise step 1. It needs one read-only
   request and I will not make it unasked.

## What is still not established

The mapping stays UNVERIFIED and the puzzle corpus stays POSITION-GRADE
(D-448): one witness at turn 11, below the D-447 horizon of 14, so it closes
nothing. No snapshot was taken, so no census or horizon was re-measured on a new
population, and the 16-of-17 staleness is unchanged. No credential exists and
none was sought. Nothing here is a strength claim, and nothing here says any
prior measurement is wrong — only that one of them rests on a population that
cannot currently be re-derived.
