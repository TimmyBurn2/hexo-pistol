# WP-1.5d — REVIEW-design, revision 2

## Header

**Artefact reviewed:** `docs/experiments/wp15d_design.md` at commit
`13720fc5267295d1bf981e0ae009e1347464b87c` ("docs(wp15d): design revision 2 —
the guard moves into turns, and the store rule the matrix wrongly dismissed
lands"), **revision 2**, 409 lines.

**Does it still match HEAD?** YES.

```
$ git rev-parse HEAD
13720fc5267295d1bf981e0ae009e1347464b87c
$ git status --short
(no output)
```

**Reviewer:** fresh context. Did not write the design, the matrix, or any of the
three DECISION-RED-TEAM rounds.

**Read in full:** `docs/experiments/wp15d_design.md` (rev 2);
`docs/experiments/matrix_M2.md` (rev 3) §0–§5;
`docs/experiments/matrix_M2_REDTEAM.md`, `_round2.md`, `_round3.md` (verdicts,
every BLOCKING, and round 3's BLOCKING 1 line by line);
`docs/experiments/WPQ_seed.md` §7.1–§7.2; `CLAUDE.md`; `docs/process.md`;
`docs/decisions.md` D-356, D-357, D-373, D-381, D-424, D-476–D-478;
`docs/rule9_justifications.md`; `tools/bench_block.sh` (all 277 lines);
`tools/determinism.sh:58-92`; `tools/SHELL_CHECKLIST.md` coverage rule via
`docs/process.md`.

**Code read:** `crates/pistol-search/src/pvs.rs` (all 989), `search.rs` (all
672), `staged.rs` (all 349), `quiescence.rs:1-200,530-560`, `candidates.rs:1-70`,
`params.rs`, `info.rs:1-80`, `tt/mod.rs:1-200`,
`crates/pistol-engine/src/{validate.rs,instance.rs}`,
`crates/pistol-cli/src/bin/pistol.rs:118-176`,
`crates/pistol-core/src/rules.rs:30-36`.

**Digests verified.** All fourteen `artifacts/wp15d_*.txt` files hash to the
values in `artifacts/wp15d_worktree_export_receipt_v2.txt` (13 listed + the
receipt itself, which by its own note does not list its own digest):

```
$ sha256sum artifacts/wp15d_*.txt | LC_ALL=C sort -k2
bd7b871b21bf0e9c0461772b8fba98ae9ba13716e90cffa116184ab5793f1cb9  artifacts/wp15d_bench_block_receipt_v1.txt
e2afea20fe76532aee1de62fc9b5bc35f473c6e023f470a7aeaded466de93f44  artifacts/wp15d_ci_stop_v1.txt
081c928a0900ae9332a2e1f2b3fe012732a2b0e56b97c4de2064e06fd34add76  artifacts/wp15d_m2_evidence_instrument_v1.txt
f73608dd3693762e02968e6ec9a4c8078ac109fcc44bf3580ab8b1fc437d632c  artifacts/wp15d_m2_evidence_instrument_v2.txt
db8a8793d6a2b4a5f2635c60139b3577a68c6a872331fc89acfa43b2fb327be5  artifacts/wp15d_m2_evidence_v1.txt
455aef9e235785986290a0ce43c5fe6cb835532e5ede0923027c169a0d0c3b7f  artifacts/wp15d_m2_evidence_v2.txt
b1e3fb2c9e5df1795f09fcd9b45c5186b43264251d67ff63a7eef5494a90f6b2  artifacts/wp15d_ply_share_verification_v1.txt
b3bbd1dd6535889f7402d7983cd99ef17e0a93d05e208e438c4f4afb084b35cb  artifacts/wp15d_redteam_round1_instrument_v1.txt
188f4a6a48e68ad8d7cabd99a9628a81ea9d97be8dc7c26a313454a159a92976  artifacts/wp15d_redteam_round2_instrument_v1.txt
854c23bfb6fd786d193692bb29341e6baad6215a8390f551d88c41a86dc289b8  artifacts/wp15d_redteam_round3_instrument_v1.txt
43fa71ce9cc6e99cf69aee40493096c4a3d30301ed97cd1c88794df5c3026c10  artifacts/wp15d_turn_axis_book_v1.txt
5a64034e3ab178beecded86359692a2076a66ae89d3e4961c83b1ac4d082a3ae  artifacts/wp15d_turn_axis_v1.txt
39483aec7a83f4ce7358c72c3280df23f6a2ad04286be53aee0d9d4c1f1e7734  artifacts/wp15d_worktree_export_receipt_v1.txt
```

**What I ran.** A detached worktree at `13720fc`, `/home/tom/Projects/pistol-wt-rvd`,
own `CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rvd-target`, on `/home`. In it:
(1) a compile probe that adds `safety_net_top_k: u64` to `StagedParams` and
enumerates every site that will not compile without it; (2) a one-line
`eprintln!` probe at the design's own §2.1 truncation site
(`pvs.rs`, immediately after the empty-set check) printing
`turns_from_root()` and `set.cells.len()` on every safety-net row, built
`--release --locked` and driven over the committed fixtures through the shipped
line protocol. **The live tree was never edited except to write this file.**
**The worktree is LEFT IN PLACE** at `/home/tom/Projects/pistol-wt-rvd` (with
its target dir at `/home/tom/Projects/pistol-wt-rvd-target`) — D-469 requires an
export receipt before removal and I may write only this one file. It holds no
`artifacts/` or `sessions/` content of its own; its only uncommitted content is
the two throwaway probes described above.

---

## VERDICT: **FAIL**

Three BLOCKING findings. **None of them is a correctness finding** — I attacked
§6.3's store rule five ways and could not break it, and §2.2's guard names
exactly the root turn at every turn number. All three are
requirement/registration findings, and two of them are the same class the three
red-team rounds kept finding: **a claim that reads correctly in isolation and is
false in the unit the engine actually works in.**

- **BLOCKING 1** — §8's registered spread expectation, marked MEASURED, is
  refuted by measurement at §8's own registered budget. The "0 prune events"
  cells come from `movetime 500`, which the artifact that produced them says a
  registered bench must re-take under `Stop::Nodes`. Under `nodes 50000` the cap
  fires 152 times at 51 stones (radius 2) and 130 times at 51 stones (radius 3).
- **BLOCKING 2** — the `ply > 1` mutant is registered twice in §7 and **no test
  in §7 can kill it.** Revision 2 exists to replace `ply > 1`; the correction
  ships with no falsifier. `docs/process.md`'s vacuous-criterion clause.
- **BLOCKING 3** — §4 declares itself "the enumerated set, not a glob" and is
  missing six files that will not compile. The two `grep`s §4 names as its
  verification method are structurally blind to the class it missed.

The mechanism itself — §2's guard, §2.3's ordering, §2.5's tie-break, and
§6.3's store rule — I could not break. See **WHAT I CHECKED AND FOUND SOUND**
for the attacks that failed and for what would flip §6.3.

---

## FINDINGS

### BLOCKING 1 — §8's spread expectation is stated MEASURED and is false at the budget §8 registers. **Registration finding, not correctness.**

**Claim attacked, verbatim (§8):**

> "- **Spread, `spread_v1`:** REPORTED, NOT GATED, and expected **exactly inert**
>   — MEASURED 0 prune events at 21 / 51 / 99 stones. Reported so no reader infers
>   the D-95 class moved; D-478 leaves that debt open."

and the same number in the preamble and §9:

> "**and the D-95 depth debt left entirely unpaid**" … "**The D-95 depth debt.**
> OPEN, re-pointed by D-478 at a package of its own, and MEASURED untouched by
> this one."

**THE ATTACK.** §8 registers its instrument as `tools/bench_block.sh` with
`--budget 'nodes 50000'`. The "0 prune events" cells are not from that budget.
Their only provenance is `artifacts/wp15d_turn_axis_v1.txt`'s S1 table, which is
run at `movetime 500` (`run(&st, radius, Some(500), 0, k, scope, false)`,
`wp15d_turn_axis_v1.txt:339,351`) — and that artifact's own header says:

> "Every movetime-derived cell here is SCOPING-ONLY: instrument mode refuses
> MovetimeMs (docs/decisions.md D-22) because wall-clock is not reproducible, so
> a registered calibration or bench re-takes these under Stop::Nodes."
> — `artifacts/wp15d_turn_axis_v1.txt:3-5`

Worse, the cells do not even hold at `movetime 500` at radius 2, the radius of
the standard bench config (`configs/instrument_staged_v0.toml`, the config
D-373's own bench ran):

```
$ /usr/bin/grep -n "S1/r2/except-root-turn/K8" artifacts/wp15d_turn_axis_v1.txt
S1/r2/except-root-turn/K8 p00 stones= 11 ... capped=2715
S1/r2/except-root-turn/K8 p01 stones= 21 ... capped=150     <- not 0
S1/r2/except-root-turn/K8 p02 stones= 51 ... capped=0
S1/r2/except-root-turn/K8 p03 stones= 99 ... capped=0
```

Only the r=3 rows (`S1/r3/except-root-turn/*`) read 0 at 21/51/99. §8 names no
config, so a reader cannot tell which radius the claim is about — and it is
false at one of the two.

**REPRODUCED at the registered budget.** Probe: one `eprintln!` at the design's
own §2.1 site, printing `turns_from_root()` and the pre-truncation pool size on
every safety-net row; the exact guard `turns_from_root() > 0 && pool > K` at
K = 8, counted over the shipped line protocol.

```
$ cd /home/tom/Projects/pistol-wt-rvd            # detached at 13720fc
# pvs.rs, inserted immediately before `set.promote_table_move(table_move);`:
#     if set.used_quiet_safety_net {
#         eprintln!("SNPROBE tfr={} pool={}", self.turns_from_root(), set.cells.len());
#     }
$ CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rvd-target cargo build --release --locked -p pistol-cli
$ ENG=/home/tom/Projects/pistol-wt-rvd-target/release/pistol
$ i=0; while IFS= read -r pos; do i=$((i+1)); \
    printf 'newgame\n%s\ngo nodes 50000\nquit\n' "$pos" \
    | "$ENG" --config configs/instrument_staged_v0.toml 2>&1 >/dev/null \
    | sed -n 's/^SNPROBE //p' \
    | awk -v p=$i '{split($1,a,"=");split($2,b,"="); rows++; if(a[2]>0 && b[2]>8) pr++} \
        END{printf "spread p%d: safety_net_rows=%d PRUNE_EVENTS=%d\n", p, rows+0, pr+0}'; \
  done < <(/usr/bin/grep '^position' crates/pistol-cli/tests/fixtures/spread_v1.txt)
spread p1: safety_net_rows=235 PRUNE_EVENTS=95      # 11 stones
spread p2: safety_net_rows=265 PRUNE_EVENTS=5       # 21 stones
spread p3: safety_net_rows=772 PRUNE_EVENTS=152     # 51 stones
spread p4: safety_net_rows=1196 PRUNE_EVENTS=0      # 99 stones
```

And the same at radius 3 (`instrument_staged_v0.toml` with `quiet_radius = 3`,
written to the scratchpad, never to a config document):

```
r=3 spread p1 nodes50000 K=8: rows=303  PRUNE_EVENTS=0
r=3 spread p2 nodes50000 K=8: rows=573  PRUNE_EVENTS=5      # 21 stones
r=3 spread p3 nodes50000 K=8: rows=1493 PRUNE_EVENTS=130    # 51 stones
r=3 spread p4 nodes50000 K=8: rows=2655 PRUNE_EVENTS=20     # 99 stones
```

**At BOTH radii, under the budget §8 registers, the cap fires at 21 and 51
stones.** "Expected exactly inert — MEASURED 0 prune events at 21 / 51 / 99
stones" is false at 21 and 51 stones at radius 2 and at 21, 51 and 99 stones at
radius 3.

(The counts above are taken on the UNCAPPED tree, since the cap is not
implemented; applying it changes the tree. That direction does not rescue the
claim: it establishes that the guard's own predicate is satisfied hundreds of
times on the fixture §8 calls inert.)

**Why BLOCKING and not MAJOR.** §8's spread line is REPORTED-NOT-GATED, so no
verdict hangs on it directly — but the number is doing work in three places
outside §8: the preamble's price statement, §8's own "Reported so no reader
infers the D-95 class moved", and §9's "**The D-95 depth debt.** … MEASURED
untouched by this one." A pre-registration that carries a MEASURED zero into
three conclusions, taken at a budget its own instrument's header forbids for
registered work, is D-374's case exactly: the number cannot be repaired after
the run without moving a registered threshold. It has to be re-taken under
`Stop::Nodes` at the named config **before** this document governs anything.
Fixing it may also require §8 to say what a non-zero spread count means for the
D-95 claim, since "0 prune events" was the whole of that argument.

---

### BLOCKING 2 — §7 registers the `ply > 1` mutant twice, and no test in §7 can kill it. The correction revision 2 exists to make has no falsifier. **Registration finding, not correctness.**

**Claims attacked, verbatim (§7's table):**

> | `the_root_turn_is_whole_at_turn_one_where_it_is_one_stone` | the same, from a
> TURN-1 root, where rule 3 gives one stone and the ply-1 node already belongs to
> the opponent | `turns_from_root() > 0` → `ply > 1`, the defect that failed
> revision 1 |

> | `no_cutoff_inside_the_played_turn_rests_on_an_unproved_bound` | **§6.4's
> census on the SHIPPED code, WARM table across a game: 0** … | the store rule
> deleted; **the guard widened to `ply > 1`** |

**THE ATTACK.** The two spellings differ at exactly one place in the whole tree.
`stones_in_turn` gives turn 1 one stone and every later turn two
(`crates/pistol-core/src/rules.rs:30-36`), so:

| root turn | ply 0 | ply 1 | ply 2 | ply 3 |
|---|---|---|---|---|
| R = 1 | tfr 0 | **tfr 1** | tfr 1 | tfr 2 |
| R ≥ 2 | tfr 0 | tfr 0 | tfr 1 | tfr 1 |

`turns_from_root() > 0` caps `{ply ≥ 1}` at R = 1 and `{ply ≥ 2}` at R ≥ 2.
`ply > 1` caps `{ply ≥ 2}` at every R. **They differ only at a turn-1 root, and
only at ply 1** — a node that is *not* in the root turn.

Row 5's stated criterion is the root turn's set identity ("the emitted set is
identical gate-on and gate-off at every node of the root turn", inherited from
row 4 via "the same"). At a turn-1 root the root turn is exactly `{ply 0}`, and
**both** spellings leave ply 0 uncapped. The mutant passes the test. This is
`docs/process.md` verbatim:

> "A criterion that is a property the named defect class PRESERVES … passes
> vacuously and is not a criterion; it must be one that defect could falsify."

Row 9 fails the same way, for a different reason. Its census counts cutoffs
inside the played turn taken on a record stored by a truncated node. Once
§6.3's store rule is in place, **no** truncated node stores an `Exact` or
`Upper` record under **any** scope predicate, so the census reads 0 whether the
guard is `turns_from_root() > 0` or `ply > 1`. Row 9 kills "the store rule
deleted"; it does not kill "the guard widened to `ply > 1`".

I walked the other nine rows. None reaches ply 1 of a turn-1 root:

- row 1 is gate-OFF, guard irrelevant;
- row 2 says "a node one turn from the root", names no root turn number, and
  registers no mutation at all (its mutation cell is `—`);
- rows 3, 10 are the boundary and the sort;
- row 4's mutation is `> 0` → `>= 0`, the other direction;
- row 6 is `used_quiet_safety_net`;
- rows 7, 8 are the store rule;
- row 11 (`tools/determinism.sh`) compares two runs of the *same* binary, and a
  mutant is as deterministic as the original.

**REPRODUCED** — by construction on the shipped code, not by argument:

```
$ /usr/bin/grep -n "fn stones_in_turn" -A 6 crates/pistol-core/src/rules.rs
30:pub const fn stones_in_turn(turn: u32) -> u32 {
31-    if turn == FIRST_TURN {
32-        FIRST_TURN_STONES        // 1
...
34-        TURN_STONES              // 2
$ sed -n '523,525p' crates/pistol-search/src/pvs.rs
    pub(crate) fn turns_from_root(&self) -> u32 {
        self.position.state().turn() - self.root_turn
    }
```

At a root with `state().turn() == 1`, `visit(.., ply=1)` is reached only after
`place` has completed turn 1 (`PlyOutcome::TurnComplete`, `pvs.rs:401-403`), so
`state().turn() == 2` and `turns_from_root() == 1 > 0` — capped. `ply > 1` is
`1 > 1` — not capped. The root turn at ply 0 is untouched by both. Row 5's
assertion is over the root turn only.

**What closes it.** A test at a turn-1 root asserting the **ply-1** node — the
opponent's first stone — **is** capped with the gate armed, and that both stones
of that opponent turn are capped alike. That is the property `ply > 1` breaks
and `turns_from_root() > 0` holds; it is also the only executable statement of
"turn-coherent", which is the word D-478 and this whole revision turn on.
Row 9's mutation cell should drop `ply > 1` or the census should be redefined to
something the scope can move.

---

### BLOCKING 3 — §4's file list is incomplete: six files will not compile without the new field, and the verification method §4 names cannot see them. **Registration finding, not correctness.**

**Claim attacked, verbatim (§4):**

> "## 4. WHERE THE CODE CHANGES — the enumerated set, not a glob
>
> Verified this session with `/usr/bin/grep -l 'kind = "staged"' configs/*.toml`
> and `/usr/bin/grep -rln 'kind = "staged"' crates/ --include=*.rs`."

**THE ATTACK.** Both named greps search for the string `kind = "staged"`. That
string appears only in TOML documents and in the one embedded TOML
(`crates/pistol-engine/tests/common/mod.rs`). It cannot find a Rust
`StagedParams { … }` struct literal, and `StagedParams` has no `Default`
(`params.rs:58`, `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`; no
`impl Default for StagedParams` anywhere), so **every** literal is a hard
compile error the moment the struct gains a field. The verification method is
blind to exactly the class it missed.

**REPRODUCED.** In the worktree at `13720fc`, add the field and compile:

```
$ cd /home/tom/Projects/pistol-wt-rvd
# crates/pistol-search/src/params.rs, added to `pub struct StagedParams`:
#     pub safety_net_top_k: u64,
$ CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rvd-target cargo test --workspace --locked --no-run 2>&1 \
    | /usr/bin/grep -E "^error|--> " | LC_ALL=C sort -u
   --> crates/pistol-engine/src/instance.rs:202:44
   --> crates/pistol-search/src/pvs.rs:827:37
   --> crates/pistol-search/src/pvs.rs:896:37
   --> crates/pistol-search/src/pvs.rs:954:22
   --> crates/pistol-search/src/quiescence.rs:541:9
error[E0063]: missing field `safety_net_top_k` in initializer of `StagedParams`
```

(The integration tests cannot even be reached until the lib compiles.) Patching
every literal until `cargo test --workspace --locked --no-run` is clean gives the
complete set — 14 literal sites in 10 files:

```
$ git diff --name-only | LC_ALL=C sort
crates/pistol-engine/src/instance.rs                          <- listed by §4
crates/pistol-search/src/params.rs                            <- listed (the definition)
crates/pistol-search/src/pvs.rs                               <- listed
crates/pistol-search/src/quiescence.rs                        <- NOT LISTED
crates/pistol-search/tests/common/mod.rs                      <- listed
crates/pistol-search/tests/staged_colony_family_tests.rs      <- NOT LISTED (2 sites)
crates/pistol-search/tests/staged_differential_gate_tests.rs  <- NOT LISTED
crates/pistol-search/tests/staged_pattern_fixture_tests.rs    <- NOT LISTED
crates/pistol-search/tests/staged_tests.rs                    <- listed
crates/pistol-search/tests/staged_tier_t_threshold_tests.rs   <- NOT LISTED
crates/pistol-search/tests/wp18b_solver_path_tests.rs         <- NOT LISTED (2 sites)
$ CARGO_TARGET_DIR=/home/tom/Projects/pistol-wt-rvd-target cargo test --workspace --locked --no-run 2>&1 \
    | /usr/bin/grep -E "^error|--> " | LC_ALL=C sort -u
(no output — clean)
```

**Six files missing.** Two further precisions on the rows §4 does list:

- `crates/pistol-search/tests/staged_tests.rs` is listed only as part of
  "`{search_determinism_tests,staged_tests}.rs` | 8 call sites". The 8 is
  correct for the `staged_searcher` helper (5 in `search_determinism_tests.rs`
  at 97/110/125/168/176, 3 in `staged_tests.rs` at 291/341/366), but
  `staged_tests.rs` also holds **two direct `StagedParams` literals**
  (`:69`, `:420`) that the "8 call sites" count does not cover.
- `crates/pistol-search/src/pvs.rs` is listed for the guard and the store rule;
  it also has **three** literals in its own `mod tests` (827, 896, 954).

The right verification is the compile itself, or
`git grep -n 'StagedParams {' -- '*.rs'`, not a search for a TOML key.

---

### MAJOR 4 — §8's two MEASURED corpus ratios have no receipt anywhere, and the per-position node-count claim is not what the cited instrument records.

**Claim attacked, verbatim (§8):**

> "The expectation is ≈ 1.00 and it is MEASURED: at every K the corpus seat's node
> counts are identical position for position and its wall ratios are 1.002 and
> 0.995."

**THE ATTACK, part 1 — the two ratios have no provenance.**

```
$ /usr/bin/grep -rn "1\.002\|0\.995" --include=*.txt --include=*.md . | LC_ALL=C sort
./docs/experiments/wp15d_design.md:379:  counts are identical position for position and its wall ratios are 1.002 and
./docs/experiments/wp15d_design.md:380:  0.995. ...
(plus three unrelated `llr_pair` hits in artifacts/wp16_*.txt and wp17_*.txt)
```

Neither number appears in any WP-1.5d artifact or in matrix M2. Every corpus
wall ratio derivable at the selected scope from the committed receipt is
different:

```
$ /usr/bin/grep -n "S2/CORPUS" artifacts/wp15d_turn_axis_v1.txt
S2/CORPUS/.../scope0/K0  sum_nodes=1104026 sum_ms=4776 sn_rows=66 capped=0
S2/CORPUS/.../scope4/K4  sum_nodes=1104026 sum_ms=4828 sn_rows=17 capped=9
S2/CORPUS/.../scope4/K8  sum_nodes=1104026 sum_ms=4795 sn_rows=18 capped=10
S2/CORPUS/.../scope4/K16 sum_nodes=1104026 sum_ms=4790 sn_rows=21 capped=13
```

4828/4776 = **1.0109**, 4795/4776 = **1.0040**, 4790/4776 = **1.0029**. The
nearest 0.995-shaped numbers in the corpus receipts are from a **different**
experiment on a different axis (`RD/CORPUS/.../recency/M{0,4,16}` in
`artifacts/wp15d_m2_evidence_v2.txt:254-256`: 4809/4844 = 0.9928,
4795/4844 = 0.9899).

**THE ATTACK, part 2 — "node counts identical position for position" is not
measured.** The S2 harness aggregates nodes and keeps only depths per position:

```
$ sed -n '368,386p' artifacts/wp15d_turn_axis_v1.txt
        for line in lines(&corpus).iter() {
            let r = run(...);
            nodes += r.nodes;          <- SUMMED
            ...
            depths.push(r.depth.to_string());   <- per position
        }
        println!("S2/CORPUS/... sum_nodes={nodes} sum_ms={ms} ... depths={}", ...)
```

The receipt shows an identical **sum** and identical per-position **depths**. It
does not show per-position node counts at all. This is round 1's MINOR 11 and
round 3's MINOR 7 arriving a third time — a MEASURED cell with no receipt —
and here it is the whole justification for the ≤ 1.10 bracket and the 1.25 abort
threshold, i.e. for a number a reader is asked to hold the run to.

**Not overrulable as prose**: D-424 reaches prose that constrains nothing, and a
registered bracket's stated expectation constrains the reading of the run.

---

### MAJOR 5 — §4 changes the handshake's `candidate_policy` line, which D-356 and `U2_node_protocol.md` §U2-M item 2 fix, with no ADR amendment; and it names `pistol.rs` as a forced-compile site when it is not one.

**Claim attacked, verbatim (§4):**

> | `crates/pistol-cli/src/bin/pistol.rs:154-159` | destructures; the handshake id
> gains the key |

**THE ATTACK.** Two separate problems.

**(a) It is not a forced site.** `pistol.rs:154-159` destructures with `..`:

```
$ sed -n '154,160p' crates/pistol-cli/src/bin/pistol.rs
        pistol_engine::config::CandidatePolicy::Staged {
            quiet_radius,
            quiet_top_k,
            ..
        } => {
            format!("candidate_policy staged quiet_radius {quiet_radius} quiet_top_k {quiet_top_k}")
```

Adding a field compiles unchanged. So the change is a deliberate protocol
decision, not a compile consequence, and §4 presents it as the latter.

**(b) The form it changes is pinned by an ADR and by `U2_node_protocol.md`.**
The immediately adjacent comment says why the line carries two keys and no
others:

```
$ sed -n '144,150p' crates/pistol-cli/src/bin/pistol.rs
    // `id candidate_policy staged quiet_radius <n> quiet_top_k <k>` — one
    // line, whitespace-delimited, multi-token value, the same shape
    // `id budgets depth_turns nodes` already establishes (docs/decisions.md
    // D-230; `U2_node_protocol.md` §U2-M item 2). `tier_t_own_count`,
    // `tier_t_opponent_count` and `widen_schedule` do not ride on this line —
    // U2-M item 2 names only `quiet_radius` and `quiet_top_k`.
```

D-356 states the same rule and calls it "the exact form U2-M item 2 states". The
line is also the seat-identity check in two governed pre-registrations
(`wp15b_sprt_prereg.md:637`, `wp16_sprt_prereg.md:1051-1059`) and in D-373 and
D-381's own evidence. Hard rule 10: "Silent architecture drift is a breach;
amend the ADR instead." §4 changes it in a table cell.

**The change is probably right and still needs the ADR line.** Without it, the
two SPRT seats this WP will register — `safety_net_top_k = 0` against the
calibrated K — have **byte-identical** `engine_id` handshakes, and the prereg's
Criterion 2 (distinct seats) cannot fire. That is the argument the amendment
should carry.

**NOT REPRODUCED as a test break**: no committed test pins the Staged handshake
line, so nothing goes red.

```
$ git grep -rn "candidate_policy staged" -- crates/ tools/
crates/pistol-cli/src/bin/pistol.rs:144   (comment)
crates/pistol-cli/src/bin/pistol.rs:159   (the format! itself)
```

`crates/pistol-cli/tests/handshake_identity_tests.rs` has no staged case. That
is a second, smaller gap: §7 registers no test for the changed handshake line.

---

### MAJOR 6 — §8 registers ratios, medians and an IQR gate, and names no instrument that computes any of them; and it names no config for either bench seat.

**Claim attacked, verbatim (§8):**

> "**INSTRUMENT.** `tools/bench_block.sh` at its committed revision, over
> `bench_positions_v1.txt` (`--grammar tail`) and `spread_v1.txt`
> (`--grammar line`), `--budget 'nodes 50000'`, `--reps 5`, both seats
> (`safety_net_top_k = 0` and the calibrated K), IQR-gated per position at the
> D-215/D-362 10 % convention."

**THE ATTACK.** `tools/bench_block.sh` says in its own header that it does none
of this:

```
$ sed -n '33,40p' tools/bench_block.sh
# This script takes NO measurement decisions. It states no budget, no rep count,
# no config and no fixture of its own — every one of them is the caller's,
# ...
# It prints one record line per (entry, rep) and a summary;
# banding, medians, IQR gating and ratios are the caller's, from these lines.
```

Its only outputs are `bench_block: record entry N stones S rep R <totals>` lines
and a `done:` summary (`tools/bench_block.sh:266-277`). The median, the IQR gate,
and the ttd/nps ratios — the three quantities §8's bracket and abort threshold
are stated in — are produced by an unnamed artefact. `docs/process.md`:

> "THE INSTRUMENT HAS A GOVERNING REVISION TOO. An artefact that produces a
> registered number — a `tools/` script, a scratchpad harness, or a command block
> the document prints — is named in the pre-registration WITH ITS REVISION."

Second gap in the same paragraph: `bench_block.sh` takes `--config PATH`, one
config per invocation, and §8 names **no config for either seat**. The OFF seat
presumably reuses `configs/instrument_staged_v0.toml`; the ON seat needs a
document carrying the calibrated K, and **no such document is in §4's file
list** — §4 introduces only `configs/gate_staged_snk_v0.toml`, which is the
determinism seat at a hard-coded K = 8. BLOCKING 1 above is a direct consequence
of this omission: with no config named, the radius the spread claim is about is
undetermined, and the claim is false at one of the two candidate radii.

---

### MAJOR 7 — §5's grid and selection rule are justified on a measurement taken WITHOUT §6.3's store rule, whose cost §6.3 itself names as material.

**Claims attacked, verbatim:**

> "The MEASURED shape here is different and is stated before the rule is applied:
> on the book `lift` is nearly FLAT across the low grid (535 / 524 / 514 at
> K = 4 / 8 / 16)" — §5

> "**What this costs, named rather than hidden**: the lost entries are ordering
> and cutoff information a later visit would have reused, so **the capped search
> gets slower**." — §6.3

**THE ATTACK.** I verified the four `lift` cells against the receipts and they
reproduce exactly (see WHAT I CHECKED AND FOUND SOUND). But every one of them
was taken by an instrument that has **no store rule**
(`artifacts/wp15d_m2_evidence_instrument_v2.txt` PART 6 truncates `set.cells`
and changes nothing at `self.table.store`). §6.3 declines a store at every
truncated node that fails low or returns exact — on the book that is a large
fraction of the 1 593 643 capped rows — and §6.3 says the effect is a slower
search. `lift(K)` is measured **at a fixed 50 000-node budget**, where "slower"
does not reduce nodes but does reduce what those nodes buy through lost ordering
and lost cutoffs, i.e. it reduces completed depth. So the shipped mechanism's
`lift` curve is not the measured one, and the design's own reason why "largest K
within 90 % of the best" is not degenerate — the flatness at 535/524/514 and the
decay the grid's upper end is chosen to expose — rests on the wrong curve.

This is not fatal to the rule: §5 registers the rule *before* the sweep, per
D-374, and the sweep is at the implementation revision, so the selection stays
honest. What is wrong is the **justification**, and the justification is what
`wp19_design_REVIEW_rev2.md` BLOCKING N1(c) asks for — "does the selection rule
select a grid extreme by construction?" — which cannot be answered against a
curve the shipped code will not produce.

**NOT REPRODUCED** (implementing §6.3 and re-running the 2 000-opening sweep is
the 44–50 minutes §5 itself budgets, and the sweep is the WP's own registered
work, not a reviewer's). The mechanism is stated at `pvs.rs:450-467` and by §6.3
itself. What discharges it is one honest sentence: that the shape is measured
without the store rule, and that the sweep at the implementation revision is
what decides whether the decay is still on the grid.

---

### MAJOR 8 — §5's channel silently folds "the search stopped on a mate" into "did not reach depth 3", and the undefined-case clause enumerates only depth 0.

**Claim attacked, verbatim (§5):**

> "**THE CHANNEL, AND ITS DIRECTION.** `lift(K)` = the number of the 2 000
> openings whose **completed** `depth_turns` is ≥ 3. It is a COUNT and **larger
> is better**. It is defined for every opening at this budget — MEASURED, the
> incumbent's histogram has **zero** openings at completed depth 0"

**THE ATTACK.** Iterative deepening does not only stop on the budget:

```
$ sed -n '357,359p' crates/pistol-search/src/search.rs
            if is_mate(score) {
                break;
            }
```

A search that returns a mate score at depth 2 stops there and reports
`depth_turns = 2` — counted by `lift` as a failure to reach 3, though it is the
strongest possible answer. With the gate armed this is not a curiosity: the cap
removes the OPPONENT's candidate cells at every node below the root turn
(`staged.rs:230`, the ball is the whole set on that row and both sides' cells
are in it), so a "mate" found inside a truncated subtree is a mate against a
defender whose replies were pruned. §6.3 is explicit that the capped search's
values are not the uncapped search's; a manufactured mate is that, plus a
control-flow effect on the very quantity `lift` measures.

The design handles one degenerate case (depth 0) with a named VOID rule and does
not name this one. `lift` therefore mixes "how deep it gets" with "how often it
stops early on a mate score", and the sign of the mixture across K is unknown.

**NOT REPRODUCED as a false mate** — producing one requires the cap implemented
and a position search; I did not build it, and I record that rather than assert
it. What IS established at `search.rs:357-359` is that a mate score truncates
deepening and therefore lowers `lift`, and that §5 does not say so. The cheap
fix is to count and report mate-terminated openings per seat beside `lift`, the
same way §5 already handles depth 0.

---

### MINOR 9 — §5 registers a VOID threshold that cannot fire, in a document whose §8 refuses to register such a threshold on D-424 grounds.

> "if more than 1 % of openings are in that state on any seat the calibration is
> VOID rather than read" — §5

Under `Stop::Nodes` the first iteration is never abortable
(`search.rs:323`: `let abortable = depth_turns > 1 || fallback.is_some();`, and
`fallback` is `None` for `Stop::Nodes` at `search.rs:232-238`), so depth 1 is
always completed and `depth_turns = 0` is structurally unreachable — which is
why the incumbent's histogram reads `[0, 10, 1869, 113, 8, 0, 0, 0]`. The clause
is therefore a threshold that can never fire, which is exactly what §8 declines
to register two pages later:

> "so registering a bound on it would be a threshold that can never fire — prose
> a reviewer must still attack (D-424)." — §8

Either the §5 clause states that it is a belt over a structural guarantee (and
names the guarantee), or §8's principle applies to it too.

---

### MINOR 10 — `configs/gate_staged_snk_v0.toml` fixes K = 8 while §9 says no K is chosen.

§4 commits a config document at `safety_net_top_k = 8`; §9 says "**K's value.**
§5's rule decides it from a sweep taken at the implementation revision. No value
is chosen here." The determinism seat only needs the cap *armed*, so 8 is
defensible — but the document should say that, and say whether the seat is
re-pointed at the calibrated K once §5 answers. As a positive: I confirmed the
seat is not vacuous (see WHAT I CHECKED AND FOUND SOUND).

---

### MINOR 11 — §4's "`Searcher::new`'s own bound" and §4's "the bound lives here" name no bound.

Every other `Searcher::new` refusal is a real range (`radius >= 1`,
`tier_t_*_count ∈ {2,3}`, `q_depth_turns <= MAX_Q_EXTENSION_PLIES/2`,
`search.rs:100-148`). `safety_net_top_k: u64` has no out-of-range value: 0 is
off and every other value is a legal ceiling. If the change is a destructure
only, §3/§4 should say so; if a bound is intended (a ceiling, or a refusal of
values above the largest possible pool), it has to be named, because rule 1
puts the default in one schema place and rule 3 forbids a silent no-op.

---

### MINOR 12 — §2.1 point 3 is true of the SITE and not of the SCOPE.

> "3. **It is exactly where every measurement in the matrix was taken.**" — §2.1

The site claim reproduces exactly: `artifacts/wp15d_m2_evidence_instrument_v2.txt`
PART 6 inserts the truncation after the empty-set check and immediately before
`set.promote_table_move(table_move);`. But the matrix's cells were taken under
`scope 4 => ply > 1`, not `turns_from_root() > 0` — the two spellings the whole
of revision 2 is about. They coincide on every sample the matrix used (no
measured position is a turn-1 root), which is why the numbers transfer; the
document should say that in one clause rather than let "exactly where every
measurement was taken" carry a scope claim it does not support.

A second, smaller drift in the same direction: the instrument's `SN_ROWS_CAPPED`
increments whenever the SCOPE applies, not whenever the cap binds
(`set.cells.truncate(cap)` is a no-op when `len <= cap` but the counter has
already ticked). §2.2's shipped `truncated` adds `set.cells.len() > K`. So
"1 593 643 / 1 724 042 (92.4 %)" is scope-applicable rows, and the shipped
`truncated` flag fires on a strictly smaller set. The design's own reasoning for
the `len() >` test is right; the inherited percentage is not the same quantity.

---

## WHAT I CHECKED AND FOUND SOUND

**§6.3's store rule is sufficient and correctly stated. I attacked it five ways
and it held.** The rule is `WPQ_seed.md` §7.2's, adopted faithfully — I read the
seed's §7.2 and the design's §6.3 quotes it without narrowing it, which is
exactly what round 3's BLOCKING 1 said matrix M2 had failed to do.

The coherent statement of what the rule buys is: define `V_K(n)` as the value of
node `n` under "truncate at every node with `turns_from_root() > 0`". Because
the truncated set is a pure function of the position — `used_quiet_safety_net`
and the pool are functions of the board and threat state, the delta ranking is a
function of the position and mover, and §2.3 puts the truncation before
`promote_table_move` (`pvs.rs:328`) and before the WP-1.7 ordering heuristics
(`pvs.rs:351-361`) — `V_K` depends on nothing but position and depth. The
root-turn search computes `W(n)`, full width at its own nodes and `V_K` below.
`W(n) >= V_K(n)` for every `n`, so a `Bound::Lower` record survives the promotion
from "deep in search N" to "inside search N+1's root turn" and `Upper`/`Exact`
do not. That is precisely the rule.

The four alternative paths I could think of, and why each is closed:

1. **An UNTRUNCATED ancestor storing `Exact` over children that were truncated.**
   The dangerous shape is a node `n` at `turns_from_root = d >= 1` whose own set
   is full but whose same-turn second-stone child `c` was truncated. `n` is
   phase-`First`, so in a later search `n` can only be the ROOT position itself
   (`check_root` refuses a non-`First` root, `search.rs:470-472`), and ply 0 is
   always PV (`iterate` opens at `(-INFINITY, INFINITY)`, `pvs.rs:153`) so it
   cannot take a table cutoff. `c` is phase-`Second`, so it lands at ply 1 where
   a null window makes `is_pv` false and a cutoff IS possible — and `c` is
   exactly the node §6.3 covers. Closed.
2. **A node reachable "upward".** Search N+1's root has strictly more stones than
   search N's root-turn nodes, and stone count is a function of the key, so
   search N's ply-0/ply-1 nodes are unreachable later. This is F11's argument in
   the direction where it is actually true, and §6.2 says so.
3. **Quiescence.** `Run::quiescence` never calls `staged_candidates` or
   `within_radius` (`quiescence.rs:76-195`), so no quiescence node is ever
   truncated; and `Table::probe` never returns a `from_quiescence` entry
   (`tt/mod.rs:129-137`). Closed on both sides.
4. **`Run::salvage` / `root_score` / the PV / `Provenance::PartialRoot`.** The
   answered move is `pv.first()` of the ply-indexed PV
   (`search.rs:328-334`), and a ply-0 promotion happens only on a COMPLETED
   child subtree at full root width (`pvs.rs:414-424`, `salvage`'s own doc at
   `pvs.rs:161-176`). Salvage is read for `Stop::Deadline` only
   (`search.rs:370-375`), and the counters are overwritten on every path
   including both salvage arms (`search.rs:445-457`), so the new `StageCounters`
   fields will not read zero on the `PartialRoot` path — which `info.rs:13-17`
   requires. Closed.
5. **`root_restrict`.** It edits the set at `ply == 0` only
   (`pvs.rs:336-347`); `turns_from_root()` is 0 there, so `truncated` is false
   and the two mechanisms never interact. (The root restriction does narrow ply 0
   under a defender proof, so "the root turn is full width" is true of THIS cap
   and not of the engine — pre-existing, D-scope wp18b's, not this design's.)

Mate re-basing is not a path either: `to_table`/`from_table` re-base against the
storing and probing node's own `turns_from_root` (`tt/mod.rs:135,159`), so a
`Lower` mate bound transfers with its distance intact.

**§2.2's guard names exactly the root turn, at every turn number.** `root_turn`
is `position.state().turn()` fixed once per search (`pvs.rs:116,123`), `Run` is
crate-private (`pvs` is not a `pub mod` in `lib.rs:24-35`), and `check_root`
refuses a decided root, a non-`First` phase, and an empty candidate set
(`search.rs:463-478`). So a mid-turn root is unreachable through the only entry
point. Rule 4 (a win on the first stone) is handled by construction: `place`
returns `PlyOutcome::Win` and the parent scores without recursing
(`pvs.rs:379-395`), so no node exists at which the guard could be asked about the
truncated turn's second stone. At a turn-1 root the root turn is one stone and
the guard exempts exactly it.

**§1's two quoted facts, and §2.4/§2.5.** `used_quiet_safety_net` is written in
exactly one place, `batched`, where `out.forced = 0`
(`staged.rs:230-235`), so the cap can exclude no Tier-F and no Tier-T cell — it
is a guard, as §1 says. `within_radius` drains a `BTreeSet`
(`candidates.rs:47-61`) so the ball arrives ascending, and `delta_rank` uses
`sort_by_key` with `Reverse(score)` (`staged.rs:292`), a stable sort — the K-th
and (K+1)-th cells are separated by a total order with no clock, thread or hash
iteration in it. Rule 4 of CLAUDE.md holds by construction, and §2.5 is right to
seat it anyway.

**Every numeric claim I could check against a receipt reproduced.**

- `lift`, recomputed from the depth histograms in
  `artifacts/wp15d_turn_axis_book_v1.txt` and
  `artifacts/wp15d_redteam_round3_instrument_v1.txt:121,95`:
  K=4 → 278+254+3 = **535**; K=8 → 305+215+4 = **524**;
  K=16 → 350+162+2 = **514**; K=32 → 422+68 = **490**; incumbent → 113+8 = 121.
  All four match the design. The incumbent's depth-0 count is **0**, as §5 says.
- Pool mean **77.7**: `pool_mean=77.70` at K=8, `77.66` / `77.72` / `77.71` at
  K=4/16/32.
- Sweep cost anchor **377–430 s**: the five 2 000-opening seats ran
  `ms=376953 / 425606 / 426503 / 429505 / 378706`. 7 seats × that ≈ 44–50 min. ✓
- Book **1.131×**: 426503 / 376953 = 1.1314. ✓
- The preamble's **153** and **1.47×**: 677 − 524 = 153 and 1.131 / 0.770 =
  1.4688, both from `matrix_M2.md` §3–§4 as cited. ✓
- **41 / 0 / 0** and **41 → 0** cold: `matrix_M2_REDTEAM_round3.md`'s
  `RT3-C/KEPT-TT` and `RT3-D/COLD-TT` rows, quoted correctly. ✓
- §4's **twelve** `kind = "staged"` documents: confirmed, exactly twelve
  (`/usr/bin/grep -l 'kind = "staged"' configs/*.toml | wc -l` → 12), and
  `crates/pistol-engine/tests/common/mod.rs`'s `VALID_STAGED` is the one embedded
  TOML.
- §4's `validate.rs:81-92` "destructures all ten fields with no `..`": confirmed
  — `quiet_radius, quiet_top_k, widen_schedule, tier_t_own_count,
  tier_t_opponent_count, q_depth_turns, q_triggers, killers, history,
  countermove`, ten, no rest pattern. `config_validate_tests.rs:186-197`
  destructures the same ten, also with no `..`.

**§3's gate is right, and the `quiet_top_k` asymmetry it names is real.** The
`q_depth_turns` precedent reproduces verbatim (`config.rs:238`, `params.rs:72`:
"`0` disables the extension"). `quiet_top_k` is validated `>= 1`
(`validate.rs:94-99`) and DISABLED in the committed documents by a LARGE value
(D-357: 1024 in `tactical_staged_v0.toml`, 128 in `gate_staged_v0.toml`), so it
has no off-value and the two knobs genuinely cannot share a shape. §3's one-key,
`0`-is-off design is correct, and its "state their status once, here" is D-423
applied properly.

**Adding counters to `StageCounters` cannot move any pinned output.**
`info.rs:8-12`: "**The line protocol does not carry these** — `report.rs` renders
an explicit field list, so no protocol output changes." §7's row 1 (gate-off
byte-identical against a sha-pinned fixture) is therefore not endangered by §4's
three new counters.

**Rule 9 is not newly engaged.** Every file §4 grows that is already over the
300-line cap has a `docs/rule9_justifications.md` entry (`pvs.rs`, `search.rs`,
`config.rs`, `pistol.rs`, `staged_tests.rs`, `config_validate_tests.rs`);
`tools/determinism.sh` is 284 lines and a fifth seat is one array line.

### Attacks that did NOT reproduce

- **"The fifth determinism seat is vacuous — the safety-net row never fires on a
  tactical fixture."** REFUTED, and comfortably. Probe as in BLOCKING 1, over
  `crates/pistol-cli/tests/fixtures/tactical_staged_v0.txt` (20 positions) under
  `configs/gate_staged_v0.toml` at both of `tools/determinism.sh`'s budgets:

  ```
  budget='depth_turns 4'  positions=20 safety_net_rows_total=151 max_pool=52
  budget='nodes 200000'   positions=20 safety_net_rows_total=153 max_pool=52
  ... 151 of 151 rows have pool > 8   (pool sizes range 33..52)
  ```

  At `quiet_radius = 1` and K = 8 the cap binds on every safety-net row the seat
  reaches. `configs/gate_staged_snk_v0.toml` is a real seat.

- **"§6.3 leaves a fourth path open."** Five candidate paths attacked above; none
  reproduced. Recorded as rejected with the attempted reasoning rather than left
  implicit.

- **"The 202 poisoned table-move promotions round 3 measured are still live."**
  They are, and they are harmless: `record.best` is read for ordering only
  (`pvs.rs:279`, `promote_table_move` rotates within `cells[forced..]`), and a
  root-turn node's set is full width, so a promoted cell is always already in the
  set. Ordering cannot change a value. Not a finding.

---

## WHAT WOULD FLIP THE VERDICT

Nothing in the mechanism. §2's guard and §6.3's store rule are, as far as I can
break them, correct — and the design has genuinely fixed round 3's BLOCKING 1
and BLOCKING 2 rather than argued around them. What fails is the registration
around the mechanism, and all three BLOCKING findings are cheap to close:

1. Re-take the spread cells under `Stop::Nodes` at a NAMED config, and say what
   a non-zero count means for the D-95 statement (BLOCKING 1). ~2 minutes of
   machine time; the command is in this report.
2. Add one test at a turn-1 root asserting the **ply-1** node IS capped, and drop
   `ply > 1` from row 9's mutation cell (BLOCKING 2).
3. Replace §4's grep-based verification with the compile, and add the six missing
   files (BLOCKING 3). The command is in this report.

Then MAJOR 4's two ratios need a receipt or need deleting, MAJOR 5 needs an ADR
line, and MAJOR 6 needs the aggregating instrument and the two bench configs
named.
